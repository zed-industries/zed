use gpui2::{AppContext, HighlightStyle, Hsla, SharedString};
use settings2::Settings;
use std::sync::Arc;

use crate::{colors::ThemeStyle, Appearance, ColorScales};

pub struct ThemeFamily {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeVariant>,
    pub scales: ColorScales,
}

impl ThemeFamily {}

pub struct ThemeVariant {
    id: String,
    pub name: String,
    pub appearance: Appearance,
    pub styles: ThemeStyle,
}

#[derive(Clone)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    // TOOD: Get this working with `#[cfg(test)]`. Why isn't it?
    pub fn new_test(colors: impl IntoIterator<Item = (&'static str, Hsla)>) -> Self {
        SyntaxTheme {
            highlights: colors
                .into_iter()
                .map(|(key, color)| {
                    (
                        key.to_owned(),
                        HighlightStyle {
                            color: Some(color),
                            ..Default::default()
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> HighlightStyle {
        self.highlights
            .iter()
            .find_map(|entry| if entry.0 == name { Some(entry.1) } else { None })
            .unwrap_or_default()
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color.unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub struct PlayerTheme {
    pub cursor: Hsla,
    pub selection: Hsla,
}

#[derive(Clone)]
pub struct ThemeMetadata {
    pub name: SharedString,
    pub is_light: bool,
}

pub struct Editor {
    pub syntax: Arc<SyntaxTheme>,
}
