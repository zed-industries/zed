use gpui::{FontWeight, Hsla};
use refineable::Refineable;
use serde::Deserialize;

use crate::{Appearance, StatusColors, StatusColorsRefinement, ThemeColors, ThemeColorsRefinement};

#[derive(Deserialize)]
pub struct UserThemeFamily {
    pub name: String,
    pub author: String,
    pub themes: Vec<UserTheme>,
}

#[derive(Deserialize)]
pub struct UserTheme {
    pub name: String,
    pub appearance: Appearance,
    pub styles: UserThemeStylesRefinement,
}

#[derive(Refineable, Clone)]
#[refineable(deserialize)]
pub struct UserThemeStyles {
    #[refineable]
    pub colors: ThemeColors,
    #[refineable]
    pub status: StatusColors,
    pub syntax: UserSyntaxTheme,
}

#[derive(Clone, Default, Deserialize)]
pub struct UserSyntaxTheme {
    pub highlights: Vec<(String, UserHighlightStyle)>,
}

#[derive(Clone, Default, Deserialize)]
pub struct UserHighlightStyle {
    pub color: Option<Hsla>,
    pub font_style: Option<UserFontStyle>,
    pub font_weight: Option<UserFontWeight>,
}

#[derive(Clone, Default, Deserialize)]
pub struct UserFontWeight(pub f32);

impl UserFontWeight {
    /// Thin weight (100), the thinnest value.
    pub const THIN: Self = Self(FontWeight::THIN.0);
    /// Extra light weight (200).
    pub const EXTRA_LIGHT: Self = Self(FontWeight::EXTRA_LIGHT.0);
    /// Light weight (300).
    pub const LIGHT: Self = Self(FontWeight::LIGHT.0);
    /// Normal (400).
    pub const NORMAL: Self = Self(FontWeight::NORMAL.0);
    /// Medium weight (500, higher than normal).
    pub const MEDIUM: Self = Self(FontWeight::MEDIUM.0);
    /// Semibold weight (600).
    pub const SEMIBOLD: Self = Self(FontWeight::SEMIBOLD.0);
    /// Bold weight (700).
    pub const BOLD: Self = Self(FontWeight::BOLD.0);
    /// Extra-bold weight (800).
    pub const EXTRA_BOLD: Self = Self(FontWeight::EXTRA_BOLD.0);
    /// Black weight (900), the thickest value.
    pub const BLACK: Self = Self(FontWeight::BLACK.0);
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum UserFontStyle {
    Normal,
    Italic,
    Oblique,
}

impl UserHighlightStyle {
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
    }
}
