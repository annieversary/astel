use crate::{routes::add_routes_for, AstelResource};
use axum::{body::Body, Router};
use conforming::ToForm;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

// based on https://docs.rs/hlist/0.1.2/hlist/

#[derive(Clone, Copy, Debug, Default)]
pub struct Nil;
#[derive(Clone, Debug, Default)]
pub struct Cons<T, R> {
    t: PhantomData<T>,
    rest: R,
}

pub trait HList: Sized {
    fn push<T>(self) -> Cons<T, Self> {
        Cons {
            t: PhantomData::<T>,
            rest: self,
        }
    }

    fn names(&self) -> Vec<&'static str>;

    fn router(&self) -> Router;
}
impl HList for Nil {
    fn names(&self) -> Vec<&'static str> {
        vec![]
    }

    fn router(&self) -> Router {
        Router::<(), Body>::new()
    }
}
impl<T, R> HList for Cons<T, R>
where
    R: HList,
    T: AstelResource + ToForm + 'static + Send + Serialize + DeserializeOwned,
{
    fn names(&self) -> Vec<&'static str> {
        let mut n = self.rest.names();
        n.push(T::NAME);
        n
    }

    fn router(&self) -> Router {
        add_routes_for::<T>(T::NAME, self.rest.router())
    }
}
