use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub struct ControlView {
    // erm
    controls: Vec<(String, String)>,
}

impl ControlView {
    pub fn new() -> Self {
        let controls = ControlView::define_controls();

        return Self { controls };
    }

    pub fn draw(&self, frame: &mut Frame, area: &Rect) {
        let mut line_area = area.clone();

        for (title, desc) in self.controls.iter() {
            let mut shortcut: String = title.clone();
            shortcut.push(' ');

            let line = Line::from(vec![
                Span::styled(shortcut, Style::default().bg(Color::Gray).fg(Color::Black)),
                Span::raw(" "),
                Span::styled(desc, Style::default()),
            ]);

            frame.render_widget(line, line_area);
            //e rm
            line_area.y += 1;
        }
    }

    fn define_controls() -> Vec<(String, String)> {
        return vec![
            ("↵".to_string(), "Toggle Heading".to_string()),
            ("␣".to_string(), "Complete Task".to_string()),
            ("s".to_string(), "Toggle Time".to_string()),
        ];
    }
}
