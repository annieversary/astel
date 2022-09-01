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
    req: Request<Body>,
) -> impl IntoResponse {
    let (mut parts, _) = req.into_parts();
    let db = <T as AstelResource>::get_db(&mut parts).await?;

    <T as AstelResource>::delete(db, &q.0.id).await
}
