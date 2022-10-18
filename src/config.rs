use std::{borrow::Cow, ops::Deref, sync::Arc};

#[derive(Default)]
pub(crate) struct AstelConfigInner {
    pub path: String,
    pub names: Vec<&'static str>,

    pub css_path: Option<Cow<'static, str>>,
    pub title: Option<Cow<'static, str>>,
    pub js_paths: Vec<Cow<'static, str>>,
}

#[derive(Clone)]
pub(crate) struct AstelConfig(Arc<AstelConfigInner>);
impl AstelConfig {
    pub fn new(inner: AstelConfigInner) -> Self {
        Self(Arc::new(inner))
    }
}
impl Deref for AstelConfig {
    type Target = AstelConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
