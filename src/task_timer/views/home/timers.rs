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
    pub task_times: Vec<(bool, Duration)>,
    pub subheading_times: Vec<(bool, Duration)>,
    pub selected_line: u16,
    pub content_height: u16,
    pub active_time: Option<u16>,

    task_offset: usize,
}

impl Timers {
    pub fn new(node: &Node) -> Self {
        let total_time = node.total_time.clone();
        let selected_line = 1;

        let task_times: Vec<(bool, Duration)> = node
            .completed_tasks
            .iter()
            .cloned()
            .zip(node.content_times.iter().cloned())
            .collect();
        let subheading_times = Timers::subheading_times(node);

        let task_offset = task_times.len();
        let content_height = task_offset as u16 + subheading_times.len() as u16;

        return Self {
            total_time,
            task_times,
            subheading_times,
            selected_line,
            content_height,
            active_time: None,
            task_offset,
        };
    }

    pub fn update_time(&mut self) {
        if self.active_time.is_some() {
            let idx = self.active_time.unwrap() as usize;

            if idx < self.task_offset {
                self.task_times[idx].1 += Duration::from_secs(1);
            } else {
                let subheading_idx = idx - self.task_offset;
                self.subheading_times[subheading_idx].1 += Duration::from_secs(1);
            }
        }
    }

    pub fn try_activate(&mut self) -> Result<InfoSubType, String> {
        if let Some(_) = self.active_time {
            self.active_time = None;
        } else {
            let timer_pos = self.selected_line - 1;

            if timer_pos >= self.task_times.len() as u16 {
                return Err("Cannot start a subheading time".to_string());
            }

            let completed = self.task_times[timer_pos as usize].0;
            if !completed {
                self.active_time = Some(timer_pos);
            } else {
                return Err("Cannot start a time on a completed task".to_string());
            }
        }

        return Ok(InfoSubType::General);
    }

    pub fn active_on_line(&self) -> bool {
        if let Some(line_num) = self.active_time {
            if line_num == self.selected_line - 1 {
                return true;
            }
        }
        return false;
    }

    fn subheading_times(node: &Node) -> Vec<(bool, Duration)> {
        let mut times: Vec<(bool, Duration)> = Vec::new();
        for subheading in node.children.iter() {
            let full_entry = Timers::extract_entry(&subheading);

            times.push(full_entry);
        }

        return times;
    }

    fn extract_entry(node: &Node) -> (bool, Duration) {
        let completed_node = node.completed_tasks.iter().all(|&x| x);
        let mut completed_subheadings = true;

        let mut entry_time: Duration = node.content_times.iter().sum();
        for subheading in node.children.iter() {
            let (completed, duration) = Timers::extract_entry(subheading);
            entry_time += duration;
            completed_subheadings &= completed;
        }

        return (completed_node && completed_subheadings, entry_time);
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
        for (idx, entry) in self.task_times.iter().enumerate() {
            let completed = entry.0;
            let time = &entry.1;

            let mut style = Style::default();
            if idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }
            if completed {
                style = style.fg(Color::DarkGray);
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

        for (idx, &(completed, time)) in self.subheading_times.iter().enumerate() {
            let mut style = Style::default();
            if self.task_offset as u16 + idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }
            if completed {
                style = style.fg(Color::DarkGray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + self.task_offset as u16 + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(Timers::format_duration(&time))
                .style(style)
                .render(display_area, buf);
        }
    }
}
