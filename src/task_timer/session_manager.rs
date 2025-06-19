use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

// TODO: hold md content
// TODO: array of Windows
// TODO: Window has render? function loop through each in draw
pub struct SessionManager {
    content: String,
}

impl SessionManager {
    pub fn new(raw_text: String) -> SessionManager {
        SessionManager { content: raw_text }
    }

    pub fn run(&self) {
        let mut terminal = ratatui::init();
        self.init();
        loop {
            terminal
                .draw(|frame| self.draw(frame))
                .expect("failed to draw frame");

            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
        ratatui::restore();
    }

    fn init(&self) {}

    fn draw(&self, frame: &mut Frame) {
        let text = Text::raw(&self.content);
        frame.render_widget(text, frame.area());
    }
}
