use std::borrow::Cow;

use anyhow::{anyhow, Result};
use gpui3::{AssetSource, SharedString};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "fonts/**/*"]
#[include = "icons/**/*"]
#[include = "themes/**/*"]
#[include = "sounds/**/*"]
#[include = "*.md"]
#[exclude = "*.DS_Store"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &SharedString) -> Result<Cow<[u8]>> {
        Self::get(path.as_ref())
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &SharedString) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path.as_ref()))
            .map(SharedString::from)
            .collect())
    }
}
