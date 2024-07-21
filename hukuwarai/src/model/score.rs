use std::collections::HashMap;

use chrono::NaiveDateTime;
use openapi::models::GetScoresPredata200ResponseInner;
use serde::Serialize;

#[derive(sqlx::FromRow)]
pub struct Score {
    pub id: i32,
    pub player_id: i32,
    pub game_id: i32,
    pub score: f64,
    pub code: String,
    pub rendered_url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct PreScore {
    player_id: i32,
    scores: Vec<openapi::models::Score>,
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
            created_at: Some(score.created_at.and_utc()),
        }
    }
}

impl From<PreScore> for GetScoresPredata200ResponseInner {
    fn from(pre_score: PreScore) -> Self {
        GetScoresPredata200ResponseInner {
            player_id: Some(pre_score.player_id),
            scores: Some(pre_score.scores),
        }
    }
}

pub async fn get_all_scores(pool: &sqlx::PgPool) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>("SELECT * FROM scores")
        .fetch_all(pool)
        .await?;
    Ok(scores)
}

pub async fn get_pre_scores(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<PreScore>, sqlx::Error> {
    let scores = get_scores_by_game_id(pool, game_id).await?;
    let mut grouped_scores: HashMap<i32, Vec<openapi::models::Score>> = HashMap::new();

    for score in scores {
        let entry = grouped_scores
            .entry(score.player_id)
            .or_insert_with(Vec::new);
        entry.push(score.into());
    }

    let pre_scores: Vec<PreScore> = grouped_scores
        .into_iter()
        .map(|(player_id, scores)| PreScore { player_id, scores })
        .collect();

    Ok(pre_scores)
}

pub async fn get_score_by_id(
    pool: &sqlx::PgPool,
    score_id: i32,
) -> Result<Option<Score>, sqlx::Error> {
    let score = sqlx::query_as::<_, Score>("SELECT * FROM scores WHERE id = $1")
        .bind(score_id)
        .fetch_optional(pool)
        .await?;
    Ok(score)
}

pub async fn get_scores_by_game_id(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>("SELECT * FROM scores WHERE game_id = $1")
        .bind(game_id)
        .fetch_all(pool)
        .await?;
    Ok(scores)
}

pub async fn get_final_scores_by_game_id(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<Score>, sqlx::Error> {
    let scores = sqlx::query_as::<_, Score>(
        "SELECT s.id, s.player_id, s.game_id, s.score, s.code, s.rendered_url, s.created_at
        FROM scores s
        INNER JOIN (
            SELECT player_id, MAX(created_at) as latest_score
            FROM scores
            WHERE game_id = $1
            GROUP BY player_id
        ) latest_scores
        ON s.player_id = latest_scores.player_id AND s.created_at = latest_scores.latest_score
        WHERE s.game_id = $1
        ORDER BY s.score DESC",
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
        "INSERT INTO scores (player_id, game_id, score, code, rendered_url) VALUES ($1, $2, $3, $4, $5) RETURNING id, player_id, game_id, score, code, rendered_url, created_at",
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
