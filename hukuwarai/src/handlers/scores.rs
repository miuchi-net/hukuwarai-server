use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::scores::{GetScoresResponse, GetScoresResultResponse, PostScoresResponse, Scores},
    models::{
        GetScoresPathParams, GetScoresResultPathParams, PostScoresPathParams, PostScoresRequest,
    },
};

use crate::model::score::add_score;

use super::api_impl::ApiImpl;

#[async_trait]
impl Scores for ApiImpl {
    async fn post_scores(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: PostScoresPathParams,
        body: Option<PostScoresRequest>,
    ) -> Result<PostScoresResponse, String> {
        let body = match body {
            Some(body) => body,
            None => return Err("Missing body".to_string()),
        };
        let score_value = 0.0;
        let rendered_url = "dummy-url";
        let score = match add_score(
            &self.pool,
            body.player_id,
            path_params.game_id,
            score_value,
            &body.code,
            rendered_url,
        )
        .await
        {
            Ok(score) => score,
            Err(err) => return Err(err.to_string()),
        };
        Ok(PostScoresResponse::Status200(score.into()))
    }

    async fn get_scores_result(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetScoresResultPathParams,
    ) -> Result<GetScoresResultResponse, String> {
        todo!()
    }

    async fn get_scores(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        _path_params: GetScoresPathParams,
    ) -> Result<GetScoresResponse, String> {
        todo!()
    }
}
