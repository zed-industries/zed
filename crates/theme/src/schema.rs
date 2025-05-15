#![allow(missing_docs)]

use anyhow::Result;
use gpui::{FontStyle, FontWeight, HighlightStyle, Hsla, WindowBackgroundAppearance};
use indexmap::IndexMap;
use palette::FromColor;
use schemars::JsonSchema;
use schemars::r#gen::SchemaGenerator;
use schemars::schema::{Schema, SchemaObject};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{StatusColorsRefinement, ThemeColorsRefinement};

pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
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
        self.colors
            .theme_colors_refinement(&self.status_colors_refinement())
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

    /// Background color for row highlights of the "ours"/"theirs" divider in merge conflicts.
    #[serde(rename = "version_control.conflict_marker.border")]
    pub version_control_conflict_marker_border: Option<String>,
}

impl ThemeColorsContent {
    /// Returns a [`ThemeColorsRefinement`] based on the colors in the [`ThemeColorsContent`].
    pub fn theme_colors_refinement(
        &self,
        status_colors: &StatusColorsRefinement,
    ) -> ThemeColorsRefinement {
        let border = self
            .border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let editor_document_highlight_read_background = self
            .editor_document_highlight_read_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let scrollbar_thumb_background = self
            .scrollbar_thumb_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or_else(|| {
                self.deprecated_scrollbar_thumb_background
                    .as_ref()
                    .and_then(|color| try_parse_color(color).ok())
            });
        ThemeColorsRefinement {
            border,
            border_variant: self
                .border_variant
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            border_focused: self
                .border_focused
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            border_selected: self
                .border_selected
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            border_transparent: self
                .border_transparent
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            border_disabled: self
                .border_disabled
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            elevated_surface_background: self
                .elevated_surface_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            surface_background: self
                .surface_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            background: self
                .background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            element_background: self
                .element_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            element_hover: self
                .element_hover
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            element_active: self
                .element_active
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            element_selected: self
                .element_selected
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            element_disabled: self
                .element_disabled
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            drop_target_background: self
                .drop_target_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ghost_element_background: self
                .ghost_element_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ghost_element_hover: self
                .ghost_element_hover
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ghost_element_active: self
                .ghost_element_active
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ghost_element_selected: self
                .ghost_element_selected
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ghost_element_disabled: self
                .ghost_element_disabled
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            text: self
                .text
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            text_muted: self
                .text_muted
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            text_placeholder: self
                .text_placeholder
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            text_disabled: self
                .text_disabled
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            text_accent: self
                .text_accent
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            icon: self
                .icon
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            icon_muted: self
                .icon_muted
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            icon_disabled: self
                .icon_disabled
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            icon_placeholder: self
                .icon_placeholder
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            icon_accent: self
                .icon_accent
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            debugger_accent: self
                .debugger_accent
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            status_bar_background: self
                .status_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            title_bar_background: self
                .title_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            title_bar_inactive_background: self
                .title_bar_inactive_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            toolbar_background: self
                .toolbar_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            tab_bar_background: self
                .tab_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            tab_inactive_background: self
                .tab_inactive_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            tab_active_background: self
                .tab_active_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            search_match_background: self
                .search_match_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            panel_background: self
                .panel_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            panel_focused_border: self
                .panel_focused_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            panel_indent_guide: self
                .panel_indent_guide
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            panel_indent_guide_hover: self
                .panel_indent_guide_hover
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            panel_indent_guide_active: self
                .panel_indent_guide_active
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            pane_focused_border: self
                .pane_focused_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            pane_group_border: self
                .pane_group_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(border),
            scrollbar_thumb_background,
            scrollbar_thumb_hover_background: self
                .scrollbar_thumb_hover_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            scrollbar_thumb_active_background: self
                .scrollbar_thumb_active_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(scrollbar_thumb_background),
            scrollbar_thumb_border: self
                .scrollbar_thumb_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            scrollbar_track_background: self
                .scrollbar_track_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            scrollbar_track_border: self
                .scrollbar_track_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_foreground: self
                .editor_foreground
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_background: self
                .editor_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_gutter_background: self
                .editor_gutter_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_subheader_background: self
                .editor_subheader_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_active_line_background: self
                .editor_active_line_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_highlighted_line_background: self
                .editor_highlighted_line_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_debugger_active_line_background: self
                .editor_debugger_active_line_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_line_number: self
                .editor_line_number
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_hover_line_number: self
                .editor_hover_line_number
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_active_line_number: self
                .editor_active_line_number
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_invisible: self
                .editor_invisible
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_wrap_guide: self
                .editor_wrap_guide
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_active_wrap_guide: self
                .editor_active_wrap_guide
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_indent_guide: self
                .editor_indent_guide
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_indent_guide_active: self
                .editor_indent_guide_active
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_document_highlight_read_background,
            editor_document_highlight_write_background: self
                .editor_document_highlight_write_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            editor_document_highlight_bracket_background: self
                .editor_document_highlight_bracket_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `editor.document_highlight.read_background`, for backwards compatibility.
                .or(editor_document_highlight_read_background),
            terminal_background: self
                .terminal_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_background: self
                .terminal_ansi_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_foreground: self
                .terminal_foreground
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_bright_foreground: self
                .terminal_bright_foreground
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_dim_foreground: self
                .terminal_dim_foreground
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_black: self
                .terminal_ansi_black
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_black: self
                .terminal_ansi_bright_black
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_black: self
                .terminal_ansi_dim_black
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_red: self
                .terminal_ansi_red
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_red: self
                .terminal_ansi_bright_red
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_red: self
                .terminal_ansi_dim_red
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_green: self
                .terminal_ansi_green
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_green: self
                .terminal_ansi_bright_green
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_green: self
                .terminal_ansi_dim_green
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_yellow: self
                .terminal_ansi_yellow
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_yellow: self
                .terminal_ansi_bright_yellow
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_yellow: self
                .terminal_ansi_dim_yellow
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_blue: self
                .terminal_ansi_blue
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_blue: self
                .terminal_ansi_bright_blue
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_blue: self
                .terminal_ansi_dim_blue
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_magenta: self
                .terminal_ansi_magenta
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_magenta: self
                .terminal_ansi_bright_magenta
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_magenta: self
                .terminal_ansi_dim_magenta
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_cyan: self
                .terminal_ansi_cyan
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_cyan: self
                .terminal_ansi_bright_cyan
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_cyan: self
                .terminal_ansi_dim_cyan
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_white: self
                .terminal_ansi_white
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_bright_white: self
                .terminal_ansi_bright_white
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            terminal_ansi_dim_white: self
                .terminal_ansi_dim_white
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            link_text_hover: self
                .link_text_hover
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            version_control_added: self
                .version_control_added
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `created`, for backwards compatibility.
                .or(status_colors.created),
            version_control_deleted: self
                .version_control_deleted
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `deleted`, for backwards compatibility.
                .or(status_colors.deleted),
            version_control_modified: self
                .version_control_modified
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `modified`, for backwards compatibility.
                .or(status_colors.modified),
            version_control_renamed: self
                .version_control_renamed
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `modified`, for backwards compatibility.
                .or(status_colors.modified),
            version_control_conflict: self
                .version_control_conflict
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `ignored`, for backwards compatibility.
                .or(status_colors.ignored),
            version_control_ignored: self
                .version_control_ignored
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                // Fall back to `conflict`, for backwards compatibility.
                .or(status_colors.ignored),
            version_control_conflict_marker_ours: self
                .version_control_conflict_marker_ours
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            version_control_conflict_marker_theirs: self
                .version_control_conflict_marker_theirs
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            version_control_conflict_marker_border: self
                .version_control_conflict_marker_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
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
        StatusColorsRefinement {
            conflict: self
                .conflict
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            conflict_background: self
                .conflict_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            conflict_border: self
                .conflict_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            created: self
                .created
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            created_background: self
                .created_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            created_border: self
                .created_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            deleted: self
                .deleted
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            deleted_background: self
                .deleted_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            deleted_border: self
                .deleted_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            error: self
                .error
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            error_background: self
                .error_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            error_border: self
                .error_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hidden: self
                .hidden
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hidden_background: self
                .hidden_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hidden_border: self
                .hidden_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hint: self
                .hint
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hint_background: self
                .hint_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            hint_border: self
                .hint_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ignored: self
                .ignored
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ignored_background: self
                .ignored_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            ignored_border: self
                .ignored_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            info: self
                .info
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            info_background: self
                .info_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            info_border: self
                .info_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            modified: self
                .modified
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            modified_background: self
                .modified_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            modified_border: self
                .modified_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            predictive: self
                .predictive
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            predictive_background: self
                .predictive_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            predictive_border: self
                .predictive_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            renamed: self
                .renamed
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            renamed_background: self
                .renamed_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            renamed_border: self
                .renamed_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            success: self
                .success
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            success_background: self
                .success_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            success_border: self
                .success_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            unreachable: self
                .unreachable
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            unreachable_background: self
                .unreachable_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            unreachable_border: self
                .unreachable_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            warning: self
                .warning
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            warning_background: self
                .warning_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            warning_border: self
                .warning_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
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
