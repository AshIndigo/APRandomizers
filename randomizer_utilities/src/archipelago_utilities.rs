use std::sync::atomic::AtomicI32;
use std::sync::{LazyLock, RwLock};
use tokio::sync::Mutex;
use archipelago_rs::client::ArchipelagoClient;
use archipelago_rs::protocol::Connected;

/// Current connections slot number
pub static SLOT_NUMBER: AtomicI32 = AtomicI32::new(-1);
pub static TEAM_NUMBER: AtomicI32 = AtomicI32::new(-1);
pub static CONNECTED: RwLock<Option<Connected>> = RwLock::new(None);
pub static CLIENT: LazyLock<Mutex<Option<ArchipelagoClient>>> =
    LazyLock::new(|| Mutex::new(None));