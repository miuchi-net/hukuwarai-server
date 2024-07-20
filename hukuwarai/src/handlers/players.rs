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
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetPlayersPathParams,
    ) -> Result<GetPlayersResponse, String> {
        todo!()
    }

    async fn post_players(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PostPlayersPathParams,
        body: Option<models::PostPlayersRequest>,
    ) -> Result<PostPlayersResponse, String> {
        todo!()
    }
}
