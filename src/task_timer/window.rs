use ratatui::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct Window {
    title: String,
    contents: String,
}

// TODO: Layout created per heading
impl Window {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            contents: "".to_string(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn title(&self) -> String {
        return self.title.clone();
    }

    pub fn update_contents(&mut self, contents: String) {
        self.contents = contents.clone();
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::default().title(self.title()).borders(Borders::ALL);
        let inner_area = block.inner(area);
        let body = Paragraph::new(self.contents.clone());

        frame.render_widget(block, area);
        frame.render_widget(body, inner_area);
    }
}
