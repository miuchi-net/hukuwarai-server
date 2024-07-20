-- Add up migration script here
create votes (
    id int primary key,
    game_id int not null,
    player_id int not null,
    foreign key (game_id) references games(id),
    foreign key (player_id) references players(id)
);
