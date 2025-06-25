use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders};

use crate::task_timer::markdown_view::MarkdownView;
use crate::task_timer::node::Node;

pub struct Window {
    pub file_name: String,

    area_bounds: Rect,
    task_list: MarkdownView,

    selected_line: u16,
    content_height: u16,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),

            area_bounds: Rect::new(0, 0, 0, 0),
            task_list: MarkdownView::new(),
            selected_line: 1,
            content_height: 0,
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.task_list.update_contents(contents);
    }

    pub fn content_height(&self) -> u16 {
        return self.content_height;
    }

    pub fn collapse_heading(&mut self, collapse: bool) {
        self.task_list.to_collapse = collapse;
    }

    pub fn to_collapse(&self) -> bool {
        return self.task_list.to_collapse;
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let invalid_root = self.task_list.content_tree.heading == None
            && self.task_list.content_tree.content.len() == 0
            && self.task_list.content_tree.children.len() == 0;

        if invalid_root {
            panic!("Attempting to render a window where an invalid root node has been provided.");
        }

        let area = frame.area();
        let root_title = self.file_name.clone();
        let root_block = Block::default().title(root_title).borders(Borders::ALL);

        let inner_area = root_block.inner(area);
        self.area_bounds = inner_area.clone();
        self.task_list.area_bounds = self.area_bounds;

        frame.render_widget(root_block, area);

        let root_node = &self.task_list.content_tree.clone();
        let new_height = self.task_list.render_node(frame, root_node, 0);

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
            self.task_list.selected_line = self.selected_line;
        }
    }
}
