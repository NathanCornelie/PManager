// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{
    command,
    data::{models::Project, task::Task},
};
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, InvokeError, Manager, State};
mod database;
mod state;

#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    app_handle.db(|db| database::add_item(name, db)).unwrap();

    let items = app_handle.db(|db| database::get_all(db)).unwrap();

    let items_string = items.join(" | ");
    print!("Your name log: {}", items_string);
    format!("Your name log: {}", items_string)
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn get_projects(app_handle: AppHandle) -> Result<Vec<Project>, InvokeError> {
    let items = app_handle.db(|db| database::get_projects(db).unwrap());
    println!("projects returned {}", items.len());
    Ok(items)
}
#[tauri::command]
fn get_tasks(app_handle: AppHandle) -> Result<Vec<Task>, InvokeError> {
    let items = app_handle.db(|db| database::get_tasks(db).unwrap());
    println!("tasks returned {} ", items.len());
    Ok(items)
}

#[tauri::command]
fn create_project(
    app_handle: AppHandle,
    name: &str,
    path: &str,
    description: &str,
) -> Result<(), InvokeError> {
    app_handle.db(|db| {
        command::project::create_project(&name, &path, &description, db);
    });
    println!("project created {:?}", [name, path, description]);
    Ok(())
}
#[tauri::command]
async fn create_task(
    app_handle: AppHandle,
    name: &str,
    project_id: &str,
    description: &str,
) -> Result<(), InvokeError> {
    println!("called");
    app_handle.db(|db| {
        let _ = command::project::create_task(&name, &project_id, &description, db);
    });
    println!("task created {:?}", [name, project_id, description]);
    Ok(())
}
#[tauri::command]
async fn delete_task_cmd(app_handle: AppHandle, task_id: &str) -> Result<(), InvokeError> {
    println!("called");
    app_handle.db(|db| {
        let _ = app::data::services::task::delete_task(db, &task_id.parse::<i32>().unwrap());
    });
    println!("task  {} deleted", task_id);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_projects,
            create_project,
            create_task,
            delete_task_cmd,
            get_tasks
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db =
                database::initialize_database(&handle).expect("Database initialize should succeed");

            *app_state.db.lock().unwrap() = Some(db);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// https://github.com/RandomEngy/tauri-sqlite
