use archipelago_rs::protocol::{DataPackageObject, RoomInfo};
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{LazyLock, RwLock};

pub const CACHE_FILENAME: &str = "cache.json";

/// Checks for the Archipelago RoomInfo cache file
/// If file exists then check the checksums in it
/// Returns false if file doesn't exist (or if it cant be checked for)
pub fn check_for_cache_file() -> bool {
    Path::new(CACHE_FILENAME)
        .try_exists()
        .unwrap_or_else(|err| {
            log::info!("Failed to check for cache file: {}", err);
            false
        })
}


#[derive(Debug, Clone)]
pub struct ChecksumError {
    pub games: Vec<String>,
}

impl Display for ChecksumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} have invalid data package checksums", self.games)
    }
}

impl Error for ChecksumError {}
/// Check the cached checksums with the stored file. Return any that do not match
pub fn find_checksum_errors(room_info: &RoomInfo) -> Result<(), Box<dyn Error>> {
    let data_package_object = DataPackageObject::deserialize(
        &mut serde_json::Deserializer::from_reader(BufReader::new(File::open(CACHE_FILENAME)?)),
    )?;
    let mut failed_checks = vec![];
    for name in &room_info.games {
        // For all games in room
        if data_package_object.games.contains_key(name) {
            // See if cache file has game
            if room_info
                .datapackage_checksums
                .get(name)
                .ok_or(format!("{} is not in RoomInfo DataPackage Checksums", name))?
                != &data_package_object
                    .games
                    .get(name)
                    .ok_or(format!("{} is not in local cache file", name))?
                    .checksum
            {
                failed_checks.push(name.clone()); // Checksums do not match
            }
        } else {
            failed_checks.push(name.clone()); // Cache file is missing game
        }
    }
    if !failed_checks.is_empty() {
        return Err(ChecksumError {
            games: failed_checks,
        }.into());
    }
    Ok(())
}

/// Write the DataPackage to a JSON file
pub fn write_cache(data: &DataPackageObject) -> Result<(), Box<dyn Error>> {
    fs::write(
        CACHE_FILENAME,
        serde_json::to_string_pretty(&data)?.as_bytes(),
    )?;
    Ok(())
}

pub fn read_cache() -> Result<DataPackageObject, Box<dyn Error>> {
    let cache = DataPackageObject::deserialize(&mut serde_json::Deserializer::from_reader(
        BufReader::new(File::open(CACHE_FILENAME)?),
    ))?;
    Ok(cache)
}

pub static DATA_PACKAGE: LazyLock<RwLock<Option<DataPackageObject>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn set_data_package(value: DataPackageObject) -> Result<(), Box<dyn Error>> {
    *DATA_PACKAGE.write()? = Some(value);
    Ok(())
}

pub fn update_cache(data: &DataPackageObject) -> Result<(), Box<dyn Error>> {
    let mut current_cache = read_cache()?;
    for (game, data) in data.games.iter() {
        current_cache.games.insert(game.clone(), data.clone());
    }
    fs::write(
        CACHE_FILENAME,
        serde_json::to_string_pretty(&current_cache)?.as_bytes(),
    )?;
    Ok(())
}