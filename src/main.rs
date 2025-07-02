mod file_watcher;
mod task_timer;

use std::env;

use task_timer::session_manager::SessionManager;

fn main() {
    if let Some(file_name) = extract_file_name() {
        let mut s_manager = SessionManager::new();

        let res = s_manager.attach_file_watcher(&file_name);
        if let Ok(_) = res {
            s_manager.run();
        } else if let Err(e) = res {
            println!("{}", e);
        }
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
