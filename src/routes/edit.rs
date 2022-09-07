use axum::{extract::Form, response::Redirect};
use conforming::ToForm;
use serde::de::DeserializeOwned;

use super::*;

pub(crate) async fn edit_resource_get<'de, T: AstelResource + ToForm>(
    t: GetOne<T>,
) -> impl IntoResponse {
    let html = <T as ToForm>::serialize(&t.0).unwrap().build();

    Html(html)
}

pub(crate) async fn edit_resource_post<T: AstelResource + DeserializeOwned>(
    conf: Extension<AstelConfig>,
    q: Q<T>,
    DbExtract(mut db): DbExtract<T>,
    Form(t): Form<T>,
) -> impl IntoResponse {
    T::edit(&mut db, &q.0.id, t).await?;

    let path = &conf.path;
    Ok::<_, T::Error>(Redirect::to(&format!("{path}/{}", T::NAME)))
}
