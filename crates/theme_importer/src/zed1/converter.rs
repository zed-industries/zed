use anyhow::{Context, Result};
use gpui::{Hsla, Rgba};
use theme::{
    color_alpha, Appearance, PlayerColor, PlayerColors, StatusColorsRefinement,
    ThemeColorsRefinement, UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme,
    UserTheme, UserThemeStylesRefinement,
};

use crate::zed1::theme::{
    Color as Zed1Color, ColorScheme, HighlightStyle as Zed1HighlightStyle, Theme as Zed1Theme,
    Weight,
};

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
        font_weight: highlight.weight.map(|weight| match weight {
            Weight::thin => UserFontWeight::THIN,
            Weight::extra_light => UserFontWeight::EXTRA_LIGHT,
            Weight::light => UserFontWeight::LIGHT,
            Weight::normal => UserFontWeight::NORMAL,
            Weight::medium => UserFontWeight::MEDIUM,
            Weight::semibold => UserFontWeight::SEMIBOLD,
            Weight::bold => UserFontWeight::BOLD,
            Weight::extra_bold => UserFontWeight::EXTRA_BOLD,
            Weight::black => UserFontWeight::BLACK,
        }),
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

        let base_theme: ColorScheme = serde_json::from_value(self.theme.base_theme.clone())
            .with_context(|| "failed to parse `theme.base_theme`")?;

        let lowest = &base_theme.lowest;

        let editor = &self.theme.editor;

        Ok(StatusColorsRefinement {
            created: convert(lowest.positive.default.foreground),
            created_background: convert(lowest.positive.default.background),
            created_border: convert(lowest.positive.default.border),
            modified: convert(lowest.warning.default.foreground),
            modified_background: convert(lowest.warning.default.background),
            modified_border: convert(lowest.warning.default.border),
            deleted: convert(lowest.negative.default.foreground),
            deleted_background: convert(lowest.negative.default.background),
            deleted_border: convert(lowest.negative.default.border),
            success: convert(lowest.positive.default.foreground),
            success_background: convert(lowest.positive.default.background),
            success_border: convert(lowest.positive.default.border),
            warning: convert(lowest.warning.default.foreground),
            warning_background: convert(lowest.warning.default.background),
            warning_border: convert(lowest.warning.default.border),
            error: convert(lowest.negative.default.foreground),
            error_background: convert(lowest.negative.default.background),
            error_border: convert(lowest.negative.default.border),
            // The `hint` color used in Zed1 is inlined from the syntax colors.
            hint: editor
                .hint
                .color
                .map(zed1_color_to_hsla)
                .or(convert(lowest.accent.default.foreground)),
            hint_background: convert(lowest.accent.default.background),
            hint_border: convert(lowest.accent.default.border),
            predictive: editor
                .suggestion
                .color
                .map(zed1_color_to_hsla)
                .or(convert(lowest.positive.default.foreground)),
            predictive_background: convert(lowest.positive.default.background),
            predictive_border: convert(lowest.positive.default.border),
            conflict: convert(lowest.warning.default.foreground),
            conflict_background: convert(lowest.warning.default.background),
            conflict_border: convert(lowest.warning.default.border),
            hidden: convert(lowest.base.disabled.foreground),
            hidden_background: convert(lowest.base.disabled.background),
            hidden_border: convert(lowest.base.disabled.border),
            ignored: convert(lowest.variant.default.foreground),
            ignored_background: convert(lowest.variant.default.background),
            ignored_border: convert(lowest.variant.default.border),
            info: convert(lowest.accent.default.foreground),
            info_background: convert(lowest.accent.default.background),
            info_border: convert(lowest.accent.default.border),
            renamed: convert(lowest.accent.default.foreground),
            renamed_background: convert(lowest.accent.default.background),
            renamed_border: convert(lowest.accent.default.border),
            unreachable: convert(lowest.variant.default.foreground), // TODO: Should this be transparent?
            unreachable_background: convert(lowest.variant.default.background),
            unreachable_border: convert(lowest.variant.default.border),
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
            border_variant: convert(middle.variant.default.border),
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
            text: convert(lowest.base.default.foreground),
            text_muted: convert(lowest.variant.default.foreground),
            text_placeholder: convert(lowest.base.disabled.foreground), // TODO: What should placeholder be?
            text_disabled: convert(lowest.base.disabled.foreground),
            text_accent: convert(lowest.accent.default.foreground),
            icon: convert(lowest.base.default.foreground),
            icon_muted: convert(lowest.variant.default.foreground),
            icon_disabled: convert(lowest.base.disabled.foreground),
            icon_placeholder: convert(lowest.variant.default.foreground),
            icon_accent: convert(lowest.accent.default.foreground),
            status_bar_background: convert(lowest.base.default.background),
            title_bar_background: convert(lowest.base.default.background),
            toolbar_background: convert(highest.base.default.background),
            tab_bar_background: convert(middle.base.default.background),
            tab_inactive_background: convert(middle.base.default.background),
            tab_active_background: convert(highest.base.default.background),
            search_match_background: convert(highest.accent.default.background),
            panel_background: convert(middle.base.default.background),
            panel_focused_border: convert(lowest.accent.hovered.border),
            pane_focused_border: convert(lowest.accent.hovered.border),
            scrollbar_thumb_background: convert(middle.base.inverted.background)
                .map(|color| color_alpha(color, 0.3)),
            scrollbar_thumb_hover_background: convert(middle.base.hovered.background),
            scrollbar_thumb_border: convert(middle.base.default.border),
            scrollbar_track_background: Some(gpui::transparent_black()),
            scrollbar_track_border: convert(highest.variant.default.border),
            editor_foreground: convert(editor.text_color),
            editor_background: convert(editor.background),
            editor_gutter_background: convert(editor.gutter_background),
            editor_subheader_background: convert(middle.base.default.background),
            editor_active_line_background: convert(editor.active_line_background),
            editor_highlighted_line_background: convert(editor.highlighted_line_background),
            editor_line_number: convert(editor.line_number),
            editor_active_line_number: convert(editor.line_number_active),
            editor_invisible: convert(editor.whitespace),
            editor_wrap_guide: convert(editor.wrap_guide),
            editor_active_wrap_guide: convert(editor.active_wrap_guide),
            editor_document_highlight_read_background: convert(
                editor.document_highlight_read_background,
            ),
            editor_document_highlight_write_background: convert(
                editor.document_highlight_write_background,
            ),
            terminal_background: convert(terminal.background),
            terminal_foreground: convert(terminal.foreground),
            terminal_bright_foreground: convert(terminal.bright_foreground),
            terminal_dim_foreground: convert(terminal.dim_foreground),
            terminal_ansi_black: convert(terminal.black),
            terminal_ansi_bright_black: convert(terminal.bright_black),
            terminal_ansi_dim_black: convert(terminal.dim_black),
            terminal_ansi_red: convert(terminal.red),
            terminal_ansi_bright_red: convert(terminal.bright_red),
            terminal_ansi_dim_red: convert(terminal.dim_red),
            terminal_ansi_green: convert(terminal.green),
            terminal_ansi_bright_green: convert(terminal.bright_green),
            terminal_ansi_dim_green: convert(terminal.dim_green),
            terminal_ansi_yellow: convert(terminal.yellow),
            terminal_ansi_bright_yellow: convert(terminal.bright_yellow),
            terminal_ansi_dim_yellow: convert(terminal.dim_yellow),
            terminal_ansi_blue: convert(terminal.blue),
            terminal_ansi_bright_blue: convert(terminal.bright_blue),
            terminal_ansi_dim_blue: convert(terminal.dim_blue),
            terminal_ansi_magenta: convert(terminal.magenta),
            terminal_ansi_bright_magenta: convert(terminal.bright_magenta),
            terminal_ansi_dim_magenta: convert(terminal.dim_magenta),
            terminal_ansi_cyan: convert(terminal.cyan),
            terminal_ansi_bright_cyan: convert(terminal.bright_cyan),
            terminal_ansi_dim_cyan: convert(terminal.dim_cyan),
            terminal_ansi_white: convert(terminal.white),
            terminal_ansi_bright_white: convert(terminal.bright_white),
            terminal_ansi_dim_white: convert(terminal.dim_white),
            link_text_hover: convert(highest.accent.default.foreground),
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
