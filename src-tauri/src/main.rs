// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu};
use tauri_specta::ts;
use tauri_text_editor::{backend_api::file_system::*, StateManager};

fn main() {
    #[cfg(debug_assertions)]
    ts::export(
        collect_types![get_file_system_info, open_file, get_file_info],
        "../src/bindings.ts",
    )
    .unwrap();

    let file_menu = Submenu::new(
        "File",
        Menu::with_items([
            MenuItem::CloseWindow.into(),
            CustomMenuItem::new("Reload", "Reload").into(),
            CustomMenuItem::new("Open", "Open").into(),
        ]),
    );

    tauri::Builder::default()
        .menu(Menu::with_items([MenuEntry::Submenu(file_menu)]))
        .invoke_handler(tauri::generate_handler![
            open_file,
            get_file_info,
            get_file_system_info
        ])
        .setup(|app| {
            app.manage(StateManager::new());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
