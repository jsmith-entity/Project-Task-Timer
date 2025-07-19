use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    text::Line,
    widgets::Widget,
};

use crate::task_timer::views::log::{
    filter::Filter,
    log_type::{InfoSubType, LogType},
    time_stamp::TimeStamp,
};

#[derive(Clone)]
pub struct LogEntry {
    pub log_type: LogType,
    pub time_stamp: TimeStamp,
    pub message: String,
}

impl Widget for &LogEntry {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let [info, _, content] = Layout::vertical([Length(1), Length(1), Min(0)]).areas(area);
        let [type_area, subtype_area] = Layout::horizontal([Min(0), Length(17)]).areas(info);

        let subtype = match self.log_type {
            LogType::INFO(subtype) => format!(" - {}", subtype.to_string()),
            _ => "".to_string(),
        };

        let log_heading = format!("{}{}", self.log_type.clone(), subtype);
        let color = self.log_type.color();
        Line::from(log_heading).fg(color).render(type_area, buf);

        let time = self.time_stamp.print();
        Line::from(time).render(subtype_area, buf);

        Line::from(format!("{}", self.message.clone())).render(content, buf);
    }
}

#[derive(Default)]
pub struct LogView {
    pub logs: Vec<LogEntry>,

    selected_filter: Filter,
}

impl LogView {
    pub fn new() -> Self {
        return Self {
            logs: Vec::new(),

            selected_filter: Filter::default(),
        };
    }

    pub fn prev_filter(&mut self) {
        self.selected_filter = self.selected_filter.prev();
    }

    pub fn next_filter(&mut self) {
        self.selected_filter = self.selected_filter.next();
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.logs.push(LogEntry {
            log_type,
            time_stamp: TimeStamp::new(),
            message: message.to_string(),
        });

        if self.logs.len() >= 40 {
            self.logs.remove(0);
        }
    }

    pub fn recent(&self) -> Vec<LogEntry> {
        const SIZE: usize = 15;

        let recent_log = if self.logs.len() >= SIZE {
            &self.logs[self.logs.len() - SIZE..]
        } else {
            &self.logs[..]
        };

        return recent_log.to_vec();
    }

    pub fn handle_events(&mut self, key_code: KeyCode) -> Result<InfoSubType, String> {
        match key_code {
            KeyCode::Char('h') => self.prev_filter(),
            KeyCode::Char('l') => self.next_filter(),
            _ => (),
        }

        return Ok(InfoSubType::None);
    }
}

impl Widget for &LogView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(2), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        self.selected_filter.render(header_area, buf);
        //self.render_tabs(frame, header_area);

        let dash_len: usize = 60;
        let dash_line = "-".repeat(dash_len);

        Line::from(dash_line.clone()).render(body_area, buf);

        const ENTRY_HEIGHT: u16 = 3;
        let mut total_height = 1;
        for entry in self.logs.iter().rev() {
            if !self.selected_filter.includes(entry.log_type) {
                continue;
            }

            let entry_area = Rect {
                x: body_area.x,
                y: body_area.y + total_height,
                width: body_area.width,
                height: ENTRY_HEIGHT,
            };

            total_height += entry_area.height;

            let separator_area = Rect {
                x: body_area.x,
                y: body_area.y + total_height,
                width: body_area.width,
                height: 1,
            };

            total_height += separator_area.height;

            entry.render(entry_area, buf);
            Line::from(dash_line.clone()).render(separator_area, buf);
        }
    }
}
