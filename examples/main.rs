use astel::{Astel, AstelResource, RouterExt, ToForm};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
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
        .route("/", get(index).post(create_user))
        .astel(Astel::new("/astel").register_resource::<User>())
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
        r#"<html>
    <head>
        <title>astel demo</title>
    </head>
    <body>
        <a href="/astel">admin panel</a>

        <h1>This is the main page</h1>
        <p>there are {count} users registered</p>
    </body>
</html>
"#
    ))
}

async fn create_user(Extension(db): Extension<Db>, Form(user): Form<User>) -> impl IntoResponse {
    db.write().unwrap().insert(user.username.clone(), user);

    StatusCode::CREATED
}

// astel stuff

#[derive(Serialize, Deserialize, Clone, ToForm)]
struct User {
    username: String,
    #[input(input_type = "email")]
    email: String,
}

#[axum::async_trait]
impl AstelResource for User {
    type Error = StatusCode;
    type Db = Db;
    type ID = String;

    const NAME: &'static str = "Users";

    fn id(&self) -> &Self::ID {
        &self.username
    }

    async fn load_all(db: &mut Self::Db) -> Result<Vec<Self>, Self::Error> {
        Ok(db.read().unwrap().values().cloned().collect())
    }

    async fn load_one(db: &mut Self::Db, id: &Self::ID) -> Result<Option<Self>, Self::Error> {
        Ok(db.read().unwrap().get(id).cloned())
    }

    async fn new(db: &mut Self::Db, t: Self) -> Result<Self::ID, Self::Error> {
        let mut db = db.write().unwrap();
        let id = t.username.clone();
        db.insert(id.clone(), t);
        Ok(id)
    }

    async fn delete(db: &mut Self::Db, id: &Self::ID) -> Result<(), Self::Error> {
        db.write().unwrap().remove(id);
        Ok(())
    }

    async fn edit(db: &mut Self::Db, id: &Self::ID, t: Self) -> Result<(), Self::Error> {
        db.write().unwrap().remove(id);

        Self::new(db, t).await.map(|_| ())
    }
}
