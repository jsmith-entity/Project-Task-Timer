use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    text::Line,
    widgets::{Tabs, Widget},
};

use serde::{Deserialize, Serialize};

use crate::task_timer::{
    InfoSubType,
    log_type::*,
    traits::ViewEventHandler,
    views::{
        log::{filter::Filter, subfilter::SubFilter, time_stamp::TimeStamp},
        paginator::Paginator,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub log_type: LogType,
    pub time_stamp: TimeStamp,
    pub message: String,
}

impl Widget for &LogEntry {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let [info, content] = Layout::vertical([Length(1), Min(0)]).areas(area);
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

#[derive(Serialize, Deserialize, Default)]
pub struct LogView {
    pub logs: Vec<LogEntry>,

    selected_filter: Filter,
    selected_subfilter: Option<SubFilter>,
    paginator: Paginator,
}

impl LogView {
    pub fn new() -> Self {
        return Self {
            logs: Vec::new(),

            selected_filter: Filter::default(),
            selected_subfilter: None,
            paginator: Paginator {
                page: 0,
                page_size: 8,
                entry_len: 0,
            },
        };
    }

    pub fn log(&mut self, message: &str, log_type: LogType) {
        self.logs.push(LogEntry {
            log_type,
            time_stamp: TimeStamp::new(),
            message: message.to_string(),
        });

        self.paginator.entry_len = self.logs.len();

        if self.logs.len() >= 100 {
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

    fn prev_filter(&mut self) {
        self.selected_filter = self.selected_filter.prev();
        if self.selected_filter == Filter::INFO {
            self.selected_subfilter = Some(SubFilter {
                info_type: InfoSubType::General,
                selected: true,
            });
        } else {
            self.selected_subfilter = None;
        }
    }

    fn next_filter(&mut self) {
        self.selected_filter = self.selected_filter.next();
        if self.selected_filter == Filter::INFO {
            self.selected_subfilter = Some(SubFilter {
                info_type: InfoSubType::General,
                selected: true,
            });
        } else {
            self.selected_subfilter = None;
        }
    }

    fn render_log_page(&self, area: Rect, buf: &mut Buffer) {
        let (start_idx, end_idx) = self.paginator.page_slice();

        let dash_line = "-".repeat(area.width as usize);

        Line::from(dash_line.clone()).render(area, buf);

        const ENTRY_HEIGHT: u16 = 2;
        let mut total_height = 1;
        for entry in self.logs[start_idx..end_idx].iter().rev() {
            if !self.selected_filter.includes(entry.log_type) {
                continue;
            }

            let entry_area = Rect {
                x: area.x,
                y: area.y + total_height,
                width: area.width,
                height: ENTRY_HEIGHT,
            };

            entry.render(entry_area, buf);
            total_height += entry_area.height;

            let separator_area = Rect {
                x: area.x,
                y: area.y + total_height,
                width: area.width,
                height: 1,
            };

            total_height += separator_area.height;

            Line::from(dash_line.clone()).render(separator_area, buf);
        }
    }

    fn present_subfilters(&self) -> Vec<SubFilter> {
        let mut found_subtypes: Vec<SubFilter> = Vec::new();

        for log in self.logs.iter() {
            if let LogType::INFO(sub_type) = log.log_type {
                let selected = sub_type == self.selected_subfilter.unwrap().info_type;

                let entry = SubFilter {
                    info_type: sub_type,
                    selected,
                };

                if !found_subtypes.contains(&entry) {
                    found_subtypes.push(entry);
                }
            }
        }

        return found_subtypes;
    }
}

impl ViewEventHandler for LogView {
    fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String> {
        match key_code {
            KeyCode::Char('h') => self.prev_filter(),
            KeyCode::Char('l') => self.next_filter(),
            KeyCode::Char('j') => self.paginator.next_page(),
            KeyCode::Char('k') => self.paginator.prev_page(),
            _ => (),
        }

        return Ok((InfoSubType::None, "erm".to_string()));
    }
}

impl Widget for &LogView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(2), Length(2), Min(0), Length(1)]);
        let [filter_area, subfilter_area, body_area, page_area] = vertical.areas(area);

        self.selected_filter.render(filter_area, buf);
        if self.selected_subfilter.is_some() {
            SubFilter::render_tabs(subfilter_area, buf, &self.present_subfilters());
        }

        self.paginator.render(page_area, buf);
        self.render_log_page(body_area, buf);
    }
}
