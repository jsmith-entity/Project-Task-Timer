use ratatui::{
    Frame,
    prelude::{Constraint, Layout, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::Tabs,
};

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

use crate::task_timer::{log_type::LogType, logger::LogRecord};

#[derive(Default, EnumIter, Display, Clone, Copy, FromRepr, PartialEq)]
enum SelectedFilter {
    #[default]
    #[strum(to_string = "All")]
    ALL,
    #[strum(to_string = "Info")]
    INFO,
    #[strum(to_string = "Error")]
    ERROR,
}

impl SelectedFilter {
    fn title(self) -> Line<'static> {
        return format!("  {self}  ").fg(Color::Black).bg(Color::DarkGray).into();
    }

    fn prev(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn includes(self, log_type: LogType) -> bool {
        let mut includes_filter = false;

        if self == SelectedFilter::ALL {
            includes_filter = true;
        } else if matches!(log_type, LogType::INFO(_)) && self == SelectedFilter::INFO {
            includes_filter = true;
        } else if log_type == LogType::ERROR && self == SelectedFilter::ERROR {
            includes_filter = true;
        }

        return includes_filter;
    }
}

#[derive(Default)]
pub struct LoggerView {
    pub recent_log: Vec<LogRecord>,

    selected_filter: SelectedFilter,
}

impl LoggerView {
    pub fn new() -> Self {
        return Self {
            recent_log: Vec::new(),

            selected_filter: SelectedFilter::default(),
        };
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([Length(2), Min(0)]);
        let [header_area, body_area] = vertical.areas(area);

        self.render_tabs(frame, header_area);

        let dash_len: usize = 60;
        let dash_line = "-".repeat(dash_len);

        let mut log_area = Rect::new(body_area.x, body_area.y, dash_len as u16, body_area.height);

        frame.render_widget(Line::from(dash_line.clone()), log_area);
        log_area.y += 1;

        use Constraint::{Length, Min};
        for entry in self.recent_log.iter().rev() {
            let log_type = entry.log_type.clone();

            if !self.selected_filter.includes(log_type) {
                continue;
            }

            let areas = Layout::vertical([Length(1), Min(0)]).split(log_area);
            let info = Layout::horizontal([Min(0), Length(17)]).split(areas[0]);

            let log_type = entry.log_type.clone().to_string();
            let subtype = match entry.log_type {
                LogType::INFO(subtype) => format!(" - {}", subtype.to_string()),
                _ => "".to_string(),
            };

            frame.render_widget(
                Line::from(format!("{}{}", log_type, subtype)).fg(entry.log_type.color()),
                info[0],
            );
            frame.render_widget(Line::from(entry.time_stamp.print()), info[1]);

            log_area.y += 2;

            let info_line = Line::from(format!("{}", entry.message.clone()));
            frame.render_widget(info_line, log_area);

            log_area.y += 1;

            frame.render_widget(Line::from(dash_line.clone()), log_area);
            log_area.y += 1;
        }
    }

    pub fn prev_filter(&mut self) {
        self.selected_filter = self.selected_filter.prev();
    }

    pub fn next_filter(&mut self) {
        self.selected_filter = self.selected_filter.next();
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles = SelectedFilter::iter().map(SelectedFilter::title);

        let selected_tab_idx = self.selected_filter as usize;
        let highlight_style = (Color::Black, Color::Gray);

        let erm = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_idx)
            .padding("", "")
            .divider(" ");

        frame.render_widget(erm, area);
    }
}
