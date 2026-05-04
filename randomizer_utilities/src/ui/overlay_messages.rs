use crate::ui::dx11_state::D3D11State;
use crate::ui::font_handler;
use crate::ui::font_handler::FontColorCB;
use archipelago_rs::LocatedItem;
use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

static MESSAGE_QUEUE: LazyLock<Mutex<VecDeque<OverlayMessage>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));
pub static ACTIVE_MESSAGES: LazyLock<Mutex<VecDeque<TimedMessage>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

pub struct MessageSegment {
    pub text: String,
    pub color: FontColorCB,
}

impl MessageSegment {
    pub fn new(text: String, color: FontColorCB) -> Self {
        Self { text, color }
    }
}

pub struct OverlayMessage {
    segments: Vec<MessageSegment>,
    duration: Duration,
    _x: f32,
    _y: f32,
    _msg_type: MessageType,
}

impl OverlayMessage {
    pub fn new(
        segments: Vec<MessageSegment>,
        duration: Duration,
        x: f32,
        y: f32,
        msg_type: MessageType,
    ) -> OverlayMessage {
        OverlayMessage {
            segments,
            duration,
            _x: x,
            _y: y,
            _msg_type: msg_type,
        }
    }
}

// TODO This doesn't matter right now, but it could be used later
pub enum MessageType {
    _Default,     // Take the X and Y values as they are given
    Notification, // Disregard coordinates, automatically align to upper right (Used for newly received items+DL)
}

pub fn add_message(overlay: OverlayMessage) {
    match MESSAGE_QUEUE.lock() {
        Ok(mut queue) => {
            queue.push_back(overlay);
        }
        Err(err) => {
            log::error!("PoisonError upon trying to add message {:?}", err);
        }
    }
}

pub fn draw_colored_message(
    state: &D3D11State,
    msg: &TimedMessage,
    screen_width: f32,
    screen_height: f32,
    y: f32,
) {
    const FALLBACK_MULT: f32 = 32.0;
    let total_width: f32 = msg
        .message
        .segments
        .iter()
        .map(|seg| {
            if let Some(atlas) = &state.atlas {
                seg.text
                    .chars()
                    .map(|c| atlas.glyph_advance(c))
                    .sum::<f32>()
            } else {
                seg.text.len() as f32 * FALLBACK_MULT
            }
        })
        .sum();

    // Start on right
    let mut cursor_x = screen_width - total_width;

    for segment in msg.message.segments.iter() {
        font_handler::draw_string(
            state,
            &segment.text,
            cursor_x,
            y,
            screen_width,
            screen_height,
            &segment.color,
        );

        if let Some(atlas) = &state.atlas {
            cursor_x += segment
                .text
                .chars()
                .map(|c| atlas.glyph_advance(c))
                .sum::<f32>();
        } else {
            cursor_x += segment.text.len() as f32 * FALLBACK_MULT;
        }
    }
}

pub struct TimedMessage {
    message: OverlayMessage,
    pub expiration: Instant,
}

pub fn pop_buffer_message() {
    if let Ok(mut queue) = MESSAGE_QUEUE.lock()
        && let Some(message) = queue.pop_front()
    {
        let expiration = Instant::now() + message.duration;
        let timed = TimedMessage {
            message,
            expiration,
        };
        if let Ok(mut active) = ACTIVE_MESSAGES.lock() {
            active.push_back(timed);
        }
    }
}

pub fn get_color_for_item(item: &LocatedItem) -> FontColorCB {
    const CYAN: FontColorCB = FontColorCB::new(0.0, 0.933, 0.933, 1.0);
    const PLUM: FontColorCB = FontColorCB::new(0.686, 0.6, 0.937, 1.0);
    const STATE_BLUE: FontColorCB = FontColorCB::new(0.427, 0.545, 0.91, 1.0);
    const SALMON: FontColorCB = FontColorCB::new(0.98, 0.502, 0.447, 1.0);

    match (item.is_trap(), item.is_useful(), item.is_progression()) {
        (true, _, _) => SALMON,
        (false, _, true) => PLUM,
        (false, true, false) => STATE_BLUE,
        (false, false, false) => CYAN,
    }
}
