use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use schemars::schema_for;
use settings::ProjectSettingsContent;
use theme::{IconThemeFamilyContent, ThemeFamilyContent};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_enum)]
    pub schema_type: SchemaType,

    /// The path to write the output to.
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum SchemaType {
    Theme,
    IconTheme,
    Project,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let schema_json = match args.schema_type {
        SchemaType::Theme => {
            let schema = schema_for!(ThemeFamilyContent);
            serde_json::to_string_pretty(&schema)?
        }
        SchemaType::IconTheme => {
            let schema = schema_for!(IconThemeFamilyContent);
            serde_json::to_string_pretty(&schema)?
        }
        SchemaType::Project => {
            let schema = schema_for!(ProjectSettingsContent);
            serde_json::to_string_pretty(&schema)?
        }
    };

    if let Some(output_path) = args.output {
        let mut file = File::create(output_path)?;
        file.write_all(schema_json.as_bytes())?;
    } else {
        println!("{}", schema_json);
    }

    Ok(())
}
