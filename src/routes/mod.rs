use crate::{config::AstelConfig, table_serializer::to_table, AstelResource};
use axum::{
    body::Body,
    extract::{FromRequestParts, Query},
    http::{request::Parts, Request},
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use conforming::ToForm;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

mod delete;
mod edit;
pub(crate) mod index;
mod new;
mod view;

pub fn add_routes_for<T>(name: &str, r: Router) -> Router
where
    T: AstelResource + ToForm + 'static + Send + Serialize + DeserializeOwned,
{
    r.route(&format!("/{}", name), get(view::view_resource::<T>))
        .route(&format!("/{}/", name), get(view::view_resource::<T>))
        .route(
            &format!("/{}/new", name),
            get(new::new_resource_get::<T>).post(new::new_resource_post::<T>),
        )
        .route(
            &format!("/{}/new/", name),
            get(new::new_resource_get::<T>).post(new::new_resource_post::<T>),
        )
        .route(
            &format!("/{}/edit", name),
            get(edit::edit_resource_get::<T>).post(edit::edit_resource_post::<T>),
        )
        .route(
            &format!("/{}/edit/", name),
            get(edit::edit_resource_get::<T>).post(edit::edit_resource_post::<T>),
        )
        .route(
            &format!("/{}/delete", name),
            get(delete::delete_resource_get::<T>).post(delete::delete_resource_post::<T>),
        )
        .route(
            &format!("/{}/delete/", name),
            get(delete::delete_resource_get::<T>).post(delete::delete_resource_post::<T>),
        )
}

// extractors for routes

#[derive(Deserialize, Serialize)]
pub(crate) struct Id<I> {
    pub id: I,
}
type Q<I> = Query<Id<<I as AstelResource>::ID>>;

pub(crate) struct GetAll<T>(Vec<T>);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for GetAll<T>
where
    T: AstelResource + Send,
    S: Send + Sync,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut db = <T as AstelResource>::get_db(parts).await?;

        <T as AstelResource>::load_all(&mut db).await.map(Self)
    }
}

/// Extracts based on the `id` query param
pub(crate) struct GetOne<T>(T);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for GetOne<T>
where
    T: AstelResource + Send,
    S: Send + Sync,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let id = Q::<T>::from_request_parts(parts, state).await.unwrap().0.id;

        let mut db = <T as AstelResource>::get_db(parts).await?;

        <T as AstelResource>::load_one(&mut db, &id)
            .await
            .transpose()
            .unwrap()
            .map(Self)
    }
}

/// Extracts based on the `id` query param
pub(crate) struct DbExtract<T: AstelResource>(T::Db);

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for DbExtract<T>
where
    T: AstelResource + Send,
    S: Send + Sync,
{
    // TODO write wrapper for this error
    type Rejection = <T as AstelResource>::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        <T as AstelResource>::get_db(parts).await.map(Self)
    }
}
