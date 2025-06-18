mod task_timer;

use std::env;
use std::fs;

use task_timer::session_manager::SessionManager;

fn main() {
    let contents = list_contents();
    if let Err(e) = contents {
        println!("{}", e);
        return;
    }

    let s_manager = SessionManager::new();
    s_manager.run();
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
