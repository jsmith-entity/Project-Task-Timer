use ratatui::style::Color;

use strum_macros::{Display, EnumIter};

#[derive(Default, EnumIter, Clone, Copy, PartialEq, Display)]
pub enum InfoSubType {
    #[default]
    #[strum(to_string = "General")]
    General,
    #[strum(to_string = "Node Traverse")]
    EnterSubheading,
    #[strum(to_string = "Node Traverse")]
    EnterParent,
    #[strum(to_string = "None")]
    None,
}

impl InfoSubType {
    pub fn message(&self) -> String {
        use InfoSubType::*;
        return match self {
            EnterSubheading => "Entering subheading node",
            EnterParent => "Entering parent node",
            _ => "erm",
        }
        .to_string();
    }
}

#[derive(EnumIter, Display, Clone, Copy, PartialEq)]
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
