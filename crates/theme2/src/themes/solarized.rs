use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn solarized() -> UserThemeFamily {
    UserThemeFamily {
        name: "Solarized".into(),
        author: "Ethan Schoonover (altercation)".into(),
        themes: vec![
            UserTheme {
                name: "Solarized Dark".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Solarized Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
        ],
    }
}
