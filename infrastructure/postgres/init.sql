CREATE DATABASE todo;
CREATE USER todo WITH PASSWORD 'password';
GRANT ALL ON DATABASE todo TO todo;

\connect todo todo;
CREATE SCHEMA todo;

create table if not exists todo.todo
(
    id          serial primary key not null,
    owner       varchar not null,
    name        varchar not null,
    description varchar not null,
    status      varchar not null,
    created     timestamp with time zone default (now() at time zone 'utc'),
    modified    timestamp with time zone default (now() at time zone 'utc')
);