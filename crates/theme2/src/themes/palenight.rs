use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn palenight() -> UserThemeFamily {
    UserThemeFamily {
        name: "Palenight".into(),
        author: "Olaolu Olawuyi (whizkydee)".into(),
        themes: vec![
            UserTheme {
                name: "Palenight".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Palenight Operator".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Palenight (Mild Contrast)".into(),
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
