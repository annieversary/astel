use astel::{Astel, AstelResource};
use axum::{
    body::Body,
    extract::{Extension, RequestParts},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() {
    let db = Db::default();
    db.write().unwrap().insert(
        "cat".to_string(),
        User {
            username: "cat".to_string(),
            email: "cat@meow.nya".to_string(),
        },
    );

    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
        .nest(
            "/astel",
            Astel::new("/astel")
                .register_type::<User>("users")
                .register_type::<User>("other")
                .build(),
        )
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://127.0.0.1:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type Db = Arc<RwLock<HashMap<String, User>>>;

async fn index(Extension(db): Extension<Db>) -> impl IntoResponse {
    let count = db.read().unwrap().len();
    Html(format!(
        "<a href=\"/astel\">admin panel</a><p>there are {count} users</p>"
    ))
}

async fn create_user(Extension(db): Extension<Db>, Json(user): Json<User>) -> impl IntoResponse {
    db.write().unwrap().insert(user.username.clone(), user);

    StatusCode::CREATED
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    username: String,
    email: String,
}

#[axum::async_trait]
impl AstelResource for User {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Vec<Self>, Self::Rejection> {
        let db = req.extensions().get::<Db>().unwrap();

        Ok(db.read().unwrap().values().cloned().collect())
    }
}
