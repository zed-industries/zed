use anyhow::Result;
use gpui::{Hsla, Rgba};
use gpui1::color::Color as Zed1Color;
use gpui1::fonts::HighlightStyle as Zed1HighlightStyle;
use theme::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeStylesRefinement,
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
        let syntax_theme = self.convert_syntax_theme()?;

        Ok(UserTheme {
            name: format!("{} (Zed1)", self.theme.meta.name),
            appearance,
            styles: UserThemeStylesRefinement {
                colors: theme_colors_refinement,
                status: status_colors_refinement,
                syntax: Some(syntax_theme),
            },
        })
    }

    fn convert_status_colors(&self) -> Result<StatusColorsRefinement> {
        fn convert(color: Zed1Color) -> Option<Hsla> {
            Some(zed1_color_to_hsla(color))
        }

        let diff_style = self.theme.editor.diff.clone();

        Ok(StatusColorsRefinement {
            created: convert(diff_style.inserted),
            modified: convert(diff_style.modified),
            deleted: convert(diff_style.deleted),
            ..Default::default()
        })
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsRefinement> {
        fn convert(color: Zed1Color) -> Option<Hsla> {
            Some(zed1_color_to_hsla(color))
        }

        let scrollbar = self.theme.editor.scrollbar.clone();

        let zed1_titlebar_border = convert(self.theme.titlebar.container.border.color);

        Ok(ThemeColorsRefinement {
            border: zed1_titlebar_border,
            border_variant: zed1_titlebar_border,
            background: convert(self.theme.workspace.background),
            title_bar_background: self
                .theme
                .titlebar
                .container
                .background_color
                .map(zed1_color_to_hsla),
            status_bar_background: self
                .theme
                .workspace
                .status_bar
                .container
                .background_color
                .map(zed1_color_to_hsla),
            text: convert(self.theme.editor.text_color),
            editor_foreground: convert(self.theme.editor.text_color),
            editor_background: convert(self.theme.editor.background),
            scrollbar_track_background: scrollbar.track.background_color.map(zed1_color_to_hsla),
            scrollbar_track_border: convert(scrollbar.track.border.color),
            scrollbar_thumb_background: scrollbar.thumb.background_color.map(zed1_color_to_hsla),
            terminal_ansi_bright_black: convert(self.theme.terminal.bright_black),
            terminal_ansi_bright_red: convert(self.theme.terminal.bright_red),
            terminal_ansi_bright_green: convert(self.theme.terminal.bright_green),
            terminal_ansi_bright_yellow: convert(self.theme.terminal.bright_yellow),
            terminal_ansi_bright_blue: convert(self.theme.terminal.bright_blue),
            terminal_ansi_bright_magenta: convert(self.theme.terminal.bright_magenta),
            terminal_ansi_bright_cyan: convert(self.theme.terminal.bright_cyan),
            terminal_ansi_bright_white: convert(self.theme.terminal.bright_white),
            terminal_ansi_black: convert(self.theme.terminal.black),
            terminal_ansi_red: convert(self.theme.terminal.red),
            terminal_ansi_green: convert(self.theme.terminal.green),
            terminal_ansi_yellow: convert(self.theme.terminal.yellow),
            terminal_ansi_blue: convert(self.theme.terminal.blue),
            terminal_ansi_magenta: convert(self.theme.terminal.magenta),
            terminal_ansi_cyan: convert(self.theme.terminal.cyan),
            terminal_ansi_white: convert(self.theme.terminal.white),
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
