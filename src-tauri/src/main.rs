#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_handler;
mod log_parser;

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![
        log_parser::parse_xml,
        log_parser::begin_search
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
