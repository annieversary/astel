use axum::{Extension, Router};

use crate::Astel;

pub trait RouterExt {
    fn astel<DB: Send + Sync + Clone + 'static>(self, astel: Astel, db: DB) -> Self;
}

impl RouterExt for Router {
    fn astel<DB: Send + Sync + Clone + 'static>(self, astel: Astel, db: DB) -> Self {
        let path = astel.config.path.clone();
        self.nest(&path, astel.build().layer(Extension(db)))
    }
}
