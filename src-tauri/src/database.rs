use app::data::{ models::Project, task::Task};
use rusqlite::{named_params, Connection};
use std::fs;
use tauri::AppHandle;

// ! database location : /home/nathan/.local/share/com.tauri.devprojects

const CURRENT_DB_VERSION: u32 = 3;

pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist. ");
    
    fs::create_dir_all(&app_dir).expect("The app data directory should be created");
    let sqlite_path = app_dir.join("PManager.sqlite");

    let mut db = Connection::open(sqlite_path)?;
    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    let _ = upgrade_database_if_needed(&mut db, existing_user_version);
    Ok(db)
}

pub fn upgrade_database_if_needed(
    db: &mut Connection,
    existing_version: u32,
) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "pragma_name", "WAL")?;
        let tx = db.transaction()?;
        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;
        tx.execute_batch(
            "

        create table tasks(
            id integer PRIMARY KEY AUTOINCREMENT,
            name VARCHAR NOT NULL,
            description TEXT,
            project_id INT,
            priority VARCHAR,
            due_date DATETIME,
            pinned BOOLEAN,
            FOREIGN KEY(project_id) references projects(id)
);
        ",
        )?;
        println!("Data base update to v {}", CURRENT_DB_VERSION);
        tx.commit()?;
    }
    Ok(())
}

pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
    let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
    statement.execute(named_params! {"@title": title})?;
    Ok(())
}

pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM items")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let title: String = row.get("title")?;
        items.push(title);
    }

    Ok(items)
}

pub fn get_projects(db: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM projects")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        let project = Project::new(
            row.get("id")?,
            row.get("path")?,
            row.get("name")?,
            row.get("description")?,
        );
        items.push(project)
    }
    Ok(items)
}
pub fn get_tasks(db: &Connection) -> Result<Vec<Task>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT * FROM tasks")?;
    let mut row = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = row.next()? {
        let task = Task::new(
            row.get("id")?,
            row.get("name")?,
            row.get("description")?,
            row.get("project_id")?,
        );
        items.push(task)
    }
    Ok(items)
}
