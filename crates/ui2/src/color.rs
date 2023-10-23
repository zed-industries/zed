pub use crate::{theme, ButtonVariant, ElementExt, Theme};
use gpui2::{hsla, rgb, Hsla, WindowContext};
use strum::EnumIter;

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
}

impl PlayerThemeColors {
    pub fn new(cx: &WindowContext, ix: usize) -> Self {
        let theme = theme(cx);

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

impl SyntaxColor {
    pub fn new(cx: &WindowContext) -> Self {
        let theme = theme(cx);

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

impl ThemeColor {
    pub fn new(cx: &WindowContext) -> Self {
        let theme = theme(cx);
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
            transparent,
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
            border: theme.lowest.base.default.border,
            border_variant: theme.lowest.variant.default.border,
            border_focused: theme.lowest.accent.default.border,
            border_transparent: transparent,
            elevated_surface: theme.lowest.base.default.background,
            surface: theme.middle.base.default.background,
            background: theme.lowest.base.default.background,
            filled_element: theme.lowest.base.default.background,
            filled_element_hover: theme.lowest.base.hovered.background,
            filled_element_active: theme.lowest.base.active.background,
            filled_element_selected: theme.lowest.accent.default.background,
            filled_element_disabled: transparent,
            ghost_element: transparent,
            ghost_element_hover: theme.lowest.base.default.background,
            ghost_element_active: theme.lowest.base.hovered.background,
            ghost_element_selected: theme.lowest.accent.default.background,
            ghost_element_disabled: transparent,
            text: theme.lowest.base.default.foreground,
            text_muted: theme.lowest.variant.default.foreground,
            /// TODO: map this to a real value
            text_placeholder: theme.lowest.negative.default.foreground,
            text_disabled: theme.lowest.base.disabled.foreground,
            text_accent: theme.lowest.accent.default.foreground,
            icon_muted: theme.lowest.variant.default.foreground,
            syntax: SyntaxColor::new(cx),

            status_bar: theme.lowest.base.default.background,
            title_bar: theme.lowest.base.default.background,
            toolbar: theme.highest.base.default.background,
            tab_bar: theme.middle.base.default.background,
            editor: theme.highest.base.default.background,
            editor_subheader: theme.middle.base.default.background,
            terminal: theme.highest.base.default.background,
            editor_active_line: theme.highest.on.default.background,
            image_fallback_background: theme.lowest.base.default.background,

            git_created: theme.lowest.positive.default.foreground,
            git_modified: theme.lowest.accent.default.foreground,
            git_deleted: theme.lowest.negative.default.foreground,
            git_conflict: theme.lowest.warning.default.foreground,
            git_ignored: theme.lowest.base.disabled.foreground,
            git_renamed: theme.lowest.warning.default.foreground,

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
