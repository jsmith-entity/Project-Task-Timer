use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;

use serde::{Deserialize, Serialize};

use crate::task_timer::node::{Node, NodePath};

#[derive(Serialize, Deserialize, Clone)]
struct NodeEntry {
    pub line_num: u16,
    pub node_path: NodePath,
    pub visible: bool,
    pub task_lines: Vec<u16>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskView {
    pub selected_line: u16,

    content_tree: Node,
    node_data: Vec<NodeEntry>,
    drawn_nodes: Vec<NodePath>,
    completed_lines: Vec<u16>,

    current_node_task_lines: Vec<u16>,

    #[serde(skip)]
    area: Rect,
}

impl TaskView {
    pub fn new() -> Self {
        Self {
            selected_line: 2,

            content_tree: Node::new(),
            area: Rect::new(0, 0, 0, 0),
            node_data: Vec::new(),
            drawn_nodes: Vec::new(),
            completed_lines: Vec::new(),

            current_node_task_lines: Vec::new(),
        }
    }

    pub fn render(
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
            let indent = 0;
            height = self.draw_node(frame, &child_node, height, indent)
        }

        return (height, (self.drawn_nodes.clone(), self.completed_lines.clone()));
    }

    fn draw_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16, indent: usize) -> u16 {
        let block_height = node.content.len() as u16 + 2;

        let node_area = Rect::new(self.area.x, self.area.y + height, self.area.width, block_height);

        let inner_area = self.draw_block(frame, &node, &node_area, indent);

        let drawn = self.try_draw_content(frame, &node, &inner_area, indent);
        if drawn {
            height += block_height - 1;
        } else {
            height += 1;
        }

        self.update_node_data(node, &node_area, drawn);

        for child_node in node.children.iter() {
            height = self.draw_node(frame, child_node, height, indent + 1);
        }

        return height;
    }

    fn draw_block(&self, frame: &mut Frame, node: &Node, area: &Rect, indent: usize) -> Rect {
        assert!(node.heading.is_some());

        let mut heading = node.heading.clone().unwrap();
        let indent_space = " ".repeat(indent);
        heading = format!("{}{:width$}", indent_space, heading, width = area.width as usize);

        let mut styled_title = Line::from(heading).style(Style::default());
        if self.selected_line == area.y {
            styled_title = styled_title.bg(Color::Gray).fg(Color::Black);
        }

        let block = Block::default().title(styled_title);
        let inner_area = block.inner(*area);
        frame.render_widget(block, *area);

        return inner_area;
    }

    fn try_draw_content(&mut self, frame: &mut Frame, node: &Node, area: &Rect, indent: usize) -> bool {
        let mut node_path = Vec::new();
        if !Node::find_path(&self.content_tree, node, &mut node_path) {
            panic!("Comparing nodes that are not in same tree.")
        }

        let mut rendered = false;
        if let Some(node_entry) = self.node_data.iter().find(|e| e.node_path == node_path) {
            self.current_node_task_lines = Vec::new();

            if node_entry.visible {
                self.draw_content(frame, node, &area, indent);
                self.drawn_nodes.push(node_path.clone());
                rendered = true;
            } else {
                for _ in 0..node.children.len() {
                    // Placeholder indicating that the task line has not been rendered
                    //self.current_node_task_lines.push(0);
                }
            }
        }

        return rendered;
    }

    fn draw_content(&mut self, frame: &mut Frame, node: &Node, area: &Rect, indent: usize) {
        let mut line_area = area.clone();

        let indent_space = " ".repeat(indent);

        for (idx, line) in node.content.iter().enumerate() {
            let text = format!("{}{}", indent_space, line.clone());
            let mut line_widget = Line::from(text);

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

impl TaskView {
    pub fn try_collapse(&mut self) {
        if let Some(idx) = self
            .node_data
            .iter()
            .position(|e| e.line_num == self.selected_line)
        {
            self.node_data[idx].visible = !self.node_data[idx].visible;

            let found_node_depth = self.node_data[idx].node_path.len();

            let data_copy = self.node_data[idx].clone();
            for entry in self.node_data.iter_mut() {
                let target_node_depth = entry.node_path.len();
                if target_node_depth < found_node_depth {
                    continue;
                }

                let depth_diff = (target_node_depth - found_node_depth).max(1);

                let path = &entry.node_path[..entry.node_path.len().saturating_sub(depth_diff)];
                if path == data_copy.node_path {
                    entry.visible = !entry.visible;
                }
            }
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

    pub fn toggle_nodes(&mut self, visible: bool) {
        for entry in self.node_data.iter_mut() {
            entry.visible = visible;
        }
    }
}
