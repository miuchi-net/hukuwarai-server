use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::votes::{GetVoteResponse, PostVoteResponse, Votes},
    models,
};

use super::api_impl::ApiImpl;

#[async_trait]
impl Votes for ApiImpl {
    async fn get_vote(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::GetVotePathParams,
    ) -> Result<GetVoteResponse, String> {
        todo!()
    }

    async fn post_vote(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::PostVotePathParams,
        body: Option<models::PostVoteRequest>,
    ) -> Result<PostVoteResponse, String> {
        todo!()
    }
}
