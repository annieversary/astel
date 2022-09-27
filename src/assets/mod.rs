use axum::{http::header, response::IntoResponse};

pub async fn main_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("main.css"),
    )
}
