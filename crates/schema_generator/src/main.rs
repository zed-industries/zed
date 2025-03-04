use anyhow::Result;
use clap::Parser;
use schemars::schema_for;
use theme::{IconThemeFamilyContent, ThemeFamilyContent};

#[derive(Parser, Debug)]
struct Args {}

fn main() -> Result<()> {
    env_logger::init();

    let _args = Args::parse();

    let theme_family_schema = schema_for!(ThemeFamilyContent);
    println!("Theme Schema:");
    println!("{}", serde_json::to_string_pretty(&theme_family_schema)?);

    let icon_theme_family_schema = schema_for!(IconThemeFamilyContent);
    println!("Icon Theme Schema:");
    println!(
        "{}",
        serde_json::to_string_pretty(&icon_theme_family_schema)?
    );

    Ok(())
}
