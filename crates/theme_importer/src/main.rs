mod theme_printer;
mod util;
mod vscode;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use gpui::serde_json;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::SimpleLogger;
use theme::{default_color_scales, Appearance, ThemeFamily};
use vscode::VsCodeThemeConverter;

use crate::theme_printer::ThemeFamilyPrinter;
use crate::vscode::VsCodeTheme;

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeMetadata>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeAppearanceJson {
    Light,
    Dark,
}

impl From<ThemeAppearanceJson> for Appearance {
    fn from(value: ThemeAppearanceJson) -> Self {
        match value {
            ThemeAppearanceJson::Light => Self::Light,
            ThemeAppearanceJson::Dark => Self::Dark,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub file_name: String,
    pub appearance: ThemeAppearanceJson,
}

// Load a vscode theme from json
// Load it's LICENSE from the same folder
// Create a ThemeFamily for the theme
// Create a ThemeVariant or Variants for the theme
// Output a rust file with the ThemeFamily and ThemeVariant(s) in it

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

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

        let family_metadata: FamilyMetadata = serde_json::from_reader(family_metadata_file)
            .context(format!(
                "failed to parse `family.json` for '{theme_family_slug}'"
            ))?;

        let mut themes = Vec::new();

        for theme_metadata in family_metadata.themes {
            let theme_file_path = theme_family_dir.path().join(&theme_metadata.file_name);

            let theme_file = File::open(&theme_file_path)?;

            let vscode_theme: VsCodeTheme = serde_json::from_reader(theme_file)
                .context(format!("failed to parse theme {theme_file_path:?}"))?;

            let converter = VsCodeThemeConverter::new(vscode_theme, theme_metadata);

            let theme = converter.convert()?;

            themes.push(theme);
        }

        let theme_family = ThemeFamily {
            id: uuid::Uuid::new_v4().to_string(),
            name: family_metadata.name.into(),
            author: family_metadata.author.into(),
            themes,
            scales: default_color_scales(),
        };

        theme_families.push(theme_family);
    }

    let themes_output_path = PathBuf::from_str("crates/theme2/src/themes")?;

    let mut theme_modules = Vec::new();

    for theme_family in theme_families {
        let theme_family_slug = theme_family.name.to_string().to_case(Case::Snake);

        let mut output_file =
            File::create(themes_output_path.join(format!("{theme_family_slug}.rs")))?;

        let theme_module = format!(
            r#"
            use gpui2::rgba;

            use crate::{{
                default_color_scales, Appearance, GitStatusColors, PlayerColor, PlayerColors, StatusColors,
                SyntaxTheme, SystemColors, ThemeColors, ThemeFamily, ThemeStyles, ThemeVariant,
            }};

            pub fn {theme_family_slug}() -> ThemeFamily {{
                {theme_family_definition}
            }}
            "#,
            theme_family_definition = format!("{:#?}", ThemeFamilyPrinter::new(theme_family))
        );

        output_file.write_all(theme_module.as_bytes())?;

        theme_modules.push(theme_family_slug);
    }

    let mut mod_rs_file = File::create(themes_output_path.join(format!("mod.rs")))?;

    let mod_rs_contents = format!(
        r#"
        {mod_statements}

        {use_statements}
        "#,
        mod_statements = theme_modules
            .iter()
            .map(|module| format!("mod {module};"))
            .collect::<Vec<_>>()
            .join("\n"),
        use_statements = theme_modules
            .iter()
            .map(|module| format!("pub use {module}::*;"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    mod_rs_file.write_all(mod_rs_contents.as_bytes())?;

    Ok(())
}
