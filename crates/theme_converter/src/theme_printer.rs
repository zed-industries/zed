use std::fmt::{self, Debug};

use gpui2::{Hsla, Rgba};
use theme2::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub struct ThemePrinter(Theme);

impl ThemePrinter {
    pub fn new(theme: Theme) -> Self {
        Self(theme)
    }
}

struct HslaPrinter(Hsla);

impl Debug for HslaPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", IntoPrinter(&Rgba::from(self.0)))
    }
}

struct IntoPrinter<'a, D: Debug>(&'a D);

impl<'a, D: Debug> Debug for IntoPrinter<'a, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}.into()", self.0)
    }
}

impl Debug for ThemePrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Theme")
            .field("metadata", &ThemeMetadataPrinter(self.0.metadata.clone()))
            .field("transparent", &HslaPrinter(self.0.transparent))
            .field(
                "mac_os_traffic_light_red",
                &HslaPrinter(self.0.mac_os_traffic_light_red),
            )
            .field(
                "mac_os_traffic_light_yellow",
                &HslaPrinter(self.0.mac_os_traffic_light_yellow),
            )
            .field(
                "mac_os_traffic_light_green",
                &HslaPrinter(self.0.mac_os_traffic_light_green),
            )
            .field("border", &HslaPrinter(self.0.border))
            .field("border_variant", &HslaPrinter(self.0.border_variant))
            .field("border_focused", &HslaPrinter(self.0.border_focused))
            .field(
                "border_transparent",
                &HslaPrinter(self.0.border_transparent),
            )
            .field("elevated_surface", &HslaPrinter(self.0.elevated_surface))
            .field("surface", &HslaPrinter(self.0.surface))
            .field("background", &HslaPrinter(self.0.background))
            .field("filled_element", &HslaPrinter(self.0.filled_element))
            .field(
                "filled_element_hover",
                &HslaPrinter(self.0.filled_element_hover),
            )
            .field(
                "filled_element_active",
                &HslaPrinter(self.0.filled_element_active),
            )
            .field(
                "filled_element_selected",
                &HslaPrinter(self.0.filled_element_selected),
            )
            .field(
                "filled_element_disabled",
                &HslaPrinter(self.0.filled_element_disabled),
            )
            .field("ghost_element", &HslaPrinter(self.0.ghost_element))
            .field(
                "ghost_element_hover",
                &HslaPrinter(self.0.ghost_element_hover),
            )
            .field(
                "ghost_element_active",
                &HslaPrinter(self.0.ghost_element_active),
            )
            .field(
                "ghost_element_selected",
                &HslaPrinter(self.0.ghost_element_selected),
            )
            .field(
                "ghost_element_disabled",
                &HslaPrinter(self.0.ghost_element_disabled),
            )
            .field("text", &HslaPrinter(self.0.text))
            .field("text_muted", &HslaPrinter(self.0.text_muted))
            .field("text_placeholder", &HslaPrinter(self.0.text_placeholder))
            .field("text_disabled", &HslaPrinter(self.0.text_disabled))
            .field("text_accent", &HslaPrinter(self.0.text_accent))
            .field("icon_muted", &HslaPrinter(self.0.icon_muted))
            .field("syntax", &SyntaxThemePrinter(self.0.syntax.clone()))
            .field("status_bar", &HslaPrinter(self.0.status_bar))
            .field("title_bar", &HslaPrinter(self.0.title_bar))
            .field("toolbar", &HslaPrinter(self.0.toolbar))
            .field("tab_bar", &HslaPrinter(self.0.tab_bar))
            .field("editor", &HslaPrinter(self.0.editor))
            .field("editor_subheader", &HslaPrinter(self.0.editor_subheader))
            .field(
                "editor_active_line",
                &HslaPrinter(self.0.editor_active_line),
            )
            .field("terminal", &HslaPrinter(self.0.terminal))
            .field(
                "image_fallback_background",
                &HslaPrinter(self.0.image_fallback_background),
            )
            .field("git_created", &HslaPrinter(self.0.git_created))
            .field("git_modified", &HslaPrinter(self.0.git_modified))
            .field("git_deleted", &HslaPrinter(self.0.git_deleted))
            .field("git_conflict", &HslaPrinter(self.0.git_conflict))
            .field("git_ignored", &HslaPrinter(self.0.git_ignored))
            .field("git_renamed", &HslaPrinter(self.0.git_renamed))
            .field("players", &self.0.players.map(PlayerThemePrinter))
            .finish()
    }
}

pub struct ThemeMetadataPrinter(ThemeMetadata);

impl Debug for ThemeMetadataPrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeMetadata")
            .field("name", &IntoPrinter(&self.0.name))
            .field("is_light", &self.0.is_light)
            .finish()
    }
}

pub struct SyntaxThemePrinter(SyntaxTheme);

impl Debug for SyntaxThemePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxTheme")
            .field(
                "highlights",
                &VecPrinter(
                    &self
                        .0
                        .highlights
                        .iter()
                        .map(|(token, highlight)| {
                            (IntoPrinter(token), HslaPrinter(highlight.color.unwrap()))
                        })
                        .collect(),
                ),
            )
            .finish()
    }
}

pub struct VecPrinter<'a, T>(&'a Vec<T>);

impl<'a, T: Debug> Debug for VecPrinter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec!{:?}", &self.0)
    }
}

pub struct PlayerThemePrinter(PlayerTheme);

impl Debug for PlayerThemePrinter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerTheme")
            .field("cursor", &HslaPrinter(self.0.cursor))
            .field("selection", &HslaPrinter(self.0.selection))
            .finish()
    }
}
