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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeStyleName {
    Border,
    BorderVariant,
    BorderFocused,
    BorderTransparent,
    ElevatedSurface,
    Surface,
    Background,
    Element,
    ElementHover,
    ElementActive,
    ElementSelected,
    ElementDisabled,
    ElementPlaceholder,
    GhostElement,
    GhostElementHover,
    GhostElementActive,
    GhostElementSelected,
    GhostElementDisabled,
    Text,
    TextMuted,
    TextPlaceholder,
    TextDisabled,
    TextAccent,
    Icon,
    IconMuted,
    IconDisabled,
    IconPlaceholder,
    IconAccent,
    StatusBar,
    TitleBar,
    Toolbar,
    TabBar,
    Editor,
    EditorSubheader,
    EditorActiveLine,
}

pub struct ThemeColors(pub IndexMap<ThemeStyleName, Hsla>);

impl ThemeColors {
    pub fn text_muted(&self) -> Hsla {
        self.0
            .get(&ThemeStyleName::TextMuted)
            .cloned()
            .unwrap_or_default()
    }
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

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct ThemeColors2 {
    pub text_muted: Hsla,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refineable_colors_with_none() {
        let mut base_colors = ThemeColors2 {
            text_muted: gpui2::red(),
        };

        let overrides = ThemeColors2Refinement { text_muted: None };

        base_colors.refine(&overrides);

        assert_eq!(base_colors.text_muted, gpui2::red())
    }

    #[test]
    fn test_refineable_colors_with_some() {
        let mut base_colors = ThemeColors2 {
            text_muted: gpui2::red(),
        };

        let overrides = ThemeColors2Refinement {
            text_muted: Some(gpui2::white()),
        };

        base_colors.refine(&overrides);

        assert_eq!(base_colors.text_muted, gpui2::white())
    }
}
