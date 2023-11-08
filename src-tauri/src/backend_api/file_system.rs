use crate::error;
use crate::get_directory_items;
use crate::StateManager;
use home::home_dir;
use path_absolutize::Absolutize;
use std::path::PathBuf;

use super::responses::FileSystemInfo;
use super::responses::OpenFile;

#[tauri::command]
#[specta::specta]
pub fn get_file_system_info(dir: Option<String>) -> FileSystemInfo {
    if let Some(dir) = dir {
        let mut path_buf = PathBuf::from(dir);
        if !path_buf.is_absolute() {
            path_buf = path_buf.absolutize().unwrap().to_path_buf();
        }
        let directory_items = get_directory_items(&path_buf, 2);
        return FileSystemInfo::create(
            path_buf.into_os_string().into_string().unwrap(),
            directory_items,
        );
    }
    let home_dir = home_dir().unwrap().absolutize().unwrap().to_path_buf();
    let directory_items = get_directory_items(&home_dir, 2);
    FileSystemInfo::create(
        home_dir.into_os_string().into_string().unwrap(),
        directory_items,
    )
}

// FIXME: This is a temporary Custom Error type
//        will need to implement more formal Error
use thiserror::Error;
#[derive(Error, Debug)]
pub enum CustomError {
    #[error("couldn't open the file")]
    LoadFileError(#[from] std::io::Error),
    #[error("{}", message)]
    GetFileError { message: String },
}

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
#[tauri::command]
#[specta::specta]
pub fn open_file(state: tauri::State<StateManager>, path: String) -> Result<OpenFile, CustomError> {
    let mut file_manager = state.file_manager.lock().unwrap();
    let file_info = file_manager.load_file(path)?;
    let binding = file_manager
        ._get_file(&file_info.0)
        .map_err(|err| CustomError::GetFileError { message: err })?;
    let file = binding
        .lock()
        .unwrap();
    Ok(OpenFile::create(file_info, &file))
}

#[tauri::command]
#[specta::specta]
pub fn close_file(state: tauri::State<StateManager>, id: u32) -> Result<(), String> {
    let mut file_maneger = state.file_manager.lock().unwrap();
    file_maneger.files.as_mut().remove(&id);
    Ok(())
}
