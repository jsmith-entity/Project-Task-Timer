use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub struct ControlView {
    controls: Vec<(String, String)>,
}

impl ControlView {
    pub fn new() -> Self {
        let controls = ControlView::define_controls();

        return Self { controls };
    }

    pub fn draw(&self, frame: &mut Frame, area: &Rect) {
        let line_length: u16 = 20;

        for (i, (title, desc)) in self.controls.iter().enumerate() {
            let x = area.x + (i as u16) * line_length;
            let line_area = Rect {
                x,
                y: area.y,
                width: line_length,
                height: area.height,
            };

            let mut shortcut: String = title.clone();
            shortcut.push(' ');

            let line = Line::from(vec![
                Span::styled(shortcut, Style::default().bg(Color::Gray).fg(Color::Black)),
                Span::raw(" "),
                Span::styled(desc, Style::default()),
            ]);

            frame.render_widget(line, line_area);
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
