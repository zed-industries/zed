use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use fs::Fs;
use gpui::{AsyncWindowContext, Model, Task};
use project::{Project, ProjectPath};

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
pub struct ProjectMetadata {
    pub name: Option<String>,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub license: Option<String>,
    pub dependencies: Vec<String>,
}

impl ProjectMetadata {
    pub fn new() -> Self {
        ProjectMetadata {
            name: None,
            description: None,
            authors: Vec::new(),
            version: None,
            license: None,
            dependencies: Vec::new(),
        }
    }

    pub fn render_as_string(&self) -> String {
        let mut prompt = "You are in a Rust project".to_string();
        if let Some(name) = self.name.as_ref() {
            prompt.push_str(&format!(" named \"{name}\""));
        }
        prompt.push_str(". ");

        if let Some(description) = self.description.as_ref() {
            prompt.push_str("It describes itself as ");
            prompt.push_str(&format!("\"{description}\""));
            prompt.push_str(". ");
        }

        if !self.dependencies.is_empty() {
            prompt.push_str("The following dependencies are installed: ");
            prompt.push_str(&self.dependencies.join(", "));
            prompt.push_str(". ");
        }

        prompt
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
) -> Result<Task<Result<ProjectMetadata>>> {
    let path_to_cargo_toml = cx.update(|cx| {
        let worktree = project
            .read(cx)
            .worktrees()
            .next()
            .ok_or_else(|| anyhow!("no worktree"))?;

        let path_to_cargo_toml = worktree.update(cx, |worktree, cx| {
            let cargo_toml = worktree.entry_for_path("Cargo.toml")?;
            Some(ProjectPath {
                worktree_id: worktree.id(),
                path: cargo_toml.path.clone(),
            })
        });
        let path_to_cargo_toml =
            path_to_cargo_toml.and_then(|path| project.read(cx).absolute_path(&path, cx));

        anyhow::Ok(path_to_cargo_toml)
    })??;

    let path_to_cargo_toml = path_to_cargo_toml.ok_or_else(|| anyhow!("no Cargo.toml"))?;

    Ok(cx.spawn(|_cx| async move {
        let project_info = populate_project_metadata(fs, &path_to_cargo_toml).await?;

        anyhow::Ok(project_info)
    }))
}

async fn populate_project_metadata(fs: Arc<dyn Fs>, path: &Path) -> Result<ProjectMetadata> {
    let buffer = fs.load(path).await?;

    let cargo_toml: cargo_toml::Manifest = toml::from_str(&buffer)?;

    dbg!(&cargo_toml);

    Ok(ProjectMetadata {
        name: cargo_toml
            .package
            .as_ref()
            .map(|package| package.name.clone()),
        authors: cargo_toml
            .package
            .as_ref()
            .and_then(|package| package.authors.get().ok().cloned())
            .unwrap_or_default(),
        description: cargo_toml
            .package
            .as_ref()
            .and_then(|package| package.description.as_ref())
            .and_then(|description| description.get().ok().cloned()),
        version: cargo_toml
            .package
            .as_ref()
            .and_then(|package| package.version.get().ok().cloned()),
        license: cargo_toml
            .package
            .as_ref()
            .and_then(|package| package.license.as_ref())
            .and_then(|license| license.get().ok().cloned()),
        dependencies: cargo_toml.dependencies.keys().cloned().collect(),
    })
}
