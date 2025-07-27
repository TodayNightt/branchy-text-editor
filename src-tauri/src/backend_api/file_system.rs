use crate::{
    StateManager,
    error::{MutexLockError, PathError, Result},
    files_api::get_directory_items,
};
use home::home_dir;
use path_absolutize::Absolutize;
use std::path::PathBuf;

use super::responses::FileSystemInfo;
use super::responses::OpenFile;

pub fn _get_file_system_info(dir: Option<String>) -> Result<FileSystemInfo> {
    if let Some(dir) = dir {
        let mut path_buf = PathBuf::from(dir);
        if !path_buf.is_absolute() {
            path_buf = path_buf.absolutize().unwrap().to_path_buf();
        }
        let directory_items = get_directory_items(&path_buf, 4)?;
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

pub fn _open_file(state: tauri::State<StateManager>, path: String) -> Result<OpenFile> {
    let mut file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let file_info = file_manager.load_file(path)?;
    let same_name_exist = file_info.1;
    let id = file_info.0;
    let file_info = file_manager.get_file_info(&id)?;
    Ok(OpenFile::create(id, same_name_exist, file_info))
}

pub fn _close_file(state: tauri::State<StateManager>, id: u32) -> Result<()> {
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
