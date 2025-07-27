use crate::StateManager;
use crate::backend_api::file_data::{
    _get_source_code_if_any, _handle_file_changes, _reset, _save_file, _set_highlights,
};
use crate::error::Response;
use crate::treesitter_backend::parser::ChangesRange;
use crate::treesitter_backend::query::RangePoint;

#[tauri::command]
#[specta::specta]
pub fn get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Response<String> {
    _get_source_code_if_any(state, id).into()
}

#[tauri::command]
#[specta::specta]
pub fn reset(state: tauri::State<StateManager>) -> Response<()> {
    _reset(state).into()
}

#[tauri::command]
#[specta::specta]
pub fn save_file(state: tauri::State<StateManager>, id: u32) -> Response<()> {
    _save_file(state, id).into()
}

#[tauri::command]
#[specta::specta]
pub fn handle_file_changes(
    state: tauri::State<StateManager>,
    id: u32,
    source_code: String,
    range: Option<ChangesRange>,
) -> Response<()> {
    _handle_file_changes(state, id, source_code, range).into()
}

#[tauri::command]
#[specta::specta]
pub fn set_highlights(
    state: tauri::State<StateManager>,
    id: u32,
    ranged_source_code: String,
    range: RangePoint,
) -> Response<Vec<u32>> {
    _set_highlights(state, id, ranged_source_code, range).into()
}
