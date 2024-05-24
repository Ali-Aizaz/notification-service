use axum::{
    middleware,
    routing::{get, post, put},
    Extension, Router,
};

pub use self::errors::{CustomError, Result};
use anyhow::Context;
use controllers::{message, mw_auth, user};
use sqlx::postgres::{PgListener, PgPoolOptions};
use std::fs;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod ctx;
mod errors;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "CONNECTION_STRING");

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

    // sqlx::migrate!("./migrations").run(&pool).await?;
    let listen_task = tokio::spawn(listen_notifications());

    let app = Router::new()
        .route("/hello", get(root))
        .route("/messages", get(message::all_messages))
        .route("/messages", post(message::new_message))
        .route("/messages/:id", put(message::update_message))
        .layer(middleware::from_fn(mw_auth::mw_ctx_resolver))
        .route("/auth/signup", post(user::signup))
        .route("/auth/login", post(user::login))
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener);
    axum::serve(listener, app).await.unwrap();
    listen_task.await??;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn listen_notifications() -> anyhow::Result<()> {
    // Establish a connection to the database
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "CONNECTION_STRING");

    let mut listener = PgListener::connect(database_url).await?;
    listener.listen("channel_msg").await?;

    // Continuously receive notifications

    loop {
        // Ask for the next notification, re-connecting transparently if needed
        let notification = listener.recv().await?;
        println!("Received notification: {:?}", notification);
    }
}
