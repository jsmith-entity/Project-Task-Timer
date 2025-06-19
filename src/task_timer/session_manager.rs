use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
use std::time::Duration;

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::window::Window;

// TODO: update content if file has been read
// TODO: Window has render? function loop through each in draw
pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    window: Window,
    contents: String,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            file_watcher: None,
            window: Window::new(),
            contents: "".to_string(),
        }
    }

    pub fn attach_file_watcher(&mut self, file_name: &str) -> Result<(), notify::Error> {
        let watcher = FileWatcher::new(file_name)?;
        self.file_watcher = Some(watcher);
        Ok(())
    }

    pub fn run(&mut self) {
        if let None = self.file_watcher {
            println!("File watcher has not been set.");
            return;
        }

        let mut terminal = ratatui::init();
        self.init();
        loop {
            if let Some(buf) = self.file_watcher.as_mut().unwrap().poll_change() {
                self.contents = buf;
            }

            terminal
                .draw(|frame| self.draw(frame))
                .expect("failed to draw frame");

            if event::poll(Duration::from_millis(50)).unwrap() {
                if let Event::Key(_) = event::read().unwrap() {
                    break;
                }
            }
        }
        ratatui::restore();
    }

    fn init(&self) {}

    fn draw(&self, frame: &mut Frame) {
        let text = Text::raw(&self.contents);
        frame.render_widget(text, frame.area());
    }
}
