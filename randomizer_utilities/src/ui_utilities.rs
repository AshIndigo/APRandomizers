#[derive(Copy, Clone, strum_macros::Display, strum_macros::FromRepr)]
pub enum Status {
    Disconnected = 0,
    Connected = 1,
    InvalidSlot = 2,
    InvalidGame = 3,
    IncompatibleVersion = 4,
    InvalidPassword = 5,
    InvalidItemHandling = 6,
}

impl From<Status> for isize {
    fn from(value: Status) -> Self {
        match value {
            Status::Disconnected => 0,
            Status::Connected => 1,
            Status::InvalidSlot => 2,
            Status::InvalidGame => 3,
            Status::IncompatibleVersion => 4,
            Status::InvalidPassword => 5,
            Status::InvalidItemHandling => 6,
        }
    }
}

impl From<isize> for Status {
    fn from(value: isize) -> Self {
        match value {
            0 => Status::Disconnected,
            1 => Status::Connected,
            2 => Status::InvalidSlot,
            3 => Status::InvalidGame,
            4 => Status::IncompatibleVersion,
            5 => Status::InvalidPassword,
            6 => Status::InvalidItemHandling,
            _ => Status::Disconnected,
        }
    }
}

pub fn get_status_text(connection_status: isize) -> &'static str {
    match connection_status.into() {
        Status::Connected => "Connected",
        Status::Disconnected => "Disconnected",
        Status::InvalidSlot => "Invalid slot (Check name)",
        Status::InvalidGame => "Invalid game (Wrong url/port or name?)",
        Status::IncompatibleVersion => "Incompatible Version, post on GitHub or Discord",
        Status::InvalidPassword => "Invalid password",
        Status::InvalidItemHandling => "Invalid item handling, post on Github or Discord",
    }
}