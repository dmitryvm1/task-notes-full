// @generated automatically by Diesel CLI.

diesel::table! {
    app_user (id) {
        id -> Int4,
        email -> Varchar,
    }
}

diesel::table! {
    project (id) {
        id -> Int4,
        owner_id -> Int4,
        title -> Varchar,
        priority -> Int4,
    }
}

diesel::table! {
    task (id) {
        id -> Int4,
        project_id -> Int4,
        task_list_id -> Nullable<Int4>,
        title -> Varchar,
        completed -> Bool,
    }
}

diesel::table! {
    task_list (id) {
        id -> Int4,
        title -> Varchar,
        project_id -> Int4,
    }
}

diesel::joinable!(task -> project (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user,
    project,
    task,
    task_list,
);
