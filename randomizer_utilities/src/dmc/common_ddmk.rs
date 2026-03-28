use crate::BasicNothingFunc;
use crate::dmc::dmc_constants::DDMKHandler;
use imgui_sys::{ImGuiCond, ImGuiWindowFlags, ImVec2, cty};
use minhook::MinHook;
use std::collections::HashSet;
use std::os::raw::c_char;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

pub static DDMK_INFO: OnceLock<DDMKHandler> = OnceLock::new();

pub static SETUP: AtomicBool = AtomicBool::new(false);
static ORIG_TIMESTEP_FUNC: OnceLock<Option<BasicNothingFunc>> = OnceLock::new();

pub fn run_common_ddmk_code() {
    if let Some(ddmk_info) = DDMK_INFO.get() {
        init_render_func(ddmk_info);
        init_timestep_func(ddmk_info);
        unsafe {
            MinHook::enable_hook((*(ddmk_info.ddmk_address) + ddmk_info.timestep_func_addr) as _)
                .expect("Failed to enable timestep hook");
        }
    }
}

fn init_timestep_func(ddmk_info: &DDMKHandler) {
    ORIG_TIMESTEP_FUNC.get_or_init(|| {
        Some(unsafe {
            std::mem::transmute::<_, BasicNothingFunc>(
                MinHook::create_hook(
                    (*(ddmk_info.ddmk_address) + ddmk_info.timestep_func_addr) as _,
                    hooked_timestep as _,
                )
                .expect("Failed to create timestep hook"),
            )
        })
    });
}

unsafe extern "C" fn hooked_timestep() {
    unsafe {
        if !SETUP.load(Ordering::SeqCst)
            && let Some(ddmk_info) = DDMK_INFO.get()
        {
            MinHook::enable_hook((*(ddmk_info.ddmk_address) + ddmk_info.main_func_addr) as _)
                .expect("Failed to enable hook");
            SETUP.store(true, Ordering::SeqCst);
        }

        match get_orig_timestep_func() {
            None => {
                panic!("ORIG_TIMESTEP_FUNC not initialized in hooked render");
            }
            Some(timestep_func) => {
                timestep_func();
            }
        }
    }
}

pub fn checkbox_text(item: &String, list: &HashSet<String>) -> String {
    format!("{} [{}]", item, if list.contains(item) { "X" } else { " " })
}

pub fn get_orig_timestep_func() -> Option<BasicNothingFunc> {
    *ORIG_TIMESTEP_FUNC.get().unwrap_or(&None)
}

static ORIG_RENDER_FUNC: OnceLock<Option<BasicNothingFunc>> = OnceLock::new();

fn init_render_func(ddmk_info: &DDMKHandler) {
    ORIG_RENDER_FUNC.get_or_init(|| {
        Some(unsafe {
            std::mem::transmute::<*mut std::ffi::c_void, BasicNothingFunc>(
                MinHook::create_hook(
                    (*(ddmk_info.ddmk_address) + ddmk_info.main_func_addr) as _,
                    ddmk_info.hooked_render as _,
                )
                .expect("Failed to create hook"),
            )
        })
    });
}

pub fn get_orig_render_func() -> Option<BasicNothingFunc> {
    *ORIG_RENDER_FUNC.get().unwrap_or(&None)
}
// Bindings section
pub type ImGuiBegin =
    extern "C" fn(name: *const cty::c_char, p_open: *mut bool, flags: ImGuiWindowFlags) -> bool;
pub type ImGuiButton = extern "C" fn(label: *const cty::c_char, size: &ImVec2) -> bool;
pub type ImGuiText = extern "C" fn(text: *const cty::c_char, text_end: *const cty::c_char);
pub type ImGuiNextWindowPos = extern "C" fn(pos: &ImVec2, cond: ImGuiCond, pivot: &ImVec2);

pub static IMGUI_END: OnceLock<BasicNothingFunc> = OnceLock::new();
pub static IMGUI_BEGIN: OnceLock<ImGuiBegin> = OnceLock::new();
pub static IMGUI_BUTTON: OnceLock<ImGuiButton> = OnceLock::new();
pub static IMGUI_POS: OnceLock<ImGuiNextWindowPos> = OnceLock::new();

pub fn text<T: AsRef<str>>(text: T) {
    let s = text.as_ref();
    unsafe {
        let start = s.as_ptr();
        let end = start.add(s.len());
        if let Some(ddmk_info) = DDMK_INFO.get() {
            std::mem::transmute::<usize, ImGuiText>(
                *(ddmk_info.ddmk_address) + ddmk_info.text_addr,
            )(start as *const c_char, end as *const c_char);
        }
    }
}

pub fn get_imgui_end() -> &'static BasicNothingFunc {
    IMGUI_END.get_or_init(|| unsafe {
        let ddmk_info = DDMK_INFO.get().expect("DDMK_INFO not initialized");
        std::mem::transmute::<_, BasicNothingFunc>(*(ddmk_info.ddmk_address) + ddmk_info.end_addr)
    })
}

pub fn get_imgui_begin() -> &'static ImGuiBegin {
    IMGUI_BEGIN.get_or_init(|| unsafe {
        let ddmk_info = DDMK_INFO.get().expect("DDMK_INFO not initialized");
        std::mem::transmute::<_, ImGuiBegin>(*(ddmk_info.ddmk_address) + ddmk_info.begin_addr)
    })
}

pub fn get_imgui_button() -> &'static ImGuiButton {
    IMGUI_BUTTON.get_or_init(|| unsafe {
        let ddmk_info = DDMK_INFO.get().expect("DDMK_INFO not initialized");
        std::mem::transmute::<_, ImGuiButton>(*(ddmk_info.ddmk_address) + ddmk_info.button_addr)
    })
}

pub fn get_imgui_next_pos() -> &'static ImGuiNextWindowPos {
    IMGUI_POS.get_or_init(|| unsafe {
        let ddmk_info = DDMK_INFO.get().expect("DDMK_INFO not initialized");
        std::mem::transmute::<_, ImGuiNextWindowPos>(*(ddmk_info.ddmk_address) + ddmk_info.next_pos)
    })
}
