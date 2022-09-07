#[macro_use]
extern crate serde;

use axum::{http::request::Parts, response::IntoResponse, routing::get, Extension, Router};
use config::AstelConfig;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod config;
mod router_extension;
mod routes;

mod table_serializer;
mod type_list;

pub use router_extension::RouterExt;
use routes::home;
use type_list::{Cons, HList, Nil};

pub use conforming::ToForm;

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
    /// registers a resource
    ///
    /// resources must implement `AstelResource`, `ToForm`, and Serialize/Deserialize
    pub fn register_resource<'de, T>(self, name: impl ToString) -> Astel<Cons<T, L>>
    where
        T: Serialize + Deserialize<'de> + AstelResource + ToForm + 'static + Send,
    {
        Astel {
            list: self.list.push::<T>(name.to_string()),
            path: self.path,
        }
    }

    pub(crate) fn names(&self) -> Vec<String> {
        self.list.names()
    }

    pub fn build(self) -> Router {
        let config = AstelConfig::new(self.path.clone(), self.names());
        self.list
            .router()
            .route("/", get(home::home))
            // TODO add a fallback 404 page
            .layer(Extension(config))
    }
}

/// trait for resources that want to be displayed in astel
#[axum::async_trait]
pub trait AstelResource: Sized {
    // TODO maybe we skip this and make our own error type?
    type Error: IntoResponse;

    /// Type of the db
    type Db: Send + Sync + Clone + 'static;

    /// Type of the model's id
    type ID: Serialize + DeserializeOwned + Send + Sync;

    /// Returns the ID for this model
    ///
    /// This should uniquely identify the model
    fn id(&self) -> &Self::ID;

    /// Extracts the db for this resource out of the Request
    ///
    /// By default uses the `Extension<Db>` extractor
    async fn get_db(parts: &mut Parts) -> Result<Self::Db, Self::Error> {
        Ok(parts.extensions.get::<Self::Db>().unwrap().clone())
    }

    /// Loads all the resources
    async fn load_all(db: &mut Self::Db) -> Result<Vec<Self>, Self::Error>;

    /// Loads the resource with the given id
    async fn load_one(db: &mut Self::Db, id: &Self::ID) -> Result<Option<Self>, Self::Error>;

    /// Creates a new model
    async fn new(db: &mut Self::Db, t: Self) -> Result<Self::ID, Self::Error>;

    /// Deletes the model with the provided id
    async fn delete(db: &mut Self::Db, id: &Self::ID) -> Result<(), Self::Error>;

    /// Edits the model with the provided id
    ///
    /// Should fail or do nothing if no model with the provided ID
    async fn edit(db: &mut Self::Db, id: &Self::ID, t: Self) -> Result<(), Self::Error>;
}
