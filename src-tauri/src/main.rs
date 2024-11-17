// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use config::Config;
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[tauri::command]
fn read_config() -> Result<Config, String> {
    config::read_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn write_config(config: Config) -> Result<(), String> {
    config::write_config(&config).map_err(|e| e.to_string())
}

fn main() {
    fmt().with_max_level(Level::INFO).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![read_config, write_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
