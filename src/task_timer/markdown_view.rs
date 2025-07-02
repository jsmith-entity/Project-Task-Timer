use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;

use crate::task_timer::node::{Node, NodePath};

struct NodeEntry {
    pub line_num: u16,
    pub node_path: NodePath,
    pub visible: bool,
}

pub struct MarkdownView {
    pub selected_line: u16,

    content_tree: Node,

    layout: Layout,
    time_area: Option<Rect>,
    task_area: Option<Rect>,

    node_data: Vec<NodeEntry>,
}

impl MarkdownView {
    pub fn new() -> Self {
        Self {
            content_tree: Node::new(),
            selected_line: 1,

            layout: Layout::new(
                Direction::Horizontal,
                [Constraint::Length(13), Constraint::Min(0)],
            ),
            time_area: None,
            task_area: None,

            node_data: Vec::new(),
        }
    }

    pub fn update(&mut self, new_tree: Node) {
        self.content_tree = new_tree;
    }

    pub fn try_collapse(&mut self) {
        if let Some(idx) = self
            .node_data
            .iter()
            .position(|e| e.line_num == self.selected_line)
        {
            if self.node_data[idx].visible {
                self.node_data[idx].visible = false;
            } else {
                self.node_data[idx].visible = true;
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: &Rect) -> u16 {
        let areas = self.layout.split(*area);
        self.time_area = Some(areas[0]);
        self.task_area = Some(areas[1]);

        let mut height = 0;

        let root_node = self.content_tree.clone();
        for child_node in root_node.children.iter() {
            height = self.draw_node(frame, &child_node, height)
        }

        return height;
    }

    fn draw_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        assert!(self.task_area.is_some());
        assert!(self.time_area.is_some());

        let task_area = self.task_area.unwrap();
        let time_area = self.time_area.unwrap();

        let block_height = node.content.len() as u16 + 2;

        let areas = vec![
            Rect::new(
                time_area.x,
                time_area.y + height + 1,
                time_area.width,
                block_height,
            ),
            Rect::new(
                task_area.x,
                task_area.y + height,
                task_area.width,
                block_height,
            ),
        ];

        let inner_area = self.draw_block(frame, &node, &areas[1]);

        let drawn = self.try_draw_content(frame, &node, &inner_area);
        if drawn {
            height += block_height - 1;
        } else {
            height += 1;
        }

        self.draw_timers(frame, &node, drawn, &areas[0]);

        self.update_node_data(node, &areas[1]);

        for child_node in node.children.iter() {
            height = self.draw_node(frame, child_node, height);
        }

        return height;
    }

    fn draw_block(&self, frame: &mut Frame, node: &Node, area: &Rect) -> Rect {
        assert!(node.heading.is_some());

        let mut heading = node.heading.clone().unwrap();
        heading = format!("{:width$}", heading, width = area.width as usize);

        let mut styled_title = Line::from(heading).style(Style::default());
        if self.selected_line == area.y {
            styled_title = styled_title.bg(Color::Gray).fg(Color::Black);
        }

        let block = Block::default().title(styled_title);
        let inner_area = block.inner(*area);
        frame.render_widget(block, *area);

        return inner_area;
    }

    fn try_draw_content(&self, frame: &mut Frame, node: &Node, area: &Rect) -> bool {
        let mut node_path = Vec::new();
        if !Node::find_path(&self.content_tree, node, &mut node_path) {
            panic!("Comparing nodes that are not in same tree.")
        }

        let mut rendered = false;
        if let Some(node_entry) = self.node_data.iter().find(|e| e.node_path == node_path) {
            if node_entry.visible {
                self.draw_content(frame, node, &area);
                rendered = true;
            }
        } else {
            // TODO: error for not found
        }

        return rendered;
    }

    fn draw_content(&self, frame: &mut Frame, node: &Node, area: &Rect) {
        let mut line_area = area.clone();

        for line in node.content.iter() {
            let mut line_widget = Line::from(line.clone());
            if self.selected_line == line_area.y {
                line_widget = line_widget.bg(Color::Gray).fg(Color::Black);
            }

            frame.render_widget(line_widget, line_area);

            line_area.y += 1;
        }
    }

    fn update_node_data(&mut self, node: &Node, area: &Rect) {
        let mut path = Vec::new();
        if !Node::find_path(&self.content_tree, node, &mut path) {
            panic!("Comparing nodes that are not in same tree.")
        }

        if self.add_new_node(path.clone(), area.top()) {
            return;
        }

        if let Some(idx) = self.node_data.iter().position(|e| *e.node_path == path) {
            self.node_data[idx].line_num = area.top();
        }
    }

    fn add_new_node(&mut self, path: NodePath, line: u16) -> bool {
        if !self.node_data.iter().any(|e| *e.node_path == *path) {
            let new_entry = NodeEntry {
                line_num: line,
                node_path: path,
                visible: true,
            };

            self.node_data.push(new_entry);
            return true;
        } else {
            return false;
        }
    }

    fn draw_timers(&self, frame: &mut Frame, node: &Node, visible: bool, area: &Rect) {
        assert!(node.content.len() == node.content_times.len());

        let mut line_area = area.clone();

        let mut total_seconds = 0;
        let initial_y = area.y;
        for time in node.content_times.iter() {
            if visible {
                let text = MarkdownView::format_time(time.as_secs(), 1);
                let mut line = Line::from(text);
                if self.selected_line == line_area.y {
                    line = line.bg(Color::Gray).fg(Color::Black);
                }

                frame.render_widget(line, line_area);

                line_area.y += 1;
            }

            total_seconds += time.as_secs();
        }

        let text = MarkdownView::format_time(total_seconds, 0);
        let mut line = Line::from(text);
        if self.selected_line == initial_y - 1 {
            line = line.bg(Color::Gray).fg(Color::Black);
        }

        let heading_area = Rect::new(area.x, initial_y - 1, area.width, area.height);
        frame.render_widget(line, heading_area)
    }

    fn format_time(total_seconds: u64, indent_level: usize) -> String {
        let hours = total_seconds / 3600;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let indent = format!("{:1$}", "", indent_level);
        return format!("{}[{:02}:{:02}:{:02}] ", indent, hours, minutes, seconds);
    }
}
