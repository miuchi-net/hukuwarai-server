use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::games::{Games, GetGameResponse, GetGamesResponse, PostGamesResponse, PutGameResponse},
    models::{self, PutGamePathParams},
};

use crate::model::game::get_all_games;

use super::api_impl::ApiImpl;

#[async_trait]
impl Games for ApiImpl {
    async fn get_game(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::GetGamePathParams,
    ) -> Result<GetGameResponse, String> {
        todo!()
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
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _body: Option<models::PostGamesRequest>,
    ) -> Result<PostGamesResponse, String> {
        todo!()
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
