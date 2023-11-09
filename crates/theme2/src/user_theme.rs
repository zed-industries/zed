use crate::{Appearance, ThemeColors, ThemeColorsRefinement};
use refineable::Refineable;
use serde::Deserialize;

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
#[refineable(Deserialize)]
pub struct UserThemeStyles {
    #[refineable]
    pub colors: ThemeColors,
}
