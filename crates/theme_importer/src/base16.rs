use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use serde::Deserialize;
use strum::IntoEnumIterator;
use theme::{
    HighlightStyleContent, StatusColorsContent, ThemeColorsContent, ThemeContent, ThemeStyleContent,
};

use crate::{syntax::ZedSyntaxToken, ThemeMetadata};

/// `Base16Theme` defines a color scheme for themes adhering to [Base16](https://github.com/chriskempson/base16), which dictates a set palette of sixteen colors: eight base shades (`Base00`-`Base07`) and eight accents (`Base08`-`Base0F`). This structure ensures consistency across themes. Each color is represented as a string within the `Base16Theme` struct.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Base16Theme {
    scheme: String,
    author: String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
}

// This is a pretty naive way to validate the theme,
// but base16 themes are really simple.
//
// We could pull in a json schema validator,
// but we should probably just do that when we add
// one for Zed themes.
//
/// Check all required fields are present and non-empty
/// in the given `Base16Theme`.
fn validate_theme(theme: &Base16Theme) -> Result<()> {
    let fields = [
        ("scheme", &theme.scheme),
        ("author", &theme.author),
        ("base00", &theme.base00),
        ("base01", &theme.base01),
        ("base02", &theme.base02),
        ("base03", &theme.base03),
        ("base04", &theme.base04),
        ("base05", &theme.base05),
        ("base06", &theme.base06),
        ("base07", &theme.base07),
        ("base08", &theme.base08),
        ("base09", &theme.base09),
        ("base0A", &theme.base0A),
        ("base0B", &theme.base0B),
        ("base0C", &theme.base0C),
        ("base0D", &theme.base0D),
        ("base0E", &theme.base0E),
        ("base0F", &theme.base0F),
    ];

    for (name, value) in fields.iter() {
        if value.is_empty() {
            return Err(anyhow!("{} field is not specified", name));
        }
    }

    Ok(())
}

impl ZedSyntaxToken {
    pub fn to_base16(&self) -> Vec<&'static str> {
        match self {
            ZedSyntaxToken::Attribute => vec!["base0D"],
            ZedSyntaxToken::Boolean => vec!["base09"],
            ZedSyntaxToken::Comment => vec!["base03"],
            ZedSyntaxToken::CommentDoc => vec!["base03"],
            ZedSyntaxToken::Constant => vec!["base09"],
            ZedSyntaxToken::Constructor => vec!["base0E"],
            ZedSyntaxToken::Embedded => vec!["base0F"],
            ZedSyntaxToken::Emphasis => vec!["base09"],
            ZedSyntaxToken::EmphasisStrong => vec!["base08"],
            ZedSyntaxToken::Enum => vec!["base0E"],
            ZedSyntaxToken::Function => vec!["base0D"],
            ZedSyntaxToken::Hint => vec!["base0C"],
            ZedSyntaxToken::Keyword => vec!["base0E"],
            ZedSyntaxToken::Label => vec!["base0A"],
            ZedSyntaxToken::LinkText => vec!["base0D"],
            ZedSyntaxToken::LinkUri => vec!["base0D"],
            ZedSyntaxToken::Number => vec!["base09"],
            ZedSyntaxToken::Operator => vec!["base05"],
            ZedSyntaxToken::Predictive => vec!["base0E"],
            ZedSyntaxToken::Preproc => vec!["base0A"],
            ZedSyntaxToken::Primary => vec!["base08"],
            ZedSyntaxToken::Property => vec!["base0A"],
            ZedSyntaxToken::Punctuation => vec!["base05"],
            ZedSyntaxToken::PunctuationBracket => vec!["base0B"],
            ZedSyntaxToken::PunctuationDelimiter => vec!["base0F"],
            ZedSyntaxToken::PunctuationListMarker => vec!["base0B"],
            ZedSyntaxToken::PunctuationSpecial => vec!["base0C"],
            ZedSyntaxToken::String => vec!["base0B"],
            ZedSyntaxToken::StringEscape => vec!["base0F"],
            ZedSyntaxToken::StringRegex => vec!["base0C"],
            ZedSyntaxToken::StringSpecial => vec!["base0E"],
            ZedSyntaxToken::StringSpecialSymbol => vec!["base0E"],
            ZedSyntaxToken::Tag => vec!["base0A"],
            ZedSyntaxToken::TextLiteral => vec!["base0B"],
            ZedSyntaxToken::Title => vec!["base0D"],
            ZedSyntaxToken::Type => vec!["base0A"],
            ZedSyntaxToken::Variable => vec!["base08"],
            ZedSyntaxToken::VariableSpecial => vec!["base0E"],
            ZedSyntaxToken::Variant => vec!["base0E"],
        }
    }
}

pub struct Base16ThemeConverter {
    theme: Base16Theme,
    theme_metadata: ThemeMetadata,
    syntax_overrides: IndexMap<String, Vec<String>>,
}

impl Base16ThemeConverter {
    pub fn new(
        theme: Base16Theme,
        theme_metadata: ThemeMetadata,
        syntax_overrides: IndexMap<String, Vec<String>>,
    ) -> Self {
        Self {
            theme,
            theme_metadata,
            syntax_overrides,
        }
    }

    fn convert_status_colors(&self) -> Result<StatusColorsContent> {
        Ok(StatusColorsContent {
            conflict: Some(self.theme.base08.clone()),
            created: Some(self.theme.base0B.clone()),
            deleted: Some(self.theme.base08.clone()),
            error: Some(self.theme.base08.clone()),
            hidden: Some(self.theme.base03.clone()),
            hint: Some(self.theme.base0C.clone()),
            ignored: Some(self.theme.base03.clone()),
            info: Some(self.theme.base0D.clone()),
            modified: Some(self.theme.base0E.clone()),
            warning: Some(self.theme.base0A.clone()),
            ..Default::default()
        })
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsContent> {
        Ok(ThemeColorsContent {
            border: Some(self.theme.base00.clone()),
            border_variant: Some(self.theme.base01.clone()),
            border_focused: Some(self.theme.base0D.clone()),
            border_selected: Some(self.theme.base0D.clone()),
            border_disabled: Some(self.theme.base03.clone()),
            elevated_surface_background: Some(self.theme.base00.clone()),
            surface_background: Some(self.theme.base00.clone()),
            background: Some(self.theme.base00.clone()),
            element_background: Some(self.theme.base01.clone()),
            element_hover: Some(self.theme.base02.clone()),
            element_selected: Some(self.theme.base0D.clone()),
            drop_target_background: Some(self.theme.base02.clone()),
            ghost_element_hover: Some(self.theme.base02.clone()),
            ghost_element_selected: Some(self.theme.base0D.clone()),
            text: Some(self.theme.base05.clone()),
            text_muted: Some(self.theme.base03.clone()),
            status_bar_background: Some(self.theme.base00.clone()),
            title_bar_background: Some(self.theme.base00.clone()),
            toolbar_background: Some(self.theme.base01.clone()),
            tab_bar_background: Some(self.theme.base01.clone()),
            tab_inactive_background: Some(self.theme.base01.clone()),
            tab_active_background: Some(self.theme.base00.clone()),
            panel_background: Some(self.theme.base00.clone()),
            scrollbar_thumb_background: Some(self.theme.base02.clone()),
            scrollbar_thumb_hover_background: Some(self.theme.base03.clone()),
            scrollbar_thumb_border: Some(self.theme.base01.clone()),
            scrollbar_track_background: Some(self.theme.base00.clone()),
            scrollbar_track_border: Some(self.theme.base01.clone()),
            editor_foreground: Some(self.theme.base05.clone()),
            editor_background: Some(self.theme.base00.clone()),
            editor_gutter_background: Some(self.theme.base00.clone()),
            editor_active_line_background: Some(self.theme.base01.clone()),
            editor_line_number: Some(self.theme.base03.clone()),
            editor_active_line_number: Some(self.theme.base04.clone()),
            editor_wrap_guide: Some(self.theme.base02.clone()),
            editor_active_wrap_guide: Some(self.theme.base03.clone()),
            terminal_background: Some(self.theme.base00.clone()),
            terminal_ansi_black: Some(self.theme.base00.clone()),
            terminal_ansi_bright_black: Some(self.theme.base03.clone()),
            terminal_ansi_red: Some(self.theme.base08.clone()),
            terminal_ansi_bright_red: Some(self.theme.base0B.clone()),
            terminal_ansi_green: Some(self.theme.base0B.clone()),
            terminal_ansi_bright_green: Some(self.theme.base0A.clone()),
            terminal_ansi_yellow: Some(self.theme.base0A.clone()),
            terminal_ansi_bright_yellow: Some(self.theme.base09.clone()),
            terminal_ansi_blue: Some(self.theme.base0D.clone()),
            terminal_ansi_bright_blue: Some(self.theme.base0C.clone()),
            terminal_ansi_magenta: Some(self.theme.base0E.clone()),
            terminal_ansi_bright_magenta: Some(self.theme.base0D.clone()),
            terminal_ansi_cyan: Some(self.theme.base0C.clone()),
            terminal_ansi_bright_cyan: Some(self.theme.base0B.clone()),
            terminal_ansi_white: Some(self.theme.base05.clone()),
            terminal_ansi_bright_white: Some(self.theme.base06.clone()),
            link_text_hover: Some(self.theme.base0D.clone()),
            ..Default::default()
        })
    }

    fn convert_syntax_theme(&self) -> Result<IndexMap<String, HighlightStyleContent>> {
        let mut highlight_styles = IndexMap::new();

        for syntax_token in ZedSyntaxToken::iter() {
            let base16_colors = syntax_token.to_base16();
            let mut highlight_style = HighlightStyleContent::default();

            for color in base16_colors {
                let color_value = match color {
                    "base00" => &self.theme.base00,
                    "base01" => &self.theme.base01,
                    "base02" => &self.theme.base02,
                    "base03" => &self.theme.base03,
                    "base04" => &self.theme.base04,
                    "base05" => &self.theme.base05,
                    "base06" => &self.theme.base06,
                    "base07" => &self.theme.base07,
                    "base08" => &self.theme.base08,
                    "base09" => &self.theme.base09,
                    "base0A" => &self.theme.base0A,
                    "base0B" => &self.theme.base0B,
                    "base0C" => &self.theme.base0C,
                    "base0D" => &self.theme.base0D,
                    "base0E" => &self.theme.base0E,
                    "base0F" => &self.theme.base0F,
                    _ => continue,
                };

                highlight_style.color = Some(color_value.clone());
                break;
            }

            if !highlight_style.is_empty() {
                highlight_styles.insert(syntax_token.to_string(), highlight_style);
            }
        }

        Ok(highlight_styles)
    }

    pub fn convert(self) -> Result<ThemeContent> {
        validate_theme(&self.theme)?;

        let appearance = self.theme_metadata.appearance.into();

        let status_colors = self.convert_status_colors()?;
        let theme_colors = self.convert_theme_colors()?;
        let syntax_theme = self.convert_syntax_theme()?;

        Ok(ThemeContent {
            name: self.theme.scheme.clone(),
            appearance,
            style: ThemeStyleContent {
                colors: theme_colors,
                status: status_colors,
                players: Vec::new(),
                syntax: syntax_theme,
            },
        })
    }
}
