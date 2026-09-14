use crate::{
    ItemId, MultiWorkspace, SerializableItemRegistry, SerializedItemIds, Workspace, WorkspaceDb,
    item::test::TestItem,
    persistence::{
        SerializedAxis,
        model::{SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace},
    },
    register_serializable_item,
    tests::init_test,
};
use collections::HashSet;
use fs::FakeFs;
use gpui::{AppContext, Axis, Entity, EntityId, Global, TestAppContext, VisualTestContext};
use project::{
    Project,
    bookmark_store::SerializedBookmark,
    debugger::breakpoint_store::{BreakpointState, SourceBreakpoint},
};
use serde_json::json;
use std::{
    cell::Cell,
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::path;

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
