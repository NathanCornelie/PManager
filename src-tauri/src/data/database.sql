create table
    projects (
        id integer primary key autoincrement,
        path varchar,
        name varchar not null,
        description varchar
    );

create table
    tasks (
        id integer PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        description TEXT,
        project_id INT,
        priority VARCHAR,
        due_date DATETIME,
        pinned BOOLEAN,
        FOREIGN KEY (project_id) references projects (id)
    );