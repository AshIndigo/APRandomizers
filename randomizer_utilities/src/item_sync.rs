use archipelago_rs::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::AtomicI64;

// Note this is all tailored for DMC games
const SYNC_FILE_NAME: &str = "archipelago.json";
pub static CURRENT_INDEX: AtomicI64 = AtomicI64::new(0);

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct SlotSyncInfo {
    // Index for each save slot
    pub sync_index: [i64; 10],
    pub offline_checks: Vec<i64>,
}

pub fn write_sync_data_file<S: DeserializeOwned + 'static>(
    data: SlotSyncInfo,
    client: &Client<S>,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(format!(
        "{}{}",
        crate::get_room_path(client)?,
        SYNC_FILE_NAME
    ))?;
    log::debug!("Writing sync file");
    file.write_all(serde_json::to_string_pretty(&data)?.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn check_for_sync_file<S: DeserializeOwned + 'static>(client: &Client<S>) -> bool {
    Path::new(&format!(
        "{}{}",
        crate::get_room_path(client).unwrap_or_else(|err| {
            log::info!("Failed to check for sync file: {}", err);
            "unknown".parse().unwrap()
        }),
        SYNC_FILE_NAME
    ))
    .try_exists()
    .unwrap_or_else(|err| {
        log::info!("Failed to check for sync file: {}", err);
        false
    })
}

/// Reads the received items indices from the save file
pub fn read_save_data<S: DeserializeOwned + 'static>(
    client: &Client<S>,
) -> Result<SlotSyncInfo, Box<dyn Error>> {
    if !check_for_sync_file(client) {
        Ok(SlotSyncInfo::default())
    } else {
        let file = File::open(format!(
            "{}{}",
            crate::get_room_path(client)?,
            SYNC_FILE_NAME
        ))?;
        let save_data = SlotSyncInfo::deserialize(&mut serde_json::Deserializer::from_reader(
            BufReader::new(file),
        ))?;
        Ok(save_data)
    }
}

pub static OFFLINE_CHECKS: Mutex<Vec<i64>> = Mutex::new(Vec::new());
pub fn add_offline_check(location: i64) {
    OFFLINE_CHECKS.lock().unwrap().push(location);
}

pub fn send_offline_checks<T: DeserializeOwned>(
    client: &mut Client<T>,
) -> Result<(), Box<dyn Error>> {
    log::debug!("Attempting to send any offline checks");
    let mut sync_data = read_save_data(client)?;

    match client.mark_checked(sync_data.offline_checks.clone()) {
        Ok(_) => {
            log::info!("Successfully sent offline checks");
            sync_data.offline_checks.clear();
            write_sync_data_file(sync_data, client)?;
        }
        Err(err) => {
            log::error!(
                "Failed to send offline checks, will attempt next reconnection: {}",
                err
            );
        }
    }

    Ok(())
}
