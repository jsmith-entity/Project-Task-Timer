use ratatui::{Frame, prelude::Rect, text::Line};

use super::super::node::{Node, NodePath};

#[derive(Default)]
pub struct TaskStatus {
    area: Rect,
    drawn_nodes: Vec<NodePath>,
    completed_tasks: Vec<u16>,
    active_lines: Vec<u16>,

    offset_y: u16,
}

impl TaskStatus {
    pub fn new() -> Self {
        return Self {
            area: Rect::new(0, 0, 0, 0),
            drawn_nodes: Vec::new(),
            completed_tasks: Vec::new(),
            active_lines: Vec::new(),

            offset_y: 1,
        };
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        root_node: &Node,
        drawn_data: &(Vec<NodePath>, Vec<u16>),
        active_lines: &Vec<u16>,
    ) {
        self.area = area;
        self.drawn_nodes = drawn_data.0.clone();
        self.completed_tasks = drawn_data.1.clone();
        self.active_lines = active_lines.clone();

        self.offset_y = 0;

        for node in root_node.children.iter() {
            self.try_render_status(frame, root_node, node);
        }
    }

    fn try_render_status(&mut self, frame: &mut Frame, root_node: &Node, node: &Node) {
        let mut node_path = Vec::new();
        if !Node::find_path(root_node, node, &mut node_path) {
            panic!("Comparing nodes that are not in the same tree");
        }

        let render_content = self.drawn_nodes.contains(&node_path);

        self.render_status(frame, node, render_content);

        for child_node in node.children.iter() {
            self.try_render_status(frame, root_node, child_node);
        }
    }

    fn render_status(&mut self, frame: &mut Frame, node: &Node, render_content: bool) {
        let content_height = if render_content {
            node.content.len() as u16 + 1
        } else {
            1 as u16
        };

        if render_content {
            let mut area = Rect::new(
                self.area.x + 1,
                self.area.y + self.offset_y + 1,
                self.area.width,
                content_height,
            );
            for _ in node.content.iter() {
                if self.active_lines.contains(&area.y) {
                    frame.render_widget(Line::from("ACTIVE"), area);
                }

                area.y += 1;
            }
        }

        self.offset_y += content_height;
    }
}
