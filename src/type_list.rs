use crate::{routes::add_routes_for, AstelResource};
use axum::{body::Body, Router};
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

    fn names(&self) -> Vec<String>;

    fn router(&self) -> Router;
}
impl HList for Nil {
    fn names(&self) -> Vec<String> {
        vec![]
    }

    fn router(&self) -> Router {
        Router::<(), Body>::new()
    }
}
impl<'de, T, R> HList for Cons<T, R>
where
    R: HList,
    T: AstelResource + 'static + Send + Serialize + Deserialize<'de>,
{
    fn names(&self) -> Vec<String> {
        let mut n = self.rest.names();
        n.push(self.name.clone());
        n
    }

    fn router(&self) -> Router {
        add_routes_for::<T>(&self.name, self.rest.router())
    }
}
