use anyhow::Result;
use gpui::{Hsla, Rgba};
use gpui1::color::Color as Zed1Color;
use gpui1::fonts::HighlightStyle as Zed1HighlightStyle;
use theme::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme,
    UserThemeStylesRefinement,
};
use theme1::Theme as Zed1Theme;

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

        let picker = &self.theme.picker;
        let title_bar = &self.theme.titlebar;
        let status_bar = &self.theme.workspace.status_bar;
        let project_panel = &self.theme.project_panel;
        let tab_bar = &self.theme.workspace.tab_bar;
        let active_tab = &self.theme.workspace.tab_bar.tab_style(true, true);
        let inactive_tab = &self.theme.workspace.tab_bar.tab_style(true, false);
        let toolbar = &self.theme.workspace.toolbar;
        let editor = &self.theme.editor;
        let scrollbar = &self.theme.editor.scrollbar;
        let terminal = &self.theme.terminal;

        Ok(ThemeColorsRefinement {
            border: convert(active_tab.container.border.color),
            border_variant: convert(toolbar.container.border.color),
            background: convert(self.theme.workspace.background),
            elevated_surface_background: editor
                .hover_popover
                .container
                .background_color
                .map(zed1_color_to_hsla),
            title_bar_background: title_bar.container.background_color.map(zed1_color_to_hsla),
            status_bar_background: status_bar
                .container
                .background_color
                .map(zed1_color_to_hsla)
                .or_else(|| title_bar.container.background_color.map(zed1_color_to_hsla)),
            panel_background: project_panel
                .container
                .background_color
                .map(zed1_color_to_hsla),
            text: convert(self.theme.collab_panel.channel_name.text.color),
            text_muted: convert(tab_bar.pane_button.default_style().color),
            text_accent: convert(status_bar.panel_buttons.button.active_state().icon_color),
            text_disabled: convert(status_bar.panel_buttons.button.disabled_style().icon_color),
            text_placeholder: picker
                .empty_input_editor
                .placeholder_text
                .as_ref()
                .map(|placeholder_text| placeholder_text.color)
                .map(zed1_color_to_hsla),
            element_hover: picker
                .item
                .hovered
                .as_ref()
                .and_then(|hovered| hovered.container.background_color)
                .map(zed1_color_to_hsla),
            element_selected: picker
                .item
                .active_state()
                .container
                .background_color
                .map(zed1_color_to_hsla),
            tab_bar_background: tab_bar.container.background_color.map(zed1_color_to_hsla),
            tab_active_background: active_tab
                .container
                .background_color
                .map(zed1_color_to_hsla),
            tab_inactive_background: inactive_tab
                .container
                .background_color
                .map(zed1_color_to_hsla),
            drop_target_background: convert(self.theme.workspace.drop_target_overlay_color),
            toolbar_background: toolbar.container.background_color.map(zed1_color_to_hsla),
            editor_foreground: convert(editor.text_color),
            editor_background: convert(editor.background),
            editor_gutter_background: convert(editor.gutter_background),
            editor_line_number: convert(editor.line_number),
            editor_active_line_number: convert(editor.line_number_active),
            editor_wrap_guide: convert(editor.wrap_guide),
            editor_active_wrap_guide: convert(editor.active_wrap_guide),
            scrollbar_track_background: scrollbar.track.background_color.map(zed1_color_to_hsla),
            scrollbar_track_border: convert(scrollbar.track.border.color),
            scrollbar_thumb_background: scrollbar.thumb.background_color.map(zed1_color_to_hsla),
            scrollbar_thumb_border: convert(scrollbar.thumb.border.color),
            scrollbar_thumb_hover_background: scrollbar
                .thumb
                .background_color
                .map(zed1_color_to_hsla),
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
