use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Line;

use crate::task_timer::node::{Node, NodePath};

#[derive(PartialEq)]
pub struct TimeData {
    pub line_num: u16,
    pub node_path: NodePath,
    pub task_num: usize,
    pub active: bool,
}

pub struct TimerView {
    pub selected_line: u16,

    area: Rect,
    root_node: Node,
    drawn_nodes: Vec<NodePath>,
    completed_tasks: Vec<u16>,

    time_data: Vec<TimeData>,
}

impl TimerView {
    pub fn new() -> Self {
        return Self {
            selected_line: 1,

            area: Rect::new(0, 0, 0, 0),
            root_node: Node::new(),
            drawn_nodes: Vec::new(),
            completed_tasks: Vec::new(),

            time_data: Vec::new(),
        };
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame,
        area: &Rect,
        root_node: &Node,
        drawn_data: &(Vec<NodePath>, Vec<u16>),
    ) -> u16 {
        self.area = area.clone();
        self.root_node = root_node.clone();
        self.drawn_nodes = drawn_data.0.clone();
        self.completed_tasks = drawn_data.1.clone();

        let mut height = 0;

        for node in root_node.children.iter() {
            height = self.try_draw_timers(frame, node, height);
        }

        return height;
    }

    pub fn try_activate(&mut self) {
        if let Some(idx) = self
            .time_data
            .iter()
            .position(|e| e.line_num == self.selected_line)
        {
            if self.completed_tasks.contains(&self.selected_line) {
                return;
            }

            for (i, entry) in self.time_data.iter_mut().enumerate() {
                if i != idx {
                    entry.active = false;
                }
            }

            self.time_data[idx].active = !self.time_data[idx].active;
        }
    }

    pub fn active_times(&self) -> Vec<&TimeData> {
        return self.time_data.iter().filter(|e| e.active).collect();
    }

    fn try_draw_timers(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        let mut node_path = Vec::new();
        if !Node::find_path(&self.root_node, &node, &mut node_path) {
            panic!("Comparing nodes that are not in the same tree.");
        }

        if self.drawn_nodes.contains(&node_path) {
            height += self.draw_timers(frame, node, &node_path, height, true);
        } else {
            height += self.draw_timers(frame, node, &node_path, height, false);
        }

        for child_node in node.children.iter() {
            height = self.try_draw_timers(frame, child_node, height);
        }

        return height;
    }

    fn draw_timers(
        &mut self,
        frame: &mut Frame,
        node: &Node,
        node_path: &NodePath,
        height: u16,
        draw_content: bool,
    ) -> u16 {
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
        for (idx, time) in node.content_times.iter().enumerate() {
            if draw_content {
                let line = self.create_line(time.as_secs(), timer_area.y, 1);

                frame.render_widget(line, timer_area);

                self.update_time_data(timer_area.y, node_path.to_vec(), idx, false);
                timer_area.y += 1;
            }

            total_seconds += time.as_secs();
        }

        let heading_line = self.create_line(total_seconds, initial_y - 1, 0);
        let heading_area = Rect::new(self.area.x, initial_y - 1, self.area.width, self.area.height);
        frame.render_widget(heading_line, heading_area);

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

    fn update_time_data(&mut self, line_num: u16, node_path: NodePath, task_num: usize, active: bool) {
        let entry = TimeData {
            line_num,
            node_path,
            task_num,
            active,
        };

        if !self.time_data.contains(&entry) {
            self.time_data.push(entry);
        } else {
            let idx = self.time_data.iter().position(|e| *e == entry).unwrap();
            self.time_data[idx].active = active;
        }
    }

    fn create_line(&self, seconds: u64, pos_y: u16, indent: usize) -> Line {
        let text = TimerView::format_time(seconds, indent);
        let mut line = Line::from(text);
        if self.selected_line == pos_y {
            line = line.bg(Color::Gray).fg(Color::Black);
        }

        if self.completed_tasks.contains(&pos_y) {
            line = line.fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT);
        }

        return line;
    }
}
