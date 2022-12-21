use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_http::cors::{AllowHeaders, CorsLayer};

use crate::endpoints::{v1, v2};

pub fn endpoints(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/api/v1/random", get(v1::random::endpoint))
        .route("/api/v1/meta", get(v1::meta::endpoint))
        .route("/api/v1/by/:type/:category", get(v1::by::endpoint))
        .route("/api/v1/asset/:name", get(v1::asset::endpoint))
        .route("/api/v2/img/:category", get(v2::img::get))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers(AllowHeaders::any())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
