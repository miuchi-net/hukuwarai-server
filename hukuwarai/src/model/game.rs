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
    game_id: i32,
) -> Result<Option<Game>, sqlx::Error> {
    let game = sqlx::query_as::<_, Game>("SELECT * FROM games WHERE id = $1")
        .bind(game_id)
        .fetch_optional(pool)
        .await?;
    Ok(game)
}

pub async fn add_game(
    pool: &sqlx::PgPool,
    name: &str,
    answer_url: &str,
) -> Result<Game, sqlx::Error> {
    let result = sqlx::query_as::<_, Game>(
        "INSERT INTO games (name, answer_url,started, finished) VALUES ($1, $2, false, false) RETURNING id, name, answer_url, started, finished",
    ).bind(name)
    .bind(answer_url)
    .fetch_one(pool)
    .await?;
    Ok(result)
}

pub async fn update_game(
    pool: &sqlx::PgPool,
    game: openapi::models::Game,
) -> Result<Game, sqlx::Error> {
    let result = sqlx::query_as::<_, Game>(
        "UPDATE games SET name = $1, started = $2, finished = $3, answer_url = $4 WHERE id = $5 RETURNING id, name, started, finished, answer_url",
    ).bind(&game.name)
    .bind(game.started)
    .bind(game.finished)
    .bind(&game.answer_url)
    .bind(game.id)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
