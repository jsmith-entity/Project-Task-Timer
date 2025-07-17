use ratatui::{
    Frame,
    prelude::{Constraint, Layout, Rect},
    text::Line,
};

use super::super::logger::LogRecord;

#[derive(Default)]
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
        let dash_len: usize = 60;
        let dash_line = "-".repeat(dash_len);

        let mut log_area = Rect::new(area.x, area.y, dash_len as u16, area.height);

        use Constraint::{Length, Min};
        for entry in self.recent_log.iter().rev() {
            let areas = Layout::vertical([Length(1), Min(0)]).split(log_area);
            let info = Layout::horizontal([Min(0), Length(17)]).split(areas[0]);

            frame.render_widget(Line::from(entry.log_type.clone().title()), info[0]);
            frame.render_widget(Line::from(entry.time_stamp.print()), info[1]);

            log_area.y += 2;

            let info_line = Line::from(format!("{}", entry.message.clone()));
            frame.render_widget(info_line, log_area);

            log_area.y += 1;

            frame.render_widget(Line::from(dash_line.clone()), log_area);
            log_area.y += 1;
        }
    }
}
