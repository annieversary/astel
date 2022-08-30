use crate::AstelResource;
use axum::{
    body::Body,
    extract::{FromRequest, RequestParts},
    response::{Html, IntoResponse},
    routing::{get, MethodRouter},
    Json,
};
use serde::Serialize;

pub(crate) async fn view_resource<T: Serialize>(ts: Getter<T>) -> impl IntoResponse {
    // TODO display all the resources in a table

    Json(ts.0)
}

pub(crate) fn index(path: &str, names: Vec<&str>) -> MethodRouter {
    let names = names
        .into_iter()
        .map(|name| format!("<a href=\"{path}/{name}\">{name}</a>"))
        .collect::<String>();

    // TODO construct a fuller html

    let html = Html(names);

    get(|| async { html })
}

pub(crate) struct Getter<T>(Vec<T>);

#[axum::async_trait]
impl<T> FromRequest<Body> for Getter<T>
where
    T: AstelResource,
{
    type Rejection = <T as AstelResource>::Rejection;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        <T as AstelResource>::from_request(req).await.map(Self)
    }
}
