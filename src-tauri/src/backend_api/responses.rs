use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{files_api::DirectoryItem, language::Lang};

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
    pub fn create(id: u32, same_name_exist: bool, file: (String, Option<Lang>, PathBuf)) -> Self {
        Self {
            id,
            same_name_exist,
            name: file.0.to_owned(),
            language: file.1.to_owned(),
            path: file.2.to_str().unwrap().to_string(),
        }
    }
}
