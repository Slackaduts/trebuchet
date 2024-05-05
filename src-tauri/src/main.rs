// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(dead_code)]
#![forbid(unsafe_code)]

pub mod modules;
use crate::modules::news::{parse_pirate_news, parse_wizard_news};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            parse_wizard_news,
            parse_pirate_news
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
