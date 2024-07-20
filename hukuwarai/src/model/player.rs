#[derive(sqlx::FromRow)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub game_id: i32,
}

impl From<Player> for openapi::models::Player {
    fn from(player: Player) -> Self {
        openapi::models::Player {
            id: Some(player.id),
            name: Some(player.name),
            game_id: Some(player.game_id),
        }
    }
}

pub async fn get_all_players(pool: &sqlx::PgPool) -> Result<Vec<Player>, sqlx::Error> {
    let players = sqlx::query_as::<_, Player>("SELECT id, name, game_id FROM players")
        .fetch_all(pool)
        .await?;
    Ok(players)
}

pub async fn get_all_players_from_game_id(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<Player>, sqlx::Error> {
    let players =
        sqlx::query_as::<_, Player>("SELECT id, name, game_id FROM players WHERE game_id = $1")
            .bind(game_id)
            .fetch_all(pool)
            .await?;
    Ok(players)
}

pub async fn get_player_by_id(
    pool: &sqlx::PgPool,
    player_id: i64,
) -> Result<Option<Player>, sqlx::Error> {
    let player = sqlx::query_as::<_, Player>("SELECT id, name, game_id FROM players WHERE id = $1")
        .bind(player_id)
        .fetch_optional(pool)
        .await?;
    Ok(player)
}

pub async fn add_player(
    pool: &sqlx::PgPool,
    name: &str,
    game_id: i64,
) -> Result<Player, sqlx::Error> {
    let result = sqlx::query_as::<_, Player>(
        "INSERT INTO players (name, game_id) VALUES ($1, $2) RETURNING id, name, game_id",
    )
    .bind(name)
    .bind(game_id)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
