use crossterm::event::KeyCode;

use crate::task_timer::{InfoSubType, session_manager::SessionState};

pub trait EventHandler {
    fn handle_events(&mut self, key_code: KeyCode) -> SessionState;
}

pub trait ViewEventHandler {
    fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String>;
}
