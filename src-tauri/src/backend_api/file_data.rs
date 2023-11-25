use crate::{
    error::{Error, FileError, MutexLockError},
    treesitter_backend::{highlighter::MonacoHighlights, parser::ChangesRange, query::RangePoint},
    StateManager,
};

#[tauri::command]
#[specta::specta]
pub fn get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Result<Option<String>, Error> {
    let file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let mut parser_helper = state
        .parser_helper
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let source_code = file_manager.read_source_code_in_bytes(&id)?;
    let file_language = file_manager.get_file_language(&id)?;
    if file_language.is_some() {
        if parser_helper.parser_exist(&file_language.clone().unwrap()) {
            parser_helper.append_tree(&id, file_language.clone())?;
            parser_helper.parse(&id, &file_language.unwrap(), &source_code)?;
        }
    }

    match source_code.len() {
        s if s > 0 => Ok(Some(
            String::from_utf8(source_code).map_err(|_err| FileError::InvalidUtf8StringError(id))?,
        )),
        _ => Ok(None),
    }
}

#[tauri::command]
#[specta::specta]
pub fn reset(state: tauri::State<StateManager>) -> Result<(), Error> {
    let mut file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    file_manager.clear();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn save_file(state: tauri::State<StateManager>, id: u32) -> Result<(), Error> {
    let file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    Ok(file_manager.save_file(&id)?)
}

#[tauri::command]
#[specta::specta]
pub fn handle_file_changes(
    state: tauri::State<StateManager>,
    id: u32,
    source_code: String,
    range: Option<ChangesRange>,
) -> Result<(), Error> {
    let file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let mut parser_helper = state
        .parser_helper
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let source_code_in_bytes = source_code.as_bytes().to_vec();
    file_manager.update_source_code_for_file(&id, &source_code_in_bytes)?;
    let file_language = file_manager.get_file_language(&id)?;
    if file_language.is_some() {
        if parser_helper.parser_exist(&file_language.clone().unwrap()) {
            parser_helper.update_tree(&id, range)?;
            parser_helper.parse(&id, &file_language.unwrap(), &source_code_in_bytes)?;
        }
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn set_highlights(
    state: tauri::State<StateManager>,
    id: u32,
    ranged_source_code: String,
    range: RangePoint,
) -> Result<Vec<u32>, Error> {
    let file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    if !file_manager.file_exists(&id) {
        return Ok(vec![]);
    }
    let parser_helper = state
        .parser_helper
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let source_code_in_bytes = ranged_source_code.as_bytes().to_vec();
    let query_iter = &state.query_iter;
    let file_language = file_manager.get_file_language(&id)?;

    match file_language {
        Some(language) => {
            let tokens = query_iter.iter_query_with_range(
                &parser_helper.get_tree(&id)?,
                &language.clone(),
                &source_code_in_bytes,
                range,
            )?;

            let mut token_data = query_iter.sort_layer(tokens, &language)?;

            let highlights = token_data.analyse_layer();
            Ok(MonacoHighlights::emit(&highlights))
        }
        None => {
            let extension = file_manager.get_file_extension(&id)?;
            Err(Error::FileError(FileError::LanguageNotSupportError(
                extension,
            )))
        }
    }
}
