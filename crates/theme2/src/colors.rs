use gpui2::Hsla;
use indexmap::IndexMap;
use refineable::Refineable;

use crate::{generate_struct_with_overrides, SyntaxStyles};

pub struct SystemColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

pub struct PlayerColors(pub Vec<PlayerColor>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusColorName {
    Conflict,
    Created,
    Deleted,
    Error,
    Hidden,
    Ignored,
    Info,
    Modified,
    Renamed,
    Success,
    Warning,
}

pub struct StatusColors(pub IndexMap<StatusColorName, Hsla>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitStatusColorName {
    Conflict,
    Created,
    Deleted,
    Ignored,
    Modified,
    Renamed,
}

pub struct GitStatusColors(pub IndexMap<GitStatusColorName, Hsla>);

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct ThemeColors {
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    pub background: Hsla,
    pub element: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,
    pub element_placeholder: Hsla,
    pub ghost_element: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,
    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
}

generate_struct_with_overrides! {
    ThemeStyle,
    ThemeStyleOverrides,
    system: SystemColors,
    colors: ThemeColors,
    status: StatusColors,
    git: GitStatusColors,
    player: PlayerColors,
    syntax: SyntaxStyles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_a_single_theme_color() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui2::rgb(0xff00ff);

        assert_ne!(colors.text, magenta);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
    }

    #[test]
    fn override_multiple_theme_colors() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui2::rgb(0xff00ff);
        let green: Hsla = gpui2::rgb(0x00ff00);

        assert_ne!(colors.text, magenta);
        assert_ne!(colors.background, green);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            background: Some(green),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
        assert_eq!(colors.background, green);
    }
}
