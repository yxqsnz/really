use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite};
use tower_http::cors::CorsLayer;

use crate::endpoints::{asset, by, meta, random};
pub fn endpoints(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route("/api/v1/random", get(random::endpoint))
        .route("/api/v1/meta", get(meta::endpoint))
        .route("/api/v1/by/:type/:category", get(by::endpoint))
        .route("/api/v1/asset/:name", get(asset::endpoint))
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            // python3 -m http.server 8000 in `static`
            CorsLayer::new()
                .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .with_state(pool)
}
