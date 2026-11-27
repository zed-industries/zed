use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context as _, Result, ensure};
use toml_edit::{DocumentMut, Item, TableLike, value};

use super::release::{DependencyEdit, Release};

pub(super) fn prepare(plan: &Path, repository: &str, checkout: &Path) -> Result<()> {
    let release: Release = serde_json::from_slice(&fs::read(plan)?)?;
    let group = release
        .repositories
        .iter()
        .find(|group| group.repository == repository)
        .with_context(|| format!("{repository} is not in the GPUI release plan"))?;
    let head = super::run_command(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(checkout)
            .stdout(std::process::Stdio::piped()),
    )?;
    ensure!(
        std::str::from_utf8(&head.stdout)?.trim() == group.revision,
        "{repository} checkout does not match planned revision {}",
        group.revision
    );

    let mut documents = BTreeMap::new();
    for manifest in group
        .dependencies
        .keys()
        .chain(group.crates.iter().map(|krate| &krate.manifest))
        .map(PathBuf::as_path)
        .chain([Path::new("Cargo.toml")])
        .collect::<BTreeSet<_>>()
    {
        ensure!(
            manifest
                .components()
                .all(|component| matches!(component, std::path::Component::Normal(_))),
            "invalid repository-relative manifest path: {}",
            manifest.display()
        );
        let path = checkout.join(manifest);
        let contents = fs::read_to_string(&path)?;
        let document: DocumentMut = contents
            .parse()
            .with_context(|| format!("invalid manifest {}", path.display()))?;
        documents.insert(manifest.to_owned(), (contents, document));
    }

    let version = release.version.to_string();
    for krate in &group.crates {
        let (_, document) = documents
            .get_mut(&krate.manifest)
            .context("missing crate manifest")?;
        *document =
            super::update_crate_package_fields(&document.to_string(), &krate.name, &version)?
                .parse()?;
        if krate.original_name != krate.name {
            let library = document.get_mut("lib").and_then(Item::as_table_like_mut);
            if let Some(library) = library
                && !library.contains_key("name")
            {
                library.insert("name", value(krate.original_name.replace('-', "_")));
            } else if document.get("lib").is_none()
                && checkout
                    .join(&krate.manifest)
                    .with_file_name("src")
                    .join("lib.rs")
                    .is_file()
            {
                document["lib"]["name"] = value(krate.original_name.replace('-', "_"));
            }
        }
    }

    let mut inherited = BTreeMap::new();
    for (manifest, edits) in &group.dependencies {
        let (_, document) = documents
            .get_mut(manifest)
            .context("missing dependency manifest")?;
        visit_dependencies(document.as_table_mut(), |dependencies, development| {
            for (alias, edit) in edits {
                let Some(specification) = dependencies.get_mut(alias) else {
                    continue;
                };
                if specification.get("workspace").and_then(Item::as_bool) == Some(true) {
                    if let Some(previous) = inherited.insert(alias, edit) {
                        ensure!(previous == edit, "conflicting inherited dependency {alias}");
                    }
                } else if dependency_name(specification, alias) == edit.original_name {
                    rewrite_dependency(specification, edit, &version, development)?;
                }
            }
            Ok(())
        })?;
    }
    let (_, root) = documents
        .get_mut(Path::new("Cargo.toml"))
        .context("missing root manifest")?;
    let mut development_paths = BTreeMap::new();
    for (alias, edit) in inherited {
        let specification = root
            .get_mut("workspace")
            .and_then(|workspace| workspace.get_mut("dependencies"))
            .and_then(|dependencies| dependencies.get_mut(alias))
            .with_context(|| format!("missing inherited dependency {alias}"))?;
        rewrite_dependency(specification, edit, &version, false)?;
        if specification.get("path").is_some() {
            development_paths.insert(alias, specification.clone());
        }
    }
    for krate in &group.crates {
        if krate.original_name != krate.name {
            super::update_profile_override_in_doc(root, &krate.original_name, &krate.name)?;
        }
    }

    // Cargo strips versionless path dev-dependencies from the archive. Leaving a version
    // on a dev-only edge can require a crate that cannot be published until after this one.
    for manifest in group.dependencies.keys() {
        let (_, document) = documents
            .get_mut(manifest)
            .context("missing dependency manifest")?;
        visit_dependencies(document.as_table_mut(), |dependencies, development| {
            if development {
                for (alias, shared) in &development_paths {
                    if let Some(specification) = dependencies.get_mut(alias)
                        && specification.get("workspace").and_then(Item::as_bool) == Some(true)
                    {
                        expand_development_path(manifest, specification, shared)?;
                    }
                }
            }
            Ok(())
        })?;
    }

    for (manifest, (original, document)) in documents {
        let contents = document.to_string();
        if contents != original {
            fs::write(checkout.join(manifest), contents)?;
        }
    }
    Ok(())
}

fn expand_development_path(manifest: &Path, specification: &mut Item, shared: &Item) -> Result<()> {
    let mut expanded = shared.clone();
    let shared = expanded
        .as_table_like_mut()
        .context("invalid workspace path dependency")?;
    let path = shared
        .get("path")
        .and_then(Item::as_str)
        .context("missing workspace dependency path")?;
    let parents = manifest.parent().context("missing manifest directory")?;
    let path = format!("{}{path}", "../".repeat(parents.components().count()));
    shared.insert("path", value(path));
    shared.remove("version");
    let local = specification
        .as_table_like()
        .context("invalid inherited dev-dependency")?;
    for (key, item) in local.iter() {
        if key == "workspace" {
            continue;
        }
        if key == "features"
            && let Some(shared_features) = shared.get_mut("features").and_then(Item::as_array_mut)
            && let Some(local_features) = item.as_array()
        {
            shared_features.extend(local_features.iter().cloned());
        } else {
            shared.insert(key, item.clone());
        }
    }
    *specification = Item::Value(
        expanded
            .into_value()
            .map_err(|_| anyhow::anyhow!("invalid workspace path dependency"))?,
    );
    Ok(())
}

fn dependency_name<'a>(specification: &'a Item, alias: &'a str) -> &'a str {
    specification
        .get("package")
        .and_then(Item::as_str)
        .unwrap_or(alias)
}

fn visit_dependencies(
    manifest: &mut dyn TableLike,
    mut visit: impl FnMut(&mut dyn TableLike, bool) -> Result<()>,
) -> Result<()> {
    fn visit_tables(
        table: &mut dyn TableLike,
        visit: &mut impl FnMut(&mut dyn TableLike, bool) -> Result<()>,
    ) -> Result<()> {
        for (key, development) in [
            ("dependencies", false),
            ("build-dependencies", false),
            ("dev-dependencies", true),
        ] {
            if let Some(dependencies) = table.get_mut(key).and_then(Item::as_table_like_mut) {
                visit(dependencies, development)?;
            }
        }
        Ok(())
    }
    visit_tables(manifest, &mut visit)?;
    if let Some(targets) = manifest.get_mut("target").and_then(Item::as_table_like_mut) {
        for (_, target) in targets.iter_mut() {
            if let Some(target) = target.as_table_like_mut() {
                visit_tables(target, &mut visit)?;
            }
        }
    }
    Ok(())
}

fn rewrite_dependency(
    specification: &mut Item,
    edit: &DependencyEdit,
    version: &str,
    development: bool,
) -> Result<()> {
    // A registry declaration can resolve to Git through [patch]. Renaming only the
    // declarations we own would split the crate's identity from registry dependents.
    if specification.get("git").is_none() && specification.get("path").is_none() {
        return Ok(());
    }
    let table = specification
        .as_table_like_mut()
        .context("invalid dependency specification")?;
    for key in ["git", "rev", "branch", "tag", "registry"] {
        table.remove(key);
    }
    if !edit.same_repository {
        table.remove("path");
    }
    table.insert("package", value(&edit.name));
    if development && table.contains_key("path") {
        table.remove("version");
    } else {
        table.insert("version", value(format!("={version}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_dependencies_without_changing_their_aliases() -> Result<()> {
        let mut manifest: DocumentMut = indoc::indoc! {r#"
            [dependencies]
            local = { path = "crates/local", features = ["default-feature"] }
            fork = { git = "https://github.com/zed-industries/fork", rev = "abc", optional = true }

            [dev-dependencies]
            local = { workspace = true, features = ["test-support"] }
        "#}
        .parse()?;
        let local = DependencyEdit {
            original_name: "local".into(),
            name: "gpui_local".into(),
            same_repository: true,
        };
        let fork = DependencyEdit {
            original_name: "fork".into(),
            name: "zed-fork".into(),
            same_repository: false,
        };
        rewrite_dependency(
            &mut manifest["dependencies"]["local"],
            &local,
            "0.3.0",
            false,
        )?;
        rewrite_dependency(&mut manifest["dependencies"]["fork"], &fork, "0.3.0", false)?;
        let fork = &manifest["dependencies"]["fork"];
        assert!(fork.get("git").is_none() && fork.get("rev").is_none());
        assert_eq!(fork["package"].as_str(), Some("zed-fork"));
        assert_eq!(fork["version"].as_str(), Some("=0.3.0"));
        assert_eq!(fork["optional"].as_bool(), Some(true));

        let shared = manifest["dependencies"]["local"].clone();
        let development = &mut manifest["dev-dependencies"]["local"];
        expand_development_path(Path::new("crates/main/Cargo.toml"), development, &shared)?;
        assert_eq!(development["package"].as_str(), Some("gpui_local"));
        assert_eq!(development["path"].as_str(), Some("../../crates/local"));
        assert!(development.get("version").is_none());
        let features = development["features"]
            .as_array()
            .context("missing features")?;
        assert_eq!(
            features
                .iter()
                .filter_map(|feature| feature.as_str())
                .collect::<Vec<_>>(),
            ["default-feature", "test-support"]
        );
        Ok(())
    }
}
