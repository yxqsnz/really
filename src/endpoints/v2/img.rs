use crate::utils::{database::Connection, status_code::not_found_or_500, RESTResult};

use axum::{
    extract::{Path, Query},
    Json,
};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Serialize)]
pub struct Response {
    pub url: String,
    pub provider: String,
    pub nsfw: bool,
}

#[derive(Deserialize)]
pub struct Params {
    nsfw: Option<bool>,
}

pub async fn get(
    Connection(mut conn): Connection,
    Path((category,)): Path<(String,)>,
    Query(params): Query<Params>,
) -> RESTResult<Json<Response>> {
    let (name, provider, nsfw): (String, String, bool) = sqlx::query_as(
        "select name, provider, nsfw from asset where nsfw=? and category=? order by random() limit 1",
    )
    .bind(params.nsfw.unwrap_or(false))
    .bind(category)
    .fetch_one(&mut conn)
    .await
    .map_err(not_found_or_500)?;

    Ok(Json(Response {
        url: format!("{}/api/v1/asset/{name}", var("Api.Url").unwrap_or_default()),
        provider,
        nsfw,
    }))
}
