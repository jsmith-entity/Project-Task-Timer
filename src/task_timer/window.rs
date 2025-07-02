use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::widgets::{Block, Borders};

use crate::task_timer::markdown_view::MarkdownView;
use crate::task_timer::node::Node;

pub struct Window {
    pub file_name: String,

    area_bounds: Rect,
    selected_line: u16,
    content_height: u16,

    task_list: MarkdownView,
}

impl Window {
    pub fn new() -> Self {
        Self {
            file_name: "???".to_string(),

            area_bounds: Rect::new(0, 0, 0, 0),
            selected_line: 1,
            content_height: 0,

            task_list: MarkdownView::new(),
        }
    }

    pub fn update_contents(&mut self, contents: Node) {
        self.task_list.update(contents);
    }

    pub fn content_height(&self) -> u16 {
        return self.content_height;
    }

    pub fn try_collapse_heading(&mut self) {
        self.task_list.try_collapse();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let root_title = self.file_name.clone();
        let root_block = Block::default().title(root_title).borders(Borders::ALL);

        // TODO: Consider removing if not needed for bottom vertical view
        self.area_bounds = root_block.inner(area);

        frame.render_widget(root_block, area);
        let new_height = self.task_list.draw(frame, &self.area_bounds);

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
