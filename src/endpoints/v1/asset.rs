use crate::utils::status_code::not_found_or_500;
use crate::utils::{database::Connection, image::Image};
use axum::{extract::Path, http::StatusCode};

pub async fn endpoint(
    Connection(mut conn): Connection,
    Path(name): Path<String>,
) -> Result<Image, (StatusCode, String)> {
    sqlx::query_as("select name, content from asset where name=? limit 1")
        .bind(name)
        .fetch_one(&mut conn)
        .await
        .map_err(not_found_or_500)
        .map(|(name, content)| Image(name, content))
}
