use super::*;

pub(crate) async fn view_resource<'de, T: AstelResource + Serialize + Deserialize<'de>>(
    ts: GetAll<T>,
    request: Request<Body>,
) -> impl IntoResponse {
    Html(to_table(&ts.0, request.uri().path()))
}
