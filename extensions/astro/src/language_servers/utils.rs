use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use zed_extension_api::{self as zed, serde_json, Result};

pub const TYPESCRIPT_PACKAGE_NAME: &str = "typescript";
pub const TYPESCRIPT_TSDK_PATH: &str = "node_modules/typescript/lib";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

pub fn typescript_exists_for_worktree(worktree: &zed::Worktree) -> Result<bool> {
    let package_json = worktree.read_text_file("package.json")?;
    let package_json: PackageJson = serde_json::from_str(&package_json)
        .map_err(|err| format!("failed to parse package.json: {err}"))?;

    let dev_dependencies = &package_json.dev_dependencies;
    let dependencies = &package_json.dependencies;

    Ok(dev_dependencies.contains_key(TYPESCRIPT_PACKAGE_NAME)
        || dependencies.contains_key(TYPESCRIPT_PACKAGE_NAME))
}

pub fn install_typescript_if_needed(worktree: &zed::Worktree) -> Result<String> {
    if typescript_exists_for_worktree(worktree).unwrap_or_default() {
        println!("found local TypeScript installation at '{TYPESCRIPT_TSDK_PATH}'");
        return Ok(TYPESCRIPT_TSDK_PATH.to_owned());
    }

    let installed_typescript_version = zed::npm_package_installed_version(TYPESCRIPT_PACKAGE_NAME)?;
    let latest_typescript_version = zed::npm_package_latest_version(TYPESCRIPT_PACKAGE_NAME)?;

    if installed_typescript_version.as_ref() != Some(&latest_typescript_version) {
        println!("installing {TYPESCRIPT_PACKAGE_NAME}@{latest_typescript_version}");
        zed::npm_install_package(TYPESCRIPT_PACKAGE_NAME, &latest_typescript_version)?;
    } else {
        println!("typescript already installed");
    }

    Ok(env::current_dir()
        .unwrap()
        .join(TYPESCRIPT_TSDK_PATH)
        .to_string_lossy()
        .to_string())
}
