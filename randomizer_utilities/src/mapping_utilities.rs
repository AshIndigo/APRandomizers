pub trait GameConfig {
    // TODO Either expand on this to reduce duped code between DMC games, or drop it?
    const REMOTE_ID: u32;
    const GAME_NAME: &'static str;
}
