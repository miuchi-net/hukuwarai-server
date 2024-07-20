use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetVoteResponse {
    /// 投票結果を返します
    Status200
    (Vec<models::Vote>)
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Not Found
    Status404_NotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostVoteResponse {
    /// 投票に対するレスポンスを返します
    Status200
    (models::PostVote200Response)
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Not Found
    Status404_NotFound
}


/// Votes
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Votes {
    /// 投票結果取得API.
    ///
    /// GetVote - GET /votes/{gameId}
    async fn get_vote(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetVotePathParams,
    ) -> Result<GetVoteResponse, String>;

    /// 投票API.
    ///
    /// PostVote - POST /votes/{gameId}
    async fn post_vote(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PostVotePathParams,
            body: Option<models::PostVoteRequest>,
    ) -> Result<PostVoteResponse, String>;
}
