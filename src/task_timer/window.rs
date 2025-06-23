use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::task_timer::node::Node;

pub struct Window {
    content_tree: Node,
}

// TODO: Layout created per heading
impl Window {
    pub fn new() -> Self {
        Self {
            content_tree: Node::new(),
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.content_tree = contents
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        Window::render_node(frame, &area, &self.content_tree, &mut 0);
    }

    fn render_node(frame: &mut Frame, frame_area: &Rect, node: &Node, y_offset: &mut u16) {
        let title = node.heading.clone().unwrap_or_else(|| "???".to_string());
        let content = node.content.clone().unwrap_or_else(|| "???".to_string());

        let content_lines = content.split("\n").count() as u16;
        let block_height = content_lines + 2;
        let area = Rect::new(
            frame_area.x,
            frame_area.y + *y_offset,
            frame_area.width,
            block_height,
        );

        let block = Block::default().title(title);
        let inner_area = block.inner(area);
        let body = Paragraph::new(content);

        frame.render_widget(block, area);
        frame.render_widget(body, inner_area);

        *y_offset += block_height;

        for child_node in node.children.iter() {
            Window::render_node(frame, frame_area, child_node, y_offset);
        }
    }
}
