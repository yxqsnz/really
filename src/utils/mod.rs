use axum::http::StatusCode;

pub mod database;
pub mod image;
pub mod status_code;

pub type RESTResult<T> = Result<T, (StatusCode, String)>;
