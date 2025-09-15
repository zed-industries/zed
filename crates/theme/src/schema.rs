#![allow(missing_docs)]

use anyhow::Result;
use gpui::{FontStyle, FontWeight, HighlightStyle, Hsla, WindowBackgroundAppearance};
use indexmap::IndexMap;
use palette::FromColor;
use schemars::{JsonSchema, JsonSchema_repr};
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

fn ensure_non_opaque(color: Hsla) -> Hsla {
    const MAXIMUM_OPACITY: f32 = 0.7;
    if color.a <= MAXIMUM_OPACITY {
        color
    } else {
        Hsla {
            a: MAXIMUM_OPACITY,
            ..color
        }
    }
}

fn ensure_opaque(color: Hsla) -> Hsla {
    Hsla { a: 1.0, ..color }
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
        let scrollbar_thumb_hover_background = self
            .scrollbar_thumb_hover_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let scrollbar_thumb_active_background = self
            .scrollbar_thumb_active_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(scrollbar_thumb_background);
        let scrollbar_thumb_border = self
            .scrollbar_thumb_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let element_hover = self
            .element_hover
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
        let panel_background = self
            .panel_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok());
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
            element_hover,
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
            element_selection_background: self
                .element_selection_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            drop_target_background: self
                .drop_target_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            drop_target_border: self
                .drop_target_border
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
            panel_background,
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
            panel_overlay_background: self
                .panel_overlay_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(panel_background.map(ensure_opaque)),
            panel_overlay_hover: self
                .panel_overlay_hover
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(panel_background
                    .zip(element_hover)
                    .map(|(panel_bg, hover_bg)| panel_bg.blend(hover_bg))
                    .map(ensure_opaque)),
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
            scrollbar_thumb_hover_background,
            scrollbar_thumb_active_background,
            scrollbar_thumb_border,
            scrollbar_track_background: self
                .scrollbar_track_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            scrollbar_track_border: self
                .scrollbar_track_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok()),
            minimap_thumb_background: self
                .minimap_thumb_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(scrollbar_thumb_background.map(ensure_non_opaque)),
            minimap_thumb_hover_background: self
                .minimap_thumb_hover_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(scrollbar_thumb_hover_background.map(ensure_non_opaque)),
            minimap_thumb_active_background: self
                .minimap_thumb_active_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(scrollbar_thumb_active_background.map(ensure_non_opaque)),
            minimap_thumb_border: self
                .minimap_thumb_border
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
                .or(scrollbar_thumb_border),
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
            #[allow(deprecated)]
            version_control_conflict_marker_ours: self
                .version_control_conflict_marker_ours
                .as_ref()
                .or(self.version_control_conflict_ours_background.as_ref())
                .and_then(|color| try_parse_color(color).ok()),
            #[allow(deprecated)]
            version_control_conflict_marker_theirs: self
                .version_control_conflict_marker_theirs
                .as_ref()
                .or(self.version_control_conflict_theirs_background.as_ref())
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

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, JsonSchema_repr, PartialEq)]
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
