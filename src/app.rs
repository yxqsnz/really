use axum::{routing::get, Router};
use sqlx::{Pool, Sqlite};

use crate::endpoints::random;

pub fn endpoints(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/api/v1/random", get(random::endpoint))
        .with_state(pool)
}
