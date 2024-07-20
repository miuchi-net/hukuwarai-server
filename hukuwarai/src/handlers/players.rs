use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::players::{GetPlayersResponse, Players, PostPlayersResponse},
    models::{self, GetPlayers200Response},
};

use crate::model::{self};

use super::api_impl::ApiImpl;

#[async_trait]
impl Players for ApiImpl {
    async fn get_players(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::GetPlayersPathParams,
    ) -> Result<GetPlayersResponse, String> {
        let players = model::player::get_all_players_from_game_id(&self.pool, path_params.game_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch players: {e}");
                "Failed to fetch players".to_string()
            })?;
        let players = players.into_iter().map(|p| p.into()).collect::<Vec<_>>();
        let resp = GetPlayersResponse::Status200(GetPlayers200Response {
            status: Some(players),
        });

        Ok(resp)
    }

    async fn post_players(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: models::PostPlayersPathParams,
        _body: Option<models::PostPlayersRequest>,
    ) -> Result<PostPlayersResponse, String> {
        todo!()
    }
}
