use crate::StateManager;

#[tauri::command]
#[specta::specta]
pub fn get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Result<Option<String>, String> {
    let file_manager = state.file_manager.lock().unwrap();
    let source_code = file_manager._get_file(&id).source_code;
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
pub fn save_file(state: tauri::State<StateManager>, id: u32, changes: String) {
    let file_manager = state.file_manager.lock().unwrap();

    file_manager.save_file(&id, changes);
}
