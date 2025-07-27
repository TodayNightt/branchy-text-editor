use crate::StateManager;
use crate::app_config::EditorTheme;
use crate::backend_api::editor_info::{
    _get_current_language_theme, _get_editor_config, _get_tokens_legend,
};
use crate::error::Response;
use crate::language::Lang;
use crate::treesitter_backend::query::SemanticLegend;
use crate::treesitter_backend::theme::LanguageTheme;

#[tauri::command]
#[specta::specta]
pub fn get_current_language_theme(state: tauri::State<StateManager>) -> Response<LanguageTheme> {
    _get_current_language_theme(state).into()
}

#[tauri::command]
#[specta::specta]
pub fn get_editor_config(
    state: tauri::State<StateManager>,
) -> Response<(LanguageTheme, EditorTheme)> {
    _get_editor_config(state).into()
}

#[tauri::command]
#[specta::specta]
pub fn get_tokens_legend(
    state: tauri::State<StateManager>,
    lang: Lang,
) -> Response<SemanticLegend> {
    _get_tokens_legend(state, lang).into()
}

#[tauri::command]
#[specta::specta]
pub fn get_currently_supported_language(state: tauri::State<StateManager>) -> Vec<Lang> {
    let parser_helper = state.parser_helper.lock().unwrap();
    parser_helper.currently_supported_language()
}
