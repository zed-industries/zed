use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use collections::HashMap;
use fs::FakeFs;
use gpui::{SharedString, TestAppContext};
use language::{
    FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageName, ManifestName,
    Toolchain, ToolchainList, ToolchainLister, ToolchainMetadata, ToolchainRootIndicator,
    ToolchainRootMarker, ToolchainScope,
};
use fs::Fs as _;
use lsp::Uri;
use project::{ManifestProvidersStore, Project, ProjectPath, Toolchains};
use serde_json::json;
use task::ShellKind;
use util::{path, rel_path::rel_path};

use super::init_test;

// ---------------------------------------------------------------------------
// Feature 1 – Root indicators
// ---------------------------------------------------------------------------

/// A toolchain lister that returns `.venv/` directories found in ancestors,
/// and declares `.venv/pyvenv.cfg` (directory + required child) as root indicator.
struct VenvMarkerLister(Arc<FakeFs>);

#[async_trait]
impl ToolchainLister for VenvMarkerLister {
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<util::rel_path::RelPath>,
        _: Option<HashMap<String, String>>,
    ) -> ToolchainList {
        let mut toolchains = vec![];
        for ancestor in subroot_relative_path.ancestors() {
            let venv_path = worktree_root.join(ancestor.as_std_path()).join(".venv");
            if self.0.is_dir(&venv_path).await {
                toolchains.push(Toolchain {
                    name: SharedString::from(format!("{}/venv", ancestor.as_unix_str())),
                    path: venv_path
                        .join("bin/python3")
                        .to_string_lossy()
                        .into_owned()
                        .into(),
                    language_name: LanguageName(SharedString::new_static("Python")),
                    as_json: serde_json::Value::Null,
                });
            }
        }
        ToolchainList {
            toolchains,
            ..Default::default()
        }
    }

    async fn resolve(
        &self,
        _: PathBuf,
        _: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Toolchain> {
        Err(anyhow::anyhow!("not implemented"))
    }

    fn meta(&self) -> ToolchainMetadata {
        ToolchainMetadata {
            term: SharedString::new_static("Virtual Environment"),
            new_toolchain_placeholder: SharedString::new_static("path to python3"),
            root_indicators: vec![ToolchainRootIndicator::Marker(
                ToolchainRootMarker::directory_with_required_child(
                    SharedString::new_static(".venv"),
                    SharedString::new_static("pyvenv.cfg"),
                ),
            )],
        }
    }

    fn activation_script(
        &self,
        _: &Toolchain,
        _: ShellKind,
        _: &gpui::App,
    ) -> futures::future::BoxFuture<'static, Vec<String>> {
        Box::pin(async { vec![] })
    }
}

fn python_lang_with_venv_marker(fs: Arc<FakeFs>) -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Python".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["py".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .with_toolchain_lister(Some(Arc::new(VenvMarkerLister(fs)))),
    )
}

/// A lister that uses BOTH the manifest indicator (pyproject.toml) AND the
/// marker indicator (.venv/pyvenv.cfg), in that order.
struct ManifestAndVenvMarkerLister(Arc<FakeFs>);

#[async_trait]
impl ToolchainLister for ManifestAndVenvMarkerLister {
    async fn list(
        &self,
        worktree_root: PathBuf,
        subroot_relative_path: Arc<util::rel_path::RelPath>,
        _: Option<HashMap<String, String>>,
    ) -> ToolchainList {
        let mut toolchains = vec![];
        for ancestor in subroot_relative_path.ancestors() {
            let venv_path = worktree_root.join(ancestor.as_std_path()).join(".venv");
            if self.0.is_dir(&venv_path).await {
                toolchains.push(Toolchain {
                    name: SharedString::from(format!("{}/venv", ancestor.as_unix_str())),
                    path: venv_path
                        .join("bin/python3")
                        .to_string_lossy()
                        .into_owned()
                        .into(),
                    language_name: LanguageName(SharedString::new_static("Python")),
                    as_json: serde_json::Value::Null,
                });
            }
        }
        ToolchainList {
            toolchains,
            ..Default::default()
        }
    }

    async fn resolve(
        &self,
        _: PathBuf,
        _: Option<HashMap<String, String>>,
    ) -> anyhow::Result<Toolchain> {
        Err(anyhow::anyhow!("not implemented"))
    }

    fn meta(&self) -> ToolchainMetadata {
        ToolchainMetadata {
            term: SharedString::new_static("Virtual Environment"),
            new_toolchain_placeholder: SharedString::new_static("path to python3"),
            root_indicators: vec![
                ToolchainRootIndicator::Manifest(ManifestName::from(
                    SharedString::new_static("pyproject.toml"),
                )),
                ToolchainRootIndicator::Marker(ToolchainRootMarker::directory_with_required_child(
                    SharedString::new_static(".venv"),
                    SharedString::new_static("pyvenv.cfg"),
                )),
            ],
        }
    }

    fn activation_script(
        &self,
        _: &Toolchain,
        _: ShellKind,
        _: &gpui::App,
    ) -> futures::future::BoxFuture<'static, Vec<String>> {
        Box::pin(async { vec![] })
    }
}

fn python_lang_with_manifest_and_venv_marker(fs: Arc<FakeFs>) -> Arc<Language> {
    Arc::new(
        Language::new(
            LanguageConfig {
                name: "Python".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["py".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .with_manifest(Some(ManifestName::from(SharedString::new_static(
            "pyproject.toml",
        ))))
        .with_toolchain_lister(Some(Arc::new(ManifestAndVenvMarkerLister(fs)))),
    )
}

#[gpui::test]
async fn test_venv_marker_used_as_root_indicator(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".zed": {
                "settings.json": r#"{"languages": {"Python": {"language_servers": ["ty"]}}}"#
            },
            "service": {
                ".venv": {
                    "pyvenv.cfg": "home = /usr/bin",
                    "bin": {}
                },
                "main.py": ""
            },
            "other.py": ""
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let _fake_server = language_registry.register_fake_lsp(
        "Python",
        FakeLspAdapter {
            name: "ty",
            ..Default::default()
        },
    );
    language_registry.add(python_lang_with_venv_marker(fs.clone()));

    let (service_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/service/main.py"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let servers = project.update(cx, |project, cx| {
        project.lsp_store().update(cx, |this, cx| {
            service_buffer.update(cx, |buffer, cx| {
                this.running_language_servers_for_local_buffer(buffer, cx)
                    .map(|(adapter, server)| (adapter.clone(), server.clone()))
                    .collect::<Vec<_>>()
            })
        })
    });
    assert_eq!(servers.len(), 1);
    let (_, server) = servers.into_iter().next().unwrap();
    // The LSP root must be `service/`, not the worktree root, because `.venv/pyvenv.cfg`
    // signals that `service/` is the subproject boundary.
    assert_eq!(
        server.workspace_folders(),
        std::collections::BTreeSet::from_iter(
            [Uri::from_file_path(path!("/root/service")).unwrap()].into_iter()
        )
    );
}

#[gpui::test]
async fn test_venv_marker_without_required_child_not_recognized(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    // `.venv/` exists but `pyvenv.cfg` is absent – not a valid root indicator.
    fs.insert_tree(
        path!("/root"),
        json!({
            ".zed": {
                "settings.json": r#"{"languages": {"Python": {"language_servers": ["ty"]}}}"#
            },
            "service": {
                ".venv": {
                    "bin": {}
                },
                "main.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let _fake_server = language_registry.register_fake_lsp(
        "Python",
        FakeLspAdapter {
            name: "ty",
            ..Default::default()
        },
    );
    language_registry.add(python_lang_with_venv_marker(fs.clone()));

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/service/main.py"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let servers = project.update(cx, |project, cx| {
        project.lsp_store().update(cx, |this, cx| {
            buffer.update(cx, |buffer, cx| {
                this.running_language_servers_for_local_buffer(buffer, cx)
                    .map(|(_, server)| server.clone())
                    .collect::<Vec<_>>()
            })
        })
    });
    assert_eq!(servers.len(), 1);
    // No valid root indicator → falls back to worktree root.
    assert_eq!(
        servers[0].workspace_folders(),
        std::collections::BTreeSet::from_iter(
            [Uri::from_file_path(path!("/root")).unwrap()].into_iter()
        )
    );
}

#[gpui::test]
async fn test_inner_venv_marker_overrides_outer_manifest(cx: &mut TestAppContext) {
    use language::{ManifestProvider, ManifestQuery};
    init_test(cx);

    // Register a minimal pyproject.toml provider so the manifest indicator is functional.
    struct PyprojectProvider;
    impl ManifestProvider for PyprojectProvider {
        fn name(&self) -> ManifestName {
            SharedString::new_static("pyproject.toml").into()
        }
        fn search(
            &self,
            ManifestQuery {
                path,
                depth,
                delegate,
            }: ManifestQuery,
        ) -> Option<Arc<util::rel_path::RelPath>> {
            for ancestor in path.ancestors().take(depth) {
                let pyproject = ancestor.join(rel_path("pyproject.toml"));
                if delegate.exists(&pyproject, Some(false)) {
                    return Some(Arc::from(ancestor));
                }
            }
            None
        }
    }
    cx.update(|cx| {
        ManifestProvidersStore::global(cx).register(Arc::new(PyprojectProvider))
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            ".zed": {
                "settings.json": r#"{"languages": {"Python": {"language_servers": ["ty"]}}}"#
            },
            "pyproject.toml": "",
            "service": {
                ".venv": {
                    "pyvenv.cfg": "home = /usr/bin",
                    "bin": {}
                },
                "main.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    let _fake_server = language_registry.register_fake_lsp(
        "Python",
        FakeLspAdapter {
            name: "ty",
            ..Default::default()
        },
    );
    // This language checks manifest first, then .venv marker.
    language_registry.add(python_lang_with_manifest_and_venv_marker(fs.clone()));

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/service/main.py"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let servers = project.update(cx, |project, cx| {
        project.lsp_store().update(cx, |this, cx| {
            buffer.update(cx, |buffer, cx| {
                this.running_language_servers_for_local_buffer(buffer, cx)
                    .map(|(_, server)| server.clone())
                    .collect::<Vec<_>>()
            })
        })
    });
    assert_eq!(servers.len(), 1);
    // `root_for_path_with_indicators` walks from the file outward; it finds `service/`
    // via the manifest indicator first (pyproject.toml is checked at `service/` which
    // doesn't exist, then the marker at `service/.venv/pyvenv.cfg` matches).
    // The outer `pyproject.toml` at root must not win over the inner `.venv/pyvenv.cfg`.
    assert_eq!(
        servers[0].workspace_folders(),
        std::collections::BTreeSet::from_iter(
            [Uri::from_file_path(path!("/root/service")).unwrap()].into_iter()
        )
    );
}

// ---------------------------------------------------------------------------
// Feature 2 – Subproject toolchain scope isolation
// ---------------------------------------------------------------------------

#[gpui::test]
async fn test_user_toolchain_scoped_to_subproject_not_visible_in_other_subproject(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "project-a": { ".venv": { "pyvenv.cfg": "", "bin": {} }, "main.py": "" },
            "project-b": { ".venv": { "pyvenv.cfg": "", "bin": {} }, "main.py": "" }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    // A toolchain lister is required for `available_toolchains` to resolve the language.
    project.read_with(cx, |project, _| project.languages().clone()).add(
        python_lang_with_venv_marker(fs.clone()),
    );
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let worktree_abs = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).abs_path()
    });

    let toolchain_a = Toolchain {
        name: "project-a venv".into(),
        path: format!("{}/project-a/.venv/bin/python3", worktree_abs.display()).into(),
        language_name: LanguageName::new_static("Python"),
        as_json: serde_json::Value::Null,
    };

    project.update(cx, |project, cx| {
        project.add_toolchain(
            toolchain_a.clone(),
            ToolchainScope::Subproject(worktree_abs.clone(), rel_path("project-a").into()),
            cx,
        )
    });

    // Listing toolchains for a file in project-b must not surface project-a's toolchain.
    let toolchains_for_b = project
        .update(cx, |project, cx| {
            project.available_toolchains(
                ProjectPath {
                    worktree_id,
                    path: rel_path("project-b/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;

    let Toolchains { user_toolchains, .. } = toolchains_for_b.expect("toolchains available");
    let subproject_toolchains: Vec<_> = user_toolchains
        .into_values()
        .flat_map(|set| set.into_iter())
        .collect();
    assert!(
        !subproject_toolchains.contains(&toolchain_a),
        "project-a toolchain must not appear when listing toolchains for project-b"
    );

    // Conversely, listing for project-a must include the toolchain.
    let toolchains_for_a = project
        .update(cx, |project, cx| {
            project.available_toolchains(
                ProjectPath {
                    worktree_id,
                    path: rel_path("project-a/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;

    let Toolchains {
        user_toolchains: user_toolchains_a,
        ..
    } = toolchains_for_a.expect("toolchains available");
    let subproject_toolchains_a: Vec<_> = user_toolchains_a
        .into_values()
        .flat_map(|set| set.into_iter())
        .collect();
    assert!(
        subproject_toolchains_a.contains(&toolchain_a),
        "project-a toolchain must appear when listing toolchains for project-a"
    );
}

// ---------------------------------------------------------------------------
// Feature 3 – Toolchain cleanup on directory deletion
// ---------------------------------------------------------------------------

#[gpui::test]
async fn test_active_toolchain_cleared_when_containing_dir_deleted(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "service": {
                ".venv": {
                    "pyvenv.cfg": "",
                    "bin": { "python3": "" }
                },
                "main.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let worktree_abs = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).abs_path()
    });

    let toolchain = Toolchain {
        name: "service venv".into(),
        path: format!("{}/service/.venv/bin/python3", worktree_abs.display()).into(),
        language_name: LanguageName::new_static("Python"),
        as_json: serde_json::Value::Null,
    };

    project
        .update(cx, |project, cx| {
            project.activate_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service").into(),
                },
                toolchain.clone(),
                cx,
            )
        })
        .await
        .expect("toolchain activated");

    // Verify the toolchain is active for a file inside the service subproject.
    let active_before = project
        .update(cx, |project, cx| {
            project.active_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;
    assert_eq!(
        active_before.as_ref().map(|t| t.path.as_ref()),
        Some(toolchain.path.as_ref()),
        "toolchain must be active before directory deletion"
    );

    // Delete the directory that contains the toolchain executable.
    let venv_entry_id = project.read_with(cx, |project, cx| {
        project
            .entry_for_path(
                &ProjectPath {
                    worktree_id,
                    path: rel_path("service/.venv").into(),
                },
                cx,
            )
            .map(|e| e.id)
    });
    let venv_entry_id = venv_entry_id.expect(".venv entry must exist in worktree snapshot");

    project
        .update(cx, |project, cx| {
            project.delete_entry(venv_entry_id, false, cx)
        })
        .expect("delete_entry returned a task")
        .await
        .expect("delete_entry succeeded");
    cx.run_until_parked();

    let active_after = project
        .update(cx, |project, cx| {
            project.active_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;
    assert!(
        active_after.is_none(),
        "toolchain must be cleared after its containing directory is deleted"
    );
}

#[gpui::test]
async fn test_user_toolchain_cleared_when_containing_dir_deleted(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "service": {
                ".venv": {
                    "pyvenv.cfg": "",
                    "bin": { "python3": "" }
                },
                "main.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let worktree_abs = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).abs_path()
    });

    let toolchain = Toolchain {
        name: "service venv".into(),
        path: format!("{}/service/.venv/bin/python3", worktree_abs.display()).into(),
        language_name: LanguageName::new_static("Python"),
        as_json: serde_json::Value::Null,
    };

    project.update(cx, |project, cx| {
        project.add_toolchain(
            toolchain.clone(),
            ToolchainScope::Subproject(worktree_abs.clone(), rel_path("service").into()),
            cx,
        )
    });

    let user_toolchains_before = project.read_with(cx, |project, cx| {
        project.user_toolchains(cx).unwrap_or_default()
    });
    let all_before: Vec<_> = user_toolchains_before
        .into_values()
        .flat_map(|s| s.into_iter())
        .collect();
    assert!(
        all_before.contains(&toolchain),
        "toolchain must be present in user_toolchains before deletion"
    );

    let venv_entry_id = project.read_with(cx, |project, cx| {
        project
            .entry_for_path(
                &ProjectPath {
                    worktree_id,
                    path: rel_path("service/.venv").into(),
                },
                cx,
            )
            .map(|e| e.id)
    });
    let venv_entry_id = venv_entry_id.expect(".venv entry must exist in worktree snapshot");

    project
        .update(cx, |project, cx| {
            project.delete_entry(venv_entry_id, false, cx)
        })
        .expect("delete_entry returned a task")
        .await
        .expect("delete_entry succeeded");
    cx.run_until_parked();

    let user_toolchains_after = project.read_with(cx, |project, cx| {
        project.user_toolchains(cx).unwrap_or_default()
    });
    let all_after: Vec<_> = user_toolchains_after
        .into_values()
        .flat_map(|s| s.into_iter())
        .collect();
    assert!(
        !all_after.contains(&toolchain),
        "toolchain must be removed from user_toolchains after its directory is deleted"
    );
}

#[gpui::test]
async fn test_parent_directory_deletion_clears_nested_toolchain(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    // Toolchain is two levels deep; deleting a grandparent should still clean it up.
    fs.insert_tree(
        path!("/root"),
        json!({
            "service": {
                ".venv": {
                    "pyvenv.cfg": "",
                    "bin": { "python3": "" }
                },
                "main.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let worktree_abs = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).abs_path()
    });

    let toolchain = Toolchain {
        name: "service venv".into(),
        path: format!("{}/service/.venv/bin/python3", worktree_abs.display()).into(),
        language_name: LanguageName::new_static("Python"),
        as_json: serde_json::Value::Null,
    };

    project
        .update(cx, |project, cx| {
            project.activate_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service").into(),
                },
                toolchain.clone(),
                cx,
            )
        })
        .await
        .expect("toolchain activated");

    // Delete the `service/` directory – the toolchain is two levels inside it.
    let service_entry_id = project.read_with(cx, |project, cx| {
        project
            .entry_for_path(
                &ProjectPath {
                    worktree_id,
                    path: rel_path("service").into(),
                },
                cx,
            )
            .map(|e| e.id)
    });
    let service_entry_id = service_entry_id.expect("service entry must exist");

    project
        .update(cx, |project, cx| {
            project.delete_entry(service_entry_id, false, cx)
        })
        .expect("delete_entry returned a task")
        .await
        .expect("delete_entry succeeded");
    cx.run_until_parked();

    let active_after = project
        .update(cx, |project, cx| {
            project.active_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;
    assert!(
        active_after.is_none(),
        "toolchain must be cleared when the grandparent directory containing it is deleted"
    );
}

#[gpui::test]
async fn test_file_deletion_does_not_clear_toolchain(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/root"),
        json!({
            "service": {
                ".venv": {
                    "pyvenv.cfg": "",
                    "bin": { "python3": "" }
                },
                "main.py": "",
                "util.py": ""
            }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let worktree_abs = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).abs_path()
    });

    let toolchain = Toolchain {
        name: "service venv".into(),
        path: format!("{}/service/.venv/bin/python3", worktree_abs.display()).into(),
        language_name: LanguageName::new_static("Python"),
        as_json: serde_json::Value::Null,
    };

    project
        .update(cx, |project, cx| {
            project.activate_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service").into(),
                },
                toolchain.clone(),
                cx,
            )
        })
        .await
        .expect("toolchain activated");

    // Delete a regular Python file – the toolchain must NOT be cleared.
    let util_entry_id = project.read_with(cx, |project, cx| {
        project
            .entry_for_path(
                &ProjectPath {
                    worktree_id,
                    path: rel_path("service/util.py").into(),
                },
                cx,
            )
            .map(|e| e.id)
    });
    let util_entry_id = util_entry_id.expect("util.py entry must exist");

    project
        .update(cx, |project, cx| {
            project.delete_entry(util_entry_id, false, cx)
        })
        .expect("delete_entry returned a task")
        .await
        .expect("delete_entry succeeded");
    cx.run_until_parked();

    let active_after = project
        .update(cx, |project, cx| {
            project.active_toolchain(
                ProjectPath {
                    worktree_id,
                    path: rel_path("service/main.py").into(),
                },
                LanguageName::new_static("Python"),
                cx,
            )
        })
        .await;
    assert_eq!(
        active_after.as_ref().map(|t| t.path.as_ref()),
        Some(toolchain.path.as_ref()),
        "toolchain must survive file deletion (only directory deletion triggers cleanup)"
    );
}
