mod rose_pine;

pub use rose_pine::*;

use anyhow::{Context, Result};
use gpui3::serde_json;
use serde::Deserialize;
use ui::Theme;

use crate::assets::Assets;

#[derive(Deserialize)]
struct LegacyTheme {
    pub base_theme: serde_json::Value,
}

/// Loads the [`Theme`] with the given name.
pub fn load_theme(name: String) -> Result<Theme> {
    let theme_contents = Assets::get(&format!("themes/{name}.json"))
        .with_context(|| format!("theme file not found: '{name}'"))?;

    let legacy_theme: LegacyTheme =
        serde_json::from_str(std::str::from_utf8(&theme_contents.data)?)
            .context("failed to parse legacy theme")?;

    let theme: Theme = serde_json::from_value(legacy_theme.base_theme.clone())
        .context("failed to parse `base_theme`")?;

    Ok(theme)
}
