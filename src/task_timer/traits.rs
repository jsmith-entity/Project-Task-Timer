use crate::task_timer::log_type::InfoSubType;
use crossterm::event::KeyCode;

pub trait EventHandler {
    fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String>;
}
