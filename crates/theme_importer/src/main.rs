mod color;
mod vscode;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context as _, Result};
use clap::Parser;
use indexmap::IndexMap;
use log::LevelFilter;
use serde::Deserialize;
use simplelog::ColorChoice;
use simplelog::{TermLogger, TerminalMode};
use theme::{Appearance, AppearanceContent};

use crate::vscode::VsCodeTheme;
use crate::vscode::VsCodeThemeConverter;

const ZED_THEME_SCHEMA_URL: &str = "https://zed.dev/schema/themes/v0.2.0.json";

#[derive(Debug, Deserialize)]
struct FamilyMetadata {
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub name: String,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub author: String,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub themes: Vec<ThemeMetadata>,

    /// Overrides for specific syntax tokens.
    ///
    /// Use this to ensure certain Zed syntax tokens are matched
    /// to an exact set of scopes when it is not otherwise possible
    /// to rely on the default mappings in the theme importer.
    #[serde(default)]
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub syntax: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeAppearanceJson {
    Light,
    Dark,
}

impl From<ThemeAppearanceJson> for AppearanceContent {
    fn from(value: ThemeAppearanceJson) -> Self {
        match value {
            ThemeAppearanceJson::Light => Self::Light,
            ThemeAppearanceJson::Dark => Self::Dark,
        }
    }
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
    /// The path to the theme to import.
    theme_path: PathBuf,

    /// Whether to warn when values are missing from the theme.
    #[arg(long)]
    warn_on_missing: bool,

    /// The path to write the output to.
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let log_config = {
        let mut config = simplelog::ConfigBuilder::new();

        if !args.warn_on_missing {
            config.add_filter_ignore_str("theme_printer");
        }

        config.build()
    };

    TermLogger::init(
        LevelFilter::Trace,
        log_config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .expect("could not initialize logger");

    let theme_file_path = args.theme_path;

    let theme_file = match File::open(&theme_file_path) {
        Ok(file) => file,
        Err(err) => {
            log::info!("Failed to open file at path: {:?}", theme_file_path);
            return Err(err)?;
        }
    };

    let vscode_theme: VsCodeTheme = serde_json_lenient::from_reader(theme_file)
        .context(format!("failed to parse theme {theme_file_path:?}"))?;

    let theme_metadata = ThemeMetadata {
        name: vscode_theme.name.clone().unwrap_or("".to_string()),
        appearance: ThemeAppearanceJson::Dark,
        file_name: "".to_string(),
    };

    let converter = VsCodeThemeConverter::new(vscode_theme, theme_metadata, IndexMap::new());

    let theme = converter.convert()?;
    let mut theme = serde_json::to_value(theme).unwrap();
    theme.as_object_mut().unwrap().insert(
        "$schema".to_string(),
        serde_json::Value::String(ZED_THEME_SCHEMA_URL.to_string()),
    );
    let theme_json = serde_json::to_string_pretty(&theme).unwrap();

    if let Some(output) = args.output {
        let mut file = File::create(output)?;
        file.write_all(theme_json.as_bytes())?;
    } else {
        println!("{}", theme_json);
    }

    log::info!("Done!");

    Ok(())
}
