// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use tauri::Manager;
use tauri_specta::ts;
use tauri_text_editor::{
    backend_api::{editor_info::*, file_data::*, file_system::*},
    StateManager,
};

use std::path::PathBuf;

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![
            get_file_system_info,
            open_file,
            get_source_code_if_any,
            close_file,
            save_file,
            handle_file_changes,
            reset,
            get_current_language_theme,
            get_editor_config,
            get_tokens_legend,
            set_highlights,
            get_currently_supported_language
        ],
        "../src/backendApi/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file,
            get_file_system_info,
            get_source_code_if_any,
            close_file,
            save_file,
            handle_file_changes,
            reset,
            get_current_language_theme,
            get_editor_config,
            get_tokens_legend,
            set_highlights,
            get_currently_supported_language
        ])
        .setup(|app| {
            app.manage(StateManager::new());
            let app_handle = app.handle();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
