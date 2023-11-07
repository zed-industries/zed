use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn synthwave_84() -> UserThemeFamily {
    UserThemeFamily {
        name: "Synthwave 84".into(),
        author: "Robb Owen (robb0wen)".into(),
        themes: vec![UserTheme {
            name: "Synthwave 84".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    ..Default::default()
                },
            },
        }],
    }
}
