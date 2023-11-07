use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn rose_pine() -> UserThemeFamily {
    UserThemeFamily {
        name: "Rose Pine".into(),
        author: "Rosé Pine".into(),
        themes: vec![
            UserTheme {
                name: "Rose Pine".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Rose Moon".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Rose Pine Dawn".into(),
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
