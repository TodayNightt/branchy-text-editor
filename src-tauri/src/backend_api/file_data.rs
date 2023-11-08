use crate::{treesitter_backend::parser::ChangesRange, StateManager};

#[tauri::command]
#[specta::specta]
pub fn get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Result<Option<String>, String> {
    let file_manager = state.file_manager.lock().unwrap();
    let source_code = file_manager.read_source_code_in_bytes(&id).unwrap();
    let file_mutex = file_manager._get_file(&id)?;
    let mut file = file_mutex.lock().unwrap();
    file.update_source_code(&source_code);
    match source_code.len() {
        s if s > 0 => Ok(Some(String::from_utf8(source_code).unwrap())),
        _ => Ok(None),
    }
}

#[tauri::command]
#[specta::specta]
pub fn reset(state: tauri::State<StateManager>) {
    let mut file_manager = state.file_manager.lock().unwrap();
    file_manager.files.as_mut().clear();
}

#[tauri::command]
#[specta::specta]
pub fn save_file(state: tauri::State<StateManager>, id: u32) -> Result<(), String> {
    let file_manager = state.file_manager.lock().unwrap();
    file_manager
        .save_file(&id)
        .map_err(|_err| "File cannot be saved".to_string())
}

#[tauri::command]
#[specta::specta]
pub fn handle_file_changes(
    state: tauri::State<StateManager>,
    id: u32,
    source_code: String,
    range: Option<ChangesRange>,
) -> Result<(), String> {
    let file_manager = state.file_manager.lock().unwrap();
    let file_mutex = file_manager
        ._get_file(&id)
        .map_err(|err| format!("{} handleChange", err))?;
    let mut file = file_mutex.lock().unwrap();
    file.update_source_code(source_code.as_bytes().to_vec().as_ref());
    file.update_tree(range);
    file.parse();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_highlights(
    state: tauri::State<StateManager>,
    id: u32,
    ranged_source_code: String,
) -> Result<Vec<u32>, String> {
    let file_manager = state.file_manager.lock().unwrap();

    file_manager
        ._get_file(&id)?
        .lock()
        .unwrap()
        .highlight(ranged_source_code.into_bytes())
        .map_err(|_err| "Error getting a query".to_string())
}
