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
    /// The path to the theme to import.
    theme_path: PathBuf,

    /// Whether to warn when values are missing from the theme.
    #[arg(long)]
    warn_on_missing: bool,

    /// The path to write the output to.
    #[arg(long, short)]
    output: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Prints the JSON schema for a theme.
    PrintSchema,
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

    if let Some(command) = args.command {
        match command {
            Command::PrintSchema => {
                let theme_family_schema = schema_for!(ThemeFamilyContent);

                println!(
                    "{}",
                    serde_json::to_string_pretty(&theme_family_schema).unwrap()
                );

                return Ok(());
            }
        }
    }

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
