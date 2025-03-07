use anyhow::Result;
use clap::{Parser, ValueEnum};
use schemars::schema_for;
use theme::{IconThemeFamilyContent, ThemeFamilyContent};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(value_enum)]
    pub schema_type: SchemaType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum SchemaType {
    Theme,
    IconTheme,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    match args.schema_type {
        SchemaType::Theme => {
            let schema = schema_for!(ThemeFamilyContent);
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
        SchemaType::IconTheme => {
            let schema = schema_for!(IconThemeFamilyContent);
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
    }

    Ok(())
}
