use std::path::PathBuf;
use std::str::FromStr;
use std::{borrow::Cow, fs::File};

use anyhow::{anyhow, Context, Result};
use convert_case::Case;
use gpui::{AssetSource, SharedString};
use log::LevelFilter;
use rust_embed::RustEmbed;
use serde::Deserialize;
use simplelog::SimpleLogger;
use theme::{default_color_scales, ThemeColorsRefinement, ThemeFamily};

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

#[derive(Deserialize)]
struct FamilyJson {
    pub name: String,
    pub themes: Vec<ThemeVariantJson>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ThemeAppearanceJson {
    Light,
    Dark,
}

#[derive(Deserialize)]
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

    let mut theme_modules = Vec::new();

    for theme_path in Assets.list("themes/src/vsc/")? {
        let (_, theme_name) = theme_path.split_once("themes/").unwrap();

        if theme_name == ".gitkeep" {
            continue;
        }

        let theme_contents = Assets::get(&theme_path)
            .with_context(|| format!("theme file not found: '{theme_path}'"))?;

        // let json_theme: JsonTheme =
        //     serde_json::from_str(std::str::from_utf8(&theme_contents.data)?)
        //         .context("failed to parse legacy theme")?;

        // let (json_theme, legacy_theme) = load_theme(&theme_path)?;

        // let theme = convert_theme(json_theme, legacy_theme)?;

        // let theme_slug = theme
        //     .metadata
        //     .name
        //     .as_ref()
        //     .replace("é", "e")
        //     .to_case(Case::Snake);

        // let mut output_file = File::create(themes_path.join(format!("{theme_slug}.rs")))?;

        // let theme_module = format!(
        //     r#"
        //         use gpui2::rgba;

        //         use crate::{{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata}};

        //         pub fn {theme_slug}() -> Theme {{
        //             {theme_definition}
        //         }}
        //     "#,
        //     theme_definition = format!("{:#?}", ThemePrinter::new(theme))
        // );

        // output_file.write_all(theme_module.as_bytes())?;

        theme_modules.push(theme_slug);
    }

    println!("Hello, world!");

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
