-- Add up migration script here
create table votes (
    id serial primary key,
    game_id serial not null,
    player_id serial not null,
    foreign key (game_id) references games(id),
    foreign key (player_id) references players(id)
);
