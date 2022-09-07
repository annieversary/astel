use super::*;

pub(crate) async fn index_resource<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    ts: GetAll<T>,
    conf: Extension<AstelConfig>,
) -> impl IntoResponse {
    Html(to_table(&ts.0, &format!("{}/{}", conf.path, T::NAME)))
}
