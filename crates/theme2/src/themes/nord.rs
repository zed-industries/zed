use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn nord() -> UserThemeFamily {
    UserThemeFamily {
        name: "Nord".into(),
        author: "Sven Greb (svengreb)".into(),
        themes: vec![UserTheme {
            name: "Nord".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    ..Default::default()
                },
            },
        }],
    }
}
