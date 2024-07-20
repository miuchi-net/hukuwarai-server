use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::players::{GetPlayersResponse, Players, PostPlayersResponse},
    models,
};

use crate::model::player::add_player;

use super::api_impl::ApiImpl;

#[async_trait]
impl Players for ApiImpl {
    async fn get_players(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::GetPlayersPathParams,
    ) -> Result<GetPlayersResponse, String> {
        todo!()
    }

    async fn post_players(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PostPlayersPathParams,
        body: Option<models::PostPlayersRequest>,
    ) -> Result<PostPlayersResponse, String> {
        let body = match body {
            Some(body) => body,
            None => return Err("body is required".to_string()),
        };
        let player = match add_player(&self.pool, &body.name, path_params.game_id).await {
            Ok(player) => player,
            Err(err) => return Err(err.to_string()),
        };
        Ok(PostPlayersResponse::Status200(models::Player::from(player)))
    }
}
