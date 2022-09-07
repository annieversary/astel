use super::*;

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
    DbExtract(mut db): DbExtract<T>,
) -> impl IntoResponse {
    <T as AstelResource>::delete(&mut db, &q.0.id).await
}
