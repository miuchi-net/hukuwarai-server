-- Add up migration script here
create table players (
    id int primary key,
    name text not null,
    game_id int not null,
    foreign key game_id references games(id),
);

