#[cfg(not(target_arch = "wasm32"))]
use super::schema::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use diesel::PgConnection;
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=project))]
#[cfg_attr(target_arch = "wasm32", derive(Debug, Serialize, Deserialize, Clone))]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Serialize, Deserialize, Clone, Identifiable, Queryable))]
pub struct Project {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub priority: i32,
}


#[cfg(not(target_arch = "wasm32"))]
impl Project {
    pub fn by_id(conn: &mut PgConnection, project_id: i32) -> Option<Project> {
        use crate::schema::project::dsl::*;
        let res: Result<Vec<Project>, _> = project.filter(id.eq(project_id)).load(conn);
        match res {
            Ok(p) => {
                if p.len() > 0 {
                    Some(p[0].clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Identifiable, Associations, Queryable))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(belongs_to(Project)))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=task_list))]
pub struct TaskList {
    pub id: i32,
    pub title: String,
    pub project_id: i32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Identifiable, Queryable, Insertable))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=app_user))]
pub struct AppUser {
    pub id: i32,
    pub email: String,
}


#[cfg(not(target_arch = "wasm32"))]
impl AppUser {
    pub fn by_email(conn: &mut PgConnection, user_email: &String) -> Option<AppUser> {
        use crate::schema::app_user::dsl::*;
        let res: Result<Vec<AppUser>, _> = app_user.filter(email.eq(user_email)).load(conn);
        match res {
            Ok(p) => {
                if p.len() > 0 {
                    Some(p[0].clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=project))]
pub struct NewProject {
    pub title: String,
    //  pub id: i32,
    pub owner_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(AsChangeset, Identifiable))]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=project))]
pub struct PatchProject {
    pub title: String,
    pub id: i32,
    //pub owner_id: i32,
    //pub priority: i32
}

impl PatchProject {
    pub fn patch(&self, target: &mut Project) {
        target.title = self.title.clone();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "app_user"]
pub struct NewAppUser {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Insertable))]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(rename_all(serialize = "camelCase"))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=task))]
pub struct NewTask {
    pub title: String,
    pub project_id: i32,
    pub task_list_id: Option<i32>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Identifiable, Associations, Queryable))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(table_name=task))]
#[cfg_attr(not(target_arch = "wasm32"), diesel(belongs_to(Project)))]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub task_list_id: Option<i32>,
    pub title: String,
    pub completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Task {
    pub fn by_id(conn: &mut PgConnection, task_id: i32) -> Option<Task> {
        use crate::schema::task::dsl::*;
        let res: Result<Vec<Task>, _> = task.filter(id.eq(task_id)).load(conn);
        match res {
            Ok(t) => {
                if t.len() > 0 {
                    Some(t[0].clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
