use axum::{extract::Form, response::Redirect};
use conforming::ToForm;
use serde::de::DeserializeOwned;

use super::*;

pub(crate) async fn new_resource_get<T: AstelResource + ToForm>() -> impl IntoResponse {
    let html = T::to_form().build();

    Html(html)
}

pub(crate) async fn new_resource_post<T: AstelResource + DeserializeOwned + Send>(
    conf: Extension<AstelConfig>,
    DbExtract(mut db): DbExtract<T>,
    Form(t): Form<T>,
) -> impl IntoResponse {
    T::new(&mut db, t).await?;

    let path = &conf.path;
    Ok::<_, T::Error>(Redirect::to(&format!("{path}/{}", T::NAME)))
}
