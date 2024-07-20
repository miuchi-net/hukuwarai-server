#[derive(sqlx::FromRow)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub started: bool,
    pub finished: bool,
    pub answer_url: String,
}

impl From<Game> for openapi::models::Game {
    fn from(game: Game) -> Self {
        openapi::models::Game {
            id: Some(game.id),
            name: Some(game.name),
            started: Some(game.started),
            finished: Some(game.finished),
            answer_url: Some(game.answer_url),
        }
    }
}

pub async fn get_all_games(pool: &sqlx::PgPool) -> Result<Vec<Game>, sqlx::Error> {
    let games = sqlx::query_as::<_, Game>("SELECT id, name, started, finished FROM games")
        .fetch_all(pool)
        .await?;
    Ok(games)
}

pub async fn get_game_by_id(
    pool: &sqlx::PgPool,
    game_id: i64,
) -> Result<Option<Game>, sqlx::Error> {
    let game =
        sqlx::query_as::<_, Game>("SELECT id, name, started, finished FROM games WHERE id = $1")
            .bind(game_id)
            .fetch_optional(pool)
            .await?;
    Ok(game)
}

pub async fn add_game(pool: &sqlx::PgPool, name: &str) -> Result<Game, sqlx::Error> {
    let result = sqlx::query_as::<_, Game>(
        "INSERT INTO games (name, started, finished) VALUES ($1, false, false) RETURNING id, name, started, finished",
    ).bind(name)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
