use gpui2::{HighlightStyle, Hsla, SharedString};
use std::sync::Arc;

use crate::{colors::ThemeStyle, Appearance, ColorScales};

pub struct ThemeFamily {
    pub(crate) id: String,
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeVariant>,
    pub scales: ColorScales,
}

impl ThemeFamily {}

pub struct ThemeVariant {
    pub(crate) id: String,
    pub name: String,
    pub appearance: Appearance,
    pub styles: ThemeStyle,
}

#[derive(Clone)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
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
