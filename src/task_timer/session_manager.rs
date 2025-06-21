use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
use std::time::Duration;

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::window::Window;

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
        self.window.set_title(
            self.file_watcher
                .as_ref()
                .expect("should have file watcher when setting title")
                .get_title()
                .to_string(),
        );

        let initial_contents = self.file_watcher.as_ref().unwrap().read_file();
        self.update_contents(initial_contents);

        Ok(())
    }

    pub fn run(&mut self) {
        if let None = self.file_watcher {
            println!("File watcher has not been set.");
            return;
        }

        let mut terminal = ratatui::init();

        loop {
            if let Some(buf) = self.file_watcher.as_mut().unwrap().poll_change() {
                self.update_contents(buf);
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

    fn update_contents(&mut self, contents: String) {
        self.window.update_contents(contents.clone());
    }

    fn draw(&self, frame: &mut Frame) {
        self.window.render(frame);
    }
}
