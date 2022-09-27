use axum::{extract::Form, response::Redirect};
use conforming::ToForm;
use serde::de::DeserializeOwned;

use super::*;

pub(crate) async fn new_resource_get<T: AstelResource + ToForm>(
    Extension(_config): Extension<AstelConfig>,
    Extension(html): Extension<HtmlContextBuilder>,
) -> impl IntoResponse {
    let form = T::to_form().with_submit("Create").build();

    let content = html! {
        h1 {
            (T::NAME)
            " - Create"
        }

        (PreEscaped(form));
    };

    html.build(content)
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
