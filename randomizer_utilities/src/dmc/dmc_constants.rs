use std::sync::LazyLock;

pub trait GameConfig {
    // TODO Either expand on this to reduce duped code between DMC games, or drop it?
    const REMOTE_ID: u32;
    const GAME_NAME: &'static str;
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
