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
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: PostScoresPathParams,
        _body: Option<PostScoresRequest>,
    ) -> Result<PostScoresResponse, String> {
        todo!()
    }
}
