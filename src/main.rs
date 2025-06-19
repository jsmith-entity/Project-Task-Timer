mod task_timer;

use std::env;
use std::fs;

use task_timer::session_manager::SessionManager;

fn main() {
    let res = list_contents();
    if let Ok(contents) = res {
        let s_manager = SessionManager::new(contents);
        s_manager.run();
    } else if let Err(e) = res {
        println!("{}", e);
        return;
    }
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
