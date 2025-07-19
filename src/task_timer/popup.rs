use ratatui::{
    layout::Flex,
    prelude::{Alignment, Buffer, Constraint, Layout, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

#[derive(Clone)]
pub struct Popup {
    pub message: String,
}

impl Popup {
    pub fn new(message: String) -> Self {
        return Self { message };
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        return area;
    }

    fn render_options(&self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Min, Percentage};

        let options = vec![Line::from("Y"), Line::from("N")];
        let options_area = Popup::center(area, Percentage(50), Min(0));

        let option_width = 3;
        let spacing = 1;
        let total_options = options.len();
        let total_width = total_options * option_width + (total_options - 1) * spacing;

        let option_area_center_x = options_area.x + options_area.width / 2;
        let starting_x = option_area_center_x - (total_width / 2) as u16;

        let mut current_x = starting_x;
        for option in options {
            let rect = Rect::new(
                current_x,
                options_area.y,
                option_width as u16,
                options_area.height,
            );

            Line::from(option)
                .bg(Color::Gray)
                .fg(Color::Black)
                .alignment(Alignment::Center)
                .render(rect, buf);

            current_x += option_width as u16 + spacing as u16;
        }
    }
}

impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Min, Percentage};

        let prompt_area = Popup::center(area, Percentage(20), Percentage(15));

        Clear.render(prompt_area, buf);

        let block = Block::bordered();
        let inner_area = block.inner(prompt_area);
        block.render(prompt_area, buf);

        let [message_area, options_area] =
            Layout::vertical([Percentage(60), Percentage(40)]).areas(inner_area);

        let centered_message_area = Popup::center(message_area, Percentage(50), Min(0));
        Paragraph::new(self.message.clone())
            .alignment(Alignment::Center)
            .render(centered_message_area, buf);

        self.render_options(options_area, buf)
    }
}
