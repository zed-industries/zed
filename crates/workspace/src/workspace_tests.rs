use crate::{
    CloseIntent, ItemHandle, ItemId, MultiWorkspace, OpenMode, SerializableItemRegistry,
    SerializedItemIds, Workspace, WorkspaceDb, WorkspaceId,
    item::test::TestItem,
    persistence::{
        SerializedAxis,
        model::{SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace},
    },
    register_serializable_item,
    tests::init_test,
};
use anyhow::{Result, anyhow};
use collections::{HashMap, HashSet};
use fs::FakeFs;
use futures::{
    Future, FutureExt as _,
    channel::{mpsc, oneshot},
    future::Shared,
};
use gpui::{
    App, AppContext, AsyncApp, Axis, Entity, EntityId, Global, Task, TestAppContext,
    VisualTestContext, WindowHandle, WindowId,
};
use project::{
    Project,
    bookmark_store::SerializedBookmark,
    debugger::breakpoint_store::{BreakpointState, SourceBreakpoint},
};
use remote::{
    CommandTemplate, Interactive, RemoteClientDelegate, RemoteConnection, RemoteConnectionOptions,
    RemotePlatform,
};
use serde_json::json;
use settings::{OnLastWindowClosed, SettingsStore};
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};
use util::{
    path,
    paths::{PathStyle, RemotePathBuf},
};

#[test]
fn test_serialized_item_ids_unsigned_boundaries() {
    let runtime_id = EntityId::from(0x1_0000_0001);
    for (maximum, expected) in [
        (i64::MAX as u64, 1_u64 << 63),
        (1_u64 << 63, (1_u64 << 63) + 1),
        (u64::MAX - 1, u64::MAX),
    ] {
        let mut namespace = SerializedItemIds {
            reserved: HashSet::from_iter([runtime_id.as_u64(), maximum]),
            ..SerializedItemIds::default()
        };
        assert_eq!(
            namespace.allocate(runtime_id).expect("allocate ID"),
            expected
        );
        assert_eq!(namespace.allocate(runtime_id).expect("stable ID"), expected);
    }
    let mut namespace = SerializedItemIds {
        reserved: HashSet::from_iter([runtime_id.as_u64(), u64::MAX]),
        ..SerializedItemIds::default()
    };
    assert_eq!(
        namespace
            .allocate(runtime_id)
            .expect_err("exhausted namespace")
            .to_string(),
        "serialized item ID namespace exhausted"
    );
    assert_eq!(namespace.by_runtime_id.len(), 0);
    let unreserved = EntityId::from(0x1_0000_0002);
    assert_eq!(
        namespace.allocate(unreserved).expect("unreserved ID"),
        unreserved.as_u64()
    );
}

#[test]
fn test_serialized_item_ids_reject_conflicting_registrations() {
    let first = EntityId::from(0x1_0000_0001);
    let second = EntityId::from(0x1_0000_0002);
    let mut namespace = SerializedItemIds {
        reserved: HashSet::from_iter([1, 2]),
        ..SerializedItemIds::default()
    };
    assert!(namespace.register(first, 3).is_err());
    assert_eq!(namespace.by_runtime_id.len(), 0);
    namespace.register(first, 1).expect("register saved ID");
    namespace
        .register(first, 1)
        .expect("idempotent registration");
    assert!(namespace.register(first, 2).is_err());
    assert!(namespace.register(second, 1).is_err());
    assert_eq!(namespace.by_runtime_id.len(), 1);
    assert_eq!(namespace.allocate(first).expect("stable ID"), 1);
    namespace
        .register(second, 2)
        .expect("register second saved ID");
    assert_eq!(namespace.allocate(second).expect("second stable ID"), 2);
}

#[gpui::test]
async fn test_serialized_item_id_query_keeps_unsigned_extremes(cx: &mut TestAppContext) {
    let (_, database, mut saved, cx) = restore_fixture(cx).await;
    let expected = vec![0, i64::MAX as u64, 1_u64 << 63, u64::MAX];
    let mut items = expected
        .iter()
        .map(|item_id| SerializedItem::new("TestItem", *item_id, false, false))
        .collect::<Vec<_>>();
    items.push(SerializedItem::new("OtherItem", 1, false, false));
    saved.center_group = SerializedPaneGroup::Pane(SerializedPane::new(items, true, 0));
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("save IDs");
    let mut item_ids = cx.update(|_, cx| {
        <TestItem as crate::SerializableItem>::serialized_item_ids(saved.id, cx)
            .expect("read graph IDs")
    });
    item_ids.sort_unstable();
    assert_eq!(item_ids, expected);
}

#[gpui::test]
async fn test_serialization_ids_reserve_all_provider_rows_lazily(cx: &mut TestAppContext) {
    let (workspace, _, _, cx) = restore_fixture(cx).await;
    let first = cx.new(TestItem::new);
    let second = cx.new(TestItem::new);
    let first_id = first.entity_id();
    let second_id = second.entity_id();
    let orphan_id = 1_u64 << 63;
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: vec![first_id.as_u64(), second_id.as_u64(), orphan_id],
            reads: Cell::new(0),
        });
        let registry = cx.global_mut::<SerializableItemRegistry>();
        let descriptor = registry
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        descriptor.serialized_item_ids = |_, cx| {
            let provider = cx.global::<ItemIdProvider>();
            provider.reads.set(provider.reads.get() + 1);
            Ok(provider.ids.clone())
        };
        let descriptor = *descriptor;
        registry
            .descriptors_by_kind
            .insert(Arc::from("OtherItem"), descriptor);
    });
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace.assigned_serialized_item_ids("TestItem"),
            Vec::<ItemId>::new()
        );
        assert_eq!(
            workspace.assigned_serialized_item_ids("Unknown"),
            Vec::<ItemId>::new()
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 0);
        assert_eq!(
            workspace
                .serialization_id("TestItem", first_id, cx)
                .expect("new ID"),
            orphan_id + 1
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 1);
        workspace
            .register_serialized_item_id("TestItem", second_id, second_id.as_u64(), cx)
            .expect("register unread saved ID");
        assert_eq!(
            workspace
                .serialization_id("TestItem", second_id, cx)
                .expect("saved ID"),
            second_id.as_u64()
        );
        assert_eq!(
            workspace
                .serialization_id("TestItem", first_id, cx)
                .expect("stable ID"),
            orphan_id + 1
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 1);
        assert_eq!(
            workspace
                .serialization_id("OtherItem", first_id, cx)
                .expect("separate kind"),
            orphan_id + 1
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 2);
    });
    let weak_first = first.downgrade();
    let weak_second = second.downgrade();
    drop(first);
    drop(second);
    assert!(weak_first.upgrade().is_none());
    assert!(weak_second.upgrade().is_none());
    workspace.read_with(cx, |workspace, _| {
        assert_eq!(
            workspace.serialized_item_ids["TestItem"]
                .by_runtime_id
                .len(),
            2
        );
        assert_eq!(workspace.serialized_item_ids["TestItem"].reserved.len(), 4);
        let mut assigned = workspace.assigned_serialized_item_ids("TestItem");
        assigned.sort_unstable();
        assert_eq!(assigned, vec![second_id.as_u64(), orphan_id + 1]);
        assert_eq!(
            workspace.assigned_serialized_item_ids("OtherItem"),
            vec![orphan_id + 1]
        );
    });
}

#[gpui::test]
async fn test_serialization_id_read_failure_blocks_payload_and_graph(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, _| Err(anyhow!("injected item ID read failure"));
    });
    let item = cx.new(|cx| TestItem::new(cx).with_serialize(|| panic!("payload must not start")));
    let task = workspace.update(cx, |workspace, cx| {
        item.to_serializable_item_handle(cx)
            .expect("serializable item")
            .serialize(workspace, false, cx)
            .expect("allocation error task")
    });
    assert_eq!(
        task.await.expect_err("read failure").to_string(),
        "injected item ID read failure"
    );
    let publication = workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
        workspace.serialize_workspace_internal(window, cx)
    });
    assert_eq!(
        publication
            .await
            .expect_err("publication must fail")
            .to_string(),
        "injected item ID read failure"
    );
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
    workspace.read_with(cx, |workspace, _| {
        assert_eq!(workspace.serialized_item_ids.len(), 0)
    });
}

#[gpui::test]
async fn test_cleanup_preserves_payloads_assigned_before_and_after_submission(
    cx: &mut TestAppContext,
) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id;
    database
        .write(move |connection| {
            connection.exec(
                "CREATE TABLE cleanup_test_payloads (workspace_id INTEGER, item_id INTEGER) STRICT",
            )?()?;
            let mut insert = connection.exec_bound::<(WorkspaceId, ItemId)>(
                "INSERT INTO cleanup_test_payloads VALUES (?, ?)",
            )?;
            for item_id in [1, 2, 3] {
                insert((workspace_id, item_id))?;
            }
            insert((WorkspaceId::from_i64(-1), 3))
        })
        .await
        .expect("seed payloads");
    let first = cx.new(TestItem::new);
    let second = cx.new(TestItem::new);
    let (first_id, second_id, first_write, cleanup, second_write) =
        workspace.update(cx, |workspace, cx| {
            let first_id = workspace
                .serialization_id("TestItem", first.entity_id(), cx)
                .expect("assign before cleanup");
            let first_write = database.write(move |connection| {
                connection.exec_bound::<(WorkspaceId, ItemId)>(
                    "INSERT INTO cleanup_test_payloads VALUES (?, ?)",
                )?((workspace_id, first_id))
            });
            let mut keep = database
                .serialized_item_ids(workspace_id, "TestItem")
                .expect("committed graph");
            keep.extend(workspace.assigned_serialized_item_ids("TestItem"));
            let cleanup = crate::delete_unloaded_items(
                keep,
                workspace_id,
                "cleanup_test_payloads",
                &database,
                cx,
            );
            let second_id = workspace
                .serialization_id("TestItem", second.entity_id(), cx)
                .expect("assign after cleanup");
            let second_write = database.write(move |connection| {
                connection.exec_bound::<(WorkspaceId, ItemId)>(
                    "INSERT INTO cleanup_test_payloads VALUES (?, ?)",
                )?((workspace_id, second_id))
            });
            (first_id, second_id, first_write, cleanup, second_write)
        });
    first_write.await.expect("write before cleanup");
    second_write.await.expect("write after cleanup");
    cleanup.await.expect("cleanup payloads");
    let remaining = database
        .select_bound::<WorkspaceId, ItemId>(
            "SELECT item_id FROM cleanup_test_payloads WHERE workspace_id = ? ORDER BY item_id",
        )
        .expect("prepare remaining payloads")(workspace_id)
    .expect("read remaining payloads");
    assert_eq!(remaining, vec![1, 2, first_id, second_id]);
    let other_workspace = database
        .select_bound::<WorkspaceId, ItemId>(
            "SELECT item_id FROM cleanup_test_payloads WHERE workspace_id = ?",
        )
        .expect("prepare other workspace")(WorkspaceId::from_i64(-1))
    .expect("read other workspace");
    assert_eq!(other_workspace, vec![3]);
}

#[gpui::test]
async fn test_serialization_ids_retain_unread_and_failed_graph_reservations(
    cx: &mut TestAppContext,
) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let first = cx.new(TestItem::new);
    let second = cx.new(TestItem::new);
    let first_id = first.entity_id();
    let second_id = second.entity_id();
    let maximum = first_id.as_u64().max(second_id.as_u64());
    saved.center_group = restored_pane_group(first_id.as_u64(), second_id.as_u64());
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("save unread IDs");
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .deserialize =
            |_, _, _, _, _, _| Task::ready(Err(anyhow!("injected item restore failure")));
    });
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace
                .serialization_id("TestItem", first_id, cx)
                .expect("reserve unread IDs"),
            maximum + 1
        );
    });
    let error = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .err()
        .expect("restore must fail");
    assert_eq!(
        error.root_cause().to_string(),
        "injected item restore failure"
    );
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace
                .serialization_id("TestItem", second_id, cx)
                .expect("retain failed reservation"),
            maximum + 2
        );
        assert_eq!(
            workspace
                .serialization_id("TestItem", first_id, cx)
                .expect("retain allocated ID"),
            maximum + 1
        );
    });
}

#[gpui::test]
async fn test_serialization_id_registration_failure_preserves_graph(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, _| Err(anyhow!("injected item ID read failure"));
    });
    let restoration = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    let error = restoration.await.err().expect("registration must fail");
    assert_eq!(
        error.root_cause().to_string(),
        "injected item ID read failure"
    );
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
    workspace.read_with(cx, |workspace, cx| {
        assert!(workspace.is_restoring());
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .sum::<usize>(),
            0
        );
    });
}

#[gpui::test]
async fn test_workspace_restore_mixed_item_failure_preserves_graph_and_payloads(
    cx: &mut TestAppContext,
) {
    assert_item_restore_failure_preserves_graph_and_payloads(false, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_all_item_failures_preserve_graph_and_payloads(
    cx: &mut TestAppContext,
) {
    assert_item_restore_failure_preserves_graph_and_payloads(true, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_suppresses_partial_serialization(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id;
    let (entered, release, serialized_items) = install_restore_gate(cx);

    let (restore, pre_poll_flush) = workspace.update_in(cx, |workspace, window, cx| {
        workspace.serialize_workspace(window, cx);
        assert!(workspace._schedule_serialize_workspace.is_some());
        let restore = workspace.load_workspace(saved.clone(), Vec::new(), window, cx);
        assert!(workspace.is_restoring());
        assert!(workspace._schedule_serialize_workspace.is_none());
        let flush = workspace.flush_serialization(window, cx);
        (restore, flush)
    });
    pre_poll_flush.await;
    entered.await.expect("second pane deserializer entered");
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        assert!(workspace.is_restoring());
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            vec![0, 1, 0]
        );
        assert_eq!(workspace.center.first_pane().read(cx).items_len(), 0);
    });
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.serialize_workspace(window, cx);
        assert!(workspace._schedule_serialize_workspace.is_none());
    });
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
    assert_eq!(*serialized_items.borrow(), Vec::<ItemId>::new());

    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.serialize_workspace_internal(window, cx)
        })
        .await
        .expect("serialization suppressed during restore");
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));

    release.send(()).expect("release second pane");
    assert_eq!(restore.await.expect("restored workspace").len(), 0);
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();

    assert_restored_graph(&database, &saved, cx);
    workspace.read_with(cx, |workspace, cx| {
        assert!(!workspace.is_restoring());
        assert_eq!(workspace.panes.len(), 2);
        assert_eq!(
            workspace
                .active_pane()
                .read(cx)
                .active_item()
                .map(|item| item.item_id().as_u64()),
            cx.global::<RestoreGate>()
                .restored_items
                .last()
                .map(|(_, id)| *id)
        );
    });
}

#[gpui::test]
async fn test_workspace_restore_survives_waiter_drop_before_poll(cx: &mut TestAppContext) {
    assert_restore_survives_waiter_drop(true, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_survives_waiter_drop_after_first_pane(cx: &mut TestAppContext) {
    assert_restore_survives_waiter_drop(false, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_survives_waiter_drop_during_metadata(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (entered, release, _) = install_restore_gate(cx);
    let waiter = Rc::new(RefCell::new(None));
    let (dropped_sender, dropped) = oneshot::channel();
    let bookmark_store = workspace.read_with(cx, |workspace, cx| {
        workspace.project().read(cx).bookmark_store()
    });
    let _subscription = cx.update(|_, cx| {
        let workspace = workspace.downgrade();
        let waiter = waiter.clone();
        let mut dropped_sender = Some(dropped_sender);
        cx.subscribe(&bookmark_store, move |_, _, cx| {
            if let Some(waiter) = waiter.borrow_mut().take() {
                workspace
                    .read_with(cx, |workspace, _| {
                        assert!(workspace.is_restoring());
                        assert_eq!(workspace.panes.len(), 2);
                    })
                    .expect("workspace during metadata restoration");
                drop(waiter);
                dropped_sender
                    .take()
                    .expect("drop sender")
                    .send(())
                    .expect("drop listener");
            }
        })
    });
    workspace.update_in(cx, |workspace, window, cx| {
        *waiter.borrow_mut() =
            Some(workspace.load_workspace(saved.clone(), Vec::new(), window, cx));
    });
    entered.await.expect("second pane deserializer entered");
    release.send(()).expect("release second pane");
    dropped
        .await
        .expect("waiter dropped during metadata restoration");
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, _| {
        assert!(!workspace.is_restoring());
        assert!(workspace._restore_workspace_task.is_none());
    });
    assert_restored_graph(&database, &saved, cx);
}

#[gpui::test]
async fn test_workspace_restore_does_not_keep_workspace_alive(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let (entered, _release, _) = install_restore_gate(cx);
    let restore = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved, Vec::new(), window, cx)
    });
    entered.await.expect("second pane deserializer entered");
    let weak_workspace = workspace.downgrade();
    cx.update(|window, _| window.remove_window());
    cx.cx.update(|_| drop(workspace));
    cx.run_until_parked();
    assert!(weak_workspace.upgrade().is_none());
    assert!(restore.await.is_err());
}

#[gpui::test]
async fn test_workspace_restore_flushes_during_cleanup(cx: &mut TestAppContext) {
    assert_restore_flushes_during_cleanup(false, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_flushes_on_shutdown_during_cleanup(cx: &mut TestAppContext) {
    assert_restore_flushes_during_cleanup(true, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_payload_failure_skips_cleanup(cx: &mut TestAppContext) {
    assert_restore_publication_failure_skips_cleanup(false, cx).await;
}

#[gpui::test]
async fn test_workspace_restore_graph_failure_skips_cleanup(cx: &mut TestAppContext) {
    assert_restore_publication_failure_skips_cleanup(true, cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_close_prompts(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::CloseWindow, cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_quit_prompts(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::Quit, cx).await;
}

#[gpui::test]
async fn test_open_workspace_by_id_reuses_owner_after_concurrent_restore(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    saved.id = database.next_id().await.expect("unowned workspace ID");
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed unowned workspace");
    let workspace_id = saved.id;
    let (first, second) = cx.update(|_, cx| {
        (
            crate::open_workspace_by_id(workspace_id, app_state.clone(), None, cx),
            crate::open_workspace_by_id(workspace_id, app_state.clone(), None, cx),
        )
    });
    let (first, second) = futures::join!(first, second);
    let first = first.expect("first restore");
    assert_eq!(first, second.expect("concurrent restore"));
    let restored = first
        .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
        .expect("restored workspace");
    restored.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.database_id(), Some(workspace_id));
        assert_eq!(workspace.items(cx).count(), 2);
    });
    let reused = cx
        .update(|_, cx| crate::open_workspace_by_id(workspace_id, app_state, None, cx))
        .await
        .expect("reuse live owner");
    assert_eq!(reused, first);
    assert_eq!(cx.update(|_, cx| cx.windows().len()), 2);
}

#[gpui::test]
async fn test_new_window_starts_fresh_when_saved_identity_is_owned(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("TestItem descriptor");
        descriptor.deserialize =
            |_, _, _, _, _, _| panic!("independent open must not read source items");
        descriptor.cleanup = |_, _, _, _| panic!("independent open must not clean source items");
    });
    let independent = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_workspace_for_paths(
                OpenMode::NewWindow,
                vec![PathBuf::from(path!("/project"))],
                window,
                cx,
            )
        })
        .await
        .expect("independent window");
    let independent_id = independent.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.items(cx).count(), 0);
        assert!(!workspace.is_restoring());
        workspace.database_id().expect("independent ID")
    });
    assert_ne!(independent_id, saved.id);
    independent
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(cx.update(|_, cx| cx.windows().len()), 2);
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
    let independent_graph = database
        .workspace_for_id(independent_id)
        .expect("independent graph");
    assert_eq!(independent_graph.paths, saved.paths);
    assert!(independent_graph.bookmarks.is_empty());
    assert!(independent_graph.breakpoints.is_empty());
    assert!(independent_graph.recent_navigation_history.is_empty());
}

#[gpui::test]
async fn test_new_window_propagates_allocation_failure(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    database
        .write(|connection| {
            connection.exec(
                "CREATE TRIGGER fail_fork_id BEFORE INSERT ON workspaces
             BEGIN SELECT RAISE(ABORT, 'injected fork allocation failure'); END;",
            )?()
        })
        .await
        .expect("inject allocation failure");
    let result = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_workspace_for_paths(
                OpenMode::NewWindow,
                vec![PathBuf::from(path!("/project"))],
                window,
                cx,
            )
        })
        .await;
    assert_eq!(
        result
            .err()
            .expect("allocation must fail")
            .root_cause()
            .to_string(),
        "Sqlite call failed with code 1811 and message: Some(\"injected fork allocation failure\")",
    );
    assert_eq!(cx.update(|_, cx| cx.windows().len()), 1);
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
}

#[gpui::test]
async fn test_concurrent_local_opens_recheck_ownership_before_attachment(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    saved.id = database.next_id().await.expect("unowned workspace ID");
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed unowned workspace");
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let (first, second) = cx.update(|_, cx| {
        let open = |cx: &mut gpui::App| {
            Workspace::new_local(
                vec![PathBuf::from(path!("/project"))],
                app_state.clone(),
                None,
                None,
                None,
                OpenMode::Activate,
                cx,
            )
        };
        (open(cx), open(cx))
    });
    let (first, second) = futures::join!(first, second);
    let first = first.expect("first open");
    let second = second.expect("concurrent open");
    let ids = [&first.workspace, &second.workspace].map(|workspace| {
        workspace.read_with(cx, |workspace, cx| {
            let workspace_id = workspace.database_id().expect("durable ID");
            assert_eq!(
                workspace.items(cx).count(),
                if workspace_id == saved.id { 2 } else { 0 }
            );
            workspace_id
        })
    });
    assert_ne!(ids[0], ids[1]);
    assert!(ids[0] == saved.id || ids[1] == saved.id);
    assert_eq!(cx.update(|_, cx| cx.windows().len()), 3);
}

#[gpui::test]
async fn test_unowned_new_window_restores_saved_identity(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    saved.id = database.next_id().await.expect("unowned ID");
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed unowned workspace");
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let opened = cx
        .update(|_, cx| {
            Workspace::new_local(
                vec![PathBuf::from(path!("/project"))],
                app_state,
                None,
                None,
                None,
                OpenMode::NewWindow,
                cx,
            )
        })
        .await
        .expect("restore unowned workspace");
    opened.workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.database_id(), Some(saved.id));
        assert_eq!(workspace.items(cx).count(), 2);
    });
}

#[gpui::test]
async fn test_local_open_preserves_saved_and_independent_root_order(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/roots"), json!({"A": {}, "B": {}}))
        .await;
    let requested = vec![
        PathBuf::from(path!("/roots/A")),
        PathBuf::from(path!("/roots/B")),
    ];
    let saved_order = vec![
        PathBuf::from(path!("/roots/B")),
        PathBuf::from(path!("/roots/A")),
    ];
    saved.id = database.next_id().await.expect("unowned ID");
    saved.paths = crate::PathList::new(&saved_order);
    saved.bookmarks.clear();
    saved.breakpoints.clear();
    saved.recent_navigation_history.clear();
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed reversed roots");
    let mut opened_workspaces = Vec::new();
    for expected in [&saved_order, &requested] {
        let opened = cx
            .update(|_, cx| {
                Workspace::new_local(
                    requested.clone(),
                    app_state.clone(),
                    None,
                    None,
                    None,
                    OpenMode::NewWindow,
                    cx,
                )
            })
            .await
            .expect("open ordered roots");
        let workspace_id = opened.workspace.read_with(cx, |workspace, cx| {
            assert_eq!(
                workspace
                    .root_paths(cx)
                    .iter()
                    .map(|path| path.to_path_buf())
                    .collect::<Vec<_>>(),
                *expected
            );
            assert_eq!(workspace.project().read(cx).worktrees(cx).count(), 2);
            let workspace_id = workspace.database_id().expect("opened ID");
            assert_eq!(
                workspace.items(cx).count(),
                if workspace_id == saved.id { 2 } else { 0 }
            );
            workspace_id
        });
        if opened_workspaces.is_empty() {
            assert_eq!(workspace_id, saved.id);
        } else {
            assert_ne!(workspace_id, saved.id);
        }
        opened
            .window
            .update(cx, |_, window, cx| {
                opened.workspace.update(cx, |workspace, cx| {
                    workspace.flush_serialization(window, cx)
                })
            })
            .expect("flush ordered roots")
            .await;
        assert_eq!(
            database
                .workspace_for_id(workspace_id)
                .expect("persisted roots")
                .paths
                .ordered_paths()
                .cloned()
                .collect::<Vec<_>>(),
            *expected
        );
        opened_workspaces.push((opened, workspace_id, expected.clone()));
    }
    for (opened, workspace_id, expected) in opened_workspaces {
        opened
            .window
            .update(cx, |_, window, _| window.remove_window())
            .expect("close ordered roots");
        drop(opened);
        cx.run_until_parked();
        let reopened = cx
            .update(|_, cx| crate::open_workspace_by_id(workspace_id, app_state.clone(), None, cx))
            .await
            .expect("reopen ordered roots");
        reopened
            .read_with(cx, |multi_workspace, cx| {
                let workspace = multi_workspace.workspace().read(cx);
                assert_eq!(workspace.database_id(), Some(workspace_id));
                assert_eq!(
                    workspace
                        .root_paths(cx)
                        .iter()
                        .map(|path| path.to_path_buf())
                        .collect::<Vec<_>>(),
                    expected
                );
                assert_eq!(workspace.project().read(cx).worktrees(cx).count(), 2);
            })
            .expect("reopened roots");
        assert_eq!(
            database
                .workspace_for_id(workspace_id)
                .expect("reopened persisted roots")
                .paths
                .ordered_paths()
                .cloned()
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[gpui::test]
async fn test_pending_workspace_owner_is_joined_by_id(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let requested = vec![
        PathBuf::from(path!("/roots/A")),
        PathBuf::from(path!("/roots/B")),
    ];
    saved.paths = crate::PathList::new(&[
        PathBuf::from(path!("/roots/B")),
        PathBuf::from(path!("/roots/A")),
    ]);
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/roots"), json!({"A": {}, "B": {}}))
        .await;
    saved.id = database.next_id().await.expect("unowned ID");
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed workspace");
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let mut async_cx = cx.cx.to_async();
    let crate::WorkspaceOpen::Claimed(claim) =
        crate::claim_workspace_open(saved.id, false, &mut async_cx)
            .await
            .expect("claim")
    else {
        panic!("unowned workspace must be claimed")
    };
    let opening =
        cx.update(|_, cx| crate::open_workspace_by_id(saved.id, app_state.clone(), None, cx));
    cx.run_until_parked();
    assert!(!opening.is_ready());
    let independent = cx
        .update(|_, cx| {
            Workspace::new_local(
                requested.clone(),
                app_state,
                None,
                None,
                None,
                OpenMode::NewWindow,
                cx,
            )
        })
        .await
        .expect("independent contender");
    let independent_id = independent.workspace.read_with(cx, |workspace, cx| {
        assert_ne!(workspace.database_id(), Some(saved.id));
        assert_eq!(workspace.items(cx).count(), 0);
        assert_eq!(
            workspace
                .root_paths(cx)
                .iter()
                .map(|path| path.to_path_buf())
                .collect::<Vec<_>>(),
            requested
        );
        assert_eq!(workspace.project().read(cx).worktrees(cx).count(), 2);
        workspace.database_id().expect("independent ID")
    });
    independent
        .window
        .update(cx, |_, window, cx| {
            independent.workspace.update(cx, |workspace, cx| {
                workspace.flush_serialization(window, cx)
            })
        })
        .expect("flush pending contender")
        .await;
    assert_eq!(
        database
            .workspace_for_id(independent_id)
            .expect("independent persisted roots")
            .paths
            .ordered_paths()
            .cloned()
            .collect::<Vec<_>>(),
        requested
    );
    workspace.update(cx, |workspace, _| workspace.set_database_id(saved.id));
    let owner_window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("owner window")
    });
    claim.complete();
    assert_eq!(opening.await.expect("joined owner"), owner_window);
    assert_eq!(cx.update(|_, cx| cx.windows().len()), 2);
    assert!(cx.update(|_, cx| {
        cx.global::<crate::WorkspaceOpenClaims>()
            .0
            .borrow()
            .is_empty()
    }));
}

#[gpui::test]
async fn test_cancelled_workspace_claim_is_released(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    saved.id = database.next_id().await.expect("unowned ID");
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed workspace");
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let mut async_cx = cx.cx.to_async();
    let crate::WorkspaceOpen::Claimed(claim) =
        crate::claim_workspace_open(saved.id, false, &mut async_cx)
            .await
            .expect("claim")
    else {
        panic!("unowned workspace must be claimed")
    };
    let opening =
        cx.update(|_, cx| crate::open_workspace_by_id(saved.id, app_state.clone(), None, cx));
    cx.run_until_parked();
    assert!(!opening.is_ready());
    drop(claim);
    assert_eq!(
        opening
            .await
            .err()
            .expect("pending owner cancelled")
            .to_string(),
        "pending workspace open failed or was cancelled"
    );
    let window = cx
        .update(|_, cx| crate::open_workspace_by_id(saved.id, app_state, None, cx))
        .await
        .expect("retry after cancellation");
    window
        .read_with(cx, |multi_workspace, cx| {
            assert_eq!(
                multi_workspace.workspace().read(cx).database_id(),
                Some(saved.id)
            );
        })
        .expect("restored owner");
}

#[gpui::test]
async fn test_remote_pending_opens_claim_distinct_server_identities(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| release_channel::init("0.0.0".parse().expect("test version"), cx));
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let options = remote::RemoteConnectionOptions::Ssh(remote::SshConnectionOptions {
        host: "pending.test".into(),
        ..remote::SshConnectionOptions::default()
    });
    saved.id = database.next_id().await.expect("remote ID");
    saved.window_bounds = None;
    saved.display = None;
    saved.location = crate::SerializedWorkspaceLocation::Remote(options.clone());
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed remote workspace");
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
    let connection = Arc::new(PendingRemoteConnection {
        options,
        identifiers: std::sync::Mutex::new(Vec::new()),
    });
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    let (cancel_first, first_cancelled) = oneshot::channel();
    let (cancel_second, second_cancelled) = oneshot::channel();
    let (_cancel_join, join_cancelled) = oneshot::channel();
    let first = cx.update(|_, cx| {
        crate::open_remote_project_with_new_connection(
            window,
            connection.clone(),
            first_cancelled,
            Arc::new(remote::MockDelegate),
            app_state.clone(),
            vec![PathBuf::from(path!("/project"))],
            None,
            cx,
        )
    });
    cx.run_until_parked();
    assert_eq!(connection.identifiers.lock().expect("identifiers").len(), 1);
    let second = cx.update(|_, cx| {
        crate::open_remote_project_with_new_connection(
            window,
            connection.clone(),
            second_cancelled,
            Arc::new(remote::MockDelegate),
            app_state.clone(),
            vec![PathBuf::from(path!("/project"))],
            None,
            cx,
        )
    });
    let joined = cx.update(|_, cx| {
        crate::open_remote_project_with_new_connection(
            window,
            connection.clone(),
            join_cancelled,
            Arc::new(remote::MockDelegate),
            app_state.clone(),
            vec![PathBuf::from(path!("/project"))],
            Some(saved.id),
            cx,
        )
    });
    cx.run_until_parked();
    assert!(!first.is_ready());
    assert!(!second.is_ready());
    assert!(!joined.is_ready());
    let identities = connection.identifiers.lock().expect("identifiers").clone();
    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0], identities[1]);
    assert_eq!(
        identities[0]
            .rsplit_once('-')
            .expect("server ID")
            .1
            .parse::<i64>()
            .expect("numeric ID"),
        i64::from(saved.id)
    );
    cancel_first.send(()).expect("cancel first RPC");
    assert!(first.await.expect("cancelled RPC").is_none());
    assert_eq!(
        joined.await.err().expect("owner cancelled").to_string(),
        "pending workspace open failed or was cancelled"
    );
    drop(second);
    drop(cancel_second);
    cx.run_until_parked();
    assert!(cx.update(|_, cx| {
        cx.global::<crate::WorkspaceOpenClaims>()
            .0
            .borrow()
            .is_empty()
    }));
    let (cancel_retry, retry_cancelled) = oneshot::channel();
    let retry = cx.update(|_, cx| {
        crate::open_remote_project_with_new_connection(
            window,
            connection.clone(),
            retry_cancelled,
            Arc::new(remote::MockDelegate),
            app_state,
            vec![PathBuf::from(path!("/project"))],
            Some(saved.id),
            cx,
        )
    });
    cx.run_until_parked();
    assert_eq!(
        connection
            .identifiers
            .lock()
            .expect("identifiers")
            .as_slice(),
        &[
            identities[0].clone(),
            identities[1].clone(),
            identities[0].clone()
        ]
    );
    cancel_retry.send(()).expect("cancel retry");
    assert!(retry.await.expect("cancelled retry").is_none());
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
}

#[gpui::test]
async fn test_window_close_cancellation_preserves_all_bindings(cx: &mut TestAppContext) {
    let (fixture, cx) = close_fixture(true, true, cx).await;
    let window = fixture.window;
    let closing = cx.cx.spawn(async move |mut cx| {
        crate::prepare_window_to_close(window, CloseIntent::CloseWindow, &mut cx).await
    });
    cx.run_until_parked();
    assert!(cx.has_pending_prompt());
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    cx.simulate_prompt_answer("Don't Save");
    cx.run_until_parked();
    assert!(cx.has_pending_prompt());
    fixture
        .window
        .read_with(cx, |multi_workspace, _| {
            assert_eq!(
                multi_workspace.workspace(),
                fixture.workspaces.last().expect("second workspace")
            );
        })
        .expect("window while second prompt is pending");
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    cx.simulate_prompt_answer("Cancel");
    assert!(!closing.await.expect("cancelled close"));
    cx.run_until_parked();
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    assert_live_bindings(&fixture, cx);
}

#[gpui::test]
async fn test_window_close_clears_exact_group_after_pending_graph(cx: &mut TestAppContext) {
    let (fixture, cx) = close_fixture(true, true, cx).await;
    let window = fixture.window;
    let closing = cx.cx.spawn(async move |mut cx| {
        crate::prepare_window_to_close(window, CloseIntent::CloseWindow, &mut cx).await
    });
    cx.run_until_parked();
    cx.simulate_prompt_answer("Don't Save");
    cx.run_until_parked();
    assert!(cx.has_pending_prompt());
    let workspace = fixture.workspaces.first().expect("first workspace");
    let (workspace_id, session_id, window_id) = workspace.read_with(cx, |workspace, _| {
        (
            workspace.database_id().expect("workspace ID"),
            workspace.session_id.clone(),
            workspace.serialized_window_id,
        )
    });
    let mut pending_graph = fixture
        .database
        .workspace_for_id(workspace_id)
        .expect("saved graph");
    pending_graph.session_id = session_id;
    pending_graph.window_id = window_id.map(|id| id.as_u64());
    let (release, receiver) = oneshot::channel();
    let database = fixture.database.clone();
    let pending = cx
        .executor()
        .spawn(async move {
            receiver.await.map_err(|error| Arc::new(anyhow!(error)))?;
            database
                .try_save_workspace(pending_graph)
                .await
                .map_err(Arc::new)
        })
        .shared();
    workspace.update(cx, |workspace, _| {
        workspace.pending_workspace_serialization = Some(pending)
    });
    cx.simulate_prompt_answer("Don't Save");
    cx.run_until_parked();
    assert!(!closing.is_ready());
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    release.send(()).expect("release old graph write");
    assert!(closing.await.expect("accepted close"));
    let mut expected = fixture.bindings.clone();
    for (_, session_id, window_id) in expected.iter_mut().take(3) {
        *session_id = None;
        *window_id = None;
    }
    assert_eq!(session_bindings(&fixture.database), expected);
    for workspace in &fixture.workspaces {
        workspace.update_in(cx, |workspace, window, cx| {
            assert_eq!(workspace.session_id, None);
            assert_eq!(workspace.serialized_window_id, None);
            workspace.serialize_workspace(window, cx);
        });
    }
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert_eq!(session_bindings(&fixture.database), expected);
}

#[gpui::test]
async fn test_window_close_database_failure_restores_membership(cx: &mut TestAppContext) {
    let (fixture, cx) = close_fixture(true, false, cx).await;
    fixture
        .database
        .write(|connection| {
            connection.exec(
                "CREATE TRIGGER fail_window_session_clear
            BEFORE UPDATE OF session_id ON workspaces
            WHEN OLD.session_id IS NOT NULL AND NEW.session_id IS NULL
            BEGIN SELECT RAISE(ABORT, 'injected session clear failure'); END;",
            )?()
        })
        .await
        .expect("install session clear failure");
    let window = fixture.window;
    let closing = cx.cx.spawn(async move |mut cx| {
        crate::prepare_window_to_close(window, CloseIntent::CloseWindow, &mut cx).await
    });
    let error = closing.await.expect_err("session clear must fail");
    assert_eq!(
        error.root_cause().to_string(),
        "Sqlite call failed with code 1811 and message: Some(\"injected session clear failure\")"
    );
    cx.run_until_parked();
    assert!(fixture.window.read_with(cx, |_, _| ()).is_ok());
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    assert_live_bindings(&fixture, cx);
}

#[gpui::test]
async fn test_window_close_quit_preserves_group(cx: &mut TestAppContext) {
    assert_window_close_preserves_group(CloseIntent::Quit, true, cx).await;
}

#[gpui::test]
async fn test_window_close_last_window_quits_preserves_group(cx: &mut TestAppContext) {
    assert_window_close_preserves_group(CloseIntent::CloseWindow, false, cx).await;
}

#[gpui::test]
async fn test_window_close_preserves_hot_exit_when_another_window_opens(cx: &mut TestAppContext) {
    let (fixture, cx) = close_fixture(false, false, cx).await;
    let workspace = fixture.workspaces.first().expect("first workspace");
    let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
    let (release, receiver) = oneshot::channel();
    let payload = cx
        .executor()
        .spawn(async move { receiver.await.map_err(|error| Arc::new(anyhow!(error))) })
        .shared();
    let executor = cx.executor();
    let item = cx.new(|cx| {
        TestItem::new(cx).with_dirty(true).with_serialize(move || {
            let payload = payload.clone();
            Some(executor.spawn(async move { payload.await.map_err(|error| anyhow!(error)) }))
        })
    });
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
    });
    let window = fixture.window;
    let closing = cx.cx.spawn(async move |mut cx| {
        crate::prepare_window_to_close(window, CloseIntent::CloseWindow, &mut cx).await
    });
    cx.run_until_parked();
    assert!(!closing.is_ready());
    assert!(!cx.has_pending_prompt());
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    cx.cx.add_window(|window, cx| {
        let workspace = cx.new(|cx| Workspace::new(None, project, app_state, window, cx));
        MultiWorkspace::new(workspace, window, cx)
    });
    release.send(()).expect("release hot-exit payload");
    assert!(closing.await.expect("accepted close"));
    assert!(!cx.has_pending_prompt());
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    for workspace in &fixture.workspaces {
        workspace.read_with(cx, |workspace, _| {
            assert!(workspace.session_id.is_some());
            assert!(workspace.serialized_window_id.is_some());
        });
    }
}

#[gpui::test]
async fn test_remote_setup_failure_protects_saved_graph_before_restore(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let (project, app_state) = workspace.read_with(cx, |workspace, _| {
        (workspace.project().clone(), workspace.app_state().clone())
    });
    saved.id = database.next_id().await.expect("remote restore ID");
    saved.window_bounds = None;
    saved.display = None;
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed remote restore");
    database
        .write(|connection| {
            connection.exec("ALTER TABLE toolchains RENAME TO unavailable_toolchains")?()
        })
        .await
        .expect("inject toolchain setup failure");
    let guarded_before_attachment = Rc::new(Cell::new(false));
    let _subscription = cx.update(|_, cx| {
        let guarded_before_attachment = guarded_before_attachment.clone();
        cx.observe_new::<Workspace>(move |workspace, _, _| {
            guarded_before_attachment.set(workspace.restoring_workspace);
        })
    });
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    let workspace_id = saved.id;
    let serialized_workspace = saved.clone();
    let opening = cx.cx.spawn(async move |mut cx| {
        crate::open_remote_project_inner(
            project,
            vec![PathBuf::from(path!("/project"))],
            workspace_id,
            Some(serialized_workspace),
            app_state,
            window,
            None,
            None,
            &mut cx,
        )
        .await
    });
    assert_eq!(
        opening
            .await
            .err()
            .expect("toolchain setup failure")
            .to_string(),
        "select toolchains"
    );
    assert!(guarded_before_attachment.get());
    database
        .write(|connection| {
            connection.exec("ALTER TABLE unavailable_toolchains RENAME TO toolchains")?()
        })
        .await
        .expect("restore toolchain table");
    let restoring = window
        .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
        .expect("attached workspace");
    restoring
        .update_in(cx, |workspace, window, cx| {
            assert!(workspace.restoring_workspace);
            assert_eq!(workspace.database_id(), Some(workspace_id));
            workspace.flush_serialization(window, cx)
        })
        .await;
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved));
}

struct PendingRemoteConnection {
    options: RemoteConnectionOptions,
    identifiers: Mutex<Vec<String>>,
}

impl RemoteConnection for PendingRemoteConnection {
    fn start_proxy(
        &self,
        unique_identifier: String,
        _reconnect: bool,
        incoming: mpsc::UnboundedSender<client::proto::Envelope>,
        outgoing: mpsc::UnboundedReceiver<client::proto::Envelope>,
        activity: mpsc::Sender<()>,
        _delegate: Arc<dyn RemoteClientDelegate>,
        cx: &mut AsyncApp,
    ) -> Task<Result<i32>> {
        self.identifiers
            .lock()
            .expect("identifiers")
            .push(unique_identifier);
        cx.background_spawn(async move {
            let result = futures::future::pending().await;
            drop((incoming, outgoing, activity));
            result
        })
    }

    fn upload_directory(&self, _: PathBuf, _: RemotePathBuf, _: &App) -> Task<Result<()>> {
        Task::ready(Err(anyhow!("unexpected upload")))
    }

    fn kill<'life0, 'async_trait>(
        &'life0 self,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async { Ok(()) })
    }

    fn has_been_killed(&self) -> bool {
        false
    }

    fn build_command(
        &self,
        _: Option<String>,
        _: &[String],
        _: &HashMap<String, String>,
        _: Option<String>,
        _: Option<(u16, String, u16)>,
        _: Interactive,
    ) -> Result<CommandTemplate> {
        Err(anyhow!("unexpected command"))
    }

    fn build_forward_ports_command(&self, _: Vec<(u16, String, u16)>) -> Result<CommandTemplate> {
        Err(anyhow!("unexpected port forwarding"))
    }

    fn connection_options(&self) -> RemoteConnectionOptions {
        self.options.clone()
    }
    fn path_style(&self) -> PathStyle {
        PathStyle::Unix
    }
    fn remote_platform(&self) -> RemotePlatform {
        RemotePlatform {
            os: remote::RemoteOs::Linux,
            arch: remote::RemoteArch::X86_64,
        }
    }
    fn remote_os_version(&self) -> Option<String> {
        None
    }
    fn shell(&self) -> String {
        "sh".to_owned()
    }
    fn default_system_shell(&self) -> String {
        "sh".to_owned()
    }
    fn has_wsl_interop(&self) -> bool {
        false
    }
}

struct ItemIdProvider {
    ids: Vec<ItemId>,
    reads: Cell<usize>,
}

impl Global for ItemIdProvider {}

async fn restore_fixture(
    cx: &mut TestAppContext,
) -> (
    Entity<Workspace>,
    WorkspaceDb,
    SerializedWorkspace,
    &mut VisualTestContext,
) {
    init_test(cx);
    cx.update(|cx| {
        register_serializable_item::<TestItem>(cx);
        cx.on_app_quit(crate::flush_windows_serialization_on_quit)
            .detach();
    });
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"a.rs": "first\nsecond\n"}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let database = cx.update(|_, cx| WorkspaceDb::global(cx));
    let workspace_id = database.next_id().await.expect("workspace ID");
    workspace.update(cx, |workspace, _| workspace.set_database_id(workspace_id));
    cx.run_until_parked();
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    cx.run_until_parked();
    let mut saved = database.workspace_for_id(workspace_id).expect("workspace");
    saved.center_group = restored_pane_group(1, 2);
    saved.bookmarks = BTreeMap::from([(
        Arc::from(Path::new(path!("/project/a.rs"))),
        vec![SerializedBookmark {
            row: 1,
            label: "saved bookmark".to_owned(),
        }],
    )]);
    saved.breakpoints = BTreeMap::from([(
        Arc::from(Path::new(path!("/project/a.rs"))),
        vec![SourceBreakpoint {
            row: 1,
            path: Arc::from(Path::new(path!("/project/a.rs"))),
            message: None,
            condition: None,
            hit_condition: None,
            state: BreakpointState::Disabled,
        }],
    )]);
    saved.recent_navigation_history = vec![PathBuf::from(path!("/project/a.rs"))];
    database.save_workspace(saved.clone()).await;
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
    (workspace, database, saved, cx)
}

fn restored_pane_group(first_item_id: ItemId, second_item_id: ItemId) -> SerializedPaneGroup {
    SerializedPaneGroup::Group {
        axis: SerializedAxis(Axis::Horizontal),
        flexes: Some(vec![0.5, 1.5]),
        children: vec![
            SerializedPaneGroup::Pane(SerializedPane::new(
                vec![SerializedItem::new("TestItem", first_item_id, true, false)],
                false,
                1,
            )),
            SerializedPaneGroup::Pane(SerializedPane::new(
                vec![SerializedItem::new("TestItem", second_item_id, true, true)],
                true,
                0,
            )),
        ],
    }
}

struct PayloadGate(Shared<Task<Result<(), Arc<anyhow::Error>>>>);

impl Global for PayloadGate {}

struct RestoreGate {
    entered: Option<oneshot::Sender<()>>,
    receiver: Option<oneshot::Receiver<()>>,
    failed_items: HashSet<ItemId>,
    restored_items: Vec<(ItemId, ItemId)>,
    serialized_items: Rc<RefCell<Vec<ItemId>>>,
}

impl Global for RestoreGate {}

struct CleanupGate {
    entered: Option<oneshot::Sender<()>>,
    receiver: Option<oneshot::Receiver<()>>,
    item_ids: Vec<ItemId>,
}

impl Global for CleanupGate {}

#[derive(Default)]
struct RestorePayloadCleanup {
    calls: Vec<Vec<ItemId>>,
}

impl Global for RestorePayloadCleanup {}

async fn assert_hot_exit_graph_failure_prompts(close_intent: CloseIntent, cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id.0;
    database
        .write(move |connection| {
            connection.exec(&format!(
                "CREATE TRIGGER fail_hot_exit_graph BEFORE INSERT ON items
             WHEN NEW.workspace_id = {workspace_id}
             BEGIN SELECT RAISE(ABORT, 'injected hot-exit graph failure'); END;"
            ))?()
        })
        .await
        .expect("inject graph publication failure");
    let serialized = Rc::new(Cell::new(0));
    let item = cx.new(|cx| {
        let serialized = serialized.clone();
        let mut item = TestItem::new(cx).with_dirty(true).with_serialize(move || {
            serialized.set(serialized.get() + 1);
            Some(Task::ready(Ok(())))
        });
        item.state = String::from("new dirty untitled text absent from old graph");
        item
    });
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
    });
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    for answer in ["Cancel", "Don't Save", "Save"] {
        let closing = cx.cx.spawn(async move |mut cx| {
            crate::prepare_window_to_close(window, close_intent, &mut cx).await
        });
        cx.run_until_parked();
        assert!(serialized.get() > 0);
        assert!(cx.has_pending_prompt());
        assert!(!closing.is_ready());
        assert_eq!(
            database
                .workspace_for_id(saved.id)
                .map(|workspace| workspace.center_group),
            Some(saved.center_group.clone())
        );
        item.read_with(cx, |item, _| {
            assert_eq!(item.state, "new dirty untitled text absent from old graph");
            assert!(item.is_dirty);
            assert_eq!(item.save_as_count, 0);
        });
        cx.simulate_prompt_answer(answer);
        if answer == "Save" {
            cx.run_until_parked();
            cx.simulate_new_path_selection(|_| Some(PathBuf::from(path!("/project/saved.txt"))));
        }
        let decision = closing.await;
        if answer == "Cancel" {
            assert!(!decision.expect("cancel close"));
            assert!(!workspace.read_with(cx, |workspace, _| workspace.removing));
        } else if close_intent == CloseIntent::CloseWindow {
            assert_eq!(
                decision
                    .expect_err("failed session publication keeps window open")
                    .to_string(),
                "removing closed window from session",
            );
            assert!(!workspace.read_with(cx, |workspace, _| workspace.removing));
        } else {
            assert!(decision.expect("accept quit after explicit save or discard"));
        }
        assert!(window.read_with(cx, |_, _| ()).is_ok());
    }
    item.read_with(cx, |item, _| {
        assert_eq!(item.state, "new dirty untitled text absent from old graph");
        assert_eq!(item.save_as_count, 1);
        assert!(!item.is_dirty);
    });
}

async fn assert_restore_publication_failure_skips_cleanup(
    graph_failure: bool,
    cx: &mut TestAppContext,
) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (publish, receiver) = oneshot::channel();
    let payload = cx
        .executor()
        .spawn(async move {
            let success = receiver.await.map_err(|error| Arc::new(anyhow!(error)))?;
            if success {
                Ok(())
            } else {
                Err(Arc::new(anyhow!("injected payload failure")))
            }
        })
        .shared();
    cx.update(|_, cx| cx.set_global(PayloadGate(payload)));
    let (entered, release, serialized_items) = install_restore_gate(cx);
    let (mut cleanup_entered, _release_cleanup) = install_cleanup_gate(cx);
    if graph_failure {
        let workspace_id = saved.id.0;
        database
            .write(move |connection| {
                connection.exec(&format!(
                    "CREATE TRIGGER fail_restore_graph BEFORE INSERT ON items
                WHEN NEW.workspace_id = {workspace_id}
                BEGIN SELECT RAISE(ABORT, 'injected graph publication failure'); END;"
                ))?()
            })
            .await
            .expect("install graph publication failure");
    }
    let restore = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    entered.await.expect("second pane deserializer entered");
    release.send(()).expect("release second pane");
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert!(!restore.is_ready());
    assert_eq!(cleanup_entered.try_recv().expect("cleanup sender"), None);
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
    publish
        .send(graph_failure)
        .expect("release payload publication");
    let error = restore.await.err().expect("publication must fail");
    assert_eq!(
        error.to_string(),
        "persisting restored workspace before cleanup"
    );
    cx.run_until_parked();
    assert_eq!(cleanup_entered.try_recv().expect("cleanup sender"), None);
    assert_eq!(database.workspace_for_id(saved.id), Some(saved));
    let serialized_ids = serialized_items
        .borrow()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(serialized_ids, BTreeSet::from([1, 2]));
}

async fn assert_restore_survives_waiter_drop(before_poll: bool, cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (entered, release, _) = install_restore_gate(cx);
    let restore = workspace.update_in(cx, |workspace, window, cx| {
        let restore = workspace.load_workspace(saved.clone(), Vec::new(), window, cx);
        assert!(workspace.is_restoring());
        if before_poll {
            drop(restore);
            None
        } else {
            Some(restore)
        }
    });
    cx.run_until_parked();
    entered
        .now_or_never()
        .expect("restore stopped before the second pane")
        .expect("second pane deserializer entered");
    workspace.read_with(cx, |workspace, cx| {
        assert!(workspace.is_restoring());
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            vec![0, 1, 0]
        );
    });
    drop(restore);
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.serialize_workspace(window, cx);
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));

    release.send(()).expect("release second pane");
    cx.run_until_parked();
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, _| {
        assert!(!workspace.is_restoring());
        assert!(workspace._restore_workspace_task.is_none());
    });
    assert_restored_graph(&database, &saved, cx);
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_restored_graph(&database, &saved, cx);
}

async fn assert_restore_flushes_during_cleanup(shutdown: bool, cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (entered, release, serialized_items) = install_restore_gate(cx);
    let (cleanup_entered, release_cleanup) = install_cleanup_gate(cx);
    let restore = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    entered.await.expect("second pane deserializer entered");
    let pending_item = cx.new(TestItem::new);
    let pending_id = workspace.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", pending_item.entity_id(), cx)
            .expect("assign unpublished item")
    });
    drop(pending_item);
    release.send(()).expect("release second pane");
    cleanup_entered.await.expect("cleanup entered");
    cx.update(|_, cx| {
        let gate = cx.global_mut::<CleanupGate>();
        gate.item_ids.sort_unstable();
        assert_eq!(gate.item_ids, vec![1, 2, pending_id]);
    });
    cx.run_until_parked();
    assert_restored_graph(&database, &saved, cx);
    workspace.read_with(cx, |workspace, _| {
        assert!(!workspace.is_restoring());
        assert_eq!(workspace.panes.len(), 2);
        assert!(workspace._restore_workspace_task.is_some());
    });
    drop(restore);
    let overlapping_restore = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await;
    assert_eq!(
        overlapping_restore
            .err()
            .expect("overlapping restore must not cancel cleanup")
            .to_string(),
        "workspace restoration is already in progress"
    );
    serialized_items.borrow_mut().clear();

    if shutdown {
        cx.executor().allow_parking();
        cx.executor().set_block_on_ticks(10_000..=10_000);
        cx.cx.update(|cx| cx.shutdown());
        cx.executor().forbid_parking();
    } else {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
    }
    assert_restored_graph(&database, &saved, cx);
    serialized_items.borrow_mut().sort_unstable();
    assert_eq!(*serialized_items.borrow(), vec![1, 2]);

    if !shutdown {
        release_cleanup.send(()).expect("release cleanup");
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, _| {
            assert!(!workspace.is_restoring());
            assert!(workspace._restore_workspace_task.is_none());
        });
        assert_restored_graph(&database, &saved, cx);
    }
}

async fn assert_item_restore_failure_preserves_graph_and_payloads(
    all_fail: bool,
    cx: &mut TestAppContext,
) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id;
    let items = (1..=3)
        .map(|item_id| {
            SerializedItem::new(
                "TestItem",
                item_id,
                item_id == 3 || (!all_fail && item_id == 1),
                item_id == 3,
            )
        })
        .collect::<Vec<_>>();
    saved.center_group = if all_fail {
        SerializedPaneGroup::Pane(SerializedPane::new(items, true, 2))
    } else {
        SerializedPaneGroup::Group {
            axis: SerializedAxis(Axis::Horizontal),
            flexes: Some(vec![0.5, 1.5]),
            children: vec![
                SerializedPaneGroup::Pane(SerializedPane::new(items[..1].to_vec(), false, 1)),
                SerializedPaneGroup::Pane(SerializedPane::new(items[1..].to_vec(), true, 1)),
            ],
        }
    };
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("save restoration graph");
    database
        .write(move |connection| {
            connection.exec(
                "CREATE TABLE restore_test_payloads (workspace_id INTEGER, item_id INTEGER, payload TEXT) STRICT",
            )?()?;
            let mut insert = connection.exec_bound::<(WorkspaceId, ItemId, String)>(
                "INSERT INTO restore_test_payloads VALUES (?, ?, ?)",
            )?;
            for item_id in [1, 2, 3, 99] {
                insert((workspace_id, item_id, format!("saved payload {item_id}")))?;
            }
            anyhow::Ok(())
        })
        .await
        .expect("seed saved payloads");
    let expected_payloads = vec![
        (1, "saved payload 1".to_owned()),
        (2, "saved payload 2".to_owned()),
        (3, "saved payload 3".to_owned()),
        (99, "saved payload 99".to_owned()),
    ];
    assert_eq!(restore_payloads(&database, workspace_id), expected_payloads);
    let original_pane = workspace.read_with(cx, |workspace, _| workspace.center.first_pane());
    let (entered, release, serialized_items) = install_restore_gate(cx);
    cx.update(|_, cx| {
        cx.global_mut::<RestoreGate>().failed_items = if all_fail {
            HashSet::from_iter([1, 2, 3])
        } else {
            HashSet::from_iter([2])
        };
        cx.set_global(RestorePayloadCleanup::default());
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("TestItem descriptor")
            .cleanup = |workspace_id, mut item_ids, _, cx| {
            item_ids.sort_unstable();
            cx.global_mut::<RestorePayloadCleanup>()
                .calls
                .push(item_ids.clone());
            crate::delete_unloaded_items(
                item_ids,
                workspace_id,
                "restore_test_payloads",
                &WorkspaceDb::global(cx),
                cx,
            )
        };
    });
    let restore = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    entered.await.expect("item deserializer entered");
    let overlapping = workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .err()
        .expect("overlapping restore must fail");
    assert_eq!(
        overlapping.to_string(),
        "workspace restoration is already in progress"
    );
    cx.run_until_parked();
    assert!(!restore.is_ready());
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
    release.send(()).expect("release item deserializer");
    let error = restore.await.err().expect("item restoration must fail");
    let failed_item_id = if all_fail { 1 } else { 2 };
    assert_eq!(error.to_string(), "Could not deserialize pane");
    assert_eq!(
        error.root_cause().to_string(),
        format!("injected payload decode failure for item {failed_item_id}")
    );
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        assert!(workspace.is_restoring());
        assert!(workspace._restore_workspace_task.is_none());
        assert_eq!(workspace.center.first_pane(), original_pane);
        assert_eq!(original_pane.read(cx).items_len(), 0);
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            if all_fail { vec![0, 0] } else { vec![0, 1, 1] }
        );
        let mut assigned_ids = workspace.assigned_serialized_item_ids("TestItem");
        assigned_ids.sort_unstable();
        assert_eq!(assigned_ids, if all_fail { Vec::new() } else { vec![1, 3] });
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.serialize_workspace(window, cx);
            assert!(workspace._schedule_serialize_workspace.is_none());
            workspace.serialize_workspace_internal(window, cx)
        })
        .await
        .expect("serialization suppressed after failure");
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    cx.update(|_, cx| crate::flush_windows_serialization_on_quit(cx))
        .await;
    cx.executor().advance_clock(Duration::from_millis(500));
    cx.run_until_parked();
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
    assert_eq!(restore_payloads(&database, workspace_id), expected_payloads);
    assert_eq!(*serialized_items.borrow(), Vec::<ItemId>::new());
    cx.update(|_, cx| {
        assert_eq!(
            cx.global::<RestorePayloadCleanup>().calls,
            Vec::<Vec<ItemId>>::new()
        );
        let cleanup = cx
            .global::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get("TestItem")
            .expect("TestItem descriptor")
            .cleanup;
        register_serializable_item::<TestItem>(cx);
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("TestItem descriptor")
            .cleanup = cleanup;
    });
    let workspace = workspace.update_in(cx, |workspace, window, cx| {
        let project = workspace.project().clone();
        let app_state = workspace.app_state().clone();
        cx.new(|cx| Workspace::new(Some(workspace_id), project, app_state, window, cx))
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(
                database
                    .workspace_for_id(workspace_id)
                    .expect("preserved graph"),
                Vec::new(),
                window,
                cx,
            )
        })
        .await
        .expect("clean restoration retry");
    workspace.read_with(cx, |workspace, cx| {
        assert!(!workspace.is_restoring());
        assert!(workspace._restore_workspace_task.is_none());
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            if all_fail { vec![3] } else { vec![1, 2] }
        );
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(
        database
            .workspace_for_id(workspace_id)
            .expect("restored graph")
            .center_group,
        saved.center_group
    );
    assert_eq!(
        restore_payloads(&database, workspace_id),
        expected_payloads[..3]
    );
    cx.update(|_, cx| {
        assert_eq!(
            cx.global::<RestorePayloadCleanup>().calls,
            vec![vec![1, 2, 3]]
        );
    });
}

fn restore_payloads(database: &WorkspaceDb, workspace_id: WorkspaceId) -> Vec<(ItemId, String)> {
    database
        .select_bound::<WorkspaceId, (ItemId, String)>(
            "SELECT item_id, payload FROM restore_test_payloads WHERE workspace_id = ? ORDER BY item_id",
        )
        .expect("prepare saved payloads")(workspace_id)
        .expect("read saved payloads")
}

fn install_cleanup_gate(
    cx: &mut VisualTestContext,
) -> (oneshot::Receiver<()>, oneshot::Sender<()>) {
    let (cleanup_sender, cleanup_entered) = oneshot::channel();
    let (release_cleanup, cleanup_receiver) = oneshot::channel();
    cx.update(|_, cx| {
        cx.set_global(CleanupGate {
            entered: Some(cleanup_sender),
            receiver: Some(cleanup_receiver),
            item_ids: Vec::new(),
        });
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("TestItem descriptor")
            .cleanup = |_, item_ids, _, cx| {
            let gate = cx.global_mut::<CleanupGate>();
            gate.item_ids = item_ids;
            let entered = gate.entered.take().expect("cleanup sender");
            let receiver = gate.receiver.take().expect("cleanup receiver");
            cx.foreground_executor().spawn(async move {
                entered.send(()).expect("cleanup listener");
                receiver.await?;
                Ok(())
            })
        };
    });
    (cleanup_entered, release_cleanup)
}

fn install_restore_gate(
    cx: &mut VisualTestContext,
) -> (
    oneshot::Receiver<()>,
    oneshot::Sender<()>,
    Rc<RefCell<Vec<ItemId>>>,
) {
    let (entered_sender, entered) = oneshot::channel();
    let (release, receiver) = oneshot::channel();
    let serialized_items = Rc::new(RefCell::new(Vec::new()));
    cx.update(|_, cx| {
        cx.set_global(RestoreGate {
            entered: Some(entered_sender),
            receiver: Some(receiver),
            failed_items: HashSet::default(),
            restored_items: Vec::new(),
            serialized_items: serialized_items.clone(),
        });
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("TestItem descriptor")
            .deserialize = |_, _, workspace_id, item_id, _, cx| {
            let gate = cx.global_mut::<RestoreGate>();
            let receiver = if item_id == 2 {
                gate.entered
                    .take()
                    .expect("gate sender")
                    .send(())
                    .expect("gate listener");
                Some(gate.receiver.take().expect("gate receiver"))
            } else {
                None
            };
            let serialized_items = gate.serialized_items.clone();
            let fail = gate.failed_items.contains(&item_id);
            let payload = cx.try_global::<PayloadGate>().map(|gate| gate.0.clone());
            let executor = cx.background_executor().clone();
            let item = cx.new(|cx| {
                TestItem::new_deserialized(workspace_id, cx).with_serialize_id(
                    move |serialized_id| {
                        serialized_items.borrow_mut().push(serialized_id);
                        let payload = payload.clone();
                        Some(executor.spawn(async move {
                            if let Some(payload) = payload {
                                payload.await.map_err(|error| anyhow!(error))?;
                            }
                            Ok(())
                        }))
                    },
                )
            });
            cx.global_mut::<RestoreGate>()
                .restored_items
                .push((item_id, item.entity_id().as_u64()));
            cx.spawn(async move |_, _| {
                if let Some(receiver) = receiver {
                    receiver.await?;
                }
                if fail {
                    return Err(
                        anyhow!("injected payload decode failure for item {item_id}")
                            .context("reading saved TestItem payload"),
                    );
                }
                Ok(Box::new(item) as Box<dyn ItemHandle>)
            })
        };
    });
    (entered, release, serialized_items)
}

fn assert_restored_graph(
    database: &WorkspaceDb,
    saved: &SerializedWorkspace,
    cx: &mut VisualTestContext,
) {
    let restored_items = cx
        .cx
        .read(|cx| cx.global::<RestoreGate>().restored_items.clone());
    let [(1, _), (2, _)] = restored_items.as_slice() else {
        panic!("unexpected restored items: {restored_items:?}");
    };
    let restored = database.workspace_for_id(saved.id).expect("restored graph");
    assert_eq!(restored.center_group, restored_pane_group(1, 2));
    assert_eq!(restored.bookmarks, saved.bookmarks);
    assert_eq!(restored.breakpoints, saved.breakpoints);
    assert_eq!(
        restored.recent_navigation_history,
        saved.recent_navigation_history
    );
}

struct CloseFixture {
    window: WindowHandle<MultiWorkspace>,
    workspaces: Vec<Entity<Workspace>>,
    database: WorkspaceDb,
    bindings: Vec<(WorkspaceId, Option<String>, Option<u64>)>,
}

async fn close_fixture(
    other_window: bool,
    dirty: bool,
    cx: &mut TestAppContext,
) -> (CloseFixture, &mut VisualTestContext) {
    init_test(cx);
    cx.update(register_serializable_item::<TestItem>);
    cx.update_global::<SettingsStore, ()>(|store, cx| {
        store.update_user_settings(cx, |settings| {
            settings.workspace.on_last_window_closed = Some(OnLastWindowClosed::QuitApp);
        });
    });
    let fs = FakeFs::new(cx.executor());
    if other_window {
        let project = Project::test(fs.clone(), [], cx).await;
        cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
    }
    let first_project = Project::test(fs.clone(), [], cx).await;
    let second_project = Project::test(fs, [], cx).await;
    let database = cx.read(WorkspaceDb::global);
    let first_id = database.next_id().await.expect("first workspace ID");
    let second_id = database.next_id().await.expect("second workspace ID");
    let saved_window_id = WindowId::from(4_294_967_510);
    let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
        let workspace = cx.new(|cx| {
            let mut workspace = Workspace::test_new(first_project, window, cx);
            workspace.set_database_id(first_id);
            workspace.serialized_window_id = Some(saved_window_id);
            workspace
        });
        MultiWorkspace::new(workspace, window, cx)
    });
    let first =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let session_id = first.read_with(cx, |workspace, _| {
        workspace.session_id.clone().expect("session ID")
    });
    let second = multi_workspace.update_in(cx, |multi_workspace, window, cx| {
        multi_workspace.open_sidebar(cx);
        let workspace = cx.new(|cx| Workspace::test_new(second_project, window, cx));
        workspace.update(cx, |workspace, _| {
            workspace.set_database_id(second_id);
            workspace.session_id = Some(session_id.clone());
        });
        multi_workspace.add(workspace.clone(), window, cx);
        workspace
    });
    let workspaces = vec![first, second];
    for workspace in &workspaces {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
    }
    cx.run_until_parked();
    let template = database
        .workspace_for_id(first_id)
        .expect("saved workspace");
    for (session_id, window_id) in [
        (session_id.clone(), saved_window_id.as_u64()),
        (session_id.clone(), saved_window_id.as_u64() + 1),
        (String::from("other-session"), saved_window_id.as_u64()),
    ] {
        let mut saved = template.clone();
        saved.id = database
            .next_id()
            .await
            .expect("uninstantiated workspace ID");
        saved.session_id = Some(session_id);
        saved.window_id = Some(window_id);
        database
            .try_save_workspace(saved)
            .await
            .expect("seed uninstantiated workspace");
    }
    for workspace in &workspaces {
        let item = cx.new(|cx| TestItem::new(cx).with_dirty(dirty));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item), None, true, window, cx);
        });
    }
    cx.run_until_parked();
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("multi-workspace window")
    });
    let bindings = session_bindings(&database);
    assert_eq!(bindings.len(), 5);
    (
        CloseFixture {
            window,
            workspaces,
            database,
            bindings,
        },
        cx,
    )
}

async fn assert_window_close_preserves_group(
    close_intent: CloseIntent,
    other_window: bool,
    cx: &mut TestAppContext,
) {
    let (fixture, cx) = close_fixture(other_window, false, cx).await;
    let window = fixture.window;
    let closing = cx.cx.spawn(async move |mut cx| {
        crate::prepare_window_to_close(window, close_intent, &mut cx).await
    });
    assert!(closing.await.expect("accepted close"));
    for workspace in &fixture.workspaces {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
    }
    assert_eq!(session_bindings(&fixture.database), fixture.bindings);
    for workspace in &fixture.workspaces {
        workspace.read_with(cx, |workspace, _| {
            assert!(workspace.session_id.is_some());
            assert!(workspace.serialized_window_id.is_some());
        });
    }
}

fn assert_live_bindings(fixture: &CloseFixture, cx: &VisualTestContext) {
    for (workspace, (_, session_id, window_id)) in fixture.workspaces.iter().zip(&fixture.bindings)
    {
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(&workspace.session_id, session_id);
            assert_eq!(
                workspace.serialized_window_id.map(|id| id.as_u64()),
                *window_id
            );
            assert!(!workspace.removing);
        });
    }
}

fn session_bindings(database: &WorkspaceDb) -> Vec<(WorkspaceId, Option<String>, Option<u64>)> {
    database
        .select::<(WorkspaceId, Option<String>, Option<u64>)>(
            "SELECT workspace_id, session_id, window_id FROM workspaces ORDER BY workspace_id",
        )
        .expect("prepare session bindings query")()
    .expect("read session bindings")
}
