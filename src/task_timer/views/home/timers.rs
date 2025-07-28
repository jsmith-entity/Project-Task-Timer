use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::task_timer::{node::Node, views::log::log_type::InfoSubType};

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Timers {
    pub total_time: Duration,
    pub lines: Vec<(bool, Duration)>,
    pub task_offset: usize,
    pub selected_line: u16,
    pub content_height: u16,
    pub active_time: Option<u16>,

    page_start: usize,
    page_end: usize,
}

impl Timers {
    pub fn new(node: &Node) -> Self {
        let total_time = node.total_time.clone();
        let tasks: Vec<(bool, Duration)> = node
            .completed_tasks
            .iter()
            .cloned()
            .zip(node.content_times.iter().cloned())
            .collect();
        let subheadings = Timers::subheading_times(node);

        let content_height = tasks.len() as u16 + subheadings.len() as u16;

        return Self {
            total_time,
            lines: tasks.clone().into_iter().chain(subheadings.into_iter()).collect(),
            task_offset: tasks.len(),
            selected_line: 1,
            content_height,
            active_time: None,
            page_start: 0,
            page_end: 0,
        };
    }

    pub fn update_time(&mut self) {
        if self.active_time.is_some() {
            let idx = self.active_time.unwrap() as usize;

            if idx < self.task_offset {
                self.lines[idx].1 += Duration::from_secs(1);
            } else {
                let subheading_idx = idx + self.task_offset;
                self.lines[subheading_idx].1 += Duration::from_secs(1);
            }
        }
    }

    pub fn try_activate(&mut self) -> Result<(InfoSubType, String), String> {
        let info_type: InfoSubType;

        if let Some(_) = self.active_time {
            info_type = InfoSubType::StopTimer;
            self.active_time = None;
        } else {
            info_type = InfoSubType::StartTimer;

            let timer_pos = self.page_start + self.selected_line as usize - 1;

            if timer_pos >= self.task_offset {
                return Err("Cannot start a subheading time".to_string());
            }

            let completed = self.lines[timer_pos].0;
            if !completed {
                self.active_time = Some(timer_pos as u16);
            } else {
                return Err("Cannot start a time on a completed task".to_string());
            }
        }

        return Ok((info_type, self.selected_line.to_string()));
    }

    pub fn active_on_line(&self) -> bool {
        if let Some(line_num) = self.active_time {
            if line_num == self.page_start as u16 + self.selected_line - 1 {
                return true;
            }
        }
        return false;
    }

    pub fn task_slice(&self) -> &[(bool, Duration)] {
        return &self.lines[0..self.task_offset];
    }

    pub fn subheading_slice(&self) -> &[(bool, Duration)] {
        return &self.lines[self.task_offset..self.lines.len()];
    }

    pub fn slice_bounds(&mut self, start_idx: usize, end_idx: usize) {
        self.page_start = start_idx;
        self.page_end = end_idx;
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
        let entry_slice = &self.lines[self.page_start..self.page_end];
        for (idx, entry) in entry_slice.iter().enumerate() {
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
    }
}
