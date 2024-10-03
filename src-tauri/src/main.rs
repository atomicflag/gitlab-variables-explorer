// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use config::Config;
use tracing::Level;
use tracing_subscriber::fmt;

#[tauri::command]
fn read_config() -> Config {
    config::read_config()
}

#[tauri::command]
fn write_config(config: Config) {
    config::write_config(&config);
}

fn main() {
    fmt().with_max_level(Level::INFO).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![read_config, write_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
