#[derive(sqlx::FromRow)]
pub struct Score {
    pub id: i32,
    pub player_id: i32,
    pub game_id: i32,
    pub score: f64,
    pub code: String,
    pub rendered_url: String,
    pub created_at: String,
}

impl From<Score> for openapi::models::Score {
    fn from(score: Score) -> Self {
        openapi::models::Score {
            id: Some(score.id),
            player_id: Some(score.player_id),
            game_id: Some(score.game_id),
            score: Some(score.score),
            code: Some(score.code),
            rendered_url: Some(score.rendered_url),
            created_at: Some(score.created_at.parse().unwrap()),
        }
    }
}

pub async fn get_all_scores(pool: &sqlx::PgPool) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>(
        "SELECT id, player_id, game_id, score, code, rendered_url FROM scores",
    )
    .fetch_all(pool)
    .await?;
    Ok(scores)
}

pub async fn get_score_by_id(
    pool: &sqlx::PgPool,
    score_id: i32,
) -> Result<Option<Score>, sqlx::Error> {
    let score = sqlx::query_as::<_, Score>(
        "SELECT id, player_id, game_id, score, code, rendered_url FROM scores WHERE id = $1",
    )
    .bind(score_id)
    .fetch_optional(pool)
    .await?;
    Ok(score)
}

pub async fn get_scores_by_game_id(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>(
        "SELECT id, player_id, game_id, score, code, rendered_url FROM scores WHERE game_id = $1",
    )
    .bind(game_id)
    .fetch_all(pool)
    .await?;
    Ok(scores)
}

pub async fn add_score(
    pool: &sqlx::PgPool,
    player_id: i32,
    game_id: i32,
    score_value: f64,
    code: &str,
    rendered_url: &str,
) -> Result<Score, sqlx::Error> {
    let result = sqlx::query_as::<_, Score>(
        "INSERT INTO scores (player_id, game_id, score, code, rendered_url) VALUES ($1, $2, $3, $4, $5) RETURNING id, player_id, game_id, score, code, rendered_url",
    )
    .bind(player_id)
    .bind(game_id)
    .bind(score_value)
    .bind(code)
    .bind(rendered_url)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
