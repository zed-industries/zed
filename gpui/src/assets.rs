use anyhow::{anyhow, Result};
use std::borrow::Cow;

pub trait AssetSource: 'static {
    fn load(&self, path: &str) -> Result<Cow<[u8]>>;
}

impl AssetSource for () {
    fn load(&self, path: &str) -> Result<Cow<[u8]>> {
        Err(anyhow!(
            "get called on empty asset provider with \"{}\"",
            path
        ))
    }
}

pub struct AssetCache {
    source: Box<dyn AssetSource>,
}

impl AssetCache {
    pub fn new(source: impl AssetSource) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}
