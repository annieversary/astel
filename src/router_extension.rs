use axum::Router;

use crate::{type_list::HList, Astel};

pub trait RouterExt {
    fn astel<T: HList>(self, astel: Astel<T>) -> Self;
}

impl RouterExt for Router {
    fn astel<T: HList>(self, astel: Astel<T>) -> Self {
        let path = astel.path.clone();
        self.nest(&path, astel.build())
    }
}
