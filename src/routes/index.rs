use super::*;

pub(crate) async fn index(Extension(config): Extension<AstelConfig>) -> impl IntoResponse {
    let path = &config.path;
    let names = config
        .names
        .iter()
        .map(|name| format!("<a href=\"{path}/{name}\">{name}</a>"))
        .collect::<String>();

    Html(names)
}
