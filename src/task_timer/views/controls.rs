use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ControlView {
    controls: Vec<(String, String)>,
}

impl ControlView {
    pub fn new() -> Self {
        let controls = ControlView::define_controls();

        return Self { controls };
    }

    pub fn draw(&self, frame: &mut Frame, area: &Rect) {
        assert!(self.controls.len() > 0);

        let mut temp_area = area.clone();
        let line_length: u16 = 20;

        let mut x: u16 = area.x;

        for (title, desc) in self.controls.iter() {
            if x + line_length > area.x + area.width {
                x = area.x;
                temp_area.y += 1;
            }

            let line_area = Rect {
                x,
                y: temp_area.y,
                width: line_length,
                height: temp_area.height,
            };

            let mut shortcut: String = title.clone();
            shortcut.push(' ');

            let line = Line::from(vec![
                Span::styled(shortcut, Style::default().bg(Color::Gray).fg(Color::Black)),
                Span::raw(" "),
                Span::styled(desc, Style::default()),
            ]);

            frame.render_widget(line, line_area);

            x += line_length;
        }
    }

    fn define_controls() -> Vec<(String, String)> {
        return vec![
            ("↵".to_string(), "Toggle Heading".to_string()),
            ("␣".to_string(), "Complete Task".to_string()),
            ("s".to_string(), "Toggle Time".to_string()),
            ("o".to_string(), "Open Headings".to_string()),
            ("c".to_string(), "Close Headings".to_string()),
        ];
    }
}
