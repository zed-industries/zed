use anyhow::{anyhow, Result};
use gpui::AssetSource;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
        Self::get(path).ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }
}
