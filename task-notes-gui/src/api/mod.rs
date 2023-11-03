use model::models::{Task, Project, NewProject, NewTask, PatchProject};
use serde::{Serialize, Deserialize};

pub mod action;
pub mod common;


#[derive(Serialize, Deserialize)]
pub enum Update {
    ProjectList(Vec<Project>),
    TaskList(Vec<Task>),
    ProjectCreated(Project),
    ProjectDeleted(Project),
    TaskDeleted(Task),
    ProjectChanged(PatchProject),
    TaskCreated(Task)
}

pub async fn get_projects(server_url: &str) -> Option<Update> {
    let js_value = common::get_json(format!("{}api/project?ownerId=1", server_url)).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::ProjectList(v))
}

pub async fn edit_project(server_url: &str, changes: &PatchProject) -> Option<Update> {
    let data = serde_json::to_string(changes).unwrap();
    let data = serde_wasm_bindgen::to_value(&data).unwrap();
    let js_value = common::patch_json(format!("{}api/project", server_url), &data).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::ProjectChanged(v))
}


pub async fn delete_project(server_url: &str, project_id: i32) -> Option<Update> {
    let js_value = common::delete_json(format!("{}api/project?id={}", server_url, project_id)).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::ProjectDeleted(v))
}

pub async fn delete_task(server_url: &str, task_id: i32) -> Option<Update> {
    let js_value = common::delete_json(format!("{}api/task?id={}", server_url, task_id)).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::TaskDeleted(v))
}

pub async fn create_project(server_url: &str, project: &NewProject) -> Option<Update> {
    let data = serde_json::to_string(project).unwrap();
    let data = serde_wasm_bindgen::to_value(&data).unwrap();
    let js_value = common::post_json(format!("{}api/project", server_url), &data).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::ProjectCreated(v))
}

pub async fn create_task(server_url: &str, task: &NewTask) -> Option<Update> {
    let data = serde_json::to_string(task).unwrap();
    let data = serde_wasm_bindgen::to_value(&data).unwrap();
    let js_value = common::post_json(format!("{}api/task", server_url), &data).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::TaskCreated(v))
}

pub async fn get_tasks(server_url: &str, project_id: usize) -> Option<Update> {
    let js_value = common::get_json(format!("{}api/task?projectId={}", server_url, project_id)).await.ok()?;
    let v = serde_wasm_bindgen::from_value(js_value).ok()?;
    Some(Update::TaskList(v))
}