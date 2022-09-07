use std::{ops::Deref, sync::Arc};

pub(crate) struct AstelConfigInner {
    pub path: String,
    pub names: Vec<&'static str>,
}

#[derive(Clone)]
pub(crate) struct AstelConfig(Arc<AstelConfigInner>);
impl AstelConfig {
    pub fn new(path: String, names: Vec<&'static str>) -> Self {
        Self(Arc::new(AstelConfigInner { path, names }))
    }
}
impl Deref for AstelConfig {
    type Target = AstelConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
