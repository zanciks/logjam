#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// technically this won't run right now. We need to move patterns.xml, index.html, and script.js to a plugin 
// and then implement plugin loading. It is expected to be broken right now!!

use tauri::{generate_handler, generate_context};
mod log_parser;

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![
        log_parser::parse_xml,
        log_parser::begin_search
    ])
    .run(generate_context!())
    .expect("error while running tauri application");
}
