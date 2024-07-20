use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::scores::{PostScoresResponse, Scores},
    models::{PostScoresPathParams, PostScoresRequest},
};

use super::api_impl::ApiImpl;

#[async_trait]
impl Scores for ApiImpl {
    async fn post_scores(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: PostScoresPathParams,
        body: Option<PostScoresRequest>,
    ) -> Result<PostScoresResponse, String> {
        todo!()
    }
}
