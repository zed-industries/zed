use crate::theme;
pub use crate::{old_theme, ButtonVariant, ElementExt, Theme};
use gpui2::{hsla, rgb, Hsla, WindowContext};
use strum::EnumIter;

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl std::fmt::Debug for PlayerThemeColors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerThemeColors")
            .field("cursor", &self.cursor.to_rgb().to_hex())
            .field("selection", &self.selection.to_rgb().to_hex())
            .finish()
    }
}

impl PlayerThemeColors {
    pub fn new(cx: &WindowContext, ix: usize) -> Self {
        let theme = old_theme(cx);

        if ix < theme.players.len() {
            Self {
                cursor: theme.players[ix].cursor,
                selection: theme.players[ix].selection,
            }
        } else {
            Self {
                cursor: rgb::<Hsla>(0xff00ff),
                selection: rgb::<Hsla>(0xff00ff),
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct SyntaxColor {
    pub comment: Hsla,
    pub string: Hsla,
    pub function: Hsla,
    pub keyword: Hsla,
}

impl std::fmt::Debug for SyntaxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyntaxColor")
            .field("comment", &self.comment.to_rgb().to_hex())
            .field("string", &self.string.to_rgb().to_hex())
            .field("function", &self.function.to_rgb().to_hex())
            .field("keyword", &self.keyword.to_rgb().to_hex())
            .finish()
    }
}

impl SyntaxColor {
    pub fn new(cx: &WindowContext) -> Self {
        let theme = old_theme(cx);

        Self {
            comment: theme
                .syntax
                .get("comment")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            string: theme
                .syntax
                .get("string")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            function: theme
                .syntax
                .get("function")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            keyword: theme
                .syntax
                .get("keyword")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
        }
    }
}

/// ThemeColor is the primary interface for coloring elements in the UI.
///
/// It is a mapping layer between semantic theme colors and colors from the reference library.
///
/// While we are between zed and zed2 we use this to map semantic colors to the old theme.
#[derive(Clone, Copy)]
pub struct ThemeColor {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    /// The background color of an elevated surface, like a modal, tooltip or toast.
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    /// Window background color of the base app
    pub background: Hsla,
    /// Default background for elements like filled buttons,
    /// text fields, checkboxes, radio buttons, etc.
    /// - TODO: Map to step 3.
    pub filled_element: Hsla,
    /// The background color of a hovered element, like a button being hovered
    /// with a mouse, or hovered on a touch screen.
    /// - TODO: Map to step 4.
    pub filled_element_hover: Hsla,
    /// The background color of an active element, like a button being pressed,
    /// or tapped on a touch screen.
    /// - TODO: Map to step 5.
    pub filled_element_active: Hsla,
    /// The background color of a selected element, like a selected tab,
    /// a button toggled on, or a checkbox that is checked.
    pub filled_element_selected: Hsla,
    pub filled_element_disabled: Hsla,
    pub ghost_element: Hsla,
    /// The background color of a hovered element with no default background,
    /// like a ghost-style button or an interactable list item.
    /// - TODO: Map to step 3.
    pub ghost_element_hover: Hsla,
    /// - TODO: Map to step 4.
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon_muted: Hsla,
    pub syntax: SyntaxColor,

    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    /// The background of the editor
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub terminal: Hsla,
    pub image_fallback_background: Hsla,

    pub git_created: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub git_conflict: Hsla,
    pub git_ignored: Hsla,
    pub git_renamed: Hsla,

    pub player: [PlayerThemeColors; 8],
}

impl std::fmt::Debug for ThemeColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeColor")
            .field("transparent", &self.transparent.to_rgb().to_hex())
            .field(
                "mac_os_traffic_light_red",
                &self.mac_os_traffic_light_red.to_rgb().to_hex(),
            )
            .field(
                "mac_os_traffic_light_yellow",
                &self.mac_os_traffic_light_yellow.to_rgb().to_hex(),
            )
            .field(
                "mac_os_traffic_light_green",
                &self.mac_os_traffic_light_green.to_rgb().to_hex(),
            )
            .field("border", &self.border.to_rgb().to_hex())
            .field("border_variant", &self.border_variant.to_rgb().to_hex())
            .field("border_focused", &self.border_focused.to_rgb().to_hex())
            .field(
                "border_transparent",
                &self.border_transparent.to_rgb().to_hex(),
            )
            .field("elevated_surface", &self.elevated_surface.to_rgb().to_hex())
            .field("surface", &self.surface.to_rgb().to_hex())
            .field("background", &self.background.to_rgb().to_hex())
            .field("filled_element", &self.filled_element.to_rgb().to_hex())
            .field(
                "filled_element_hover",
                &self.filled_element_hover.to_rgb().to_hex(),
            )
            .field(
                "filled_element_active",
                &self.filled_element_active.to_rgb().to_hex(),
            )
            .field(
                "filled_element_selected",
                &self.filled_element_selected.to_rgb().to_hex(),
            )
            .field(
                "filled_element_disabled",
                &self.filled_element_disabled.to_rgb().to_hex(),
            )
            .field("ghost_element", &self.ghost_element.to_rgb().to_hex())
            .field(
                "ghost_element_hover",
                &self.ghost_element_hover.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_active",
                &self.ghost_element_active.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_selected",
                &self.ghost_element_selected.to_rgb().to_hex(),
            )
            .field(
                "ghost_element_disabled",
                &self.ghost_element_disabled.to_rgb().to_hex(),
            )
            .field("text", &self.text.to_rgb().to_hex())
            .field("text_muted", &self.text_muted.to_rgb().to_hex())
            .field("text_placeholder", &self.text_placeholder.to_rgb().to_hex())
            .field("text_disabled", &self.text_disabled.to_rgb().to_hex())
            .field("text_accent", &self.text_accent.to_rgb().to_hex())
            .field("icon_muted", &self.icon_muted.to_rgb().to_hex())
            .field("syntax", &self.syntax)
            .field("status_bar", &self.status_bar.to_rgb().to_hex())
            .field("title_bar", &self.title_bar.to_rgb().to_hex())
            .field("toolbar", &self.toolbar.to_rgb().to_hex())
            .field("tab_bar", &self.tab_bar.to_rgb().to_hex())
            .field("editor", &self.editor.to_rgb().to_hex())
            .field("editor_subheader", &self.editor_subheader.to_rgb().to_hex())
            .field(
                "editor_active_line",
                &self.editor_active_line.to_rgb().to_hex(),
            )
            .field("terminal", &self.terminal.to_rgb().to_hex())
            .field(
                "image_fallback_background",
                &self.image_fallback_background.to_rgb().to_hex(),
            )
            .field("git_created", &self.git_created.to_rgb().to_hex())
            .field("git_modified", &self.git_modified.to_rgb().to_hex())
            .field("git_deleted", &self.git_deleted.to_rgb().to_hex())
            .field("git_conflict", &self.git_conflict.to_rgb().to_hex())
            .field("git_ignored", &self.git_ignored.to_rgb().to_hex())
            .field("git_renamed", &self.git_renamed.to_rgb().to_hex())
            .field("player", &self.player)
            .finish()
    }
}

impl ThemeColor {
    pub fn new(cx: &WindowContext) -> Self {
        let theme2 = theme(cx);
        let transparent = hsla(0.0, 0.0, 0.0, 0.0);

        let players = [
            PlayerThemeColors::new(cx, 0),
            PlayerThemeColors::new(cx, 1),
            PlayerThemeColors::new(cx, 2),
            PlayerThemeColors::new(cx, 3),
            PlayerThemeColors::new(cx, 4),
            PlayerThemeColors::new(cx, 5),
            PlayerThemeColors::new(cx, 6),
            PlayerThemeColors::new(cx, 7),
        ];

        Self {
            transparent: theme2.transparent,
            mac_os_traffic_light_red: theme2.mac_os_traffic_light_red,
            mac_os_traffic_light_yellow: theme2.mac_os_traffic_light_yellow,
            mac_os_traffic_light_green: theme2.mac_os_traffic_light_green,
            border: theme2.border,
            border_variant: theme2.border_variant,
            border_focused: theme2.border_focused,
            border_transparent: theme2.border_transparent,
            elevated_surface: theme2.elevated_surface,
            surface: theme2.surface,
            background: theme2.background,
            filled_element: theme2.filled_element,
            filled_element_hover: theme2.filled_element_hover,
            filled_element_active: theme2.filled_element_active,
            filled_element_selected: theme2.filled_element_selected,
            filled_element_disabled: theme2.filled_element_disabled,
            ghost_element: theme2.ghost_element,
            ghost_element_hover: theme2.ghost_element_hover,
            ghost_element_active: theme2.ghost_element_active,
            ghost_element_selected: theme2.ghost_element_selected,
            ghost_element_disabled: theme2.ghost_element_disabled,
            text: theme2.text,
            text_muted: theme2.text_muted,
            /// TODO: map this to a real value
            text_placeholder: theme2.text_placeholder,
            text_disabled: theme2.text_disabled,
            text_accent: theme2.text_accent,
            icon_muted: theme2.icon_muted,
            syntax: SyntaxColor::new(cx),

            status_bar: theme2.status_bar,
            title_bar: theme2.title_bar,
            toolbar: theme2.toolbar,
            tab_bar: theme2.tab_bar,
            editor: theme2.editor,
            editor_subheader: theme2.editor_subheader,
            terminal: theme2.terminal,
            editor_active_line: theme2.editor_active_line,
            image_fallback_background: theme2.image_fallback_background,

            git_created: theme2.git_created,
            git_modified: theme2.git_modified,
            git_deleted: theme2.git_deleted,
            git_conflict: theme2.git_conflict,
            git_ignored: theme2.git_ignored,
            git_renamed: theme2.git_renamed,

            player: players,
        }
    }
}

/// Colors used exclusively for syntax highlighting.
///
/// For now we deserialize these from a theme.
/// These will be defined statically in the new theme.
#[derive(Default, PartialEq, EnumIter, Clone, Copy)]
pub enum HighlightColor {
    #[default]
    Default,
    Comment,
    String,
    Function,
    Keyword,
}

impl HighlightColor {
    pub fn hsla(&self, theme: &Theme) -> Hsla {
        match self {
            Self::Default => theme
                .syntax
                .get("primary")
                .cloned()
                .expect("Couldn't find `primary` in theme.syntax"),
            Self::Comment => theme
                .syntax
                .get("comment")
                .cloned()
                .expect("Couldn't find `comment` in theme.syntax"),
            Self::String => theme
                .syntax
                .get("string")
                .cloned()
                .expect("Couldn't find `string` in theme.syntax"),
            Self::Function => theme
                .syntax
                .get("function")
                .cloned()
                .expect("Couldn't find `function` in theme.syntax"),
            Self::Keyword => theme
                .syntax
                .get("keyword")
                .cloned()
                .expect("Couldn't find `keyword` in theme.syntax"),
        }
    }
}
