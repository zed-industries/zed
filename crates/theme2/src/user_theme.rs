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
}
