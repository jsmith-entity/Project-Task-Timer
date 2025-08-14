mod file_watcher;
mod markdown_serialiser;

mod app;
mod components;
mod config;
mod events;
mod info_subtype;
mod log_type;
mod node;
mod traits;

use std::env;
use std::time::Duration;

use crossterm::event::KeyCode;

use crate::file_watcher::FileWatcher;
use crate::{app::App, config::KeyConfig, events::*, node::Node, traits::EventHandler};

fn main() {
    if let Some(file_name) = extract_file_name() {
        let res = FileWatcher::new(&file_name);
        if let Err(e) = res {
            println!("{}", e);
            return;
        }

        let mut file_watcher = res.unwrap();

        let file_info = file_watcher.info();
        let root_node = Node::convert_from(&file_watcher.read_file());
        let key_config = KeyConfig::default();
        let mut app = App::new(file_info, root_node, key_config);

        let mut terminal = ratatui::init();
        let poll_rate = Duration::from_millis(100);

        loop {
            terminal
                .draw(|frame| frame.render_widget(&app.window, frame.area()))
                .expect("failed to draw frame");

            if let Some(new_root) = file_watcher.poll_change() {
                app.update_tree(new_root);
            }

            app.update();

            match next_key_event(poll_rate) {
                Some(key_event) => {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }

                    app.handle_events(key_event.code);
                }
                None => {}
            }
        }

        ratatui::restore();
    } else {
        println!("Provide the file name as the first argument");
    }
}

fn extract_file_name() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return None;
    }

    return Some(args[1].clone());
}
