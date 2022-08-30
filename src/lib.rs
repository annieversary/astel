use axum::{body::Body, extract::RequestParts, response::IntoResponse, Router};
use serde::Serialize;

mod router_extension;
mod routes;
mod type_list;

pub use router_extension::RouterExt;
use routes::index;
use type_list::{Cons, HList, Nil};

pub struct Astel<L> {
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
