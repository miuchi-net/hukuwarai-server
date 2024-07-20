-- Add up migration script here
create table games (
    id int primary key,
    name text not null,
    started boolean not null default false,
    finished boolean not null default false,
    answer_url text not null,
);
