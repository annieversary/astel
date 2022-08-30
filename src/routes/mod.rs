use crate::{table_serializer::to_table, AstelResource};
use axum::{
    body::Body,
    extract::{FromRequest, RequestParts},
    response::{Html, IntoResponse},
    routing::{get, MethodRouter},
};
use serde::{Deserialize, Serialize};

pub(crate) async fn view_resource<'de, T: Serialize + Deserialize<'de>>(
    ts: Getter<T>,
) -> impl IntoResponse {
    Html(to_table(&ts.0))
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
    T: AstelResource + Send,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        let db = <T as AstelResource>::get_db(req).await?;

        <T as AstelResource>::load(db).await.map(Self)
    }
}
