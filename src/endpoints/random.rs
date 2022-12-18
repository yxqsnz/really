use crate::utils::{database::Connection, image::Image, status_code::internal_error};
use {axum::extract::Query, axum::http::StatusCode};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    nsfw: bool,
}

pub async fn endpoint(
    Connection(mut conn): Connection,
    Query(params): Query<Params>,
) -> Result<Image, (StatusCode, String)> {
    sqlx::query_as("select name, content from asset where nsfw=? order by random() limit 1")
        .bind(params.nsfw)
        .fetch_one(&mut conn)
        .await
        .map_err(internal_error)
        .map(|(name, content)| Image(name, content))
}
