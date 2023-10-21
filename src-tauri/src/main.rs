// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use specta::{collect_types, Type};
use std::path::Path;
use tauri::api::{dir::read_dir, dir::DiskEntry, file::read_string};
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};
use tauri_specta::ts;
use tauri_text_editor::StateManager;
// use tauri_text_editor::Entry;

#[tauri::command]
// #[specta::specta]
fn get_current_dir_items() -> Vec<DiskEntry> {
    read_dir(".", true).unwrap()
}
#[tauri::command]
#[specta::specta]
fn get_file_lines(file: String) -> Vec<String> {
    let file_result = read_string(file);

    match file_result {
        Ok(file) => file.split("\n").map(|a| a.to_string()).collect(),
        Err(e) => {
            vec!["File Not Found".to_string(), e.to_string()]
        }
    }
}

use tauri_text_editor::backend_api::file_system::*;
fn main() {
    ts::export(
        collect_types![get_file_lines, open_file],
        "../src/bindings.ts",
    );
    let file_menu = Submenu::new(
        "File",
        Menu::with_items([
            MenuItem::CloseWindow.into(),
            CustomMenuItem::new("Reload", "Reload").into(),
            CustomMenuItem::new("Open", "Open").into(),
        ]),
    );

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file,
            get_file_lines,
            get_current_dir_items
        ])
        .manage(tauri_text_editor::StateManager::new())
        .menu(Menu::with_items([MenuEntry::Submenu(file_menu)]))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
