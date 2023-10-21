use crate::StateManager;
use std::path::Path;
#[tauri::command]
pub fn open_file(state: tauri::State<StateManager>, path: String) -> Result<u64, String> {
    let path_path = Path::new(&path);
    let mut file_manager = state.file_manager.lock().unwrap();
    let file_result = file_manager.load_file(path_path);
    if let Ok(file_id) = file_result {
        Ok(file_id)
    } else {
        Err("Cannot open file".to_string())
    }
}
