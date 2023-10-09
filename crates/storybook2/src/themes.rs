mod rose_pine;

pub use rose_pine::*;

use gpui3::serde_json;
use serde::Deserialize;
use ui::Theme;

use crate::assets::Assets;

#[derive(Deserialize)]
struct LegacyTheme {
    pub base_theme: serde_json::Value,
}

/// Loads the [`Theme`] with the given name.
pub fn load_theme(name: String) -> Theme {
    let theme_contents = Assets::get(&format!("themes/{name}.json"))
        .unwrap_or_else(|| panic!("failed to load theme: {name}.json"));

    let legacy_theme: LegacyTheme =
        serde_json::from_str(std::str::from_utf8(&theme_contents.data).unwrap()).unwrap();

    let theme: Theme = serde_json::from_value(legacy_theme.base_theme.clone()).unwrap();

    theme
}
