use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use collections::HashMap;
use fs::Fs;
use gpui::{AsyncWindowContext, Model};
use project::Project;
use serde::Deserialize;

// Let's get all the useful info we can about the currently open Rust project
//
// Find all the Cargo.tomls in the project
//  - metadata (name, version, authors, description, license, etc.)
//    - edition
//    - rust-version
//  - workspace vs non-workspace
//    - original crates?
//  - dependencies & their features
//
// Summarize the Readme and evaluate if any of that information is useful
// Are there things the Cargo.lock gives us that we don't get from Cargo.toml?
//
// Output:
//
// - What is this project?
// - How is it configured?
//  - Deps, features, etc.
//  - Coding styles (lint rules, formatting, etc.)
//

// Identify this as a rust project by finding a Cargo.toml
// Load the root cargo.toml

#[derive(Debug)]
struct ProjectMetadata {
    pub name: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
}

impl ProjectMetadata {
    pub fn new() -> Self {
        ProjectMetadata {
            name: None,
            description: None,
            authors: Vec::new(),
            version: None,
            license: None,
        }
    }
}

#[derive(Debug)]
struct ProjectInfo {
    pub metadata: Option<ProjectMetadata>,
    pub languages: Vec<String>,
}

pub fn identify_project(
    fs: Arc<dyn Fs>,
    project: Model<Project>,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let worktree = cx.update(|cx| {
        project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .ok_or_else(|| anyhow!("no worktree"))
    })??;

    let cargo_toml = worktree.update(cx, |worktree, _cx| {
        worktree.entry_for_path("Cargo.toml").cloned()
    })?;

    dbg!(&cargo_toml);

    cx.spawn(|cx| async move {
        let cargo_toml = cargo_toml.ok_or_else(|| anyhow!("no Cargo.toml"))?;

        let project_info = populate_project_metadata(fs, &cargo_toml.path).await?;

        dbg!(&project_info);

        anyhow::Ok(())
    })
    .detach();

    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct CargoToml {
    package: Option<Package>,
    workspace: Option<Workspace>,
}

#[derive(Debug, Clone, Deserialize)]
struct Package {
    name: Option<String>,
    version: Option<String>,
    authors: Option<Vec<String>>,
    edition: Option<String>,
    license: Option<String>,
    description: Option<String>,
    #[serde(rename = "rust-version")]
    rust_version: Option<String>,
    dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Version(String),
    Details(DependencyDetails),
}

#[derive(Debug, Clone, Deserialize)]
struct DependencyDetails {
    version: Option<String>,
    features: Option<Vec<String>>,
    optional: Option<bool>,
    default_features: Option<bool>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Workspace {
    members: Option<Vec<String>>,
    #[serde(rename = "default-members")]
    default_members: Option<Vec<String>>,
}

async fn populate_project_metadata(fs: Arc<dyn Fs>, path: &Path) -> Result<ProjectMetadata> {
    let buffer = fs.load(path).await?;

    let cargo_toml: CargoToml = toml::from_str(&buffer)?;

    dbg!(&cargo_toml);

    Ok(ProjectMetadata {
        name: cargo_toml.package.clone().and_then(|package| package.name),
        authors: cargo_toml
            .package
            .clone()
            .and_then(|package| package.authors)
            .unwrap_or_default(),
        description: cargo_toml
            .package
            .clone()
            .and_then(|package| package.description),
        version: cargo_toml
            .package
            .clone()
            .and_then(|package| package.version),
        license: cargo_toml.package.and_then(|package| package.license),
    })
}
