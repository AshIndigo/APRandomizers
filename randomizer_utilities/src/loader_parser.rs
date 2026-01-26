use crate::versions::VersionInformation;
use std::fmt::{Display, Formatter};
use std::sync::OnceLock;
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader;
use windows::core::PCSTR;

pub static LOADER_STATUS: OnceLock<LoaderStatus> = OnceLock::new();

#[derive(Debug, Clone)]
#[repr(C)]
pub struct LoaderStatus {
    pub game_information: VersionInformation,
    pub mod_information: Vec<VersionInformation>,
}

impl Display for LoaderStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type GetStatusFn = unsafe extern "C" fn() -> *const LoaderStatus;

pub fn set_loader_status() {
    let loader_status = unsafe {
        let loader_hmodule =
            LibraryLoader::LoadLibraryA(PCSTR::from_raw(c"dinput8.dll".as_ptr() as *const u8));
        let proc_addr = LibraryLoader::GetProcAddress(
            loader_hmodule.unwrap(),
            PCSTR::from_raw(c"get_loader_status".as_ptr() as *const u8),
        );
        (*std::mem::transmute::<FARPROC, GetStatusFn>(proc_addr)()).clone()
    };
    log::info!("Loader Status: {loader_status:?}");
    if LOADER_STATUS.set(loader_status).is_err() {
        log::error!("Failed to set global loader status");
    }
}
