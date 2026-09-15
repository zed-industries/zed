use crate::{
    CloseIntent, ItemHandle, ItemId, MultiWorkspace, OpenMode, SerializableItemRegistry,
    SerializedItemIds, Workspace, WorkspaceDb, WorkspaceId,
    invalid_item_view::{InvalidItemView, SerializedItemReference},
    item::{
        SaveDisposition,
        test::{TestItem, TestProjectItem},
    },
    pane::{DraggedTab, SaveIntent},
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
use settings::{OnLastWindowClosed, Settings, SettingsStore};
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
        let mut namespace = SerializedItemIds::default();
        namespace.reserve_ids(&[runtime_id.as_u64(), maximum]);
        assert_eq!(
            namespace.allocate(runtime_id).expect("allocate ID"),
            expected
        );
        assert_eq!(namespace.allocate(runtime_id).expect("stable ID"), expected);
    }
    let mut namespace = SerializedItemIds::default();
    namespace.reserve_ids(&[runtime_id.as_u64(), u64::MAX]);
    assert_eq!(
        namespace
            .allocate(runtime_id)
            .expect_err("exhausted namespace")
            .to_string(),
        "serialized item ID namespace exhausted"
    );
    assert_eq!(namespace.by_runtime_id.len(), 0);
    let unreserved = EntityId::from(0x1_0000_0002);
    assert!(namespace.allocate(unreserved).is_err());
}

#[test]
fn test_serialized_item_ids_reject_conflicting_registrations() {
    let first = EntityId::from(0x1_0000_0001);
    let second = EntityId::from(0x1_0000_0002);
    let mut namespace = SerializedItemIds::default();
    namespace.reserve_ids(&[1, 2]);
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

#[test]
fn test_saved_reference_reservations_advance_cached_high_water_mark() {
    for maximum in [1_u64 << 63, u64::MAX - 1, u64::MAX] {
        let first = EntityId::from(0x1_0000_0010);
        let second = EntityId::from(0x1_0000_0011);
        let mut namespace = SerializedItemIds::default();
        assert_eq!(
            namespace.allocate(first).expect("initial allocation"),
            first.as_u64()
        );
        namespace
            .reserve_references("Terminal", &[0, maximum])
            .expect("decoded graph references");
        assert_eq!(
            namespace
                .allocate(first)
                .expect("stable existing allocation"),
            first.as_u64()
        );
        if maximum == u64::MAX {
            assert_eq!(
                namespace
                    .allocate(second)
                    .expect_err("exhausted namespace")
                    .to_string(),
                "serialized item ID namespace exhausted"
            );
        } else {
            assert_eq!(
                namespace
                    .allocate(second)
                    .expect("skip all saved references"),
                maximum + 1
            );
        }
        assert_eq!(
            namespace.reference_kinds.get(&0).map(AsRef::as_ref),
            Some("Terminal")
        );
    }
    let mut namespace = SerializedItemIds::default();
    namespace.reserve_ids(&[0x1_0000_0064]);
    namespace
        .reserve_references("Terminal", &[1])
        .expect("lower late reference");
    assert_eq!(
        namespace
            .allocate(EntityId::from(0x1_0000_0002))
            .expect("keep earlier maximum"),
        0x1_0000_0065
    );
}

#[gpui::test]
async fn test_panel_only_missing_references_reserve_ids_without_claiming_liveness(
    cx: &mut TestAppContext,
) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let allocated = EntityId::from(0x1_0000_0020);
    let missing_id = 1_u64 << 63;
    workspace.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", allocated, cx)
            .expect("initialize namespace before panel decode");
        assert_eq!(
            workspace
                .serialized_item_ids
                .as_ref()
                .expect("namespace")
                .reserved
                .get(&missing_id),
            None
        );
        workspace
            .reserve_serialized_item_ids(saved.id, "Terminal", &[missing_id, missing_id + 1], cx)
            .expect("all decoded panel references");
        assert_eq!(
            workspace.live_serialized_item_ids("Terminal", cx),
            Vec::<ItemId>::new()
        );
        assert_eq!(
            workspace.assigned_serialized_item_ids("Terminal"),
            Vec::<ItemId>::new()
        );
        let fresh = workspace
            .serialization_id("TestItem", EntityId::from(0x1_0000_0021), cx)
            .expect("allocation after panel reservation");
        assert_eq!(fresh, missing_id + 2);
        let failed = EntityId::from(0x1_0000_0022);
        workspace
            .register_serialized_item_id("Terminal", failed, missing_id, cx)
            .expect("missing payload placeholder owns its saved ID");
        workspace
            .register_serialized_item_id("Terminal", failed, missing_id, cx)
            .expect("idempotent registration");
        assert_eq!(
            workspace
                .serialization_id("Terminal", failed, cx)
                .expect("original identity"),
            missing_id
        );
        assert!(
            workspace
                .register_serialized_item_id(
                    "TestItem",
                    EntityId::from(0x1_0000_0023),
                    missing_id + 1,
                    cx
                )
                .is_err()
        );
        assert!(
            workspace
                .register_serialized_item_id(
                    "Terminal",
                    EntityId::from(0x1_0000_0024),
                    missing_id,
                    cx
                )
                .is_err()
        );
        assert_eq!(
            workspace.live_serialized_item_ids("Terminal", cx),
            Vec::<ItemId>::new()
        );
    });
    assert_eq!(
        database
            .serialized_item_ids(saved.id, "Terminal")
            .expect("no center terminal references"),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_saved_reference_reservation_rejects_cross_kind_and_wrong_workspace_atomically(
    cx: &mut TestAppContext,
) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    workspace.update(cx, |workspace, cx| {
        let owner = EntityId::from(0x1_0000_0030);
        let allocated = workspace
            .serialization_id("TestItem", owner, cx)
            .expect("current owner");
        let next = allocated + 100;
        let before = workspace
            .serialized_item_ids
            .as_ref()
            .expect("namespace")
            .reserved
            .clone();
        for conflicting_id in [1, allocated] {
            assert!(
                workspace
                    .reserve_serialized_item_ids(saved.id, "Terminal", &[next, conflicting_id], cx)
                    .is_err()
            );
            assert_eq!(
                workspace
                    .serialized_item_ids
                    .as_ref()
                    .expect("namespace")
                    .reserved,
                before
            );
            assert_eq!(
                workspace
                    .serialized_item_ids
                    .as_ref()
                    .expect("namespace")
                    .reference_kinds
                    .get(&next),
                None
            );
        }
        assert!(
            workspace
                .reserve_serialized_item_ids(WorkspaceId::from_i64(-1), "Terminal", &[next], cx)
                .is_err()
        );
        assert_eq!(
            workspace
                .serialized_item_ids
                .as_ref()
                .expect("namespace")
                .reserved,
            before
        );
        workspace
            .reserve_serialized_item_ids(saved.id, "TestItem", &[allocated], cx)
            .expect("reservation does not transfer ownership");
        assert!(
            workspace
                .register_serialized_item_id(
                    "TestItem",
                    EntityId::from(0x1_0000_0031),
                    allocated,
                    cx
                )
                .is_err()
        );
        assert_eq!(
            workspace
                .serialization_id("TestItem", owner, cx)
                .expect("existing owner unchanged"),
            allocated
        );
    });
}

#[gpui::test]
async fn test_registration_refreshes_only_missing_provider_ids(cx: &mut TestAppContext) {
    let (workspace, _, _, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: Vec::new(),
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
        let mut unrelated = *descriptor;
        unrelated.serialized_item_ids = |_, _| Ok(Vec::new());
        registry
            .descriptors_by_kind
            .insert(Arc::from("OtherItem"), unrelated);
    });
    workspace.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", EntityId::from(0x1_0000_0040), cx)
            .expect("cached namespace");
    });
    let repaired_id = 1_u64 << 63;
    cx.update(|_, cx| {
        cx.global_mut::<ItemIdProvider>().ids = vec![repaired_id];
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("OtherItem")
            .expect("unrelated provider")
            .serialized_item_ids = |_, _| panic!("unrelated provider must not refresh");
    });
    workspace.update(cx, |workspace, cx| {
        let restored = EntityId::from(0x1_0000_0041);
        workspace
            .register_serialized_item_id("TestItem", restored, repaired_id, cx)
            .expect("repaired provider row");
        workspace
            .register_serialized_item_id("TestItem", restored, repaired_id, cx)
            .expect("known registration does not refresh");
        assert_eq!(
            workspace
                .serialization_id("TestItem", restored, cx)
                .expect("repaired identity"),
            repaired_id
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 2);
        assert_eq!(
            workspace
                .serialization_id("OtherItem", EntityId::from(0x1_0000_0042), cx)
                .expect("late payload is reserved globally"),
            repaired_id + 1
        );
        assert!(
            workspace
                .register_serialized_item_id(
                    "TestItem",
                    EntityId::from(0x1_0000_0043),
                    repaired_id,
                    cx
                )
                .is_err()
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 2);
    });
}

#[gpui::test]
async fn test_registration_refreshes_repaired_graph_and_checks_kind_before_cached_payload(
    cx: &mut TestAppContext,
) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let corrected_id = 1_u64 << 63;
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: vec![corrected_id + 1],
            reads: Cell::new(0),
        });
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, cx| Ok(cx.global::<ItemIdProvider>().ids.clone());
    });
    workspace.update(cx, |workspace, cx| {
        workspace
            .serialized_item_id_namespace(cx)
            .expect("old namespace");
    });
    saved.center_group = SerializedPaneGroup::Pane(SerializedPane::new(
        vec![
            SerializedItem::new("Terminal", corrected_id, true, false),
            SerializedItem::new("Terminal", corrected_id + 1, false, false),
        ],
        true,
        0,
    ));
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("repaired graph source");
    workspace.update(cx, |workspace, cx| {
        workspace
            .register_serialized_item_id(
                "Terminal",
                EntityId::from(0x1_0000_0050),
                corrected_id,
                cx,
            )
            .expect("new graph reference without a payload row");
        assert!(
            workspace
                .register_serialized_item_id(
                    "TestItem",
                    EntityId::from(0x1_0000_0051),
                    corrected_id + 1,
                    cx
                )
                .is_err()
        );
        workspace
            .register_serialized_item_id(
                "Terminal",
                EntityId::from(0x1_0000_0052),
                corrected_id + 1,
                cx,
            )
            .expect("authoritative graph kind");
        assert_eq!(
            workspace.assigned_serialized_item_ids("TestItem"),
            Vec::<ItemId>::new()
        );
    });
}

#[gpui::test]
async fn test_repaired_workspace_source_reserves_all_ids_and_refreshes_negative_payload_snapshot(
    cx: &mut TestAppContext,
) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: Vec::new(),
            reads: Cell::new(0),
        });
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        descriptor.serialized_item_ids = |_, cx| {
            let provider = cx.global::<ItemIdProvider>();
            provider.reads.set(provider.reads.get() + 1);
            Ok(provider.ids.clone())
        };
    });
    workspace.update(cx, |workspace, cx| {
        workspace
            .serialized_item_id_namespace(cx)
            .expect("negative payload snapshot");
    });
    let first_id = 1_u64 << 63;
    saved.center_group = restored_pane_group(first_id, first_id + 1);
    cx.update(|_, cx| {
        cx.global_mut::<ItemIdProvider>().ids = vec![first_id, first_id + 1];
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .deserialize = |_, workspace, workspace_id, _, _, cx| {
            let unrelated = cx.new(TestItem::new);
            let allocated = workspace
                .update(cx, |workspace, cx| {
                    workspace.serialization_id("OtherItem", unrelated.entity_id(), cx)
                })
                .expect("workspace")
                .expect("allocation during construction");
            assert!(allocated > (1_u64 << 63) + 1);
            Task::ready(Ok(
                Box::new(cx.new(|cx| TestItem::new_deserialized(workspace_id, cx)))
                    as Box<dyn ItemHandle>,
            ))
        };
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("repaired decoded source");
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("published graph")
            .center_group,
        saved.center_group
    );
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.items_of_type::<InvalidItemView>(cx).count(), 0);
        assert_eq!(workspace.items_of_type::<TestItem>(cx).count(), 2);
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 2);
    });
}

#[gpui::test]
async fn test_provider_refresh_failure_preserves_reservations_and_cached_ids(
    cx: &mut TestAppContext,
) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    workspace.update(cx, |workspace, cx| {
        workspace
            .serialized_item_id_namespace(cx)
            .expect("cached namespace");
    });
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, _| Err(anyhow!("injected refresh failure"));
    });
    workspace.update(cx, |workspace, cx| {
        let namespace = workspace.serialized_item_ids.as_ref().expect("namespace");
        let reserved = namespace.reserved.clone();
        let payload_ids = namespace.payload_ids.clone();
        let high_water_mark = namespace.high_water_mark;
        assert_eq!(
            workspace
                .refresh_serialized_item_ids(saved.id, "TestItem", cx)
                .expect_err("refresh failure")
                .to_string(),
            "injected refresh failure"
        );
        let namespace = workspace.serialized_item_ids.as_ref().expect("namespace");
        assert_eq!(namespace.reserved, reserved);
        assert_eq!(namespace.payload_ids, payload_ids);
        assert_eq!(namespace.high_water_mark, high_water_mark);
        workspace
            .reserve_serialized_item_ids(saved.id, "Terminal", &[121, 122], cx)
            .expect("decoded references do not require a provider refresh");
    });
}

#[gpui::test]
async fn test_registration_rejects_inflight_owner_until_its_serialization_finishes(
    cx: &mut TestAppContext,
) {
    let (workspace, _, _, cx) = restore_fixture(cx).await;
    let (release, receiver) = oneshot::channel();
    let serialization = workspace.update(cx, |workspace, cx| {
        let receiver = RefCell::new(Some(receiver));
        let executor = cx.foreground_executor().clone();
        let first = cx.new(|cx| {
            TestItem::new(cx).with_serialize(move || {
                let receiver = receiver.borrow_mut().take().expect("single serialization");
                Some(executor.spawn(async move {
                    receiver.await?;
                    Ok(())
                }))
            })
        });
        workspace
            .register_serialized_item_id("TestItem", first.entity_id(), 1, cx)
            .expect("first owner");
        first
            .to_serializable_item_handle(cx)
            .expect("serializable owner")
            .serialize(workspace, false, cx)
            .expect("inflight serialization")
    });
    let replacement = cx.new(TestItem::new);
    cx.run_until_parked();
    workspace.update(cx, |workspace, cx| {
        assert_eq!(workspace.live_serialized_item_ids("TestItem", cx), vec![1]);
        assert!(
            workspace
                .register_serialized_item_id("TestItem", replacement.entity_id(), 1, cx)
                .is_err()
        );
    });
    release.send(()).expect("release serialization");
    serialization.await.expect("serialization finished");
    cx.run_until_parked();
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace.live_serialized_item_ids("TestItem", cx),
            Vec::<ItemId>::new()
        );
        workspace
            .register_serialized_item_id("TestItem", replacement.entity_id(), 1, cx)
            .expect("released owner can be replaced");
        workspace.track_serialized_item(replacement.downgrade_item());
        assert_eq!(workspace.live_serialized_item_ids("TestItem", cx), vec![1]);
    });
}

#[gpui::test]
async fn test_failed_registration_does_not_release_an_unproven_saved_identity(
    cx: &mut TestAppContext,
) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let first = cx.new(TestItem::new);
    let first_runtime_id = first.entity_id();
    let item_id = workspace.update(cx, |workspace, cx| {
        let item_id = workspace
            .serialization_id("TestItem", first_runtime_id, cx)
            .expect("allocated identity");
        workspace.track_serialized_item(first.downgrade_item());
        item_id
    });
    cx.update(|_, _| drop(first));
    cx.run_until_parked();
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, _| Err(anyhow!("injected refresh failure"));
    });
    let replacement = cx.new(TestItem::new);
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace
                .register_serialized_item_id("TestItem", replacement.entity_id(), item_id, cx)
                .expect_err("cannot prove saved identity")
                .to_string(),
            "injected refresh failure"
        );
        assert_eq!(
            workspace
                .serialized_item_ids
                .as_ref()
                .expect("namespace")
                .by_runtime_id
                .get(&first_runtime_id),
            Some(&item_id)
        );
        assert!(
            workspace
                .reserve_serialized_item_ids(saved.id, "Terminal", &[item_id], cx)
                .is_err()
        );
        assert_eq!(
            workspace.live_serialized_item_ids("TestItem", cx),
            Vec::<ItemId>::new()
        );
    });
}

#[gpui::test]
async fn test_registration_releases_only_proven_dead_owners(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let first = cx.new(TestItem::new);
    let second = cx.new(TestItem::new);
    workspace.update(cx, |workspace, cx| {
        workspace
            .register_serialized_item_id("TestItem", first.entity_id(), 1, cx)
            .expect("first owner");
        workspace.track_serialized_item(first.downgrade_item());
        assert!(
            workspace
                .register_serialized_item_id("TestItem", second.entity_id(), 1, cx)
                .is_err()
        );
    });
    cx.update(|_, _| drop(first));
    cx.run_until_parked();
    workspace.update(cx, |workspace, cx| {
        workspace
            .register_serialized_item_id("TestItem", second.entity_id(), 1, cx)
            .expect("released owner can be restored again");
        workspace.track_serialized_item(second.downgrade_item());
        assert_eq!(workspace.live_serialized_item_ids("TestItem", cx), vec![1]);
        assert!(
            workspace
                .reserve_serialized_item_ids(saved.id, "Terminal", &[1], cx)
                .is_err()
        );
        workspace
            .register_serialized_item_id("TestItem", EntityId::from(0x1_0000_0060), 2, cx)
            .expect("owner without lifetime tracking");
        assert!(
            workspace
                .register_serialized_item_id("TestItem", EntityId::from(0x1_0000_0061), 2, cx)
                .is_err()
        );
        assert!(
            workspace
                .register_serialized_item_id("Terminal", EntityId::from(0x1_0000_0062), 99, cx)
                .is_err()
        );
    });
}

#[gpui::test]
async fn test_serialization_ids_reserve_all_provider_rows_lazily(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let first = EntityId::from(0x1_0000_0010);
    let second = EntityId::from(first.as_u64() + 1);
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: vec![first.as_u64()],
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
        let mut other = *descriptor;
        other.serialized_item_ids = |_, _| Ok(Vec::new());
        registry
            .descriptors_by_kind
            .insert(Arc::from("OtherItem"), other);
    });
    workspace.update(cx, |workspace, cx| {
        assert_eq!(
            workspace.assigned_serialized_item_ids("TestItem"),
            Vec::<ItemId>::new()
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 0);
        assert_eq!(
            workspace
                .serialization_id("TestItem", first, cx)
                .expect("remap first kind"),
            first.as_u64() + 1
        );
        assert_eq!(
            workspace
                .serialization_id("OtherItem", second, cx)
                .expect("adjacent other kind"),
            first.as_u64() + 2
        );
        assert_eq!(
            workspace
                .serialization_id("TestItem", first, cx)
                .expect("stable first"),
            first.as_u64() + 1
        );
        assert_eq!(cx.global::<ItemIdProvider>().reads.get(), 1);
        assert_eq!(
            workspace.assigned_serialized_item_ids("TestItem"),
            vec![first.as_u64() + 1]
        );
        assert_eq!(
            workspace.assigned_serialized_item_ids("OtherItem"),
            vec![first.as_u64() + 2]
        );
    });
    saved.center_group = SerializedPaneGroup::Pane(SerializedPane::new(
        vec![
            SerializedItem::new("TestItem", first.as_u64() + 1, true, false),
            SerializedItem::new("OtherItem", first.as_u64() + 2, false, false),
        ],
        true,
        0,
    ));
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("distinct cross-kind graph keys");
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("graph")
            .center_group,
        saved.center_group
    );
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
        assert!(workspace.serialized_item_ids.is_none())
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
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("install failed tabs");
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("saved graph")
            .center_group,
        failed_preview_pane_group(first_id.as_u64(), second_id.as_u64())
    );
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
        assert!(!workspace.is_restoring());
        assert_eq!(workspace.panes.len(), 1);
        assert_eq!(workspace.items(cx).count(), 0);
    });
}

#[gpui::test]
async fn test_nonserializable_tabs_skip_payload_and_graph(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let item = cx.new(|cx| {
        let mut item = TestItem::new(cx).with_serialize(|| panic!("nonserializable payload write"));
        item.serializable = false;
        item
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
            assert!(
                item.to_serializable_item_handle(cx)
                    .expect("registered handle")
                    .serialize(workspace, true, cx)
                    .is_none()
            );
            workspace.serialize_workspace_internal(window, cx)
        })
        .await
        .expect("skip nonserializable tab");
    assert_eq!(
        database
            .serialized_item_ids(saved.id, "TestItem")
            .expect("graph IDs"),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_missing_payload_keeps_visible_reference(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = |_, _| Ok(vec![1]);
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("legacy missing payload");
    workspace.read_with(cx, |workspace, cx| {
        assert!(!workspace.is_restoring());
        assert_eq!(workspace.panes.len(), 2);
        let failed = workspace.panes[1]
            .read(cx)
            .item_for_index(0)
            .expect("tab")
            .downcast::<crate::invalid_item_view::InvalidItemView>()
            .expect("visible failure");
        assert_eq!(
            failed.read(cx).error.as_ref(),
            "Saved TestItem payload 2 is missing"
        );
    });
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("preserved graph")
            .center_group,
        failed_preview_pane_group(1, 2)
    );
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
    assert_hot_exit_graph_failure_prompts(CloseIntent::CloseWindow, "Cancel", cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_quit_prompts(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::Quit, "Cancel", cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_close_allows_save(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::CloseWindow, "Save", cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_close_allows_discard(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::CloseWindow, "Don't Save", cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_quit_allows_save(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::Quit, "Save", cx).await;
}

#[gpui::test]
async fn test_hot_exit_graph_failure_quit_allows_discard(cx: &mut TestAppContext) {
    assert_hot_exit_graph_failure_prompts(CloseIntent::Quit, "Don't Save", cx).await;
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
async fn test_open_workspace_by_id_rejects_remote_location(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let unowned_id = database.next_id().await.expect("unowned ID");
    saved.location = crate::SerializedWorkspaceLocation::Remote(RemoteConnectionOptions::Ssh(
        remote::SshConnectionOptions {
            host: "remote.test".into(),
            ..remote::SshConnectionOptions::default()
        },
    ));
    for workspace_id in [saved.id, unowned_id] {
        saved.id = workspace_id;
        if workspace_id == unowned_id {
            saved.window_bounds = None;
            saved.display = None;
        }
        database
            .try_save_workspace(saved.clone())
            .await
            .expect("seed remote workspace");
        let result = cx
            .update(|_, cx| crate::open_workspace_by_id(workspace_id, app_state.clone(), None, cx))
            .await;
        assert_eq!(
            result.expect_err("reject remote provider").to_string(),
            format!("Workspace {workspace_id:?} is not local")
        );
        assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
        assert_eq!(cx.update(|_, cx| cx.windows().len()), 1);
    }
}

#[gpui::test]
async fn test_remote_restore_rejects_mismatched_provider(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let connection = Arc::new(PendingRemoteConnection {
        options: RemoteConnectionOptions::Ssh(remote::SshConnectionOptions {
            host: "requested.test".into(),
            ..remote::SshConnectionOptions::default()
        }),
        identifiers: Mutex::new(Vec::new()),
    });
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    for location in [
        crate::SerializedWorkspaceLocation::Local,
        crate::SerializedWorkspaceLocation::Remote(RemoteConnectionOptions::Ssh(
            remote::SshConnectionOptions {
                host: "other.test".into(),
                ..remote::SshConnectionOptions::default()
            },
        )),
    ] {
        saved.location = location;
        database
            .try_save_workspace(saved.clone())
            .await
            .expect("seed provider");
        let (_cancel, cancelled) = oneshot::channel();
        let result = cx
            .update(|_, cx| {
                crate::open_remote_project_with_new_connection(
                    window,
                    connection.clone(),
                    cancelled,
                    Arc::new(remote::MockDelegate),
                    app_state.clone(),
                    Vec::new(),
                    Some(saved.id),
                    cx,
                )
            })
            .await;
        assert_eq!(
            result
                .err()
                .expect("reject mismatched provider")
                .to_string(),
            format!(
                "Workspace {:?} does not match the remote connection",
                saved.id
            )
        );
        assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
        assert!(
            connection
                .identifiers
                .lock()
                .expect("identifiers")
                .is_empty()
        );
    }
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
        MultiWorkspace::test_from_workspace(workspace, window, cx)
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
async fn test_replace_pinned_workspace_waits_for_detached_publication(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (app_state, project) = workspace.read_with(cx, |workspace, _| {
        (workspace.app_state().clone(), workspace.project().clone())
    });
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/replacement"), json!({}))
        .await;
    cx.update(|_, cx| {
        project::DisableAiSettings::override_global(
            project::DisableAiSettings { disable_ai: true },
            cx,
        );
    });
    cx.run_until_parked();
    let pending_id = database.next_id().await.expect("pending member ID");
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    let pending_member = window
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace.add(workspace.clone(), window, cx);
            let pending = cx.new(|cx| {
                let mut workspace =
                    Workspace::new(Some(pending_id), project, app_state.clone(), window, cx);
                workspace.restoring_workspace = true;
                workspace
            });
            multi_workspace.add(pending.clone(), window, cx);
            pending
        })
        .expect("retain startup members");
    let pending_binding = pending_member.read_with(cx, |workspace, _| {
        (workspace.session_id.clone(), workspace.serialized_window_id)
    });
    let mut pending_graph = saved.clone();
    pending_graph.centered_layout = !saved.centered_layout;
    let (release, receiver) = oneshot::channel();
    let publication = cx
        .executor()
        .spawn({
            let database = database.clone();
            async move {
                receiver.await.map_err(|error| Arc::new(anyhow!(error)))?;
                database
                    .try_save_workspace(pending_graph)
                    .await
                    .map_err(Arc::new)
            }
        })
        .shared();
    workspace.update(cx, |workspace, _| {
        workspace._schedule_serialize_workspace.take();
        workspace.pending_workspace_serialization = Some(publication);
    });
    let replacement = window
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace.open_project(
                vec![PathBuf::from(path!("/replacement"))],
                OpenMode::Activate,
                window,
                cx,
            )
        })
        .expect("start replacement")
        .await
        .expect("replace consented member");
    window
        .read_with(cx, |multi_workspace, _| {
            assert_eq!(multi_workspace.workspace(), &replacement);
            assert_eq!(
                multi_workspace.workspaces().cloned().collect::<Vec<_>>(),
                vec![pending_member.clone(), replacement.clone()]
            );
        })
        .expect("replacement members");
    let removal_waiters = window
        .update(cx, |multi_workspace, _, _| {
            multi_workspace.take_pending_removal_tasks()
        })
        .expect("removal waiters");
    drop(removal_waiters);
    cx.run_until_parked();
    let blocked = cx
        .update(|_, cx| crate::open_workspace_by_id(saved.id, app_state.clone(), None, cx))
        .await;
    assert_eq!(
        blocked
            .expect_err("detached publisher still owns the ID")
            .to_string(),
        format!("Workspace {:?} is being detached", saved.id)
    );
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
    let replacement_binding = replacement.read_with(cx, |workspace, _| {
        (workspace.session_id.clone(), workspace.serialized_window_id)
    });
    release.send(()).expect("release detached publication");
    cx.run_until_parked();
    let detached = database
        .workspace_for_id(saved.id)
        .expect("published detached graph");
    assert_eq!(detached.centered_layout, saved.centered_layout);
    assert_eq!(detached.center_group, saved.center_group);
    assert_eq!(detached.session_id, None);
    assert_eq!(detached.window_id, None);
    assert_eq!(
        pending_member.read_with(cx, |workspace, _| {
            (workspace.session_id.clone(), workspace.serialized_window_id)
        }),
        pending_binding
    );
    assert_eq!(
        replacement.read_with(cx, |workspace, _| {
            (workspace.session_id.clone(), workspace.serialized_window_id)
        }),
        replacement_binding
    );
    let reopened = cx
        .update(|_, cx| crate::open_workspace_by_id(saved.id, app_state, None, cx))
        .await
        .expect("reopen after final unbind");
    reopened
        .read_with(cx, |multi_workspace, cx| {
            let reopened = multi_workspace.workspace();
            assert_ne!(reopened, &workspace);
            assert_eq!(reopened.read(cx).database_id(), Some(saved.id));
            assert_eq!(
                reopened
                    .read(cx)
                    .panes
                    .iter()
                    .map(|pane| pane.read(cx).items_len())
                    .collect::<Vec<_>>(),
                vec![1, 1]
            );
        })
        .expect("reopened owner");
    assert_eq!(
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.serialize_workspace_internal(window, cx)
            })
            .await
            .expect_err("detached owner cannot overwrite reopened graph")
            .to_string(),
        "workspace was detached"
    );
}

#[gpui::test]
async fn test_local_owner_reuse_joins_restoration_after_opener_cancellation(
    cx: &mut TestAppContext,
) {
    assert_owner_reuse_after_cancellation(None, cx).await;
}

#[gpui::test]
async fn test_remote_owner_reuse_joins_restoration_after_opener_cancellation(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    assert_owner_reuse_after_cancellation(Some(server_cx), cx).await;
}

#[gpui::test]
async fn test_local_owner_retries_repaired_setup_failure(cx: &mut TestAppContext) {
    assert_owner_retries_setup_failure(None, cx).await;
}

#[gpui::test]
async fn test_remote_owner_retries_repaired_setup_failure(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    assert_owner_retries_setup_failure(Some(server_cx), cx).await;
}

#[gpui::test]
async fn test_owner_retry_preserves_items_opened_after_setup_failure(cx: &mut TestAppContext) {
    let (fixture, cx) = owner_restore_fixture(None, cx).await;
    let original = cx.update(|_, cx| {
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        let original = descriptor.serialized_item_ids;
        descriptor.serialized_item_ids = |_, _| Err(anyhow!("injected owner setup failure"));
        original
    });
    assert!(start_owner_restore(&fixture, cx).await.is_err());
    let owner = cx.cx.update(|cx| {
        crate::find_open_workspace_by_id(fixture.saved.id, cx)
            .expect("failed owner")
            .1
    });
    let item = cx.new(|cx| {
        let mut item = TestItem::new(cx).with_dirty(true);
        item.state = String::from("new text after failed restoration");
        item
    });
    owner.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(item.clone()), None, true, window, cx);
    });
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = original;
    });
    assert_eq!(
        reuse_owner_restore(&fixture, cx)
            .await
            .expect_err("retry must not replace new items")
            .to_string(),
        "cannot retry workspace restoration over open items"
    );
    owner.read_with(cx, |workspace, cx| {
        assert_eq!(
            workspace
                .items(cx)
                .map(|item| item.item_id())
                .collect::<Vec<_>>(),
            vec![item.entity_id()]
        );
        assert_eq!(item.read(cx).state, "new text after failed restoration");
        assert!(item.read(cx).is_dirty);
    });
    assert_eq!(
        fixture.database.workspace_for_id(fixture.saved.id),
        Some(fixture.saved)
    );
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
    let retry_project = project.clone();
    let retry_app_state = app_state.clone();
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
    assert!(!guarded_before_attachment.get());
    assert_eq!(
        window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .expect("original owner"),
        workspace
    );
    assert_eq!(database.workspace_for_id(workspace_id), Some(saved.clone()));
    database
        .write(|connection| {
            connection.exec("ALTER TABLE unavailable_toolchains RENAME TO toolchains")?()
        })
        .await
        .expect("restore toolchain table");
    let retry = cx.cx.spawn(async move |mut cx| {
        crate::open_remote_project_inner(
            retry_project,
            vec![PathBuf::from(path!("/project"))],
            workspace_id,
            Some(saved),
            retry_app_state,
            window,
            None,
            None,
            &mut cx,
        )
        .await
    });
    let (restored, _) = retry.await.expect("retry repaired remote setup");
    assert!(guarded_before_attachment.get());
    restored.read_with(cx, |workspace, cx| {
        assert!(!workspace.is_restoring());
        assert_eq!(workspace.database_id(), Some(workspace_id));
        assert_eq!(workspace.items(cx).count(), 2);
    });
    assert_eq!(
        database
            .workspace_for_id(workspace_id)
            .expect("restored graph")
            .center_group,
        restored_pane_group(1, 2)
    );
}

#[gpui::test]
async fn test_failed_tab_save_and_close_disposition(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 1, cx);
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    cx.update(|_, cx| {
        assert_eq!(failed.save_disposition(cx), SaveDisposition::DiscardOnly);
        assert!(!failed.can_save(cx));
        assert!(!failed.can_save_as(cx));
    });
    for intent in [SaveIntent::Save, SaveIntent::SaveAs, SaveIntent::SaveAll] {
        assert_eq!(
            workspace
                .update_in(cx, |workspace, window, cx| workspace
                    .save_active_item(intent, window, cx))
                .await
                .expect_err("unrecovered active item cannot be saved")
                .to_string(),
            "Retry the failed tab before saving; its saved reference has been kept."
        );
        let saving = workspace.update_in(cx, |workspace, window, cx| {
            workspace.save_all_internal(intent, false, window, cx)
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("OK");
        assert!(!saving.await.expect("save must retain unrecovered data"));
        assert_eq!(pane_state(&pane, cx).0, vec![failed.entity_id()]);
        let closing = pane.update_in(cx, |pane, window, cx| {
            pane.close_item_by_id(failed.entity_id(), intent, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer("OK");
        closing.await.expect("save on close");
        assert_eq!(pane_state(&pane, cx).0, vec![failed.entity_id()]);
    }
    for answer in ["Cancel", "Discard"] {
        let closing = pane.update_in(cx, |pane, window, cx| {
            pane.close_item_by_id(failed.entity_id(), SaveIntent::Close, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer(answer);
        closing.await.expect("close disposition");
        let expected = if answer == "Cancel" {
            vec![failed.entity_id()]
        } else {
            Vec::new()
        };
        assert_eq!(pane_state(&pane, cx).0, expected);
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
        assert_eq!(
            database
                .serialized_item_ids(saved.id, "TestItem")
                .expect("references"),
            if answer == "Cancel" {
                vec![1]
            } else {
                Vec::new()
            }
        );
    }
}

#[gpui::test]
async fn test_failed_tabs_bulk_close_cancel_save_and_discard(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let first = add_failed_item(&workspace, saved.id, 1, cx);
    let second = add_failed_item(&workspace, saved.id, 2, cx);
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    for answer in ["Cancel", "Save all", "Discard all"] {
        let closing = pane.update_in(cx, |pane, window, cx| {
            pane.close_items(window, cx, SaveIntent::Close, &|_| true)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer(answer);
        if answer == "Save all" {
            for _ in 0..2 {
                cx.run_until_parked();
                cx.simulate_prompt_answer("OK");
            }
        }
        closing.await.expect("bulk close disposition");
        assert_eq!(
            pane_state(&pane, cx).0,
            if answer == "Discard all" {
                Vec::new()
            } else {
                vec![first.entity_id(), second.entity_id()]
            }
        );
    }
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(
        database
            .serialized_item_ids(saved.id, "TestItem")
            .expect("discarded references"),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_failed_tabs_workspace_close_cancel_save_and_discard(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let first = add_failed_item(&workspace, saved.id, 1, cx);
    let second = add_failed_item(&workspace, saved.id, 2, cx);
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    for answer in ["Cancel", "Save all", "Discard all"] {
        let closing = workspace.update_in(cx, |workspace, window, cx| {
            workspace.save_all_internal(SaveIntent::Close, false, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer(answer);
        if answer == "Save all" {
            cx.run_until_parked();
            cx.simulate_prompt_answer("OK");
        }
        assert_eq!(
            closing.await.expect("workspace close"),
            answer == "Discard all"
        );
        assert_eq!(
            pane_state(&pane, cx).0,
            if answer == "Discard all" {
                Vec::new()
            } else {
                vec![first.entity_id(), second.entity_id()]
            }
        );
    }
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(
        database
            .serialized_item_ids(saved.id, "TestItem")
            .expect("discarded references"),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_failed_tab_unreachable_workspace_close_requires_discard(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 1, cx);
    let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
    project.update(cx, |project, cx| {
        let worktrees = project
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).id())
            .collect::<Vec<_>>();
        for worktree in worktrees {
            project.remove_worktree(worktree, cx);
        }
    });
    for answer in ["Cancel", "Discard"] {
        let closing = workspace.update_in(cx, |workspace, window, cx| {
            assert_eq!(
                workspace.project().read(cx).visible_worktrees(cx).count(),
                0
            );
            workspace.prepare_to_close_internal(CloseIntent::CloseWindow, false, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer(answer);
        assert_eq!(
            closing.await.expect("unreachable close"),
            answer == "Discard"
        );
        assert_eq!(
            workspace.read_with(cx, |workspace, cx| workspace
                .items(cx)
                .map(|item| item.item_id())
                .collect::<Vec<_>>()),
            if answer == "Cancel" {
                vec![failed.entity_id()]
            } else {
                Vec::new()
            }
        );
    }
}

#[gpui::test]
async fn test_failed_tab_retry_does_not_deserialize_after_close(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 2, cx);
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .deserialize = |_, _, _, _, _, _| panic!("closed failed tab must not deserialize");
    });
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    cx.update(|window, cx| {
        failed.update(cx, |failed, cx| failed.retry(window, cx));
        pane.update(cx, |pane, cx| {
            pane.remove_item(failed.entity_id(), false, false, window, cx)
        });
    });
    cx.run_until_parked();
    assert_eq!(pane_state(&pane, cx).0, Vec::<EntityId>::new());
    assert_eq!(
        workspace.read_with(cx, |workspace, _| workspace
            .assigned_serialized_item_ids("TestItem")),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_failed_tab_retry_closed_during_deserialization(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 2, cx);
    let (entered, release, _) = install_restore_gate(cx);
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    failed.update_in(cx, |failed, window, cx| failed.retry(window, cx));
    entered.await.expect("retry entered");
    pane.update_in(cx, |pane, window, cx| {
        pane.close_item_by_id(failed.entity_id(), SaveIntent::Skip, window, cx)
    })
    .await
    .expect("discard during retry");
    release.send(()).expect("release retry");
    cx.run_until_parked();
    assert_eq!(pane_state(&pane, cx).0, Vec::<EntityId>::new());
    assert_eq!(
        workspace.read_with(cx, |workspace, _| workspace
            .assigned_serialized_item_ids("TestItem")),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_failed_tab_retry_preserves_inactive_duplicate_owner(cx: &mut TestAppContext) {
    assert_retry_preserves_duplicate_owner(false, cx).await;
}

#[gpui::test]
async fn test_failed_tab_retry_preserves_active_duplicate_owner(cx: &mut TestAppContext) {
    assert_retry_preserves_duplicate_owner(true, cx).await;
}

#[gpui::test]
async fn test_failed_tab_retry_follows_move_during_deserialization(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 2, cx);
    let source = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    let destination = workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(source.clone(), crate::SplitDirection::Right, window, cx)
    });
    let active = cx.new(TestItem::new);
    destination.update_in(cx, |pane, window, cx| {
        pane.add_item(Box::new(active.clone()), true, true, None, window, cx)
    });
    let (entered, release, _) = install_restore_gate(cx);
    failed.update_in(cx, |failed, window, cx| failed.retry(window, cx));
    entered.await.expect("retry entered");
    cx.update(|window, cx| {
        crate::move_item(
            &source,
            &destination,
            failed.entity_id(),
            0,
            false,
            window,
            cx,
        )
    });
    destination.update_in(cx, |pane, window, cx| {
        pane.set_pinned_count(1);
        pane.activate_item(1, true, true, window, cx);
    });
    release.send(()).expect("release moved retry");
    cx.run_until_parked();
    let restored_id =
        cx.update(|_, cx| EntityId::from(cx.global::<RestoreGate>().restored_items[0].1));
    assert_eq!(
        pane_state(&destination, cx),
        (
            vec![restored_id, active.entity_id()],
            Some(active.entity_id()),
            1,
            None
        )
    );
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(
            workspace
                .items(cx)
                .filter(|item| item.item_id() == restored_id)
                .count(),
            1
        );
        assert_eq!(
            workspace
                .serialized_item_ids
                .as_ref()
                .expect("namespace")
                .by_runtime_id
                .get(&restored_id),
            Some(&2)
        );
    });
}

#[gpui::test]
async fn test_failed_tab_retry_does_not_activate_an_unfocused_pane(cx: &mut TestAppContext) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 2, cx);
    let source = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    source.update(cx, |pane, _| pane.set_pinned_count(1));
    let destination = workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(source.clone(), crate::SplitDirection::Right, window, cx)
    });
    let active = cx.new(TestItem::new);
    destination.update_in(cx, |pane, window, cx| {
        pane.add_item(Box::new(active.clone()), true, true, None, window, cx)
    });
    cx.run_until_parked();
    let (entered, release, _) = install_restore_gate(cx);
    failed.update_in(cx, |failed, window, cx| failed.retry(window, cx));
    entered.await.expect("retry entered");
    release.send(()).expect("release retry");
    cx.run_until_parked();
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.active_pane(), &destination);
        assert_eq!(source.read(cx).items_len(), 1);
        assert_eq!(source.read(cx).pinned_count(), 1);
        assert_eq!(
            source.read(cx).items_of_type::<InvalidItemView>().count(),
            0
        );
    });
    assert_eq!(pane_state(&destination, cx).1, Some(active.entity_id()));
    cx.update(|window, cx| assert!(active.item_focus_handle(cx).contains_focused(window, cx)));
}

#[gpui::test]
async fn test_cleanup_keeps_inflight_items_only_until_serialization_completes(
    cx: &mut TestAppContext,
) {
    let (workspace, _, _, cx) = restore_fixture(cx).await;
    let (release, receiver) = oneshot::channel();
    let (item_id, serialization) = workspace.update(cx, |workspace, cx| {
        let receiver = RefCell::new(Some(receiver));
        let executor = cx.foreground_executor().clone();
        let item = cx.new(|cx| {
            TestItem::new(cx).with_serialize(move || {
                let receiver = receiver.borrow_mut().take().expect("single serialization");
                Some(executor.spawn(async move {
                    receiver.await?;
                    Ok(())
                }))
            })
        });
        let serializable = item
            .to_serializable_item_handle(cx)
            .expect("serializable item");
        let task = serializable
            .serialize(workspace, false, cx)
            .expect("serialization task");
        let item_id = workspace
            .serialization_id("TestItem", item.entity_id(), cx)
            .expect("assigned ID");
        (item_id, task)
    });
    cx.run_until_parked();
    assert_eq!(
        workspace.read_with(cx, |workspace, cx| workspace
            .live_serialized_item_ids("TestItem", cx)),
        vec![item_id]
    );
    release.send(()).expect("release serialization");
    serialization.await.expect("serialization completes");
    cx.run_until_parked();
    assert_eq!(
        workspace.read_with(cx, |workspace, cx| workspace
            .live_serialized_item_ids("TestItem", cx)),
        Vec::<ItemId>::new()
    );
}

#[gpui::test]
async fn test_failed_tab_tabbar_drop_rejects_noncenter_and_other_workspace_atomically(
    cx: &mut TestAppContext,
) {
    let (workspace, _, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 1, cx);
    let source = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    source.update(cx, |pane, cx| {
        pane.set_pinned_count(1);
        pane.set_preview_item_id(Some(failed.entity_id()), cx);
    });
    let (terminal_pane, other_workspace) = workspace.update_in(cx, |workspace, window, cx| {
        let project = workspace.project().clone();
        let terminal_pane = cx.new(|cx| {
            crate::Pane::new(
                workspace.weak_handle(),
                project.clone(),
                Arc::default(),
                None,
                Box::new(crate::NewTerminal::default()),
                false,
                window,
                cx,
            )
        });
        let other =
            cx.new(|cx| Workspace::new(None, project, workspace.app_state().clone(), window, cx));
        (terminal_pane, other)
    });
    let other_pane = other_workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    for destination in [terminal_pane, other_pane] {
        let existing = cx.new(TestItem::new);
        destination.update_in(cx, |pane, window, cx| {
            pane.add_item(Box::new(existing.clone()), true, true, None, window, cx);
            pane.set_pinned_count(1);
            pane.set_preview_item_id(Some(existing.entity_id()), cx);
        });
        let before_source = pane_state(&source, cx);
        let before_destination = pane_state(&destination, cx);
        let before_panes = workspace.read_with(cx, |workspace, _| workspace.panes.clone());
        let dragged = DraggedTab {
            pane: source.clone(),
            item: Box::new(failed.clone()),
            ix: 0,
            detail: 0,
            is_active: true,
        };
        destination.update_in(cx, |pane, window, cx| {
            pane.handle_tab_drop(&dragged, 0, false, window, cx)
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("OK");
        cx.run_until_parked();
        assert_eq!(pane_state(&source, cx), before_source);
        assert_eq!(pane_state(&destination, cx), before_destination);
        assert_eq!(
            workspace.read_with(cx, |workspace, _| workspace.panes.clone()),
            before_panes
        );
    }
}

#[gpui::test]
async fn test_owner_retry_preserves_draft_created_while_deserializing(cx: &mut TestAppContext) {
    let (fixture, cx) = owner_restore_fixture(None, cx).await;
    let original = cx.update(|_, cx| {
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        let original = descriptor.serialized_item_ids;
        descriptor.serialized_item_ids = |_, _| Err(anyhow!("injected setup failure"));
        original
    });
    assert!(start_owner_restore(&fixture, cx).await.is_err());
    let owner = cx.cx.update(|cx| {
        crate::find_open_workspace_by_id(fixture.saved.id, cx)
            .expect("owner")
            .1
    });
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = original
    });
    let (entered, release, _) = install_restore_gate(cx);
    let retry = reuse_owner_restore(&fixture, cx);
    entered.await.expect("retry passed preflight");
    let draft = cx.new(|cx| {
        let mut item = TestItem::new(cx).with_dirty(true);
        item.state = String::from("draft created during retry");
        item
    });
    let interim = owner.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(draft.clone()), None, true, window, cx);
        workspace.active_pane().clone()
    });
    interim.update(cx, |pane, _| pane.set_pinned_count(1));
    release.send(()).expect("install restored panes");
    retry.await.expect("retry completes");
    owner.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.panes.len(), 3);
        assert_eq!(workspace.center.panes().len(), 3);
        assert_eq!(workspace.active_pane(), &interim);
        assert_eq!(
            workspace
                .items(cx)
                .filter(|item| item.item_id() == draft.entity_id())
                .count(),
            1
        );
        assert_eq!(draft.read(cx).state, "draft created during retry");
        assert!(draft.read(cx).is_dirty);
    });
    assert_eq!(
        pane_state(&interim, cx),
        (vec![draft.entity_id()], Some(draft.entity_id()), 1, None)
    );
    let ids = fixture
        .database
        .serialized_item_ids(fixture.saved.id, "TestItem")
        .expect("published references")
        .into_iter()
        .collect::<BTreeSet<_>>();
    let draft_id = owner.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", draft.entity_id(), cx)
            .expect("draft ID")
    });
    assert_eq!(ids, BTreeSet::from([1, 2, draft_id]));
}

#[gpui::test]
async fn test_workspace_restore_preserves_tab_order_selection_preview_and_pins(
    cx: &mut TestAppContext,
) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    saved.center_group = SerializedPaneGroup::Pane(SerializedPane::new(
        vec![
            SerializedItem::new("TestItem", 1, false, false),
            SerializedItem::new("TestItem", 2, true, false),
            SerializedItem::new("TestItem", 3, false, true),
        ],
        true,
        2,
    ));
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("saved tab order");
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("restored tab order");
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("published tab order")
            .center_group,
        saved.center_group
    );
    workspace.read_with(cx, |workspace, cx| {
        let pane = workspace.active_pane().read(cx);
        let namespace = workspace.serialized_item_ids.as_ref().expect("namespace");
        assert_eq!(
            pane.items()
                .map(|item| namespace.by_runtime_id[&item.item_id()])
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(pane.pinned_count(), 2);
    });
}

#[gpui::test]
async fn test_cleanup_reservations_do_not_keep_orphaned_payloads(cx: &mut TestAppContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id;
    saved.center_group = SerializedPaneGroup::Pane(SerializedPane::new(
        vec![SerializedItem::new("TestItem", 1, true, false)],
        true,
        0,
    ));
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("one live reference");
    database.write(move |connection| {
        connection.exec("CREATE TABLE restore_test_payloads (workspace_id INTEGER, item_id INTEGER, payload TEXT) STRICT")?()?;
        let mut insert = connection.exec_bound::<(WorkspaceId, ItemId, String)>("INSERT INTO restore_test_payloads VALUES (?, ?, ?)")?;
        for id in [1, 2, 99] { insert((workspace_id, id, format!("payload {id}")))?; }
        Ok::<_, anyhow::Error>(())
    }).await.expect("seed independent payload rows");
    cx.update(|_, cx| {
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        descriptor.serialized_item_ids = |workspace_id, cx| {
            WorkspaceDb::global(cx).select_bound::<WorkspaceId, ItemId>(
                "SELECT item_id FROM restore_test_payloads WHERE workspace_id = ?",
            )?(workspace_id)
        };
        descriptor.cleanup = |workspace_id, ids, _, cx| {
            crate::delete_unloaded_items(
                ids,
                workspace_id,
                "restore_test_payloads",
                &WorkspaceDb::global(cx),
                cx,
            )
        };
    });
    let discarded = cx.new(TestItem::new);
    workspace.update(cx, |workspace, cx| {
        workspace
            .register_serialized_item_id("TestItem", discarded.entity_id(), 2, cx)
            .expect("previous owner");
        workspace
            .reserve_serialized_item_ids(workspace_id, "TestItem", &[99], cx)
            .expect("reservation is not liveness");
        workspace.track_serialized_item(discarded.downgrade_item());
    });
    cx.update(|_, _| drop(discarded));
    cx.run_until_parked();
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved, Vec::new(), window, cx)
        })
        .await
        .expect("restore and cleanup");
    assert_eq!(
        restore_payloads(&database, workspace_id),
        vec![(1, String::from("payload 1"))]
    );
    let next = cx.new(TestItem::new);
    let next_id = workspace.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", next.entity_id(), cx)
            .expect("reserved namespace")
    });
    assert!(next_id > 99);
}

#[gpui::test]
async fn test_move_project_group_waits_for_cleanup_and_publication(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let (cleanup_entered, release_cleanup) = install_cleanup_gate(cx);
    let restoration = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    cleanup_entered.await.expect("restoration cleanup entered");
    let (window, key) = cx.update(|window, cx| {
        (
            window
                .window_handle()
                .downcast::<MultiWorkspace>()
                .expect("window"),
            workspace.read(cx).project_group_key(cx),
        )
    });
    window
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace.add(workspace.clone(), window, cx);
        })
        .expect("retain moved workspace");
    let (release_publication, receiver) = oneshot::channel();
    let publication = cx
        .executor()
        .spawn({
            let database = database.clone();
            let saved = saved.clone();
            async move {
                receiver.await.map_err(|error| Arc::new(anyhow!(error)))?;
                database.try_save_workspace(saved).await.map_err(Arc::new)
            }
        })
        .shared();
    workspace.update(cx, |workspace, _| {
        workspace._schedule_serialize_workspace.take();
        workspace.pending_workspace_serialization = Some(publication);
    });
    let moving = window
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace.open_project_group_in_new_window(&key, window, cx)
        })
        .expect("start explicit move");
    cx.run_until_parked();
    assert!(!moving.is_ready());
    release_publication.send(()).expect("release old publisher");
    cx.run_until_parked();
    assert!(
        !moving.is_ready(),
        "move must wait for restoration cleanup and final unbind"
    );
    let mut competing = saved.clone();
    competing.id = database.next_id().await.expect("competing root identity");
    competing.window_bounds = None;
    competing.display = None;
    competing.center_group = SerializedPaneGroup::Pane(SerializedPane::new(Vec::new(), true, 0));
    database
        .try_save_workspace(competing.clone())
        .await
        .expect("save competing roots");
    let competing_id = competing.id;
    database
        .write(move |connection| {
            connection.exec_bound::<WorkspaceId>(
                "UPDATE workspaces SET timestamp = '9999-12-31 23:59:59' WHERE workspace_id = ?",
            )?(competing_id)
        })
        .await
        .expect("make competing roots the newest candidate");
    assert_eq!(
        database
            .workspace_for_roots(&[PathBuf::from(path!("/project"))])
            .expect("root candidate")
            .id,
        competing.id
    );
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .cleanup = |_, _, _, _| Task::ready(Ok(()));
    });
    release_cleanup.send(()).expect("finish old cleanup");
    restoration.await.expect("restoration finishes");
    moving.await.expect("move saved workspace");
    let (new_window, reopened) = cx
        .cx
        .update(|cx| crate::find_open_workspace_by_id(saved.id, cx).expect("moved identity"));
    assert_ne!(new_window, window);
    assert_ne!(reopened, workspace);
    reopened.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.database_id(), Some(saved.id));
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            vec![1, 1]
        );
    });
    let binding = reopened.read_with(cx, |workspace, _| {
        (
            saved.id,
            workspace.session_id.clone(),
            workspace.serialized_window_id.map(|id| id.as_u64()),
        )
    });
    assert_eq!(
        session_bindings(&database)
            .into_iter()
            .find(|binding| binding.0 == saved.id),
        Some(binding)
    );
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("moved graph")
            .center_group,
        saved.center_group
    );
    assert!(workspace.read_with(cx, |workspace, _| workspace.serialization_detached));
    assert_eq!(database.workspace_for_id(competing.id), Some(competing));
}

#[gpui::test]
async fn test_move_project_group_unbind_failure_does_not_open_new_window(cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("restore moved tabs");
    let before = session_bindings(&database);
    let windows = cx.cx.update(|cx| cx.windows());
    let workspace_id = saved.id.0;
    database.write(move |connection| {
        connection.exec(&format!("CREATE TRIGGER fail_move_unbind BEFORE UPDATE OF session_id, window_id ON workspaces WHEN NEW.workspace_id = {workspace_id} AND NEW.window_id IS NULL BEGIN SELECT RAISE(ABORT, 'injected move unbind failure'); END;"))?()
    }).await.expect("fail final unbind");
    let (window, key) = cx.update(|window, cx| {
        (
            window
                .window_handle()
                .downcast::<MultiWorkspace>()
                .expect("window"),
            workspace.read(cx).project_group_key(cx),
        )
    });
    let moving = window
        .update(cx, |multi_workspace, window, cx| {
            assert!(!multi_workspace.is_workspace_retained(&workspace));
            multi_workspace.open_project_group_in_new_window(&key, window, cx)
        })
        .expect("move unpinned active workspace");
    assert!(moving.await.is_err());
    assert_eq!(cx.cx.update(|cx| cx.windows()), windows);
    assert_eq!(session_bindings(&database), before);
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("retained tabs")
            .center_group,
        saved.center_group
    );
}

#[gpui::test]
async fn test_restoration_publication_retry_flush_preserves_installed_entities(
    cx: &mut TestAppContext,
) {
    assert_publication_retry_preserves_installed_entities(false, cx).await;
}

#[gpui::test]
async fn test_restoration_publication_retry_by_id_preserves_installed_entities(
    cx: &mut TestAppContext,
) {
    assert_publication_retry_preserves_installed_entities(true, cx).await;
}

async fn assert_publication_retry_preserves_installed_entities(
    by_id: bool,
    cx: &mut TestAppContext,
) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id.0;
    database.write(move |connection| {
        connection.exec(&format!("CREATE TRIGGER fail_publication_retry BEFORE INSERT ON items WHEN NEW.workspace_id = {workspace_id} BEGIN SELECT RAISE(ABORT, 'injected publication retry failure'); END;"))?()
    }).await.expect("fail final publication");
    let (entered, release, _) = install_restore_gate(cx);
    let restoration = workspace.update_in(cx, |workspace, window, cx| {
        workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
    });
    entered.await.expect("deserialization entered");
    release.send(()).expect("install panes");
    assert_eq!(
        restoration
            .await
            .err()
            .expect("publication fails")
            .to_string(),
        "persisting restored workspace before cleanup"
    );
    let (panes, items, app_state) = workspace.read_with(cx, |workspace, cx| {
        (
            workspace.panes.clone(),
            workspace
                .items(cx)
                .map(|item| item.item_id())
                .collect::<Vec<_>>(),
            workspace.app_state().clone(),
        )
    });
    assert_eq!(database.workspace_for_id(saved.id), Some(saved.clone()));
    database
        .write(|connection| connection.exec("DROP TRIGGER fail_publication_retry")?())
        .await
        .expect("repair publication");
    let draft = cx.new(|cx| {
        let mut item = TestItem::new(cx).with_dirty(true);
        item.state = String::from("draft after publication failure");
        item
    });
    let draft_id = workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(draft.clone()), None, true, window, cx);
        workspace.active_pane().update(cx, |pane, cx| {
            pane.set_preview_item_id(None, cx);
            pane.set_pinned_count(1);
        });
        workspace
            .serialization_id("TestItem", draft.entity_id(), cx)
            .expect("draft identity")
    });
    if by_id {
        cx.update(|_, cx| crate::open_workspace_by_id(saved.id, app_state, None, cx))
            .await
            .expect("retry installed owner by ID");
    } else {
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
    }
    workspace.read_with(cx, |workspace, cx| {
        assert_eq!(workspace.panes, panes);
        assert_eq!(
            workspace
                .items(cx)
                .map(|item| item.item_id())
                .collect::<Vec<_>>(),
            items
                .into_iter()
                .chain([draft.entity_id()])
                .collect::<Vec<_>>()
        );
        assert_eq!(draft.read(cx).state, "draft after publication failure");
        assert!(draft.read(cx).is_dirty);
        assert!(!workspace.restoration_failed);
    });
    workspace
        .read_with(cx, |workspace, _| workspace.wait_for_restoration())
        .await
        .expect("repaired restoration result");
    assert_eq!(
        cx.update(|_, cx| cx.global::<RestoreGate>().restored_items.len()),
        2
    );
    assert_eq!(
        database
            .workspace_for_id(saved.id)
            .expect("repaired graph")
            .center_group,
        SerializedPaneGroup::Group {
            axis: SerializedAxis(Axis::Horizontal),
            flexes: Some(vec![0.5, 1.5]),
            children: vec![
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![SerializedItem::new("TestItem", 1, true, false)],
                    false,
                    1
                )),
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![
                        SerializedItem::new("TestItem", 2, false, false),
                        SerializedItem::new("TestItem", draft_id, true, false)
                    ],
                    true,
                    1
                )),
            ],
        }
    );
}

struct OwnerRestoreFixture {
    database: WorkspaceDb,
    saved: SerializedWorkspace,
    app_state: Arc<crate::AppState>,
    window: WindowHandle<MultiWorkspace>,
    remote: Option<(Entity<Project>, RemoteConnectionOptions, Entity<()>)>,
}

async fn owner_restore_fixture<'a>(
    server_cx: Option<&mut TestAppContext>,
    cx: &'a mut TestAppContext,
) -> (OwnerRestoreFixture, &'a mut VisualTestContext) {
    let (workspace, database, mut saved, cx) = restore_fixture(cx).await;
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let remote = if let Some(server_cx) = server_cx {
        cx.update(|_, cx| release_channel::init("0.0.0".parse().expect("test version"), cx));
        server_cx.update(|cx| release_channel::init("0.0.0".parse().expect("test version"), cx));
        let (options, server, guard) = remote::RemoteClient::fake_server(&mut cx.cx, server_cx);
        let handler = server_cx.new(|_| ());
        server.add_request_handler::<client::proto::Ping, _, _, _>(
            handler.downgrade(),
            |_, _, _| async { Ok(client::proto::Ack {}) },
        );
        drop(guard);
        let client = remote::RemoteClient::connect_mock(options.clone(), &mut cx.cx).await;
        let project = cx.update(|_, cx| {
            Project::remote(
                client,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                true,
                cx,
            )
        });
        saved.location = crate::SerializedWorkspaceLocation::Remote(options.clone());
        saved.paths = crate::PathList::default();
        saved.bookmarks.clear();
        saved.breakpoints.clear();
        saved.recent_navigation_history.clear();
        Some((project, options, handler))
    } else {
        None
    };
    saved.id = database.next_id().await.expect("unowned workspace ID");
    saved.window_bounds = None;
    saved.display = None;
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("seed saved owner");
    let window = cx.update(|window, _| {
        window
            .window_handle()
            .downcast::<MultiWorkspace>()
            .expect("window")
    });
    (
        OwnerRestoreFixture {
            database,
            saved,
            app_state,
            window,
            remote,
        },
        cx,
    )
}

fn start_owner_restore(
    fixture: &OwnerRestoreFixture,
    cx: &mut VisualTestContext,
) -> Task<Result<WindowHandle<MultiWorkspace>>> {
    let window = fixture.window;
    let saved = fixture.saved.clone();
    let app_state = fixture.app_state.clone();
    if let Some((project, _, _)) = &fixture.remote {
        let project = project.clone();
        cx.cx.spawn(async move |mut cx| {
            crate::open_remote_project_inner(
                project,
                Vec::new(),
                saved.id,
                Some(saved),
                app_state,
                window,
                None,
                None,
                &mut cx,
            )
            .await?;
            Ok(window)
        })
    } else {
        cx.update(|_, cx| crate::open_workspace_by_id(saved.id, app_state, Some(window), cx))
    }
}

fn reuse_owner_restore(
    fixture: &OwnerRestoreFixture,
    cx: &mut VisualTestContext,
) -> Task<Result<WindowHandle<MultiWorkspace>>> {
    let workspace_id = fixture.saved.id;
    if let Some((_, options, _)) = &fixture.remote {
        let options = options.clone();
        cx.cx.spawn(async move |mut cx| {
            Ok(
                crate::reuse_open_remote_workspace(workspace_id, &options, &mut cx)
                    .await?
                    .ok_or_else(|| anyhow!("missing remote owner"))?
                    .window,
            )
        })
    } else {
        cx.update(|_, cx| {
            crate::open_workspace_by_id(workspace_id, fixture.app_state.clone(), None, cx)
        })
    }
}

async fn assert_owner_reuse_after_cancellation(
    server_cx: Option<&mut TestAppContext>,
    cx: &mut TestAppContext,
) {
    let (fixture, cx) = owner_restore_fixture(server_cx, cx).await;
    let (entered, release, _) = install_restore_gate(cx);
    let opening = start_owner_restore(&fixture, cx);
    entered.await.expect("attached owner started deserializing");
    drop(opening);
    cx.run_until_parked();
    let owner = cx.cx.update(|cx| {
        crate::find_open_workspace_by_id(fixture.saved.id, cx)
            .expect("attached owner")
            .1
    });
    let joined = reuse_owner_restore(&fixture, cx);
    cx.run_until_parked();
    assert!(!joined.is_ready());
    assert_eq!(
        fixture.database.workspace_for_id(fixture.saved.id),
        Some(fixture.saved.clone())
    );
    release.send(()).expect("release restoration");
    assert_eq!(
        joined.await.expect("join owned restoration"),
        fixture.window
    );
    assert_restored_owner(&fixture, &owner, cx);
}

async fn assert_owner_retries_setup_failure(
    server_cx: Option<&mut TestAppContext>,
    cx: &mut TestAppContext,
) {
    let (fixture, cx) = owner_restore_fixture(server_cx, cx).await;
    let original = cx.update(|_, cx| {
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        let original = descriptor.serialized_item_ids;
        descriptor.serialized_item_ids = |_, _| Err(anyhow!("injected owner setup failure"));
        original
    });
    assert_eq!(
        start_owner_restore(&fixture, cx)
            .await
            .expect_err("setup fails")
            .to_string(),
        "injected owner setup failure"
    );
    let owner = cx.cx.update(|cx| {
        crate::find_open_workspace_by_id(fixture.saved.id, cx)
            .expect("failed owner retained")
            .1
    });
    assert_eq!(
        reuse_owner_restore(&fixture, cx)
            .await
            .expect_err("reuse must propagate failure")
            .to_string(),
        "injected owner setup failure"
    );
    assert_eq!(
        fixture.database.workspace_for_id(fixture.saved.id),
        Some(fixture.saved.clone())
    );
    cx.update(|_, cx| {
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .serialized_item_ids = original;
    });
    let (entered, release, _) = install_restore_gate(cx);
    let retry = reuse_owner_restore(&fixture, cx);
    entered.await.expect("retry starts deserialization");
    let joined = reuse_owner_restore(&fixture, cx);
    cx.run_until_parked();
    assert!(!retry.is_ready());
    assert!(!joined.is_ready());
    release.send(()).expect("release repaired restoration");
    assert_eq!(retry.await.expect("retry repaired owner"), fixture.window);
    assert_eq!(joined.await.expect("join repaired owner"), fixture.window);
    assert_restored_owner(&fixture, &owner, cx);
}

fn assert_restored_owner(
    fixture: &OwnerRestoreFixture,
    owner: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) {
    fixture
        .window
        .read_with(cx, |multi_workspace, cx| {
            let owners = multi_workspace
                .workspaces()
                .filter(|workspace| workspace.read(cx).database_id() == Some(fixture.saved.id))
                .cloned()
                .collect::<Vec<_>>();
            assert_eq!(owners, vec![owner.clone()]);
            let workspace = owner.read(cx);
            if let Some((_, options, _)) = &fixture.remote {
                assert_eq!(
                    workspace.project().read(cx).remote_connection_options(cx),
                    Some(options.clone())
                );
            }
            assert_eq!(
                workspace
                    .panes
                    .iter()
                    .map(|pane| pane.read(cx).items_len())
                    .collect::<Vec<_>>(),
                vec![1, 1]
            );
            assert_eq!(
                workspace
                    .assigned_serialized_item_ids("TestItem")
                    .into_iter()
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from([1, 2])
            );
            assert!(!workspace.is_restoring());
        })
        .expect("restored owner");
    assert_eq!(
        fixture
            .database
            .workspace_for_id(fixture.saved.id)
            .expect("published owner")
            .center_group,
        restored_pane_group(1, 2)
    );
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

fn add_failed_item(
    workspace: &Entity<Workspace>,
    workspace_id: WorkspaceId,
    item_id: ItemId,
    cx: &mut VisualTestContext,
) -> Entity<InvalidItemView> {
    workspace.update_in(cx, |workspace, window, cx| {
        let failed = cx.new(|cx| {
            InvalidItemView::for_serialized_item(
                SerializedItemReference {
                    workspace_id,
                    kind: Arc::from("TestItem"),
                    item_id,
                },
                workspace.weak_handle(),
                &anyhow!("injected failure"),
                window,
                cx,
            )
        });
        workspace.add_item_to_active_pane(Box::new(failed.clone()), None, true, window, cx);
        failed
    })
}

fn pane_state(
    pane: &Entity<crate::Pane>,
    cx: &VisualTestContext,
) -> (Vec<EntityId>, Option<EntityId>, usize, Option<EntityId>) {
    pane.read_with(cx, |pane, _| {
        (
            pane.items().map(|item| item.item_id()).collect(),
            pane.active_item().map(|item| item.item_id()),
            pane.pinned_count(),
            pane.preview_item_id(),
        )
    })
}

async fn assert_retry_preserves_duplicate_owner(active: bool, cx: &mut TestAppContext) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let failed = add_failed_item(&workspace, saved.id, 2, cx);
    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    let project_item = cx.update(|_, cx| TestProjectItem::new(1, "a.rs", cx));
    let existing = cx.new(|cx| {
        TestItem::new(cx)
            .with_project_items(std::slice::from_ref(&project_item))
            .with_dirty(true)
    });
    let restored = cx.new(|cx| {
        TestItem::new_deserialized(saved.id, cx)
            .with_project_items(&[project_item])
            .with_dirty(true)
    });
    existing.update(cx, |item, _| item.state = String::from("current edits"));
    restored.update(cx, |item, _| item.state = String::from("recovered edits"));
    pane.update_in(cx, |pane, window, cx| {
        pane.add_item(Box::new(existing.clone()), true, true, None, window, cx);
        pane.set_pinned_count(1);
        if active {
            pane.activate_item(0, true, true, window, cx);
        }
    });
    let (entered, release) = install_item_retry_gate(restored.clone(), cx);
    failed.update_in(cx, |failed, window, cx| {
        failed.retry(window, cx);
        failed.retry(window, cx);
    });
    entered.await.expect("retry entered once");
    release.send(()).expect("release retry");
    cx.run_until_parked();
    assert_eq!(
        pane_state(&pane, cx),
        (
            vec![restored.entity_id(), existing.entity_id()],
            Some(if active {
                restored.entity_id()
            } else {
                existing.entity_id()
            }),
            1,
            None
        )
    );
    cx.update(|_, cx| {
        assert_eq!(existing.read(cx).state, "current edits");
        assert_eq!(restored.read(cx).state, "recovered edits");
        assert_eq!(cx.global::<ItemRetryGate>().calls, 1);
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    let existing_id = workspace.update(cx, |workspace, cx| {
        workspace
            .serialization_id("TestItem", existing.entity_id(), cx)
            .expect("existing ID")
    });
    let graph = database.workspace_for_id(saved.id).expect("retry graph");
    assert_eq!(
        graph.center_group,
        SerializedPaneGroup::Pane(SerializedPane::new(
            vec![
                SerializedItem::new("TestItem", 2, active, false),
                SerializedItem::new("TestItem", existing_id, !active, false),
            ],
            true,
            1
        ))
    );
}

struct ItemRetryGate {
    entered: Option<oneshot::Sender<()>>,
    receiver: Option<oneshot::Receiver<()>>,
    item: Entity<TestItem>,
    calls: usize,
}

impl Global for ItemRetryGate {}

fn install_item_retry_gate(
    item: Entity<TestItem>,
    cx: &mut VisualTestContext,
) -> (oneshot::Receiver<()>, oneshot::Sender<()>) {
    let (sender, entered) = oneshot::channel();
    let (release, receiver) = oneshot::channel();
    cx.update(|_, cx| {
        cx.set_global(ItemRetryGate {
            entered: Some(sender),
            receiver: Some(receiver),
            item,
            calls: 0,
        });
        cx.global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor")
            .deserialize = |_, _, _, _, _, cx| {
            let gate = cx.global_mut::<ItemRetryGate>();
            gate.calls += 1;
            gate.entered
                .take()
                .expect("single call")
                .send(())
                .expect("listener");
            let receiver = gate.receiver.take().expect("receiver");
            let item = gate.item.clone();
            cx.foreground_executor().spawn(async move {
                receiver.await?;
                Ok(Box::new(item) as Box<dyn ItemHandle>)
            })
        };
    });
    (entered, release)
}

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

fn failed_preview_pane_group(first_item_id: ItemId, second_item_id: ItemId) -> SerializedPaneGroup {
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
                vec![SerializedItem::new("TestItem", second_item_id, true, false)],
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

async fn assert_hot_exit_graph_failure_prompts(
    close_intent: CloseIntent,
    answer: &str,
    cx: &mut TestAppContext,
) {
    let (workspace, database, saved, cx) = restore_fixture(cx).await;
    let workspace_id = saved.id.0;
    let live_binding = workspace.read_with(cx, |workspace, _| {
        (
            workspace.session_id.clone(),
            workspace.serialized_window_id.map(|id| id.as_u64()),
        )
    });
    database
        .set_session_binding(saved.id, live_binding.0.clone(), live_binding.1)
        .await
        .expect("bind seeded hot-exit graph");
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
    {
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
        } else {
            assert!(decision.expect("accept close after explicit save or discard"));
        }
        let (_, session_id, window_id) = session_bindings(&database)
            .into_iter()
            .find(|binding| binding.0 == saved.id)
            .expect("retained graph");
        let binding = (session_id, window_id);
        if answer != "Cancel" && close_intent == CloseIntent::CloseWindow {
            assert_eq!(binding, (None, None));
        } else {
            assert_eq!(binding, live_binding);
        }
        assert!(window.read_with(cx, |_, _| ()).is_ok());
    }
    item.read_with(cx, |item, _| {
        assert_eq!(item.state, "new dirty untitled text absent from old graph");
        assert_eq!(item.save_as_count, usize::from(answer == "Save"));
        assert_eq!(item.is_dirty, answer != "Save");
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
    let app_state = workspace.read_with(cx, |workspace, _| workspace.app_state().clone());
    let reuse = cx
        .update(|_, cx| crate::open_workspace_by_id(saved.id, app_state, None, cx))
        .await;
    assert_eq!(
        reuse
            .expect_err("failed publication cannot report successful reuse")
            .to_string(),
        format!("{error:#}")
    );
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
    assert!(pending_id > 2);
    drop(pending_item);
    release.send(()).expect("release second pane");
    cleanup_entered.await.expect("cleanup entered");
    cx.update(|_, cx| {
        let gate = cx.global_mut::<CleanupGate>();
        gate.item_ids.sort_unstable();
        assert_eq!(gate.item_ids, vec![1, 2]);
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
    saved.center_group = SerializedPaneGroup::Group {
        axis: SerializedAxis(Axis::Horizontal),
        flexes: Some(vec![1.0, 1.0, 1.0]),
        children: (1..=3)
            .map(|item_id| {
                SerializedPaneGroup::Pane(SerializedPane::new(
                    vec![SerializedItem::new("TestItem", item_id, true, false)],
                    item_id == 2,
                    1,
                ))
            })
            .collect(),
    };
    database
        .try_save_workspace(saved.clone())
        .await
        .expect("saved graph");
    database.write(move |connection| {
        connection.exec("CREATE TABLE restore_test_payloads (workspace_id INTEGER, item_id INTEGER, payload TEXT) STRICT")?()?;
        let mut insert = connection.exec_bound::<(WorkspaceId, ItemId, String)>("INSERT INTO restore_test_payloads VALUES (?, ?, ?)")?;
        for item_id in [1, 2, 3, 99] {
            insert((workspace_id, item_id, format!("saved payload {item_id}")))?;
        }
        Ok::<_, anyhow::Error>(())
    }).await.expect("payloads");
    let expected_payloads = vec![
        (1, "saved payload 1".to_owned()),
        (2, "saved payload 2".to_owned()),
        (3, "saved payload 3".to_owned()),
    ];
    cx.update(|_, cx| {
        cx.set_global(ItemIdProvider {
            ids: if all_fail { vec![1, 2, 3] } else { vec![2] },
            reads: Cell::new(0),
        });
        let descriptor = cx
            .global_mut::<SerializableItemRegistry>()
            .descriptors_by_kind
            .get_mut("TestItem")
            .expect("descriptor");
        descriptor.serialized_item_ids = |workspace_id, cx| {
            WorkspaceDb::global(cx).select_bound::<WorkspaceId, ItemId>(
                "SELECT item_id FROM restore_test_payloads WHERE workspace_id = ?",
            )?(workspace_id)
        };
        descriptor.deserialize = |_, _, workspace_id, item_id, _, cx| {
            if cx.global::<ItemIdProvider>().ids.contains(&item_id) {
                Task::ready(Err(anyhow!("injected failure {item_id}")))
            } else {
                Task::ready(Ok(
                    Box::new(cx.new(|cx| TestItem::new_deserialized(workspace_id, cx)))
                        as Box<dyn ItemHandle>,
                ))
            }
        };
        descriptor.cleanup = |workspace_id, ids, _, cx| {
            crate::delete_unloaded_items(
                ids,
                workspace_id,
                "restore_test_payloads",
                &WorkspaceDb::global(cx),
                cx,
            )
        };
    });
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(saved.clone(), Vec::new(), window, cx)
        })
        .await
        .expect("partial restoration");
    workspace.read_with(cx, |workspace, cx| {
        assert!(!workspace.is_restoring());
        assert_eq!(
            workspace
                .panes
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .collect::<Vec<_>>(),
            vec![1, 1, 1]
        );
        assert_eq!(
            workspace
                .items(cx)
                .filter(|item| item
                    .downcast::<crate::invalid_item_view::InvalidItemView>()
                    .is_some())
                .count(),
            if all_fail { 3 } else { 1 }
        );
    });
    workspace
        .read_with(cx, |workspace, _| workspace.wait_for_restoration())
        .await
        .expect("joined completion");
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.flush_serialization(window, cx)
        })
        .await;
    assert_eq!(
        database
            .workspace_for_id(workspace_id)
            .expect("flushed graph")
            .center_group,
        saved.center_group
    );
    assert_eq!(restore_payloads(&database, workspace_id), expected_payloads);
    let restarted = workspace.update_in(cx, |workspace, window, cx| {
        let project = workspace.project().clone();
        let app_state = workspace.app_state().clone();
        cx.new(|cx| Workspace::new(Some(workspace_id), project, app_state, window, cx))
    });
    restarted
        .update_in(cx, |workspace, window, cx| {
            workspace.load_workspace(
                database
                    .workspace_for_id(workspace_id)
                    .expect("saved restart"),
                Vec::new(),
                window,
                cx,
            )
        })
        .await
        .expect("restart with failures");
    assert_eq!(
        database
            .workspace_for_id(workspace_id)
            .expect("restart graph")
            .center_group,
        saved.center_group
    );
    assert_eq!(restore_payloads(&database, workspace_id), expected_payloads);
    let failed = restarted.read_with(cx, |workspace, cx| {
        workspace.panes[1]
            .read(cx)
            .item_for_index(0)
            .expect("failed tab")
            .downcast::<crate::invalid_item_view::InvalidItemView>()
            .expect("visible failure")
    });
    if all_fail {
        let pane = restarted.read_with(cx, |workspace, _| workspace.panes[1].clone());
        let closing = pane.update_in(cx, |pane, window, cx| {
            pane.close_item_by_id(failed.entity_id(), crate::SaveIntent::Close, window, cx)
        });
        cx.run_until_parked();
        cx.simulate_prompt_answer("Discard");
        closing.await.expect("explicitly discard failed tab");
        restarted
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
        assert_eq!(
            database
                .serialized_item_ids(workspace_id, "TestItem")
                .expect("remaining references")
                .into_iter()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 3])
        );
    } else {
        cx.update(|_, cx| cx.global_mut::<ItemIdProvider>().ids.clear());
        let destination = restarted.read_with(cx, |workspace, _| workspace.panes[0].clone());
        let source = restarted.read_with(cx, |workspace, _| workspace.panes[1].clone());
        let other = restarted.update_in(cx, |workspace, window, cx| {
            let project = workspace.project().clone();
            let app_state = workspace.app_state().clone();
            cx.new(|cx| Workspace::new(None, project, app_state, window, cx))
        });
        let other_pane = other.read_with(cx, |workspace, _| workspace.active_pane().clone());
        cx.update(|window, cx| {
            crate::move_item(
                &source,
                &other_pane,
                failed.entity_id(),
                0,
                true,
                window,
                cx,
            )
        });
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("OK");
        assert_eq!(source.read_with(cx, |pane, _| pane.items_len()), 1);
        assert_eq!(other_pane.read_with(cx, |pane, _| pane.items_len()), 0);
        cx.update(|window, cx| {
            crate::move_item(
                &source,
                &destination,
                failed.entity_id(),
                1,
                true,
                window,
                cx,
            )
        });
        failed.update_in(cx, |failed, window, cx| {
            failed.retry(window, cx);
            failed.retry(window, cx);
        });
        cx.run_until_parked();
        restarted
            .update_in(cx, |workspace, window, cx| {
                workspace.flush_serialization(window, cx)
            })
            .await;
        restarted.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.items(cx).count(), 3);
            assert_eq!(
                workspace
                    .items(cx)
                    .filter(|item| item
                        .downcast::<crate::invalid_item_view::InvalidItemView>()
                        .is_some())
                    .count(),
                0
            );
            let ids = workspace
                .assigned_serialized_item_ids("TestItem")
                .into_iter()
                .collect::<BTreeSet<_>>();
            assert_eq!(ids, BTreeSet::from([1, 2, 3]));
        });
        assert_eq!(
            database
                .serialized_item_ids(workspace_id, "TestItem")
                .expect("retry graph")
                .into_iter()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([1, 2, 3])
        );
        assert_eq!(restore_payloads(&database, workspace_id), expected_payloads);
    }
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
        MultiWorkspace::test_from_workspace(workspace, window, cx)
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
