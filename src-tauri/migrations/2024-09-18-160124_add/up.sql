CREATE TABLE
    tasks_new (
        id integer PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        description TEXT,
        project_id INT,
        priority VARCHAR,
        due_date DATETIME,
        pinned BOOLEAN,
        status VARCHAR DEFAULT 'CREATED',
        FOREIGN KEY (project_id) references projects (id)
    );

-- Copier les données de l'ancienne table vers la nouvelle
INSERT INTO
    tasks_new (
        id,
        name,
        description,
        project_id,
        priority,
        due_date,
        pinned,
        status
    )
SELECT
    id,
    name,
    description,
    project_id,
    priority,
    due_date,
    pinned,
    'CREATED'
FROM
    tasks;

-- Supprimer l'ancienne table
DROP TABLE tasks;

-- Renommer la nouvelle table avec le nom original
ALTER TABLE tasks_new
RENAME TO tasks;