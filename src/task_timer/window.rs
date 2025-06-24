use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::task_timer::node::Node;

pub struct Window {
    pub file_name: String,
    content_tree: Node,
    selected_line: u16,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),
            content_tree: Node::new(),
            selected_line: 0,
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.content_tree = contents
    }

    pub fn render(&self, frame: &mut Frame) {
        let invalid_root = self.content_tree.heading == None
            && self.content_tree.content.len() == 0
            && self.content_tree.children.len() == 0;

        if invalid_root {
            panic!("Attempting to render a window where an invalid root node has been provided.");
        }

        let area = frame.area();

        let root_title = self.file_name.clone();
        let root_block = Block::default().title(root_title).borders(Borders::ALL);
        let inner_area = root_block.inner(area);

        frame.render_widget(root_block, area);

        self.render_node(frame, &inner_area, &self.content_tree, &mut 0);
    }

    pub fn select_line(&mut self, line_num: u16) {
        self.selected_line = line_num;
    }

    fn render_node(&self, frame: &mut Frame, frame_area: &Rect, node: &Node, y_offset: &mut u16) {
        let mut title = node.heading.clone().unwrap_or_else(|| "???".to_string());
        let content = node.content.clone();

        if title != "???" {
            let content_lines = content.len() as u16;
            let block_height = content_lines + 2;
            let area = Rect::new(
                frame_area.x,
                frame_area.y + *y_offset,
                frame_area.width,
                block_height,
            );

            // Format title so the element takes up all frame width
            title = format!("{:width$}", title, width = frame_area.width as usize);

            let mut styled_title = Line::from(title).style(Style::default());
            if self.selected_line == area.y {
                styled_title = styled_title.bg(Color::Gray);
            }

            let block = Block::default().title(styled_title);
            let mut inner_area = block.inner(area);

            frame.render_widget(block, area);

            for line in content {
                let mut line_widget = Line::from(line);
                if self.selected_line == inner_area.y {
                    line_widget = line_widget.bg(Color::Gray);
                }

                frame.render_widget(line_widget, inner_area);

                inner_area.y += 1;
            }

            *y_offset += block_height;
        }

        for child_node in node.children.iter() {
            self.render_node(frame, frame_area, child_node, y_offset);
        }
    }
}
