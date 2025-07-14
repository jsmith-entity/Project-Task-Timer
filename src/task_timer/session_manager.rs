use crossterm::event::{self, Event, KeyCode};
use std::fs;
use std::time::{Duration, Instant};

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::node::Node;
use crate::task_timer::window::Window;

pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    window: Window,

    current_line: u16,
    last_update_tick: Instant,
    last_save_tick: Instant,
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

            current_line: 1,
            last_update_tick: Instant::now(),
            last_save_tick: Instant::now(),
        }
    }

    pub fn attach_file_watcher(&mut self, file_name: &str) -> Result<(), notify::Error> {
        let watcher = FileWatcher::new(file_name)?;
        self.file_watcher = Some(watcher);

        let initial_contents = self.file_watcher.as_ref().unwrap().read_file();
        let markdown_tree = Node::convert_from(&initial_contents);
        self.window.content_tree = markdown_tree;
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
                self.window.content_tree = new_content_tree;
            }

            if self.last_update_tick.elapsed().as_secs() >= 1 {
                self.window.update_time();
                self.last_update_tick = Instant::now();
            }

            if self.last_save_tick.elapsed().as_secs() >= 1 {
                let message = match self.save() {
                    Ok(()) => "Saved".to_string(),
                    Err(e) => e.to_string(),
                };

                self.window.log(&message);
                self.last_save_tick = Instant::now();
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

    fn handle_input(&mut self, event: &Event) -> InputResult {
        let Event::Key(key_event) = event else {
            return InputResult::Continue;
        };

        match key_event.code {
            KeyCode::Char('j') => {
                if self.current_line < self.window.content_height {
                    self.current_line += 1;
                    self.window.select_line(self.current_line);
                }
            }
            KeyCode::Char('k') => {
                if self.current_line > 1 {
                    // TODO: something looks wrong
                    self.current_line -= 1;
                    self.window.select_line(self.current_line)
                }
            }
            KeyCode::Char('s') => {
                self.window.timers.try_activate();
            }
            KeyCode::Char(' ') => {
                self.window.update_completed_task();
            }
            KeyCode::Char('o') => {
                self.window.toggle_headings(true);
                self.current_line = 1;
            }
            KeyCode::Char('c') => {
                self.window.toggle_headings(false);
                self.current_line = 1;
            }
            KeyCode::Enter => {
                self.window.task_list.try_collapse();
            }
            KeyCode::Esc | KeyCode::Char('q') => return InputResult::Exit,
            _ => (),
        }

        return InputResult::Continue;
    }

    fn save(&mut self) -> Result<(), &str> {
        assert!(self.file_watcher.is_some());

        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                return Err("Save failed. Could not create saves directory");
            }
        }

        if let Some(dir_name) = self.file_watcher.as_ref().unwrap().file_path.file_name() {
            let save_dir_name: String = dir_name.to_string_lossy().to_string();

            let save_dir = format!("{}/{}", saves, save_dir_name);
            if !fs::exists(&save_dir).unwrap() {
                if let Err(_) = fs::create_dir(&save_dir) {
                    return Err("Save failed. Could not create save directory");
                }
                // rewriting save
            } else {
                // new save
            }
        }

        return Ok(());
    }
}
