use ratatui::{prelude::Stylize, style::Color, text::Line};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Serialize, Deserialize, Default, EnumIter, Display, Clone, Copy, FromRepr, PartialEq)]
pub enum InfoSubType {
    #[default]
    #[strum(to_string = "General")]
    General,
    #[strum(to_string = "Parent Entry")]
    EnterSubheading,
    #[strum(to_string = "Subheading Entry")]
    EnterParent,
    #[strum(to_string = "Complete Task")]
    CompleteTask,
    #[strum(to_string = "Uncomplete Task")]
    UncompleteTask,
    #[strum(to_string = "Starting Timer")]
    StartTimer,
    #[strum(to_string = "Stopping Timer")]
    StopTimer,
    #[strum(to_string = "Saving State")]
    Save,
    #[strum(to_string = "None")]
    None,
}

impl InfoSubType {
    pub fn message<T: ToString>(&self, info: T) -> String {
        use InfoSubType::*;
        return match self {
            EnterSubheading => format!("Entering subheading:  {}", info.to_string()),
            EnterParent => format!("Entering parent: {}", info.to_string()),
            CompleteTask => format!("Completing task: {}", info.to_string()),
            UncompleteTask => format!("Cancelling completion of task: {}", info.to_string()),
            StartTimer => format!("Starting time on task at line: {}", info.to_string()),
            StopTimer => format!("Stopping timer on task at line: {}", info.to_string()),
            Save => "Successfully saved the project".to_string(),
            _ => "erm".to_string(),
        }
        .to_string();
    }

    pub fn prev(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        return format!("  {self}  ").fg(Color::Black).bg(Color::DarkGray).into();
    }
}
