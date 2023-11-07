use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn notctis() -> UserThemeFamily {
    UserThemeFamily {
        name: "Notctis".into(),
        author: "Liviu Schera (liviuschera)".into(),
        themes: vec![
            UserTheme {
                name: "Noctis Azureus".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Bordo".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctus Hibernus".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Lilac".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Lux".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Minimus".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Obscuro".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Sereno".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Uva".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Noctis Viola".into(),
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
