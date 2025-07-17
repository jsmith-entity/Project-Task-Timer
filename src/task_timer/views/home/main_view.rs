use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::task_timer::{
    node::{Node, NodePath},
    window::{RenderedNode, RenderedNodeType},
};

#[derive(Default, Clone)]
pub struct MainView {
    // timers
    // tasks
    // status
    pub content_area: Rect,
    root_node: Node,
    display_data: Vec<RenderedNode>,
    selected_line: u16,
}

impl MainView {
    pub fn new() -> Self {
        return Self {
            content_area: Rect::default(),
            root_node: Node::new(),
            display_data: Vec::new(),
            selected_line: 1,
        };
    }

    pub fn collect_display_data(root_node: &Node, node: &Node) -> Result<Vec<RenderedNode>, String> {
        let mut data = Vec::new();

        let mut node_path = Vec::new();
        if !Node::find_path(root_node, node, &mut node_path) {
            return Err(
                "Comparing nodes that do not belong on the same tree when collecting display data"
                    .to_string(),
            );
        }

        if node.heading.is_some() {
            data.push(RenderedNode {
                node_type: RenderedNodeType::Heading,
                node_path: node_path.clone(),
            })
        } else {
            // root node
            data.push(RenderedNode {
                node_type: RenderedNodeType::Heading,
                node_path: Vec::new(),
            })
        }

        for (idx, _) in node.content.iter().enumerate() {
            data.push(RenderedNode {
                node_type: RenderedNodeType::Task(idx),
                node_path: node_path.clone(),
            });
        }

        for child_node in &node.children {
            let mut child_node_path = Vec::new();
            if !Node::find_path(root_node, child_node, &mut child_node_path) {
                return Err(
                    "Comparing nodes that do not belong on the same tree when collecting display data"
                        .to_string(),
                );
            }

            data.push(RenderedNode {
                node_type: RenderedNodeType::ChildHeading(child_node_path),
                node_path: node_path.clone(),
            })
        }

        return Ok(data);
    }

    pub fn update(
        &mut self,
        content_area: Rect,
        root_node: Node,
        display_data: Vec<RenderedNode>,
        selected_line: u16,
    ) {
        self.content_area = content_area;
        self.root_node = root_node;
        self.display_data = display_data;
        self.selected_line = selected_line;
    }

    pub fn get_subheading_path(&self, pos: usize) -> Option<NodePath> {
        let task_offset = self
            .display_data
            .iter()
            .filter(|e| matches!(e.node_type, RenderedNodeType::Task(_)))
            .count();

        let mut subheading_idx = 0;
        for entry in &self.display_data {
            if let RenderedNodeType::ChildHeading(ref child_path) = entry.node_type {
                let line = task_offset + subheading_idx + 1;
                if line == pos {
                    return Some(child_path.clone());
                }

                subheading_idx += 1;
            }
        }

        return None;
    }
}

impl Widget for &MainView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // TODO: render breadcrumbs
        // TODO: render tabs
        //
        // TODO: get heading in display data (should only ever be 1) display that separately

        let mut entry_idx = 0;
        for to_render in self.display_data.iter() {
            let node = self.root_node.get_node(&to_render.node_path).unwrap().clone();

            let mut style = Style::default();
            if entry_idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }

            let content = match &to_render.node_type {
                RenderedNodeType::Task(task_num) => Some(node.content.get(*task_num).unwrap().to_string()),
                RenderedNodeType::ChildHeading(child_path) => {
                    let child_node_heading = self.root_node.get_node(&child_path).unwrap().heading.clone();
                    Some(child_node_heading.unwrap_or("<no heading>".to_string()))
                }
                _ => None,
            };

            if content.is_none() {
                continue;
            }

            let pos_y = area.y + entry_idx as u16;
            if pos_y < area.y + area.height {
                let line = Line::from(content.unwrap()).style(style);

                let display_area = Rect {
                    x: area.x,
                    y: pos_y,
                    width: area.width,
                    height: 1,
                };

                line.render(display_area, buf);
            }

            entry_idx += 1;
        }
    }
}
