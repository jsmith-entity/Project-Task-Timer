use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect},
    widgets::Widget,
};

use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::task_timer::{
    node::Node,
    views::{
        home::{navigation_bar::NavigationBar, tasks_overview::TaskOverview, timers::Timers},
        log::log_type::*,
    },
};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MainView {
    pub root_node: Node,
    #[serde(skip)]
    pub content_area: Rect,
    pub displayed_node: Node,
    pub task_overview: TaskOverview,
    pub timers: Timers,

    content_height: u16,
    selected_line: u16,

    nav_bar: NavigationBar,
}

impl MainView {
    pub fn new() -> Self {
        return Self {
            root_node: Node::new(),
            content_area: Rect::default(),

            displayed_node: Node::new(),
            task_overview: TaskOverview::default(),
            timers: Timers::default(),

            content_height: 0,
            selected_line: 1,

            nav_bar: NavigationBar::new(),
        };
    }

    pub fn new_with(main_view: MainView) -> Self {
        return Self {
            root_node: main_view.root_node,
            content_area: Rect::default(),

            displayed_node: main_view.displayed_node,
            task_overview: main_view.task_overview,
            timers: main_view.timers,

            content_height: main_view.content_height,
            selected_line: main_view.selected_line,

            nav_bar: main_view.nav_bar,
        };
    }

    pub fn update_display_data(&mut self, new_display_node: Node) {
        self.timers = Timers::new(&new_display_node);

        let completed_subheadings = self.timers.subheading_times.iter().map(|e| e.0).collect();
        self.task_overview = TaskOverview::new(&new_display_node, &completed_subheadings);

        let timers_len = self.timers.task_times.len() + self.timers.subheading_times.len();
        let tasks_len = self.task_overview.tasks.len() + self.task_overview.subheadings.len();
        assert!(timers_len == tasks_len);

        self.content_height = timers_len as u16;
        self.selected_line = 1;
        self.displayed_node = new_display_node;
    }

    pub fn get_subheading(&self, pos: usize) -> Option<Node> {
        let task_offset = self.displayed_node.content.len();
        if task_offset > pos {
            return None;
        }

        let subheading_pos = pos - 1 - task_offset;
        let subheading_len = self.displayed_node.children.len();
        if subheading_pos < subheading_len {
            return Some(self.displayed_node.children[subheading_pos].clone());
        } else {
            return None;
        }
    }

    pub fn handle_events(&mut self, key_code: KeyCode) -> Result<(InfoSubType, String), String> {
        match key_code {
            KeyCode::Char('j') => self.select_line(self.selected_line + 1),
            KeyCode::Char('k') => self.select_line(self.selected_line - 1),
            _ => (),
        }

        return match key_code {
            KeyCode::Char('s') => self.timers.try_activate(),
            KeyCode::Char(' ') => self.toggle_task(),
            KeyCode::Char('b') => self.enter_prev_node(),
            KeyCode::Enter => self.enter_next_node(),
            _ => Ok((InfoSubType::None, "erm".to_string())),
        };
    }

    pub fn update_time(&mut self) -> Result<(), String> {
        self.timers.update_time();

        let node_path = match Node::find_path(&self.root_node, &self.displayed_node) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        let mut total_time = Duration::default();

        for (idx, entry) in self.timers.task_times.iter().enumerate() {
            let time = &entry.1;

            total_time += time.clone();
            self.displayed_node.content_times[idx] = time.clone();
        }

        for subheading in self.displayed_node.children.iter() {
            total_time += subheading.total_time;
        }

        self.displayed_node.total_time = total_time.clone();

        return self.root_node.update_node(&node_path, &self.displayed_node);
    }

    pub fn toggle_task(&mut self) -> Result<(InfoSubType, String), String> {
        assert!(self.timers.task_times.len() == self.task_overview.tasks.len());

        let info_type: InfoSubType;

        let idx = self.selected_line as usize - 1;
        if idx < self.timers.task_times.len() {
            if self.timers.active_on_line() {
                self.timers.active_time = None;
            }

            assert!(self.timers.task_times[idx].0 == self.task_overview.tasks[idx].0);
            if self.timers.task_times[idx].0 {
                info_type = InfoSubType::UncompleteTask;
            } else {
                info_type = InfoSubType::CompleteTask;
            }

            self.timers.task_times[idx].0 = !self.timers.task_times[idx].0;
            self.task_overview.tasks[idx].0 = !self.task_overview.tasks[idx].0;
        } else {
            return Err("Cannot complete a subheading".to_string());
        }

        let task_name = self.task_overview.tasks[idx].1.clone();
        return Ok((info_type, task_name));
    }

    fn select_line(&mut self, line_num: u16) {
        if line_num > 0 && line_num <= self.content_height {
            self.selected_line = line_num;
            self.task_overview.selected_line = line_num;
            self.timers.selected_line = line_num;
        }
    }

    fn enter_prev_node(&mut self) -> Result<(InfoSubType, String), String> {
        let mut curr_node_path = match Node::find_path(&self.root_node, &self.displayed_node) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        self.displayed_node.completed_tasks = self.task_overview.tasks.iter().map(|e| e.0).collect();
        if let Err(e) = self.root_node.update_node(&curr_node_path, &self.displayed_node) {
            return Err(e);
        }

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
        let node_path = match Node::find_path(&self.root_node, &self.displayed_node) {
            Ok(path) => path,
            Err(e) => return Err(e),
        };

        self.displayed_node.completed_tasks = self.task_overview.tasks.iter().map(|e| e.0).collect();
        if let Err(e) = self.root_node.update_node(&node_path, &self.displayed_node) {
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

impl Widget for &MainView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(3), Min(0)]);
        let [navigation_row, content_area] = vertical.areas(area);

        self.nav_bar.render(navigation_row, buf);

        let horizontal = Layout::horizontal([Length(12), Min(0)]);
        let [time_area, task_area] = horizontal.areas(content_area);

        self.timers.render(time_area, buf);
        self.task_overview.render(task_area, buf);
    }
}
