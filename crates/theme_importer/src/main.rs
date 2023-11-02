use std::borrow::Cow;
use std::fs::{self, File};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use convert_case::Case;
use gpui::{serde_json, AssetSource, SharedString};
use log::LevelFilter;
use rust_embed::RustEmbed;
use serde::Deserialize;
use simplelog::SimpleLogger;
use theme::{
    default_color_scales, Appearance, GitStatusColors, PlayerColors, StatusColors, SyntaxTheme,
    SystemColors, ThemeColors, ThemeColorsRefinement, ThemeFamily, ThemeStyles, ThemeVariant,
};

use crate::vscode::VsCodeTheme;

mod vscode;

pub(crate) fn new_theme_family(name: String, author: String) -> ThemeFamily {
    ThemeFamily {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.into(),
        author: author.into(),
        themes: Vec::new(),
        scales: default_color_scales(),
    }
}

#[derive(Debug, Deserialize)]
struct FamilyJson {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeVariantJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ThemeAppearanceJson {
    Light,
    Dark,
}

#[derive(Debug, Deserialize)]
struct ThemeVariantJson {
    pub name: String,
    pub appearance: ThemeAppearanceJson,
}

struct ImportedThemeFamily {
    pub id: String,
    pub name: String,
    pub author: String,
    pub url: Option<String>,
    // App should panic if we try to load a theme without a lisence
    pub license: String,
    pub themes: Vec<ImportedThemeVariant>,
}

struct ImportedThemeVariant {
    pub id: String,
    pub name: String,
    pub colors: ThemeColorsRefinement,
}

// Load a vscode theme from json
// Load it's LICENSE from the same folder
// Create a ThemeFamily for the theme
// Create a ThemeVariant or Variants for the theme
// Output a rust file with the ThemeFamily and ThemeVariant(s) in it

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let themes_path = PathBuf::from_str("crates/theme2/src/themes")?;

    let vscode_themes_path = PathBuf::from_str("assets/themes/src/vscode/")?;

    let mut theme_families = Vec::new();

    for theme_family_dir in fs::read_dir(&vscode_themes_path)? {
        let theme_family_dir = theme_family_dir?;

        let theme_family_slug = theme_family_dir
            .path()
            .file_stem()
            .ok_or(anyhow!("no file stem"))
            .map(|stem| stem.to_string_lossy().to_string())?;

        let family_metadata_file = File::open(theme_family_dir.path().join("family.json"))
            .context(format!("no `family.json` found for '{theme_family_slug}'"))?;

        let family_metadata: FamilyJson = serde_json::from_reader(family_metadata_file).context(
            format!("failed to parse `family.json` for '{theme_family_slug}'"),
        )?;

        let mut themes = Vec::new();

        for theme_entry in fs::read_dir(vscode_themes_path.join(theme_family_slug))? {
            let theme_entry = theme_entry?;

            let theme_file_path = theme_entry.path();

            let file_name = theme_file_path
                .file_name()
                .ok_or(anyhow!("no file stem"))
                .map(|file_name| file_name.to_string_lossy())?;

            if !file_name.ends_with(".json") {
                continue;
            }

            if file_name == "family.json" {
                continue;
            }

            let theme_file = File::open(&theme_file_path)?;

            let theme: VsCodeTheme = serde_json::from_reader(theme_file)
                .context(format!("failed to parse theme {theme_file_path:?}"))?;

            themes.push(theme);
        }

        let theme_family = ThemeFamily {
            id: uuid::Uuid::new_v4().to_string(),
            name: family_metadata.name.into(),
            author: family_metadata.author.into(),
            themes: themes
                .into_iter()
                .map(|theme| ThemeVariant {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: "".into(),
                    appearance: Appearance::Dark,
                    styles: ThemeStyles {
                        system: SystemColors::default(),
                        colors: ThemeColors::default_dark(),
                        status: StatusColors::default(),
                        git: GitStatusColors::default(),
                        player: PlayerColors::default(),
                        syntax: SyntaxTheme::default_dark(),
                    },
                })
                .collect(),
            scales: default_color_scales(),
        };

        theme_families.push(theme_family);
    }

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "themes/**/*"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Cow<[u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}
