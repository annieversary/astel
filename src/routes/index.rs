use super::*;

pub(crate) async fn index_resource<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    ts: GetAll<T>,
    conf: Extension<AstelConfig>,
    resconf: Extension<ResourceConfig>,
) -> impl IntoResponse {
    Html(to_table(&ts.0, &format!("{}/{}", conf.path, resconf.name)))
}
