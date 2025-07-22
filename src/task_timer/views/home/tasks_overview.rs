use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

use crate::task_timer::{node::Node, views::log::log_type::InfoSubType};

#[derive(Default, Clone)]
pub struct TaskOverview {
    pub tasks: Vec<(bool, String)>,
    pub subheadings: Vec<String>,
    pub selected_line: u16,
    pub content_height: u16,
}

impl TaskOverview {
    pub fn new(node: &Node) -> Self {
        let tasks: Vec<(bool, String)> = node
            .completed_tasks
            .iter()
            .cloned()
            .zip(node.content.iter().cloned())
            .collect();
        let subheadings: Vec<_> = node.children.iter().filter_map(|e| e.heading.clone()).collect();
        let selected_line = 1;
        let content_height = tasks.len() as u16 + subheadings.len() as u16;

        return Self {
            tasks,
            subheadings,
            selected_line,
            content_height,
        };
    }

    fn render_tasks(&self, area: Rect, buf: &mut Buffer) -> u16 {
        let mut task_offset: u16 = 0;
        for (completed, task) in self.tasks.iter() {
            let mut style = Style::default();
            if task_offset + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }
            if *completed {
                style = style.fg(Color::DarkGray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + task_offset,
                width: area.width,
                height: 1,
            };

            Line::from(task.clone()).style(style).render(display_area, buf);

            task_offset += 1;
        }

        return task_offset;
    }

    fn render_subheadings(&self, area: Rect, buf: &mut Buffer, task_offset: u16) {
        for (idx, subheading) in self.subheadings.iter().enumerate() {
            let mut style = Style::default();
            if task_offset + idx as u16 + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
            }

            let display_area = Rect {
                x: area.x,
                y: area.y + task_offset + idx as u16,
                width: area.width,
                height: 1,
            };

            Line::from(subheading.clone())
                .style(style)
                .render(display_area, buf);
        }
    }
}

impl Widget for &TaskOverview {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let task_offset = self.render_tasks(area, buf);
        self.render_subheadings(area, buf, task_offset);
    }
}
