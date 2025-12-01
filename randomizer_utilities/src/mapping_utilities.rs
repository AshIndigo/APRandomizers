use crate::archipelago_utilities::{CONNECTED, SLOT_NUMBER};
use crate::cache;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;

pub trait GameConfig {
    const REMOTE_ID: u32;
    const GAME_NAME: &'static str;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationData {
    // Item name, used for descriptions
    #[serde(default)]
    item_id: Option<i64>,
    // Slot ID for recipient
    owner: i64,
}

impl LocationData {
    fn is_item_remote(&self) -> bool {
        self.owner != SLOT_NUMBER.load(Ordering::SeqCst)
    }

    pub fn get_in_game_id<C: GameConfig>(&self) -> u32 {
        // Used for setting values in DMC1 and 3
        if self.is_item_remote() {
            C::REMOTE_ID
        } else {
            match self.item_id {
                None => 0,
                // For DMC1 This will need to be translated to a proper ID+Category
                Some(id) => id as u32,
            }
        }
    }

    pub fn get_item_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        //let player_name = get_slot_name(self.owner)?;
        if let Some(cache) = (*cache::DATA_PACKAGE).read()?.as_ref() {
            let game_name = &{
                match CONNECTED.read().as_ref() {
                    Ok(con) => match &**con {
                        None => return Err("Connected is None".into()),
                        Some(connected) => match connected.slot_info.get(&self.owner) {
                            None => {
                                return Err(format!("Missing slot info for {}", self.owner).into());
                            }
                            Some(info) => info.game.clone(),
                        },
                    },
                    Err(_err) => {
                        return Err("PoisonError occurred when getting 'Connected'".into());
                    }
                }
            };
            match self.item_id {
                None => Err("Item ID is None, cannot get name".into()),
                Some(item_id) => match cache.item_id_to_name.get(game_name) {
                    None => Err(format!("{} does not exist in cache", game_name).into()),
                    Some(item_id_to_name) => match item_id_to_name.get(&(item_id)) {
                        None => Err(format!(
                            "{:?} does not exist in {}'s item cache",
                            item_id, game_name
                        )
                            .into()),
                        Some(name) => Ok(name.clone()),
                    },
                },
            }
        } else {
            Err(Box::from("Data package is not here"))
        }
    }
    // Format a description
    pub fn get_description(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "{}'s {}",
            get_slot_name(self.owner)?,
            self.get_item_name()?
        ))
    }
}

pub fn get_own_slot_name() -> Result<String, Box<dyn std::error::Error>> {
    get_slot_name(SLOT_NUMBER.load(Ordering::SeqCst))
}

pub fn get_slot_name(slot: i64) -> Result<String, Box<dyn std::error::Error>> {
    let uslot = slot as usize;
    match CONNECTED.read() {
        Ok(conn_opt) => {
            if let Some(connected) = conn_opt.as_ref() {
                if slot == 0 {
                    return Ok("Server".to_string());
                }
                if (slot < 0) || (uslot - 1 >= connected.players.len()) {
                    return Err(format!("Slot index not valid: {}", slot).into());
                }
                Ok(connected.players[uslot - 1].name.clone())
            } else {
                Err("Not connected, cannot get name".into())
            }
        }
        Err(err) => Err(err.into()),
    }
}