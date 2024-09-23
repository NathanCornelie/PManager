-- Your SQL goes here
create table substasks (
    id integer PRIMARY KEY AUTOINCREMENT,
    task_id integer,
    value TEXT,
    done BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (task_id) REFERENCES tasks (id)
)