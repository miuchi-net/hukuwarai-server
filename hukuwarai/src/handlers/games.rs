use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::games::{Games, GetGameResponse, GetGamesResponse, PostGamesResponse, PutGameResponse},
    models::{self, PutGamePathParams},
};

use super::api_impl::ApiImpl;

#[async_trait]
impl Games for ApiImpl {
    async fn get_game(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetGamePathParams,
    ) -> Result<GetGameResponse, String> {
        todo!()
    }

    async fn get_games(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<GetGamesResponse, String> {
        todo!()
    }

    async fn post_games(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: Option<models::PostGamesRequest>,
    ) -> Result<PostGamesResponse, String> {
        todo!()
    }

    async fn put_game(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: PutGamePathParams,
        body: Option<models::Game>,
    ) -> Result<PutGameResponse, String> {
        todo!()
    }
}
