use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

pub struct TaskOverview {
    pub tasks: Vec<String>,
    pub subheadings: Vec<String>,
    pub selected_line: u16,
    pub content_height: u16,
}

impl TaskOverview {
    fn render_tasks(&self, area: Rect, buf: &mut Buffer) -> u16 {
        let mut task_offset: u16 = 0;
        for task in self.tasks.iter() {
            let mut style = Style::default();
            if task_offset + 1 == self.selected_line {
                style = style.fg(Color::Black).bg(Color::Gray);
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

    fn render_subheadings(&self, area: Rect, buf: &mut Buffer) {}
}

impl Widget for &TaskOverview {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // define areas
    }
}
