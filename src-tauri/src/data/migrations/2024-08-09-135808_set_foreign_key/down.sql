-- This file should undo anything in `up.sql`

drop table projects;

create table tasks_new(
id integer PRIMARY KEY AUTOINCREMENT,
name VARCHAR NOT NULL,
description TEXT,
project_id INT,
priority VARCHAR,
due_date DATETIME,
pinned BOOLEAN

);
insert into tasks_new (id, name, description, project_id,priority,due_date,pinned)
select * from tasks;

drop table tasks;

alter table tasks_new rename to tasks;