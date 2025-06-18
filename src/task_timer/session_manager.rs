use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

pub struct SessionManager {
    exit_flag: bool,
}

impl SessionManager {
    pub fn new() -> SessionManager {
        SessionManager { exit_flag: false }
    }

    pub fn run(&self) {
        let mut terminal = ratatui::init();
        loop {
            terminal
                .draw(SessionManager::draw)
                .expect("failed to draw frame");

            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
        ratatui::restore();
    }

    fn draw(frame: &mut Frame) {
        let text = Text::raw("Hello World!");
        frame.render_widget(text, frame.area());
    }
}
