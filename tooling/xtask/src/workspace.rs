use anyhow::{Context as _, Result};
use cargo_metadata::{Metadata, MetadataCommand};

/// Returns the Cargo workspace.
pub fn load_workspace() -> Result<Metadata> {
    MetadataCommand::new()
        .exec()
        .context("failed to load cargo metadata")
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet, VecDeque},
        fs,
    };

    use anyhow::{Context as _, Result};
    use cargo_metadata::{CargoOpt, DependencyKind, Metadata, MetadataCommand};

    use super::load_workspace;

    const FORBIDDEN_OBJC_DEPENDENCIES: &[(&str, &str)] = &[
        ("block", "use `block2` instead"),
        (
            "cocoa",
            "use `objc2-app-kit` and `objc2-foundation` instead",
        ),
        ("cocoa-foundation", "use `objc2-foundation` instead"),
        ("metal", "use `objc2-metal` and `objc2-quartz-core` instead"),
        ("objc", "use `objc2` instead"),
        ("objc-foundation", "use `objc2-foundation` instead"),
        ("objc_id", "use `objc2::rc::Retained` instead"),
    ];

    /// Crates that must not depend on each other, directly or transitively:
    /// such edges chain large UI crates one after another and serialize the
    /// build, badly hurting incremental compile times. Dev-dependencies are
    /// exempt since they don't affect `cargo build`.
    const FORBIDDEN_DEPENDENCIES: &[(&str, &str)] = &[
        ("agent_ui", "git_ui"),
        ("file_finder", "project_panel"),
        ("git_ui", "agent_ui"),
        ("git_ui", "search"),
        ("open_path_prompt", "project_panel"),
        ("picker", "editor"),
        ("project_panel", "git_ui"),
        ("project_panel", "search"),
        ("search", "git_ui"),
        ("search", "project_panel"),
        ("sidebar", "git_ui"),
        ("title_bar", "git_ui"),
    ];

    #[test]
    fn no_forbidden_objc_dependencies() -> Result<()> {
        let workspace = MetadataCommand::new()
            .features(CargoOpt::AllFeatures)
            .exec()
            .context("loading cargo metadata with all workspace features")?;
        let violations = forbidden_objc_dependency_violations(&workspace)?;

        assert_eq!(
            violations,
            Vec::<String>::new(),
            "forbidden Objective-C dependencies:\n{}",
            violations.join("\n"),
        );
        Ok(())
    }

    #[test]
    fn forbidden_objc_dependencies_cover_manifests_and_transitive_packages() -> Result<()> {
        let fixture = tempfile::tempdir()?;
        for name in [
            "block",
            "cocoa",
            "cocoa-foundation",
            "objc",
            "objc-foundation",
            "objc_id",
            "block2",
            "objc2",
            "objc2-app-kit",
            "objc2-foundation",
            "objc2-metal",
            "objc2-quartz-core",
            "metal",
            "intermediate",
            "workspace",
        ] {
            let directory = fixture.path().join(name);
            fs::create_dir(&directory)?;
            fs::write(directory.join("lib.rs"), "")?;
            let dependencies = match name {
                "metal" => "[dependencies]\nobjc = { path = '../objc' }\n",
                "intermediate" => {
                    "[dependencies]\nmetal = { path = '../metal', optional = true }\n"
                }
                _ => "",
            };
            fs::write(
                directory.join("Cargo.toml"),
                format!(
                    "[package]\nname = '{name}'\nversion = '0.1.0'\nedition = '2024'\n\
                     [lib]\npath = 'lib.rs'\n{dependencies}"
                ),
            )?;
        }
        let manifest = fixture.path().join("workspace/Cargo.toml");
        let allowed = format!(
            "{}\n{}",
            fs::read_to_string(&manifest)?,
            r#"[workspace]
resolver = "2"
[dependencies]
block2 = { path = "../block2" }
objc2 = { path = "../objc2" }
objc2-app-kit = { path = "../objc2-app-kit" }
objc2-foundation = { path = "../objc2-foundation" }
objc2-metal = { path = "../objc2-metal" }
objc2-quartz-core = { path = "../objc2-quartz-core" }
"#,
        );
        let forbidden = r#"
[dependencies.block]
path = "../block"
[dependencies.legacy_cocoa]
package = "cocoa"
path = "../cocoa"
[dependencies.cocoa-foundation]
path = "../cocoa-foundation"
optional = true
[dependencies.legacy_metal]
package = "metal"
path = "../metal"
[target.'cfg(any())'.dependencies]
objc = { path = "../objc" }
[dev-dependencies]
objc-foundation = { path = "../objc-foundation" }
[target.'cfg(any())'.build-dependencies]
legacy_id = { workspace = true, optional = true }
[workspace.dependencies]
legacy_id = { package = "objc_id", path = "../objc_id" }
objc = "0.1.0"
unused_cocoa = { package = "cocoa", version = "0.1.0" }
unused_metal = { package = "metal", version = "0.1.0" }
unused_objc2 = { package = "objc2", version = "0.1.0" }
"#;
        fs::write(&manifest, format!("{allowed}{forbidden}"))?;
        let mut command = MetadataCommand::new();
        command
            .manifest_path(&manifest)
            .other_options(vec!["--offline".to_owned()]);
        let mut violations = forbidden_objc_dependency_violations(&command.exec()?)?;
        violations.sort();
        assert_eq!(
            violations,
            [
                "Cargo.toml [workspace.dependencies]: `legacy_id` uses `objc_id`; use `objc2::rc::Retained` instead",
                "Cargo.toml [workspace.dependencies]: `objc` uses `objc`; use `objc2` instead",
                "Cargo.toml [workspace.dependencies]: `unused_cocoa` uses `cocoa`; use `objc2-app-kit` and `objc2-foundation` instead",
                "Cargo.toml [workspace.dependencies]: `unused_metal` uses `metal`; use `objc2-metal` and `objc2-quartz-core` instead",
                "workspace directly depends on `block`; use `block2` instead",
                "workspace directly depends on `cocoa-foundation`; use `objc2-foundation` instead",
                "workspace directly depends on `cocoa`; use `objc2-app-kit` and `objc2-foundation` instead",
                "workspace directly depends on `metal`; use `objc2-metal` and `objc2-quartz-core` instead",
                "workspace directly depends on `objc-foundation`; use `objc2-foundation` instead",
                "workspace directly depends on `objc_id`; use `objc2::rc::Retained` instead",
                "workspace directly depends on `objc`; use `objc2` instead",
            ],
            "including renamed optional inactive-target build dependency",
        );
        fs::write(
            &manifest,
            format!(
                "{allowed}\n[target.'cfg(any())'.dependencies]\n\
                 intermediate = {{ path = '../intermediate', optional = true }}\n\
                 [features]\nlegacy = ['dep:intermediate', 'intermediate/metal']\n"
            ),
        )?;
        assert_eq!(
            forbidden_objc_dependency_violations(&command.exec()?)?,
            Vec::<String>::new(),
        );
        command.features(CargoOpt::AllFeatures);
        let mut violations = forbidden_objc_dependency_violations(&command.exec()?)?;
        violations.sort();
        assert_eq!(
            violations,
            [
                "dependency graph contains `metal`; use `objc2-metal` and `objc2-quartz-core` instead",
                "dependency graph contains `objc`; use `objc2` instead",
            ],
        );
        fs::write(&manifest, allowed)?;
        assert_eq!(
            forbidden_objc_dependency_violations(&command.exec()?)?,
            Vec::<String>::new(),
        );
        Ok(())
    }

    #[test]
    fn no_test_support_in_non_dev_dependencies() {
        let workspace = load_workspace().expect("failed to load cargo metadata");
        let mut violations = Vec::new();

        for package in workspace.workspace_packages() {
            for dependency in &package.dependencies {
                if dependency.kind == DependencyKind::Development
                    || !dependency
                        .features
                        .iter()
                        .any(|feature| feature == "test-support")
                {
                    continue;
                }

                let manifest = package
                    .manifest_path
                    .strip_prefix(&workspace.workspace_root)
                    .unwrap_or(&package.manifest_path);
                let name = dependency.rename.as_deref().unwrap_or(&dependency.name);
                let target = dependency
                    .target
                    .as_ref()
                    .map(|target| format!(" for {target}"))
                    .unwrap_or_default();
                violations.push(format!(
                    "{manifest}: `{name}` enables `test-support` in {:?} dependencies{target}",
                    dependency.kind,
                ));
            }
        }

        assert!(
            violations.is_empty(),
            "non-dev dependencies must not enable `test-support`; \
             use dev-dependencies or explicit feature forwarding instead:\n{violations:?}",
        );
    }

    #[test]
    fn no_forbidden_dependencies_between_feature_crates() {
        let workspace = load_workspace().expect("failed to load cargo metadata");
        let packages = workspace.workspace_packages();
        let member_names = packages
            .iter()
            .map(|package| package.name.as_str())
            .collect::<BTreeSet<_>>();

        let mut graph = BTreeMap::new();
        for package in &packages {
            let dependencies = package
                .dependencies
                .iter()
                .filter(|dependency| dependency.kind != DependencyKind::Development)
                .map(|dependency| dependency.name.as_str())
                .filter(|name| member_names.contains(name))
                .collect::<BTreeSet<_>>();
            graph.insert(package.name.as_str(), dependencies);
        }

        let mut violations = Vec::new();
        for &(from, to) in FORBIDDEN_DEPENDENCIES {
            if let Some(path) = dependency_path(&graph, from, to) {
                violations.push(path.join(" -> "));
            }
        }
        assert_eq!(
            violations,
            Vec::<String>::new(),
            "forbidden dependency paths between sibling feature crates; \
             break the dependency (e.g. by extracting shared code into a lower-level crate) \
             instead of joining these crates into one serial build chain",
        );
    }

    fn forbidden_objc_dependency_violations(workspace: &Metadata) -> Result<Vec<String>> {
        let mut violations = Vec::new();
        let mut reported_dependencies = BTreeSet::new();
        for package in workspace.workspace_packages() {
            for dependency in &package.dependencies {
                let Some((_, reason)) = FORBIDDEN_OBJC_DEPENDENCIES
                    .iter()
                    .find(|(name, _)| *name == dependency.name)
                else {
                    continue;
                };
                violations.push(format!(
                    "{} directly depends on `{}`; {reason}",
                    package.name, dependency.name
                ));
                reported_dependencies.insert(dependency.name.as_str());
            }
        }
        for package in &workspace.packages {
            let Some((_, reason)) = FORBIDDEN_OBJC_DEPENDENCIES
                .iter()
                .find(|(name, _)| *name == package.name.as_str())
            else {
                continue;
            };
            if reported_dependencies.insert(package.name.as_str()) {
                violations.push(format!(
                    "dependency graph contains `{}`; {reason}",
                    package.name,
                ));
            }
        }

        let manifest_path = workspace.workspace_root.join("Cargo.toml");
        let contents = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read {manifest_path}"))?;
        let manifest = toml::from_str::<toml::Value>(&contents)
            .with_context(|| format!("failed to parse {manifest_path}"))?;
        let dependencies = manifest
            .get("workspace")
            .and_then(|workspace| workspace.get("dependencies"))
            .and_then(toml::Value::as_table);
        for (alias, dependency) in dependencies.into_iter().flatten() {
            let name = dependency
                .get("package")
                .and_then(toml::Value::as_str)
                .unwrap_or(alias);
            let Some((_, reason)) = FORBIDDEN_OBJC_DEPENDENCIES
                .iter()
                .find(|(forbidden, _)| *forbidden == name)
            else {
                continue;
            };
            violations.push(format!(
                "Cargo.toml [workspace.dependencies]: `{alias}` uses `{name}`; {reason}"
            ));
        }
        Ok(violations)
    }

    fn dependency_path<'a>(
        graph: &BTreeMap<&'a str, BTreeSet<&'a str>>,
        from: &'a str,
        to: &str,
    ) -> Option<Vec<&'a str>> {
        let mut parents = BTreeMap::new();
        let mut queue = VecDeque::from([from]);
        while let Some(current) = queue.pop_front() {
            if current == to {
                let mut path = vec![current];
                let mut node = current;
                while let Some(&parent) = parents.get(node) {
                    path.push(parent);
                    node = parent;
                }
                path.reverse();
                return Some(path);
            }
            for &dependency in graph.get(current).into_iter().flatten() {
                if dependency != from && !parents.contains_key(dependency) {
                    parents.insert(dependency, current);
                    queue.push_back(dependency);
                }
            }
        }
        None
    }
}
