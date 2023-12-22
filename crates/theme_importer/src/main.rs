mod assets;
mod color;
mod theme_printer;
mod util;
mod vscode;
mod zed1;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;

use any_ascii::any_ascii;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use convert_case::{Case, Casing};
use gpui::{serde_json, AssetSource};
use indexmap::IndexMap;
use json_comments::StripComments;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::{TermLogger, TerminalMode};
use theme::{Appearance, UserTheme, UserThemeFamily};
use theme1::Theme as Zed1Theme;

use crate::assets::Assets;
use crate::theme_printer::UserThemeFamilyPrinter;
use crate::vscode::VsCodeTheme;
use crate::vscode::VsCodeThemeConverter;
use crate::zed1::Zed1ThemeConverter;

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeMetadata>,

    /// Overrides for specific syntax tokens.
    ///
    /// Use this to ensure certain Zed syntax tokens are matched
    /// to an exact set of scopes when it is not otherwise possible
    /// to rely on the default mappings in the theme importer.
    #[serde(default)]
    pub syntax: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Whether to warn when values are missing from the theme.
    #[arg(long)]
    warn_on_missing: bool,
}

fn main() -> Result<()> {
    const SOURCE_PATH: &str = "assets/themes/src/vscode";
    const OUT_PATH: &str = "crates/theme2/src/themes";

    let args = Args::parse();

    let log_config = {
        let mut config = simplelog::ConfigBuilder::new();
        config
            .set_level_color(log::Level::Trace, simplelog::Color::Cyan)
            .set_level_color(log::Level::Info, simplelog::Color::Blue)
            .set_level_color(log::Level::Warn, simplelog::Color::Yellow)
            .set_level_color(log::Level::Error, simplelog::Color::Red);

        if !args.warn_on_missing {
            config.add_filter_ignore_str("theme_printer");
        }

        config.build()
    };

    TermLogger::init(LevelFilter::Trace, log_config, TerminalMode::Mixed)
        .expect("could not initialize logger");

    log::info!("Loading themes source...");
    let vscode_themes_path = PathBuf::from_str(SOURCE_PATH)?;
    if !vscode_themes_path.exists() {
        return Err(anyhow!(format!(
            "Couldn't find {}, make sure it exists",
            SOURCE_PATH
        )));
    }

    let mut theme_families = Vec::new();

    for theme_family_dir in fs::read_dir(&vscode_themes_path)? {
        let theme_family_dir = theme_family_dir?;

        if !theme_family_dir.file_type()?.is_dir() {
            continue;
        }

        let theme_family_slug = theme_family_dir
            .path()
            .file_stem()
            .ok_or(anyhow!("no file stem"))
            .map(|stem| stem.to_string_lossy().to_string())?;

        let family_metadata_file = File::open(theme_family_dir.path().join("family.json"))
            .context(format!(
                "no `family.json` found for '{}'",
                theme_family_slug
            ))?;

        let license_file_path = theme_family_dir.path().join("LICENSE");

        if !license_file_path.exists() {
            log::info!("Skipping theme family '{}' because it does not have a LICENSE file. This theme will only be imported once a LICENSE file is provided.", theme_family_slug);
            continue;
        }

        let family_metadata: FamilyMetadata = serde_json::from_reader(family_metadata_file)
            .context(format!(
                "failed to parse `family.json` for '{theme_family_slug}'"
            ))?;

        let mut themes = Vec::new();

        for theme_metadata in family_metadata.themes {
            log::info!("Converting '{}' theme", &theme_metadata.name);

            let theme_file_path = theme_family_dir.path().join(&theme_metadata.file_name);

            let theme_file = match File::open(&theme_file_path) {
                Ok(file) => file,
                Err(_) => {
                    log::info!("Failed to open file at path: {:?}", theme_file_path);
                    continue;
                }
            };

            let theme_without_comments = StripComments::new(theme_file);
            let vscode_theme: VsCodeTheme = serde_json::from_reader(theme_without_comments)
                .context(format!("failed to parse theme {theme_file_path:?}"))?;

            let converter = VsCodeThemeConverter::new(
                vscode_theme,
                theme_metadata,
                family_metadata.syntax.clone(),
            );

            let theme = converter.convert()?;

            themes.push(theme);
        }

        let theme_family = UserThemeFamily {
            name: family_metadata.name.into(),
            author: family_metadata.author.into(),
            themes,
        };

        theme_families.push(theme_family);
    }

    let zed1_themes_path = PathBuf::from_str("assets/themes")?;

    let zed1_theme_familes = [
        "Andromeda",
        "Atelier",
        "One",
        "Gruvbox",
        "Rosé Pine",
        "Solarized",
    ];

    let mut zed1_themes_by_family: HashMap<String, Vec<UserTheme>> = HashMap::from_iter(
        zed1_theme_familes
            .into_iter()
            .map(|family| (family.to_string(), Vec::new())),
    );

    let platform = gpui1::platform::current::platform();
    let zed1_font_cache = Arc::new(gpui1::FontCache::new(platform.fonts()));

    let mut embedded_fonts = Vec::new();
    for font_path in Assets.list("fonts")? {
        if font_path.ends_with(".ttf") {
            let font_bytes = Assets.load(&font_path)?.to_vec();
            embedded_fonts.push(Arc::from(font_bytes));
        }
    }

    platform.fonts().add_fonts(&embedded_fonts)?;

    for entry in fs::read_dir(&zed1_themes_path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            continue;
        }

        match entry.path().extension() {
            None => continue,
            Some(extension) => {
                if extension != "json" {
                    continue;
                }
            }
        }

        let theme_file_path = entry.path();

        let theme_file = match File::open(&theme_file_path) {
            Ok(file) => file,
            Err(_) => {
                log::info!("Failed to open file at path: {:?}", theme_file_path);
                continue;
            }
        };

        let theme_without_comments = StripComments::new(theme_file);

        let zed1_theme: Zed1Theme = gpui1::fonts::with_font_cache(zed1_font_cache.clone(), || {
            serde_json::from_reader(theme_without_comments)
                .context(format!("failed to parse theme {theme_file_path:?}"))
        })?;

        let theme_name = zed1_theme.meta.name.clone();

        let converter = Zed1ThemeConverter::new(zed1_theme);

        let theme = converter.convert()?;

        let Some((_, themes_for_family)) = zed1_themes_by_family
            .iter_mut()
            .find(|(family, _)| theme_name.starts_with(*family))
        else {
            log::warn!("No theme family found for '{}'.", theme_name);
            continue;
        };

        themes_for_family.push(theme);
    }

    for (family, themes) in zed1_themes_by_family {
        let theme_family = UserThemeFamily {
            name: format!("{family} (Zed1)"),
            author: "Zed Industries".to_string(),
            themes,
        };

        theme_families.push(theme_family);
    }

    let themes_output_path = PathBuf::from_str(OUT_PATH)?;

    if !themes_output_path.exists() {
        log::info!("Creating directory: {:?}", themes_output_path);
        fs::create_dir_all(&themes_output_path)?;
    }

    let mut mod_rs_file = File::create(themes_output_path.join(format!("mod.rs")))?;

    let mut theme_modules = Vec::new();

    for theme_family in theme_families {
        let theme_family_slug = any_ascii(&theme_family.name).to_case(Case::Snake);

        let mut output_file =
            File::create(themes_output_path.join(format!("{theme_family_slug}.rs")))?;
        log::info!(
            "Creating file: {:?}",
            themes_output_path.join(format!("{theme_family_slug}.rs"))
        );

        let theme_module = format!(
            r#"
            // This file was generated by the `theme_importer`.
            // Be careful when modifying it by hand.

            use gpui::rgba;

            #[allow(unused)]
            use crate::{{
                Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserHighlightStyle, UserSyntaxTheme,
                UserTheme, UserThemeFamily, UserThemeStylesRefinement, UserFontWeight, UserFontStyle
            }};

            pub fn {theme_family_slug}() -> UserThemeFamily {{
                {theme_family_definition}
            }}
            "#,
            theme_family_definition = format!("{:#?}", UserThemeFamilyPrinter::new(theme_family))
        );

        output_file.write_all(theme_module.as_bytes())?;

        theme_modules.push(theme_family_slug);
    }

    let themes_vector_contents = format!(
        r#"
        use crate::UserThemeFamily;

        pub(crate) fn all_user_themes() -> Vec<UserThemeFamily> {{
            vec![{all_themes}]
        }}
        "#,
        all_themes = theme_modules
            .iter()
            .map(|module| format!("{}()", module))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mod_rs_contents = format!(
        r#"
        // This file was generated by the `theme_importer`.
        // Be careful when modifying it by hand.

        {mod_statements}

        {use_statements}

        {themes_vector_contents}
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
            .join("\n"),
        themes_vector_contents = themes_vector_contents
    );

    mod_rs_file.write_all(mod_rs_contents.as_bytes())?;

    log::info!("Formatting themes...");

    let format_result = format_themes_crate()
        // We need to format a second time to catch all of the formatting issues.
        .and_then(|_| format_themes_crate());

    if let Err(err) = format_result {
        log::error!("Failed to format themes: {}", err);
    }

    log::info!("Done!");

    Ok(())
}

fn format_themes_crate() -> std::io::Result<std::process::Output> {
    Command::new("cargo")
        .args(["fmt", "--package", "theme2"])
        .output()
}
