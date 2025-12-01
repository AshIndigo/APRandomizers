use std::error::Error;
use std::ffi::OsStr;
use std::{fs, ptr};
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::sync::OnceLock;
use figment::Figment;
use figment::providers::{Format, Toml};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::onstartup::OnStartUpTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use log::LevelFilter;
use tokio::sync;
use tokio::sync::mpsc::{Receiver, Sender};
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};
use windows::Win32::Foundation::GetLastError;

pub mod cache;
pub mod exception_handler;
pub mod item_sync;
pub mod archipelago_utilities;
pub mod mapping_utilities;
pub mod ui_utilities;

/// Default config for log files
///
/// # Arguments
///
/// * `prefix`: Prefix for the log file (I.e dmc3_rando.log)
///
/// returns: Handle
pub fn setup_logger(prefix: &str) -> Handle {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {h({l})} {t} - {m}{n}")))
        .build();

    let log_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .append(false)
        .build(
            format!("logs/{}_latest.log", prefix),
            Box::new(CompoundPolicy::new(
                Box::new(OnStartUpTrigger::new(10)), // 0x35c Rough guess based on the usual log output I spill out
                Box::new(
                    FixedWindowRoller::builder()
                        .build(format!("logs/{}_{}.log", prefix, "{}").as_str(), 3)
                        .unwrap(),
                ),
            )),
        )
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .logger(Logger::builder().build("tracing::span", LevelFilter::Warn))
        .logger(Logger::builder().build("minhook", LevelFilter::Warn))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("log_file")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}

pub fn is_library_loaded(name: &str) -> bool {
    let wide_name: Vec<u16> = OsStr::new(name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        if let Ok(module_handle) = GetModuleHandleW(PCWSTR::from_raw(wide_name.as_ptr())) {
            !module_handle.is_invalid()
        } else {
            false
        }
    }
}

/// Generic method to get the base address for the specified module, returns 0 if it doesn't exist
pub fn get_base_address(module_name: &str) -> usize {
    let wide_name: Vec<u16> = OsStr::new(&module_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe {
        if let Ok(module_handle) = GetModuleHandleW(PCWSTR::from_raw(wide_name.as_ptr())) {
            module_handle.0 as usize
        } else {
            0
        }
    }
}

/// Reads <T> data from a provided offset
pub fn read_data_from_address<T>(address: usize) -> T
where
    T: Copy,
{
    unsafe { *(address as *const T) }
}


/// Loads or create a config file with the given name and struct.
///
/// This will also remake the config if the file cannot be read
pub fn load_config<T>(config_name: &str) -> Result<T, Box<dyn Error>>
where T: Default + serde::ser::Serialize + serde::de::Deserialize<'static> {
    if !fs::exists("archipelago")? {
        fs::create_dir("archipelago/")?;
    }
    let config_path = format!("archipelago/{}.toml", config_name);
    if !Path::new(&config_path).exists() {
        log::debug!("Config file not found. Creating a default one.");
        let toml_string =
            toml::to_string(&T::default()).expect("Could not serialize default config");
        fs::write(&config_path, toml_string).expect("Could not write default config to file");
    }
    match Figment::new()
        .merge(Toml::file(&config_path))
        .extract::<T>()
    {
        Ok(config) => Ok(config),
        Err(err) => {
            log::warn!("Failed to parse config: {err}. Backing up and regenerating.");
            
            let backup_path = format!("archipelago/{}.old.toml", config_name);
            fs::rename(&config_path, &backup_path)?;
            log::info!("Old config backed up to {backup_path}");


            let toml_string =
                toml::to_string(&T::default()).expect("Could not serialize default config");
            fs::write(&config_path, &toml_string)?;

            Ok(T::default())
        }
    }
}

pub fn setup_channel_pair<T>(channel: &OnceLock<Sender<T>>, buffer_count: Option<usize>) -> Receiver<T>  {
    let (tx, rx) = sync::mpsc::channel(buffer_count.unwrap_or(8));
    channel.set(tx).expect("TX already initialized");
    rx
}

pub fn modify_protected_memory<F, R, T>(f: F, offset: *mut T) -> Result<R, Box<dyn Error>>
where
    F: FnOnce() -> R,
{
    let length = size_of::<T>();
    let mut old_protect = PAGE_PROTECTION_FLAGS::default();
    unsafe {
        if VirtualProtect(
            offset as *mut _,
            length,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
        .is_err()
        {
            return Err(format!("Failed to use VirtualProtect (1): {:?}", GetLastError()).into());
        }
        let res = f();
        if VirtualProtect(offset as *mut _, length, old_protect, &mut old_protect).is_err() {
            return Err(format!("Failed to use VirtualProtect (2): {:?}", GetLastError()).into());
        }
        Ok(res)
    }
}

pub unsafe fn replace_single_byte(offset_orig: usize, new_value: u8) {
    let offset = offset_orig as *mut u8;
    match modify_protected_memory(
        || unsafe {
            ptr::write(offset, new_value);
        },
        offset,
    ) {
        Ok(()) => {
            const LOG_BYTE_REPLACEMENTS: bool = false;
            if LOG_BYTE_REPLACEMENTS {
                log::debug!(
                    "Modified byte at: Offset: {:X}, byte: {:X}",
                    offset_orig,
                    new_value
                );
            }
        }
        Err(err) => {
            log::error!("Failed to modify byte at offset: {offset_orig:X}: {err:?}");
        }
    }
}