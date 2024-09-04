
CREATE TABLE tasks (
    id integer PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    description TEXT,
    project_id INT,
    priority VARCHAR,
    due_date DATETIME,
    pinned BOOLEAN
)

