use super::*;

pub(crate) async fn edit_resource_get<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    t: GetOne<T>,
    request: Request<Body>,
) -> impl IntoResponse {
    Html(to_table(&[t.0], request.uri().path()))
}

pub(crate) async fn edit_resource_post<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    _t: GetOne<T>,
    _request: Request<Body>,
) -> impl IntoResponse {
    todo!()
}
