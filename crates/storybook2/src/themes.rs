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

pub fn load_theme(override_theme_name: Option<String>) -> Theme {
    let theme = if let Some(theme) = override_theme_name {
        let theme_contents = Assets::get(&format!("themes/{theme}.json"))
            .unwrap_or_else(|| panic!("failed to load theme: {theme}.json"));

        let legacy_theme: LegacyTheme =
            serde_json::from_str(std::str::from_utf8(&theme_contents.data).unwrap()).unwrap();

        let new_theme: Theme = serde_json::from_value(legacy_theme.base_theme.clone()).unwrap();

        new_theme
    } else {
        rose_pine_dawn()
    };

    theme
}
