pub use gpui3::{
    div, Click, Element, Hover, IntoAnyElement, ParentElement, ScrollState, SharedString, Styled,
    ViewContext, WindowContext,
};

use crate::settings::user_settings;
pub use crate::{theme, ButtonVariant, ElementExt, Theme};

use gpui3::{hsla, rems, rgb, Hsla, Rems};
use strum::EnumIter;

#[derive(Default)]
pub struct SystemColor {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub state_hover_background: Hsla,
    pub state_active_background: Hsla,
}

impl SystemColor {
    pub fn new() -> SystemColor {
        SystemColor {
            transparent: hsla(0.0, 0.0, 0.0, 0.0),
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
            state_hover_background: hsla(0.0, 0.0, 0.0, 0.08),
            state_active_background: hsla(0.0, 0.0, 0.0, 0.16),
        }
    }
    pub fn color(&self) -> Hsla {
        self.transparent
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
            comment: theme.syntax.comment,
            string: theme.syntax.string,
            function: theme.syntax.function,
            keyword: theme.syntax.keyword,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeColor {
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    /// The background color of an elevated surface, like a modal, tooltip or toast.
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    /// Window background color
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
    pub icon_muted: Hsla,
    pub syntax: SyntaxColor,

    pub toolbar_background: Hsla,
    pub editor_background: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub image_fallback_background: Hsla,
}

impl ThemeColor {
    pub fn new(cx: &WindowContext) -> Self {
        let theme = theme(cx);
        let system_color = SystemColor::new();

        Self {
            border: theme.lowest.base.default.border,
            border_variant: theme.lowest.variant.default.border,
            border_focused: theme.lowest.accent.default.border,
            border_transparent: system_color.transparent,
            elevated_surface: theme.middle.base.default.background,
            surface: theme.middle.base.default.background,
            background: theme.lowest.base.default.background,
            filled_element: theme.lowest.base.default.background,
            filled_element_hover: theme.lowest.base.hovered.background,
            filled_element_active: theme.lowest.base.active.background,
            filled_element_selected: theme.lowest.accent.default.background,
            filled_element_disabled: system_color.transparent,
            ghost_element: system_color.transparent,
            ghost_element_hover: theme.lowest.base.default.background,
            ghost_element_active: theme.lowest.base.hovered.background,
            ghost_element_selected: theme.lowest.accent.default.background,
            ghost_element_disabled: system_color.transparent,
            text: theme.lowest.base.default.foreground,
            text_muted: theme.lowest.variant.default.foreground,
            /// TODO: map this to a real value
            text_placeholder: theme.lowest.negative.default.foreground,
            text_disabled: theme.lowest.base.disabled.foreground,
            icon_muted: theme.lowest.variant.default.foreground,
            syntax: SyntaxColor::new(cx),
            toolbar_background: theme.highest.base.default.background,
            editor_background: theme.highest.base.default.background,
            editor_subheader: theme.middle.base.default.background,
            editor_active_line: theme.highest.on.default.background,
            image_fallback_background: theme.lowest.base.default.background,
        }
    }
}

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
        let system_color = SystemColor::new();

        match self {
            Self::Default => theme
                .syntax
                .get("primary")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            Self::Comment => theme
                .syntax
                .get("comment")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            Self::String => theme
                .syntax
                .get("string")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            Self::Function => theme
                .syntax
                .get("function")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
            Self::Keyword => theme
                .syntax
                .get("keyword")
                .cloned()
                .unwrap_or_else(|| rgb::<Hsla>(0xff00ff)),
        }
    }
}

pub fn ui_size(size: f32) -> Rems {
    const UI_SCALE_RATIO: f32 = 0.875;

    let setting = user_settings();

    rems(*setting.ui_scale * UI_SCALE_RATIO * size)
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum FileSystemStatus {
    #[default]
    None,
    Conflict,
    Deleted,
}

impl FileSystemStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::None => "None".to_string(),
            Self::Conflict => "Conflict".to_string(),
            Self::Deleted => "Deleted".to_string(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum GitStatus {
    #[default]
    None,
    Created,
    Modified,
    Deleted,
    Conflict,
    Renamed,
}

impl GitStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::None => "None".to_string(),
            Self::Created => "Created".to_string(),
            Self::Modified => "Modified".to_string(),
            Self::Deleted => "Deleted".to_string(),
            Self::Conflict => "Conflict".to_string(),
            Self::Renamed => "Renamed".to_string(),
        }
    }

    pub fn hsla(&self, cx: &WindowContext) -> Hsla {
        let color = ThemeColor::new(cx);
        let system_color = SystemColor::new();

        match self {
            Self::None => system_color.transparent,
            Self::Created => theme.lowest.positive.default.foreground,
            Self::Modified => theme.lowest.warning.default.foreground,
            Self::Deleted => theme.lowest.negative.default.foreground,
            Self::Conflict => theme.lowest.warning.default.foreground,
            Self::Renamed => theme.lowest.accent.default.foreground,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DiagnosticStatus {
    #[default]
    None,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum IconSide {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum OrderMethod {
    #[default]
    Ascending,
    Descending,
    MostRecent,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum Shape {
    #[default]
    Circle,
    RoundedRectangle,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DisclosureControlVisibility {
    #[default]
    OnHover,
    Always,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumIter)]
pub enum DisclosureControlStyle {
    /// Shows the disclosure control only when hovered where possible.
    ///
    /// More compact, but not available everywhere.
    ChevronOnHover,
    /// Shows an icon where possible, otherwise shows a chevron.
    ///
    /// For example, in a file tree a folder or file icon is shown
    /// instead of a chevron
    Icon,
    /// Always shows a chevron.
    Chevron,
    /// Completely hides the disclosure control where possible.
    None,
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter, strum::Display)]
pub enum InteractionState {
    #[default]
    Enabled,
    Hovered,
    Active,
    Focused,
    Disabled,
}

impl InteractionState {
    pub fn if_enabled(&self, enabled: bool) -> Self {
        if enabled {
            *self
        } else {
            InteractionState::Disabled
        }
    }
}

#[derive(Default, PartialEq)]
pub enum SelectedState {
    #[default]
    Unselected,
    PartiallySelected,
    Selected,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Toggleable {
    Toggleable(ToggleState),
    #[default]
    NotToggleable,
}

impl Toggleable {
    pub fn is_toggled(&self) -> bool {
        match self {
            Self::Toggleable(ToggleState::Toggled) => true,
            _ => false,
        }
    }
}

impl From<ToggleState> for Toggleable {
    fn from(state: ToggleState) -> Self {
        Self::Toggleable(state)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ToggleState {
    /// The "on" state of a toggleable element.
    ///
    /// Example:
    ///     - A collasable list that is currently expanded
    ///     - A toggle button that is currently on.
    Toggled,
    /// The "off" state of a toggleable element.
    ///
    /// Example:
    ///     - A collasable list that is currently collapsed
    ///     - A toggle button that is currently off.
    #[default]
    NotToggled,
}

impl From<Toggleable> for ToggleState {
    fn from(toggleable: Toggleable) -> Self {
        match toggleable {
            Toggleable::Toggleable(state) => state,
            Toggleable::NotToggleable => ToggleState::NotToggled,
        }
    }
}

impl From<bool> for ToggleState {
    fn from(toggled: bool) -> Self {
        if toggled {
            ToggleState::Toggled
        } else {
            ToggleState::NotToggled
        }
    }
}
