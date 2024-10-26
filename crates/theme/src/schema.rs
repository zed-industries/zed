#![allow(missing_docs)]

use anyhow::Result;
use gpui::{FontStyle, FontWeight, HighlightStyle, Hsla, WindowBackgroundAppearance};
use indexmap::IndexMap;
use palette::FromColor;
use schemars::gen::SchemaGenerator;
use schemars::schema::{Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{StatusColorsRefinement, ThemeColorsRefinement};

/// The maximum depth of resolution for a `@reference`.
const MAX_RESOLUTION_DEPTH: usize = 8;

pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
    if let Some(reference) = color.strip_prefix('@') {
        return Err(anyhow::anyhow!("REFERENCE:{}", reference));
    }

    let rgba = gpui::Rgba::try_from(color)?;
    let rgba = palette::rgb::Srgba::from_components((rgba.r, rgba.g, rgba.b, rgba.a));
    let hsla = palette::Hsla::from_color(rgba);

    let hsla = gpui::hsla(
        hsla.hue.into_positive_degrees() / 360.,
        hsla.saturation,
        hsla.lightness,
        hsla.alpha,
    );

    Ok(hsla)
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceContent {
    Light,
    Dark,
}

/// The background appearance of the window.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WindowBackgroundContent {
    Opaque,
    Transparent,
    Blurred,
}

impl From<WindowBackgroundContent> for WindowBackgroundAppearance {
    fn from(value: WindowBackgroundContent) -> Self {
        match value {
            WindowBackgroundContent::Opaque => WindowBackgroundAppearance::Opaque,
            WindowBackgroundContent::Transparent => WindowBackgroundAppearance::Transparent,
            WindowBackgroundContent::Blurred => WindowBackgroundAppearance::Blurred,
        }
    }
}

/// The content of a serialized theme family.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ThemeFamilyContent {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeContent>,
}

/// The content of a serialized theme.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ThemeContent {
    pub name: String,
    pub appearance: AppearanceContent,
    pub style: ThemeStyleContent,
}

/// The content of a serialized theme.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
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

impl ThemeStyleContent {
    /// Returns a [`ThemeColorsRefinement`] based on the colors in the [`ThemeContent`].
    #[inline(always)]
    pub fn theme_colors_refinement(&self) -> ThemeColorsRefinement {
        self.colors.theme_colors_refinement()
    }

    /// Returns a [`StatusColorsRefinement`] based on the colors in the [`ThemeContent`].
    #[inline(always)]
    pub fn status_colors_refinement(&self) -> StatusColorsRefinement {
        self.status.status_colors_refinement()
    }

    /// Returns the syntax style overrides in the [`ThemeContent`].
    pub fn syntax_overrides(&self) -> Vec<(String, HighlightStyle)> {
        self.syntax
            .iter()
            .map(|(key, style)| {
                (
                    key.clone(),
                    HighlightStyle {
                        color: style
                            .color
                            .as_ref()
                            .and_then(|color| try_parse_color(color).ok()),
                        background_color: style
                            .background_color
                            .as_ref()
                            .and_then(|color| try_parse_color(color).ok()),
                        font_style: style.font_style.map(FontStyle::from),
                        font_weight: style.font_weight.map(FontWeight::from),
                        ..Default::default()
                    },
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
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
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
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

    /// Background Color. Used for the area that shows where a dragged element will be dropped.
    #[serde(rename = "drop_target.background")]
    pub drop_target_background: Option<String>,

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
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
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
    /// This might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight.
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

    /// The border color of the scrollbar thumb.
    #[serde(rename = "scrollbar.thumb.border")]
    pub scrollbar_thumb_border: Option<String>,

    /// The background color of the scrollbar track.
    #[serde(rename = "scrollbar.track.background")]
    pub scrollbar_track_background: Option<String>,

    /// The border color of the scrollbar track.
    #[serde(rename = "scrollbar.track.border")]
    pub scrollbar_track_border: Option<String>,

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

    /// Text Color. Used for the text of the line number in the editor gutter.
    #[serde(rename = "editor.line_number")]
    pub editor_line_number: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter when the line is highlighted.
    #[serde(rename = "editor.active_line_number")]
    pub editor_active_line_number: Option<String>,

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
}

impl ThemeColorsContent {
    // Don't format this function, as it reduces readability.
    #[rustfmt::skip]
    /// Returns a [`ThemeColorsRefinement`] based on the colors in the [`ThemeColorsContent`].
    pub fn theme_colors_refinement(&self) -> ThemeColorsRefinement {
        let mut resolved_colors = std::collections::HashMap::new();
        let mut unresolved_refs = std::collections::HashMap::new();

        fn process_color(
            key: &str,
            color_str: &str,
            resolved: &mut std::collections::HashMap<String, Hsla>,
            unresolved: &mut std::collections::HashMap<String, String>,
        ) {
            match try_parse_color(color_str) {
                Ok(color) => {
                    resolved.insert(key.to_string(), color);
                }
                Err(e) => {
                    if let Some(ref_str) = e.to_string().strip_prefix("REFERENCE:") {
                        unresolved.insert(key.to_string(), ref_str.to_string());
                    }
                }
            }
        }

        for (key, value) in [
            ("border", &self.border),
            ("border.variant", &self.border_variant),
            ("border.focused", &self.border_focused),
            ("border.selected", &self.border_selected),
            ("border.transparent", &self.border_transparent),
            ("border.disabled", &self.border_disabled),
            ("elevated_surface.background", &self.elevated_surface_background),
            ("surface.background", &self.surface_background),
            ("background", &self.background),
            ("element.background", &self.element_background),
            ("element.hover", &self.element_hover),
            ("element.active", &self.element_active),
            ("element.selected", &self.element_selected),
            ("element.disabled", &self.element_disabled),
            ("drop_target.background", &self.drop_target_background),
            ("ghost_element.background", &self.ghost_element_background),
            ("ghost_element.hover", &self.ghost_element_hover),
            ("ghost_element.active", &self.ghost_element_active),
            ("ghost_element.selected", &self.ghost_element_selected),
            ("ghost_element.disabled", &self.ghost_element_disabled),
            ("text", &self.text),
            ("text.muted", &self.text_muted),
            ("text.placeholder", &self.text_placeholder),
            ("text.disabled", &self.text_disabled),
            ("text.accent", &self.text_accent),
            ("icon", &self.icon),
            ("icon.muted", &self.icon_muted),
            ("icon.disabled", &self.icon_disabled),
            ("icon.placeholder", &self.icon_placeholder),
            ("icon.accent", &self.icon_accent),
            ("status_bar.background", &self.status_bar_background),
            ("title_bar.background", &self.title_bar_background),
            ("title_bar.inactive_background", &self.title_bar_inactive_background),
            ("toolbar.background", &self.toolbar_background),
            ("tab_bar.background", &self.tab_bar_background),
            ("tab.inactive_background", &self.tab_inactive_background),
            ("tab.active_background", &self.tab_active_background),
            ("search.match_background", &self.search_match_background),
            ("panel.background", &self.panel_background),
            ("panel.focused_border", &self.panel_focused_border),
            ("panel.indent_guide", &self.panel_indent_guide),
            ("panel.indent_guide_hover", &self.panel_indent_guide_hover),
            ("panel.indent_guide_active", &self.panel_indent_guide_active),
            ("pane.focused_border", &self.pane_focused_border),
            ("pane_group.border", &self.pane_group_border),
            ("scrollbar.thumb.background", &self.scrollbar_thumb_background),
            ("scrollbar.thumb.hover_background", &self.scrollbar_thumb_hover_background),
            ("scrollbar.thumb.border", &self.scrollbar_thumb_border),
            ("scrollbar.track.background", &self.scrollbar_track_background),
            ("scrollbar.track.border", &self.scrollbar_track_border),
            ("editor.foreground", &self.editor_foreground),
            ("editor.background", &self.editor_background),
            ("editor.gutter.background", &self.editor_gutter_background),
            ("editor.subheader.background", &self.editor_subheader_background),
            ("editor.active_line.background", &self.editor_active_line_background),
            ("editor.highlighted_line.background", &self.editor_highlighted_line_background),
            ("editor.line_number", &self.editor_line_number),
            ("editor.active_line_number", &self.editor_active_line_number),
            ("editor.invisible", &self.editor_invisible),
            ("editor.wrap_guide", &self.editor_wrap_guide),
            ("editor.active_wrap_guide", &self.editor_active_wrap_guide),
            ("editor.indent_guide", &self.editor_indent_guide),
            ("editor.indent_guide_active", &self.editor_indent_guide_active),
            ("editor.document_highlight.read_background", &self.editor_document_highlight_read_background),
            ("editor.document_highlight.write_background", &self.editor_document_highlight_write_background),
            ("editor.document_highlight.bracket_background", &self.editor_document_highlight_bracket_background),
            ("terminal.background", &self.terminal_background),
            ("terminal.ansi.background", &self.terminal_ansi_background),
            ("terminal.foreground", &self.terminal_foreground),
            ("terminal.bright_foreground", &self.terminal_bright_foreground),
            ("terminal.dim_foreground", &self.terminal_dim_foreground),
            ("terminal.ansi.black", &self.terminal_ansi_black),
            ("terminal.ansi.bright_black", &self.terminal_ansi_bright_black),
            ("terminal.ansi.dim_black", &self.terminal_ansi_dim_black),
            ("terminal.ansi.red", &self.terminal_ansi_red),
            ("terminal.ansi.bright_red", &self.terminal_ansi_bright_red),
            ("terminal.ansi.dim_red", &self.terminal_ansi_dim_red),
            ("terminal.ansi.green", &self.terminal_ansi_green),
            ("terminal.ansi.bright_green", &self.terminal_ansi_bright_green),
            ("terminal.ansi.dim_green", &self.terminal_ansi_dim_green),
            ("terminal.ansi.yellow", &self.terminal_ansi_yellow),
            ("terminal.ansi.bright_yellow", &self.terminal_ansi_bright_yellow),
            ("terminal.ansi.dim_yellow", &self.terminal_ansi_dim_yellow),
            ("terminal.ansi.blue", &self.terminal_ansi_blue),
            ("terminal.ansi.bright_blue", &self.terminal_ansi_bright_blue),
            ("terminal.ansi.dim_blue", &self.terminal_ansi_dim_blue),
            ("terminal.ansi.magenta", &self.terminal_ansi_magenta),
            ("terminal.ansi.bright_magenta", &self.terminal_ansi_bright_magenta),
            ("terminal.ansi.dim_magenta", &self.terminal_ansi_dim_magenta),
            ("terminal.ansi.cyan", &self.terminal_ansi_cyan),
            ("terminal.ansi.bright_cyan", &self.terminal_ansi_bright_cyan),
            ("terminal.ansi.dim_cyan", &self.terminal_ansi_dim_cyan),
            ("terminal.ansi.white", &self.terminal_ansi_white),
            ("terminal.ansi.bright_white", &self.terminal_ansi_bright_white),
            ("terminal.ansi.dim_white", &self.terminal_ansi_dim_white),
            ("link_text.hover", &self.link_text_hover),
        ] {
            if let Some(color_str) = value {
                process_color(key, color_str, &mut resolved_colors, &mut unresolved_refs);
            }
        }

        let max_iterations = MAX_RESOLUTION_DEPTH;
        let mut iterations = 0;
        let mut changed = true;
        while changed && iterations < max_iterations {
            changed = false;
            let mut new_resolved = std::collections::HashMap::new();
            let mut new_unresolved = std::collections::HashMap::new();

            for (key, ref_name) in &unresolved_refs {
                if let Some(&color) = resolved_colors.get(ref_name) {
                    new_resolved.insert(key.clone(), color);
                    changed = true;
                } else if let Some(next_ref) = unresolved_refs.get(ref_name) {
                    new_unresolved.insert(key.clone(), next_ref.clone());
                    changed = true;
                } else {
                    new_unresolved.insert(key.clone(), ref_name.clone());
                }
            }

            resolved_colors.extend(new_resolved);
            unresolved_refs = new_unresolved;
            iterations += 1;
        }

        if iterations == max_iterations {
            log::warn!("Hit iteration limit of {} while resolving color references. Some colors may not be resolved.", MAX_RESOLUTION_DEPTH);
        }

        ThemeColorsRefinement {
            border: resolved_colors.get("border").copied(),
            border_variant: resolved_colors.get("border.variant").copied(),
            border_focused: resolved_colors.get("border.focused").copied(),
            border_selected: resolved_colors.get("border.selected").copied(),
            border_transparent: resolved_colors.get("border.transparent").copied(),
            border_disabled: resolved_colors.get("border.disabled").copied(),
            elevated_surface_background: resolved_colors.get("elevated_surface.background").copied(),
            surface_background: resolved_colors.get("surface.background").copied(),
            background: resolved_colors.get("background").copied(),
            element_background: resolved_colors.get("element.background").copied(),
            element_hover: resolved_colors.get("element.hover").copied(),
            element_active: resolved_colors.get("element.active").copied(),
            element_selected: resolved_colors.get("element.selected").copied(),
            element_disabled: resolved_colors.get("element.disabled").copied(),
            drop_target_background: resolved_colors.get("drop_target.background").copied(),
            ghost_element_background: resolved_colors.get("ghost_element.background").copied(),
            ghost_element_hover: resolved_colors.get("ghost_element.hover").copied(),
            ghost_element_active: resolved_colors.get("ghost_element.active").copied(),
            ghost_element_selected: resolved_colors.get("ghost_element.selected").copied(),
            ghost_element_disabled: resolved_colors.get("ghost_element.disabled").copied(),
            text: resolved_colors.get("text").copied(),
            text_muted: resolved_colors.get("text.muted").copied(),
            text_placeholder: resolved_colors.get("text.placeholder").copied(),
            text_disabled: resolved_colors.get("text.disabled").copied(),
            text_accent: resolved_colors.get("text.accent").copied(),
            icon: resolved_colors.get("icon").copied(),
            icon_muted: resolved_colors.get("icon.muted").copied(),
            icon_disabled: resolved_colors.get("icon.disabled").copied(),
            icon_placeholder: resolved_colors.get("icon.placeholder").copied(),
            icon_accent: resolved_colors.get("icon.accent").copied(),
            status_bar_background: resolved_colors.get("status_bar.background").copied(),
            title_bar_background: resolved_colors.get("title_bar.background").copied(),
            title_bar_inactive_background: resolved_colors.get("title_bar.inactive_background").copied(),
            toolbar_background: resolved_colors.get("toolbar.background").copied(),
            tab_bar_background: resolved_colors.get("tab_bar.background").copied(),
            tab_inactive_background: resolved_colors.get("tab.inactive_background").copied(),
            tab_active_background: resolved_colors.get("tab.active_background").copied(),
            search_match_background: resolved_colors.get("search.match_background").copied(),
            panel_background: resolved_colors.get("panel.background").copied(),
            panel_focused_border: resolved_colors.get("panel.focused_border").copied(),
            panel_indent_guide: resolved_colors.get("panel.indent_guide").copied(),
            panel_indent_guide_hover: resolved_colors.get("panel.indent_guide_hover").copied(),
            panel_indent_guide_active: resolved_colors.get("panel.indent_guide_active").copied(),
            pane_focused_border: resolved_colors.get("pane.focused_border").copied(),
            pane_group_border: resolved_colors.get("pane_group.border").copied(),
            scrollbar_thumb_background: resolved_colors.get("scrollbar.thumb.background").copied(),
            scrollbar_thumb_hover_background: resolved_colors.get("scrollbar.thumb.hover_background").copied(),
            scrollbar_thumb_border: resolved_colors.get("scrollbar.thumb.border").copied(),
            scrollbar_track_background: resolved_colors.get("scrollbar.track.background").copied(),
            scrollbar_track_border: resolved_colors.get("scrollbar.track.border").copied(),
            editor_foreground: resolved_colors.get("editor.foreground").copied(),
            editor_background: resolved_colors.get("editor.background").copied(),
            editor_gutter_background: resolved_colors.get("editor.gutter.background").copied(),
            editor_subheader_background: resolved_colors.get("editor.subheader.background").copied(),
            editor_active_line_background: resolved_colors.get("editor.active_line.background").copied(),
            editor_highlighted_line_background: resolved_colors.get("editor.highlighted_line.background").copied(),
            editor_line_number: resolved_colors.get("editor.line_number").copied(),
            editor_active_line_number: resolved_colors.get("editor.active_line_number").copied(),
            editor_invisible: resolved_colors.get("editor.invisible").copied(),
            editor_wrap_guide: resolved_colors.get("editor.wrap_guide").copied(),
            editor_active_wrap_guide: resolved_colors.get("editor.active_wrap_guide").copied(),
            editor_indent_guide: resolved_colors.get("editor.indent_guide").copied(),
            editor_indent_guide_active: resolved_colors.get("editor.indent_guide_active").copied(),
            editor_document_highlight_read_background: resolved_colors.get("editor.document_highlight.read_background").copied(),
            editor_document_highlight_write_background: resolved_colors.get("editor.document_highlight.write_background").copied(),
            editor_document_highlight_bracket_background: resolved_colors.get("editor.document_highlight.bracket_background").copied(),
            terminal_background: resolved_colors.get("terminal.background").copied(),
            terminal_ansi_background: resolved_colors.get("terminal.ansi.background").copied(),
            terminal_foreground: resolved_colors.get("terminal.foreground").copied(),
            terminal_bright_foreground: resolved_colors.get("terminal.bright_foreground").copied(),
            terminal_dim_foreground: resolved_colors.get("terminal.dim_foreground").copied(),
            terminal_ansi_black: resolved_colors.get("terminal.ansi.black").copied(),
            terminal_ansi_bright_black: resolved_colors.get("terminal.ansi.bright_black").copied(),
            terminal_ansi_dim_black: resolved_colors.get("terminal.ansi.dim_black").copied(),
            terminal_ansi_red: resolved_colors.get("terminal.ansi.red").copied(),
            terminal_ansi_bright_red: resolved_colors.get("terminal.ansi.bright_red").copied(),
            terminal_ansi_dim_red: resolved_colors.get("terminal.ansi.dim_red").copied(),
            terminal_ansi_green: resolved_colors.get("terminal.ansi.green").copied(),
            terminal_ansi_bright_green: resolved_colors.get("terminal.ansi.bright_green").copied(),
            terminal_ansi_dim_green: resolved_colors.get("terminal.ansi.dim_green").copied(),
            terminal_ansi_yellow: resolved_colors.get("terminal.ansi.yellow").copied(),
            terminal_ansi_bright_yellow: resolved_colors.get("terminal.ansi.bright_yellow").copied(),
            terminal_ansi_dim_yellow: resolved_colors.get("terminal.ansi.dim_yellow").copied(),
            terminal_ansi_blue: resolved_colors.get("terminal.ansi.blue").copied(),
            terminal_ansi_bright_blue: resolved_colors.get("terminal.ansi.bright_blue").copied(),
            terminal_ansi_dim_blue: resolved_colors.get("terminal.ansi.dim_blue").copied(),
            terminal_ansi_magenta: resolved_colors.get("terminal.ansi.magenta").copied(),
            terminal_ansi_bright_magenta: resolved_colors.get("terminal.ansi.bright_magenta").copied(),
            terminal_ansi_dim_magenta: resolved_colors.get("terminal.ansi.dim_magenta").copied(),
            terminal_ansi_cyan: resolved_colors.get("terminal.ansi.cyan").copied(),
            terminal_ansi_bright_cyan: resolved_colors.get("terminal.ansi.bright_cyan").copied(),
            terminal_ansi_dim_cyan: resolved_colors.get("terminal.ansi.dim_cyan").copied(),
            terminal_ansi_white: resolved_colors.get("terminal.ansi.white").copied(),
            terminal_ansi_bright_white: resolved_colors.get("terminal.ansi.bright_white").copied(),
            terminal_ansi_dim_white: resolved_colors.get("terminal.ansi.dim_white").copied(),
            link_text_hover: resolved_colors.get("link_text.hover").copied(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
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

impl StatusColorsContent {
    /// Returns a [`StatusColorsRefinement`] based on the colors in the [`StatusColorsContent`].
    pub fn status_colors_refinement(&self) -> StatusColorsRefinement {
        let mut resolved_colors = std::collections::HashMap::new();
        let mut unresolved_refs = std::collections::HashMap::new();

        fn process_color(
            key: &str,
            color_str: &str,
            resolved: &mut std::collections::HashMap<String, Hsla>,
            unresolved: &mut std::collections::HashMap<String, String>,
        ) {
            match try_parse_color(color_str) {
                Ok(color) => {
                    resolved.insert(key.to_string(), color);
                }
                Err(e) => {
                    if let Some(ref_str) = e.to_string().strip_prefix("REFERENCE:") {
                        unresolved.insert(key.to_string(), ref_str.to_string());
                    }
                }
            }
        }

        for (key, value) in [
            ("conflict", &self.conflict),
            ("conflict.background", &self.conflict_background),
            ("conflict.border", &self.conflict_border),
            ("created", &self.created),
            ("created.background", &self.created_background),
            ("created.border", &self.created_border),
            ("deleted", &self.deleted),
            ("deleted.background", &self.deleted_background),
            ("deleted.border", &self.deleted_border),
            ("error", &self.error),
            ("error.background", &self.error_background),
            ("error.border", &self.error_border),
            ("hidden", &self.hidden),
            ("hidden.background", &self.hidden_background),
            ("hidden.border", &self.hidden_border),
            ("hint", &self.hint),
            ("hint.background", &self.hint_background),
            ("hint.border", &self.hint_border),
            ("ignored", &self.ignored),
            ("ignored.background", &self.ignored_background),
            ("ignored.border", &self.ignored_border),
            ("info", &self.info),
            ("info.background", &self.info_background),
            ("info.border", &self.info_border),
            ("modified", &self.modified),
            ("modified.background", &self.modified_background),
            ("modified.border", &self.modified_border),
            ("predictive", &self.predictive),
            ("predictive.background", &self.predictive_background),
            ("predictive.border", &self.predictive_border),
            ("renamed", &self.renamed),
            ("renamed.background", &self.renamed_background),
            ("renamed.border", &self.renamed_border),
            ("success", &self.success),
            ("success.background", &self.success_background),
            ("success.border", &self.success_border),
            ("unreachable", &self.unreachable),
            ("unreachable.background", &self.unreachable_background),
            ("unreachable.border", &self.unreachable_border),
            ("warning", &self.warning),
            ("warning.background", &self.warning_background),
            ("warning.border", &self.warning_border),
        ] {
            if let Some(color_str) = value {
                process_color(key, color_str, &mut resolved_colors, &mut unresolved_refs);
            }
        }

        let max_iterations = MAX_RESOLUTION_DEPTH;
        let mut iterations = 0;
        let mut changed = true;
        while changed && iterations < max_iterations {
            changed = false;
            let mut new_resolved = std::collections::HashMap::new();
            let mut new_unresolved = std::collections::HashMap::new();

            for (key, ref_name) in &unresolved_refs {
                if let Some(&color) = resolved_colors.get(ref_name) {
                    new_resolved.insert(key.clone(), color);
                    changed = true;
                } else if let Some(next_ref) = unresolved_refs.get(ref_name) {
                    new_unresolved.insert(key.clone(), next_ref.clone());
                    changed = true;
                } else {
                    new_unresolved.insert(key.clone(), ref_name.clone());
                }
            }

            resolved_colors.extend(new_resolved);
            unresolved_refs = new_unresolved;
            iterations += 1;
        }

        if iterations == max_iterations {
            log::warn!("Hit iteration limit of {} while resolving color references. Some colors may not be resolved.", MAX_RESOLUTION_DEPTH);
        }

        StatusColorsRefinement {
            conflict: resolved_colors.get("conflict").copied(),
            conflict_background: resolved_colors.get("conflict.background").copied(),
            conflict_border: resolved_colors.get("conflict.border").copied(),
            created: resolved_colors.get("created").copied(),
            created_background: resolved_colors.get("created.background").copied(),
            created_border: resolved_colors.get("created.border").copied(),
            deleted: resolved_colors.get("deleted").copied(),
            deleted_background: resolved_colors.get("deleted.background").copied(),
            deleted_border: resolved_colors.get("deleted.border").copied(),
            error: resolved_colors.get("error").copied(),
            error_background: resolved_colors.get("error.background").copied(),
            error_border: resolved_colors.get("error.border").copied(),
            hidden: resolved_colors.get("hidden").copied(),
            hidden_background: resolved_colors.get("hidden.background").copied(),
            hidden_border: resolved_colors.get("hidden.border").copied(),
            hint: resolved_colors.get("hint").copied(),
            hint_background: resolved_colors.get("hint.background").copied(),
            hint_border: resolved_colors.get("hint.border").copied(),
            ignored: resolved_colors.get("ignored").copied(),
            ignored_background: resolved_colors.get("ignored.background").copied(),
            ignored_border: resolved_colors.get("ignored.border").copied(),
            info: resolved_colors.get("info").copied(),
            info_background: resolved_colors.get("info.background").copied(),
            info_border: resolved_colors.get("info.border").copied(),
            modified: resolved_colors.get("modified").copied(),
            modified_background: resolved_colors.get("modified.background").copied(),
            modified_border: resolved_colors.get("modified.border").copied(),
            predictive: resolved_colors.get("predictive").copied(),
            predictive_background: resolved_colors.get("predictive.background").copied(),
            predictive_border: resolved_colors.get("predictive.border").copied(),
            renamed: resolved_colors.get("renamed").copied(),
            renamed_background: resolved_colors.get("renamed.background").copied(),
            renamed_border: resolved_colors.get("renamed.border").copied(),
            success: resolved_colors.get("success").copied(),
            success_background: resolved_colors.get("success.background").copied(),
            success_border: resolved_colors.get("success.border").copied(),
            unreachable: resolved_colors.get("unreachable").copied(),
            unreachable_background: resolved_colors.get("unreachable.background").copied(),
            unreachable_border: resolved_colors.get("unreachable.border").copied(),
            warning: resolved_colors.get("warning").copied(),
            warning_background: resolved_colors.get("warning.background").copied(),
            warning_border: resolved_colors.get("warning.border").copied(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AccentContent(pub Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct PlayerColorContent {
    pub cursor: Option<String>,
    pub background: Option<String>,
    pub selection: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq)]
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

impl JsonSchema for FontWeightContent {
    fn schema_name() -> String {
        "FontWeightContent".to_owned()
    }

    fn is_referenceable() -> bool {
        false
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            enum_values: Some(vec![
                100.into(),
                200.into(),
                300.into(),
                400.into(),
                500.into(),
                600.into(),
                700.into(),
                800.into(),
                900.into(),
            ]),
            ..Default::default()
        }
        .into()
    }
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_color_references() {
        let colors: ThemeColorsContent = serde_json::from_value(json!({
            "text": "#ff0000ff",
            "icon": "@text",
            "scrollbar.thumb.border": "#ff00ffff",
            "scrollbar.track.border": "@scrollbar.thumb.border"
        }))
        .unwrap();

        let refinement = colors.theme_colors_refinement();

        let expected_color_1 = gpui::rgba(0xff0000ff);
        let expected_color_2 = gpui::rgba(0xff00ffff);
        assert_eq!(refinement.text, Some(expected_color_1.into()));
        assert_eq!(refinement.icon, Some(expected_color_1.into()));
        assert_eq!(
            refinement.scrollbar_thumb_border,
            Some(expected_color_2.into())
        );
        assert_eq!(
            refinement.scrollbar_track_border,
            Some(expected_color_2.into())
        );
    }

    #[test]
    fn test_short_reference_chain() {
        let colors: StatusColorsContent = serde_json::from_value(json!({
            "conflict": "#ff0000ff",
            "modified": "@conflict",
            "warning": "@modified"
        }))
        .unwrap();

        let refinement = colors.status_colors_refinement();

        let expected_color = gpui::rgba(0xff0000ff);
        assert_eq!(refinement.conflict, Some(expected_color.into()));
        assert_eq!(refinement.modified, Some(expected_color.into()));
        assert_eq!(refinement.warning, Some(expected_color.into()));
    }

    #[test]
    fn test_long_reference_chain() {
        let color_json = json!({
            "border": "#ff0000ff",
            "border_variant": "@border",
            "border_focused": "@border_variant",
            "border_selected": "@border_focused",
            "border_transparent": "@border_selected",
            "border_disabled": "@border_transparent",
            "elevated_surface_background": "@border_disabled",
            "surface_background": "@elevated_surface_background",
            "background": "@surface_background",
            "element_background": "@background",
            "element_hover": "@element_background",
            "element_active": "@element_hover",
            "element_selected": "@element_active",
            "element_disabled": "@element_selected",
            "drop_target_background": "@element_disabled",
            "ghost_element_background": "@drop_target_background",
            "ghost_element_hover": "@ghost_element_background",
            "ghost_element_active": "@ghost_element_hover",
            "ghost_element_selected": "@ghost_element_active",
            "ghost_element_disabled": "@ghost_element_selected",
            "text": "@ghost_element_disabled",
            "text_muted": "@text",
            "text_placeholder": "@text_muted",
            "text_disabled": "@text_placeholder",
            "text_accent": "@text_disabled",
            "link_text_hover": "@text_accent"
        });

        let colors: ThemeColorsContent = serde_json::from_value(color_json).unwrap();
        let refinement = colors.theme_colors_refinement();

        let expected_color = gpui::rgba(0xff0000ff);

        // Long reference chains won't be resolved if they are greater than MAX_RESOLUTION_DEPTH
        assert_eq!(refinement.border, Some(expected_color.into()));
        assert_eq!(refinement.link_text_hover, None);
    }

    #[test]
    fn test_circular_references() {
        let colors: ThemeColorsContent = serde_json::from_value(json!({
            "text": "@icon",
            "icon": "@text"
        }))
        .unwrap();

        let refinement = colors.theme_colors_refinement();

        // Circular references should be left unresolved
        assert_eq!(refinement.text, None);
        assert_eq!(refinement.icon, None);
    }
}
