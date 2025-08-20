use ratatui::{
    prelude::{
        Buffer, Constraint,
        Constraint::{Length, Min},
        Layout, Rect, Stylize,
    },
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    components::Component,
    config::KeyConfig,
    events::*,
    info_subtype::InfoSubType,
    node::{Node, NodePath},
};

use super::Paginator;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Task {
    pub name: String,
    pub duration: Duration,
    pub completed: bool,

    pub style: Style,
}

impl Task {
    fn format_duration(&self) -> String {
        let secs = self.duration.as_secs();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds)
    }
}

impl Widget for &Task {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let duration = self.format_duration();

        let task = format!("{} {}", duration, self.name);

        Line::from(task).style(self.style).render(area, buf);
    }
}

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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct NavigationBar {
    back_text: String,
    breadcrumbs: Vec<String>,
}

impl NavigationBar {
    pub fn new() -> Self {
        return Self {
            back_text: " (b) Back ".to_string(),
            breadcrumbs: Vec::new(),
        };
    }

    pub fn push_breadcrumb(&mut self, new_heading: String) {
        let heading = new_heading.trim_start_matches('#').trim().to_string();
        self.breadcrumbs.push(heading);
    }

    pub fn pop_breadcrumb(&mut self) {
        self.breadcrumbs.pop();
    }
}

impl Widget for &NavigationBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let back_len = self.back_text.len() as u16;
        let horizontal = Layout::horizontal([Length(back_len), Length(5), Min(0)]);
        let [back_area, _, breadcrumb_area] = horizontal.areas(area);

        Line::from(self.back_text.clone())
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(back_area, buf);

        let mut breadcrumb_content = Vec::new();
        for (i, breadcrumb) in self.breadcrumbs.iter().enumerate() {
            if i > 0 {
                breadcrumb_content.push(Span::raw(" / "));
            }

            if i == self.breadcrumbs.len() - 1 {
                breadcrumb_content.push(Span::styled(
                    breadcrumb.to_string(),
                    Style::default().bold().fg(Color::Blue),
                ));
            } else {
                breadcrumb_content.push(Span::styled(
                    breadcrumb.to_string(),
                    Style::default().fg(Color::Blue),
                ));
            }
        }

        Line::from(breadcrumb_content).render(breadcrumb_area, buf);
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct TaskView {
    pub root_node: Node,
    #[serde(skip)]
    pub content_area: Rect,
    pub displayed_node: Node,
    pub tasks: Tasks,

    paginator: Paginator,
    content_height: u16,
    selected_line: u16,

    nav_bar: NavigationBar,

    #[serde(skip)]
    key_config: KeyConfig,
}

impl TaskView {
    pub fn new(key_config: KeyConfig) -> Self {
        return Self {
            root_node: Node::new(),
            content_area: Rect::default(),

            displayed_node: Node::new(),
            tasks: Tasks::default(),

            paginator: Paginator {
                page: 0,
                page_size: 25,
                entry_len: 0,
            },
            content_height: 0,
            selected_line: 1,

            nav_bar: NavigationBar::new(),
            key_config,
        };
    }

    pub fn new_with(task_view: TaskView, key_config: KeyConfig) -> Self {
        return Self {
            root_node: task_view.root_node,
            content_area: Rect::default(),

            displayed_node: task_view.displayed_node,
            tasks: task_view.tasks.clone(),

            paginator: task_view.paginator,
            content_height: task_view.tasks.content_height,
            selected_line: task_view.selected_line,

            nav_bar: task_view.nav_bar,
            key_config,
        };
    }

    pub fn update(&mut self) {
        self.tasks.update(self.selected_line);
        self.content_height = self.paginator.content_height();
    }

    pub fn update_display_data(&mut self, new_display_node: Node) {
        self.tasks = Tasks::new(&new_display_node);

        self.selected_line = 1;
        self.displayed_node = new_display_node;

        self.paginator.page = 0;
        let entry_len = self.tasks.lines.len();
        self.update_paginator(entry_len);
        self.content_height = self.paginator.content_height();
    }

    pub fn get_subheading(&self, line_num: usize) -> Option<Node> {
        let global_idx = self.paginator.offset() + line_num - 1;
        let subheading_idx = global_idx - self.tasks.task_offset;

        if global_idx >= self.tasks.task_offset && subheading_idx < self.displayed_node.children.len() {
            return Some(self.displayed_node.children[subheading_idx].clone());
        } else {
            return None;
        }
    }

    pub fn update_time(&mut self) -> Result<(), String> {
        self.tasks.update_time();

        let node_path = match Node::find_path(&self.root_node, &self.displayed_node) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        let mut total_time = Duration::default();

        let task_slice = self.tasks.task_slice();
        for (idx, entry) in task_slice.iter().enumerate() {
            total_time += entry.duration.clone();
            self.displayed_node.content_times[idx] = entry.duration.clone();
        }

        for subheading in self.displayed_node.children.iter() {
            total_time += subheading.total_time;
        }

        self.displayed_node.total_time = total_time.clone();

        return self.root_node.update_node(&node_path, &self.displayed_node);
    }

    pub fn toggle_task(&mut self) -> Result<(InfoSubType, String), String> {
        let idx = self.paginator.offset() + (self.selected_line as usize - 1);

        let info_type: InfoSubType;
        if idx < self.tasks.task_offset {
            info_type = self.tasks.toggle_task(idx);
            if let Err(e) = self.update_root() {
                return Err(e);
            }
        } else {
            return Err("Cannot complete a subheading".to_string());
        }

        let task_name = self.tasks.lines[idx].name.clone();
        return Ok((info_type, task_name));
    }

    fn update_paginator(&mut self, entry_len: usize) {
        self.paginator.entry_len = entry_len;
        let (page_start, page_end) = self.paginator.page_slice();

        self.tasks.slice_bounds(page_start, page_end);
    }

    fn update_root(&mut self) -> Result<NodePath, String> {
        let node_path = match Node::find_path(&self.root_node, &self.displayed_node) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        let task_slice = self.tasks.task_slice();
        self.displayed_node.completed_tasks = task_slice.iter().map(|e| e.completed).collect();

        if let Err(e) = self.root_node.update_node(&node_path, &self.displayed_node) {
            return Err(e);
        }

        return Ok(node_path);
    }
}

impl TaskView {
    fn select_line(&mut self, line_num: u16) {
        if line_num > 0 && line_num <= self.content_height {
            self.selected_line = line_num;
            self.tasks.selected_line = line_num;
        }
    }

    fn enter_prev_node(&mut self) -> Result<(InfoSubType, String), String> {
        let res = self.update_root();
        if let Err(e) = res {
            return Err(e);
        }

        let mut curr_node_path = res.unwrap();

        curr_node_path.pop();
        if let Some(new_node) = self.root_node.get_node(&curr_node_path) {
            self.update_display_data(new_node.clone());
            self.nav_bar.pop_breadcrumb();
        } else {
            return Err("Failed to convert node path to node when entering previous heading".to_string());
        }

        let heading_name = self
            .displayed_node
            .heading
            .clone()
            .unwrap_or_else(|| "Root Node".to_string());
        return Ok((InfoSubType::EnterParent, heading_name));
    }

    fn enter_next_node(&mut self) -> Result<(InfoSubType, String), String> {
        if let Err(e) = self.update_root() {
            return Err(e);
        }

        if let Some(new_node) = self.get_subheading(self.selected_line as usize) {
            self.update_display_data(new_node.clone());
            self.add_breadcrumb();
        } else {
            return Err("No subheading found on selected line".to_string());
        }

        let heading_name = self
            .displayed_node
            .heading
            .clone()
            .unwrap_or_else(|| "Root Node".to_string());
        return Ok((InfoSubType::EnterSubheading, heading_name));
    }

    fn add_breadcrumb(&mut self) {
        let new_heading_name = self.displayed_node.heading.clone();

        if new_heading_name.is_some() {
            let new_breadcrumb = new_heading_name.unwrap().to_string();
            self.nav_bar.push_breadcrumb(new_breadcrumb);
        }
    }
}

impl Component for TaskView {
    fn event(&mut self, key: KeyCode) -> anyhow::Result<EventState> {
        if key == self.key_config.down {
            self.select_line(self.selected_line + 1);
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.up {
            self.select_line(self.selected_line - 1);
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.page_down {
            self.paginator.next_page();
            let (page_start, page_end) = self.paginator.page_slice();
            self.tasks.slice_bounds(page_start, page_end);
            self.selected_line = 1;
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.page_up {
            self.paginator.prev_page();
            let (page_start, page_end) = self.paginator.page_slice();
            self.tasks.slice_bounds(page_start, page_end);
            self.selected_line = self.content_height;
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.start_timer {
            self.tasks.try_activate();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.complete {
            self.toggle_task();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.back {
            self.enter_prev_node();
            return Ok(EventState::Consumed);
        }

        if key == self.key_config.enter {
            self.enter_next_node();
            return Ok(EventState::Consumed);
        }

        return Ok(EventState::NotConsumed);
    }
}

impl Widget for &TaskView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Length(3), Min(0), Length(1)]);
        let [navigation_row, task_area, page_area] = vertical.areas(area);

        self.nav_bar.render(navigation_row, buf);
        self.tasks.render(task_area, buf);
        self.paginator.render(page_area, buf);
    }
}
