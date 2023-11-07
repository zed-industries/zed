use std::fmt::{self, Debug};

use gpui::{Hsla, Rgba};
use theme::{
    Appearance, GitStatusColors, PlayerColor, PlayerColors, StatusColors, SyntaxTheme,
    SystemColors, ThemeColors, ThemeFamily, ThemeStyles, ThemeVariant,
};

struct RawSyntaxPrinter<'a>(&'a str);

impl<'a> Debug for RawSyntaxPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

pub struct VecPrinter<'a, T>(&'a Vec<T>);

impl<'a, T: Debug> Debug for VecPrinter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "vec!{:?}", &self.0)
    }
}

pub struct ThemeFamilyPrinter(ThemeFamily);

impl ThemeFamilyPrinter {
    pub fn new(theme_family: ThemeFamily) -> Self {
        Self(theme_family)
    }
}

impl Debug for ThemeFamilyPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThemeFamily")
            .field("id", &IntoPrinter(&self.0.id))
            .field("name", &IntoPrinter(&self.0.name))
            .field("author", &IntoPrinter(&self.0.author))
            .field(
                "themes",
                &VecPrinter(
                    &self
                        .0
                        .themes
                        .iter()
                        .map(|theme| ThemeVariantPrinter(theme))
                        .collect(),
                ),
            )
            .field("scales", &RawSyntaxPrinter("default_color_scales()"))
            .finish()
    }
}

pub struct ThemeVariantPrinter<'a>(&'a ThemeVariant);

impl<'a> Debug for ThemeVariantPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThemeVariant")
            .field("id", &IntoPrinter(&self.0.id))
            .field("name", &IntoPrinter(&self.0.name))
            .field("appearance", &AppearancePrinter(self.0.appearance))
            .field("styles", &ThemeStylesPrinter(&self.0.styles))
            .finish()
    }
}

pub struct AppearancePrinter(Appearance);

impl Debug for AppearancePrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Appearance::{:?}", self.0)
    }
}

pub struct ThemeStylesPrinter<'a>(&'a ThemeStyles);

impl<'a> Debug for ThemeStylesPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThemeStyles")
            .field("system", &SystemColorsPrinter(&self.0.system))
            .field("colors", &ThemeColorsPrinter(&self.0.colors))
            .field("status", &StatusColorsPrinter(&self.0.status))
            .field("git", &GitStatusColorsPrinter(&self.0.git))
            .field("player", &PlayerColorsPrinter(&self.0.player))
            .field("syntax", &SyntaxThemePrinter(&self.0.syntax))
            .finish()
    }
}

pub struct SystemColorsPrinter<'a>(&'a SystemColors);

impl<'a> Debug for SystemColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SystemColors")
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
            .finish()
    }
}

pub struct ThemeColorsPrinter<'a>(&'a ThemeColors);

impl<'a> Debug for ThemeColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThemeColors")
            .field("border", &HslaPrinter(self.0.border))
            .field("border_variant", &HslaPrinter(self.0.border_variant))
            .field("border_focused", &HslaPrinter(self.0.border_focused))
            .field("border_selected", &HslaPrinter(self.0.border_selected))
            .field(
                "border_transparent",
                &HslaPrinter(self.0.border_transparent),
            )
            .field("border_disabled", &HslaPrinter(self.0.border_disabled))
            .field(
                "elevated_surface_background",
                &HslaPrinter(self.0.elevated_surface_background),
            )
            .field(
                "surface_background",
                &HslaPrinter(self.0.surface_background),
            )
            .field("background", &HslaPrinter(self.0.background))
            .field(
                "element_background",
                &HslaPrinter(self.0.element_background),
            )
            .field("element_hover", &HslaPrinter(self.0.element_hover))
            .field("element_active", &HslaPrinter(self.0.element_active))
            .field("element_selected", &HslaPrinter(self.0.element_selected))
            .field("element_disabled", &HslaPrinter(self.0.element_disabled))
            .field(
                "element_placeholder",
                &HslaPrinter(self.0.element_placeholder),
            )
            .field(
                "element_drop_target",
                &HslaPrinter(self.0.element_drop_target),
            )
            .field(
                "ghost_element_background",
                &HslaPrinter(self.0.ghost_element_background),
            )
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
            .field("icon", &HslaPrinter(self.0.icon))
            .field("icon_muted", &HslaPrinter(self.0.icon_muted))
            .field("icon_disabled", &HslaPrinter(self.0.icon_disabled))
            .field("icon_placeholder", &HslaPrinter(self.0.icon_placeholder))
            .field("icon_accent", &HslaPrinter(self.0.icon_accent))
            .field(
                "status_bar_background",
                &HslaPrinter(self.0.status_bar_background),
            )
            .field(
                "title_bar_background",
                &HslaPrinter(self.0.title_bar_background),
            )
            .field(
                "toolbar_background",
                &HslaPrinter(self.0.toolbar_background),
            )
            .field(
                "tab_bar_background",
                &HslaPrinter(self.0.tab_bar_background),
            )
            .field(
                "tab_inactive_background",
                &HslaPrinter(self.0.tab_inactive_background),
            )
            .field(
                "tab_active_background",
                &HslaPrinter(self.0.tab_active_background),
            )
            .field("editor_background", &HslaPrinter(self.0.editor_background))
            .field(
                "editor_gutter_background",
                &HslaPrinter(self.0.editor_gutter_background),
            )
            .field(
                "editor_subheader_background",
                &HslaPrinter(self.0.editor_subheader_background),
            )
            .field(
                "editor_active_line_background",
                &HslaPrinter(self.0.editor_active_line_background),
            )
            .field(
                "editor_highlighted_line_background",
                &HslaPrinter(self.0.editor_highlighted_line_background),
            )
            .field(
                "editor_line_number",
                &HslaPrinter(self.0.editor_line_number),
            )
            .field(
                "editor_active_line_number",
                &HslaPrinter(self.0.editor_active_line_number),
            )
            .field("editor_invisible", &HslaPrinter(self.0.editor_invisible))
            .field("editor_wrap_guide", &HslaPrinter(self.0.editor_wrap_guide))
            .field(
                "editor_active_wrap_guide",
                &HslaPrinter(self.0.editor_active_wrap_guide),
            )
            .field(
                "editor_document_highlight_read_background",
                &HslaPrinter(self.0.editor_document_highlight_read_background),
            )
            .field(
                "editor_document_highlight_write_background",
                &HslaPrinter(self.0.editor_document_highlight_write_background),
            )
            .field(
                "terminal_background",
                &HslaPrinter(self.0.terminal_background),
            )
            .field(
                "terminal_ansi_bright_black",
                &HslaPrinter(self.0.terminal_ansi_bright_black),
            )
            .field(
                "terminal_ansi_bright_red",
                &HslaPrinter(self.0.terminal_ansi_bright_red),
            )
            .field(
                "terminal_ansi_bright_green",
                &HslaPrinter(self.0.terminal_ansi_bright_green),
            )
            .field(
                "terminal_ansi_bright_yellow",
                &HslaPrinter(self.0.terminal_ansi_bright_yellow),
            )
            .field(
                "terminal_ansi_bright_blue",
                &HslaPrinter(self.0.terminal_ansi_bright_blue),
            )
            .field(
                "terminal_ansi_bright_magenta",
                &HslaPrinter(self.0.terminal_ansi_bright_magenta),
            )
            .field(
                "terminal_ansi_bright_cyan",
                &HslaPrinter(self.0.terminal_ansi_bright_cyan),
            )
            .field(
                "terminal_ansi_bright_white",
                &HslaPrinter(self.0.terminal_ansi_bright_white),
            )
            .field(
                "terminal_ansi_black",
                &HslaPrinter(self.0.terminal_ansi_black),
            )
            .field("terminal_ansi_red", &HslaPrinter(self.0.terminal_ansi_red))
            .field(
                "terminal_ansi_green",
                &HslaPrinter(self.0.terminal_ansi_green),
            )
            .field(
                "terminal_ansi_yellow",
                &HslaPrinter(self.0.terminal_ansi_yellow),
            )
            .field(
                "terminal_ansi_blue",
                &HslaPrinter(self.0.terminal_ansi_blue),
            )
            .field(
                "terminal_ansi_magenta",
                &HslaPrinter(self.0.terminal_ansi_magenta),
            )
            .field(
                "terminal_ansi_cyan",
                &HslaPrinter(self.0.terminal_ansi_cyan),
            )
            .field(
                "terminal_ansi_white",
                &HslaPrinter(self.0.terminal_ansi_white),
            )
            .finish()
    }
}

pub struct StatusColorsPrinter<'a>(&'a StatusColors);

impl<'a> Debug for StatusColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusColors")
            .field("conflict", &HslaPrinter(self.0.conflict))
            .field("created", &HslaPrinter(self.0.created))
            .field("deleted", &HslaPrinter(self.0.deleted))
            .field("error", &HslaPrinter(self.0.error))
            .field("hidden", &HslaPrinter(self.0.hidden))
            .field("ignored", &HslaPrinter(self.0.ignored))
            .field("info", &HslaPrinter(self.0.info))
            .field("modified", &HslaPrinter(self.0.modified))
            .field("renamed", &HslaPrinter(self.0.renamed))
            .field("success", &HslaPrinter(self.0.success))
            .field("warning", &HslaPrinter(self.0.warning))
            .finish()
    }
}

pub struct GitStatusColorsPrinter<'a>(&'a GitStatusColors);

impl<'a> Debug for GitStatusColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitStatusColors")
            .field("conflict", &HslaPrinter(self.0.conflict))
            .field("created", &HslaPrinter(self.0.created))
            .field("deleted", &HslaPrinter(self.0.deleted))
            .field("ignored", &HslaPrinter(self.0.ignored))
            .field("modified", &HslaPrinter(self.0.modified))
            .field("renamed", &HslaPrinter(self.0.renamed))
            .finish()
    }
}

pub struct PlayerColorsPrinter<'a>(&'a PlayerColors);

impl<'a> Debug for PlayerColorsPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PlayerColors")
            .field(&VecPrinter(
                &self
                    .0
                     .0
                    .iter()
                    .map(|player_color| PlayerColorPrinter(player_color))
                    .collect(),
            ))
            .finish()
    }
}

pub struct PlayerColorPrinter<'a>(&'a PlayerColor);

impl<'a> Debug for PlayerColorPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlayerColor")
            .field("cursor", &HslaPrinter(self.0.cursor))
            .field("background", &HslaPrinter(self.0.background))
            .field("selection", &HslaPrinter(self.0.selection))
            .finish()
    }
}

pub struct SyntaxThemePrinter<'a>(&'a SyntaxTheme);

impl<'a> Debug for SyntaxThemePrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
