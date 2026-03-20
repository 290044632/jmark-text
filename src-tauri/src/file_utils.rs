use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tauri::command;

#[derive(Serialize)]
pub struct FileTreeEntry {
    label: String,
    path: String,
    children: Vec<FileTreeEntry>,
    is_folder: bool,
}

#[command]
pub fn read_dir_recursive(path: String) -> Result<Vec<FileTreeEntry>, String> {
    fn read_dir_inner(dir: PathBuf) -> Result<Vec<FileTreeEntry>, String> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = path.is_dir();
            let children = if is_dir {
                Some(read_dir_inner(path.clone())?)
            } else {
                None
            };
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md"){
                entries.push(FileTreeEntry {
                    label: name,
                    path: path.to_string_lossy().to_string(),
                    children: children.unwrap_or_default(),
                    is_folder: is_dir,
                });
            }else {
                let _children = children.unwrap_or_default();
                if !_children.is_empty() {
                    entries.push(FileTreeEntry {
                        label: name,
                        path: path.to_string_lossy().to_string(),
                        children: _children,
                        is_folder: is_dir,
                    });
                }
            }
        }
        Ok(entries)
    }
    read_dir_inner(PathBuf::from(path))
}
