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

use anyhow::anyhow;
use crossterm::event::KeyCode;

use crate::file_watcher::FileWatcher;
use crate::{app::App, config::KeyConfig, events::*, node::Node, traits::EventHandler};

fn main() -> anyhow::Result<()> {
    let file_name = extract_file_name()?;
    let mut file_watcher = FileWatcher::new(&file_name)?;

    let file_info = file_watcher.info();
    let root_node = Node::convert_from(&file_watcher.read_file());
    let key_config = KeyConfig::default();
    let mut app = App::new(file_info, root_node, key_config);

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);
    let mut terminal = ratatui::init();

    loop {
        terminal
            .draw(|frame| frame.render_widget(&app.window, frame.area()))
            .expect("failed to draw frame");

        if let Some(new_root) = file_watcher.poll_change() {
            app.update_tree(new_root);
        }

        app.update();

        match events.next()? {
            InputEvent::Input(key) => {
                if key.code == KeyCode::Esc {
                    break;
                }

                app.handle_events(key.code);
            }
            InputEvent::Tick => (),
        }
    }

    ratatui::restore();

    Ok(())
}

fn extract_file_name() -> anyhow::Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Provide the file name as the first argument"));
    }

    return Ok(args[1].clone());
}
