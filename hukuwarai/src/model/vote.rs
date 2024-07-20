#[derive(sqlx::FromRow)]
pub struct Vote {
    pub id: i64,
    pub game_id: i64,
    pub player_id: i64,
}

pub async fn get_all_votes(pool: &sqlx::PgPool) -> Result<Vec<Vote>, sqlx::Error> {
    let votes = sqlx::query_as::<_, Vote>("SELECT id, game_id, player_id FROM votes")
        .fetch_all(pool)
        .await?;
    Ok(votes)
}

pub async fn add_vote(
    pool: &sqlx::PgPool,
    game_id: i64,
    player_id: i64,
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
