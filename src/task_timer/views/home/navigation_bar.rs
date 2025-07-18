use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::Widget,
};

#[derive(Default, Clone)]
pub struct NavigationBar {
    // erm
    back_text: String,
}

impl NavigationBar {
    pub fn new() -> Self {
        return Self {
            back_text: " (b) Back ".to_string(),
        };
    }
}

impl Widget for &NavigationBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let back_len = self.back_text.len() as u16;
        let horizontal = Layout::horizontal([Length(back_len), Min(0)]);
        let [back_area, breadcrumb_area] = horizontal.areas(area);

        Line::from(self.back_text.clone())
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(back_area, buf);
    }
}
