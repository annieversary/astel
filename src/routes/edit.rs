use axum::extract::Form;
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
    q: Q<T>,
    DbExtract(mut db): DbExtract<T>,
    Form(t): Form<T>,
) -> impl IntoResponse {
    T::edit(&mut db, &q.0.id, t).await
}
