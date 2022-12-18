use axum::{
    http::{
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
        HeaderValue,
    },
    response::{IntoResponse, Response},
};

pub struct Image(pub String, pub Vec<u8>);

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        (
            [
                (CONTENT_TYPE, HeaderValue::from_static("image/jpeg")),
                (
                    CONTENT_DISPOSITION,
                    HeaderValue::from_maybe_shared(format!("inline; filename={}", self.0)).unwrap(),
                ),
            ],
            self.1,
        )
            .into_response()
    }
}
