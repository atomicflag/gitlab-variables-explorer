// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod variables;

use config::Config;
use gitlab::Project;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Builder, Manager, State};
use tracing::{info, Level};
use tracing_subscriber::fmt;
use variables::BoundVariable;

#[tauri::command]
fn read_config(state: State<'_, Mutex<AppState>>) -> Result<Config, String> {
    let mut state = state.lock().unwrap();
    match config::read_config() {
        Ok(config) => {state.config = config; Ok(state.config.clone())},
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
fn write_config(config: Config, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.config = config;
    config::write_config(&state.config).map_err(|e| e.to_string())
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProjectsAndVariables {
    pub projects: Vec<Project>,
    pub variables: Vec<BoundVariable>,
}

#[tauri::command]
async fn fetch_variables(state: State<'_, Mutex<AppState>>) -> Result<ProjectsAndVariables, String> {
    let config = state.lock().unwrap().config.clone();
    // TODO: save vars to the app state
    match variables::fetch_variables(&config).await {
        Ok((projects, variables)) => Ok(ProjectsAndVariables {
            projects,
            variables,
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Default)]
struct AppState {
    config: Config,
}

fn main() {
    fmt().with_max_level(Level::INFO).init();
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            read_config,
            write_config,
            fetch_variables
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
