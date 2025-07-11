use ratatui::Frame;
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use std::time::Duration;

use crate::task_timer::markdown_view::MarkdownView;
use crate::task_timer::node::Node;
use crate::task_timer::timer_view::TimerView;

pub struct Window {
    pub file_name: String,
    pub content_height: u16,
    pub content_tree: Node,
    pub task_list: MarkdownView,
    pub timers: TimerView,

    layout: Layout,
    area_bounds: Rect,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),
            content_height: 0,
            content_tree: Node::new(),
            task_list: MarkdownView::new(),
            timers: TimerView::new(),

            layout: Layout::new(
                Direction::Horizontal,
                [Constraint::Length(13), Constraint::Min(0)],
            ),
            area_bounds: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let root_title = self.file_name.clone();
        let root_block = Block::default().title(root_title).borders(Borders::ALL);

        let content = &self.content_tree;

        self.area_bounds = root_block.inner(area);
        let areas = self.layout.split(self.area_bounds);

        frame.render_widget(root_block, area);
        let (task_height, drawn_nodes) = self.task_list.draw(frame, &areas[1], content);
        let time_height = self.timers.draw(frame, &areas[0], &content, &drawn_nodes);

        assert!(task_height == time_height);

        self.content_height = task_height;
    }

    pub fn select_line(&mut self, line_num: u16) {
        let win_max_height = self.area_bounds.y + self.area_bounds.height;

        let lower_bound = self.area_bounds.y;
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
            node.completed_tasks[task_idx] = !node.completed_tasks[task_idx];
        }
    }
}
