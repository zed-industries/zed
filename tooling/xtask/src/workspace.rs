use std::fs;

use anyhow::{anyhow, Result};
use cargo_toml::{Manifest, Workspace};
use toml;

/// Returns the Cargo workspace.
pub fn load_workspace() -> Result<Workspace> {
    let workspace_cargo_toml = fs::read_to_string("Cargo.toml")?;
    let workspace_cargo_toml: Manifest = toml::from_str(&workspace_cargo_toml)?;

    let workspace = workspace_cargo_toml
        .workspace
        .ok_or_else(|| anyhow!("top-level Cargo.toml is not a Cargo workspace"))?;

    Ok(workspace)
}
