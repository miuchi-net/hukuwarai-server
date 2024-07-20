use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::games::Games + apis::players::Players + apis::scores::Scores + apis::votes::Votes + 'static,
{
    // build our application with a route
    Router::new()
        .route("/games",
            get(get_games::<I, A>).post(post_games::<I, A>)
        )
        .route("/games/:game_id",
            get(get_game::<I, A>).put(put_game::<I, A>)
        )
        .route("/players/:game_id",
            get(get_players::<I, A>).post(post_players::<I, A>)
        )
        .route("/scores/$:game_id",
            post(post_scores::<I, A>)
        )
        .route("/votes/:game_id",
            get(get_vote::<I, A>).post(post_vote::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn get_game_validation(
  path_params: models::GetGamePathParams,
) -> std::result::Result<(
  models::GetGamePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetGame - GET /games/{gameId}
#[tracing::instrument(skip_all)]
async fn get_game<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetGamePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::games::Games,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_game_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_game(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::games::GetGameResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::games::GetGameResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::games::GetGameResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_games_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// GetGames - GET /games
#[tracing::instrument(skip_all)]
async fn get_games<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::games::Games,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_games_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_games(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::games::GetGamesResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostGamesBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::PostGamesRequest,
    }


#[tracing::instrument(skip_all)]
fn post_games_validation(
        body: Option<models::PostGamesRequest>,
) -> std::result::Result<(
        Option<models::PostGamesRequest>,
), ValidationErrors>
{
            if let Some(body) = &body {
              let b = PostGamesBodyValidator { body };
              b.validate()?;
            }

Ok((
    body,
))
}
/// PostGames - POST /games
#[tracing::instrument(skip_all)]
async fn post_games<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::PostGamesRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::games::Games,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_games_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().post_games(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::games::PostGamesResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PutGameBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Game,
    }


#[tracing::instrument(skip_all)]
fn put_game_validation(
  path_params: models::PutGamePathParams,
        body: Option<models::Game>,
) -> std::result::Result<(
  models::PutGamePathParams,
        Option<models::Game>,
), ValidationErrors>
{
  path_params.validate()?;
            if let Some(body) = &body {
              let b = PutGameBodyValidator { body };
              b.validate()?;
            }

Ok((
  path_params,
    body,
))
}
/// PutGame - PUT /games/{gameId}
#[tracing::instrument(skip_all)]
async fn put_game<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PutGamePathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::Game>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::games::Games,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    put_game_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().put_game(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::games::PutGameResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::games::PutGameResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::games::PutGameResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_players_validation(
  path_params: models::GetPlayersPathParams,
) -> std::result::Result<(
  models::GetPlayersPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetPlayers - GET /players/{gameId}
#[tracing::instrument(skip_all)]
async fn get_players<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetPlayersPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::players::Players,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_players_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_players(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::players::GetPlayersResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::players::GetPlayersResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::players::GetPlayersResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostPlayersBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::PostPlayersRequest,
    }


#[tracing::instrument(skip_all)]
fn post_players_validation(
  path_params: models::PostPlayersPathParams,
        body: Option<models::PostPlayersRequest>,
) -> std::result::Result<(
  models::PostPlayersPathParams,
        Option<models::PostPlayersRequest>,
), ValidationErrors>
{
  path_params.validate()?;
            if let Some(body) = &body {
              let b = PostPlayersBodyValidator { body };
              b.validate()?;
            }

Ok((
  path_params,
    body,
))
}
/// PostPlayers - POST /players/{gameId}
#[tracing::instrument(skip_all)]
async fn post_players<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PostPlayersPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::PostPlayersRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::players::Players,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_players_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().post_players(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::players::PostPlayersResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::players::PostPlayersResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::players::PostPlayersResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostScoresBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::PostScoresRequest,
    }


#[tracing::instrument(skip_all)]
fn post_scores_validation(
  path_params: models::PostScoresPathParams,
        body: Option<models::PostScoresRequest>,
) -> std::result::Result<(
  models::PostScoresPathParams,
        Option<models::PostScoresRequest>,
), ValidationErrors>
{
  path_params.validate()?;
            if let Some(body) = &body {
              let b = PostScoresBodyValidator { body };
              b.validate()?;
            }

Ok((
  path_params,
    body,
))
}
/// PostScores - POST /scores/${gameId}
#[tracing::instrument(skip_all)]
async fn post_scores<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PostScoresPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::PostScoresRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::scores::Scores,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_scores_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().post_scores(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::scores::PostScoresResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::scores::PostScoresResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::scores::PostScoresResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_vote_validation(
  path_params: models::GetVotePathParams,
) -> std::result::Result<(
  models::GetVotePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// GetVote - GET /votes/{gameId}
#[tracing::instrument(skip_all)]
async fn get_vote<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetVotePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::votes::Votes,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    get_vote_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().get_vote(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::votes::GetVoteResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::votes::GetVoteResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::votes::GetVoteResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PostVoteBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::PostVoteRequest,
    }


#[tracing::instrument(skip_all)]
fn post_vote_validation(
  path_params: models::PostVotePathParams,
        body: Option<models::PostVoteRequest>,
) -> std::result::Result<(
  models::PostVotePathParams,
        Option<models::PostVoteRequest>,
), ValidationErrors>
{
  path_params.validate()?;
            if let Some(body) = &body {
              let b = PostVoteBodyValidator { body };
              b.validate()?;
            }

Ok((
  path_params,
    body,
))
}
/// PostVote - POST /votes/{gameId}
#[tracing::instrument(skip_all)]
async fn post_vote<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PostVotePathParams>,
 State(api_impl): State<I>,
          Json(body): Json<Option<models::PostVoteRequest>>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::votes::Votes,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    post_vote_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().post_vote(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::votes::PostVoteResponse::Status200
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::votes::PostVoteResponse::Status400_BadRequest
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::votes::PostVoteResponse::Status404_NotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

