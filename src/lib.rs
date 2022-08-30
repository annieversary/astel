#[macro_use]
extern crate serde;

use axum::{body::Body, extract::RequestParts, response::IntoResponse, Router};
use serde::Serialize;

mod router_extension;
mod routes;
mod table_serializer;
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
    type Error: IntoResponse;

    type Db: Send + Sync + Clone + 'static;

    type ID;

    /// Returns the ID for this model
    ///
    /// This should uniquely identify the model
    fn id(&self) -> &Self::ID;

    /// Extracts the db for this resource out of the Request
    ///
    /// By default uses the `Extension<Db>` extractor
    async fn get_db(req: &mut RequestParts<Body>) -> Result<&mut Self::Db, Self::Error> {
        Ok(req.extensions_mut().get_mut::<Self::Db>().unwrap())
    }

    /// get all the resources out of the request body
    async fn load(db: &mut Self::Db) -> Result<Vec<Self>, Self::Error>;

    /// deletes the model with this id
    async fn delete(db: &mut Self::Db, id: &Self::ID) -> Result<(), Self::Error>;
}
