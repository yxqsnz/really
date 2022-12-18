use crate::utils::{database::Connection, status_code::internal_error};
use axum::extract::Query;
use axum::http::header::CONTENT_DISPOSITION;
use axum::http::header::CONTENT_TYPE;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    nsfw: bool,
}

pub async fn endpoint(
    Connection(conn): Connection,
    Query(params): Query<Params>,
) -> Result<Response, (StatusCode, String)> {
    let mut conn = conn;

    let (name, content): (String, Vec<u8>) =
        sqlx::query_as("select name,content from asset where nsfw=? order by random() LIMIT 1")
            .bind(params.nsfw)
            .fetch_one(&mut conn)
            .await
            .map_err(internal_error)?;
    Ok((
        [
            (CONTENT_TYPE, HeaderValue::from_static("image/jpeg")),
            (
                CONTENT_DISPOSITION,
                HeaderValue::from_maybe_shared(format!("inline; filename={name}")).unwrap(),
            ),
        ],
        content,
    )
        .into_response())
}
