use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_http::cors::{AllowHeaders, CorsLayer};

use crate::endpoints::{asset, by, meta, random};
pub fn endpoints(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/api/v1/random", get(random::endpoint))
        .route("/api/v1/meta", get(meta::endpoint))
        .route("/api/v1/by/:type/:category", get(by::endpoint))
        .route("/api/v1/asset/:name", get(asset::endpoint))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
