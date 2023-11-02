use std::path::{Path, PathBuf};

use anyhow::Result;
use gpui::{Hsla, Refineable, Rgba};
use serde::Deserialize;
use theme::{
    default_color_scales, Appearance, ColorScales, GitStatusColors, PlayerColors, StatusColors,
    SyntaxTheme, SystemColors, ThemeColors, ThemeColorsRefinement, ThemeFamily, ThemeStyles,
    ThemeVariant,
};
use uuid::Uuid;

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
    editor: String,
}

pub(crate) fn new_theme_family_from_vsc(path: &Path) -> Result<ThemeFamily> {
    todo!()

    // let path_str = path.to_str().unwrap();
    // let family_name = path_str.split('/').last().unwrap();

    // let mut json_files: Vec<String> = Vec::new();

    // if path.is_dir() {
    //     for entry in std::fs::read_dir(path).unwrap() {
    //         let entry = entry.unwrap();
    //         let path = entry.path();
    //         if path.is_file() {
    //             if let Some(extension) = path.extension() {
    //                 if extension == "json" {
    //                     json_files.push(path.file_name().unwrap().to_str().unwrap().to_string());
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     anyhow::bail!("Path is not a directory");
    // }

    // let mut theme_family = ThemeFamily {
    //     id: uuid::Uuid::new_v4().to_string(),
    //     name: family_name.into(),
    //     author: "New Theme Family".into(),
    //     themes: Vec::new(),
    //     scales: default_color_scales(),
    // };

    // Ok(theme_family)
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
            background: Some(try_parse_color(&vscode_colors.editor)?),
            text: Some(try_parse_color(&vscode_colors.text)?),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_deserialize_theme() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_dir = manifest_dir.parent().unwrap().parent().unwrap();

        let mut d = root_dir.to_path_buf();
        d.push("assets/themes/src/vsc/dracula/dracula.json");

        let data = std::fs::read_to_string(d).expect("Unable to read file");

        let result: Theme = serde_json::from_str(&data).unwrap();
        println!("{:#?}", result);
    }
}
