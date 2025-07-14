use ratatui::Frame;
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use std::time::Duration;

use crate::task_timer::node::Node;
use crate::task_timer::time_stamp::LogRecord;
use crate::task_timer::views::{controls::*, logger::*, tasks::*, timers::*};

pub struct Window {
    pub file_name: String,
    pub content_height: u16,
    pub content_tree: Node,

    pub task_list: MarkdownView,
    pub timers: TimerView,
    pub controls: ControlView,
    pub log: LoggerView,

    markdown_area_bounds: Rect,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),
            content_height: 0,
            content_tree: Node::new(),
            task_list: MarkdownView::new(),
            timers: TimerView::new(),
            controls: ControlView::new(),
            log: LoggerView::new(),

            markdown_area_bounds: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let root_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        )
        .split(area);

        self.draw_task_window(frame, root_layout[0]);

        let bottom_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(60), Constraint::Percentage(40)],
        )
        .split(root_layout[1]);

        self.draw_control_window(frame, bottom_layout[0]);
        self.draw_log_window(frame, bottom_layout[1]);
    }

    pub fn select_line(&mut self, line_num: u16) {
        let area_bounds = self.markdown_area_bounds;

        let win_max_height = area_bounds.y + area_bounds.height;

        let lower_bound = area_bounds.y;
        let upper_bound = if self.content_height < win_max_height {
            self.content_height + 1
        } else {
            win_max_height
        };

        let within_bounds = line_num >= lower_bound && line_num < upper_bound;
        if within_bounds {
            self.task_list.selected_line = line_num;
            self.timers.selected_line = line_num;
        }
    }

    pub fn update_time(&mut self) {
        let node_data = self.timers.active_times();

        for entry in node_data.iter() {
            let node = self.content_tree.get_node(&entry.node_path).unwrap();

            node.content_times[entry.task_num] += Duration::from_secs(1);
        }
    }

    pub fn update_completed_task(&mut self) {
        if let Some((task_idx, found_path)) = self.task_list.selected_task() {
            let node = self.content_tree.get_node(found_path).unwrap();

            // stop a timer if it exists
            if self.timers.active_on_selected() {
                self.timers.stop_selected_time();
            }

            node.completed_tasks[task_idx] = !node.completed_tasks[task_idx];
        }
    }

    pub fn toggle_headings(&mut self, visible: bool) {
        self.task_list.toggle_nodes(visible);
        self.task_list.selected_line = 1;
        self.timers.selected_line = 1;
    }

    pub fn update_log(&mut self, recent_log: Vec<LogRecord>) {
        self.log.recent_log = recent_log;
    }

    fn draw_task_window(&mut self, frame: &mut Frame, root_area: Rect) {
        let local_title = self.file_name.clone();
        let local_block = Block::default().title(local_title).borders(Borders::ALL);
        frame.render_widget(&local_block, root_area);

        let inner_area = local_block.inner(root_area);
        self.markdown_area_bounds = inner_area;

        let areas = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(13), Constraint::Min(0)],
        )
        .split(inner_area);

        let content = &self.content_tree;
        let (task_height, drawn_data) = self.task_list.draw(frame, &areas[1], content);
        let time_height = self.timers.draw(frame, &areas[0], &content, &drawn_data);
        assert!(task_height == time_height);

        self.content_height = task_height;
    }

    fn draw_control_window(&mut self, frame: &mut Frame, area: Rect) {
        let local_block = Block::default()
            .title("Controls".to_string())
            .borders(Borders::ALL);
        frame.render_widget(&local_block, area);

        let inner_area = local_block.inner(area);

        self.controls.draw(frame, &inner_area);
    }

    fn draw_log_window(&mut self, frame: &mut Frame, area: Rect) {
        let local_block = Block::default().title("Log".to_string()).borders(Borders::ALL);
        frame.render_widget(&local_block, area);

        let inner_area = local_block.inner(area);

        self.log.draw(frame, &inner_area);
    }
}
