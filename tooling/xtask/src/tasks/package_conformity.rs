use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context as _, Result};
use cargo_toml::{Dependency, Manifest};
use clap::Parser;

use crate::workspace::load_workspace;

#[derive(Parser)]
pub struct PackageConformityArgs {}

pub fn run_package_conformity(_args: PackageConformityArgs) -> Result<()> {
    let workspace = load_workspace()?;

    let mut non_workspace_dependencies = BTreeMap::new();

    for package in workspace.workspace_packages() {
        let is_extension = package
            .manifest_path
            .parent()
            .and_then(|parent| parent.parent())
            .is_some_and(|grandparent_dir| grandparent_dir.ends_with("extensions"));

        let cargo_toml = read_cargo_toml(&package.manifest_path)?;

        let is_using_workspace_lints = cargo_toml.lints.is_some_and(|lints| lints.workspace);
        if !is_using_workspace_lints {
            eprintln!(
                "{package:?} is not using workspace lints",
                package = package.name
            );
        }

        // Extensions should not use workspace dependencies.
        if is_extension || package.name == "zed_extension_api" {
            continue;
        }

        // Ignore `workspace-hack`, as it produces a lot of false positives.
        if package.name == "workspace-hack" {
            continue;
        }

        for dependencies in [
            &cargo_toml.dependencies,
            &cargo_toml.dev_dependencies,
            &cargo_toml.build_dependencies,
        ] {
            for (name, dependency) in dependencies {
                if let Dependency::Inherited(_) = dependency {
                    continue;
                }

                non_workspace_dependencies
                    .entry(name.to_owned())
                    .or_insert_with(Vec::new)
                    .push(package.name.clone());
            }
        }
    }

    for (dependency, packages) in non_workspace_dependencies {
        eprintln!(
            "{dependency} is being used as a non-workspace dependency: {}",
            packages.join(", ")
        );
    }

    Ok(())
}

/// Returns the contents of the `Cargo.toml` file at the given path.
fn read_cargo_toml(path: impl AsRef<Path>) -> Result<Manifest> {
    let path = path.as_ref();
    let cargo_toml_bytes = fs::read(path)?;
    Manifest::from_slice(&cargo_toml_bytes)
        .with_context(|| format!("reading Cargo.toml at {path:?}"))
}
