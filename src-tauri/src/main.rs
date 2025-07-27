// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use branchy_text_editor::{
    backend_api::{editor_info::*, file_data::*, file_system::*},
    StateManager,
};
use tauri_specta::{collect_commands, Builder};
use tauri::Manager;
use specta_typescript::Typescript;

fn main() {
    let mut builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
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
            get_currently_supported_language]
        );

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            // This is also required if you want to use events
            builder.mount_events(app);
            let app_handle = app.handle();
            let config_dir = app_handle.path_resolver().app_local_data_dir();
            let state_manager = StateManager::new(config_dir)?;
            app_handle.manage(state_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}