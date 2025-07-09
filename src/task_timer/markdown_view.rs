use ratatui::Frame;
use ratatui::prelude::Rect;
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
    area: Rect,
    node_data: Vec<NodeEntry>,
    drawn_nodes: Vec<NodePath>,
}

impl MarkdownView {
    pub fn new() -> Self {
        Self {
            selected_line: 1,

            content_tree: Node::new(),
            area: Rect::new(0, 0, 0, 0),
            node_data: Vec::new(),
            drawn_nodes: Vec::new(),
        }
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

    pub fn draw(&mut self, frame: &mut Frame, area: &Rect, content_tree: &Node) -> (u16, Vec<NodePath>) {
        self.drawn_nodes = Vec::new();
        self.content_tree = content_tree.clone();
        self.area = area.clone();

        let mut height = 0;

        let children: Vec<_> = self.content_tree.children.iter().cloned().collect();
        for child_node in children {
            height = self.draw_node(frame, &child_node, height)
        }

        return (height, self.drawn_nodes.clone());
    }

    fn draw_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        let block_height = node.content.len() as u16 + 2;

        let node_area = Rect::new(self.area.x, self.area.y + height, self.area.width, block_height);

        let inner_area = self.draw_block(frame, &node, &node_area);

        let (drawn, node_path) = self.try_draw_content(frame, &node, &inner_area);
        if drawn {
            height += block_height - 1;
            self.drawn_nodes.push(node_path);
        } else {
            height += 1;
        }

        self.update_node_data(node, &node_area);

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

    fn try_draw_content(&self, frame: &mut Frame, node: &Node, area: &Rect) -> (bool, NodePath) {
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

        return (rendered, node_path);
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
}
