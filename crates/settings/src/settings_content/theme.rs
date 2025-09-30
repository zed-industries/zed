use collections::{HashMap, IndexMap};
use gpui::{FontFallbacks, FontFeatures, FontStyle, FontWeight};
use schemars::{JsonSchema, JsonSchema_repr};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use settings_macros::MergeFrom;
use std::sync::Arc;

use serde_with::skip_serializing_none;

/// Settings for rendering text in UI and text buffers.

#[skip_serializing_none]
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, JsonSchema, MergeFrom)]
pub struct ThemeSettingsContent {
    /// The default font size for text in the UI.
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    /// The name of a font to use for rendering in the UI.
    #[serde(default)]
    pub ui_font_family: Option<FontFamilyName>,
    /// The font fallbacks to use for rendering in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_fallbacks")]
    #[schemars(extend("uniqueItems" = true))]
    pub ui_font_fallbacks: Option<Vec<FontFamilyName>>,
    /// The OpenType features to enable for text in the UI.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub ui_font_features: Option<FontFeatures>,
    /// The weight of the UI font in CSS units from 100 to 900.
    #[serde(default)]
    pub ui_font_weight: Option<f32>,
    /// The name of a font to use for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_family: Option<FontFamilyName>,
    /// The font fallbacks to use for rendering in text buffers.
    #[serde(default)]
    #[schemars(extend("uniqueItems" = true))]
    pub buffer_font_fallbacks: Option<Vec<FontFamilyName>>,
    /// The default font size for rendering in text buffers.
    #[serde(default)]
    pub buffer_font_size: Option<f32>,
    /// The weight of the editor font in CSS units from 100 to 900.
    #[serde(default)]
    pub buffer_font_weight: Option<f32>,
    /// The buffer's line height.
    #[serde(default)]
    pub buffer_line_height: Option<BufferLineHeight>,
    /// The OpenType features to enable for rendering in text buffers.
    #[serde(default)]
    #[schemars(default = "default_font_features")]
    pub buffer_font_features: Option<FontFeatures>,
    /// The font size for the agent panel. Falls back to the UI font size if unset.
    #[serde(default)]
    pub agent_font_size: Option<f32>,
    /// The name of the Zed theme to use.
    #[serde(default)]
    pub theme: Option<ThemeSelection>,
    /// The name of the icon theme to use.
    #[serde(default)]
    pub icon_theme: Option<IconThemeSelection>,

    /// UNSTABLE: Expect many elements to be broken.
    ///
    // Controls the density of the UI.
    #[serde(rename = "unstable.ui_density", default)]
    pub ui_density: Option<UiDensity>,

    /// How much to fade out unused code.
    #[serde(default)]
    pub unnecessary_code_fade: Option<f32>,

    /// EXPERIMENTAL: Overrides for the current theme.
    ///
    /// These values will override the ones on the current theme specified in `theme`.
    #[serde(rename = "experimental.theme_overrides", default)]
    pub experimental_theme_overrides: Option<ThemeStyleContent>,

    /// Overrides per theme
    ///
    /// These values will override the ones on the specified theme
    #[serde(default)]
    pub theme_overrides: HashMap<String, ThemeStyleContent>,
}

fn default_font_features() -> Option<FontFeatures> {
    Some(FontFeatures::default())
}

fn default_font_fallbacks() -> Option<FontFallbacks> {
    Some(FontFallbacks::default())
}

/// Represents the selection of a theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(untagged)]
pub enum ThemeSelection {
    /// A static theme selection, represented by a single theme name.
    Static(ThemeName),
    /// A dynamic theme selection, which can change based the [ThemeMode].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The theme to use for light mode.
        light: ThemeName,
        /// The theme to use for dark mode.
        dark: ThemeName,
    },
}

/// Represents the selection of an icon theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(untagged)]
pub enum IconThemeSelection {
    /// A static icon theme selection, represented by a single icon theme name.
    Static(IconThemeName),
    /// A dynamic icon theme selection, which can change based on the [`ThemeMode`].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The icon theme to use for light mode.
        light: IconThemeName,
        /// The icon theme to use for dark mode.
        dark: IconThemeName,
    },
}

// TODO: Rename ThemeMode -> ThemeAppearanceMode
/// The mode use to select a theme.
///
/// `Light` and `Dark` will select their respective themes.
///
/// `System` will select the theme based on the system's appearance.
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, MergeFrom,
)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    /// Use the specified `light` theme.
    Light,

    /// Use the specified `dark` theme.
    Dark,

    /// Use the theme based on the system's appearance.
    #[default]
    System,
}

/// Specifies the density of the UI.
/// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    MergeFrom,
)]
#[serde(rename_all = "snake_case")]
pub enum UiDensity {
    /// A denser UI with tighter spacing and smaller elements.
    #[serde(alias = "compact")]
    Compact,
    #[default]
    #[serde(alias = "default")]
    /// The default UI density.
    Default,
    #[serde(alias = "comfortable")]
    /// A looser UI with more spacing and larger elements.
    Comfortable,
}

impl UiDensity {
    /// The spacing ratio of a given density.
    /// TODO: Standardize usage throughout the app or remove
    pub fn spacing_ratio(self) -> f32 {
        match self {
            UiDensity::Compact => 0.75,
            UiDensity::Default => 1.0,
            UiDensity::Comfortable => 1.25,
        }
    }
}

/// Font family name.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct FontFamilyName(pub Arc<str>);

/// The buffer's line height.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema, MergeFrom, Default)]
#[serde(rename_all = "snake_case")]
pub enum BufferLineHeight {
    /// A less dense line height.
    #[default]
    Comfortable,
    /// The default line height.
    Standard,
    /// A custom line height, where 1.0 is the font's height. Must be at least 1.0.
    Custom(#[serde(deserialize_with = "deserialize_line_height")] f32),
}

fn deserialize_line_height<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = f32::deserialize(deserializer)?;
    if value < 1.0 {
        return Err(serde::de::Error::custom(
            "buffer_line_height.custom must be at least 1.0",
        ));
    }

    Ok(value)
}

/// The content of a serialized theme.
#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(default)]
pub struct ThemeStyleContent {
    #[serde(default, rename = "background.appearance")]
    pub window_background_appearance: Option<WindowBackgroundContent>,

    #[serde(default)]
    pub accents: Vec<AccentContent>,

    #[serde(flatten, default)]
    pub colors: ThemeColorsContent,

    #[serde(flatten, default)]
    pub status: StatusColorsContent,

    #[serde(default)]
    pub players: Vec<PlayerColorContent>,

    /// The styles for syntax nodes.
    #[serde(default)]
    pub syntax: IndexMap<String, HighlightStyleContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct AccentContent(pub Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
pub struct PlayerColorContent {
    pub cursor: Option<String>,
    pub background: Option<String>,
    pub selection: Option<String>,
}

/// Theme name.
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct ThemeName(pub Arc<str>);

/// Icon Theme Name
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq, Eq)]
#[serde(transparent)]
pub struct IconThemeName(pub Arc<str>);

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(default)]
pub struct ThemeColorsContent {
    /// Border color. Used for most borders, is usually a high contrast color.
    #[serde(rename = "border")]
    pub border: Option<String>,

    /// Border color. Used for deemphasized borders, like a visual divider between two sections
    #[serde(rename = "border.variant")]
    pub border_variant: Option<String>,

    /// Border color. Used for focused elements, like keyboard focused list item.
    #[serde(rename = "border.focused")]
    pub border_focused: Option<String>,

    /// Border color. Used for selected elements, like an active search filter or selected checkbox.
    #[serde(rename = "border.selected")]
    pub border_selected: Option<String>,

    /// Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change.
    #[serde(rename = "border.transparent")]
    pub border_transparent: Option<String>,

    /// Border color. Used for disabled elements, like a disabled input or button.
    #[serde(rename = "border.disabled")]
    pub border_disabled: Option<String>,

    /// Background color. Used for elevated surfaces, like a context menu, popup, or dialog.
    #[serde(rename = "elevated_surface.background")]
    pub elevated_surface_background: Option<String>,

    /// Background Color. Used for grounded surfaces like a panel or tab.
    #[serde(rename = "surface.background")]
    pub surface_background: Option<String>,

    /// Background Color. Used for the app background and blank panels or windows.
    #[serde(rename = "background")]
    pub background: Option<String>,

    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    #[serde(rename = "element.background")]
    pub element_background: Option<String>,

    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    #[serde(rename = "element.hover")]
    pub element_hover: Option<String>,

    /// Background Color. Used for the active state of an element that should have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressed.
    #[serde(rename = "element.active")]
    pub element_active: Option<String>,

    /// Background Color. Used for the selected state of an element that should have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    #[serde(rename = "element.selected")]
    pub element_selected: Option<String>,

    /// Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    #[serde(rename = "element.disabled")]
    pub element_disabled: Option<String>,

    /// Background Color. Used for the background of selections in a UI element.
    #[serde(rename = "element.selection_background")]
    pub element_selection_background: Option<String>,

    /// Background Color. Used for the area that shows where a dragged element will be dropped.
    #[serde(rename = "drop_target.background")]
    pub drop_target_background: Option<String>,

    /// Border Color. Used for the border that shows where a dragged element will be dropped.
    #[serde(rename = "drop_target.border")]
    pub drop_target_border: Option<String>,

    /// Used for the background of a ghost element that should have the same background as the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have a different background than the surface it's on, use `element_background`.
    #[serde(rename = "ghost_element.background")]
    pub ghost_element_background: Option<String>,

    /// Background Color. Used for the hover state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    #[serde(rename = "ghost_element.hover")]
    pub ghost_element_hover: Option<String>,

    /// Background Color. Used for the active state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressed.
    #[serde(rename = "ghost_element.active")]
    pub ghost_element_active: Option<String>,

    /// Background Color. Used for the selected state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    #[serde(rename = "ghost_element.selected")]
    pub ghost_element_selected: Option<String>,

    /// Background Color. Used for the disabled state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    #[serde(rename = "ghost_element.disabled")]
    pub ghost_element_disabled: Option<String>,

    /// Text Color. Default text color used for most text.
    #[serde(rename = "text")]
    pub text: Option<String>,

    /// Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.
    #[serde(rename = "text.muted")]
    pub text_muted: Option<String>,

    /// Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data.
    #[serde(rename = "text.placeholder")]
    pub text_placeholder: Option<String>,

    /// Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state.
    #[serde(rename = "text.disabled")]
    pub text_disabled: Option<String>,

    /// Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search.
    #[serde(rename = "text.accent")]
    pub text_accent: Option<String>,

    /// Fill Color. Used for the default fill color of an icon.
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    /// Fill Color. Used for the muted or deemphasized fill color of an icon.
    ///
    /// This might be used to show an icon in an inactive pane, or to deemphasize a series of icons to give them less visual weight.
    #[serde(rename = "icon.muted")]
    pub icon_muted: Option<String>,

    /// Fill Color. Used for the disabled fill color of an icon.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a icon button.
    #[serde(rename = "icon.disabled")]
    pub icon_disabled: Option<String>,

    /// Fill Color. Used for the placeholder fill color of an icon.
    ///
    /// This might be used to show an icon in an input that disappears when the user enters text.
    #[serde(rename = "icon.placeholder")]
    pub icon_placeholder: Option<String>,

    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    #[serde(rename = "icon.accent")]
    pub icon_accent: Option<String>,

    /// Color used to accent some of the debuggers elements
    /// Only accent breakpoint & breakpoint related symbols right now
    #[serde(rename = "debugger.accent")]
    pub debugger_accent: Option<String>,

    #[serde(rename = "status_bar.background")]
    pub status_bar_background: Option<String>,

    #[serde(rename = "title_bar.background")]
    pub title_bar_background: Option<String>,

    #[serde(rename = "title_bar.inactive_background")]
    pub title_bar_inactive_background: Option<String>,

    #[serde(rename = "toolbar.background")]
    pub toolbar_background: Option<String>,

    #[serde(rename = "tab_bar.background")]
    pub tab_bar_background: Option<String>,

    #[serde(rename = "tab.inactive_background")]
    pub tab_inactive_background: Option<String>,

    #[serde(rename = "tab.active_background")]
    pub tab_active_background: Option<String>,

    #[serde(rename = "search.match_background")]
    pub search_match_background: Option<String>,

    #[serde(rename = "panel.background")]
    pub panel_background: Option<String>,

    #[serde(rename = "panel.focused_border")]
    pub panel_focused_border: Option<String>,

    #[serde(rename = "panel.indent_guide")]
    pub panel_indent_guide: Option<String>,

    #[serde(rename = "panel.indent_guide_hover")]
    pub panel_indent_guide_hover: Option<String>,

    #[serde(rename = "panel.indent_guide_active")]
    pub panel_indent_guide_active: Option<String>,

    #[serde(rename = "panel.overlay_background")]
    pub panel_overlay_background: Option<String>,

    #[serde(rename = "panel.overlay_hover")]
    pub panel_overlay_hover: Option<String>,

    #[serde(rename = "pane.focused_border")]
    pub pane_focused_border: Option<String>,

    #[serde(rename = "pane_group.border")]
    pub pane_group_border: Option<String>,

    /// The deprecated version of `scrollbar.thumb.background`.
    ///
    /// Don't use this field.
    #[serde(rename = "scrollbar_thumb.background", skip_serializing)]
    #[schemars(skip)]
    pub deprecated_scrollbar_thumb_background: Option<String>,

    /// The color of the scrollbar thumb.
    #[serde(rename = "scrollbar.thumb.background")]
    pub scrollbar_thumb_background: Option<String>,

    /// The color of the scrollbar thumb when hovered over.
    #[serde(rename = "scrollbar.thumb.hover_background")]
    pub scrollbar_thumb_hover_background: Option<String>,

    /// The color of the scrollbar thumb whilst being actively dragged.
    #[serde(rename = "scrollbar.thumb.active_background")]
    pub scrollbar_thumb_active_background: Option<String>,

    /// The border color of the scrollbar thumb.
    #[serde(rename = "scrollbar.thumb.border")]
    pub scrollbar_thumb_border: Option<String>,

    /// The background color of the scrollbar track.
    #[serde(rename = "scrollbar.track.background")]
    pub scrollbar_track_background: Option<String>,

    /// The border color of the scrollbar track.
    #[serde(rename = "scrollbar.track.border")]
    pub scrollbar_track_border: Option<String>,

    /// The color of the minimap thumb.
    #[serde(rename = "minimap.thumb.background")]
    pub minimap_thumb_background: Option<String>,

    /// The color of the minimap thumb when hovered over.
    #[serde(rename = "minimap.thumb.hover_background")]
    pub minimap_thumb_hover_background: Option<String>,

    /// The color of the minimap thumb whilst being actively dragged.
    #[serde(rename = "minimap.thumb.active_background")]
    pub minimap_thumb_active_background: Option<String>,

    /// The border color of the minimap thumb.
    #[serde(rename = "minimap.thumb.border")]
    pub minimap_thumb_border: Option<String>,

    #[serde(rename = "editor.foreground")]
    pub editor_foreground: Option<String>,

    #[serde(rename = "editor.background")]
    pub editor_background: Option<String>,

    #[serde(rename = "editor.gutter.background")]
    pub editor_gutter_background: Option<String>,

    #[serde(rename = "editor.subheader.background")]
    pub editor_subheader_background: Option<String>,

    #[serde(rename = "editor.active_line.background")]
    pub editor_active_line_background: Option<String>,

    #[serde(rename = "editor.highlighted_line.background")]
    pub editor_highlighted_line_background: Option<String>,

    /// Background of active line of debugger
    #[serde(rename = "editor.debugger_active_line.background")]
    pub editor_debugger_active_line_background: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter.
    #[serde(rename = "editor.line_number")]
    pub editor_line_number: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter when the line is highlighted.
    #[serde(rename = "editor.active_line_number")]
    pub editor_active_line_number: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter when the line is hovered over.
    #[serde(rename = "editor.hover_line_number")]
    pub editor_hover_line_number: Option<String>,

    /// Text Color. Used to mark invisible characters in the editor.
    ///
    /// Example: spaces, tabs, carriage returns, etc.
    #[serde(rename = "editor.invisible")]
    pub editor_invisible: Option<String>,

    #[serde(rename = "editor.wrap_guide")]
    pub editor_wrap_guide: Option<String>,

    #[serde(rename = "editor.active_wrap_guide")]
    pub editor_active_wrap_guide: Option<String>,

    #[serde(rename = "editor.indent_guide")]
    pub editor_indent_guide: Option<String>,

    #[serde(rename = "editor.indent_guide_active")]
    pub editor_indent_guide_active: Option<String>,

    /// Read-access of a symbol, like reading a variable.
    ///
    /// A document highlight is a range inside a text document which deserves
    /// special attention. Usually a document highlight is visualized by changing
    /// the background color of its range.
    #[serde(rename = "editor.document_highlight.read_background")]
    pub editor_document_highlight_read_background: Option<String>,

    /// Read-access of a symbol, like reading a variable.
    ///
    /// A document highlight is a range inside a text document which deserves
    /// special attention. Usually a document highlight is visualized by changing
    /// the background color of its range.
    #[serde(rename = "editor.document_highlight.write_background")]
    pub editor_document_highlight_write_background: Option<String>,

    /// Highlighted brackets background color.
    ///
    /// Matching brackets in the cursor scope are highlighted with this background color.
    #[serde(rename = "editor.document_highlight.bracket_background")]
    pub editor_document_highlight_bracket_background: Option<String>,

    /// Terminal background color.
    #[serde(rename = "terminal.background")]
    pub terminal_background: Option<String>,

    /// Terminal foreground color.
    #[serde(rename = "terminal.foreground")]
    pub terminal_foreground: Option<String>,

    /// Terminal ANSI background color.
    #[serde(rename = "terminal.ansi.background")]
    pub terminal_ansi_background: Option<String>,

    /// Bright terminal foreground color.
    #[serde(rename = "terminal.bright_foreground")]
    pub terminal_bright_foreground: Option<String>,

    /// Dim terminal foreground color.
    #[serde(rename = "terminal.dim_foreground")]
    pub terminal_dim_foreground: Option<String>,

    /// Black ANSI terminal color.
    #[serde(rename = "terminal.ansi.black")]
    pub terminal_ansi_black: Option<String>,

    /// Bright black ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_black")]
    pub terminal_ansi_bright_black: Option<String>,

    /// Dim black ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_black")]
    pub terminal_ansi_dim_black: Option<String>,

    /// Red ANSI terminal color.
    #[serde(rename = "terminal.ansi.red")]
    pub terminal_ansi_red: Option<String>,

    /// Bright red ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_red")]
    pub terminal_ansi_bright_red: Option<String>,

    /// Dim red ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_red")]
    pub terminal_ansi_dim_red: Option<String>,

    /// Green ANSI terminal color.
    #[serde(rename = "terminal.ansi.green")]
    pub terminal_ansi_green: Option<String>,

    /// Bright green ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_green")]
    pub terminal_ansi_bright_green: Option<String>,

    /// Dim green ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_green")]
    pub terminal_ansi_dim_green: Option<String>,

    /// Yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.yellow")]
    pub terminal_ansi_yellow: Option<String>,

    /// Bright yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_yellow")]
    pub terminal_ansi_bright_yellow: Option<String>,

    /// Dim yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_yellow")]
    pub terminal_ansi_dim_yellow: Option<String>,

    /// Blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.blue")]
    pub terminal_ansi_blue: Option<String>,

    /// Bright blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_blue")]
    pub terminal_ansi_bright_blue: Option<String>,

    /// Dim blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_blue")]
    pub terminal_ansi_dim_blue: Option<String>,

    /// Magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.magenta")]
    pub terminal_ansi_magenta: Option<String>,

    /// Bright magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_magenta")]
    pub terminal_ansi_bright_magenta: Option<String>,

    /// Dim magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_magenta")]
    pub terminal_ansi_dim_magenta: Option<String>,

    /// Cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.cyan")]
    pub terminal_ansi_cyan: Option<String>,

    /// Bright cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_cyan")]
    pub terminal_ansi_bright_cyan: Option<String>,

    /// Dim cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_cyan")]
    pub terminal_ansi_dim_cyan: Option<String>,

    /// White ANSI terminal color.
    #[serde(rename = "terminal.ansi.white")]
    pub terminal_ansi_white: Option<String>,

    /// Bright white ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_white")]
    pub terminal_ansi_bright_white: Option<String>,

    /// Dim white ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_white")]
    pub terminal_ansi_dim_white: Option<String>,

    #[serde(rename = "link_text.hover")]
    pub link_text_hover: Option<String>,

    /// Added version control color.
    #[serde(rename = "version_control.added")]
    pub version_control_added: Option<String>,

    /// Deleted version control color.
    #[serde(rename = "version_control.deleted")]
    pub version_control_deleted: Option<String>,

    /// Modified version control color.
    #[serde(rename = "version_control.modified")]
    pub version_control_modified: Option<String>,

    /// Renamed version control color.
    #[serde(rename = "version_control.renamed")]
    pub version_control_renamed: Option<String>,

    /// Conflict version control color.
    #[serde(rename = "version_control.conflict")]
    pub version_control_conflict: Option<String>,

    /// Ignored version control color.
    #[serde(rename = "version_control.ignored")]
    pub version_control_ignored: Option<String>,

    /// Background color for row highlights of "ours" regions in merge conflicts.
    #[serde(rename = "version_control.conflict_marker.ours")]
    pub version_control_conflict_marker_ours: Option<String>,

    /// Background color for row highlights of "theirs" regions in merge conflicts.
    #[serde(rename = "version_control.conflict_marker.theirs")]
    pub version_control_conflict_marker_theirs: Option<String>,

    /// Deprecated in favor of `version_control_conflict_marker_ours`.
    #[deprecated]
    pub version_control_conflict_ours_background: Option<String>,

    /// Deprecated in favor of `version_control_conflict_marker_theirs`.
    #[deprecated]
    pub version_control_conflict_theirs_background: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(default)]
pub struct HighlightStyleContent {
    pub color: Option<String>,

    #[serde(deserialize_with = "treat_error_as_none")]
    pub background_color: Option<String>,

    #[serde(deserialize_with = "treat_error_as_none")]
    pub font_style: Option<FontStyleContent>,

    #[serde(deserialize_with = "treat_error_as_none")]
    pub font_weight: Option<FontWeightContent>,
}

impl HighlightStyleContent {
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.background_color.is_none()
            && self.font_style.is_none()
            && self.font_weight.is_none()
    }
}

fn treat_error_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(value).ok())
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(default)]
pub struct StatusColorsContent {
    /// Indicates some kind of conflict, like a file changed on disk while it was open, or
    /// merge conflicts in a Git repository.
    #[serde(rename = "conflict")]
    pub conflict: Option<String>,

    #[serde(rename = "conflict.background")]
    pub conflict_background: Option<String>,

    #[serde(rename = "conflict.border")]
    pub conflict_border: Option<String>,

    /// Indicates something new, like a new file added to a Git repository.
    #[serde(rename = "created")]
    pub created: Option<String>,

    #[serde(rename = "created.background")]
    pub created_background: Option<String>,

    #[serde(rename = "created.border")]
    pub created_border: Option<String>,

    /// Indicates that something no longer exists, like a deleted file.
    #[serde(rename = "deleted")]
    pub deleted: Option<String>,

    #[serde(rename = "deleted.background")]
    pub deleted_background: Option<String>,

    #[serde(rename = "deleted.border")]
    pub deleted_border: Option<String>,

    /// Indicates a system error, a failed operation or a diagnostic error.
    #[serde(rename = "error")]
    pub error: Option<String>,

    #[serde(rename = "error.background")]
    pub error_background: Option<String>,

    #[serde(rename = "error.border")]
    pub error_border: Option<String>,

    /// Represents a hidden status, such as a file being hidden in a file tree.
    #[serde(rename = "hidden")]
    pub hidden: Option<String>,

    #[serde(rename = "hidden.background")]
    pub hidden_background: Option<String>,

    #[serde(rename = "hidden.border")]
    pub hidden_border: Option<String>,

    /// Indicates a hint or some kind of additional information.
    #[serde(rename = "hint")]
    pub hint: Option<String>,

    #[serde(rename = "hint.background")]
    pub hint_background: Option<String>,

    #[serde(rename = "hint.border")]
    pub hint_border: Option<String>,

    /// Indicates that something is deliberately ignored, such as a file or operation ignored by Git.
    #[serde(rename = "ignored")]
    pub ignored: Option<String>,

    #[serde(rename = "ignored.background")]
    pub ignored_background: Option<String>,

    #[serde(rename = "ignored.border")]
    pub ignored_border: Option<String>,

    /// Represents informational status updates or messages.
    #[serde(rename = "info")]
    pub info: Option<String>,

    #[serde(rename = "info.background")]
    pub info_background: Option<String>,

    #[serde(rename = "info.border")]
    pub info_border: Option<String>,

    /// Indicates a changed or altered status, like a file that has been edited.
    #[serde(rename = "modified")]
    pub modified: Option<String>,

    #[serde(rename = "modified.background")]
    pub modified_background: Option<String>,

    #[serde(rename = "modified.border")]
    pub modified_border: Option<String>,

    /// Indicates something that is predicted, like automatic code completion, or generated code.
    #[serde(rename = "predictive")]
    pub predictive: Option<String>,

    #[serde(rename = "predictive.background")]
    pub predictive_background: Option<String>,

    #[serde(rename = "predictive.border")]
    pub predictive_border: Option<String>,

    /// Represents a renamed status, such as a file that has been renamed.
    #[serde(rename = "renamed")]
    pub renamed: Option<String>,

    #[serde(rename = "renamed.background")]
    pub renamed_background: Option<String>,

    #[serde(rename = "renamed.border")]
    pub renamed_border: Option<String>,

    /// Indicates a successful operation or task completion.
    #[serde(rename = "success")]
    pub success: Option<String>,

    #[serde(rename = "success.background")]
    pub success_background: Option<String>,

    #[serde(rename = "success.border")]
    pub success_border: Option<String>,

    /// Indicates some kind of unreachable status, like a block of code that can never be reached.
    #[serde(rename = "unreachable")]
    pub unreachable: Option<String>,

    #[serde(rename = "unreachable.background")]
    pub unreachable_background: Option<String>,

    #[serde(rename = "unreachable.border")]
    pub unreachable_border: Option<String>,

    /// Represents a warning status, like an operation that is about to fail.
    #[serde(rename = "warning")]
    pub warning: Option<String>,

    #[serde(rename = "warning.background")]
    pub warning_background: Option<String>,

    #[serde(rename = "warning.border")]
    pub warning_border: Option<String>,
}

/// The background appearance of the window.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "snake_case")]
pub enum WindowBackgroundContent {
    Opaque,
    Transparent,
    Blurred,
}

impl Into<gpui::WindowBackgroundAppearance> for WindowBackgroundContent {
    fn into(self) -> gpui::WindowBackgroundAppearance {
        match self {
            WindowBackgroundContent::Opaque => gpui::WindowBackgroundAppearance::Opaque,
            WindowBackgroundContent::Transparent => gpui::WindowBackgroundAppearance::Transparent,
            WindowBackgroundContent::Blurred => gpui::WindowBackgroundAppearance::Blurred,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, MergeFrom, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FontStyleContent {
    Normal,
    Italic,
    Oblique,
}

impl From<FontStyleContent> for FontStyle {
    fn from(value: FontStyleContent) -> Self {
        match value {
            FontStyleContent::Normal => FontStyle::Normal,
            FontStyleContent::Italic => FontStyle::Italic,
            FontStyleContent::Oblique => FontStyle::Oblique,
        }
    }
}

#[derive(
    Debug, Clone, Copy, Serialize_repr, Deserialize_repr, JsonSchema_repr, PartialEq, MergeFrom,
)]
#[repr(u16)]
pub enum FontWeightContent {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl From<FontWeightContent> for FontWeight {
    fn from(value: FontWeightContent) -> Self {
        match value {
            FontWeightContent::Thin => FontWeight::THIN,
            FontWeightContent::ExtraLight => FontWeight::EXTRA_LIGHT,
            FontWeightContent::Light => FontWeight::LIGHT,
            FontWeightContent::Normal => FontWeight::NORMAL,
            FontWeightContent::Medium => FontWeight::MEDIUM,
            FontWeightContent::Semibold => FontWeight::SEMIBOLD,
            FontWeightContent::Bold => FontWeight::BOLD,
            FontWeightContent::ExtraBold => FontWeight::EXTRA_BOLD,
            FontWeightContent::Black => FontWeight::BLACK,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_buffer_line_height_deserialize_valid() {
        assert_eq!(
            serde_json::from_value::<BufferLineHeight>(json!("comfortable")).unwrap(),
            BufferLineHeight::Comfortable
        );
        assert_eq!(
            serde_json::from_value::<BufferLineHeight>(json!("standard")).unwrap(),
            BufferLineHeight::Standard
        );
        assert_eq!(
            serde_json::from_value::<BufferLineHeight>(json!({"custom": 1.0})).unwrap(),
            BufferLineHeight::Custom(1.0)
        );
        assert_eq!(
            serde_json::from_value::<BufferLineHeight>(json!({"custom": 1.5})).unwrap(),
            BufferLineHeight::Custom(1.5)
        );
    }

    #[test]
    fn test_buffer_line_height_deserialize_invalid() {
        assert!(
            serde_json::from_value::<BufferLineHeight>(json!({"custom": 0.99}))
                .err()
                .unwrap()
                .to_string()
                .contains("buffer_line_height.custom must be at least 1.0")
        );
        assert!(
            serde_json::from_value::<BufferLineHeight>(json!({"custom": 0.0}))
                .err()
                .unwrap()
                .to_string()
                .contains("buffer_line_height.custom must be at least 1.0")
        );
        assert!(
            serde_json::from_value::<BufferLineHeight>(json!({"custom": -1.0}))
                .err()
                .unwrap()
                .to_string()
                .contains("buffer_line_height.custom must be at least 1.0")
        );
    }
}
