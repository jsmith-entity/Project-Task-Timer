use ratatui::{
    prelude::{Buffer, Rect},
    style::Style,
    text::Line,
    widgets::Widget,
};

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Task {
    pub name: String,
    pub duration: Duration,
    pub completed: bool,

    pub style: Style,
}

impl Task {
    fn format_duration(&self) -> String {
        let secs = self.duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
    }
}

impl Widget for &Task {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let duration = self.format_duration();

        let task = format!("{} {}", duration, self.name);

        Line::from(task).style(self.style).render(area, buf);
    }
}
