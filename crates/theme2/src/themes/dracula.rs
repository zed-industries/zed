use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn dracula() -> UserThemeFamily {
    UserThemeFamily {
        name: "Dracula".into(),
        author: "Zeno Rocha".into(),
        themes: vec![UserTheme {
            name: "Dracula".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    ..Default::default()
                },
            },
        }],
    }
}
