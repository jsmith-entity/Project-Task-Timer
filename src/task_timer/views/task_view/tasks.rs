use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::task_timer::{InfoSubType, node::Node};

use super::Task;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Tasks {
    pub lines: Vec<Task>,
    pub task_offset: usize,

    pub selected_line: u16,
    pub content_height: u16,

    pub total_time: Duration,
    pub active_time: Option<u16>,

    page_start: usize,
    page_end: usize,
}

impl Tasks {
    pub fn new(node: &Node) -> Self {
        let tasks = Tasks::extract_tasks(node);
        let subheadings = Tasks::subheading_times(node);

        let task_offset = tasks.len();

        return Self {
            lines: tasks.into_iter().chain(subheadings.into_iter()).collect(),
            task_offset,

            selected_line: 1,
            content_height: task_offset as u16,

            total_time: Duration::from_secs(0),
            active_time: None,

            page_start: 0,
            page_end: 0,
        };
    }

    pub fn update(&mut self, selected_line: u16) {
        self.selected_line = selected_line;

        let visible_slice = &mut self.lines[self.page_start..self.page_end];
        for (idx, entry) in visible_slice.into_iter().enumerate() {
            let mut style = Style::default();
            if idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }
            if entry.completed {
                style = style.fg(Color::DarkGray);
            }

            entry.style = style;
        }
    }

    fn extract_tasks(node: &Node) -> Vec<Task> {
        let mut tasks: Vec<Task> = Vec::new();

        for idx in 0..node.completed_tasks.len() {
            let name = node.content[idx].clone();
            let completed = node.completed_tasks[idx];
            let duration = node.content_times[idx];
            let style = Style::default();

            let task = Task {
                name,
                completed,
                duration,
                style,
            };

            tasks.push(task);
        }

        return tasks;
    }

    fn subheading_times(node: &Node) -> Vec<Task> {
        let mut entries: Vec<Task> = Vec::new();
        for subheading in node.children.iter() {
            let full_entry = Tasks::extract_entry(&subheading);
            entries.push(full_entry);
        }
        return entries;
    }

    fn extract_entry(node: &Node) -> Task {
        let completed_node = node.completed_tasks.iter().all(|&x| x);
        let mut completed_subheadings = true;

        let mut entry_time: Duration = node.content_times.iter().sum();
        for subheading in node.children.iter() {
            let entry = Tasks::extract_entry(subheading);

            entry_time += entry.duration;
            completed_subheadings &= entry.completed;
        }

        let task = Task {
            name: node.heading.clone().unwrap(),
            duration: entry_time,
            completed: completed_node && completed_subheadings,
            style: Style::default(),
        };

        return task;
    }

    pub fn task_slice(&self) -> &[Task] {
        return &self.lines[0..self.task_offset];
    }
}

impl Tasks {
    pub fn toggle_task(&mut self, idx: usize) -> InfoSubType {
        let info_type: InfoSubType;

        if self.active_on_line() {
            self.active_time = None;
        }

        if self.lines[idx].completed {
            info_type = InfoSubType::UncompleteTask;
        } else {
            info_type = InfoSubType::CompleteTask;
        }

        self.lines[idx].completed = !self.lines[idx].completed;

        return info_type;
    }

    pub fn slice_bounds(&mut self, start_idx: usize, end_idx: usize) {
        self.page_start = start_idx;
        self.page_end = end_idx;
    }

    pub fn active_on_line(&self) -> bool {
        if let Some(line_num) = self.active_time {
            if line_num == self.page_start as u16 + self.selected_line - 1 {
                return true;
            }
        }
        return false;
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

            let completed = self.lines[timer_pos].completed;
            if !completed {
                self.active_time = Some(timer_pos as u16);
            } else {
                return Err("Cannot start a time on a completed task".to_string());
            }
        }

        return Ok((info_type, self.selected_line.to_string()));
    }

    pub fn update_time(&mut self) {
        if self.active_time.is_some() {
            let idx = self.active_time.unwrap() as usize;

            if idx < self.task_offset {
                self.lines[idx].duration += Duration::from_secs(1);
            } else {
                let subheading_idx = idx + self.task_offset;
                self.lines[subheading_idx].duration += Duration::from_secs(1);
            }
        }
    }
}

impl Widget for &Tasks {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let entry_slice = &self.lines[self.page_start..self.page_end];
        for (idx, task) in entry_slice.iter().enumerate() {
            let display_area = Rect {
                x: area.x,
                y: area.y + idx as u16,
                width: area.width,
                height: 1,
            };

            task.render(display_area, buf);
        }
    }
}
