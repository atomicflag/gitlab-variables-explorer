pub mod app;
mod navbar;
mod settings;
mod variables;

pub use app::*;
use gitlab::{
    types::{Project, ProjectVariable},
    ProjectId,
};
use leptos::prelude::*;
use navbar::*;
use serde::{Deserialize, Serialize};
use settings::*;
use variables::*;
use wasm_bindgen::prelude::*;
use reactive_stores::Store;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct Config {
    host: String,
    token: String,
}

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Variables,
    Settings,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BoundVariable {
    pub project_id: ProjectId,
    pub variable: ProjectVariable,
}

#[derive(Clone, Serialize, Deserialize, Debug, Store, Default)]
pub struct ProjectsAndVariables {
    #[store(key: ProjectId = |row| row.id.clone())]
    pub projects: Vec<Project>,
    pub variables: Vec<BoundVariable>,
    pub active_project: Option<ProjectId>,
    pub is_refreshing: bool
}
