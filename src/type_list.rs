use crate::{routes::view_resource, AstelResource};
use axum::{body::Body, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

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
impl<'de, T, R> HList for Cons<T, R>
where
    R: HList,
    T: AstelResource + 'static + Send + Serialize + Deserialize<'de>,
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
