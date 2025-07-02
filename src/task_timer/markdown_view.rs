use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::prelude::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::Block;
use std::rc::Rc;

use crate::task_timer::node::{Node, NodePath};

pub struct MarkdownView {
    pub to_collapse: bool,
    pub content_tree: Node,
    pub collapsed_heading_paths: Vec<NodePath>,
    pub selected_line: u16,

    layout: Layout,
    time_area: Option<Rect>,
    task_area: Option<Rect>,
}

impl MarkdownView {
    pub fn new() -> Self {
        Self {
            content_tree: Node::new(),
            selected_line: 1,
            collapsed_heading_paths: Vec::new(),
            to_collapse: false,

            layout: Layout::new(
                Direction::Horizontal,
                [Constraint::Length(12), Constraint::Min(0)],
            ),
            time_area: None,
            task_area: None,
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.content_tree = contents
    }

    pub fn render(&mut self, frame: &mut Frame, area: &Rect) -> u16 {
        let invalid_root = self.content_tree.heading == None
            && self.content_tree.content.len() == 0
            && self.content_tree.children.len() == 0;

        if invalid_root {
            panic!("Attempting to render a window where an invalid root node has been provided.");
        }

        let areas = self.layout.split(*area);
        self.time_area = Some(areas[0]);
        self.task_area = Some(areas[1]);

        let root_node = &self.content_tree.clone();
        let new_height = self.render_node(frame, root_node, 0);

        return new_height;
    }

    fn render_node(&mut self, frame: &mut Frame, node: &Node, mut height: u16) -> u16 {
        if node.heading.is_none() {
            for child_node in node.children.iter() {
                height = self.render_node(frame, child_node, height);
            }

            return height;
        }

        // FIX: is there a case where theres no node title
        assert!(node.heading.is_some());
        let mut heading = node.heading.clone().unwrap();
        let content = node.content.clone();

        let content_lines = content.len() as u16;
        let block_height = content_lines + 2;

        assert!(self.time_area.is_some());
        assert!(self.task_area.is_some());
        let time_area = self.time_area.unwrap();
        let task_area = self.task_area.unwrap();
        let areas = vec![
            Rect::new(
                time_area.x,
                time_area.y + height,
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

        // Format title so the element takes up all frame width
        heading = format!("{:width$}", heading, width = areas[1].width as usize);

        let (inner_area, collapse_block) = self.render_node_block(frame, &heading, &areas[1]);

        let mut node_path: NodePath = Vec::new();
        if Node::find_path(&self.content_tree, node, &mut node_path) {
            let already_hidden = self.collapsed_heading_paths.contains(&node_path);
            if collapse_block && already_hidden {
                // Expanding heading block => remove from collapsed nodes
                self.collapsed_heading_paths
                    .retain(|path| *path != node_path);
            } else if collapse_block {
                // Hiding heading block => add to collapsed nodes
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

        for child_node in node.children.iter() {
            height = self.render_node(frame, child_node, height);
        }

        return height;
    }

    fn render_node_block(&mut self, frame: &mut Frame, title: &str, area: &Rect) -> (Rect, bool) {
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

    fn render_node_content(&mut self, frame: &mut Frame, content: &Vec<String>, area: &Rect) {
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
