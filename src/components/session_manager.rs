use crossterm::event::{self, Event, KeyCode};
use std::fs;
use std::time::{Duration, Instant};

use crate::file_watcher::file_watcher::FileWatcher;
use crate::markdown_serialiser::*;

use crate::{info_subtype::InfoSubType, log_type::LogType, node::Node, traits::EventHandler};

use super::Window;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum SessionState {
    #[default]
    Running,
    AwaitingPrompt,
    Quitting,
}

pub struct SessionManager {
    file_watcher: Option<FileWatcher>,
    root_node: Node,
    window: Window,

    last_update_tick: Instant,
    last_save_tick: Instant,

    session_state: SessionState,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            file_watcher: None,
            root_node: Node::new(),
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
            self.window.load(deserialised);
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
        self.root_node = markdown_tree.clone();
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
        self.window
            .log("Launched project", LogType::INFO(InfoSubType::General));

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
                self.root_node = new_content_tree.clone();
                self.window.update_tree(new_content_tree);
            }

            if self.last_update_tick.elapsed().as_secs() >= 1 {
                self.window.update_time();
                self.last_update_tick = Instant::now();
            }

            if self.last_save_tick.elapsed().as_secs() >= 60 {
                match self.save() {
                    Ok(()) => self.window.log(
                        &InfoSubType::Save.message(InfoSubType::Save),
                        LogType::INFO(InfoSubType::Save),
                    ),
                    Err(e) => self.window.log(&e, LogType::ERROR),
                }

                self.last_save_tick = Instant::now();
            }

            self.window.update();

            terminal
                .draw(|frame| frame.render_widget(&self.window, frame.area()))
                .expect("failed to draw frame");

            if event::poll(Duration::from_millis(50)).unwrap() {
                let event = event::read().unwrap();

                let Event::Key(key_event) = event else {
                    continue;
                };

                self.session_state = self.handle_events(key_event.code);
            }
        }
        ratatui::restore();

        self.window
            .log("Closed project", LogType::INFO(InfoSubType::General));
    }

    fn save(&mut self) -> Result<(), String> {
        assert!(self.file_watcher.is_some());

        self.root_node = self.window.extract_node();

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

        let file_name = self.file_watcher.as_ref().unwrap().file_name.clone();
        markdown_serialiser::export(self.root_node.clone(), file_name);

        return Ok(());
    }
}

impl EventHandler for SessionManager {
    fn handle_events(&mut self, key_code: KeyCode) -> SessionState {
        return self.window.handle_events(key_code);
    }
}
