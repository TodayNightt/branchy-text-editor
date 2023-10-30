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
}

impl OpenFile {
    pub fn create(id: u32, file: &OpenedFile) -> Self {
        Self {
            id,
            name: file.name.to_owned(),
            language: file.language.to_owned(),
        }
    }
}
