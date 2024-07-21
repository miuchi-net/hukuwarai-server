-- Add up migration script here
create table scores (
    id serial primary key,
    player_id serial not null,
    game_id serial not null,
    score float not null,
    code text not null,
    rendered_url text not null,
    created_at timestamp not null default now(),
    foreign key (player_id) references players(id),
    foreign key (game_id) references games(id)
);
