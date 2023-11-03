use model::models;
use model::models::NewTask;
use model::models::{NewProject, PatchProject};
use model::schema::project;
use model::schema::task;
use actix_identity::Identity;
use actix_identity::error::GetIdentityError;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::*;
use std::collections::HashMap;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// This handler uses json extractor with limit
pub async fn create_task(
    task_item: web::Json<NewTask>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    if !check_project_owner(conn, &req_identity, task_item.project_id) {
        return HttpResponse::BadRequest().finish();
    }
    let res: models::Task = diesel::insert_into(task::table)
        .values(&task_item.into_inner())
        .get_result(conn)
        .unwrap();
    HttpResponse::Ok().json(res) // <- send json response
}

pub async fn create_project(
    mut project_item: web::Json<NewProject>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    println!("{:?}", project_item);
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    project_item.owner_id = match user_id_from_identity(conn, &req_identity) {
        Ok(oid) => oid,
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };
    let res: models::Project = diesel::insert_into(project::table)
        .values(&project_item.into_inner())
        .get_result(conn)
        .unwrap();
    HttpResponse::Ok().json(res) // <- send json response
}

pub async fn get_tasks(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    use model::schema::app_user::dsl::*;
    use model::schema::project::dsl::*;
    use model::schema::task::dsl::*;
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    let oid: i32 = match user_id_from_identity(conn, &req_identity) {
        Ok(oid) => oid,
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };
    let pid: i32 = query.get("projectId").unwrap().parse().unwrap();

    let tasks: Vec<(models::Task, models::Project)> = task
        .inner_join(model::schema::project::table)
        .filter(owner_id.eq(oid).and(project_id.eq(pid)))
        .load(conn)
        .unwrap();
    //let mut tasks = task.filter(project_id.eq(pid)).load::<models::Task>(conn).unwrap();
    HttpResponse::Ok().json(
        tasks
            .iter()
            .map(|item| item.0.clone())
            .collect::<Vec<models::Task>>(),
    ) // <- send json response
}

pub async fn get_projects(
    _query: web::Query<HashMap<String, String>>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    use model::schema::project::dsl::*;
    let conn: &mut PgConnection = &mut pool.get().unwrap();

    let oid: i32 = match user_id_from_identity(conn, &req_identity) {
        Ok(oid) => oid,
        _ => {
            return HttpResponse::BadRequest().finish();
        }
    };
    let projects = project
        .filter(owner_id.eq(oid))
        .load::<models::Project>(conn)
        .unwrap();
    println!("{:?}", projects);
    HttpResponse::Ok().json(projects)
}

pub fn user_id_from_identity(
    conn: &mut PgConnection,
    req_identity: &Option<Identity>,
) -> Result<i32, GetIdentityError> {
    #[cfg(debug_assertions)]
    return Ok(1);
    #[cfg(not(debug_assertions))]
    if let Some(i) = req_identity {
        let user_email = i.id()?;
        let user = models::AppUser::by_email(conn, &user_email).unwrap();
        Ok(user.id)
    }
}

pub async fn delete_task(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    use model::schema::task::dsl::*;
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    let tid: i32 = query.get("id").unwrap().parse().unwrap();
    if !check_task_owner(conn, &req_identity, tid) {
        return HttpResponse::BadRequest().finish();
    }
    let res: models::Task = diesel::delete(task.filter(id.eq(tid)))
        .get_result(conn)
        .unwrap();
    HttpResponse::Ok().json(res) // <- send json response
}

pub fn check_task_owner(conn: &mut PgConnection, req_identity: &Option<Identity>, tid: i32) -> bool {
    let _oid = match user_id_from_identity(conn, req_identity) {
        Ok(id) => id,
        _ => {
            return false;
        }
    };
    match models::Task::by_id(conn, tid) {
        Some(t) => {
            check_project_owner(conn, req_identity, t.project_id)
        }
        _ => {
            false
        }
    }
}

pub fn check_project_owner(conn: &mut PgConnection, req_identity: &Option<Identity>, pid: i32) -> bool {
    let oid = match user_id_from_identity(conn, req_identity) {
        Ok(id) => id,
        _ => {
            return false;
        }
    };
    match models::Project::by_id(conn, pid) {
        Some(p) => {
            if p.owner_id != oid {
                return false;
            }
        }
        _ => {
            return false;
        }
    };
    true
}

pub async fn delete_project(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    use model::schema::project::dsl::*;
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    let pid: i32 = query.get("id").unwrap().parse().unwrap();
    if !check_project_owner(conn, &req_identity, pid) {
        return HttpResponse::BadRequest().finish();
    }
    let res: models::Project = diesel::delete(project.filter(id.eq(pid)))
        .get_result(conn)
        .unwrap();
    HttpResponse::Ok().json(res) // <- send json response
}

pub async fn update_project(
    p: web::Json<PatchProject>,
    pool: web::Data<Pool>,
    req_identity: Option<Identity>,
) -> HttpResponse {
    let conn: &mut PgConnection = &mut pool.get().unwrap();
    if !check_project_owner(conn, &req_identity, p.id) {
        return HttpResponse::BadRequest().finish();
    }
    let _res = update(&p.clone()).set(p.clone()).execute(conn).unwrap();
    HttpResponse::Ok().json(p.into_inner()) // <- send json response
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::Found().header("location", "/").finish()
}
