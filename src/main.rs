use axum::{
    routing::{get, post, put},
    Extension, Router,
};

use anyhow::Context;
use controllers::message::{all_messages, new_message, update_message};
use sqlx::postgres::PgPoolOptions;
use std::fs;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .context("could not connect to database_url")?;

    let app = Router::new()
        .route("/hello", get(root))
        .route("/messages", get(all_messages))
        .route("/messages", post(new_message))
        .route("/messages/:id", put(update_message))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
