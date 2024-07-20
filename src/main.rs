use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;

pub mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("env `DATABASE_URL` must be set");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("Couldn't connect to the DB");

    sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .route("/", get(handlers::ping::ping))
        .route("/users", post(handlers::users::create_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
