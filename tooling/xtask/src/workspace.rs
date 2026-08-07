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
    use std::collections::{BTreeMap, BTreeSet, VecDeque};

    use cargo_metadata::DependencyKind;

    use super::load_workspace;

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
