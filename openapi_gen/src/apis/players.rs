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
pub enum GetPlayersResponse {
    /// ゲームに参加しているプレイヤー情報を返します
    Status200
    (models::GetPlayers200Response)
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
pub enum PostPlayersResponse {
    /// 新規作成したプレイヤーデータを返します
    Status200
    (models::Player)
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Not Found
    Status404_NotFound
}


/// Players
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Players {
    /// プレイヤー取得API.
    ///
    /// GetPlayers - GET /players/{gameId}
    async fn get_players(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetPlayersPathParams,
    ) -> Result<GetPlayersResponse, String>;

    /// プレイヤー作成API.
    ///
    /// PostPlayers - POST /players/{gameId}
    async fn post_players(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PostPlayersPathParams,
            body: Option<models::PostPlayersRequest>,
    ) -> Result<PostPlayersResponse, String>;
}
