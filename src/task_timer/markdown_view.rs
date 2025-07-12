use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;

use crate::task_timer::node::{Node, NodePath};

struct NodeEntry {
    pub line_num: u16,
    pub node_path: NodePath,
    pub visible: bool,
    pub task_lines: Vec<u16>,
}

pub struct MarkdownView {
    pub selected_line: u16,

    content_tree: Node,
    area: Rect,
    node_data: Vec<NodeEntry>,
    drawn_nodes: Vec<NodePath>,
    completed_lines: Vec<u16>,

    current_node_task_lines: Vec<u16>,
}

impl MarkdownView {
    pub fn new() -> Self {
        Self {
            selected_line: 1,

            content_tree: Node::new(),
            area: Rect::new(0, 0, 0, 0),
            node_data: Vec::new(),
            drawn_nodes: Vec::new(),
            completed_lines: Vec::new(),

            current_node_task_lines: Vec::new(),
        }
    }

    pub fn try_collapse(&mut self) {
        if let Some(idx) = self
            .node_data
            .iter()
            .position(|e| e.line_num == self.selected_line)
        {
            self.node_data[idx].visible = !self.node_data[idx].visible;
        }
    }

    pub fn selected_task(&mut self) -> Option<(usize, &NodePath)> {
        let matches: Vec<_> = self
            .node_data
            .iter()
            .filter_map(|node| {
                node.task_lines
                    .iter()
                    .position(|&line| line == self.selected_line)
                    .map(|task_idx| (task_idx, &node.node_path))
            })
            .collect();

        assert!(matches.len() <= 1);

        return matches.into_iter().next();
    }

    pub fn draw(
        &mut self,
        frame: &mut Frame,
        area: &Rect,
        content_tree: &Node,
    ) -> (u16, (Vec<NodePath>, Vec<u16>)) {
        self.drawn_nodes = Vec::new();
        self.completed_lines = Vec::new();
        self.content_tree = content_tree.clone();
        self.area = area.clone();

        let mut height = 0;

        let children: Vec<_> = self.content_tree.children.iter().cloned().collect();
        for child_node in children {
            height = self.draw_node(frame, &child_node, height)
        }

        return (height, (self.drawn_nodes.clone(), self.completed_lines.clone()));
    }

    fn draw_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        let block_height = node.content.len() as u16 + 2;

        let node_area = Rect::new(self.area.x, self.area.y + height, self.area.width, block_height);

        let inner_area = self.draw_block(frame, &node, &node_area);

        let drawn = self.try_draw_content(frame, &node, &inner_area);
        if drawn {
            height += block_height - 1;
        } else {
            height += 1;
        }

        self.update_node_data(node, &node_area, drawn);

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

    fn try_draw_content(&mut self, frame: &mut Frame, node: &Node, area: &Rect) -> bool {
        let mut node_path = Vec::new();
        if !Node::find_path(&self.content_tree, node, &mut node_path) {
            panic!("Comparing nodes that are not in same tree.")
        }

        let mut rendered = false;
        if let Some(node_entry) = self.node_data.iter().find(|e| e.node_path == node_path) {
            self.current_node_task_lines = Vec::new();

            if node_entry.visible {
                self.draw_content(frame, node, &area);
                self.drawn_nodes.push(node_path.clone());
                rendered = true;
            } else {
                for _ in 0..node.children.len() {
                    // Placeholder indicating that the task line has not been rendered
                    self.current_node_task_lines.push(u16::MAX);
                }
            }
        }

        return rendered;
    }

    fn draw_content(&mut self, frame: &mut Frame, node: &Node, area: &Rect) {
        let mut line_area = area.clone();

        for (idx, line) in node.content.iter().enumerate() {
            let mut line_widget = Line::from(line.clone());

            if self.selected_line == line_area.y {
                line_widget = line_widget.bg(Color::Gray).fg(Color::Black);
            }

            if node.completed_tasks[idx] == true {
                line_widget = line_widget
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::CROSSED_OUT);
                self.completed_lines.push(line_area.y);
            }

            frame.render_widget(line_widget, line_area);

            self.current_node_task_lines.push(line_area.y);

            line_area.y += 1;
        }
    }

    fn update_node_data(&mut self, node: &Node, area: &Rect, drawn: bool) {
        let mut path = Vec::new();
        if !Node::find_path(&self.content_tree, node, &mut path) {
            panic!("Comparing nodes that are not in same tree.")
        }

        let data_entry = NodeEntry {
            line_num: area.top(),
            node_path: path.clone(),
            visible: drawn,
            task_lines: self.current_node_task_lines.clone(),
        };

        if let Some(idx) = self
            .node_data
            .iter()
            .position(|e| e.node_path == data_entry.node_path)
        {
            if self.node_data[idx].visible != data_entry.visible {
                self.node_data[idx].visible = data_entry.visible;
            }

            if self.node_data[idx].task_lines != data_entry.task_lines {
                self.node_data[idx].task_lines = data_entry.task_lines;
            }

            self.node_data[idx].line_num = area.top();
        } else {
            self.node_data.push(data_entry);
        }
    }
}
