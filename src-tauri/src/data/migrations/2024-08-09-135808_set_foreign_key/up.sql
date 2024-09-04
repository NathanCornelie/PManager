-- Your SQL goes here
create table projects(
    id integer primary key autoincrement,
    path varchar,
    name varchar not null,
    description varchar
);

create table tasks_new(
id integer PRIMARY KEY AUTOINCREMENT,
name VARCHAR NOT NULL,
description TEXT,
project_id INT,
priority VARCHAR,
due_date DATETIME,
pinned BOOLEAN,
FOREIGN KEY(project_id) references projects(id)
);

insert into tasks_new (id, name, description, project_id,priority,due_date,pinned)
select * from tasks;

drop table tasks;

alter table tasks_new rename to tasks;

