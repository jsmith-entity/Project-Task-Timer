use ratatui::style::Color;

use serde::{Deserialize, Serialize};

use strum_macros::{Display, EnumIter};

use crate::info_subtype::InfoSubType;

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
