use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect, Stylize},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct NavigationBar {
    back_text: String,
    breadcrumbs: Vec<String>,
}

impl NavigationBar {
    pub fn new() -> Self {
        return Self {
            back_text: " (b) Back ".to_string(),
            breadcrumbs: Vec::new(),
        };
    }

    pub fn push_breadcrumb(&mut self, new_heading: String) {
        let heading = new_heading.trim_start_matches('#').trim().to_string();
        self.breadcrumbs.push(heading);
    }

    pub fn pop_breadcrumb(&mut self) {
        self.breadcrumbs.pop();
    }
}

impl Widget for &NavigationBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let back_len = self.back_text.len() as u16;
        let horizontal = Layout::horizontal([Length(back_len), Length(5), Min(0)]);
        let [back_area, _, breadcrumb_area] = horizontal.areas(area);

        Line::from(self.back_text.clone())
            .fg(Color::Black)
            .bg(Color::Gray)
            .render(back_area, buf);

        let mut breadcrumb_content = Vec::new();
        for (i, breadcrumb) in self.breadcrumbs.iter().enumerate() {
            if i > 0 {
                breadcrumb_content.push(Span::raw(" / "));
            }

            if i == self.breadcrumbs.len() - 1 {
                breadcrumb_content.push(Span::styled(
                    breadcrumb.to_string(),
                    Style::default().bold().fg(Color::Blue),
                ));
            } else {
                breadcrumb_content.push(Span::styled(
                    breadcrumb.to_string(),
                    Style::default().fg(Color::Blue),
                ));
            }
        }

        Line::from(breadcrumb_content).render(breadcrumb_area, buf);
    }
}
