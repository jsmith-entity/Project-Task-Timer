use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::fs;
use std::time::{Duration, Instant};

use crate::file_watcher::file_watcher::FileWatcher;
use crate::task_timer::{node::Node, views::log::log_type::*, window::Window};

#[derive(Default, PartialEq, Clone)]
pub enum SessionState {
    #[default]
    Running,
    AwaitingPrompt,
    Quitting,
}

pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    window: Window,

    last_update_tick: Instant,
    last_save_tick: Instant,

    session_state: SessionState,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            file_watcher: None,
            window: Window::new(),

            last_update_tick: Instant::now(),
            last_save_tick: Instant::now(),

            session_state: SessionState::default(),
        }
    }

    pub fn load(&mut self) {
        assert!(self.file_watcher.is_some());

        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                self.window
                    .log("Save failed. Could not create saves directory", LogType::ERROR);
                return;
            }
        }

        let dir_name = self.project_dir_name();

        let save_dir = format!("{}/{}", saves, dir_name);
        if !fs::exists(&save_dir).unwrap() {
            if let Err(_) = fs::create_dir(&save_dir) {
                self.window
                    .log("Save failed. Could not create save directory", LogType::ERROR);
                return;
            }
        }

        let save_file = format!("{save_dir}/save.json");

        if let Ok(save_contents) = fs::read_to_string(save_file) {
            let deserialised: Window = serde_json::from_str(&save_contents).unwrap();
            self.window = deserialised;
            self.window
                .log("Retrieved save file.", LogType::INFO(InfoSubType::General));
        } else {
            self.window.log("Could not retrieve save file", LogType::ERROR);
        }
    }

    fn project_dir_name(&self) -> String {
        let path = self.file_watcher.as_ref().unwrap();
        let dir_name = path
            .file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|str| str.to_str())
            .unwrap();

        return dir_name.to_string();
    }

    pub fn attach_file_watcher(&mut self, file_name: &str) -> Result<(), notify::Error> {
        let watcher = FileWatcher::new(file_name)?;
        self.file_watcher = Some(watcher);

        let initial_contents = self.file_watcher.as_ref().unwrap().read_file();
        let markdown_tree = Node::convert_from(&initial_contents);
        self.window.update_tree(markdown_tree);

        Ok(())
    }

    pub fn run(&mut self) {
        if let None = self.file_watcher {
            println!("File watcher has not been set.");
            return;
        }

        let mut terminal = ratatui::init();

        self.window.title = self.project_dir_name();

        loop {
            if self.session_state == SessionState::Quitting {
                if let Err(e) = self.save() {
                    // TODO: print save success after terminal has quit
                    println!("{e}");
                }
                break;
            }

            if let Some(buf) = self.file_watcher.as_mut().unwrap().poll_change() {
                let new_content_tree = Node::convert_from(&buf);
                self.window.update_tree(new_content_tree);
            }

            if self.last_update_tick.elapsed().as_secs() >= 1 {
                //self.window.update_time();
                self.last_update_tick = Instant::now();
            }

            if self.last_save_tick.elapsed().as_secs() >= 3600 {
                use InfoSubType::*;
                use LogType::*;

                match self.save() {
                    Ok(()) => self.window.log("Saved", INFO(General)),
                    Err(e) => self.window.log(&e, ERROR),
                }

                self.last_save_tick = Instant::now();
            }

            terminal
                .draw(|frame| frame.render_widget(&self.window, frame.area()))
                .expect("failed to draw frame");

            if event::poll(Duration::from_millis(50)).unwrap() {
                let event = event::read().unwrap();

                self.session_state = self.handle_events(&event);
            }
        }
        ratatui::restore();
    }

    fn handle_events(&mut self, event: &Event) -> SessionState {
        let Event::Key(key_event) = event else {
            return self.session_state.clone();
        };

        let mut new_state: SessionState = match self.session_state {
            SessionState::Running => {
                self.window.handle_events(key_event.code);
                SessionState::Running
            }
            SessionState::AwaitingPrompt => self.handle_prompt_event(&key_event),
            SessionState::Quitting => SessionState::Quitting,
        };

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.window.enable_popup("Exit Project?");
                new_state = SessionState::AwaitingPrompt;
            }
            _ => (),
        }

        if self.session_state == SessionState::AwaitingPrompt && new_state == SessionState::Running {
            self.window.disable_popup();
        }

        return new_state;
    }

    // TODO: change to accomodate for any prompt, not just quit prompt - would include introducing
    // new states
    fn handle_prompt_event(&mut self, key_event: &KeyEvent) -> SessionState {
        return match key_event.code {
            KeyCode::Char('y') | KeyCode::Esc => SessionState::Quitting,
            KeyCode::Char('n') => SessionState::Running,
            _ => SessionState::AwaitingPrompt,
        };
    }

    fn save(&mut self) -> Result<(), String> {
        assert!(self.file_watcher.is_some());

        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                return Err("Save failed. Could not create saves directory".to_string());
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
                return Err("Save failed. Could not create save directory".to_string());
            }
        }

        let save_file = format!("{save_dir}/save.json");

        let serialised = serde_json::to_string_pretty(&self.window).unwrap();
        fs::write(save_file, serialised).expect("erm");

        return Ok(());
    }
}
