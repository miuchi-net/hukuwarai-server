use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::votes::{GetVoteResponse, PostVoteResponse, Votes},
    models::{self, PostVote200Response},
};

use crate::model::vote::{add_vote, get_votes_from_game_id};

use super::api_impl::ApiImpl;

#[async_trait]
impl Votes for ApiImpl {
    async fn get_vote(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::GetVotePathParams,
    ) -> Result<GetVoteResponse, String> {
        let votes = get_votes_from_game_id(&self.pool, path_params.game_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch votes: {e}");
                "Failed to fetch votes".to_string()
            })?
            .into_iter()
            .map(|v| v.into())
            .collect::<Vec<_>>();

        Ok(GetVoteResponse::Status200(votes))
    }

    async fn post_vote(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: models::PostVotePathParams,
        body: Option<models::PostVoteRequest>,
    ) -> Result<PostVoteResponse, String> {
        let body = match body {
            Some(v) => v,
            None => {
                tracing::error!("body must be provided");
                return Err("body must be provided".to_string());
            }
        };
        let _vote = add_vote(&self.pool, path_params.game_id, body.player_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create record to votes: {e}");
                "Failed to create record to votes".to_string()
            })?;

        Ok(PostVoteResponse::Status200(PostVote200Response {
            status: Some("ok".to_string()),
        }))
    }
}
