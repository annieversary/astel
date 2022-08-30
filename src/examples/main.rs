use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/users", post(create_user))
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type Db = Arc<RwLock<HashMap<String, User>>>;

async fn create_user(Json(user): Json<User>, Extension(db): Extension<Db>) -> impl IntoResponse {
    db.write().unwrap().insert(user.username.clone(), user);

    StatusCode::CREATED
}

#[derive(Serialize)]
struct User {
    username: String,
    email: String,
}
