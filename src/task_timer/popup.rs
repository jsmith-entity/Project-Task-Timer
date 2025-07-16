use ratatui::{
    Frame,
    layout::Flex,
    prelude::{Alignment, Constraint, Layout, Rect, Stylize},
    style::Color,
    text::Line,
    widgets::{Block, Clear, Paragraph},
};

#[derive(Clone)]
pub struct Popup {
    pub message: String,
}

impl Popup {
    pub fn new(message: String) -> Self {
        return Self { message };
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        use Constraint::{Min, Percentage};

        let prompt_area = Popup::center(area, Percentage(20), Percentage(15));
        frame.render_widget(Clear, prompt_area);

        let block = Block::bordered();
        let inner_area = block.inner(prompt_area);

        let vertical = Layout::vertical([Percentage(60), Percentage(40)]).split(inner_area);

        let content = Paragraph::new(self.message.clone()).alignment(Alignment::Center);
        let content_area = Popup::center(vertical[0], Percentage(50), Min(0));
        frame.render_widget(block, prompt_area);
        frame.render_widget(content, content_area);

        self.render_options(frame, vertical[1]);
    }

    fn render_options(&self, frame: &mut Frame, area: Rect) {
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

            let styled_option = option
                .bg(Color::Gray)
                .fg(Color::Black)
                .alignment(Alignment::Center);
            frame.render_widget(styled_option, rect);

            current_x += option_width as u16 + spacing as u16;
        }
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        return area;
    }
}
