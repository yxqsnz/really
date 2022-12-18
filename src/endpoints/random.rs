use crate::utils::image::Image;
use crate::utils::{database::Connection, status_code::internal_error};
use axum::extract::Query;
use axum::http::StatusCode;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    nsfw: bool,
}

pub async fn endpoint(
    Connection(mut conn): Connection,
    Query(params): Query<Params>,
) -> Result<Image, (StatusCode, String)> {
    let (name, content): (String, Vec<u8>) =
        sqlx::query_as("select name, content from asset where nsfw=? order by random() LIMIT 1")
            .bind(params.nsfw)
            .fetch_one(&mut conn)
            .await
            .map_err(internal_error)?;

    Ok(Image(name, content))
}
