use crossterm::event::KeyCode;

use crate::app::SessionState;
use crate::info_subtype::InfoSubType;

pub trait EventHandler {
    fn handle_events(&mut self, key_code: KeyCode) -> SessionState;
}

pub trait ViewEventHandler {
    fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String>;
}
