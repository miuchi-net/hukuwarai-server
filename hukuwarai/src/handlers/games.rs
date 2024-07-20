use axum::{
    async_trait,
    extract::Host,
    http::{method, Method},
};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::games::{Games, GetGameResponse, GetGamesResponse, PostGamesResponse, PutGameResponse},
    models::{self, PutGamePathParams},
};

use crate::model::game::{get_all_games, get_game_by_id};

use super::api_impl::ApiImpl;

#[async_trait]
impl Games for ApiImpl {
    async fn get_game(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::GetGamePathParams,
    ) -> Result<GetGameResponse, String> {
        let game = match get_game_by_id(&self.pool, path_params.game_id).await {
            Ok(game) => match game {
                Some(game) => openapi::models::Game::from(game),
                None => return Ok(GetGameResponse::Status404_NotFound)
            }
            Err(err) => return Err(err.to_string())
        };
        Ok(GetGameResponse::Status200(game))
    }

    async fn get_games(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
    ) -> Result<GetGamesResponse, String> {
        let games = match get_all_games(&self.pool).await {
            Ok(games) => games.into_iter().map(openapi::models::Game::from).collect(),
            Err(err) => return Err(err.to_string()),
        };
        Ok(GetGamesResponse::Status200(games))
    }

    async fn post_games(
        &self,
        method: Method,
        _host: Host,
        _cookies: CookieJar,
        body: Option<models::PostGamesRequest>,
    ) -> Result<PostGamesResponse, String> {
        if method != method::Method::POST {
            tracing::error!("Invalid request method. Expected POST, got {:?}", method);
            return Err(format!(
                "Invalid request method. Expected POST, got {:?}",
                method
            ));
        }

        let body = match body {
            Some(v) => v,
            None => {
                tracing::error!("body must be provided");
                return Err("body must be provided.".to_string());
            }
        };

        let game = sqlx::query_as::<_, crate::model::game::Game>(
            r#"INSERT INTO games (name, answer_url) VALUES ($1, $2) RETURNING *"#,
        )
        .bind(body.name)
        .bind(body.answer_url)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::info!("Failed to create record to the games table: {}", e);
            "Failed to create record to the games table.".to_string()
        })?;

        Ok(PostGamesResponse::Status200(game.into()))
    }

    async fn put_game(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: PutGamePathParams,
        _body: Option<models::Game>,
    ) -> Result<PutGameResponse, String> {
        todo!()
    }
}
