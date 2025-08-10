use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};

use crate::app::SessionState;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum PopupType {
    None,
    ConfirmQuit,
}

impl PopupType {
    pub fn handle_events(&self, key_code: KeyCode) -> SessionState {
        return match self {
            PopupType::ConfirmQuit => PopupType::confirm_quit(key_code),
            _ => SessionState::Running,
        };
    }

    fn confirm_quit(key_code: KeyCode) -> SessionState {
        return match key_code {
            KeyCode::Char('y') => SessionState::Quitting,
            KeyCode::Char('n') => SessionState::Running,
            _ => SessionState::AwaitingPrompt,
        };
    }
}
