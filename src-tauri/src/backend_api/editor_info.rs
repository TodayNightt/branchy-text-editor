use tree_sitter::Query;

use crate::{
    treesitter_backend::{
        get_query_from_each_language, get_tree_sitter_language,
        highlighter::{EditorTheme, LanguageTheme},
    },
    Lang, StateManager,
};

use super::responses::SemanticLegend;

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

