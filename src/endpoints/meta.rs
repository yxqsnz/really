use crate::utils::{database::Connection, status_code::internal_error};
use axum::{http::StatusCode, Json};

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Category {
    pub items: u32,
    pub name: String,
    pub is_nsfw: bool,
}

#[derive(Serialize, Default)]
pub struct Response {
    pub total: u32,
    pub categories: Vec<Category>,
}

pub async fn endpoint(
    Connection(mut conn): Connection,
) -> Result<Json<Response>, (StatusCode, String)> {
    let categories: Vec<(String,)> = sqlx::query_as("select distinct category from asset")
        .fetch_all(&mut conn)
        .await
        .map_err(internal_error)?;

    let mut response = Response::default();

    for (name,) in categories {
        let (is_nsfw, items): (bool, u32) =
            sqlx::query_as("select nsfw, count(hash) from asset where category=?")
                .bind(&name)
                .fetch_one(&mut conn)
                .await
                .map_err(internal_error)?;

        response.categories.push(Category {
            is_nsfw,
            items,
            name,
        });
    }

    response.total = response.categories.iter().fold(0, |acc, k| acc + k.items);

    Ok(Json(response))
}
