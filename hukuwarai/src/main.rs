use std::env;

use handlers::api_impl::ApiImpl;
use sqlx::postgres::PgPoolOptions;

pub mod handlers;
pub mod model;

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

    let router = openapi::server::new(ApiImpl { pool });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:31000")
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
