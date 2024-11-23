// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod variables;

use config::Config;
use gitlab::Project;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::fmt;
use variables::BoundVariable;

#[tauri::command]
fn read_config() -> Result<Config, String> {
    config::read_config().map_err(|e| e.to_string())
}

#[tauri::command]
fn write_config(config: Config) -> Result<(), String> {
    config::write_config(&config).map_err(|e| e.to_string())
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProjectsAndVariables {
    pub projects: Vec<Project>,
    pub variables: Vec<BoundVariable>,
}

#[tauri::command]
async fn fetch_variables(config: Config) -> Result<ProjectsAndVariables, String> {
    match variables::fetch_variables(&config).await {
        Ok((projects, variables)) => Ok(ProjectsAndVariables {
            projects,
            variables,
        }),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    fmt().with_max_level(Level::INFO).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            read_config,
            write_config,
            fetch_variables
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
