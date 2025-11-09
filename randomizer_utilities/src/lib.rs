
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
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
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

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
            format!("logs/{}.log", prefix),
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

/// Loads or create a config file with the given name and struct.
///
/// This will also remake the config if the file cannot be read
pub fn load_config<T>(config_name: &str) -> Result<T, Box<dyn std::error::Error>>
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
