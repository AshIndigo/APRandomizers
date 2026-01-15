use archipelago_rs::{LocatedItem, Print, RichText, TextColor};
use owo_colors::OwoColorize;

pub struct DeathLinkData {
    pub cause: String,
}

pub fn handle_print(print: Print) -> String {
    let mut final_message: String = "".to_string();
    match print {
        Print::ItemSend { data, item: _item } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::ItemCheat { data, item: _item } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Hint {
            data,
            item: _item,
            found: _found,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Join {
            data,
            player: _,
            tags: _,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Part { data, player: _ } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Chat {
            data,
            player: _,
            message: _,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::ServerChat {
            data,
            message: _message,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Tutorial { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::TagsChanged {
            data,
            player: _,
            tags: _,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::CommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::AdminCommandResult { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Goal { data, player: _ } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Release { data, player: _ } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Collect { data, player: _ } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Countdown {
            data,
            countdown: _countdown,
        } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
        Print::Unknown { data } => {
            for message in data {
                final_message.push_str(&handle_message_part(message));
            }
        }
    }
    final_message
}

fn handle_message_part(message: RichText) -> String {
    match message {
        RichText::PlayerName(text) => text,
        RichText::Item {
            item,
            player: _,
            progression: _,
            useful: _,
            trap: _,
        } => item.to_string(),
        RichText::Location {
            location,
            player: _,
        } => location.to_string(),
        RichText::EntranceName(text) => text,
        RichText::Color { text, color } => {
            match color {
                // This looks ugly, but I'm too lazy to have a better idea
                TextColor::Bold => text.bold().to_string(),
                TextColor::Underline => text.underline().to_string(),
                TextColor::Black => text.black().to_string(),
                TextColor::Red => text.red().to_string(),
                TextColor::Green => text.green().to_string(),
                TextColor::Yellow => text.yellow().to_string(),
                TextColor::Blue => text.blue().to_string(),
                TextColor::Magenta => text.magenta().to_string(),
                TextColor::Cyan => text.cyan().to_string(),
                TextColor::White => text.white().to_string(),
                TextColor::BlackBg => text.on_black().to_string(),
                TextColor::RedBg => text.on_red().to_string(),
                TextColor::GreenBg => text.on_green().to_string(),
                TextColor::YellowBg => text.on_yellow().to_string(),
                TextColor::BlueBg => text.on_blue().to_string(),
                TextColor::MagentaBg => text.on_magenta().to_string(),
                TextColor::CyanBg => text.on_cyan().to_string(),
                TextColor::WhiteBg => text.on_white().to_string(),
            }
        }
        RichText::Text(text) => text,
        RichText::Player(player) => player.to_string(),
    }
}

pub fn get_description(item: &LocatedItem) -> String {
    format!("{}'s {}", item.receiver().name(), item.item().name())
}
