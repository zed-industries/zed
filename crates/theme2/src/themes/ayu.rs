use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn ayu() -> UserThemeFamily {
    UserThemeFamily {
        name: "Ayu".into(),
        author: "dempfi (Ike Ku)".into(),
        themes: vec![
            UserTheme {
                name: "Ayu Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Ayu Mirage".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Ayu Dark".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
        ],
    }
}
