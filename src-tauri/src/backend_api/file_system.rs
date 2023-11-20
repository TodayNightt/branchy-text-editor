use crate::error::Error;
use crate::error::MutexLockError;
use crate::error::PathError;
use crate::get_directory_items;
use crate::StateManager;
use home::home_dir;
use path_absolutize::Absolutize;
use std::path::PathBuf;

use super::responses::FileSystemInfo;
use super::responses::OpenFile;

#[tauri::command]
#[specta::specta]
pub fn get_file_system_info(dir: Option<String>) -> Result<FileSystemInfo, Error> {
    if let Some(dir) = dir {
        let mut path_buf = PathBuf::from(dir);
        if !path_buf.is_absolute() {
            path_buf = path_buf.absolutize().unwrap().to_path_buf();
        }
        let directory_items = get_directory_items(&path_buf, 2)?;
        return Ok(FileSystemInfo::create(
            path_buf
                .into_os_string()
                .into_string()
                .map_err(|_err| PathError::ToStringError)?,
            directory_items,
        ));
    }
    let home_dir = home_dir().unwrap().absolutize().unwrap().to_path_buf();
    let directory_items = get_directory_items(&home_dir, 2)?;
    Ok(FileSystemInfo::create(
        home_dir
            .into_os_string()
            .into_string()
            .map_err(|_err| PathError::ToStringError)?,
        directory_items,
    ))
}

#[tauri::command]
#[specta::specta]
pub fn open_file(state: tauri::State<StateManager>, path: String) -> Result<OpenFile, Error> {
    let mut file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let file_info = file_manager.load_file(path)?;
    let binding = file_manager._get_file(&file_info.0)?;
    let file = binding
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    Ok(OpenFile::create(file_info, &file))
}

#[tauri::command]
#[specta::specta]
pub fn close_file(state: tauri::State<StateManager>, id: u32) -> Result<(), Error> {
    let mut file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let mut parser_helper = state
        .parser_helper
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    parser_helper.remove_tree(&id);
    file_manager.close_file(&id);
    Ok(())
}
