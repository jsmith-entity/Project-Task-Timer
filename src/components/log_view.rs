use crate::log_type::LogType;

use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::{Tabs, Widget},
};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

use crate::{config::KeyConfig, events::*, info_subtype::InfoSubType};

use super::Paginator;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeStamp {
    day: u32,
    month: String,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl TimeStamp {
    pub fn new() -> Self {
        let current_time = Local::now();
        return Self {
            day: current_time.day(),
            month: current_time.format("%B").to_string(),
            hours: current_time.hour(),
            minutes: current_time.minute(),
            seconds: current_time.second(),
        };
    }

    pub fn print(&self) -> String {
        return format!(
            "{} {}: {:02}:{:02}:{:02}",
            self.day, self.month, self.hours, self.minutes, self.seconds
        );
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct LogView {
    pub logs: Vec<LogEntry>,

    selected_filter: Filter,
    selected_subfilter: Option<SubFilter>,
    available_subfilters: Vec<SubFilter>,
    paginator: Paginator,
    #[serde(skip)]
    pub key_config: KeyConfig,
}

impl LogView {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            logs: Vec::new(),

            selected_filter: Filter::default(),
            selected_subfilter: None,
            available_subfilters: Vec::new(),
            paginator: Paginator {
                page: 0,
                page_size: 8,
                entry_len: 0,
            },
            key_config,
        };
    }

    pub fn update(&mut self) {
        self.available_subfilters = self.available_subfilters();
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

    fn available_subfilters(&self) -> Vec<SubFilter> {
        let mut found_subtypes: Vec<SubFilter> = Vec::new();

        if self.selected_subfilter.is_none() {
            return found_subtypes;
        }

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

    fn prev_subfilter(&mut self) {
        if self.selected_subfilter.is_none() {
            return;
        }

        let current_subfilter = self.selected_subfilter.unwrap().info_type;
        let found_idx = self
            .available_subfilters
            .iter()
            .position(|e| e.info_type == current_subfilter);

        if found_idx.is_some() {
            let prev_idx = if found_idx.unwrap() == 0 {
                self.available_subfilters.len() - 1
            } else {
                found_idx.unwrap() - 1
            };
            self.selected_subfilter = Some(self.available_subfilters[prev_idx]);
        } else {
            self.log(
                "Could not select previous subfilter. Current subfilter not found",
                LogType::ERROR,
            );
        }
    }

    fn next_subfilter(&mut self) {
        if self.selected_subfilter.is_none() {
            return;
        }

        let current_subfilter = self.selected_subfilter.unwrap().info_type;
        let found_idx = self
            .available_subfilters
            .iter()
            .position(|e| e.info_type == current_subfilter);

        if found_idx.is_some() {
            let next_idx = found_idx.unwrap().wrapping_add(1) % self.available_subfilters.len();
            self.selected_subfilter = Some(self.available_subfilters[next_idx]);
        } else {
            self.log(
                "Could not select next subfilter. Current subfilter not found",
                LogType::ERROR,
            );
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

            if self.selected_subfilter.is_some() {
                if let LogType::INFO(subtype) = entry.log_type {
                    if subtype != self.selected_subfilter.unwrap().info_type {
                        continue;
                    }
                }
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

    pub async fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if key == self.key_config.left {
            self.prev_filter();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.right {
            self.next_filter();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.prev_subfilter {
            self.prev_subfilter();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.next_subfilter {
            self.next_subfilter();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.page_down {
            self.paginator.next_page();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.page_up {
            self.paginator.prev_page();
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for &LogView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(2), Length(2), Min(0), Length(1)]);
        let [filter_area, subfilter_area, body_area, page_area] = vertical.areas(area);

        self.selected_filter.render(filter_area, buf);
        if self.selected_subfilter.is_some() {
            SubFilter::render_tabs(subfilter_area, buf, &self.available_subfilters);
        }

        self.paginator.render(page_area, buf);
        self.render_log_page(body_area, buf);
    }
}
