use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
use std::env;
use std::fs;

fn main() {
    let contents = list_contents();
    if let Err(e) = contents {
        println!("{}", e);
        return;
    }

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn list_contents() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Provide one argument pointing to the file to read.".to_string());
    }

    let file_path: &str = &args[1];
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e.to_string()),
    }
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
