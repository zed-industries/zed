use anyhow::{Context as _, Result, ensure};
use guppy::{
    MetadataCommand,
    graph::{DependencyDirection, ExternalSource, PackageGraph, PackageSet, PackageSource},
};
use url::Url;

pub fn load_workspace_graph() -> Result<PackageGraph> {
    MetadataCommand::new()
        .build_graph()
        .context("failed to load the workspace crate graph")
}

/// Returns all GPUI crates in the current workspace
pub fn gpui_crates(graph: &PackageGraph) -> Result<PackageSet<'_>> {
    let roots = graph
        .workspace()
        .iter_by_name()
        .filter(|(name, _)| name.starts_with("gpui"))
        .map(|(_, package)| package.id())
        .collect::<Vec<_>>();
    ensure!(!roots.is_empty(), "no GPUI crates found in the workspace");

    let dependencies = graph
        .query_forward(roots)?
        // we do not need dev dependencies for releases
        .resolve_with_fn(|_, link| !link.dev_only());

    // Keep all workspace crates and Zed forks
    Ok(
        dependencies.filter(DependencyDirection::Reverse, |package| {
            package.in_workspace() || is_zed_fork(package.source())
        }),
    )
}

fn is_zed_fork(source: PackageSource<'_>) -> bool {
    let Some(ExternalSource::Git { repository, .. }) = source.parse_external() else {
        return false;
    };

    Url::parse(repository).is_ok_and(|repository| {
        repository
            .host_str()
            .is_some_and(|host| host.eq_ignore_ascii_case("github.com"))
            && repository.path_segments().is_some_and(|mut segments| {
                segments
                    .next()
                    .is_some_and(|owner| owner.eq_ignore_ascii_case("zed-industries"))
                    && segments.next().is_some_and(|name| !name.is_empty())
            })
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use DependencyKind::{Build, Development, Normal};
    use DependencyRequirement::{Optional, Required};
    use DependencyTarget::{All, Platform};
    use PackageOrigin::{CratesIo, Git, Path, Workspace};

    struct FixturePackage {
        name: &'static str,
        origin: PackageOrigin,
        dependencies: &'static [FixtureDependency],
    }

    enum PackageOrigin {
        Workspace,
        Path,
        CratesIo,
        Git(&'static str),
    }

    struct FixtureDependency {
        to: &'static str,
        kind: DependencyKind,
        target: DependencyTarget,
    }

    enum DependencyKind {
        Normal(DependencyRequirement),
        Build(DependencyRequirement),
        Development,
    }

    enum DependencyRequirement {
        Required,
        Optional,
    }

    enum DependencyTarget {
        All,
        Platform(&'static str),
    }

    #[test]
    fn selects_release_dependencies_in_topological_order() -> Result<()> {
        let graph = fixture(&[
            FixturePackage {
                name: "gpui",
                origin: Workspace,
                dependencies: &[
                    FixtureDependency {
                        to: "gpui_platform",
                        kind: Development,
                        target: All,
                    },
                    FixtureDependency {
                        to: "gpui_macros",
                        kind: Normal(Required),
                        target: All,
                    },
                    FixtureDependency {
                        to: "collections",
                        kind: Normal(Required),
                        target: All,
                    },
                    FixtureDependency {
                        to: "collections",
                        kind: Development,
                        target: All,
                    },
                    FixtureDependency {
                        to: "build_helper",
                        kind: Build(Required),
                        target: All,
                    },
                    FixtureDependency {
                        to: "optional_helper",
                        kind: Normal(Optional),
                        target: Platform("cfg(windows)"),
                    },
                    FixtureDependency {
                        to: "test_helper",
                        kind: Development,
                        target: All,
                    },
                    FixtureDependency {
                        to: "local_dependency",
                        kind: Normal(Required),
                        target: All,
                    },
                    FixtureDependency {
                        to: "registry",
                        kind: Normal(Required),
                        target: All,
                    },
                    FixtureDependency {
                        to: "upstream",
                        kind: Normal(Required),
                        target: All,
                    },
                ],
            },
            FixturePackage {
                name: "gpui_platform",
                origin: Workspace,
                dependencies: &[FixtureDependency {
                    to: "gpui",
                    kind: Normal(Required),
                    target: All,
                }],
            },
            FixturePackage {
                name: "gpui_macros",
                origin: Workspace,
                dependencies: &[FixtureDependency {
                    to: "gpui",
                    kind: Development,
                    target: All,
                }],
            },
            FixturePackage {
                name: "collections",
                origin: Workspace,
                dependencies: &[],
            },
            FixturePackage {
                name: "build_helper",
                origin: Workspace,
                dependencies: &[],
            },
            FixturePackage {
                name: "optional_helper",
                origin: Workspace,
                dependencies: &[],
            },
            FixturePackage {
                name: "test_helper",
                origin: Workspace,
                dependencies: &[],
            },
            FixturePackage {
                name: "zed",
                origin: Workspace,
                dependencies: &[FixtureDependency {
                    to: "gpui",
                    kind: Normal(Required),
                    target: All,
                }],
            },
            FixturePackage {
                name: "local_dependency",
                origin: Path,
                dependencies: &[],
            },
            FixturePackage {
                name: "registry",
                origin: CratesIo,
                dependencies: &[FixtureDependency {
                    to: "fork",
                    kind: Normal(Required),
                    target: All,
                }],
            },
            FixturePackage {
                name: "fork",
                origin: Git("git+https://github.com/zed-industries/fork?rev=abc#abc"),
                dependencies: &[FixtureDependency {
                    to: "collections",
                    kind: Normal(Required),
                    target: All,
                }],
            },
            FixturePackage {
                name: "upstream",
                origin: Git("git+https://github.com/other/upstream#abc"),
                dependencies: &[],
            },
            FixturePackage {
                name: "gpui_unrelated",
                origin: Git("git+https://github.com/zed-industries/unrelated#abc"),
                dependencies: &[],
            },
        ])?;
        let crates = gpui_crates(&graph)?;
        let names = crates
            .packages(DependencyDirection::Reverse)
            .map(|package| package.name())
            .collect::<Vec<_>>();
        assert_eq!(
            names
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>(),
            [
                "gpui",
                "gpui_platform",
                "gpui_macros",
                "collections",
                "build_helper",
                "optional_helper",
                "fork"
            ]
            .into_iter()
            .collect(),
        );
        for (dependency, dependent) in [
            ("gpui", "gpui_platform"),
            ("gpui_macros", "gpui"),
            ("collections", "fork"),
            ("fork", "gpui"),
            ("build_helper", "gpui"),
            ("optional_helper", "gpui"),
        ] {
            let dependency_index = names
                .iter()
                .position(|name| *name == dependency)
                .context("missing dependency")?;
            let dependent_index = names
                .iter()
                .position(|name| *name == dependent)
                .context("missing dependent")?;
            assert!(
                dependency_index < dependent_index,
                "{dependency} must precede {dependent}"
            );
        }
        assert!(crates.links(DependencyDirection::Reverse).all(|link| {
            names.contains(&link.from().name()) && names.contains(&link.to().name())
        }));
        Ok(())
    }

    #[test]
    fn requires_workspace_gpui_roots() -> Result<()> {
        let graph = fixture(&[FixturePackage {
            name: "zed",
            origin: Workspace,
            dependencies: &[],
        }])?;
        assert!(gpui_crates(&graph).is_err());
        Ok(())
    }

    #[test]
    fn identifies_forks_by_git_source_not_repository_metadata() {
        for source in [
            "git+https://github.com/zed-industries/fork?rev=abc#abc",
            "git+https://github.com/zed-industries/fork.git?branch=main#abc",
            "git+ssh://git@github.com/zed-industries/fork?tag=v1#abc",
            "git+https://github.com/Zed-Industries/fork#abc",
        ] {
            assert!(is_zed_fork(PackageSource::External(source)), "{source}");
        }
        for source in [
            PackageSource::CRATES_IO_REGISTRY,
            "sparse+https://index.crates.io/",
            "git+https://github.com/other/fork#abc",
            "git+https://github.com/zed-industries-other/fork#abc",
            "git+https://github.com.example.org/zed-industries/fork#abc",
            "git+https://example.org/github.com/zed-industries/fork#abc",
            "git+https://github.com/zed-industries/#abc",
        ] {
            assert!(!is_zed_fork(PackageSource::External(source)), "{source}");
        }
    }

    fn fixture(packages: &[FixturePackage]) -> Result<PackageGraph> {
        let mut nodes = Vec::new();
        let mut metadata = Vec::new();
        for package in packages {
            let name = package.name;
            let source = match package.origin {
                Workspace | Path => None,
                CratesIo => Some(PackageSource::CRATES_IO_REGISTRY),
                Git(source) => Some(source),
            };
            let mut dependencies = Vec::new();
            let mut resolved = std::collections::BTreeMap::<&str, Value>::new();
            for dependency in package.dependencies {
                let to = dependency.to;
                let (kind, optional) = match &dependency.kind {
                    Normal(requirement) => (None, matches!(requirement, Optional)),
                    Build(requirement) => (Some("build"), matches!(requirement, Optional)),
                    Development => (Some("dev"), false),
                };
                let target = match dependency.target {
                    All => None,
                    Platform(target) => Some(target),
                };
                dependencies.push(json!({
                    "name": to, "source": null, "req": "*", "kind": kind,
                    "optional": optional, "uses_default_features": true,
                    "features": [], "target": target, "registry": null,
                }));
                let link = resolved.entry(to).or_insert_with(|| {
                    json!({
                        "name": to, "pkg": to, "dep_kinds": [],
                    })
                });
                link["dep_kinds"]
                    .as_array_mut()
                    .context("expected dependency kinds")?
                    .push(json!({"kind": kind, "target": target}));
            }
            nodes.push(json!({
                "id": name, "dependencies": resolved.keys().collect::<Vec<_>>(),
                "deps": resolved.values().collect::<Vec<_>>(), "features": [],
            }));
            metadata.push(json!({
                "name": name, "version": "0.1.0", "id": name, "source": source,
                "dependencies": dependencies, "features": {},
                "targets": [{
                    "name": name, "kind": ["lib"], "crate_types": ["lib"],
                    "src_path": format!("/workspace/{name}/src/lib.rs"),
                    "edition": "2024", "doctest": false, "test": false, "doc": false,
                }],
                "manifest_path": format!("/workspace/{name}/Cargo.toml"),
                "repository": "https://github.com/zed-industries/fixture",
            }));
        }
        Ok(PackageGraph::from_json(
            json!({
                "packages": metadata,
                "workspace_members": packages.iter()
                    .filter(|package| matches!(package.origin, Workspace))
                    .map(|package| package.name).collect::<Vec<_>>(),
                "resolve": { "nodes": nodes, "root": null },
                "workspace_root": "/workspace", "target_directory": "/workspace/target",
                "version": 1,
            })
            .to_string(),
        )?)
    }
}
