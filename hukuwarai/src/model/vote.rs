#[derive(sqlx::FromRow)]
pub struct Vote {
    pub id: i32,
    pub game_id: i32,
    pub player_id: i32,
}

impl From<Vote> for openapi::models::Vote {
    fn from(vote: Vote) -> Self {
        openapi::models::Vote {
            id: Some(vote.id),
            game_id: Some(vote.game_id),
            player_id: Some(vote.player_id),
        }
    }
}

pub async fn get_all_votes(pool: &sqlx::PgPool) -> Result<Vec<Vote>, sqlx::Error> {
    let votes = sqlx::query_as::<_, Vote>("SELECT id, game_id, player_id FROM votes")
        .fetch_all(pool)
        .await?;
    Ok(votes)
}

pub async fn get_votes_from_game_id(
    pool: &sqlx::PgPool,
    game_id: i32,
) -> Result<Vec<Vote>, sqlx::Error> {
    let votes = sqlx::query_as::<_, Vote>("SELECT * FROM votes WHERE game_id = $1")
        .bind(game_id)
        .fetch_all(pool)
        .await?;

    Ok(votes)
}

pub async fn add_vote(
    pool: &sqlx::PgPool,
    game_id: i32,
    player_id: i32,
) -> Result<Vote, sqlx::Error> {
    let result = sqlx::query_as::<_, Vote>(
        "INSERT INTO votes (game_id, player_id) VALUES ($1, $2) RETURNING id, game_id, player_id",
    )
    .bind(game_id)
    .bind(player_id)
    .fetch_one(pool)
    .await?;
    Ok(result)
}
