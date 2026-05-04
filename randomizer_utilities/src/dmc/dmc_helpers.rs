use crate::ui::dx11_types::PresentFn;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct OverlayHandler {
    pub create_device_addr: usize,
    pub present_fn: PresentFn,
}

pub struct DDMKHandler {
    pub ddmk_address: LazyLock<usize>,
    pub main_func_addr: usize,
    pub timestep_func_addr: usize,
    pub ddmk_ui_enabled: usize,
    pub hooked_render: usize,
    pub text_addr: usize,
    pub end_addr: usize,
    pub begin_addr: usize,
    pub button_addr: usize,
    pub next_pos: usize,
}
