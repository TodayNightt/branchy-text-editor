use crate::StateManager;
use crate::backend_api::file_system::{_close_file, _get_file_system_info, _open_file};
use crate::backend_api::responses::{FileSystemInfo, OpenFile};
use crate::error::Response;

#[tauri::command]
#[specta::specta]
pub fn get_file_system_info(dir: Option<String>) -> Response<FileSystemInfo> {
    _get_file_system_info(dir).into()
}

#[tauri::command]
#[specta::specta]
pub fn open_file(state: tauri::State<StateManager>, path: String) -> Response<OpenFile> {
    _open_file(state, path).into()
}

#[tauri::command]
#[specta::specta]
pub fn close_file(state: tauri::State<StateManager>, id: u32) -> Response<()> {
    _close_file(state, id).into()
}
