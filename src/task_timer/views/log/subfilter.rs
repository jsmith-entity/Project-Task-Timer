use ratatui::{
    prelude::{Buffer, Rect},
    style::Color,
    widgets::{Tabs, Widget},
};

use serde::{Deserialize, Serialize};

use crate::task_timer::InfoSubType;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
pub struct SubFilter {
    pub info_type: InfoSubType,
    pub selected: bool,
}

impl SubFilter {
    pub fn render_tabs(area: Rect, buf: &mut Buffer, available_filters: &Vec<SubFilter>) {
        let filter_iter = available_filters.iter().map(|e| e.info_type.title());

        let selected_filter_idx = available_filters.iter().position(|e| e.selected);
        let highlight_style = (Color::Black, Color::Gray);

        Tabs::new(filter_iter)
            .highlight_style(highlight_style)
            .select(selected_filter_idx)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}
