use std::{ops::Deref, sync::Arc};

pub(crate) struct AstelConfigInner {
    pub path: String,
    pub names: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct AstelConfig(Arc<AstelConfigInner>);
impl AstelConfig {
    pub fn new(path: String, names: Vec<String>) -> Self {
        Self(Arc::new(AstelConfigInner { path, names }))
    }
}
impl Deref for AstelConfig {
    type Target = AstelConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) struct ResourceConfigInner {
    pub name: String,
}

#[derive(Clone)]
pub(crate) struct ResourceConfig(Arc<ResourceConfigInner>);
impl ResourceConfig {
    pub fn new(name: &str) -> Self {
        Self(Arc::new(ResourceConfigInner {
            name: name.to_string(),
        }))
    }
}
impl Deref for ResourceConfig {
    type Target = ResourceConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
