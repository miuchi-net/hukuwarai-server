use std::env;

use axum::{async_trait, extract::Host, http::Method};
use axum_extra::extract::CookieJar;
use openapi::{
    apis::scores::{
        GetScoresPredataResponse, GetScoresResponse, GetScoresResultResponse, PostScoresResponse,
        Scores,
    },
    models::{
        GetScoresPathParams, GetScoresPredata200ResponseInner, GetScoresPredataPathParams,
        GetScoresResultPathParams, PostScoresPathParams, PostScoresRequest,
    },
};
use serde_json::Value;

use crate::model::{
    game::get_game_by_id,
    score::{add_score, get_final_scores_by_game_id, get_pre_scores, get_scores_by_game_id},
};

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
            None => {
                tracing::error!("Missing body");
                return Err("Missing body".to_string());
            }
        };
        let inference_api_endpoint = env::var("INFERENCE_API_ENDPOINT").unwrap();
        let render_api_url = format!("{}/render", inference_api_endpoint);
        let response = reqwest::Client::new()
            .post(&render_api_url)
            .json(&serde_json::json!({
                "html_src": body.code
            }))
            .send()
            .await
            .map_err(|err| err.to_string())?;
        if !response.status().is_success() {
            return Err(format!("Failed to render: {}", response.status()));
        }
        let response_body: Value = response.json().await.map_err(|err| err.to_string())?;
        let rendered_url = response_body["image_url"]
            .as_str()
            .ok_or_else(|| {
                println!(
                    "response_body does not contain 'image_url': {:?}",
                    response_body
                );
                "Missing image_url in response".to_string()
            })?
            .to_string();
        let game = get_game_by_id(&self.pool, path_params.game_id).await;
        let game = match game {
            Ok(game) => match game {
                Some(game) => game,
                None => {
                    tracing::error!("Game not found");
                    return Err("Game not found".to_string());
                }
            },
            Err(err) => {
                tracing::error!("Failed to get game by id: {err}");
                return Err(err.to_string());
            }
        };
        let similarity_api_url = format!("{}/similarity", inference_api_endpoint);
        let response = reqwest::Client::new()
            .post(&similarity_api_url)
            .json(&serde_json::json!({
                "img1": {
                    "url": rendered_url
                },
                "img2": {
                    "url": game.answer_url
                },
                "model_name": "mse"
            }))
            .send()
            .await
            .map_err(|err| err.to_string())?;
        if !response.status().is_success() {
            return Err(format!("Failed to render: {}", response.status()));
        }
        let response_body: Value = response.json().await.map_err(|err| err.to_string())?;
        println!("response_body: {:?}", response_body);
        let score_value = response_body["similarity"].as_f64().ok_or_else(|| {
            println!(
                "response_body does not contain 'similarity': {:?}",
                response_body
            );
            "Missing similarity in response".to_string()
        })?;
        let score = match add_score(
            &self.pool,
            body.player_id,
            path_params.game_id,
            score_value,
            &body.code,
            &rendered_url,
        )
        .await
        {
            Ok(score) => score,
            Err(err) => {
                tracing::error!("Failed to create record to scores table: {err}");
                return Err(err.to_string());
            }
        };

        Ok(PostScoresResponse::Status200(score.into()))
    }

    async fn get_scores_result(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetScoresResultPathParams,
    ) -> Result<GetScoresResultResponse, String> {
        let scores = match get_final_scores_by_game_id(&self.pool, path_params.game_id).await {
            Ok(scores) => scores,
            Err(err) => return Err(err.to_string()),
        };
        let scores = scores.into_iter().map(|score| score.into()).collect();
        Ok(GetScoresResultResponse::Status200(scores))
    }

    async fn get_scores(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetScoresPathParams,
    ) -> Result<GetScoresResponse, String> {
        let scores = match get_scores_by_game_id(&self.pool, path_params.game_id).await {
            Ok(scores) => scores,
            Err(err) => return Err(err.to_string()),
        };
        let scores = scores.into_iter().map(|score| score.into()).collect();
        Ok(GetScoresResponse::Status200(scores))
    }

    async fn get_scores_predata(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: GetScoresPredataPathParams,
    ) -> Result<GetScoresPredataResponse, String> {
        let scores = match get_pre_scores(&self.pool, path_params.game_id).await {
            Ok(scores) => scores,
            Err(err) => return Err(err.to_string()),
        };
        let scores = scores.into_iter().map(|score| score.into()).collect();
        Ok(GetScoresPredataResponse::Status200(scores))
    }
}
