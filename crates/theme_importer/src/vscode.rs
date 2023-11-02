use anyhow::Result;
use gpui::{Hsla, Refineable, Rgba};
use serde::Deserialize;
use theme::{
    Appearance, GitStatusColors, PlayerColors, StatusColors, SyntaxTheme, SystemColors,
    ThemeColors, ThemeColorsRefinement, ThemeStyles, ThemeVariant,
};

use crate::ThemeMetadata;

#[derive(Deserialize, Debug)]
pub struct VsCodeTheme {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub maintainers: Option<Vec<String>>,
    #[serde(rename = "semanticClass")]
    pub semantic_class: Option<String>,
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: Option<bool>,
    pub colors: VsCodeColors,
}

#[derive(Debug, Deserialize)]
pub struct VsCodeColors {
    #[serde(rename = "editor.foreground")]
    text: String,
    #[serde(rename = "editor.background")]
    editor_background: String,
    terminal_background: String,
    terminal_ansi_bright_black: String,
    terminal_ansi_bright_red: String,
    terminal_ansi_bright_green: String,
    terminal_ansi_bright_yellow: String,
    terminal_ansi_bright_blue: String,
    terminal_ansi_bright_magenta: String,
    terminal_ansi_bright_cyan: String,
    terminal_ansi_bright_white: String,
    terminal_ansi_black: String,
    terminal_ansi_red: String,
    terminal_ansi_green: String,
    terminal_ansi_yellow: String,
    terminal_ansi_blue: String,
    terminal_ansi_magenta: String,
    terminal_ansi_cyan: String,
    terminal_ansi_white: String,
}

fn try_parse_color(color: &str) -> Result<Hsla> {
    Ok(Rgba::try_from(color)?.into())
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

    pub fn convert(self) -> Result<ThemeVariant> {
        let appearance = self.theme_metadata.appearance.into();

        let mut theme_colors = match appearance {
            Appearance::Light => ThemeColors::default_light(),
            Appearance::Dark => ThemeColors::default_dark(),
        };

        let vscode_colors = &self.theme.colors;

        let theme_colors_refinements = ThemeColorsRefinement {
            background: Some(try_parse_color(&vscode_colors.editor_background)?),
            text: Some(try_parse_color(&vscode_colors.text)?),
            terminal_background: Some(try_parse_color(&vscode_colors.terminal_background)?),
            terminal_ansi_bright_black: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_black,
            )?),
            terminal_ansi_bright_red: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_red,
            )?),
            terminal_ansi_bright_green: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_green,
            )?),
            terminal_ansi_bright_yellow: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_yellow,
            )?),
            terminal_ansi_bright_blue: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_blue,
            )?),
            terminal_ansi_bright_magenta: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_magenta,
            )?),
            terminal_ansi_bright_cyan: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_cyan,
            )?),
            terminal_ansi_bright_white: Some(try_parse_color(
                &vscode_colors.terminal_ansi_bright_white,
            )?),
            terminal_ansi_black: Some(try_parse_color(&vscode_colors.terminal_ansi_black)?),
            terminal_ansi_red: Some(try_parse_color(&vscode_colors.terminal_ansi_red)?),
            terminal_ansi_green: Some(try_parse_color(&vscode_colors.terminal_ansi_green)?),
            terminal_ansi_yellow: Some(try_parse_color(&vscode_colors.terminal_ansi_yellow)?),
            terminal_ansi_blue: Some(try_parse_color(&vscode_colors.terminal_ansi_blue)?),
            terminal_ansi_magenta: Some(try_parse_color(&vscode_colors.terminal_ansi_magenta)?),
            terminal_ansi_cyan: Some(try_parse_color(&vscode_colors.terminal_ansi_cyan)?),
            terminal_ansi_white: Some(try_parse_color(&vscode_colors.terminal_ansi_white)?),
            ..Default::default()
        };

        theme_colors.refine(&theme_colors_refinements);

        Ok(ThemeVariant {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.theme_metadata.name.into(),
            appearance,
            styles: ThemeStyles {
                system: SystemColors::default(),
                colors: theme_colors,
                status: StatusColors::default(),
                git: GitStatusColors::default(),
                player: PlayerColors::default(),
                syntax: SyntaxTheme::default_dark(),
            },
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::PathBuf;

//     #[test]
//     fn test_deserialize_theme() {
//         let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         let root_dir = manifest_dir.parent().unwrap().parent().unwrap();

//         let mut d = root_dir.to_path_buf();
//         d.push("assets/themes/src/vsc/dracula/dracula.json");

//         let data = std::fs::read_to_string(d).expect("Unable to read file");

//         let result: Theme = serde_json::from_str(&data).unwrap();
//         println!("{:#?}", result);
//     }
// }
