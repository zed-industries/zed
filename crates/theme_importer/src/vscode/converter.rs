use anyhow::Result;
use gpui::rgba;
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use theme::{
    StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeStylesRefinement,
};

use crate::color::try_parse_color;
use crate::util::Traverse;
use crate::vscode::{VsCodeTheme, VsCodeTokenScope};
use crate::ThemeMetadata;

use super::ZedSyntaxToken;

pub(crate) fn try_parse_font_weight(font_style: &str) -> Option<UserFontWeight> {
    match font_style {
        style if style.contains("bold") => Some(UserFontWeight::BOLD),
        _ => None,
    }
}

pub(crate) fn try_parse_font_style(font_style: &str) -> Option<UserFontStyle> {
    match font_style {
        style if style.contains("italic") => Some(UserFontStyle::Italic),
        style if style.contains("oblique") => Some(UserFontStyle::Oblique),
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

    pub fn convert(self) -> Result<UserTheme> {
        let appearance = self.theme_metadata.appearance.into();

        let status_color_refinements = self.convert_status_colors()?;
        let theme_colors_refinements = self.convert_theme_colors()?;
        let syntax_theme = self.convert_syntax_theme()?;

        Ok(UserTheme {
            name: self.theme_metadata.name.into(),
            appearance,
            styles: UserThemeStylesRefinement {
                colors: theme_colors_refinements,
                status: status_color_refinements,
                syntax: Some(syntax_theme),
            },
        })
    }

    fn convert_status_colors(&self) -> Result<StatusColorsRefinement> {
        let vscode_colors = &self.theme.colors;

        let vscode_base_status_colors = StatusColorsRefinement {
            hint: Some(rgba(0x969696ff).into()),
            ..Default::default()
        };

        Ok(StatusColorsRefinement {
            created: vscode_colors
                .editor_gutter_added_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            modified: vscode_colors
                .editor_gutter_modified_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            deleted: vscode_colors
                .editor_gutter_deleted_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            conflict: vscode_colors
                .git_decoration_conflicting_resource_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            error: vscode_colors
                .error_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            hidden: vscode_colors
                .tab_inactive_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            hint: vscode_colors
                .editor_inlay_hint_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or(vscode_base_status_colors.hint),
            ignored: vscode_colors
                .git_decoration_ignored_resource_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            // info: None,
            // renamed: None,
            // success: None,
            warning: vscode_colors
                .list_warning_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ..Default::default()
        })
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsRefinement> {
        let vscode_colors = &self.theme.colors;

        let vscode_panel_border = vscode_colors
            .panel_border
            .as_ref()
            .traverse(|color| try_parse_color(&color))?;

        let vscode_tab_inactive_background = vscode_colors
            .tab_inactive_background
            .as_ref()
            .traverse(|color| try_parse_color(&color))?;

        let vscode_editor_background = vscode_colors
            .editor_background
            .as_ref()
            .traverse(|color| try_parse_color(&color))?;

        let vscode_scrollbar_slider_background = vscode_colors
            .scrollbar_slider_background
            .as_ref()
            .traverse(|color| try_parse_color(&color))?;

        let vscode_token_colors_foreground = self
            .theme
            .token_colors
            .iter()
            .find(|token_color| token_color.scope.is_none())
            .and_then(|token_color| token_color.settings.foreground.as_ref())
            .traverse(|color| try_parse_color(&color))
            .ok()
            .flatten();

        Ok(ThemeColorsRefinement {
            border: vscode_panel_border,
            border_variant: vscode_panel_border,
            border_focused: vscode_colors
                .focus_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_disabled: vscode_panel_border,
            border_selected: vscode_panel_border,
            border_transparent: vscode_panel_border,
            elevated_surface_background: vscode_colors
                .dropdown_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            background: vscode_editor_background,
            title_bar_background: vscode_colors
                .title_bar_active_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            status_bar_background: vscode_colors
                .status_bar_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_background: vscode_colors
                .button_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_selected: vscode_colors
                .list_active_selection_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ghost_element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ghost_element_selected: vscode_colors
                .list_active_selection_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            drop_target_background: vscode_colors
                .list_drop_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            text: vscode_colors
                .foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or(vscode_token_colors_foreground),
            text_muted: vscode_colors
                .tab_inactive_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_bar_background: vscode_colors
                .editor_group_header_tabs_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_active_background: vscode_colors
                .tab_active_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or(vscode_tab_inactive_background),
            tab_inactive_background: vscode_tab_inactive_background,
            toolbar_background: vscode_colors
                .breadcrumb_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or(vscode_editor_background),
            editor_foreground: vscode_colors
                .foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or(vscode_token_colors_foreground),
            editor_background: vscode_editor_background,
            editor_gutter_background: vscode_editor_background,
            editor_line_number: vscode_colors
                .editor_line_number_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_active_line_number: vscode_colors
                .editor_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            scrollbar_track_background: vscode_editor_background,
            scrollbar_track_border: vscode_colors
                .editor_overview_ruler_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            scrollbar_thumb_background: vscode_scrollbar_slider_background,
            scrollbar_thumb_border: vscode_scrollbar_slider_background,
            scrollbar_thumb_hover_background: vscode_colors
                .scrollbar_slider_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_background: vscode_colors
                .terminal_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_black: vscode_colors
                .terminal_ansi_bright_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_red: vscode_colors
                .terminal_ansi_bright_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_green: vscode_colors
                .terminal_ansi_bright_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_yellow: vscode_colors
                .terminal_ansi_bright_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_blue: vscode_colors
                .terminal_ansi_bright_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_magenta: vscode_colors
                .terminal_ansi_bright_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_cyan: vscode_colors
                .terminal_ansi_bright_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_white: vscode_colors
                .terminal_ansi_bright_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_black: vscode_colors
                .terminal_ansi_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_red: vscode_colors
                .terminal_ansi_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_green: vscode_colors
                .terminal_ansi_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_yellow: vscode_colors
                .terminal_ansi_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_blue: vscode_colors
                .terminal_ansi_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_magenta: vscode_colors
                .terminal_ansi_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_cyan: vscode_colors
                .terminal_ansi_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_white: vscode_colors
                .terminal_ansi_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ..Default::default()
        })
    }

    fn convert_syntax_theme(&self) -> Result<UserSyntaxTheme> {
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

            let highlight_style = UserHighlightStyle {
                color: token_color
                    .settings
                    .foreground
                    .as_ref()
                    .traverse(|color| try_parse_color(&color))?,
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

        Ok(UserSyntaxTheme {
            highlights: highlight_styles.into_iter().collect(),
        })
    }
}
