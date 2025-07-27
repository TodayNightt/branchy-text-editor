use crate::{
    StateManager,
    error::{Error, FileError, MutexLockError, Result},
    treesitter_backend::{highlighter::MonacoHighlights, parser::ChangesRange, query::RangePoint},
};

pub fn _get_source_code_if_any(
    state: tauri::State<StateManager>,
    id: u32,
) -> Result<String> {
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
    if file_language.is_some() && parser_helper.parser_exist(&file_language.clone().unwrap()) {
        parser_helper.append_tree(&id, file_language.clone())?;
        parser_helper.parse(&id, &file_language.unwrap(), &source_code)?;
    }

    match source_code.len() {
        s if s > 0 => Ok(
            String::from_utf8(source_code).map_err(|_err| FileError::InvalidUtf8StringError(id))?,
        ),
        _ => Ok("".to_string()),
    }
}

pub fn _reset(state: tauri::State<StateManager>) -> Result<()> {
    let mut file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    file_manager.clear();
    Ok(())
}
pub fn _save_file(state: tauri::State<StateManager>, id: u32) -> Result<()> {
    let file_manager = state
        .file_manager
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    file_manager.save_file(&id)
}

pub fn _handle_file_changes(
    state: tauri::State<StateManager>,
    id: u32,
    source_code: String,
    range: Option<ChangesRange>,
) -> Result<()> {
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
    if file_language.is_some() && parser_helper.parser_exist(&file_language.clone().unwrap()) {
        parser_helper.update_tree(&id, range)?;
        parser_helper.parse(&id, &file_language.unwrap(), &source_code_in_bytes)?;
    }

    Ok(())
}

pub fn _set_highlights(
    state: tauri::State<StateManager>,
    id: u32,
    ranged_source_code: String,
    range: RangePoint,
) -> Result<Vec<u32>> {
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
