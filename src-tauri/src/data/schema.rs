// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Nullable<Integer>,
        path -> Nullable<Text>,
        name -> Text,
        description -> Nullable<Text>,
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
    }
}

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
);
