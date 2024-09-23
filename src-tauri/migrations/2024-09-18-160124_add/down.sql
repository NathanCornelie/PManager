CREATE TABLE
    tasks_old (
        id integer PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        description TEXT,
        project_id INT,
        priority VARCHAR,
        due_date DATETIME,
        pinned BOOLEAN,
        FOREIGN KEY (project_id) references projects (id)
    );

-- Copier les données de la nouvelle table vers l'ancienne
INSERT INTO
    tasks_old (
        id,
        name,
        status,
        description,
        project_id,
        priority,
        due_date,
        pinned,
    )
SELECT
    id,
    name,
    status,
    description,
    project_id,
    priority,
    due_date,
    pinned,
FROM
    tasks;

-- Supprimer la table actuelle
DROP TABLE tasks;

-- Renommer l'ancienne table
ALTER TABLE tasks_old
RENAME TO tasks;