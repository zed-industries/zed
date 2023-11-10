use anyhow::Result;
use gpui::{Hsla, Rgba};
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use theme::{
    StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeStylesRefinement,
};

use crate::util::Traverse;
use crate::vscode::VsCodeTheme;
use crate::ThemeMetadata;

use super::ZedSyntaxToken;

pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
    Ok(Rgba::try_from(color)?.into())
}

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
}

impl VsCodeThemeConverter {
    pub fn new(theme: VsCodeTheme, theme_metadata: ThemeMetadata) -> Self {
        Self {
            theme,
            theme_metadata,
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

        Ok(StatusColorsRefinement {
            // conflict: None,
            // created: None,
            deleted: vscode_colors
                .error_foreground
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
            // ignored: None,
            // info: None,
            // modified: None,
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

        Ok(ThemeColorsRefinement {
            border: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_variant: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_focused: vscode_colors
                .focus_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_disabled: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_selected: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_transparent: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            elevated_surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            background: vscode_colors
                .editor_background
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
            drop_target_background: vscode_colors
                .list_drop_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            text: vscode_colors
                .foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?
                .or_else(|| {
                    self.theme
                        .token_colors
                        .iter()
                        .find(|token_color| token_color.scope.is_none())
                        .and_then(|token_color| token_color.settings.foreground.as_ref())
                        .traverse(|color| try_parse_color(&color))
                        .ok()
                        .flatten()
                }),
            tab_active_background: vscode_colors
                .tab_active_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_inactive_background: vscode_colors
                .tab_inactive_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_gutter_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_line_number: vscode_colors
                .editor_line_number_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_active_line_number: vscode_colors
                .editor_foreground
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
            let multimatch_scopes = syntax_token.to_vscode();

            let token_color = self.theme.token_colors.iter().find(|token_color| {
                token_color
                    .scope
                    .as_ref()
                    .map(|scope| scope.multimatch(&multimatch_scopes))
                    .unwrap_or(false)
            });

            let Some(token_color) = token_color else {
                continue;
            };

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

        // let mut highlight_styles = IndexMap::new();

        // for token_color in self.theme.token_colors {
        //     highlight_styles.extend(token_color.highlight_styles()?);
        // }

        // let syntax_theme = UserSyntaxTheme {
        //     highlights: highlight_styles.into_iter().collect(),
        // };

        // pub fn highlight_styles(&self) -> Result<IndexMap<String, UserHighlightStyle>> {
        // let mut highlight_styles = IndexMap::new();

        // for syntax_token in ZedSyntaxToken::iter() {
        //     let scope = syntax_token.to_scope();

        //     // let token_color =
        // }

        // let scope = match self.scope {
        //     Some(VsCodeTokenScope::One(ref scope)) => vec![scope.clone()],
        //     Some(VsCodeTokenScope::Many(ref scopes)) => scopes.clone(),
        //     None => return Ok(IndexMap::new()),
        // };

        // for scope in &scope {
        //     let Some(syntax_token) = Self::to_zed_token(&scope) else {
        //         continue;
        //     };

        //     let highlight_style = UserHighlightStyle {
        //         color: self
        //             .settings
        //             .foreground
        //             .as_ref()
        //             .traverse(|color| try_parse_color(&color))?,
        //         font_style: self
        //             .settings
        //             .font_style
        //             .as_ref()
        //             .and_then(|style| try_parse_font_style(&style)),
        //         font_weight: self
        //             .settings
        //             .font_style
        //             .as_ref()
        //             .and_then(|style| try_parse_font_weight(&style)),
        //     };

        //     if highlight_style.is_empty() {
        //         continue;
        //     }

        //     highlight_styles.insert(syntax_token, highlight_style);
        // }

        // Ok(highlight_styles)
        // }
    }
}
