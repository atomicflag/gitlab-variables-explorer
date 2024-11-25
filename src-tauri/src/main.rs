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
async fn search_variables(
    state: State<'_, Mutex<AppState>>,
    text: String,
) -> Result<ProjectsAndVariables, String> {
    {
        let mut state = state.lock().unwrap();
        if state
            .last_update
            .is_some_and(|v| v.elapsed() <= Duration::from_secs(10))
        {
            // TODO: filter projects/variables
            return Ok(ProjectsAndVariables {
                projects: state.projects.clone(),
                variables: state.variables.clone(),
            });
        } else {
            state.last_update = Some(Instant::now());
        }
    }
    let config = state.lock().unwrap().config.clone();
    match variables::fetch_variables(&config).await {
        Ok((projects, variables)) => {
            let mut state = state.lock().unwrap();
            state.projects = projects;
            state.variables = variables;
            // Ideally we need an async lock/semaphore
            state.last_update = Some(Instant::now());
            // TODO: filter projects/variables
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

fn main() {
    fmt().with_max_level(Level::INFO).init();
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            read_config,
            write_config,
            search_variables
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
