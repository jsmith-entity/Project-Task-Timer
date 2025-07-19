use ratatui::{
    prelude::{Buffer, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Default)]
struct ControlSection {
    pub title: String,
    pub control_list: Vec<(String, String)>,
}

impl Widget for &ControlSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const LINE_LENGTH: u16 = 20;

        Line::from(self.title.clone()).render(area, buf);

        let mut x: u16 = area.x;
        let mut y: u16 = area.y + 2;
        for (title, desc) in self.control_list.iter() {
            if x + LINE_LENGTH > area.x + area.width {
                x = area.x;
                y += 1;
            }

            let line_area = Rect {
                x,
                y,
                width: LINE_LENGTH,
                height: area.height,
            };

            let mut shortcut: String = title.clone();
            shortcut.push(' ');

            Line::from(vec![
                Span::styled(shortcut, Style::default().bg(Color::Gray).fg(Color::Black)),
                Span::raw(" "),
                Span::styled(desc, Style::default()),
            ])
            .render(line_area, buf);

            x += LINE_LENGTH;
        }
    }
}

#[derive(Default)]
pub struct Controls {
    main_controls: ControlSection,
    log_controls: ControlSection,
}

impl Controls {
    pub fn new() -> Self {
        let main_controls = ControlSection {
            title: "Main".to_string(),
            control_list: Controls::define_main_controls(),
        };

        let log_controls = ControlSection {
            title: "Log".to_string(),
            control_list: Controls::define_log_controls(),
        };

        return Self {
            main_controls,
            log_controls,
        };
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

impl Widget for &Controls {
    fn render(self, area: Rect, buf: &mut Buffer) {
        assert!(self.main_controls.control_list.len() > 0);
        assert!(self.log_controls.control_list.len() > 0);

        use Constraint::{Min, Percentage};

        let vertical = Layout::vertical([Percentage(20), Percentage(20), Min(0)]);
        let [main_area, log_area, _] = vertical.areas(area);

        self.main_controls.render(main_area, buf);
        self.log_controls.render(log_area, buf);
    }
}
