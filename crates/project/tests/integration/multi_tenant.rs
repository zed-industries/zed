use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc};

use collections::BTreeMap;
use dap::client::SessionId;
use fs::FakeFs;
use gpui::{Entity, TestAppContext};
use language::Buffer;
use project::{
    Event, Project, ProjectPath, TaskSourceKind,
    bookmark_store::SerializedBookmark,
    debugger::{
        breakpoint_store::{
            Breakpoint, BreakpointEditAction, BreakpointState, BreakpointWithPosition,
            SourceBreakpoint,
        },
        dap_store::DapStoreEvent,
    },
    project_settings::SettingsObserverEvent,
};
use serde_json::json;
use settings::SettingsStore;
use task::{ResolvedTask, TaskTemplate};
use util::{path, rel_path::rel_path};

fn init_test(cx: &mut TestAppContext) {
    zlog::init_test();
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
}

/// Build two `Project`s sharing a single `Host` (via `HostRegistry`),
/// with disjoint worktree roots, on a small FakeFs.
async fn two_projects_with_disjoint_worktrees(
    cx: &mut TestAppContext,
) -> (Arc<FakeFs>, Entity<Project>, Entity<Project>) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": {
                "src": { "lib.rs": "fn alpha() {}\nfn helper() {}\nfn aux() {}\n" },
            },
            "beta": {
                "src": { "lib.rs": "fn beta() {}\nfn helper() {}\nfn aux() {}\n" },
            },
        }),
    )
    .await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs.clone(), [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    // Sanity-check the Phase 2 invariant: same FakeFs → same Host
    // (any host-shared store entity will do as a proxy).
    let store_a = project_a.read_with(cx, |p, cx| p.bookmark_store(cx));
    let store_b = project_b.read_with(cx, |p, cx| p.bookmark_store(cx));
    assert_eq!(
        store_a.entity_id(),
        store_b.entity_id(),
        "both projects must share a single Host for this audit"
    );

    (fs, project_a, project_b)
}

async fn open_buffer(
    project: &Entity<Project>,
    abs_path: &str,
    cx: &mut TestAppContext,
) -> Entity<Buffer> {
    project
        .update(cx, |project, cx| {
            project.open_local_buffer(Path::new(abs_path), cx)
        })
        .await
        .unwrap()
}

// ────────────────────────────────────────────────────────────────
// BookmarkStore
// ────────────────────────────────────────────────────────────────

/// Bookmarks set in `project_a` must not leak into `project_b`'s
/// serialized state (`workspace.serialize_workspace_internal` calls
/// `project.bookmark_store(cx).read(cx).all_serialized_bookmarks(cx)`).
#[gpui::test]
async fn test_bookmark_store_per_project_serialization(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    project_a.update(cx, |project, cx| {
        let bookmark_store = project.bookmark_store(cx);
        let snapshot = buffer_a.read(cx).snapshot();
        let anchor = snapshot.anchor_after(text::Point::new(1, 0));
        bookmark_store.update(cx, |store, cx| {
            store.toggle_bookmark(buffer_a.clone(), anchor, cx);
        });
    });

    let a_bookmarks = project_a.read_with(cx, |p, cx| p.serialized_bookmarks(cx));
    let b_bookmarks = project_b.read_with(cx, |p, cx| p.serialized_bookmarks(cx));

    assert!(
        a_bookmarks
            .keys()
            .any(|p| p.ends_with(path!("alpha/src/lib.rs"))),
        "project_a should have its own bookmark"
    );
    assert!(
        b_bookmarks.is_empty(),
        "project_b should have no bookmarks, but saw: {:?}",
        b_bookmarks.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_bookmark_store_load_preserves_other_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    // Project B sets a bookmark first.
    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;
    project_b.update(cx, |project, cx| {
        let bookmark_store = project.bookmark_store(cx);
        let snapshot = buffer_b.read(cx).snapshot();
        let anchor = snapshot.anchor_after(text::Point::new(1, 0));
        bookmark_store.update(cx, |store, cx| {
            store.toggle_bookmark(buffer_b.clone(), anchor, cx);
        });
    });

    // Project A loads its (different) serialized bookmarks.
    let mut serialized: BTreeMap<Arc<Path>, Vec<SerializedBookmark>> = BTreeMap::new();
    serialized.insert(
        Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
        vec![SerializedBookmark(0)],
    );
    project_a
        .update(cx, |project, cx| {
            project.restore_serialized_bookmarks(serialized, cx)
        })
        .await
        .unwrap();

    let b_bookmarks = project_b.read_with(cx, |p, cx| p.serialized_bookmarks(cx));
    assert!(
        b_bookmarks
            .keys()
            .any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's bookmark should survive project_a's load, but found: {:?}",
        b_bookmarks.keys().collect::<Vec<_>>(),
    );
}

// ────────────────────────────────────────────────────────────────
// BreakpointStore
// ────────────────────────────────────────────────────────────────
#[gpui::test]
async fn test_breakpoint_store_per_project_serialization(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    project_a.update(cx, |project, cx| {
        let breakpoint_store = project.breakpoint_store(cx);
        let snapshot = buffer_a.read(cx).snapshot();
        let position = snapshot.anchor_after(text::Point::new(1, 0));
        breakpoint_store.update(cx, |store, cx| {
            store.toggle_breakpoint(
                buffer_a.clone(),
                BreakpointWithPosition {
                    position,
                    bp: Breakpoint::new_standard(),
                },
                BreakpointEditAction::Toggle,
                cx,
            );
        });
    });

    let a_bps = project_a.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));

    assert!(
        a_bps.keys().any(|p| p.ends_with(path!("alpha/src/lib.rs"))),
        "project_a should have its own breakpoint"
    );
    assert!(
        b_bps.is_empty(),
        "project_b should have no breakpoints, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_breakpoint_store_clear_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_a = open_buffer(&project_a, path!("/repos/alpha/src/lib.rs"), cx).await;
    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;

    for (project, buffer) in [(&project_a, &buffer_a), (&project_b, &buffer_b)] {
        project.update(cx, |project, cx| {
            let breakpoint_store = project.breakpoint_store(cx);
            let snapshot = buffer.read(cx).snapshot();
            let position = snapshot.anchor_after(text::Point::new(0, 0));
            breakpoint_store.update(cx, |store, cx| {
                store.toggle_breakpoint(
                    buffer.clone(),
                    BreakpointWithPosition {
                        position,
                        bp: Breakpoint::new_standard(),
                    },
                    BreakpointEditAction::Toggle,
                    cx,
                );
            });
        });
    }

    project_a.update(cx, |project, cx| project.clear_breakpoints(cx));

    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    assert!(
        b_bps.keys().any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's breakpoints should survive A's clear, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

#[gpui::test]
async fn test_breakpoint_store_load_preserves_other_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let buffer_b = open_buffer(&project_b, path!("/repos/beta/src/lib.rs"), cx).await;
    project_b.update(cx, |project, cx| {
        let breakpoint_store = project.breakpoint_store(cx);
        let snapshot = buffer_b.read(cx).snapshot();
        let position = snapshot.anchor_after(text::Point::new(0, 0));
        breakpoint_store.update(cx, |store, cx| {
            store.toggle_breakpoint(
                buffer_b.clone(),
                BreakpointWithPosition {
                    position,
                    bp: Breakpoint::new_standard(),
                },
                BreakpointEditAction::Toggle,
                cx,
            );
        });
    });

    let mut serialized: BTreeMap<Arc<Path>, Vec<SourceBreakpoint>> = BTreeMap::new();
    serialized.insert(
        Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
        vec![SourceBreakpoint {
            row: 0,
            path: Arc::from(Path::new(path!("/repos/alpha/src/lib.rs"))),
            message: None,
            condition: None,
            hit_condition: None,
            state: BreakpointState::Enabled,
        }],
    );

    project_a
        .update(cx, |project, cx| {
            project.restore_serialized_breakpoints(serialized, cx)
        })
        .await
        .unwrap();

    let b_bps = project_b.read_with(cx, |p, cx| p.serialized_breakpoints(cx));
    assert!(
        b_bps.keys().any(|p| p.ends_with(path!("beta/src/lib.rs"))),
        "project_b's breakpoints should survive project_a's load, but saw: {:?}",
        b_bps.keys().collect::<Vec<_>>(),
    );
}

// ────────────────────────────────────────────────────────────────
// ImageStore
// ────────────────────────────────────────────────────────────────

/// `ImageStore::images()` returns every opened image across every
/// `Project`. UI surfaces should use the filtered `Project::images`
/// accessor instead so a shared host store doesn't leak sibling
/// Projects' images.
#[gpui::test]
async fn test_image_store_images_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/repos"),
        json!({
            "alpha": { "icon.png": "" },
            "beta": { "icon.png": "" },
        }),
    )
    .await;
    // 1x1 white PNG.
    let png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    fs.insert_file(path!("/repos/alpha/icon.png"), png.clone())
        .await;
    fs.insert_file(path!("/repos/beta/icon.png"), png).await;

    let project_a = Project::test(fs.clone(), [path!("/repos/alpha").as_ref()], cx).await;
    let project_b = Project::test(fs.clone(), [path!("/repos/beta").as_ref()], cx).await;
    cx.run_until_parked();

    let worktree_a = project_a.read_with(cx, |p, cx| p.worktrees(cx).next().unwrap().read(cx).id());
    let _image_a = project_a
        .update(cx, |project, cx| {
            project.open_image(
                ProjectPath {
                    worktree_id: worktree_a,
                    path: rel_path("icon.png").into(),
                },
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    // Project-filtered view: B should see no images, A should see one.
    let b_image_count = project_b.read_with(cx, |p, cx| p.images(cx).len());
    let a_image_count = project_a.read_with(cx, |p, cx| p.images(cx).len());
    assert_eq!(
        b_image_count, 0,
        "project_b should not see project_a's images via Project::images",
    );
    assert_eq!(
        a_image_count, 1,
        "project_a should see exactly its own opened image"
    );

    // The unfiltered host accessor still sees the union — documented as
    // a known leaky bystander; UI must go through `Project::images`.
    let host_image_count =
        project_b.read_with(cx, |p, cx| p.image_store(cx).read(cx).images().count());
    assert_eq!(
        host_image_count, 1,
        "shared host ImageStore is expected to hold the union of all Projects' images"
    );
}

// ─────────────────────────────────────────────────────────────
// SettingsObserver
// ─────────────────────────────────────────────────────────────

/// `SettingsObserverEvent::LocalTasksUpdated` fires on the shared host
/// observer for every worktree on the machine. The error case is
/// surfaced as a `Project::Event::Toast`; in Phase 2 only the Project
/// owning the worktree should toast, otherwise workspace A pops a
/// toast about workspace B's settings file.
#[gpui::test]
async fn test_settings_observer_toast_scoped_to_owning_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    // Emit a failure event for a path under project_a's worktree.
    let bad_path = std::path::PathBuf::from(path!("/repos/alpha/.zed/tasks.json"));
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
            settings::InvalidSettingsError::Tasks {
                path: bad_path.clone(),
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        a_events
            .borrow()
            .iter()
            .any(|id| id.contains("alpha/.zed/tasks.json")),
        "project_a should receive a toast for its own settings file, got: {:?}",
        a_events.borrow(),
    );
    assert!(
        b_events.borrow().is_empty(),
        "project_b should not receive a toast for project_a's settings file, got: {:?}",
        b_events.borrow(),
    );
}

/// Global settings paths (e.g. `~/.config/zed/settings.json`) live
/// outside every worktree and must surface toasts in *every*
/// workspace — the multi-tenant filter must not over-restrict.
#[gpui::test]
async fn test_settings_observer_global_toasts_in_every_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    // Path outside every worktree mimics ~/.config/zed/tasks.json.
    let global_path = std::path::PathBuf::from(path!("/global/tasks.json"));
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalTasksUpdated(Err(
            settings::InvalidSettingsError::Tasks {
                path: global_path,
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        !a_events.borrow().is_empty(),
        "project_a should receive the global toast"
    );
    assert!(
        !b_events.borrow().is_empty(),
        "project_b should also receive the global toast"
    );
}

/// `InvalidSettingsError::LocalSettings` (the parse-failure variant
/// for `.zed/settings.json`) carries a `WorktreeId`. The toast must
/// fire only in the Project owning that worktree.
#[gpui::test]
async fn test_settings_observer_local_settings_scoped_by_worktree_id(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_events = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_events = a_events.clone();
        let b_events = b_events.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                a_events.borrow_mut().push(notification_id.to_string());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast {
                notification_id, ..
            } = event
            {
                b_events.borrow_mut().push(notification_id.to_string());
            }
        });
        (sub_a, sub_b)
    });

    let alpha_worktree_id = project_a.read_with(cx, |p, cx| {
        p.visible_worktrees(cx).next().unwrap().read(cx).id()
    });
    let observer = project_a.read_with(cx, |p, cx| p.settings_observer(cx));
    observer.update(cx, |_, cx| {
        cx.emit(SettingsObserverEvent::LocalSettingsUpdated(Err(
            settings::InvalidSettingsError::LocalSettings {
                worktree_id: alpha_worktree_id,
                path: rel_path(".zed/settings.json").into(),
                message: "bad json".into(),
            },
        )));
    });
    cx.run_until_parked();

    assert!(
        !a_events.borrow().is_empty(),
        "project_a should receive a toast for its own worktree's local-settings file"
    );
    assert!(
        b_events.borrow().is_empty(),
        "project_b should not receive a toast for project_a's local-settings file, got: {:?}",
        b_events.borrow(),
    );
}

// ─────────────────────────────────────────────────────────────
// TaskStore (Inventory LRU)
// ─────────────────────────────────────────────────────────────

/// The recently-scheduled-task LRU was moved off the host-shared
/// `Inventory` and onto `Project` for Phase 2: a task scheduled in
/// workspace A must not show up as workspace B's "last scheduled task".
#[gpui::test]
async fn test_task_inventory_last_scheduled_per_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let resolved = make_resolved_task("build", "cargo build");
    let kind = TaskSourceKind::UserInput;

    project_a.update(cx, |project, _| {
        project.task_scheduled(kind.clone(), resolved.clone());
    });

    let a_last = project_a.read_with(cx, |project, _| project.last_scheduled_task(None));
    let b_last = project_b.read_with(cx, |project, _| project.last_scheduled_task(None));

    assert!(
        a_last.is_some_and(|(_, t)| t.original_task().label == "build"),
        "project_a should see its own scheduled task"
    );
    assert!(
        b_last.is_none(),
        "project_b should not see project_a's last scheduled task, but got: {:?}",
        b_last
    );
}

// ─────────────────────────────────────────────────────────────
// DapStore
// ─────────────────────────────────────────────────────────────

/// `DapStoreEvent::Notification` carries an optional `SessionId`. The
/// toast should fire only in the Project that owns that session;
/// host-wide notifications (no session) still show everywhere.
#[gpui::test]
async fn test_dap_store_notification_scoped_by_session_id(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_messages = Rc::new(RefCell::new(Vec::<String>::new()));
    let b_messages = Rc::new(RefCell::new(Vec::<String>::new()));
    let _subs = cx.update(|cx| {
        let a_messages = a_messages.clone();
        let b_messages = b_messages.clone();
        let sub_a = cx.subscribe(&project_a, move |_, event: &Event, _| {
            if let Event::Toast { message, .. } = event {
                a_messages.borrow_mut().push(message.clone());
            }
        });
        let sub_b = cx.subscribe(&project_b, move |_, event: &Event, _| {
            if let Event::Toast { message, .. } = event {
                b_messages.borrow_mut().push(message.clone());
            }
        });
        (sub_a, sub_b)
    });

    // project_a claims a session id; the shared host DapStore emits
    // a session-scoped notification.
    let owned_session = SessionId(42);
    project_a.update(cx, |project, _| project.claim_dap_session(owned_session));
    let dap_store = project_a.read_with(cx, |p, cx| p.dap_store(cx));
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::Notification {
            session_id: Some(owned_session),
            message: "breakpoint failure".into(),
        });
    });
    cx.run_until_parked();

    assert!(
        a_messages
            .borrow()
            .iter()
            .any(|m| m == "breakpoint failure"),
        "project_a should see its own session's toast"
    );
    assert!(
        b_messages.borrow().is_empty(),
        "project_b should not see project_a's session toast, got: {:?}",
        b_messages.borrow(),
    );

    // A host-wide notification (no session) reaches both Projects.
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::Notification {
            session_id: None,
            message: "host-wide".into(),
        });
    });
    cx.run_until_parked();
    assert!(
        a_messages.borrow().iter().any(|m| m == "host-wide"),
        "project_a should see host-wide toast"
    );
    assert!(
        b_messages.borrow().iter().any(|m| m == "host-wide"),
        "project_b should see host-wide toast"
    );
}

/// `Project::dap_sessions` and `Project::dap_session_by_id` are
/// filtered through the per-Project `dap_sessions` set so a sibling
/// Project's debug session doesn't leak into our UI surfaces.
#[gpui::test]
async fn test_dap_store_session_ownership_set(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    let (_fs, project_a, project_b) = two_projects_with_disjoint_worktrees(cx).await;

    let a_session = SessionId(7);
    let b_session = SessionId(8);
    project_a.update(cx, |project, _| project.claim_dap_session(a_session));
    project_b.update(cx, |project, _| project.claim_dap_session(b_session));

    project_a.read_with(cx, |project, _| {
        assert!(project.owns_dap_session(a_session));
        assert!(!project.owns_dap_session(b_session));
    });
    project_b.read_with(cx, |project, _| {
        assert!(project.owns_dap_session(b_session));
        assert!(!project.owns_dap_session(a_session));
    });

    // Pruning rides on `DapStoreEvent::DebugClientShutdown`. Faking
    // the event is enough to verify the set is updated; we don't need
    // a real session entity since `dap_session_by_id` returns `None`
    // either way.
    let dap_store = project_a.read_with(cx, |p, cx| p.dap_store(cx));
    dap_store.update(cx, |_, cx| {
        cx.emit(DapStoreEvent::DebugClientShutdown(a_session));
    });
    cx.run_until_parked();
    project_a.read_with(cx, |project, _| {
        assert!(
            !project.owns_dap_session(a_session),
            "shutdown event should prune the session from the per-project set"
        );
    });
}

fn make_resolved_task(label: &str, command: &str) -> ResolvedTask {
    let template = TaskTemplate {
        label: label.into(),
        command: command.into(),
        ..Default::default()
    };
    template
        .resolve_task("test", &task::TaskContext::default())
        .expect("task should resolve with empty context")
}
