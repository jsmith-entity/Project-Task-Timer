use ratatui::{
    prelude::{
        Buffer,
        Constraint::{Length, Min},
        Layout, Rect,
    },
    widgets::Widget,
};

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::components::views::Paginator;

use crate::{
    info_subtype::InfoSubType,
    node::{Node, NodePath},
    traits::ViewEventHandler,
};

use super::{NavigationBar, Tasks};

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
}

impl TaskView {
    pub fn new() -> Self {
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
        };
    }

    pub fn new_with(task_view: TaskView) -> Self {
        return Self {
            root_node: task_view.root_node,
            content_area: Rect::default(),

            displayed_node: task_view.displayed_node,
            tasks: task_view.tasks.clone(),

            paginator: task_view.paginator,
            content_height: task_view.tasks.content_height,
            selected_line: task_view.selected_line,

            nav_bar: task_view.nav_bar,
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

impl ViewEventHandler for TaskView {
    fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String> {
        match key_code {
            KeyCode::Char('j') => self.select_line(self.selected_line + 1),
            KeyCode::Char('k') => self.select_line(self.selected_line - 1),
            KeyCode::Char('J') => {
                self.paginator.next_page();
                let (page_start, page_end) = self.paginator.page_slice();

                self.tasks.slice_bounds(page_start, page_end);
                self.selected_line = 1;
            }
            KeyCode::Char('K') => {
                self.paginator.prev_page();
                let (page_start, page_end) = self.paginator.page_slice();
                self.tasks.slice_bounds(page_start, page_end);
                self.selected_line = self.content_height;
            }
            _ => (),
        }

        return match key_code {
            KeyCode::Char('s') => self.tasks.try_activate(),
            KeyCode::Char(' ') => self.toggle_task(),
            KeyCode::Char('b') => self.enter_prev_node(),
            KeyCode::Enter => self.enter_next_node(),
            _ => Ok((InfoSubType::None, "erm".to_string())),
        };
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
