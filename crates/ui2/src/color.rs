pub use crate::{old_theme, ButtonVariant, ElementExt, Theme};
use gpui2::{rgb, Hsla, WindowContext};
use strum::EnumIter;

#[derive(Clone, Copy)]
pub struct PlayerThemeColors {
    pub cursor: Hsla,
    pub selection: Hsla,
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

#[derive(Clone, Copy, Debug)]
pub struct SyntaxColor {
    pub comment: Hsla,
    pub string: Hsla,
    pub function: Hsla,
    pub keyword: Hsla,
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

    pub players: [PlayerThemeColors; 8],
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
