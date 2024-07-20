-- Add up migration script here
create table scores (
    id int primary key,
    player_id int not null,
    game_id int not null,
    score float not null,
    code text not null,
    foreign key player_id references players(id),
    foreign key game_id references games(id),
);
