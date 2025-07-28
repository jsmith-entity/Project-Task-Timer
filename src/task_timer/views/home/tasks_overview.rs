use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use serde::{Deserialize, Serialize};

use crate::task_timer::node::Node;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TaskOverview {
    pub selected_line: u16,
    pub content_height: u16,
    pub lines: Vec<(bool, String)>,
    pub task_offset: usize,

    page_start: usize,
    page_end: usize,
}

impl TaskOverview {
    pub fn new(node: &Node, completed_subheadings: &Vec<bool>) -> Self {
        assert!(node.children.len() == completed_subheadings.len());

        let tasks: Vec<(bool, String)> = node
            .completed_tasks
            .iter()
            .cloned()
            .zip(node.content.iter().cloned())
            .collect();

        let subheadings: Vec<(bool, String)> = completed_subheadings
            .iter()
            .zip(node.children.iter())
            .filter_map(|(completed, e)| e.heading.clone().map(|h| (*completed, h)))
            .collect();

        let content_height = tasks.len() as u16 + subheadings.len() as u16;

        return Self {
            selected_line: 1,
            content_height,
            lines: tasks.clone().into_iter().chain(subheadings.into_iter()).collect(),
            task_offset: tasks.len(),

            page_start: 0,
            page_end: 0,
        };
    }

    pub fn slice_bounds(&mut self, start_idx: usize, end_idx: usize) {
        self.page_start = start_idx;
        self.page_end = end_idx;
    }
}

impl Widget for &TaskOverview {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let entry_slice = &self.lines[self.page_start..self.page_end];
        for (idx, (completed, subheading)) in entry_slice.iter().enumerate() {
            let mut style = Style::default();
            if idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }
            if *completed {
                style = style.fg(Color::DarkGray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(subheading.clone())
                .style(style)
                .render(display_area, buf);
        }
    }
}
