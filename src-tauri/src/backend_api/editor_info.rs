use std::ops::Deref;

use crate::{
    error::{Error, MutexLockError},
    treesitter_backend::theme::{EditorTheme, LanguageTheme},
    Lang, StateManager,
};

use crate::treesitter_backend::query::SemanticLegend;

#[tauri::command]
#[specta::specta]
pub fn get_current_language_theme(
    state: tauri::State<StateManager>,
) -> Result<LanguageTheme, Error> {
    let editor_config = state
        .editor_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let theme_config = editor_config
        .theme
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let language_theme = theme_config
        .language
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    Ok(language_theme.clone())
}

#[tauri::command]
#[specta::specta]
pub fn get_editor_config(
    state: tauri::State<StateManager>,
) -> Result<(LanguageTheme, EditorTheme), Error> {
    let editor_config = state
        .editor_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let theme_config = editor_config
        .theme
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let language_theme = theme_config
        .language
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let editor_theme = theme_config
        .editor
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    Ok((language_theme.clone(), editor_theme.clone()))
}

#[tauri::command]
#[specta::specta]
pub fn get_tokens_legend(
    state: tauri::State<StateManager>,
    lang: Lang,
) -> Result<SemanticLegend, String> {
    let query_iter = &state.query_iter;

    Ok(query_iter.get_legend(&lang).deref().to_owned())
}

#[tauri::command]
#[specta::specta]
pub fn get_currently_supported_language(state: tauri::State<StateManager>) -> Vec<Lang> {
    let parser_helper = state.parser_helper.lock().unwrap();
    parser_helper.currently_supported_language()
}
