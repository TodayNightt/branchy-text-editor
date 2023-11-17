use std::ops::Deref;

use crate::{
    treesitter_backend::theme::{EditorTheme, LanguageTheme},
    Lang, StateManager,
};

use crate::treesitter_backend::query::SemanticLegend;

#[tauri::command]
#[specta::specta]
pub fn get_current_language_theme(state: tauri::State<StateManager>) -> LanguageTheme {
    let editor_config = state.editor_config.lock().unwrap();

    let theme_config = editor_config.theme.lock().unwrap();

    let language_theme = theme_config.language.lock().unwrap();

    language_theme.clone()
}

#[tauri::command]
#[specta::specta]
pub fn get_editor_config(state: tauri::State<StateManager>) -> (LanguageTheme, EditorTheme) {
    let editor_config = state.editor_config.lock().unwrap();
    let theme_config = editor_config.theme.lock().unwrap();

    let language_theme = theme_config.language.lock().unwrap();
    let editor_theme = theme_config.editor.lock().unwrap();

    (language_theme.clone(), editor_theme.clone())
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
