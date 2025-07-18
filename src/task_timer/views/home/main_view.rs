use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crossterm::event::KeyCode;

use crate::task_timer::{
    node::{Node, NodePath},
    views::home::navigation_bar::NavigationBar,
};

#[derive(Clone, Debug, PartialEq)]
pub enum RenderedNodeType {
    Heading(String),
    Task(usize),            // Index of the task in node.content
    ChildHeading(NodePath), // Immediate child heading
}

#[derive(Clone, Debug)]
pub struct RenderedNode {
    pub node_type: RenderedNodeType,
    pub node_path: NodePath,
}

#[derive(Default, Clone)]
pub struct MainView {
    pub root_node: Node,
    pub content_area: Rect,

    displayed_node: Node,
    display_data: Vec<RenderedNode>,

    selected_line: u16,
    content_height: u16,

    nav_bar: NavigationBar,
}

impl MainView {
    pub fn new() -> Self {
        return Self {
            root_node: Node::new(),
            content_area: Rect::default(),

            displayed_node: Node::new(),
            display_data: Vec::new(),

            selected_line: 1,
            content_height: 0,

            nav_bar: NavigationBar::new(),
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
                node_type: RenderedNodeType::Heading(node.heading.clone().unwrap()),
                node_path: node_path.clone(),
            })
        } else {
            // Root node
            data.push(RenderedNode {
                node_type: RenderedNodeType::Heading("Root Node Placeholder".to_string()),
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

    pub fn update_display_data(&mut self, new_display_node: Node) -> Result<(), String> {
        self.displayed_node = new_display_node;
        let res = MainView::collect_display_data(&self.root_node, &self.displayed_node);

        return match res {
            Ok(new_data) => {
                assert!(new_data.len() > 0);

                self.display_data = new_data;

                // Subtracting 1 as the heading will not take up content height
                self.content_height = self.display_data.len() as u16 - 1;
                Ok(())
            }
            Err(e) => Err(e),
        };
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

    pub fn handle_events(&mut self, key_code: KeyCode) -> Result<(), String> {
        return match key_code {
            KeyCode::Char('j') => {
                self.select_line(self.selected_line + 1);
                return Ok(());
            }
            KeyCode::Char('k') => {
                self.select_line(self.selected_line - 1);
                return Ok(());
            }
            // KeyCode::Char('s') => self.timers.try_activate(),
            // KeyCode::Char(' ') => self.update_completed_task(),
            KeyCode::Char('b') => self.enter_prev_node(),
            KeyCode::Enter => return self.enter_next_node(),
            _ => Ok(()),
        };
    }

    fn select_line(&mut self, line_num: u16) {
        if line_num > 0 && line_num <= self.content_height {
            self.selected_line = line_num;
        }
    }

    fn enter_prev_node(&mut self) -> Result<(), String> {
        // TODO: update breadcrumb

        let mut curr_node_path = Vec::new();
        if !Node::find_path(&self.root_node, &self.displayed_node, &mut curr_node_path) {
            return Err(
                "Comparing nodes that do not belong on the same tree when collecting display data"
                    .to_string(),
            );
        }

        curr_node_path.pop();
        if let Some(new_node) = self.root_node.get_node(&curr_node_path) {
            match self.update_display_data(new_node.clone()) {
                Ok(()) => {
                    self.nav_bar.pop_breadcrumb();
                    self.selected_line = 1;
                }
                Err(e) => return Err(e),
            }
        } else {
            return Err("Failed to convert node path to node when entering previous heading".to_string());
        }

        return Ok(());
    }

    fn enter_next_node(&mut self) -> Result<(), String> {
        // TODO:update breadcrumb

        if let Some(new_node_path) = self.get_subheading_path(self.selected_line as usize) {
            if let Some(new_node) = self.root_node.get_node(&new_node_path) {
                match self.update_display_data(new_node.clone()) {
                    Ok(()) => {
                        self.add_breadcrumb();
                        self.selected_line = 1;
                    }
                    Err(e) => return Err(e),
                }
            } else {
                return Err("Failed to convert node path to node when entering subheading".to_string());
            }
        } else {
            return Err(
                "Failed to retrieve subheading from display data when entering subheading".to_string(),
            );
        }

        return Ok(());
    }

    fn add_breadcrumb(&mut self) {
        let mut new_heading_name = None;
        for entry in &self.display_data {
            if let RenderedNodeType::Heading(ref name) = entry.node_type {
                new_heading_name = Some(name)
            }
        }

        if new_heading_name.is_some() {
            let new_breadcrumb = new_heading_name.unwrap().to_string();
            self.nav_bar.push_breadcrumb(new_breadcrumb);
        }
    }
}

impl Widget for &MainView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Length(3), Min(0)]);
        let [navigation_row, content_area] = vertical.areas(area);
        // TODO: get heading in display data (should only ever be 1) display that separately

        self.nav_bar.render(navigation_row, buf);

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

            let pos_y = content_area.y + entry_idx as u16;
            if pos_y < content_area.y + content_area.height {
                let line = Line::from(content.unwrap()).style(style);

                let display_area = Rect {
                    x: content_area.x,
                    y: pos_y,
                    width: content_area.width,
                    height: 1,
                };

                line.render(display_area, buf);
            }

            entry_idx += 1;
        }
    }
}
