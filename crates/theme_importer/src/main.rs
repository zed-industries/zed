mod assets;
mod color;
mod vscode;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indexmap::IndexMap;
use log::LevelFilter;
use schemars::schema_for;
use serde::Deserialize;
use simplelog::ColorChoice;
use simplelog::{TermLogger, TerminalMode};
use theme::{Appearance, AppearanceContent, ThemeFamilyContent};

use crate::vscode::VsCodeTheme;
use crate::vscode::VsCodeThemeConverter;

const ZED_THEME_SCHEMA_URL: &str = "https://zed.dev/public/schema/themes/v0.2.0.json";

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
    #[command(subcommand)]
    command: Command,
}

#[derive(PartialEq, Subcommand)]
enum Command {
    /// Prints the JSON schema for a theme.
    PrintSchema,
    /// Converts a VSCode theme to Zed format [default]
    Convert {
        /// The path to the theme to import.
        theme_path: PathBuf,

        /// Whether to warn when values are missing from the theme.
        #[arg(long)]
        warn_on_missing: bool,

        /// The path to write the output to.
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::PrintSchema => {
            let theme_family_schema = schema_for!(ThemeFamilyContent);
            println!(
                "{}",
                serde_json::to_string_pretty(&theme_family_schema).unwrap()
            );
            Ok(())
        }
        Command::Convert {
            theme_path,
            warn_on_missing,
            output,
        } => convert(theme_path, output, warn_on_missing),
    }
}

fn convert(theme_file_path: PathBuf, output: Option<PathBuf>, warn_on_missing: bool) -> Result<()> {
    let log_config = {
        let mut config = simplelog::ConfigBuilder::new();
        if !warn_on_missing {
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

    let theme_file = match File::open(&theme_file_path) {
        Ok(file) => file,
        Err(err) => {
            log::info!("Failed to open file at path: {:?}", theme_file_path);
            return Err(err.into());
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

    if let Some(output) = output {
        let mut file = File::create(output)?;
        file.write_all(theme_json.as_bytes())?;
    } else {
        println!("{}", theme_json);
    }

    log::info!("Done!");

    Ok(())
}
