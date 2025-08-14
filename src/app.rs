use crossterm::event::{self, Event, KeyCode};
use std::fs;
use std::time::{Duration, Instant};

use crate::file_watcher::file_info::FileInfo;
use crate::markdown_serialiser::*;

use crate::{
    components::Window, config::KeyConfig, info_subtype::InfoSubType, log_type::LogType, node::Node,
    traits::EventHandler,
};

#[derive(Default, PartialEq, Clone, Debug)]
pub enum SessionState {
    #[default]
    Running,
    AwaitingPrompt,
    Quitting,
}

pub struct App {
    pub window: Window,
    file_info: FileInfo,
    root_node: Node,

    last_update_tick: Instant,
    last_save_tick: Instant,

    session_state: SessionState,

    key_config: KeyConfig,
}

impl App {
    pub fn new(file_info: FileInfo, root_node: Node, key_config: KeyConfig) -> Self {
        let window_title = file_info.project_dir_name();
        let mut app = App {
            window: Window::new(&window_title),
            file_info,
            root_node: root_node.clone(),

            last_update_tick: Instant::now(),
            last_save_tick: Instant::now(),

            session_state: SessionState::default(),
            key_config: key_config,
        };

        app.window.update_tree(root_node);

        return app;
    }

    pub fn load(&mut self) {
        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                self.window
                    .log("Save failed. Could not create saves directory", LogType::ERROR);
                return;
            }
        }

        // TODO: erm.
        let dir_name = self.window.title.clone();

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

    pub fn update_tree(&mut self, new_root: Node) {
        self.root_node = new_root.clone();
        self.window.update_tree(new_root);
    }

    pub fn update(&mut self) {
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
    }

    fn save(&mut self) -> Result<(), String> {
        self.root_node = self.window.extract_node();

        let home_dir = std::env::home_dir().unwrap().to_string_lossy().to_string();

        let saves = format!("{}/.project-saves", home_dir);
        if !fs::exists(&saves).unwrap() {
            if let Err(_) = fs::create_dir(&saves) {
                return Err("Save failed. Could not create saves directory".to_string());
            }
        }

        // FIX: store result as file info member var
        let dir_name = self.file_info.project_dir_name();

        let save_dir = format!("{}/{}", saves, dir_name);
        if !fs::exists(&save_dir).unwrap() {
            if let Err(_) = fs::create_dir(&save_dir) {
                return Err("Save failed. Could not create save directory".to_string());
            }
        }

        let save_file = format!("{save_dir}/save.json");

        let serialised = serde_json::to_string_pretty(&self.window).unwrap();
        fs::write(save_file, serialised).expect("erm");

        markdown_serialiser::export(self.root_node.clone(), self.file_info.file_name.clone());

        return Ok(());
    }
}

impl EventHandler for App {
    fn handle_events(&mut self, key_code: KeyCode) -> SessionState {
        return self.window.handle_events(key_code);
    }
}
