use crate::cache::{ChecksumError, DATA_PACKAGE};
use crate::item_sync::get_index;
use crate::mapping_utilities::GameConfig;
use crate::{cache, item_sync};
use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};
use archipelago_rs::protocol::{ClientMessage, Connected, GetDataPackage, ItemsHandlingFlags, RichMessageColor, RichMessagePart, RichPrint, ServerMessage};
use owo_colors::OwoColorize;
use serde_json::Value;
use std::fs::remove_file;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
use std::sync::{LazyLock, RwLock};
use tokio::sync::Mutex;

/// Current connections slot number
pub static SLOT_NUMBER: AtomicI64 = AtomicI64::new(-1);
pub static TEAM_NUMBER: AtomicI64 = AtomicI64::new(-1);
pub static CONNECTED: RwLock<Option<Connected<Value>>> = RwLock::new(None);
pub static CLIENT: LazyLock<Mutex<Option<ArchipelagoClient>>> = LazyLock::new(|| Mutex::new(None));

pub const DEATH_LINK: &str = "DeathLink";

pub struct DeathLinkData {
    pub cause: String,
}

pub fn handle_print_json(print_json: RichPrint, con_opt: &Option<Connected<Value>>) -> String {
    let mut final_message: String = "".to_string();
    match print_json {
        RichPrint::ItemSend {
            data,
            receiving: _receiving,
            item: _item,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::ItemCheat {
            data,
            receiving: _receiving,
            item: _item,
            team: _team,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Hint {
            data,
            receiving: _receiving,
            item: _item,
            found: _found,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Join {
            data,
            team: _team,
            slot: _slot,
            tags: _tags,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Part {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Chat {
            data,
            team: _team,
            slot: _slot,
            message: _message,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::ServerChat {
            data,
            message: _message,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Tutorial { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::TagsChanged {
            data,
            team: _team,
            slot: _slot,
            tags: _tags,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::CommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::AdminCommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Goal {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Release {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Collect {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Countdown {
            data,
            countdown: _countdown,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        RichPrint::Unknown { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
    }
    final_message
}

fn handle_message_part(message: RichMessagePart, con_opt: &Option<Connected<Value>>) -> String {
    match message {
        RichMessagePart::PlayerId { id, name } => match &con_opt {
            None => "<Connected is None>".to_string(),
            Some(con) => con.players[id as usize - 1].name.clone(),
        },
        RichMessagePart::PlayerName { text } => text,
        RichMessagePart::ItemId {
            id, flags, player, name
        } => {
            if let Some(data_package) = DATA_PACKAGE.read().unwrap().as_ref() {
                match con_opt {
                    None => "<Connected is None>".to_string(),
                    Some(con) => {
                        let game = &con.slot_info[&player].game.clone();
                        data_package.games
                            .get(game)
                            .unwrap()
                            .item_name_to_id
                            .get_by_right(&id)
                            .unwrap().to_string()
                    }
                }
            } else {
                "<Data package unavailable>".parse().unwrap()
            }
        }
        RichMessagePart::ItemName {
            text,
            flags,
            player,
        } => {
            log::debug!(
                "ItemName: {:?} Flags: {:?}, Player: {}",
                text,
                flags,
                player
            );
            text
        }
        RichMessagePart::LocationId { id, player, name } => {
            if let Some(data_package) = DATA_PACKAGE.read().unwrap().as_ref() {
                match con_opt {
                    None => "<Connected is None>".to_string(),
                    Some(con) => {
                        let game = &con.slot_info[&player].game.clone();
                        data_package
                            .games
                            .get(game)
                            .unwrap()
                            .location_name_to_id
                            .get_by_right(&id)
                            .unwrap().to_string()
                    }
                }
            } else {
                "<Data package unavailable>".parse().unwrap()
            }
        }
        RichMessagePart::LocationName { text, player } => {
            log::debug!("LocationName: {:?}, Player: {}", text, player);
            text
        }
        RichMessagePart::EntranceName { text } => text,
        RichMessagePart::Color { text, color } => {
            match color {
                // This looks ugly, but I'm too lazy to have a better idea
                RichMessageColor::Bold => text.bold().to_string(),
                RichMessageColor::Underline => text.underline().to_string(),
                RichMessageColor::Black => text.black().to_string(),
                RichMessageColor::Red => text.red().to_string(),
                RichMessageColor::Green => text.green().to_string(),
                RichMessageColor::Yellow => text.yellow().to_string(),
                RichMessageColor::Blue => text.blue().to_string(),
                RichMessageColor::Magenta => text.magenta().to_string(),
                RichMessageColor::Cyan => text.cyan().to_string(),
                RichMessageColor::White => text.white().to_string(),
                RichMessageColor::BlackBg => text.on_black().to_string(),
                RichMessageColor::RedBg => text.on_red().to_string(),
                RichMessageColor::GreenBg => text.on_green().to_string(),
                RichMessageColor::YellowBg => text.on_yellow().to_string(),
                RichMessageColor::BlueBg => text.on_blue().to_string(),
                RichMessageColor::MagentaBg => text.on_magenta().to_string(),
                RichMessageColor::CyanBg => text.on_cyan().to_string(),
                RichMessageColor::WhiteBg => text.on_white().to_string(),
            }
        }
        RichMessagePart::Text { text } => text,
    }
}

/// Get the Archipelago Client as well as ensuring that DataPackage checksums ar valid
pub async fn get_archipelago_client(url: &String) -> Result<ArchipelagoClient, ArchipelagoError> {
    if cache::check_for_cache_file() {
        // If the cache exists, then connect normally and verify the cache file
        let mut client = ArchipelagoClient::new(url).await?;
        match cache::find_checksum_errors(client.room_info()) {
            Ok(()) => {
                log::info!("Checksums check out!");
                Ok(client)
            }
            Err(err) => {
                match err.downcast::<ChecksumError>() {
                    Ok(checksum_error) => {
                        log::error!(
                            "Local DataPackage checksums for {:?} did not match expected values, reacquiring",
                            checksum_error.games
                        );
                        client
                            .send(ClientMessage::GetDataPackage(GetDataPackage {
                                games: Some(checksum_error.games),
                            }))
                            .await?;
                        match client.recv().await? {
                            Some(ServerMessage::DataPackage(pkg)) => {
                                cache::update_cache(&pkg.data).unwrap_or_else(|err| {
                                    log::error!("Failed to write cache: {}", err)
                                });
                            }
                            Some(received) => {
                                return Err(ArchipelagoError::IllegalResponse {
                                    received: received.type_name(),
                                    expected: "DataPackage",
                                });
                            }
                            None => return Err(ArchipelagoError::ConnectionClosed),
                        }
                        return Ok(client);
                    }
                    Err(err) => {
                        log::error!("Error checking DataPackage checksums: {:?}", err);
                        if let Err(err) = remove_file(cache::CACHE_FILENAME) {
                            log::error!("Failed to remove {}: {}", cache::CACHE_FILENAME, err);
                        };
                    }
                }
                Err(ArchipelagoError::ConnectionClosed)
            }
        }
    } else {
        // If the cache file does not exist, then it needs to be acquired
        let mut client = ArchipelagoClient::with_data_package(url, None).await?;
        client
            .send(ClientMessage::GetDataPackage(GetDataPackage {
                games: Some(client.room_info().games.clone()),
            }))
            .await?;
        match client.recv().await? {
            Some(ServerMessage::DataPackage(pkg)) => {
                cache::write_cache(&pkg.data).unwrap_or_else(|err| {
                    log::error!("Failed to write cache: {}", err)
                });
            }
            Some(received) => {
                return Err(ArchipelagoError::IllegalResponse {
                    received: received.type_name(),
                    expected: "DataPackage",
                });
            }
            None => return Err(ArchipelagoError::ConnectionClosed),
        }
        Ok(client)
    }
}

pub static CHECKED_LOCATIONS: LazyLock<RwLock<Vec<&'static str>>> =
    LazyLock::new(|| RwLock::new(vec![]));

/// Connects to a local proxy client via the provided url
pub async fn connect_local_archipelago_proxy<C: GameConfig>(
    url: String,
) -> Result<ArchipelagoClient, ArchipelagoError> {
    //log::info!("Attempting room connection with url: {url}");
    let mut ap_client = get_archipelago_client(&url).await?;
    let connected = ap_client
        .connect(
            C::GAME_NAME,
            "",
            None,
            ItemsHandlingFlags::all(),
            //Option::from(0b111),
            vec!["AP".to_string()],
        )
        .await?;
    match CONNECTED.write() {
        Ok(mut con) => {
            con.replace(connected);
        }
        Err(err) => {
            log::error!("Failed to acquire lock for connection: {}", err);
            return Err(ArchipelagoError::ConnectionClosed);
        }
    }

    match CHECKED_LOCATIONS.write() {
        Ok(mut checked_locations) => {
            checked_locations.clear();
        }
        Err(err) => {
            log::error!("Failed to get checked locations: {}", err);
            return Err(ArchipelagoError::ConnectionClosed);
        }
    }

    let index = get_index(
        &ap_client.room_info().seed_name,
        SLOT_NUMBER.load(Ordering::SeqCst),
    );
    item_sync::send_offline_checks(&mut ap_client, index)
        .await
        .unwrap();

    match CONNECTED.read().as_ref() {
        Ok(con) => {
            if let Some(con) = &**con {
                const DEBUG: bool = false;
                if DEBUG {
                    log::debug!("Connected info: {:?}", con);
                }
                SLOT_NUMBER.store(con.slot, Ordering::SeqCst);
                TEAM_NUMBER.store(con.team, Ordering::SeqCst);
            }
        }
        Err(err) => {
            log::error!("Failed to acquire lock for connection: {}", err);
            return Err(ArchipelagoError::ConnectionClosed);
        }
    }

    Ok(ap_client)
}
