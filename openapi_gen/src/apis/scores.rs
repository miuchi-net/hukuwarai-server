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
pub enum GetScoresResponse {
    /// そのゲームの全てのスコアを返します
    Status200
    (Vec<models::Score>)
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
pub enum GetScoresResultResponse {
    /// そのゲームの全ての最終スコアを返します
    Status200
    (Vec<models::Score>)
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
pub enum PostScoresResponse {
    /// 追加したスコアを返します
    Status200
    (models::Score)
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Not Found
    Status404_NotFound
}


/// Scores
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Scores {
    /// スコア取得API.
    ///
    /// GetScores - GET /scores/{gameId}
    async fn get_scores(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetScoresPathParams,
    ) -> Result<GetScoresResponse, String>;

    /// 結果スコア取得API.
    ///
    /// GetScoresResult - GET /scores/{gameId}/result
    async fn get_scores_result(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetScoresResultPathParams,
    ) -> Result<GetScoresResultResponse, String>;

    /// スコア追加API.
    ///
    /// PostScores - POST /scores/{gameId}
    async fn post_scores(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PostScoresPathParams,
            body: Option<models::PostScoresRequest>,
    ) -> Result<PostScoresResponse, String>;
}
