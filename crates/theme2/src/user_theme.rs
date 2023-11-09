use gpui::Hsla;
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
}

impl UserHighlightStyle {
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
    }
}
