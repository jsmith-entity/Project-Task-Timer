use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::window::Window;

// TODO: update content if file has been read
// TODO: Window has render? function loop through each in draw
pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    window: Window,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            file_watcher: None,
            window: Window::new(),
        }
    }

    pub fn attach_file_watcher(&mut self, file_name: &str) -> Result<(), notify::Error> {
        let watcher = FileWatcher::new(file_name)?;
        self.file_watcher = Some(watcher);
        Ok(())
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
        let text = Text::raw("erm");
        frame.render_widget(text, frame.area());
    }
}
