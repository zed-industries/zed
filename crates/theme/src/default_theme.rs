use crate::{
    one_themes::{one_dark, one_family},
    Theme, ThemeFamily,
};

impl Default for ThemeFamily {
    fn default() -> Self {
        one_family()
    }
}

impl Default for Theme {
    fn default() -> Self {
        one_dark()
    }
}
