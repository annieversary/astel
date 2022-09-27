use super::*;

pub(crate) async fn index_resource<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    ts: GetAll<T>,
    Extension(config): Extension<AstelConfig>,
    Extension(html): Extension<HtmlContextBuilder>,
) -> impl IntoResponse {
    let content = html! {
        h1 {
            (T::NAME)
        }

        a href={(config.path)"/"(T::NAME)"/new"} {
            "new"
        }

        (PreEscaped(to_table(&ts.0, &format!("{}/{}", config.path, T::NAME))));
    };

    html.build(content)
}
