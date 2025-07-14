use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::text::Line;

use crate::task_timer::time_stamp::*;

pub struct LoggerView {
    pub recent_log: Vec<LogRecord>,
}

impl LoggerView {
    pub fn new() -> Self {
        return Self {
            recent_log: Vec::new(),
        };
    }

    pub fn draw(&self, frame: &mut Frame, area: &Rect) {
        let mut log_area = area.clone();

        for (time_stamp, info) in self.recent_log.iter().rev() {
            let time_line = Line::from(time_stamp.print());
            frame.render_widget(time_line, log_area);

            log_area.y += 1;

            let info_line = Line::from(format!(" {}", info.clone()));
            frame.render_widget(info_line, log_area);

            log_area.y += 1;
        }
    }
}
