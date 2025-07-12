use ratatui::Frame;
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders};
use std::rc::Rc;
use std::time::Duration;

use crate::task_timer::control_view::ControlView;
use crate::task_timer::markdown_view::MarkdownView;
use crate::task_timer::node::Node;
use crate::task_timer::timer_view::TimerView;

pub struct Window {
    pub file_name: String,
    pub content_height: u16,
    pub content_tree: Node,

    pub task_list: MarkdownView,
    pub timers: TimerView,
    pub controls: ControlView,

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

            markdown_area_bounds: Rect::new(0, 0, 0, 0),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let (vertical, horizontal) = Window::define_layouts();

        let area = frame.area();

        let root_title = self.file_name.clone();
        let root_block = Block::default().title(root_title).borders(Borders::ALL);

        let vertical_panes = vertical.split(area);
        let task_inner_area = root_block.inner(vertical_panes[0]);
        let horizontal_panes = horizontal.split(task_inner_area);

        frame.render_widget(root_block, vertical_panes[0]);

        self.markdown_area_bounds = task_inner_area;
        self.draw_task_window(frame, horizontal_panes);
        self.draw_control_window(frame, vertical_panes[1]);
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

    fn define_layouts() -> (Layout, Layout) {
        let vertical = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        );

        let horizontal = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(13), Constraint::Min(0)],
        );

        return (vertical, horizontal);
    }

    fn draw_task_window(&mut self, frame: &mut Frame, areas: Rc<[Rect]>) {
        let content = &self.content_tree;

        let (task_height, drawn_data) = self.task_list.draw(frame, &areas[1], content);

        let time_height = self.timers.draw(frame, &areas[0], &content, &drawn_data);

        assert!(task_height == time_height);

        self.content_height = task_height;
    }

    fn draw_control_window(&mut self, frame: &mut Frame, area: Rect) {
        let root_block = Block::default()
            .title("Controls".to_string())
            .borders(Borders::ALL);

        frame.render_widget(&root_block, area);

        let inner_area = root_block.inner(area);
        self.controls.draw(frame, &inner_area);
    }
}
