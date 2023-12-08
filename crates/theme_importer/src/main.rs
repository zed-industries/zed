mod color;
mod theme_printer;
mod util;
mod vscode;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use gpui::serde_json;
use json_comments::StripComments;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::{TermLogger, TerminalMode};
use theme::{Appearance, UserThemeFamily};

use crate::theme_printer::UserThemeFamilyPrinter;
use crate::vscode::VsCodeTheme;
use crate::vscode::VsCodeThemeConverter;

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeMetadata>,
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

fn main() -> Result<()> {
    const SOURCE_PATH: &str = "assets/themes/src/vscode";
    const OUT_PATH: &str = "crates/theme2/src/themes";

    let log_config = simplelog::ConfigBuilder::new()
        .set_level_color(log::Level::Info, simplelog::Color::Blue)
        .set_level_color(log::Level::Warn, simplelog::Color::Yellow)
        .build();

    TermLogger::init(LevelFilter::Info, log_config, TerminalMode::Mixed)
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

            let converter = VsCodeThemeConverter::new(vscode_theme, theme_metadata);

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

    let themes_output_path = PathBuf::from_str(OUT_PATH)?;

    if !themes_output_path.exists() {
        log::info!("Creating directory: {:?}", themes_output_path);
        fs::create_dir_all(&themes_output_path)?;
    }

    let mut mod_rs_file = File::create(themes_output_path.join(format!("mod.rs")))?;

    let mut theme_modules = Vec::new();

    for theme_family in theme_families {
        let theme_family_slug = theme_family.name.to_string().to_case(Case::Snake);

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
