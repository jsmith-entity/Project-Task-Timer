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

    pub fn load(&mut self) {
        assert!(self.file_watcher.is_some());

        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                self.window.log("Save failed. Could not create saves directory");
                return;
            }
        }

        let path = self.file_watcher.as_ref().unwrap();
        let dir_name = path
            .file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|str| str.to_str())
            .unwrap();

        let save_dir = format!("{}/{}", saves, dir_name);
        if !fs::exists(&save_dir).unwrap() {
            if let Err(_) = fs::create_dir(&save_dir) {
                self.window.log("Save failed. Could not create save directory");
                return;
            }
        }

        let save_file = format!("{save_dir}/save.json");

        if let Ok(save_contents) = fs::read_to_string(save_file) {
            let deserialised: Window = serde_json::from_str(&save_contents).unwrap();
            self.window = deserialised;
            self.window.log("Retrieved save file.");
            self.current_line = self.window.task_list.selected_line;
        } else {
            self.window.log("Could not retrieve save file");
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

            if self.last_save_tick.elapsed().as_secs() >= 3600 {
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
            _ => self.window.handle_events(key_event.code),
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

        let path = self.file_watcher.as_ref().unwrap();
        let dir_name = path
            .file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|str| str.to_str())
            .unwrap();

        let save_dir = format!("{}/{}", saves, dir_name);
        if !fs::exists(&save_dir).unwrap() {
            if let Err(_) = fs::create_dir(&save_dir) {
                return Err("Save failed. Could not create save directory");
            }
        }

        let save_file = format!("{save_dir}/save.json");

        let serialised = serde_json::to_string_pretty(&self.window).unwrap();
        fs::write(save_file, serialised).expect("erm");

        return Ok(());
    }
}
