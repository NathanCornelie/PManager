use std::path::Path;

use regex::Regex;
use rusqlite::Connection;

use crate::data::{models::Project, services};

pub fn create_project(name: &str, path: &str, desc: &str, db: &Connection) {
    let re_name = Regex::new(r"\w+").unwrap();
    let re_path: Regex = Regex::new(r"^(.*)\/([^\/]+)$").unwrap();
    // TODO: erro handling https://tauri.app/v1/guides/features/command/#error-handling
    if !re_name.is_match(name) {
        // panic!("Le nom de projet entrée comporte des caractère invalides, caractères accepté : a-zA-Z_")
    }
    if !re_path.is_match(path) {
        //  panic!("le chemin du projet est invalide")
    }
    if !Path::new(path).exists() {
        //  panic!("Le chemin du projets n'existe pas ")
    }
    let project = Project::create(path.to_string(), name.to_string(), desc.to_string());
    services::project::create_project(db, &project);
}

pub fn create_task(name: &str, project_id: &str, desc: &str, db: &Connection) {
    //TODO: verfications on project_id
    services::task::create_task(db, name, project_id, desc);
}
