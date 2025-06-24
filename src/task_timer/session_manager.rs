use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::node::Node;
use crate::task_timer::window::Window;

pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    window: Window,
}

#[derive(PartialEq)]
enum InputResult {
    Continue,
    Exit,
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

        let initial_contents = self.file_watcher.as_ref().unwrap().read_file();
        let markdown_tree = Node::convert_from(&initial_contents);
        self.window.update_contents(markdown_tree.clone());
        self.window.file_name = file_name.to_string();

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
                let new_content_tree = Node::convert_from(&buf);
                self.window.update_contents(new_content_tree.clone());
            }

            terminal
                .draw(|frame| self.window.render(frame))
                .expect("failed to draw frame");

            if event::poll(Duration::from_millis(50)).unwrap() {
                let event = event::read().unwrap();

                if self.handle_input(&event) == InputResult::Exit {
                    break;
                }
            }
        }
        ratatui::restore();
    }

    fn handle_input(&self, event: &Event) -> InputResult {
        let Event::Key(key_event) = event else {
            return InputResult::Continue;
        };

        if key_event.code == KeyCode::Esc {
            return InputResult::Exit;
        }

        return InputResult::Continue;
    }
}
