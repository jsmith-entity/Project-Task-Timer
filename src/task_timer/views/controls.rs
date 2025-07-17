use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ControlView {
    line_length: u16,
    main_controls: Vec<(String, String)>,
    log_controls: Vec<(String, String)>,
}

impl ControlView {
    pub fn new() -> Self {
        let main_controls = ControlView::define_main_controls();
        let log_controls = ControlView::define_log_controls();

        return Self {
            line_length: 20,
            main_controls,
            log_controls,
        };
    }

    pub fn draw(&self, frame: &mut Frame, mut area: Rect) {
        assert!(self.main_controls.len() > 0);
        assert!(self.log_controls.len() > 0);

        frame.render_widget(Line::from("MAIN"), area);
        area.y += 2;
        self.render_controls(frame, &self.main_controls, &mut area);
        frame.render_widget(Line::from("-".repeat(self.line_length as usize)), area);
        area.y += 2;

        frame.render_widget(Line::from("LOG"), area);
        area.y += 2;
        self.render_controls(frame, &self.log_controls, &mut area);
    }

    fn render_controls(&self, frame: &mut Frame, controls: &Vec<(String, String)>, area: &mut Rect) {
        let mut x: u16 = area.x;

        for (title, desc) in controls.iter() {
            if x + self.line_length > area.x + area.width {
                x = area.x;
                area.y += 1;
            }

            let line_area = Rect {
                x,
                y: area.y,
                width: self.line_length,
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

            x += self.line_length;
        }

        area.y += 2;
    }

    fn define_main_controls() -> Vec<(String, String)> {
        return vec![
            ("↵".to_string(), "Toggle Heading".to_string()),
            ("␣".to_string(), "Complete Task".to_string()),
            ("s".to_string(), "Toggle Time".to_string()),
            ("o".to_string(), "Open Headings".to_string()),
            ("c".to_string(), "Close Headings".to_string()),
        ];
    }

    fn define_log_controls() -> Vec<(String, String)> {
        return vec![
            ("h".to_string(), "Previous Filter".to_string()),
            ("l".to_string(), "Next Filter".to_string()),
        ];
    }
}
