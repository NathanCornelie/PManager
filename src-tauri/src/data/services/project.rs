use crate::data::models::Project;
use crate::data::project::ProjectCreate;
use rusqlite::Connection;

pub fn get_projects(conn: &Connection) -> Result<Vec<Project>, rusqlite::Error> {
    let mut data = conn.prepare("SELECT * FROM projects")?;
    let product_iter = data.query_map([], |row| {
        Ok(Project::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ))
    })?;
    let mut products = Vec::new();
    for product in product_iter {
        products.push(product?);
    }
    Ok(products)
}

pub fn create_project(conn: &Connection, product: &ProjectCreate) {
    conn.execute(
        "INSERT INTO projects (path,name,description) VALUES (?1,?2,?3)",
        &[&product.path, &product.name, &product.description],
    )
    .unwrap();
}


pub fn delete_project(conn: &Connection, id: &i32) {
    conn.execute("DELETE FROM `projects` WHERE `id`= (?1) ", &[&id])
        .unwrap();
}

pub fn get_project_by_id(conn: &Connection, id: &i32) {
    conn.execute("SELCT * FROM `projects` WHERE `id`= (?1) ", &[&id])
        .unwrap();
}
