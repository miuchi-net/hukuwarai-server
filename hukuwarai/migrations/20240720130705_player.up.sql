-- Add up migration script here
create table players (
    id serial primary key,
    name text not null,
    game_id serial not null,
    foreign key (game_id) references games(id)
);

