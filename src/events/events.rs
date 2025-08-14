use crossterm::{event, event::KeyEvent};

use std::time::Duration;

// TODO: threaded
pub fn next_key_event(tick_rate: Duration) -> Option<KeyEvent> {
    if event::poll(tick_rate).unwrap() {
        if let Ok(ev) = event::read() {
            return ev.as_key_event();
        } else {
            return None;
        }
    } else {
        return None;
    }
}
