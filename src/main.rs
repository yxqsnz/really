use axum::Server;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::{env::var, net::SocketAddr};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod app;
mod endpoints;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "really=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&var("Database.Path")?)
        .await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app::endpoints(pool).into_make_service())
        .await?;
    Ok(())
}
