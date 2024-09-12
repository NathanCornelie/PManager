use crate::data::models::Task;
use rusqlite::Connection;

pub fn create_task(conn: &Connection, name: &str,project_id:&str, desc: &str) {
    conn.execute(
        "INSERT INTO tasks (name,project_id,description) VALUES (?1,?2,?3)",
        &[&name,&project_id, &desc],
    )
    .unwrap();
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

pub fn delete_task(conn: &Connection, id: &i32) {
    conn.execute("DELETE FROM `tasks` WHERE `id`= (?1) ", &[&id])
        .unwrap();
}
