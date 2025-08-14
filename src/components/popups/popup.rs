use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

use crate::events::*;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum PopupType {
    None,
    ConfirmQuit,
}

impl PopupType {
    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        // TODO: Popup events
        return Ok(EventState::NotConsumed);
    }
}
