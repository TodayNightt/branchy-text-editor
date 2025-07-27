use std::ops::Deref;

use crate::error::Result;
use crate::treesitter_backend::query::SemanticLegend;
use crate::{
    StateManager, app_config::EditorTheme, error::MutexLockError, language::Lang,
    treesitter_backend::theme::LanguageTheme,
};

pub fn _get_current_language_theme(state: tauri::State<StateManager>) -> Result<LanguageTheme> {
    let editor_config = state
        .editor_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let theme_config = editor_config.theme();
    let theme_config = theme_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let language_theme = theme_config.language_theme();
    let language_theme = language_theme
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    Ok(language_theme.clone())
}

pub fn _get_editor_config(
    state: tauri::State<StateManager>,
) -> Result<(LanguageTheme, EditorTheme)> {
    let editor_config = state
        .editor_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let theme_config = editor_config.theme();
    let theme_config = theme_config
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    let language_theme = theme_config.language_theme();
    let language_theme = language_theme
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;
    let editor_theme = theme_config.editor_theme();
    let editor_theme = editor_theme
        .try_lock()
        .map_err(|err| MutexLockError(err.to_string()))?;

    Ok((language_theme.clone(), editor_theme.clone()))
}

pub fn _get_tokens_legend(state: tauri::State<StateManager>, lang: Lang) -> Result<SemanticLegend> {
    let query_iter = &state.query_iter;

    Ok(query_iter.get_legend(&lang)?.deref().to_owned())
}
