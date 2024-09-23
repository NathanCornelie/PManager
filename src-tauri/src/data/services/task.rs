
use rusqlite::{params, params_from_iter, Connection, Error};

use crate::data::task::Task;

pub fn create_task(conn: &Connection, name: &str, project_id: &str, desc: &str) {
    if project_id.to_string() == String::from("0"){
        conn.execute(
            "INSERT INTO tasks (name,project_id,description) VALUES (?1,?2,?3)",
            params![&name, None::<i32>, &desc],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO tasks (name,project_id,description) VALUES (?1,?2,?3)",
            &[&name, &project_id, &desc],
        )
        .unwrap();
    }

    println!("Task {} created", { name });
}

// pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>,rusqlite::Error> {
//     let mut data =conn.prepare("SELECT * FROM tasks")?;
//     let task_iter = data.query_map([], |row|{
//         Ok(Task::new(
//             row.get(0)?,
//             row.get(1)?,
//             row.get(2)?))
//     })?;
//     let mut tasks = Vec::new();
//     for task in task_iter{

//         tasks.push(task?);
//     }
//     Ok(tasks)
// }
pub fn update_task(conn: &Connection, task: &Task) -> Result<(), Error> {
    let params = task.to_hash_map();

    let mut set_clause: Vec<String> = params
        .iter()
        .map(|(col, _)| {
            if params.get(col) != None {
                format!("{} = ?", col)
            } else {
                String::from("")
            }
        })
        .collect();
    set_clause.retain(|x| !x.is_empty());

    let set_clause_str = set_clause.join(", ");

    let mut values: Vec<&str> = params
        .iter()
        .map(|(_, val)| if !val.is_empty() { val.as_str() } else { "" })
        .collect();
    values.retain(|&x| !x.is_empty());

    let query = format!("UPDATE tasks SET {} WHERE id={}", set_clause_str, task.id);
    conn.execute(&query, params_from_iter(values))?;
    println!("Row updated successfully in table: {}", "tasks");
    Ok(())
}
pub fn delete_task(conn: &Connection, id: &i32) {
    conn.execute("DELETE FROM `tasks` WHERE `id`= (?1) ", &[&id])
        .unwrap();
}
pub fn delete_tasks(conn: &Connection) {
    conn.execute("DELETE FROM tasks", []).unwrap();
}
