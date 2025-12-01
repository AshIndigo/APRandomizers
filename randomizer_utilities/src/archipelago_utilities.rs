use crate::cache::{ChecksumError, DATA_PACKAGE};
use crate::item_sync::get_index;
use crate::mapping_utilities::GameConfig;
use crate::{cache, item_sync, mapping_utilities};
use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};
use archipelago_rs::protocol::{
    Bounce, ClientMessage, Connected, GetDataPackage, ItemsHandlingFlags, JSONColor,
    JSONMessagePart, PrintJSON, ServerMessage,
};
use owo_colors::OwoColorize;
use serde_json::{json, Value};
use std::fs::remove_file;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
use std::sync::{LazyLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
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

/// Sends a Bounce packet containing DeathLink Info as defined by the DeathLinkData struct
pub async fn send_deathlink_message(
    client: &mut ArchipelagoClient,
    data: DeathLinkData,
) -> Result<(), ArchipelagoError> {
    let name = mapping_utilities::get_own_slot_name().unwrap();
    client
        .send(ClientMessage::Bounce(Bounce {
            games: Some(vec![]),
            slots: Some(vec![]),
            tags: Some(vec![DEATH_LINK.to_string()]),
            data: json!({
                "time": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f32(),
                "source": name,
                "cause": data.cause
            }),
        }))
        .await?;
    Ok(())
}

pub fn handle_print_json(print_json: PrintJSON, con_opt: &Option<Connected<Value>>) -> String {
    let mut final_message: String = "".to_string();
    match print_json {
        PrintJSON::ItemSend {
            data,
            receiving: _receiving,
            item: _item,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::ItemCheat {
            data,
            receiving: _receiving,
            item: _item,
            team: _team,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Hint {
            data,
            receiving: _receiving,
            item: _item,
            found: _found,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Join {
            data,
            team: _team,
            slot: _slot,
            tags: _tags,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Part {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Chat {
            data,
            team: _team,
            slot: _slot,
            message: _message,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::ServerChat {
            data,
            message: _message,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Tutorial { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::TagsChanged {
            data,
            team: _team,
            slot: _slot,
            tags: _tags,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::CommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::AdminCommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Goal {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Release {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Collect {
            data,
            team: _team,
            slot: _slot,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Countdown {
            data,
            countdown: _countdown,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
        PrintJSON::Unknown { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message, con_opt));
            }
        }
    }
    final_message
}

fn handle_message_part(message: JSONMessagePart, con_opt: &Option<Connected<Value>>) -> String {
    match message {
        JSONMessagePart::PlayerId { text, player } => match &con_opt {
            None => "<Connected is None>".to_string(),
            Some(con) => con.players[text.parse::<usize>().unwrap() - 1].name.clone(),
        },
        JSONMessagePart::PlayerName { text } => text,
        JSONMessagePart::ItemId {
            text,
            flags: _flags,
            player,
        } => {
            if let Some(data_package) = DATA_PACKAGE.read().unwrap().as_ref() {
                match con_opt {
                    None => "<Connected is None>".to_string(),
                    Some(con) => {
                        let game = &con.slot_info[&player].game.clone();
                        data_package
                            .item_id_to_name
                            .get(game)
                            .unwrap()
                            .get(&text.parse::<i64>().unwrap())
                            .unwrap()
                            .clone()
                    }
                }
            } else {
                "<Data package unavailable>".parse().unwrap()
            }
        }
        JSONMessagePart::ItemName {
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
        JSONMessagePart::LocationId { text, player } => {
            if let Some(data_package) = DATA_PACKAGE.read().unwrap().as_ref() {
                match con_opt {
                    None => "<Connected is None>".to_string(),
                    Some(con) => {
                        let game = &con.slot_info[&player].game.clone();
                        data_package
                            .location_id_to_name
                            .get(game)
                            .unwrap()
                            .get(&text.parse::<i64>().unwrap())
                            .unwrap()
                            .clone()
                    }
                }
            } else {
                "<Data package unavailable>".parse().unwrap()
            }
        }
        JSONMessagePart::LocationName { text, player } => {
            log::debug!("LocationName: {:?}, Player: {}", text, player);
            text
        }
        JSONMessagePart::EntranceName { text } => text,
        JSONMessagePart::Color { text, color } => {
            match color {
                // This looks ugly, but I'm too lazy to have a better idea
                JSONColor::Bold => text.bold().to_string(),
                JSONColor::Underline => text.underline().to_string(),
                JSONColor::Black => text.black().to_string(),
                JSONColor::Red => text.red().to_string(),
                JSONColor::Green => text.green().to_string(),
                JSONColor::Yellow => text.yellow().to_string(),
                JSONColor::Blue => text.blue().to_string(),
                JSONColor::Magenta => text.magenta().to_string(),
                JSONColor::Cyan => text.cyan().to_string(),
                JSONColor::White => text.white().to_string(),
                JSONColor::BlackBg => text.on_black().to_string(),
                JSONColor::RedBg => text.on_red().to_string(),
                JSONColor::GreenBg => text.on_green().to_string(),
                JSONColor::YellowBg => text.on_yellow().to_string(),
                JSONColor::BlueBg => text.on_blue().to_string(),
                JSONColor::MagentaBg => text.on_magenta().to_string(),
                JSONColor::CyanBg => text.on_cyan().to_string(),
                JSONColor::WhiteBg => text.on_white().to_string(),
            }
        }
        JSONMessagePart::Text { text } => text,
    }
}

/// Get the Archipelago Client as well as ensuring that DataPackage checksums ar valid
pub async fn get_archipelago_client(url: &String) -> Result<ArchipelagoClient, ArchipelagoError> {
    if cache::check_for_cache_file() {
        // If the cache exists, then connect normally and verify the cache file
        let mut client = ArchipelagoClient::new(&url).await?;
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
                                    received: &received.type_name(),
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
        // If the cache file does not exist, then it needs to be acquired (TODO Need to update this call)
        let client = ArchipelagoClient::with_data_package(&url, None).await?;
        match &client.data_package() {
            // Write the data package to a local cache file
            None => {
                log::error!("No data package found");
                Err(ArchipelagoError::ConnectionClosed)
            }
            Some(dp) => {
                cache::write_cache(&dp)
                    .await
                    .unwrap_or_else(|err| log::error!("Failed to write cache: {}", err));
                Ok(client)
            }
        }
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
