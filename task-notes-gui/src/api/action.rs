use crate::api::Update;
use std::sync::mpsc::Sender;
use model::models::{PatchProject, NewProject, NewTask};

#[derive(Clone)]
pub struct Action {
    pub sender: Option<Sender<Update>>,
    pub server_url: std::sync::Arc<str>
}

impl Action {
    pub fn get_projects(&self) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::get_projects(&server).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn get_tasks(&self, project_id: i32) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::get_tasks(&server, project_id as usize).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn create_task(&self, title: &str, project_id: i32) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        let task = NewTask {
            title: title.to_string(),
            task_list_id: None,
            project_id
        };
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::create_task(&server, &task).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn create_project(&self, title: &str) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        let project = NewProject {
            title: title.to_string(),
            owner_id: 1
        };
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::create_project(&server, &project).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn delete_project(&self, project_id: i32) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::delete_project(&server, project_id).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn edit_project(&self, project: PatchProject) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::edit_project(&server, &project).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }

    pub fn delete_task(&self, task_id: i32) {
        let s = self.sender.as_ref().unwrap().clone();
        let server = self.server_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::delete_task(&server, task_id).await {
                Some(u) => {
                    s.send(u).unwrap();
                }
                _ => {}
            }
        });
    }
}