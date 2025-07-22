use ratatui::style::Color;

use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Default, EnumIter, Clone, Copy, PartialEq, Display)]
pub enum InfoSubType {
    #[default]
    #[strum(to_string = "General")]
    General,
    #[strum(to_string = "Node Traverse")]
    EnterSubheading,
    #[strum(to_string = "Node Traverse")]
    EnterParent,
    #[strum(to_string = "Complete Task")]
    CompleteTask,
    #[strum(to_string = "Uncomplete Task")]
    UncompleteTask,
    #[strum(to_string = "Starting Timer")]
    StartTimer,
    #[strum(to_string = "Stopping Timer")]
    StopTimer,
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
            _ => "erm".to_string(),
        }
        .to_string();
    }
}

#[derive(Serialize, Deserialize, EnumIter, Display, Clone, Copy, PartialEq)]
pub enum LogType {
    #[strum(to_string = "INFO")]
    INFO(InfoSubType),
    #[strum(to_string = "ERROR")]
    ERROR,
}

impl LogType {
    pub fn color(&self) -> Color {
        return match self {
            LogType::INFO(_) => Color::Blue,
            LogType::ERROR => Color::Red,
        };
    }
}
