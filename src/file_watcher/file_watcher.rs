use notify::event::ModifyKind;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};

pub struct FileWatcher {
    file_name: String,
    file_path: PathBuf,
    _watcher: RecommendedWatcher,
    recv: Receiver<notify::Result<Event>>,
}

impl FileWatcher {
    pub fn new(path: &str) -> notify::Result<Self> {
        let (file_path, watch_path) = FileWatcher::path_parent_dir(path).ok_or_else(|| {
            notify::Error::generic(format!("Could not find parent directory for file '{}'", path).as_str())
        })?;

        let (send, recv) = channel();
        let mut watcher = RecommendedWatcher::new(send, notify::Config::default())?;
        watcher.watch(std::path::Path::new(watch_path), RecursiveMode::NonRecursive)?;

        Ok(Self {
            file_name: path.to_string(),
            file_path,
            _watcher: watcher,
            recv,
        })
    }

    pub fn poll_change(&mut self) -> Option<String> {
        match self.recv.try_recv() {
            Ok(Ok(event)) => {
                if self.filter_notify_event(event) {
                    Some(self.read_file())
                } else {
                    None
                }
            }
            Ok(Err(_e)) => None,
            Err(_) => None,
        }
    }

    pub fn read_file(&self) -> String {
        return fs::read_to_string(self.file_path.clone())
            .expect(format!("Failed to read file {}", &self.file_name).as_str());
    }

    fn path_parent_dir(path_str: &str) -> Option<(PathBuf, &Path)> {
        let path: &Path = Path::new(path_str);
        if !path.exists() {
            return None;
        }

        let file_path = path.canonicalize().ok()?;
        let parent_path: &Path = path.parent()?;

        return Some((file_path, parent_path));
    }

    fn filter_notify_event(&self, ev: Event) -> bool {
        // FIX: swap order. check file first, then modification type
        if let EventKind::Modify(ModifyKind::Data(_)) = ev.kind {
            if ev.paths.iter().any(|path| path == &self.file_path) {
                return true;
            }
        }

        return false;
    }
}
