use std::{cell::RefCell, rc::Rc, sync::Arc};

use fs::FakeFs;
use futures::FutureExt;
use gpui::{TestAppContext, UpdateGlobal as _};

use node_runtime::NodeRuntime;
use project::{
    Project,
    binary_downloads::{
        self, BinaryDownloads, BinaryDownloadsEvent, DownloadGate, PendingToolInstall, ToolInstall,
        ToolInstallOrigin,
    },
};
use serde_json::json;
use settings::{LocalSettingsKind, LocalSettingsPath, SettingsStore, WorktreeId};
use util::{path, rel_path::RelPath};

use crate::init_test;

#[gpui::test]
async fn test_install_waiters_deduplicate_and_resolve_on_consent(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(binary_downloads::init);
    disable_downloads(cx);
    let requests = collect_install_requests(cx);
    let resolved = collect_resolved_installs(cx);
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
    let waiters = store.update(cx, |store, cx| {
        let silent = store.wait_until_tool_allowed(None, "tool-a", cx).unwrap();
        assert_eq!(store.pending_tool_installs(), Vec::new());
        let first = store.request_tool_install(None, "tool-a", cx).unwrap();
        let second = store.request_tool_install(None, "tool-a", cx).unwrap();
        assert_eq!(
            store.pending_tool_installs(),
            vec![ToolInstall {
                worktree_id: None,
                tool: "tool-a".into()
            }]
        );
        [silent, first, second]
    });
    assert_eq!(*requests.borrow(), vec![(None, "tool-a".to_owned())]);
    assert!(waiters.iter().all(|waiter| !*waiter.borrow()));
    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "tool-a", cx);
        assert!(store.request_tool_install(None, "tool-a", cx).is_none());
    });
    for waiter in waiters {
        assert!(binary_downloads::await_downloads_allowed(Some(waiter), "tool-a").await);
    }
    let waiter = store.update(cx, |store, cx| {
        store.request_tool_install(None, "tool-b", cx)
    });
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.update_user_settings(cx, |settings| {
            settings.project.allow_binary_downloads = Some(true)
        });
    });
    assert!(binary_downloads::await_downloads_allowed(waiter, "tool-b").await);
    assert_eq!(
        *requests.borrow(),
        vec![(None, "tool-a".to_owned()), (None, "tool-b".to_owned())]
    );
    assert_eq!(*resolved.borrow(), *requests.borrow());
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        Vec::new()
    );
    disable_downloads(cx);
    assert!(cx.update(|cx| binary_downloads::tool_download_allowed(None, "tool-a", cx)));
    assert!(!cx.update(|cx| binary_downloads::tool_download_allowed(None, "tool-b", cx)));
}

#[gpui::test]
async fn test_worktree_removal_purges_waiters_and_cancels_receivers(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().forbid_parking();
    for remove_worktree in [true, false] {
        disable_downloads(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
            .await;
        let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
        let worktree = project.read_with(cx, |project, cx| project.worktrees(cx).next().unwrap());
        let worktree_id = worktree.read_with(cx, |worktree, _| worktree.id());
        let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
        let gate = cx.update(|cx| DownloadGate::new(Some(worktree_id), cx).unwrap());
        let receiver = store.update(cx, |store, cx| {
            store.approve_tool_install(Some(worktree_id), "approved-tool", cx);
            store
                .request_tool_install(Some(worktree_id), "tool-a", cx)
                .unwrap()
        });
        assert!(gate.is_allowed("approved-tool").await);
        if remove_worktree {
            project.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
        }
        cx.update(|_| drop(project));
        cx.run_until_parked();
        assert_eq!(
            store.read_with(cx, |store, _| store.pending_tool_installs()),
            Vec::new()
        );
        assert!(!binary_downloads::await_downloads_allowed(Some(receiver), "tool-a").await);
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
        assert!(!gate.is_allowed("tool-a").await);
        assert!(!gate.is_allowed("approved-tool").await);
    }
}

#[cfg(unix)]
#[gpui::test]
async fn test_tool_terminal_rechecks_consent_before_spawn(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let directory = tempfile::tempdir().unwrap();
    let marker = directory.path().join("spawned");
    let terminal = project.update(cx, |project, cx| {
        cx.set_global(terminal::HeadlessTerminal(true));
        project.create_terminal_task_with_permission(
            task::SpawnInTerminal {
                command: Some("/usr/bin/touch".to_owned()),
                args: vec![marker.to_string_lossy().into_owned()],
                ..task::SpawnInTerminal::default()
            },
            Some(ToolInstall {
                worktree_id: None,
                tool: "test-tool".into(),
            }),
            cx,
        )
    });
    disable_downloads(cx);
    assert!(
        terminal
            .await
            .unwrap_err()
            .is::<util::ToolPermissionDenied>()
    );
    assert!(!marker.exists());
}

#[gpui::test]
async fn test_remote_worktree_removal_cancels_local_requests_and_preserves_other_origins(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    cx.update(binary_downloads::init);
    disable_downloads(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/host-a"), json!({ "a.rs": "" }))
        .await;
    fs.insert_tree(path!("/host-b"), json!({ "b.rs": "" }))
        .await;
    let project_a = Project::test(fs.clone(), [path!("/host-a").as_ref()], cx).await;
    let project_b = Project::test(fs, [path!("/host-b").as_ref()], cx).await;
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
    let (origin_a, origin_b, worktree_id) = cx.update(|cx| {
        let worktree_store_a = project_a.read(cx).worktree_store();
        let worktree_store_b = project_b.read(cx).worktree_store();
        let worktree_id = worktree_store_a
            .read(cx)
            .worktrees()
            .next()
            .unwrap()
            .read(cx)
            .id();
        for project in [&project_a, &project_b] {
            let project = project.read(cx);
            binary_downloads::track_remote_binary_downloads(
                project.worktree_store(),
                (project.client().into(), client::ProjectId(1)),
                cx,
            );
        }
        (
            worktree_store_a.downgrade(),
            worktree_store_b.downgrade(),
            worktree_id,
        )
    });
    let install = ToolInstall {
        worktree_id: Some(worktree_id),
        tool: "shared-tool".into(),
    };
    let local_wait = store
        .update(cx, |store, cx| {
            store.set_remote_pending_installs(origin_a.clone(), vec![install.clone()], cx);
            store.set_remote_pending_installs(origin_b.clone(), vec![install.clone()], cx);
            store.request_tool_install(install.worktree_id, install.tool.clone(), cx)
        })
        .unwrap();
    project_a.update(cx, |project, cx| project.remove_worktree(worktree_id, cx));
    cx.run_until_parked();
    store.read_with(cx, |store, cx| {
        assert!(!*local_wait.borrow());
        assert_eq!(store.pending_tool_installs(), Vec::new());
        assert_eq!(
            store.pending_tool_installs_for_project(project_a.read(cx), cx),
            Vec::new()
        );
        assert_eq!(
            store.pending_tool_installs_for_project(project_b.read(cx), cx),
            vec![PendingToolInstall {
                install: install.clone(),
                origin: ToolInstallOrigin::Remote(origin_b),
            }]
        );
        assert!(
            store
                .approve_remote_tool_install(&origin_a, &install)
                .is_err()
        );
        assert!(!store.tool_download_allowed(install.worktree_id, install.tool.clone(), cx));
    });
    assert!(!binary_downloads::await_downloads_allowed(Some(local_wait), "shared-tool").await);
}

#[gpui::test]
async fn test_download_gate_queries_do_not_request_installs(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(binary_downloads::init);
    disable_downloads(cx);
    let requests = collect_install_requests(cx);
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
    let gate = cx.update(|cx| DownloadGate::new(None, cx).unwrap());
    let worktree_id = WorktreeId::from_proto(7);
    let scoped_gate = cx.update(|cx| DownloadGate::new(Some(worktree_id), cx).unwrap());

    assert!(!gate.is_allowed("tool-a").await);
    assert!(!scoped_gate.is_allowed("tool-a").await);
    store.update(cx, |store, cx| {
        store.approve_tool_install(Some(worktree_id), "tool-a", cx);
    });
    assert!(scoped_gate.is_allowed("tool-a").await);
    assert!(!gate.is_allowed("tool-a").await);
    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "tool-b", cx);
    });
    assert!(!gate.is_allowed("tool-a").await);
    assert!(gate.is_allowed("tool-b").await);
    assert!(!scoped_gate.is_allowed("tool-b").await);
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    assert!(gate.is_allowed("tool-a").await);
    let runtime = cx.update(|cx| {
        binary_downloads::node_runtime_with_permission(
            &NodeRuntime::unavailable(),
            |cx| binary_downloads::tool_download_allowed(None, "tool-a", cx),
            cx,
        )
    });
    let npm = runtime.run_npm_subcommand(None, "info", &[]);
    futures::pin_mut!(npm);
    assert!(futures::poll!(npm.as_mut()).is_pending());
    let pending = gate.is_allowed("tool-a");
    futures::pin_mut!(pending);
    assert!(futures::poll!(pending.as_mut()).is_pending());
    cx.run_until_parked();
    disable_downloads(cx);
    assert!(npm.await.unwrap_err().is::<util::ToolPermissionDenied>());
    assert!(!pending.await);
    assert!(!gate.is_allowed("tool-a").await);
    cx.run_until_parked();
    assert_eq!(*requests.borrow(), Vec::new());
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        Vec::new()
    );
}

#[gpui::test]
async fn test_scoped_node_gate_does_not_borrow_other_download_authority(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(binary_downloads::init);
    disable_downloads(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project-a"), json!({ "file": "" }))
        .await;
    fs.insert_tree(path!("/project-b"), json!({ "file": "" }))
        .await;
    let project = Project::test(
        fs,
        [path!("/project-a").as_ref(), path!("/project-b").as_ref()],
        cx,
    )
    .await;
    let worktrees = project.read_with(cx, |project, cx| {
        project
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).id())
            .collect::<Vec<_>>()
    });
    let [first, second] = worktrees.as_slice() else {
        panic!("expected two worktrees")
    };
    let (first, second) = (*first, *second);
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            for (worktree, allowed) in [(first, false), (second, true)] {
                store
                    .set_local_settings(
                        worktree,
                        LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                        LocalSettingsKind::Settings,
                        Some(&format!("{{\"allow_binary_downloads\":{allowed}}}")),
                        cx,
                    )
                    .unwrap();
            }
        })
    });

    let gate = cx.update(|cx| binary_downloads::npm_install_gate(cx).unwrap());
    let root = NodeRuntime::unavailable().with_install_gate(Some(gate.clone()));
    let runtime =
        cx.update(|cx| binary_downloads::scoped_node_runtime(&root, Some(first), "tool-a", cx));
    let store = cx.update(|cx| BinaryDownloads::try_get_global(cx).unwrap());
    let denied = util::downloads_disabled_error("dependency");
    let unavailable = "`node` settings do not allow any way to use Node.js";
    let directory = std::path::Path::new(path!("/unused"));

    assert!(!gate("dependency".to_string()).await);
    assert!(!cx.update(|cx| binary_downloads::node_downloads_allowed(cx)));
    assert_eq!(
        runtime
            .npm_install_packages(directory, &[("dependency", "1.0.0")])
            .await
            .unwrap_err()
            .to_string(),
        denied
    );
    for (worktree, tool) in [
        (Some(first), "tool-b"),
        (Some(second), "tool-a"),
        (None, "tool-a"),
    ] {
        store.update(cx, |store, cx| {
            store.approve_tool_install(worktree, tool, cx)
        });
        assert_eq!(
            runtime
                .npm_install_packages(directory, &[("dependency", "1.0.0")])
                .await
                .unwrap_err()
                .to_string(),
            denied
        );
        assert!(!gate("dependency".to_string()).await);
    }
    store.update(cx, |store, cx| {
        store.approve_tool_install(Some(first), "tool-a", cx)
    });
    assert_eq!(
        runtime
            .npm_install_packages(directory, &[("dependency", "1.0.0")])
            .await
            .unwrap_err()
            .to_string(),
        unavailable
    );
    assert_eq!(
        runtime
            .run_npm_subcommand(None, "exec", &[])
            .await
            .unwrap_err()
            .to_string(),
        unavailable
    );
    assert_eq!(
        runtime
            .npm_command(None, "run-script", &["compile"])
            .await
            .unwrap_err()
            .to_string(),
        unavailable
    );
    assert!(!gate("dependency".to_string()).await);
    assert_eq!(
        root.npm_install_packages(directory, &[("dependency", "1.0.0")])
            .await
            .unwrap_err()
            .to_string(),
        denied
    );
    assert!(!cx.update(|cx| binary_downloads::node_downloads_allowed(cx)));

    store.update(cx, |store, cx| {
        store.approve_tool_install(None, "dependency", cx)
    });
    assert!(gate("dependency".to_string()).await);
    assert!(!gate("other-dependency".to_string()).await);
    assert!(!cx.update(|cx| binary_downloads::node_downloads_allowed(cx)));
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true)
            })
        })
    });
    assert!(cx.update(|cx| binary_downloads::node_downloads_allowed(cx)));
    let unrelated =
        cx.update(|cx| binary_downloads::scoped_node_runtime(&root, Some(first), "unapproved", cx));
    assert_eq!(
        unrelated
            .npm_install_packages(directory, &[("dependency", "1.0.0")])
            .await
            .unwrap_err()
            .to_string(),
        denied
    );
    disable_downloads(cx);
    assert!(!gate("other-dependency".to_string()).await);
}

#[gpui::test]
async fn test_missing_download_store_fails_closed(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
    });
    assert!(!cx.update(|cx| binary_downloads::tool_download_allowed(None, "tool", cx)));
    assert!(!cx.update(|cx| binary_downloads::node_downloads_allowed(cx)));
    let wait = cx.update(|cx| binary_downloads::request_tool_install(None, "tool", cx));
    assert!(wait.is_some());
    assert!(!binary_downloads::await_downloads_allowed(wait, "tool").await);
    let root = NodeRuntime::unavailable()
        .with_install_gate(Some(Arc::new(|_| futures::future::ready(true).boxed())));
    let runtime = cx.update(|cx| binary_downloads::scoped_node_runtime(&root, None, "tool", cx));
    assert_eq!(
        runtime
            .run_npm_subcommand(None, "install", &[])
            .await
            .unwrap_err()
            .to_string(),
        util::downloads_disabled_error("npm")
    );
}

fn disable_downloads(cx: &mut TestAppContext) {
    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });
}

fn collect_install_requests(
    cx: &mut TestAppContext,
) -> Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> {
    let requests: Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> = Rc::default();
    cx.update({
        let requests = requests.clone();
        |cx| {
            let store = BinaryDownloads::try_get_global(cx).expect("global should be initialized");
            cx.subscribe(&store, move |_, event, _| {
                if let BinaryDownloadsEvent::InstallRequested(request) = event {
                    requests
                        .borrow_mut()
                        .push((request.worktree_id, request.tool.to_string()));
                }
            })
            .detach();
        }
    });
    requests
}

fn collect_resolved_installs(
    cx: &mut TestAppContext,
) -> Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> {
    let resolved: Rc<RefCell<Vec<(Option<WorktreeId>, String)>>> = Rc::default();
    cx.update({
        let resolved = resolved.clone();
        |cx| {
            let store = BinaryDownloads::try_get_global(cx).expect("global should be initialized");
            cx.subscribe(&store, move |_, event, _| {
                if let BinaryDownloadsEvent::InstallResolved(request) = event {
                    resolved
                        .borrow_mut()
                        .push((request.worktree_id, request.tool.to_string()));
                }
            })
            .detach();
        }
    });
    resolved
}
