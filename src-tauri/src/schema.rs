// @generated automatically by Diesel CLI.

diesel::table! {
    items (rowid) {
        rowid -> Integer,
        title -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Nullable<Integer>,
        path -> Nullable<Text>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    substasks (id) {
        id -> Nullable<Integer>,
        task_id -> Nullable<Integer>,
        value -> Nullable<Text>,
        done -> Nullable<Bool>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        project_id -> Nullable<Integer>,
        priority -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        pinned -> Nullable<Bool>,
        status -> Nullable<Text>,
    }
}

diesel::joinable!(substasks -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    projects,
    substasks,
    tasks,
);
