use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;
use theme::{default_color_scales, ColorScales, ThemeFamily};

#[derive(Deserialize, Debug)]
pub struct VSCodeTheme {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub author: String,
    pub maintainers: Vec<String>,
    #[serde(rename = "semanticClass")]
    pub semantic_class: String,
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: bool,
    pub colors: VSCodeColors,
}

#[derive(Debug, Deserialize)]
pub struct VSCodeColors {
    #[serde(rename = "editor.foreground")]
    text: String,
    #[serde(rename = "editor.background")]
    editor: String,
}

pub(crate) fn new_theme_family_from_vsc(path: &Path) -> Result<ThemeFamily> {
    let path_str = path.to_str().unwrap();
    let family_name = path_str.split('/').last().unwrap();

    let mut json_files: Vec<String> = Vec::new();

    if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        json_files.push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        }
    } else {
        anyhow::bail!("Path is not a directory");
    }

    let mut theme_family = ThemeFamily {
        id: uuid::Uuid::new_v4().to_string(),
        name: family_name.into(),
        author: "New Theme Family".into(),
        themes: Vec::new(),
        scales: default_color_scales(),
    };

    Ok(theme_family)
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
