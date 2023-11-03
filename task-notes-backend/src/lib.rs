extern crate diesel;
extern crate dotenv;


use model::models::{NewTask, Task};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_task<'a>(conn: &mut PgConnection, title: &'a str) -> Task {
    use model::schema::task;

    let new_task = NewTask {
        title: title.to_owned(),
        project_id: 0,
        task_list_id: None,
    };

    diesel::insert_into(task::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error creating task")
}
