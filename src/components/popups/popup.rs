use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

use crate::{components::Component, events::*};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum PopupType {
    None,
    ConfirmQuit,
}

impl Component for PopupType {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        // TODO: Popup events
        return Ok(EventState::NotConsumed);
    }
}
