use std::{ops::Deref, sync::Arc};

pub(crate) struct Config {
    pub path: String,
    pub names: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct AstelConfig(Arc<Config>);
impl AstelConfig {
    pub fn new(path: String, names: Vec<String>) -> Self {
        Self(Arc::new(Config { path, names }))
    }
}
impl Deref for AstelConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
