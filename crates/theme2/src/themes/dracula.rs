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
                    border: Some(rgba(0xbd93f9ff).into()),
                    border_variant: Some(rgba(0xbd93f9ff).into()),
                    border_focused: Some(rgba(0xbd93f9ff).into()),
                    border_selected: Some(rgba(0xbd93f9ff).into()),
                    border_transparent: Some(rgba(0xbd93f9ff).into()),
                    border_disabled: Some(rgba(0xbd93f9ff).into()),
                    elevated_surface_background: Some(rgba(0x282a35ff).into()),
                    surface_background: Some(rgba(0x282a35ff).into()),
                    background: Some(rgba(0x282a35ff).into()),
                    element_background: Some(rgba(0x44475aff).into()),
                    text: Some(rgba(0xf8f8f2ff).into()),
                    tab_inactive_background: Some(rgba(0x21222cff).into()),
                    tab_active_background: Some(rgba(0x282a35ff).into()),
                    terminal_background: Some(rgba(0x282a35ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x6272a4ff).into()),
                    terminal_ansi_bright_red: Some(rgba(0xff6d6dff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x69ff94ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xffffa5ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0xd6abfeff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xff92dfff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0xa3fefeff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                    terminal_ansi_black: Some(rgba(0x21222cff).into()),
                    terminal_ansi_red: Some(rgba(0xff5555ff).into()),
                    terminal_ansi_green: Some(rgba(0x50fa7bff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf1fa8cff).into()),
                    terminal_ansi_blue: Some(rgba(0xbd93f9ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xff79c6ff).into()),
                    terminal_ansi_cyan: Some(rgba(0x8be9fdff).into()),
                    terminal_ansi_white: Some(rgba(0xf8f8f2ff).into()),
                    ..Default::default()
                },
            },
        }],
    }
}
