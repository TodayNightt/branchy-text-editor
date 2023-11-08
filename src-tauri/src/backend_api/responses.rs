use crate::{DirectoryItem, Lang, OpenedFile};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct FileSystemInfo {
    current_directory: String,
    directory_items: Vec<DirectoryItem>,
}

impl FileSystemInfo {
    pub fn create(current_directory: String, directory_items: Vec<DirectoryItem>) -> Self {
        Self {
            current_directory,
            directory_items,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct OpenFile {
    id: u32,
    name: String,
    language: Option<Lang>,
    same_name_exist: bool,
    path: String,
}

impl OpenFile {
    pub fn create(file_info: (u32, bool), file: &OpenedFile) -> Self {
        Self {
            id: file_info.0,
            same_name_exist: file_info.1,
            name: file.name.to_owned(),
            language: file.language.to_owned(),
            path: file.path.to_str().unwrap().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SemanticLegend {
    _token_types: Vec<String>,
    _token_modifier: Vec<String>,
}

impl SemanticLegend {
    pub fn create(name: Vec<String>) -> Self {
        Self {
            _token_types: name.to_owned(),
            _token_modifier: vec![],
        }
    }
}
