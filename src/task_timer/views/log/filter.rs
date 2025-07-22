use ratatui::{
    prelude::{Buffer, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::{Tabs, Widget},
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

use crate::task_timer::views::log::log_type::*;

#[derive(Serialize, Deserialize, Default, EnumIter, Display, Clone, Copy, FromRepr, PartialEq)]
pub enum Filter {
    #[default]
    #[strum(to_string = "All")]
    ALL,
    #[strum(to_string = "Info")]
    INFO,
    #[strum(to_string = "Error")]
    ERROR,
}

impl Filter {
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

    fn title(self) -> Line<'static> {
        return format!("  {self}  ").fg(Color::Black).bg(Color::DarkGray).into();
    }

    pub fn includes(self, log_type: LogType) -> bool {
        let mut includes_filter = false;

        if self == Filter::ALL {
            includes_filter = true;
        } else if matches!(log_type, LogType::INFO(_)) && self == Filter::INFO {
            includes_filter = true;
        } else if log_type == LogType::ERROR && self == Filter::ERROR {
            includes_filter = true;
        }

        return includes_filter;
    }
}

impl Widget for &Filter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = Filter::iter().map(Filter::title);

        let selected_tab_idx = *self as usize;
        let highlight_style = (Color::Black, Color::Gray);

        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}
