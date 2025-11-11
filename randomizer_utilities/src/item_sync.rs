use archipelago_rs::client::ArchipelagoClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::atomic::{AtomicI32};
use std::sync::{Mutex, OnceLock};

const SYNC_FILE: &str = "archipelago.json";
pub static SYNC_DATA: OnceLock<Mutex<SyncData>> = OnceLock::new();
pub static CURRENT_INDEX: AtomicI32 = AtomicI32::new(0);

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SyncData {
    pub room_sync_info: HashMap<String, RoomSyncInfo>, // String is seed
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct RoomSyncInfo {
    pub sync_index: i32,
    pub offline_checks: Vec<i64>,
}

pub fn get_sync_data() -> &'static Mutex<SyncData> {
    SYNC_DATA.get_or_init(|| Mutex::new(SyncData::default()))
}

pub fn write_sync_data_file() -> Result<(), Box<dyn Error>> {
    let mut file = File::create(SYNC_FILE)?;
    log::debug!("Writing sync file");
    file.write_all(
        serde_json::to_string_pretty(&SYNC_DATA.get().expect("Failed to get sync data"))?
            .as_bytes(),
    )?;
    file.flush()?;
    Ok(())
}

pub fn check_for_sync_file() -> bool {
    Path::new(SYNC_FILE).try_exists().unwrap_or_else(|err| {
        log::info!("Failed to check for sync file: {}", err);
        false
    })
}

/// Reads the received items indices from the save file
pub fn read_save_data() -> Result<SyncData, Box<dyn Error>> {
    if !check_for_sync_file() {
        Ok(SyncData::default())
    } else {
        let save_data = SyncData::deserialize(&mut serde_json::Deserializer::from_reader(
            BufReader::new(File::open(SYNC_FILE)?),
        ))?;
        Ok(save_data)
    }
}

pub fn get_index(seed_name: &String, slot_number: i32) -> String {
    format!(
        "{}_{}",
        seed_name,
        slot_number
    )
}

/// Adds an offline location to be sent when room connection is restored
pub async fn add_offline_check(
    location: i64,
    index: String
) -> Result<(), Box<dyn Error>> {
    let mut sync_data = get_sync_data().lock()?;
    if sync_data.room_sync_info.contains_key(&index) {
        sync_data
            .room_sync_info
            .get_mut(&index)
            .unwrap()
            .offline_checks
            .push(location);
    } else {
        sync_data
            .room_sync_info
            .insert(index, RoomSyncInfo::default());
    }
    write_sync_data_file()?;
    Ok(())
}

pub async fn send_offline_checks(
    client: &mut ArchipelagoClient,
    index: String
) -> Result<(), Box<dyn Error>> {
    log::debug!("Attempting to send offline checks");
    let mut sync_data = get_sync_data().lock()?;
    if sync_data.room_sync_info.contains_key(&index) {
        match client
            .location_checks(
                sync_data
                    .room_sync_info
                    .get(&index)
                    .unwrap()
                    .offline_checks
                    .clone(),
            )
            .await
        {
            Ok(_) => {
                log::info!("Successfully sent offline checks");
                sync_data
                    .room_sync_info
                    .get_mut(&index)
                    .unwrap()
                    .offline_checks
                    .clear();
                write_sync_data_file()?;
            }
            Err(err) => {
                log::error!(
                    "Failed to send offline checks, will attempt next reconnection: {}",
                    err
                );
            }
        }
    }
    Ok(())
}
