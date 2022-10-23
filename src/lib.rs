#[macro_use]
extern crate serde;

use std::borrow::Cow;

use axum::{
    http::request::Parts, middleware::from_fn, response::IntoResponse, routing::get, Extension,
    Router,
};
use config::{AstelConfig, AstelConfigInner};
use html::html_context_middleware;
use serde::{de::DeserializeOwned, Serialize};

mod assets;
mod config;
mod html;
mod router_extension;
mod routes;

mod table_serializer;

pub use router_extension::RouterExt;
use routes::{add_routes_for, home};

pub use conforming::ToForm;

#[derive(Default)]
pub struct Astel {
    router: Router,
    config: AstelConfigInner,
}

impl Astel {
    pub fn new(path: impl ToString) -> Self {
        Self {
            config: AstelConfigInner {
                path: path.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Astel {
    /// Builds a router containing routes for all registered resources
    pub fn build(self) -> Router {
        let config = AstelConfig::new(self.config);
        self.router
            .route("/", get(home::home))
            .route("/css/main.css", get(assets::main_css))
            // TODO add a fallback 404 page
            .layer(from_fn(html_context_middleware))
            .layer(Extension(config))
    }

    /// Registers a resource
    ///
    /// # Panics
    ///
    /// Panics if the `AstelResource::NAME` constant is already in use by another registered resource.
    pub fn register_resource<T>(mut self) -> Astel
    where
        T: Serialize + DeserializeOwned + AstelResource + ToForm + 'static + Send,
    {
        if self.config.names.contains(&T::NAME) {
            panic!("Name {} is already in use on a resource", T::NAME)
        }

        self.config.names.push(T::NAME);
        self.router = add_routes_for::<T>(T::NAME, self.router);
        self
    }

    pub fn register_dashboard(mut self, name: &str, dashboard: ()) -> Self {
        self
    }

    /// Will use a custom css that is available at the provided path
    ///
    /// `<link rel="stylesheet" href="{path}" type="text/css">`
    ///
    /// By default uses a simple css.
    pub fn with_css_path(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.config.css_path = Some(path.into());
        self
    }

    /// Sets the title in the sidebar, by default "Astel"
    pub fn with_title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.config.title = Some(title.into());
        self
    }

    /// Adds JS script at the bottom of the body. Multiple can be added.
    pub fn with_js(mut self, path: impl Into<Cow<'static, str>>) -> Self {
        self.config.js_paths.push(path.into());
        self
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

    const NAME: &'static str;

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
