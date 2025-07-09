use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;

use crate::task_timer::node::{Node, NodePath};

pub struct TimerView {
    pub selected_line: u16,

    area: Rect,
    root_node: Node,
    drawn_nodes: Vec<NodePath>,
}

impl TimerView {
    pub fn new() -> Self {
        return Self {
            selected_line: 1,

            area: Rect::new(0, 0, 0, 0),
            root_node: Node::new(),
            drawn_nodes: Vec::new(),
        };
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame,
        area: &Rect,
        root_node: &Node,
        drawn_nodes: &Vec<NodePath>,
    ) -> u16 {
        self.area = area.clone();
        self.root_node = root_node.clone();
        self.drawn_nodes = drawn_nodes.clone();

        let mut height = 0;

        for node in root_node.children.iter() {
            height = self.try_draw_timers(frame, node, height);
        }

        return height;
    }

    fn try_draw_timers(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        let mut node_path = Vec::new();
        if !Node::find_path(&self.root_node, &node, &mut node_path) {
            panic!("Comparing nodes that are not in the same tree.");
        }

        if self.drawn_nodes.contains(&node_path) {
            height += self.draw_timers(frame, node, height, true);
        } else {
            height += self.draw_timers(frame, node, height, false);
        }

        for child_node in node.children.iter() {
            height = self.try_draw_timers(frame, child_node, height);
        }

        return height;
    }

    fn draw_timers(&mut self, frame: &mut Frame, node: &Node, height: u16, draw_content: bool) -> u16 {
        assert!(node.content.len() == node.content_times.len());

        let block_height = node.content.len() as u16 + 2;
        let mut timer_area = Rect::new(
            self.area.x,
            self.area.y + height + 1,
            self.area.width,
            block_height,
        );

        let mut total_seconds = 0;
        let initial_y = timer_area.y;
        for time in node.content_times.iter() {
            if draw_content {
                let text = TimerView::format_time(time.as_secs(), 1);
                let mut line = Line::from(text);
                if self.selected_line == timer_area.y {
                    line = line.bg(Color::Gray).fg(Color::Black);
                }

                frame.render_widget(line, timer_area);

                timer_area.y += 1;
            }

            total_seconds += time.as_secs();
        }

        let text = TimerView::format_time(total_seconds, 0);
        let mut line = Line::from(text);
        if self.selected_line == initial_y - 1 {
            line = line.bg(Color::Gray).fg(Color::Black);
        }

        let heading_area = Rect::new(self.area.x, initial_y - 1, self.area.width, self.area.height);
        frame.render_widget(line, heading_area);

        let node_height = timer_area.y - initial_y + 1;
        return node_height;
    }

    fn format_time(total_seconds: u64, indent_level: usize) -> String {
        let hours = total_seconds / 3600;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let indent = format!("{:1$}", "", indent_level);
        return format!("{}[{:02}:{:02}:{:02}] ", indent, hours, minutes, seconds);
    }
}
