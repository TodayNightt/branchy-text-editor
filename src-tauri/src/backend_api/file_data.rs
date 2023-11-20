use crate::{
    treesitter_backend::{highlighter::MonacoHighlights, parser::ChangesRange},
    StateManager,
};

#[tauri::command]
#[specta::specta]
pub fn get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Result<Option<String>, String> {
    let file_manager = state.file_manager.lock().unwrap();
    let mut parser_helper = state.parser_helper.lock().unwrap();
    let source_code = file_manager.read_source_code_in_bytes(&id).unwrap();
    let file_mutex = file_manager._get_file(&id)?;
    if file_manager.get_file_language(&id).is_some() {
        parser_helper.append_tree(&id, file_mutex.clone());
        parser_helper.parse(
            &id,
            &file_manager.get_file_language(&id).unwrap(),
            &source_code,
        );
    }

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
    let mut parser_helper = state.parser_helper.lock().unwrap();
    let source_code_in_bytes = source_code.as_bytes().to_vec();
    file_manager.update_source_code_for_file(&id, &source_code_in_bytes);
    let file_language = file_manager.get_file_language(&id);
    if file_language.is_some() {
        parser_helper.update_tree(&id, range);
        parser_helper.parse(&id, &file_language.unwrap(), &source_code_in_bytes);
    }

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
    if !file_manager.file_exists(&id) {
        return Ok(vec![]);
    }
    let parser_helper = state.parser_helper.lock().unwrap();
    let source_code_in_bytes = ranged_source_code.into_bytes();
    let query_iter = &state.query_iter;
    let file_language = file_manager.get_file_language(&id);
    if file_language.is_some() {
        let tokens = query_iter.iter_query(
            &parser_helper.get_tree(&id),
            &file_language.clone().unwrap(),
            &source_code_in_bytes,
        );

        let mut token_data = query_iter.sort_layer(tokens, &file_language.unwrap());

        let highlights = token_data.analyse_layer();
        return Ok(MonacoHighlights::emit(&highlights));
    }
    Err("The file language is not supported".to_string())
}
