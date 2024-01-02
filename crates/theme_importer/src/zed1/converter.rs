use anyhow::{Context, Result};
use gpui::{serde_json, Hsla, Rgba};
use gpui1::color::Color as Zed1Color;
use gpui1::fonts::HighlightStyle as Zed1HighlightStyle;
use theme::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme,
    UserThemeStylesRefinement,
};
use theme1::{ColorScheme, Theme as Zed1Theme};

fn zed1_color_to_hsla(color: Zed1Color) -> Hsla {
    let r = color.r as f32 / 255.;
    let g = color.g as f32 / 255.;
    let b = color.b as f32 / 255.;
    let a = color.a as f32 / 255.;

    Hsla::from(Rgba { r, g, b, a })
}

fn zed1_highlight_style_to_user_highlight_style(
    highlight: Zed1HighlightStyle,
) -> UserHighlightStyle {
    UserHighlightStyle {
        color: highlight.color.map(zed1_color_to_hsla),
        font_style: highlight.italic.map(|is_italic| {
            if is_italic {
                UserFontStyle::Italic
            } else {
                UserFontStyle::Normal
            }
        }),
        font_weight: highlight.weight.map(|weight| UserFontWeight(weight.0)),
    }
}

pub struct Zed1ThemeConverter {
    theme: Zed1Theme,
}

impl Zed1ThemeConverter {
    pub fn new(theme: Zed1Theme) -> Self {
        Self { theme }
    }

    pub fn convert(self) -> Result<UserTheme> {
        let appearance = match self.theme.meta.is_light {
            true => Appearance::Light,
            false => Appearance::Dark,
        };

        let status_colors_refinement = self.convert_status_colors()?;
        let theme_colors_refinement = self.convert_theme_colors()?;
        let player_colors = self.convert_player_colors()?;
        let syntax_theme = self.convert_syntax_theme()?;

        Ok(UserTheme {
            name: self.theme.meta.name,
            appearance,
            styles: UserThemeStylesRefinement {
                colors: theme_colors_refinement,
                status: status_colors_refinement,
                player: Some(player_colors),
                syntax: Some(syntax_theme),
            },
        })
    }

    fn convert_status_colors(&self) -> Result<StatusColorsRefinement> {
        fn convert(color: Zed1Color) -> Option<Hsla> {
            Some(zed1_color_to_hsla(color))
        }

        let editor = &self.theme.editor;
        let diff_style = &self.theme.editor.diff;
        let diagnostic_summary = &self.theme.workspace.status_bar.diagnostic_summary;

        Ok(StatusColorsRefinement {
            created: convert(diff_style.inserted),
            modified: convert(diff_style.modified),
            deleted: convert(diff_style.deleted),
            success: convert(diagnostic_summary.icon_color_ok),
            warning: convert(diagnostic_summary.icon_color_warning),
            error: convert(diagnostic_summary.icon_color_error),
            hint: editor.hint.color.map(zed1_color_to_hsla),
            predictive: editor.suggestion.color.map(zed1_color_to_hsla),
            ..Default::default()
        })
    }

    fn convert_player_colors(&self) -> Result<PlayerColors> {
        let player_one = self.theme.editor.selection;

        let mut player_colors = vec![PlayerColor {
            cursor: zed1_color_to_hsla(player_one.cursor),
            selection: zed1_color_to_hsla(player_one.selection),
            background: zed1_color_to_hsla(player_one.cursor),
        }];

        for index in 1..8 {
            let player = self
                .theme
                .editor
                .selection_style_for_room_participant(index);

            player_colors.push(PlayerColor {
                cursor: zed1_color_to_hsla(player.cursor),
                selection: zed1_color_to_hsla(player.selection),
                background: zed1_color_to_hsla(player.cursor),
            });
        }

        Ok(PlayerColors(player_colors))
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsRefinement> {
        fn convert(color: Zed1Color) -> Option<Hsla> {
            Some(zed1_color_to_hsla(color))
        }

        let base_theme: ColorScheme = serde_json::from_value(self.theme.base_theme.clone())
            .with_context(|| "failed to parse `theme.base_theme`")?;

        let lowest = &base_theme.lowest;
        let middle = &base_theme.middle;
        let highest = &base_theme.highest;

        let editor = &self.theme.editor;
        let terminal = &self.theme.terminal;

        Ok(ThemeColorsRefinement {
            border: convert(lowest.base.default.border),
            border_variant: convert(lowest.variant.default.border),
            border_focused: convert(lowest.accent.hovered.border),
            border_selected: convert(lowest.accent.default.border),
            border_transparent: Some(gpui::transparent_black()),
            border_disabled: convert(lowest.base.disabled.border),
            elevated_surface_background: convert(middle.base.default.background),
            surface_background: convert(middle.base.default.background),
            background: convert(lowest.base.default.background),
            element_background: convert(lowest.on.default.background),
            element_hover: convert(lowest.on.hovered.background),
            element_active: convert(lowest.on.active.background),
            element_selected: convert(lowest.on.active.background), // TODO: Check what this should be
            element_disabled: convert(lowest.on.disabled.background),
            drop_target_background: convert(self.theme.workspace.drop_target_overlay_color),
            ghost_element_background: Some(gpui::transparent_black()),
            ghost_element_hover: convert(lowest.on.hovered.background),
            ghost_element_active: convert(lowest.on.active.background),
            ghost_element_selected: convert(lowest.on.active.background), // TODO: Check what this should be
            ghost_element_disabled: convert(lowest.on.disabled.background),
            icon: convert(lowest.base.default.foreground),
            icon_muted: convert(lowest.variant.default.foreground),
            icon_placeholder: convert(lowest.variant.default.foreground), // TODO: What should placeholder be?
            icon_disabled: convert(lowest.base.disabled.foreground),
            icon_accent: convert(lowest.accent.default.foreground),
            text: convert(lowest.base.default.foreground),
            text_muted: convert(lowest.variant.default.foreground),
            text_placeholder: convert(lowest.base.disabled.foreground),
            text_disabled: convert(lowest.base.disabled.foreground),
            text_accent: convert(lowest.accent.default.foreground),
            status_bar_background: convert(lowest.base.default.background),
            title_bar_background: convert(lowest.base.default.background),
            toolbar_background: convert(highest.base.default.background),
            tab_bar_background: convert(middle.base.default.background),
            tab_inactive_background: convert(middle.base.default.background),
            tab_active_background: convert(highest.base.default.background),
            panel_background: convert(middle.base.default.background),
            scrollbar_thumb_background: convert(middle.base.default.background),
            scrollbar_thumb_hover_background: convert(middle.base.hovered.background),
            scrollbar_thumb_border: convert(middle.base.default.border),
            scrollbar_track_background: convert(highest.base.default.background),
            scrollbar_track_border: convert(highest.variant.default.border),
            editor_foreground: convert(editor.text_color),
            editor_background: convert(editor.background),
            editor_gutter_background: convert(editor.gutter_background),
            editor_line_number: convert(editor.line_number),
            editor_active_line_number: convert(editor.line_number_active),
            editor_wrap_guide: convert(editor.wrap_guide),
            editor_active_wrap_guide: convert(editor.active_wrap_guide),
            terminal_background: convert(terminal.background),
            terminal_ansi_bright_black: convert(terminal.bright_black),
            terminal_ansi_bright_red: convert(terminal.bright_red),
            terminal_ansi_bright_green: convert(terminal.bright_green),
            terminal_ansi_bright_yellow: convert(terminal.bright_yellow),
            terminal_ansi_bright_blue: convert(terminal.bright_blue),
            terminal_ansi_bright_magenta: convert(terminal.bright_magenta),
            terminal_ansi_bright_cyan: convert(terminal.bright_cyan),
            terminal_ansi_bright_white: convert(terminal.bright_white),
            terminal_ansi_black: convert(terminal.black),
            terminal_ansi_red: convert(terminal.red),
            terminal_ansi_green: convert(terminal.green),
            terminal_ansi_yellow: convert(terminal.yellow),
            terminal_ansi_blue: convert(terminal.blue),
            terminal_ansi_magenta: convert(terminal.magenta),
            terminal_ansi_cyan: convert(terminal.cyan),
            terminal_ansi_white: convert(terminal.white),
            ..Default::default()
        })
    }

    fn convert_syntax_theme(&self) -> Result<UserSyntaxTheme> {
        Ok(UserSyntaxTheme {
            highlights: self
                .theme
                .editor
                .syntax
                .highlights
                .clone()
                .into_iter()
                .map(|(name, highlight_style)| {
                    (
                        name,
                        zed1_highlight_style_to_user_highlight_style(highlight_style),
                    )
                })
                .collect(),
        })
    }
}
