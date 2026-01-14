#![allow(missing_docs)]

use gpui::{FontStyle, FontWeight, HighlightStyle, Hsla};
use palette::FromColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::{FontWeightContent, WindowBackgroundContent};

use crate::{StatusColorsRefinement, ThemeColorsRefinement};

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
    pub style: settings::ThemeStyleContent,
}

/// Returns the syntax style overrides in the [`ThemeContent`].
pub fn syntax_overrides(this: &settings::ThemeStyleContent) -> Vec<(String, HighlightStyle)> {
    this.syntax
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

pub fn status_colors_refinement(colors: &settings::StatusColorsContent) -> StatusColorsRefinement {
    StatusColorsRefinement {
        conflict: colors
            .conflict
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        conflict_background: colors
            .conflict_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        conflict_border: colors
            .conflict_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        created: colors
            .created
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        created_background: colors
            .created_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        created_border: colors
            .created_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        deleted: colors
            .deleted
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        deleted_background: colors
            .deleted_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        deleted_border: colors
            .deleted_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        error: colors
            .error
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        error_background: colors
            .error_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        error_border: colors
            .error_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hidden: colors
            .hidden
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hidden_background: colors
            .hidden_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hidden_border: colors
            .hidden_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hint: colors
            .hint
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hint_background: colors
            .hint_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        hint_border: colors
            .hint_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ignored: colors
            .ignored
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ignored_background: colors
            .ignored_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ignored_border: colors
            .ignored_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        info: colors
            .info
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        info_background: colors
            .info_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        info_border: colors
            .info_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        modified: colors
            .modified
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        modified_background: colors
            .modified_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        modified_border: colors
            .modified_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        predictive: colors
            .predictive
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        predictive_background: colors
            .predictive_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        predictive_border: colors
            .predictive_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        renamed: colors
            .renamed
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        renamed_background: colors
            .renamed_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        renamed_border: colors
            .renamed_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        success: colors
            .success
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        success_background: colors
            .success_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        success_border: colors
            .success_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        unreachable: colors
            .unreachable
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        unreachable_background: colors
            .unreachable_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        unreachable_border: colors
            .unreachable_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        warning: colors
            .warning
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        warning_background: colors
            .warning_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        warning_border: colors
            .warning_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
    }
}

pub fn theme_colors_refinement(
    this: &settings::ThemeColorsContent,
    status_colors: &StatusColorsRefinement,
) -> ThemeColorsRefinement {
    let border = this
        .border
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let editor_document_highlight_read_background = this
        .editor_document_highlight_read_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let scrollbar_thumb_background = this
        .scrollbar_thumb_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok())
        .or_else(|| {
            this.deprecated_scrollbar_thumb_background
                .as_ref()
                .and_then(|color| try_parse_color(color).ok())
        });
    let scrollbar_thumb_hover_background = this
        .scrollbar_thumb_hover_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let scrollbar_thumb_active_background = this
        .scrollbar_thumb_active_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok())
        .or(scrollbar_thumb_background);
    let scrollbar_thumb_border = this
        .scrollbar_thumb_border
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let element_hover = this
        .element_hover
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let panel_background = this
        .panel_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let search_match_background = this
        .search_match_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok());
    let search_active_match_background = this
        .search_active_match_background
        .as_ref()
        .and_then(|color| try_parse_color(color).ok())
        .or(search_match_background);
    ThemeColorsRefinement {
        border,
        border_variant: this
            .border_variant
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        border_focused: this
            .border_focused
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        border_selected: this
            .border_selected
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        border_transparent: this
            .border_transparent
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        border_disabled: this
            .border_disabled
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        elevated_surface_background: this
            .elevated_surface_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        surface_background: this
            .surface_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        background: this
            .background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        element_background: this
            .element_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        element_hover,
        element_active: this
            .element_active
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        element_selected: this
            .element_selected
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        element_disabled: this
            .element_disabled
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        element_selection_background: this
            .element_selection_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        drop_target_background: this
            .drop_target_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        drop_target_border: this
            .drop_target_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ghost_element_background: this
            .ghost_element_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ghost_element_hover: this
            .ghost_element_hover
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ghost_element_active: this
            .ghost_element_active
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ghost_element_selected: this
            .ghost_element_selected
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        ghost_element_disabled: this
            .ghost_element_disabled
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        text: this
            .text
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        text_muted: this
            .text_muted
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        text_placeholder: this
            .text_placeholder
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        text_disabled: this
            .text_disabled
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        text_accent: this
            .text_accent
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        icon: this
            .icon
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        icon_muted: this
            .icon_muted
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        icon_disabled: this
            .icon_disabled
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        icon_placeholder: this
            .icon_placeholder
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        icon_accent: this
            .icon_accent
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        debugger_accent: this
            .debugger_accent
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        status_bar_background: this
            .status_bar_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        title_bar_background: this
            .title_bar_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        title_bar_inactive_background: this
            .title_bar_inactive_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        toolbar_background: this
            .toolbar_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        tab_bar_background: this
            .tab_bar_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        tab_inactive_background: this
            .tab_inactive_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        tab_active_background: this
            .tab_active_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        search_match_background: search_match_background,
        search_active_match_background: search_active_match_background,
        panel_background,
        panel_focused_border: this
            .panel_focused_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        panel_indent_guide: this
            .panel_indent_guide
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        panel_indent_guide_hover: this
            .panel_indent_guide_hover
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        panel_indent_guide_active: this
            .panel_indent_guide_active
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        panel_overlay_background: this
            .panel_overlay_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(panel_background.map(ensure_opaque)),
        panel_overlay_hover: this
            .panel_overlay_hover
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(panel_background
                .zip(element_hover)
                .map(|(panel_bg, hover_bg)| panel_bg.blend(hover_bg))
                .map(ensure_opaque)),
        pane_focused_border: this
            .pane_focused_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        pane_group_border: this
            .pane_group_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(border),
        scrollbar_thumb_background,
        scrollbar_thumb_hover_background,
        scrollbar_thumb_active_background,
        scrollbar_thumb_border,
        scrollbar_track_background: this
            .scrollbar_track_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        scrollbar_track_border: this
            .scrollbar_track_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        minimap_thumb_background: this
            .minimap_thumb_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(scrollbar_thumb_background.map(ensure_non_opaque)),
        minimap_thumb_hover_background: this
            .minimap_thumb_hover_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(scrollbar_thumb_hover_background.map(ensure_non_opaque)),
        minimap_thumb_active_background: this
            .minimap_thumb_active_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(scrollbar_thumb_active_background.map(ensure_non_opaque)),
        minimap_thumb_border: this
            .minimap_thumb_border
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            .or(scrollbar_thumb_border),
        editor_foreground: this
            .editor_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_background: this
            .editor_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_gutter_background: this
            .editor_gutter_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_subheader_background: this
            .editor_subheader_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_active_line_background: this
            .editor_active_line_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_highlighted_line_background: this
            .editor_highlighted_line_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_debugger_active_line_background: this
            .editor_debugger_active_line_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_line_number: this
            .editor_line_number
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_hover_line_number: this
            .editor_hover_line_number
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_active_line_number: this
            .editor_active_line_number
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_invisible: this
            .editor_invisible
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_wrap_guide: this
            .editor_wrap_guide
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_active_wrap_guide: this
            .editor_active_wrap_guide
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_indent_guide: this
            .editor_indent_guide
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_indent_guide_active: this
            .editor_indent_guide_active
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_document_highlight_read_background,
        editor_document_highlight_write_background: this
            .editor_document_highlight_write_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        editor_document_highlight_bracket_background: this
            .editor_document_highlight_bracket_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `editor.document_highlight.read_background`, for backwards compatibility.
            .or(editor_document_highlight_read_background),
        terminal_background: this
            .terminal_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_background: this
            .terminal_ansi_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_foreground: this
            .terminal_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_bright_foreground: this
            .terminal_bright_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_dim_foreground: this
            .terminal_dim_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_black: this
            .terminal_ansi_black
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_black: this
            .terminal_ansi_bright_black
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_black: this
            .terminal_ansi_dim_black
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_red: this
            .terminal_ansi_red
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_red: this
            .terminal_ansi_bright_red
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_red: this
            .terminal_ansi_dim_red
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_green: this
            .terminal_ansi_green
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_green: this
            .terminal_ansi_bright_green
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_green: this
            .terminal_ansi_dim_green
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_yellow: this
            .terminal_ansi_yellow
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_yellow: this
            .terminal_ansi_bright_yellow
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_yellow: this
            .terminal_ansi_dim_yellow
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_blue: this
            .terminal_ansi_blue
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_blue: this
            .terminal_ansi_bright_blue
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_blue: this
            .terminal_ansi_dim_blue
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_magenta: this
            .terminal_ansi_magenta
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_magenta: this
            .terminal_ansi_bright_magenta
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_magenta: this
            .terminal_ansi_dim_magenta
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_cyan: this
            .terminal_ansi_cyan
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_cyan: this
            .terminal_ansi_bright_cyan
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_cyan: this
            .terminal_ansi_dim_cyan
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_white: this
            .terminal_ansi_white
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_bright_white: this
            .terminal_ansi_bright_white
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        terminal_ansi_dim_white: this
            .terminal_ansi_dim_white
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        link_text_hover: this
            .link_text_hover
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        version_control_added: this
            .version_control_added
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `created`, for backwards compatibility.
            .or(status_colors.created),
        version_control_deleted: this
            .version_control_deleted
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `deleted`, for backwards compatibility.
            .or(status_colors.deleted),
        version_control_modified: this
            .version_control_modified
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `modified`, for backwards compatibility.
            .or(status_colors.modified),
        version_control_renamed: this
            .version_control_renamed
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `modified`, for backwards compatibility.
            .or(status_colors.modified),
        version_control_conflict: this
            .version_control_conflict
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `ignored`, for backwards compatibility.
            .or(status_colors.ignored),
        version_control_ignored: this
            .version_control_ignored
            .as_ref()
            .and_then(|color| try_parse_color(color).ok())
            // Fall back to `conflict`, for backwards compatibility.
            .or(status_colors.ignored),
        version_control_word_added: this
            .version_control_word_added
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        version_control_word_deleted: this
            .version_control_word_deleted
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        #[allow(deprecated)]
        version_control_conflict_marker_ours: this
            .version_control_conflict_marker_ours
            .as_ref()
            .or(this.version_control_conflict_ours_background.as_ref())
            .and_then(|color| try_parse_color(color).ok()),
        #[allow(deprecated)]
        version_control_conflict_marker_theirs: this
            .version_control_conflict_marker_theirs
            .as_ref()
            .or(this.version_control_conflict_theirs_background.as_ref())
            .and_then(|color| try_parse_color(color).ok()),
        vim_normal_background: this
            .vim_normal_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_insert_background: this
            .vim_insert_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_replace_background: this
            .vim_replace_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_background: this
            .vim_visual_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_line_background: this
            .vim_visual_line_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_block_background: this
            .vim_visual_block_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_helix_normal_background: this
            .vim_helix_normal_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_helix_select_background: this
            .vim_helix_select_background
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_normal_foreground: this
            .vim_normal_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_insert_foreground: this
            .vim_insert_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_replace_foreground: this
            .vim_replace_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_foreground: this
            .vim_visual_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_line_foreground: this
            .vim_visual_line_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_visual_block_foreground: this
            .vim_visual_block_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_helix_normal_foreground: this
            .vim_helix_normal_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
        vim_helix_select_foreground: this
            .vim_helix_select_foreground
            .as_ref()
            .and_then(|color| try_parse_color(color).ok()),
    }
}

pub(crate) fn try_parse_color(color: &str) -> anyhow::Result<Hsla> {
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
