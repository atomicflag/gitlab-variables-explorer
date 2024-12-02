// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod variables;

use config::Config;
use gitlab::Project;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{Builder, Manager, State};
use tracing::{info, Level};
use tracing_subscriber::fmt;
use variables::BoundVariable;

#[tauri::command]
fn read_config(state: State<'_, Mutex<AppState>>) -> Result<Config, String> {
    let mut state = state.lock().unwrap();
    match config::read_config() {
        Ok(config) => {
            state.config = config;
            Ok(state.config.clone())
        }
        Err(e) => Err(e.to_string()),
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
async fn fetch_variables(
    state: State<'_, Mutex<AppState>>,
) -> Result<ProjectsAndVariables, String> {
    let _ = API_LOCK.lock().await;
    {
        let state = state.lock().unwrap();
        if state
            .last_update
            .is_some_and(|v| v.elapsed() <= Duration::from_secs(10))
        {
            return Ok(ProjectsAndVariables {
                projects: state.projects.clone(),
                variables: state.variables.clone(),
            });
        }
    }
    let config = state.lock().unwrap().config.clone();
    match variables::fetch_variables(&config).await {
        Ok((projects, variables)) => {
            let mut state = state.lock().unwrap();
            state.projects = projects;
            state.variables = variables;
            state.last_update = Some(Instant::now());
            Ok(ProjectsAndVariables {
                projects: state.projects.clone(),
                variables: state.variables.clone(),
            })
        }
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Default)]
struct AppState {
    config: Config,
    projects: Vec<Project>,
    variables: Vec<BoundVariable>,
    last_update: Option<Instant>,
}

static API_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

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
            let mut state = AppState::default();
            state.config = config::read_config().unwrap_or_default();
            app.manage(Mutex::new(state));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
