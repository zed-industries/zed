use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn andromeda() -> UserThemeFamily {
    UserThemeFamily {
        name: "Andromeda".into(),
        author: "Eliver Lara (EliverLara)".into(),
        themes: vec![
            UserTheme {
                name: "Andromeda".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x1b1d23ff).into()),
                        border_variant: Some(rgba(0x1b1d23ff).into()),
                        border_focused: Some(rgba(0x1b1d23ff).into()),
                        border_selected: Some(rgba(0x1b1d23ff).into()),
                        border_transparent: Some(rgba(0x1b1d23ff).into()),
                        border_disabled: Some(rgba(0x1b1d23ff).into()),
                        elevated_surface_background: Some(rgba(0x23262eff).into()),
                        surface_background: Some(rgba(0x23262eff).into()),
                        background: Some(rgba(0x23262eff).into()),
                        element_background: Some(rgba(0x00e8c5cc).into()),
                        text: Some(rgba(0xd4cdd8ff).into()),
                        tab_inactive_background: Some(rgba(0x23262eff).into()),
                        tab_active_background: Some(rgba(0x23262eff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xee5d42ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x95e072ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x7bb7ffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xff00a9ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x00e8c6ff).into()),
                        terminal_ansi_red: Some(rgba(0xee5d42ff).into()),
                        terminal_ansi_green: Some(rgba(0x95e072ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_blue: Some(rgba(0x7bb7ffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xff00a9ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x00e8c6ff).into()),
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Andromeda Bordered".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x1b1d23ff).into()),
                        border_variant: Some(rgba(0x1b1d23ff).into()),
                        border_focused: Some(rgba(0x1b1d23ff).into()),
                        border_selected: Some(rgba(0x1b1d23ff).into()),
                        border_transparent: Some(rgba(0x1b1d23ff).into()),
                        border_disabled: Some(rgba(0x1b1d23ff).into()),
                        elevated_surface_background: Some(rgba(0x23262eff).into()),
                        surface_background: Some(rgba(0x23262eff).into()),
                        background: Some(rgba(0x262933ff).into()),
                        element_background: Some(rgba(0x00e8c5cc).into()),
                        text: Some(rgba(0xd4cdd8ff).into()),
                        tab_inactive_background: Some(rgba(0x23262eff).into()),
                        tab_active_background: Some(rgba(0x262933ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xee5d42ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x95e072ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x7bb7ffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xff00a9ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x00e8c6ff).into()),
                        terminal_ansi_red: Some(rgba(0xee5d42ff).into()),
                        terminal_ansi_green: Some(rgba(0x95e072ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffe66dff).into()),
                        terminal_ansi_blue: Some(rgba(0x7bb7ffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xff00a9ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x00e8c6ff).into()),
                        ..Default::default()
                    },
                },
            },
        ],
    }
}
