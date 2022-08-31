use crate::{config::AstelConfig, table_serializer::to_table, AstelResource};
use axum::{
    body::Body,
    extract::{FromRequestParts, Query},
    http::{request::Parts, Request},
    response::{Html, IntoResponse},
    Extension,
};
use serde::{Deserialize, Serialize};

pub(crate) async fn view_resource<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    ts: GetAll<T>,
    request: Request<Body>,
) -> impl IntoResponse {
    Html(to_table(&ts.0, request.uri().path()))
}

pub(crate) async fn edit_resource_get<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    t: GetOne<T>,
    request: Request<Body>,
) -> impl IntoResponse {
    Html(to_table(&[t.0], request.uri().path()))
}

pub(crate) async fn edit_resource_post<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    _t: GetOne<T>,
    _request: Request<Body>,
) -> impl IntoResponse {
    todo!()
}

pub(crate) async fn delete_resource_get<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    _request: Request<Body>,
) -> impl IntoResponse {
    Html("<form method=\"POST\"><button type=\"submit\">delete</button></form>")
}

pub(crate) async fn delete_resource_post<
    'de,
    T: Send + AstelResource + Serialize + Deserialize<'de>,
>(
    q: Q<T>,
    req: Request<Body>,
) -> impl IntoResponse {
    let (mut parts, _) = req.into_parts();
    let db = <T as AstelResource>::get_db(&mut parts).await?;

    <T as AstelResource>::delete(db, &q.0.id).await
}

pub(crate) async fn index(Extension(config): Extension<AstelConfig>) -> impl IntoResponse {
    let path = &config.path;
    let names = config
        .names
        .iter()
        .map(|name| format!("<a href=\"{path}/{name}\">{name}</a>"))
        .collect::<String>();

    Html(names)
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Id<I> {
    pub id: I,
}
type Q<I> = Query<Id<<I as AstelResource>::ID>>;

pub(crate) struct GetAll<T>(Vec<T>);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for GetAll<T>
where
    T: AstelResource + Send,
    S: Send + Sync,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let db = <T as AstelResource>::get_db(parts).await?;

        <T as AstelResource>::load_all(db).await.map(Self)
    }
}

/// Extracts based on the `id` query param
pub(crate) struct GetOne<T>(T);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for GetOne<T>
where
    T: AstelResource + Send,
    S: Send + Sync,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let id = Q::<T>::from_request_parts(parts, state).await.unwrap().0.id;

        let db = <T as AstelResource>::get_db(parts).await?;

        <T as AstelResource>::load_one(db, &id)
            .await
            .transpose()
            .unwrap()
            .map(Self)
    }
}
