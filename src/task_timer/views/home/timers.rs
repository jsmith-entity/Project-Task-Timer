use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use std::time::Duration;

use crate::task_timer::{node::Node, views::log::log_type::InfoSubType};

#[derive(Default, Clone)]
pub struct Timers {
    pub total_time: Duration,
    pub task_times: Vec<Duration>,
    pub subheading_times: Vec<Duration>,
    pub selected_line: u16,
    pub content_height: u16,

    task_offset: usize,
    active_time: Option<u16>,
}

impl Timers {
    pub fn new(node: &Node) -> Self {
        let total_time = node.total_time.clone();
        let selected_line = 1;

        let task_times = node.content_times.clone();
        let subheading_times = Timers::subheading_times(node);

        let task_offset = task_times.len();
        let content_height = task_offset as u16 + subheading_times.len() as u16;

        return Self {
            total_time,
            task_times,
            subheading_times,
            selected_line,
            content_height,
            task_offset,
            active_time: None,
        };
    }

    pub fn try_activate(&mut self) -> Result<InfoSubType, String> {
        let timer_pos = self.selected_line - 1;

        if timer_pos >= self.task_times.len() as u16 {
            return Err("Trying to activate a subheading time.".to_string());
        }

        self.active_time = Some(timer_pos);

        return Ok(InfoSubType::General);
    }

    pub fn update_time(&mut self) {
        if self.active_time.is_some() {
            let idx = self.active_time.unwrap() as usize;

            if idx < self.task_offset {
                self.task_times[idx] += Duration::from_secs(1);
            } else {
                let subheading_idx = idx - self.task_offset;
                self.subheading_times[subheading_idx] += Duration::from_secs(1);
            }
        }
    }

    fn subheading_times(node: &Node) -> Vec<Duration> {
        let mut times: Vec<Duration> = Vec::new();
        for subheading in node.children.iter() {
            let total_time = Timers::total_time(&subheading);
            times.push(total_time);
        }

        return times;
    }

    fn total_time(node: &Node) -> Duration {
        let task_time: Duration = node.content_times.iter().sum();
        let mut subheading_time = Duration::default();
        for subheading in node.children.iter() {
            subheading_time += Timers::total_time(subheading);
        }

        return task_time + subheading_time;
    }

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
        for (idx, time) in self.task_times.iter().enumerate() {
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

        for (idx, time) in self.subheading_times.iter().enumerate() {
            let mut style = Style::default();
            if self.task_offset as u16 + idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + self.task_offset as u16 + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(Timers::format_duration(time))
                .style(style)
                .render(display_area, buf);
        }
    }
}
