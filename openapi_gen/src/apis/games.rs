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
pub enum GetGameResponse {
    /// 対象のゲームのデータを返します
    Status200
    (models::Game)
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
pub enum GetGamesResponse {
    /// 作成されたゲームを全て返します
    Status200
    (Vec<models::Game>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PostGamesResponse {
    /// ゲームを作成した結果を返します
    Status200
    (models::Game)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PutGameResponse {
    /// 対象のゲームのデータを変更したものを返します
    Status200
    (models::Game)
    ,
    /// Bad Request
    Status400_BadRequest
    ,
    /// Not Found
    Status404_NotFound
}


/// Games
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Games {
    /// ゲーム取得API.
    ///
    /// GetGame - GET /games/{gameId}
    async fn get_game(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::GetGamePathParams,
    ) -> Result<GetGameResponse, String>;

    /// ゲーム全取得API.
    ///
    /// GetGames - GET /games
    async fn get_games(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<GetGamesResponse, String>;

    /// ゲーム作成API.
    ///
    /// PostGames - POST /games
    async fn post_games(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::PostGamesRequest>,
    ) -> Result<PostGamesResponse, String>;

    /// ゲームデータ変更API.
    ///
    /// PutGame - PUT /games/{gameId}
    async fn put_game(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PutGamePathParams,
            body: Option<models::Game>,
    ) -> Result<PutGameResponse, String>;
}
