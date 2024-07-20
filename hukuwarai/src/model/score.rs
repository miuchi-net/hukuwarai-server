#[derive(sqlx::FromRow)]
pub struct Score {
    pub id: i64,
    pub player_id: i64,
    pub game_id: i64,
    pub score: f64,
    pub code: String,
}

pub async fn get_all_scores(pool: &sqlx::PgPool) -> Result<Vec<Score>, sqlx::Error> {
    let scores =
        sqlx::query_as::<_, Score>("SELECT id, player_id, game_id, score, code FROM scores")
            .fetch_all(pool)
            .await?;
    Ok(scores)
}

pub async fn get_score_by_id(
    pool: &sqlx::PgPool,
    score_id: i64,
) -> Result<Option<Score>, sqlx::Error> {
    let score = sqlx::query_as::<_, Score>(
        "SELECT id, player_id, game_id, score, code FROM scores WHERE id = $1",
    )
    .bind(score_id)
    .fetch_optional(pool)
    .await?;
    Ok(score)
}

pub async fn add_score(
    pool: &sqlx::PgPool,
    player_id: i64,
    game_id: i64,
    score_value: f64,
    code: &str,
) -> Result<Score, sqlx::Error> {
    let result = sqlx::query_as::<_, Score>(
        "INSERT INTO scores (player_id, game_id, score, code) VALUES ($1, $2, $3, $4) RETURNING id, player_id, game_id, score, code"
    )
    .bind(player_id)
    .bind(game_id)
    .bind(score_value)
    .bind(code)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
