use crate::utils::{database::Connection, image::Image, status_code::internal_error};
use axum::http::StatusCode;

use axum::extract::Path;

pub async fn endpoint(
    Connection(mut conn): Connection,
    Path((kind, category)): Path<(String, String)>,
) -> Result<Image, (StatusCode, String)> {
    let is_nsfw = match kind.as_str() {
        "sfw" => false,
        "nsfw" => true,
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid category".to_string())),
    };

    sqlx::query_as(
        "select name, content from asset where nsfw=? and category=? order by random() limit 1",
    )
    .bind(is_nsfw)
    .bind(category)
    .fetch_one(&mut conn)
    .await
    .map_err(internal_error)
    .map(|(name, content)| Image(name, content))
}
