use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use std::time::Duration;

#[derive(Default, Clone)]
pub struct Timers {
    pub times: Vec<Duration>, // Positions correspond to the nodes content (task) array entries
    pub selected_line: u16,
    pub content_height: u16,
}

impl Timers {
    fn format_duration(duration: &Duration) -> String {
        let secs = duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
    }
}

impl Widget for &Timers {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for (idx, time) in self.times.iter().enumerate() {
            let mut style = Style::default();
            if idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(Timers::format_duration(time))
                .style(style)
                .render(display_area, buf);
        }
    }
}
