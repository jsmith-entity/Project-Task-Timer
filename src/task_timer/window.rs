use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};

use crate::task_timer::node::{Node, NodePath};

pub struct Window {
    pub file_name: String,
    pub to_collapse: bool,

    content_tree: Node,

    area_bounds: Rect,
    content_height: u16,
    selected_line: u16,

    collapsed_heading_paths: Vec<NodePath>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),
            content_tree: Node::new(),
            area_bounds: Rect::new(0, 0, 0, 0),
            content_height: 0,
            selected_line: 1,
            collapsed_heading_paths: Vec::new(),
            to_collapse: false,
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.content_tree = contents
    }

    pub fn content_height(&self) -> u16 {
        return self.content_height;
    }

    pub fn render(&mut self, frame: &mut Frame) {
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
        self.area_bounds = inner_area.clone();

        frame.render_widget(root_block, area);

        let root_node = &self.content_tree.clone();
        let new_height = self.render_node(frame, root_node, 0);

        self.content_height = new_height;
    }

    pub fn select_line(&mut self, line_num: u16) {
        let win_max_height = self.area_bounds.y + self.area_bounds.height;

        let lower_bound = self.area_bounds.y;
        let upper_bound = if self.content_height < win_max_height {
            self.content_height + 1
        } else {
            win_max_height
        };

        let within_bounds = line_num >= lower_bound && line_num < upper_bound;
        if within_bounds {
            self.selected_line = line_num;
        }
    }

    fn render_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        let frame_area = self.area_bounds;

        let mut title = node.heading.clone().unwrap_or_else(|| "???".to_string());
        let content = node.content.clone();

        if title != "???" {
            let content_lines = content.len() as u16;
            let block_height = content_lines + 2;
            let area = Rect::new(
                frame_area.x,
                frame_area.y + height,
                frame_area.width,
                block_height,
            );

            // Format title so the element takes up all frame width
            title = format!("{:width$}", title, width = frame_area.width as usize);

            let (inner_area, collapse_block) = self.render_node_block(frame, title, &area);

            let mut node_path: NodePath = Vec::new();
            if Node::find_path(&self.content_tree, node, &mut node_path) {
                let already_hidden = self.collapsed_heading_paths.contains(&node_path);
                if collapse_block && already_hidden {
                    // To show a hidden block
                    self.collapsed_heading_paths
                        .retain(|path| *path != node_path);
                } else if collapse_block {
                    // To hide a hidden block
                    self.collapsed_heading_paths.push(node_path.clone());
                }

                if !self.collapsed_heading_paths.contains(&node_path) {
                    self.render_node_content(frame, &content, &inner_area);
                    height += block_height - 1;
                } else {
                    height += 1;
                }
            } else {
                // TODO: write error - comparing two unrelated nodes
            }
        }

        for child_node in node.children.iter() {
            height = self.render_node(frame, child_node, height);
        }

        return height;
    }

    fn render_node_block(&mut self, frame: &mut Frame, title: String, area: &Rect) -> (Rect, bool) {
        let mut styled_title = Line::from(title).style(Style::default());
        if self.selected_line == area.y {
            styled_title = styled_title.bg(Color::Gray);
        }

        let block = Block::default().title(styled_title);
        let inner_area = block.inner(*area);
        frame.render_widget(block, *area);

        let is_selected = area.top() == self.selected_line;
        let collapse_selected_block = self.to_collapse && is_selected;

        return (inner_area, collapse_selected_block);
    }

    fn render_node_content(&self, frame: &mut Frame, content: &Vec<String>, area: &Rect) {
        let mut line_area = area.clone();

        for line in content.iter() {
            let mut line_widget = Line::from(line.clone());
            if self.selected_line == line_area.y {
                line_widget = line_widget.bg(Color::Gray);
            }

            frame.render_widget(line_widget, line_area);

            line_area.y += 1;
        }
    }
}
