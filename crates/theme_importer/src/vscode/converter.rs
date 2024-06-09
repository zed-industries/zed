use anyhow::Result;
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use theme::{
    FontStyleContent, FontWeightContent, HighlightStyleContent, StatusColorsContent,
    ThemeColorsContent, ThemeContent, ThemeStyleContent,
};

use crate::vscode::{VsCodeTheme, VsCodeTokenScope};
use crate::ThemeMetadata;

use super::ZedSyntaxToken;

pub(crate) fn try_parse_font_weight(font_style: &str) -> Option<FontWeightContent> {
    match font_style {
        style if style.contains("bold") => Some(FontWeightContent::Bold),
        _ => None,
    }
}

pub(crate) fn try_parse_font_style(font_style: &str) -> Option<FontStyleContent> {
    match font_style {
        style if style.contains("italic") => Some(FontStyleContent::Italic),
        style if style.contains("oblique") => Some(FontStyleContent::Oblique),
        _ => None,
    }
}

pub struct VsCodeThemeConverter {
    theme: VsCodeTheme,
    theme_metadata: ThemeMetadata,
    syntax_overrides: IndexMap<String, Vec<String>>,
}

impl VsCodeThemeConverter {
    pub fn new(
        theme: VsCodeTheme,
        theme_metadata: ThemeMetadata,
        syntax_overrides: IndexMap<String, Vec<String>>,
    ) -> Self {
        Self {
            theme,
            theme_metadata,
            syntax_overrides,
        }
    }

    pub fn convert(self) -> Result<ThemeContent> {
        let appearance = self.theme_metadata.appearance.into();

        let status_colors = self.convert_status_colors()?;
        let theme_colors = self.convert_theme_colors()?;
        let syntax_theme = self.convert_syntax_theme()?;

        Ok(ThemeContent {
            name: self.theme_metadata.name,
            appearance,
            style: ThemeStyleContent {
                window_background_appearance: Some(theme::WindowBackgroundContent::Opaque),
                accents: Vec::new(), //TODO can we read this from the theme?
                colors: theme_colors,
                status: status_colors,
                players: Vec::new(),
                syntax: syntax_theme,
            },
        })
    }

    fn convert_status_colors(&self) -> Result<StatusColorsContent> {
        let vscode_colors = &self.theme.colors;

        let vscode_base_status_colors = StatusColorsContent {
            hint: Some("#969696ff".to_string()),
            ..Default::default()
        };

        Ok(StatusColorsContent {
            conflict: vscode_colors
                .git_decoration
                .conflicting_resource_foreground
                .clone(),
            created: vscode_colors.editor_gutter.added_background.clone(),
            deleted: vscode_colors.editor_gutter.deleted_background.clone(),
            error: vscode_colors.editor_error.foreground.clone(),
            error_background: vscode_colors.editor_error.background.clone(),
            error_border: vscode_colors.editor_error.border.clone(),
            hidden: vscode_colors.tab.inactive_foreground.clone(),
            hint: vscode_colors
                .editor_inlay_hint
                .foreground
                .clone()
                .or(vscode_base_status_colors.hint),
            hint_border: vscode_colors.editor_hint.border.clone(),
            ignored: vscode_colors
                .git_decoration
                .ignored_resource_foreground
                .clone(),
            info: vscode_colors.editor_info.foreground.clone(),
            info_background: vscode_colors.editor_info.background.clone(),
            info_border: vscode_colors.editor_info.border.clone(),
            modified: vscode_colors.editor_gutter.modified_background.clone(),
            // renamed: None,
            // success: None,
            warning: vscode_colors.editor_warning.foreground.clone(),
            warning_background: vscode_colors.editor_warning.background.clone(),
            warning_border: vscode_colors.editor_warning.border.clone(),
            ..Default::default()
        })
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsContent> {
        let vscode_colors = &self.theme.colors;

        let vscode_panel_border = vscode_colors.panel.border.clone();
        let vscode_tab_inactive_background = vscode_colors.tab.inactive_background.clone();
        let vscode_editor_foreground = vscode_colors.editor.foreground.clone();
        let vscode_editor_background = vscode_colors.editor.background.clone();
        let vscode_scrollbar_slider_background = vscode_colors.scrollbar_slider.background.clone();
        let vscode_token_colors_foreground = self
            .theme
            .token_colors
            .iter()
            .find(|token_color| token_color.scope.is_none())
            .and_then(|token_color| token_color.settings.foreground.as_ref())
            .cloned();

        Ok(ThemeColorsContent {
            border: vscode_panel_border.clone(),
            border_variant: vscode_panel_border.clone(),
            border_focused: vscode_colors.focus_border.clone(),
            border_selected: vscode_panel_border.clone(),
            border_transparent: vscode_panel_border.clone(),
            border_disabled: vscode_panel_border.clone(),
            elevated_surface_background: vscode_colors.dropdown.background.clone(),
            surface_background: vscode_colors.panel.background.clone(),
            background: vscode_editor_background.clone(),
            element_background: vscode_colors.button.background.clone(),
            element_hover: vscode_colors.list.hover_background.clone(),
            element_selected: vscode_colors.list.active_selection_background.clone(),
            drop_target_background: vscode_colors.list.drop_background.clone(),
            ghost_element_hover: vscode_colors.list.hover_background.clone(),
            ghost_element_selected: vscode_colors.list.active_selection_background.clone(),
            text: vscode_colors
                .foreground
                .clone()
                .or(vscode_token_colors_foreground.clone()),
            text_muted: vscode_colors.tab.inactive_foreground.clone(),
            status_bar_background: vscode_colors.status_bar.background.clone(),
            title_bar_background: vscode_colors.title_bar.active_background.clone(),
            toolbar_background: vscode_colors
                .breadcrumb
                .background
                .clone()
                .or(vscode_editor_background.clone()),
            tab_bar_background: vscode_colors.editor_group_header.tabs_background.clone(),
            tab_inactive_background: vscode_tab_inactive_background.clone(),
            tab_active_background: vscode_colors
                .tab
                .active_background
                .clone()
                .or(vscode_tab_inactive_background.clone()),
            panel_background: vscode_colors.panel.background.clone(),
            scrollbar_thumb_background: vscode_scrollbar_slider_background.clone(),
            scrollbar_thumb_hover_background: vscode_colors
                .scrollbar_slider
                .hover_background
                .clone(),
            scrollbar_thumb_border: vscode_scrollbar_slider_background.clone(),
            scrollbar_track_background: vscode_editor_background.clone(),
            scrollbar_track_border: vscode_colors.editor_overview_ruler.border.clone(),
            pane_group_border: vscode_colors.editor_group.border.clone(),
            editor_foreground: vscode_editor_foreground
                .clone()
                .or(vscode_token_colors_foreground.clone()),
            editor_background: vscode_editor_background.clone(),
            editor_gutter_background: vscode_editor_background.clone(),
            editor_active_line_background: vscode_colors.editor.line_highlight_background.clone(),
            editor_line_number: vscode_colors.editor_line_number.foreground.clone(),
            editor_active_line_number: vscode_colors.editor.foreground.clone(),
            editor_wrap_guide: vscode_panel_border.clone(),
            editor_active_wrap_guide: vscode_panel_border.clone(),
            terminal_background: vscode_colors.terminal.background.clone(),
            terminal_ansi_black: vscode_colors.terminal.ansi_black.clone(),
            terminal_ansi_bright_black: vscode_colors.terminal.ansi_bright_black.clone(),
            terminal_ansi_red: vscode_colors.terminal.ansi_red.clone(),
            terminal_ansi_bright_red: vscode_colors.terminal.ansi_bright_red.clone(),
            terminal_ansi_green: vscode_colors.terminal.ansi_green.clone(),
            terminal_ansi_bright_green: vscode_colors.terminal.ansi_bright_green.clone(),
            terminal_ansi_yellow: vscode_colors.terminal.ansi_yellow.clone(),
            terminal_ansi_bright_yellow: vscode_colors.terminal.ansi_bright_yellow.clone(),
            terminal_ansi_blue: vscode_colors.terminal.ansi_blue.clone(),
            terminal_ansi_bright_blue: vscode_colors.terminal.ansi_bright_blue.clone(),
            terminal_ansi_magenta: vscode_colors.terminal.ansi_magenta.clone(),
            terminal_ansi_bright_magenta: vscode_colors.terminal.ansi_bright_magenta.clone(),
            terminal_ansi_cyan: vscode_colors.terminal.ansi_cyan.clone(),
            terminal_ansi_bright_cyan: vscode_colors.terminal.ansi_bright_cyan.clone(),
            terminal_ansi_white: vscode_colors.terminal.ansi_white.clone(),
            terminal_ansi_bright_white: vscode_colors.terminal.ansi_bright_white.clone(),
            link_text_hover: vscode_colors.text_link.active_foreground.clone(),
            ..Default::default()
        })
    }

    fn convert_syntax_theme(&self) -> Result<IndexMap<String, HighlightStyleContent>> {
        let mut highlight_styles = IndexMap::new();

        for syntax_token in ZedSyntaxToken::iter() {
            let override_match = self
                .syntax_overrides
                .get(&syntax_token.to_string())
                .and_then(|scope| {
                    self.theme.token_colors.iter().find(|token_color| {
                        token_color.scope == Some(VsCodeTokenScope::Many(scope.clone()))
                    })
                });

            let best_match = override_match
                .or_else(|| syntax_token.find_best_token_color_match(&self.theme.token_colors))
                .or_else(|| {
                    syntax_token.fallbacks().iter().find_map(|fallback| {
                        fallback.find_best_token_color_match(&self.theme.token_colors)
                    })
                });

            let Some(token_color) = best_match else {
                log::warn!("No matching token color found for '{syntax_token}'");
                continue;
            };

            log::info!(
                "Matched '{syntax_token}' to '{}'",
                token_color
                    .name
                    .clone()
                    .or_else(|| token_color
                        .scope
                        .as_ref()
                        .map(|scope| format!("{:?}", scope)))
                    .unwrap_or_else(|| "no identifier".to_string())
            );

            let highlight_style = HighlightStyleContent {
                color: token_color.settings.foreground.clone(),
                background_color: token_color.settings.background.clone(),
                font_style: token_color
                    .settings
                    .font_style
                    .as_ref()
                    .and_then(|style| try_parse_font_style(&style)),
                font_weight: token_color
                    .settings
                    .font_style
                    .as_ref()
                    .and_then(|style| try_parse_font_weight(&style)),
            };

            if highlight_style.is_empty() {
                continue;
            }

            highlight_styles.insert(syntax_token.to_string(), highlight_style);
        }

        Ok(highlight_styles)
    }
}
