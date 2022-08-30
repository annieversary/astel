use std::marker::PhantomData;

use axum::{
    body::Body,
    extract::{FromRequest, RequestParts},
    response::{Html, IntoResponse},
    routing::{get, MethodRouter},
    Json, Router,
};
use serde::Serialize;

// TODO add an extension method on Router that nests an Astel on `path`

pub struct Astel<L: HList> {
    list: L,
    path: String,
}

impl Astel<Nil> {
    pub fn new(path: impl ToString) -> Self {
        Self {
            list: Nil,
            path: path.to_string(),
        }
    }
}

impl<L: HList> Astel<L> {
    pub fn register_type<T>(self, name: impl ToString) -> Astel<Cons<T, L>>
    where
        T: Serialize + AstelResource + 'static + Send,
    {
        Astel {
            list: self.list.push::<T>(name.to_string()),
            path: self.path,
        }
    }

    pub fn names(&self) -> Vec<&str> {
        self.list.names()
    }

    pub fn build(self) -> Router {
        self.list
            .router()
            .route("/", index(&self.path, self.names()))
    }
}

#[axum::async_trait]
pub trait AstelResource: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    async fn from_request(req: &mut RequestParts<Body>) -> Result<Vec<Self>, Self::Rejection>;
}

// based on https://docs.rs/hlist/0.1.2/hlist/

#[derive(Clone, Copy, Debug, Default)]
pub struct Nil;
#[derive(Clone, Debug, Default)]
pub struct Cons<T, R> {
    t: PhantomData<T>,
    name: String,
    rest: R,
}

pub trait HList: Sized {
    fn push<T>(self, name: String) -> Cons<T, Self> {
        Cons {
            t: PhantomData::<T>,
            name,
            rest: self,
        }
    }

    fn names(&self) -> Vec<&str>;

    fn router(&self) -> Router;
}
impl HList for Nil {
    fn names(&self) -> Vec<&str> {
        vec![]
    }

    fn router(&self) -> Router {
        Router::<Body>::new()
    }
}
impl<T, R> HList for Cons<T, R>
where
    R: HList,
    T: AstelResource + 'static + Send + Serialize,
{
    fn names(&self) -> Vec<&str> {
        let mut n = self.rest.names();
        n.push(&self.name);
        n
    }

    fn router(&self) -> Router {
        self.rest
            .router()
            .route(&format!("/{}", self.name), get(view_resource::<T>))
    }
}

async fn view_resource<T: Serialize>(ts: Getter<T>) -> impl IntoResponse {
    // TODO display all the resources in a table

    Json(ts.0)
}

fn index(path: &str, names: Vec<&str>) -> MethodRouter {
    let names = names
        .into_iter()
        .map(|name| format!("<a href=\"{path}/{name}\">{name}</a>"))
        .collect::<String>();

    // TODO construct a fuller html

    let html = Html(names);

    get(|| async { html })
}

struct Getter<T>(Vec<T>);

#[axum::async_trait]
impl<T> FromRequest<Body> for Getter<T>
where
    T: AstelResource,
{
    type Rejection = <T as AstelResource>::Rejection;

    async fn from_request(req: &mut RequestParts<Body>) -> Result<Self, Self::Rejection> {
        <T as AstelResource>::from_request(req).await.map(Self)
    }
}
