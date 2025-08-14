use std::path::PathBuf;

pub struct FileInfo {
    pub file_name: String,
    pub file_path: PathBuf,
}

impl FileInfo {
    pub fn project_dir_name(&self) -> String {
        let dir_name = self
            .file_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|str| str.to_str())
            .unwrap();

        return dir_name.to_string();
    }
}
