use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::players::{GetPlayersResponse, Players, PostPlayersResponse},
    models,
};

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
        _path_params: models::PostPlayersPathParams,
        _body: Option<models::PostPlayersRequest>,
    ) -> Result<PostPlayersResponse, String> {
        todo!()
    }
}
