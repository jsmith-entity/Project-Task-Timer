use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect},
    widgets::Widget,
};

use crossterm::event::KeyCode;

use crate::task_timer::{
    node::Node,
    views::{
        home::{navigation_bar::NavigationBar, tasks_overview::TaskOverview, timers::Timers},
        log::log_type::*,
    },
};

#[derive(Default, Clone)]
pub struct MainView {
    pub root_node: Node,
    pub content_area: Rect,

    displayed_node: Node,
    task_overview: TaskOverview,
    timers: Timers,

    selected_line: u16,
    content_height: u16,

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

            selected_line: 1,
            content_height: 0,

            nav_bar: NavigationBar::new(),
        };
    }

    pub fn update_display_data(&mut self, new_display_node: Node) {
        let tasks = new_display_node.content.clone();
        let subheadings: Vec<_> = new_display_node
            .children
            .iter()
            .filter_map(|e| e.heading.clone())
            .collect();
        let times = new_display_node.content_times.clone();
        let content_height = tasks.len() as u16 + subheadings.len() as u16;
        let selected_line = 1;

        assert!(tasks.len() == times.len());

        self.task_overview = TaskOverview {
            tasks,
            subheadings,
            selected_line,
            content_height,
        };

        self.timers = Timers {
            times,
            selected_line,
            content_height,
        };

        self.content_height = content_height;
        self.displayed_node = new_display_node;
        self.selected_line = selected_line;
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

    pub fn handle_events(&mut self, key_code: KeyCode) -> Result<InfoSubType, String> {
        match key_code {
            KeyCode::Char('j') => self.select_line(self.selected_line + 1),
            KeyCode::Char('k') => self.select_line(self.selected_line - 1),
            _ => (),
        }

        return match key_code {
            // KeyCode::Char('s') => self.timers.try_activate(),
            // KeyCode::Char(' ') => self.update_completed_task(),
            KeyCode::Char('b') => self.enter_prev_node(),
            KeyCode::Enter => self.enter_next_node(),
            _ => Ok(InfoSubType::None),
        };
    }

    fn select_line(&mut self, line_num: u16) {
        if line_num > 0 && line_num <= self.content_height {
            self.selected_line = line_num;
            self.task_overview.selected_line = line_num;
            self.timers.selected_line = line_num;
        }
    }

    fn enter_prev_node(&mut self) -> Result<InfoSubType, String> {
        let mut curr_node_path = Vec::new();
        if !Node::find_path(&self.root_node, &self.displayed_node, &mut curr_node_path) {
            return Err(
                "Comparing nodes that do not belong on the same tree when collecting display data"
                    .to_string(),
            );
        }

        curr_node_path.pop();
        if let Some(new_node) = self.root_node.get_node(&curr_node_path) {
            self.update_display_data(new_node.clone());
            self.nav_bar.pop_breadcrumb();
        } else {
            return Err("Failed to convert node path to node when entering previous heading".to_string());
        }

        return Ok(InfoSubType::EnterParent);
    }

    fn enter_next_node(&mut self) -> Result<InfoSubType, String> {
        if let Some(new_node) = self.get_subheading(self.selected_line as usize) {
            self.update_display_data(new_node.clone());
            self.add_breadcrumb();
        } else {
            return Err("No subheading found on selected line".to_string());
        }

        return Ok(InfoSubType::EnterSubheading);
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

        assert!(self.timers.times.len() == self.task_overview.tasks.len());
        self.timers.render(time_area, buf);
        self.task_overview.render(task_area, buf);
    }
}
