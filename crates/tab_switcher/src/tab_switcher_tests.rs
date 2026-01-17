use super::*;
use editor::Editor;
use gpui::{TestAppContext, VisualTestContext};
use menu::SelectPrevious;
use project::{Project, ProjectPath};
use serde_json::json;
use util::{path, rel_path::rel_path};
use workspace::{
    ActivatePreviousItem, AppState, Workspace,
    item::test::TestItem,
};

#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
async fn test_open_with_prev_tab_selected_and_cycle_on_toggle_action(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
                "4.txt": "Fourth file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let tab_2 = open_buffer("2.txt", &workspace, cx).await;
    let tab_3 = open_buffer("3.txt", &workspace, cx).await;
    let tab_4 = open_buffer("4.txt", &workspace, cx).await;

    // Starts with the previously opened item selected
    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 4);
        assert_match_at_position(tab_switcher, 0, tab_4.boxed_clone());
        assert_match_selection(tab_switcher, 1, tab_3.boxed_clone());
        assert_match_at_position(tab_switcher, 2, tab_2.boxed_clone());
        assert_match_at_position(tab_switcher, 3, tab_1.boxed_clone());
    });

    cx.dispatch_action(Toggle { select_last: false });
    cx.dispatch_action(Toggle { select_last: false });
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 4);
        assert_match_at_position(tab_switcher, 0, tab_4.boxed_clone());
        assert_match_at_position(tab_switcher, 1, tab_3.boxed_clone());
        assert_match_at_position(tab_switcher, 2, tab_2.boxed_clone());
        assert_match_selection(tab_switcher, 3, tab_1.boxed_clone());
    });

    cx.dispatch_action(SelectPrevious);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 4);
        assert_match_at_position(tab_switcher, 0, tab_4.boxed_clone());
        assert_match_at_position(tab_switcher, 1, tab_3.boxed_clone());
        assert_match_selection(tab_switcher, 2, tab_2.boxed_clone());
        assert_match_at_position(tab_switcher, 3, tab_1.boxed_clone());
    });
}

#[gpui::test]
async fn test_open_with_last_tab_selected(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let tab_2 = open_buffer("2.txt", &workspace, cx).await;
    let tab_3 = open_buffer("3.txt", &workspace, cx).await;

    // Starts with the last item selected
    let tab_switcher = open_tab_switcher(true, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 3);
        assert_match_at_position(tab_switcher, 0, tab_3);
        assert_match_at_position(tab_switcher, 1, tab_2);
        assert_match_selection(tab_switcher, 2, tab_1);
    });
}

#[gpui::test]
async fn test_open_item_on_modifiers_release(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let tab_2 = open_buffer("2.txt", &workspace, cx).await;

    cx.simulate_modifiers_change(Modifiers::control());
    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 2);
        assert_match_at_position(tab_switcher, 0, tab_2.boxed_clone());
        assert_match_selection(tab_switcher, 1, tab_1.boxed_clone());
    });

    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "1.txt");
    });
    assert_tab_switcher_is_closed(workspace, cx);
}

#[gpui::test]
async fn test_open_on_empty_pane(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state.fs.as_fake().insert_tree("/root", json!({})).await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    cx.simulate_modifiers_change(Modifiers::control());
    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert!(tab_switcher.delegate.matches.is_empty());
    });

    cx.simulate_modifiers_change(Modifiers::none());
    assert_tab_switcher_is_closed(workspace, cx);
}

#[gpui::test]
async fn test_open_with_single_item(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"1.txt": "Single file"}))
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let tab = open_buffer("1.txt", &workspace, cx).await;

    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 1);
        assert_match_selection(tab_switcher, 0, tab);
    });
}

#[gpui::test]
async fn test_close_selected_item(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
                "4.txt": "Fourth file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let tab_3 = open_buffer("3.txt", &workspace, cx).await;
    let tab_2 = open_buffer("2.txt", &workspace, cx).await;
    let tab_4 = open_buffer("4.txt", &workspace, cx).await;

    // After opening all buffers, let's navigate to the previous item two times, finishing with:
    //
    // 1.txt | [3.txt] | 2.txt | 4.txt
    //
    // With 3.txt being the active item in the pane.
    cx.dispatch_action(ActivatePreviousItem);
    cx.dispatch_action(ActivatePreviousItem);
    cx.run_until_parked();

    cx.simulate_modifiers_change(Modifiers::control());
    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 4);
        assert_match_at_position(tab_switcher, 0, tab_3.boxed_clone());
        assert_match_selection(tab_switcher, 1, tab_2.boxed_clone());
        assert_match_at_position(tab_switcher, 2, tab_4.boxed_clone());
        assert_match_at_position(tab_switcher, 3, tab_1.boxed_clone());
    });

    cx.simulate_modifiers_change(Modifiers::control());
    cx.dispatch_action(CloseSelectedItem);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 3);
        assert_match_selection(tab_switcher, 0, tab_3);
        assert_match_at_position(tab_switcher, 1, tab_4);
        assert_match_at_position(tab_switcher, 2, tab_1);
    });

    // Still switches tab on modifiers release
    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "3.txt");
    });
    assert_tab_switcher_is_closed(workspace, cx);
}

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        super::init(cx);
        editor::init(cx);
        state
    })
}

#[track_caller]
fn open_tab_switcher(
    select_last: bool,
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<TabSwitcherDelegate>> {
    cx.dispatch_action(Toggle { select_last });
    get_active_tab_switcher(workspace, cx)
}

#[track_caller]
fn get_active_tab_switcher(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<TabSwitcherDelegate>> {
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<TabSwitcher>(cx)
            .expect("tab switcher is not open")
            .read(cx)
            .picker
            .clone()
    })
}

async fn open_buffer(
    file_path: &str,
    workspace: &Entity<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Box<dyn ItemHandle> {
    let project = workspace.read_with(cx, |workspace, _| workspace.project().clone());
    let worktree_id = project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).last().expect("worktree not found");
        worktree.read(cx).id()
    });
    let project_path = ProjectPath {
        worktree_id,
        path: rel_path(file_path).into(),
    };
    workspace
        .update_in(cx, move |workspace, window, cx| {
            workspace.open_path(project_path, None, true, window, cx)
        })
        .await
        .unwrap()
}

#[track_caller]
fn assert_match_selection(
    tab_switcher: &Picker<TabSwitcherDelegate>,
    expected_selection_index: usize,
    expected_item: Box<dyn ItemHandle>,
) {
    assert_eq!(
        tab_switcher.delegate.selected_index(),
        expected_selection_index,
        "item is not selected"
    );
    assert_match_at_position(tab_switcher, expected_selection_index, expected_item);
}

#[track_caller]
fn assert_match_at_position(
    tab_switcher: &Picker<TabSwitcherDelegate>,
    match_index: usize,
    expected_item: Box<dyn ItemHandle>,
) {
    let match_item = tab_switcher
        .delegate
        .matches
        .get(match_index)
        .unwrap_or_else(|| panic!("Tab Switcher has no match for index {match_index}"));
    assert_eq!(match_item.item.item_id(), expected_item.item_id());
}

#[track_caller]
fn assert_tab_switcher_is_closed(workspace: Entity<Workspace>, cx: &mut VisualTestContext) {
    workspace.update(cx, |workspace, cx| {
        assert!(
            workspace.active_modal::<TabSwitcher>(cx).is_none(),
            "tab switcher is still open"
        );
    });
}

#[track_caller]
fn open_tab_switcher_follow_mode(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<TabSwitcherDelegate>> {
    cx.dispatch_action(ToggleAll { follow_mode: true });
    get_active_tab_switcher(workspace, cx)
}

#[gpui::test]
async fn test_toggle_all_follow_mode_dedupes_by_path(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open files in first pane
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    // Split and open same file in second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });
    let _tab_1_in_pane2 = open_buffer("1.txt", &workspace, cx).await;

    // ToggleAll with follow_mode should dedupe - show only 2 items, not 3
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(
            tab_switcher.delegate.matches.len(),
            2,
            "should dedupe same file across panes"
        );
    });
}

#[gpui::test]
async fn test_toggle_all_follow_mode_previews_in_active_pane(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    // 2.txt is active. MRU order is [2.txt, 1.txt]
    let initial_active = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item(cx).map(|i| i.item_id())
    });

    // Open picker - it selects index 1 (1.txt) by default and previews it
    let _tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    cx.run_until_parked();

    // The active item should have changed to 1.txt (preview)
    let previewed_active = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item(cx).map(|i| i.item_id())
    });
    assert_ne!(
        initial_active, previewed_active,
        "active item should change when picker opens (preview in active pane)"
    );

    // Dismiss the picker
    cx.dispatch_action(menu::Cancel);
    cx.run_until_parked();

    // The active item should be restored to the original (2.txt)
    let restored_active = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item(cx).map(|i| i.item_id())
    });
    assert_eq!(
        initial_active, restored_active,
        "active item should be restored after dismiss"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_opens_in_current_pane(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open file in first pane
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;

    // Split and open different file in second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    // Now we're in pane 2. Get a reference to it.
    let current_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    // Open tab switcher and select 1.txt (which is in pane 1)
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);

    // Find and select 1.txt
    tab_switcher.update(cx, |picker, cx| {
        for (i, m) in picker.delegate.matches.iter().enumerate() {
            if m.item.tab_content_text(0, cx).contains("1.txt") {
                picker.delegate.selected_index = i;
                break;
            }
        }
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // The file should now be open in the current pane (pane 2), not pane 1
    current_pane.read_with(cx, |pane, cx| {
        let active_item = pane.active_item().expect("pane should have active item");
        assert!(
            active_item.tab_content_text(0, cx).contains("1.txt"),
            "1.txt should be open in the current pane"
        );
    });
}

#[gpui::test]
async fn test_toggle_all_follow_mode_close_removes_from_all_panes(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open file in first pane
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    // Split and open same file in second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });
    let _tab_1_pane2 = open_buffer("1.txt", &workspace, cx).await;

    // Verify 1.txt is in both panes
    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    assert_eq!(panes.len(), 2);

    let count_1txt_before: usize = panes
        .iter()
        .map(|pane| {
            pane.read_with(cx, |pane, cx| {
                pane.items()
                    .filter(|item| item.tab_content_text(0, cx).contains("1.txt"))
                    .count()
            })
        })
        .sum();
    assert_eq!(count_1txt_before, 2, "1.txt should be in both panes");

    // Open follow mode tab switcher
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);

    // Find and select 1.txt
    let idx = tab_switcher.read_with(cx, |picker, cx| {
        picker
            .delegate
            .matches
            .iter()
            .position(|m| m.item.tab_content_text(0, cx).contains("1.txt"))
            .unwrap()
    });

    tab_switcher.update(cx, |picker, _| {
        picker.delegate.selected_index = idx;
    });

    // Close the selected item
    cx.dispatch_action(CloseSelectedItem);
    cx.run_until_parked();

    // 1.txt should be closed in ALL panes
    let count_1txt_after: usize = panes
        .iter()
        .map(|pane| {
            pane.read_with(cx, |pane, cx| {
                pane.items()
                    .filter(|item| item.tab_content_text(0, cx).contains("1.txt"))
                    .count()
            })
        })
        .sum();
    assert_eq!(count_1txt_after, 0, "1.txt should be closed in all panes");

    // Verify item is removed from picker matches
    let matches_count_after = tab_switcher.read_with(cx, |picker, _cx| {
        picker.delegate.matches.len()
    });
    assert_eq!(matches_count_after, 1, "picker should have 1 item after closing one of two");
}

#[gpui::test]
async fn test_toggle_all_follow_mode_close_preserves_list_order(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open files in MRU order: 1, 2, 3
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;
    let _tab_3 = open_buffer("3.txt", &workspace, cx).await;

    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);

    // Capture initial order
    let initial_order = tab_switcher.read_with(cx, |picker, cx| {
        picker
            .delegate
            .matches
            .iter()
            .map(|m| m.item.tab_content_text(0, cx))
            .collect::<Vec<_>>()
    });

    assert_eq!(initial_order.len(), 3);

    // Close the middle item (index 1)
    tab_switcher.update(cx, |picker, _| {
        picker.delegate.selected_index = 1;
    });
    cx.dispatch_action(CloseSelectedItem);
    cx.run_until_parked();

    // Verify list has 2 items and order is preserved (not re-sorted)
    let final_order = tab_switcher.read_with(cx, |picker, cx| {
        picker
            .delegate
            .matches
            .iter()
            .map(|m| m.item.tab_content_text(0, cx))
            .collect::<Vec<_>>()
    });

    assert_eq!(final_order.len(), 2, "should have 2 items after closing 1");
    assert_eq!(
        final_order[0], initial_order[0],
        "first item should remain in same position"
    );
    assert_eq!(
        final_order[1], initial_order[2],
        "third item should now be second (middle item removed)"
    );

    // Verify selected index adjusted correctly (stayed at 1, which is now the last item)
    let selected_index = tab_switcher.read_with(cx, |picker, _cx| {
        picker.delegate.selected_index
    });
    assert_eq!(selected_index, 1, "selected index should be 1 (last item)");
}

#[gpui::test]
async fn test_toggle_all_follow_mode_close_non_file_item_updates_picker(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Create two non-file items
    let test_item_1 = cx.new(|cx| TestItem::new(cx).with_label("non-file-1"));
    let test_item_2 = cx.new(|cx| TestItem::new(cx).with_label("non-file-2"));

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(test_item_1.clone()), None, true, window, cx);
        workspace.add_item_to_active_pane(Box::new(test_item_2.clone()), None, true, window, cx);
    });

    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);

    // Verify we have 2 items, MRU sorted (test_item_2 is most recent, so at index 0)
    let initial_items = tab_switcher.read_with(cx, |picker, _cx| {
        picker
            .delegate
            .matches
            .iter()
            .map(|m| m.item.item_id())
            .collect::<Vec<_>>()
    });
    assert_eq!(initial_items.len(), 2);
    assert_eq!(initial_items[0], test_item_2.item_id(), "most recent should be first");

    // Close the first item (test_item_2)
    tab_switcher.update(cx, |picker, _| {
        picker.delegate.selected_index = 0;
    });
    cx.dispatch_action(CloseSelectedItem);
    cx.run_until_parked();

    // Verify picker list updated
    let count_after = tab_switcher.read_with(cx, |picker, _cx| picker.delegate.matches.len());
    assert_eq!(count_after, 1, "picker should show 1 item after closing");

    // Verify the remaining item is test_item_1
    let remaining_item = tab_switcher.read_with(cx, |picker, _cx| {
        picker.delegate.matches.first().unwrap().item.item_id()
    });
    assert_eq!(remaining_item, test_item_1.item_id());
}

#[gpui::test]
async fn test_toggle_all_follow_mode_single_pane_non_file_item_activates(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"file.txt": "content"}))
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open a file
    let _file = open_buffer("file.txt", &workspace, cx).await;

    // Add a non-file item (terminal-like)
    let test_item = cx.new(|cx| TestItem::new(cx).with_label("terminal"));
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(test_item.clone()), None, true, window, cx);
    });

    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    // File is now active (most recent)
    let _file2 = open_buffer("file.txt", &workspace, cx).await;

    // Open picker and select the non-file item
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    let item_index = tab_switcher.read_with(cx, |picker, _cx| {
        picker
            .delegate
            .matches
            .iter()
            .position(|m| m.item.item_id() == test_item.item_id())
            .expect("test item should be in matches")
    });

    tab_switcher.update_in(cx, |picker, window, cx| {
        picker.delegate.set_selected_index(item_index, window, cx);
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Verify item is still in pane and is now active
    let still_in_pane = pane.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });
    assert!(
        still_in_pane,
        "non-file item should still be in pane (not removed)"
    );

    let is_active = pane.read_with(cx, |pane, _| {
        pane.active_item()
            .map(|item| item.item_id() == test_item.item_id())
            .unwrap_or(false)
    });
    assert!(is_active, "non-file item should be active after confirm");
}

#[gpui::test]
async fn test_toggle_all_follow_mode_creates_independent_editors(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open file in first pane
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;

    // Split and open different file in second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    let pane_1 = panes[0].clone();
    let pane_2 = panes[1].clone();

    // Open follow mode picker in pane 2 and select 1.txt from pane 1
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    tab_switcher.update(cx, |picker, cx| {
        for (i, m) in picker.delegate.matches.iter().enumerate() {
            if m.item.tab_content_text(0, cx).contains("1.txt") {
                picker.delegate.selected_index = i;
                break;
            }
        }
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Now both panes should have 1.txt open, but as independent editor instances
    // Get the editor from each pane
    let editor_1 = pane_1.read_with(cx, |pane, cx| {
        pane.items()
            .find(|item| item.tab_content_text(0, cx).contains("1.txt"))
            .expect("pane 1 should have 1.txt")
            .act_as::<Editor>(cx)
            .expect("should be an editor")
    });

    let editor_2 = pane_2.read_with(cx, |pane, cx| {
        pane.items()
            .find(|item| item.tab_content_text(0, cx).contains("1.txt"))
            .expect("pane 2 should have 1.txt")
            .act_as::<Editor>(cx)
            .expect("should be an editor")
    });

    // Verify they are different entity instances (not mirrored)
    assert_ne!(
        editor_1.entity_id(),
        editor_2.entity_id(),
        "editors should be independent instances, not the same entity"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_moves_non_file_item_to_current_pane(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Create a non-file item (TestItem with no project_items) in first pane
    let test_item = cx.new(|cx| TestItem::new(cx).with_label("test-terminal"));
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(test_item.clone()), None, true, window, cx);
    });

    // Split and create second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });

    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    assert_eq!(panes.len(), 2);
    let pane_1 = panes[0].clone();
    let pane_2 = panes[1].clone();

    // Verify test item is in pane 1
    let item_in_pane_1_before = pane_1.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });
    assert!(
        item_in_pane_1_before,
        "test item should be in pane 1 initially"
    );

    // Open follow mode picker in pane 2 and select the test item
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    let item_index = tab_switcher.read_with(cx, |picker, _cx| {
        picker
            .delegate
            .matches
            .iter()
            .position(|m| m.item.item_id() == test_item.item_id())
            .expect("test item should be in matches")
    });

    tab_switcher.update_in(cx, |picker, window, cx| {
        picker.delegate.set_selected_index(item_index, window, cx);
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Test item should now be in pane 2 only, not pane 1
    let item_in_pane_1_after = pane_1.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });
    let item_in_pane_2_after = pane_2.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });

    assert!(
        !item_in_pane_1_after,
        "test item should be removed from pane 1"
    );
    assert!(item_in_pane_2_after, "test item should be in pane 2");
}

#[gpui::test]
async fn test_toggle_all_follow_mode_non_file_item_dismiss_restores(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Create a non-file item in first pane
    let test_item = cx.new(|cx| TestItem::new(cx).with_label("test-terminal"));
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(test_item.clone()), None, true, window, cx);
    });

    // Split and create second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });

    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    let pane_1 = panes[0].clone();
    let pane_2 = panes[1].clone();

    // Open follow mode picker in pane 2 and select the test item (this previews it)
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    let item_index = tab_switcher.read_with(cx, |picker, _cx| {
        picker
            .delegate
            .matches
            .iter()
            .position(|m| m.item.item_id() == test_item.item_id())
            .expect("test item should be in matches")
    });

    tab_switcher.update_in(cx, |picker, window, cx| {
        picker.delegate.set_selected_index(item_index, window, cx);
    });
    cx.run_until_parked();

    // During preview, item should be in both panes
    let item_in_pane_1_preview = pane_1.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });
    let item_in_pane_2_preview = pane_2.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });

    assert!(
        item_in_pane_1_preview,
        "test item should still be in pane 1 during preview"
    );
    assert!(
        item_in_pane_2_preview,
        "test item should be previewed in pane 2"
    );

    // Dismiss the picker
    cx.dispatch_action(menu::Cancel);
    cx.run_until_parked();

    // Item should be back to only pane 1
    let item_in_pane_1_after = pane_1.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });
    let item_in_pane_2_after = pane_2.read_with(cx, |pane, _| {
        pane.items()
            .any(|item| item.item_id() == test_item.item_id())
    });

    assert!(
        item_in_pane_1_after,
        "test item should be restored to pane 1"
    );
    assert!(
        !item_in_pane_2_after,
        "test item should be removed from pane 2"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_mru_order_stable_during_navigation(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open files in specific MRU order: 1, 2, 3 (so 3 is most recent)
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;
    let _tab_3 = open_buffer("3.txt", &workspace, cx).await;

    // Open follow mode picker
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);

    // Capture initial order
    let initial_order = tab_switcher.read_with(cx, |picker, cx| {
        picker
            .delegate
            .matches
            .iter()
            .map(|m| m.item.tab_content_text(0, cx))
            .collect::<Vec<_>>()
    });

    // Navigate through the list (this triggers preview and ItemAdded events)
    cx.dispatch_action(menu::SelectNext);
    cx.run_until_parked();
    cx.dispatch_action(menu::SelectNext);
    cx.run_until_parked();
    cx.dispatch_action(menu::SelectPrevious);
    cx.run_until_parked();

    // Verify order hasn't changed despite navigation/preview
    let final_order = tab_switcher.read_with(cx, |picker, cx| {
        picker
            .delegate
            .matches
            .iter()
            .map(|m| m.item.tab_content_text(0, cx))
            .collect::<Vec<_>>()
    });

    assert_eq!(
        initial_order, final_order,
        "list order should remain stable during navigation despite workspace events"
    );

    // Verify MRU order: most recent (3.txt) should be first
    assert!(
        initial_order[0].contains("3.txt"),
        "most recently used file should be first in list"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_focus_stays_in_current_pane(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open file in first pane
    let _tab_1 = open_buffer("1.txt", &workspace, cx).await;

    // Split and open different file in second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });
    let _tab_2 = open_buffer("2.txt", &workspace, cx).await;

    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    let pane_2 = panes[1].clone();

    // Verify we're in pane 2
    let initial_active_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    assert_eq!(initial_active_pane.entity_id(), pane_2.entity_id());

    // Open follow mode picker in pane 2 and select 1.txt from pane 1
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    tab_switcher.update(cx, |picker, cx| {
        for (i, m) in picker.delegate.matches.iter().enumerate() {
            if m.item.tab_content_text(0, cx).contains("1.txt") {
                picker.delegate.selected_index = i;
                break;
            }
        }
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Verify focus stayed in pane 2
    let final_active_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    assert_eq!(
        final_active_pane.entity_id(),
        pane_2.entity_id(),
        "focus should remain in pane 2 (current pane) after confirming file from pane 1"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_preserves_preview_status(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open 1.txt as a preview item
    let project_path = ProjectPath {
        worktree_id: project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        }),
        path: rel_path("1.txt").into(),
    };
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(project_path, None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    // Verify 1.txt is a preview
    let is_preview_before = pane.read_with(cx, |pane, _cx| {
        pane.active_item()
            .map(|item| pane.is_active_preview_item(item.item_id()))
            .unwrap_or(false)
    });
    assert!(is_preview_before, "1.txt should be a preview item initially");

    // Open follow mode tab switcher and select 1.txt
    let _tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Verify 1.txt is still a preview
    let is_preview_after = pane.read_with(cx, |pane, _cx| {
        pane.active_item()
            .map(|item| pane.is_active_preview_item(item.item_id()))
            .unwrap_or(false)
    });
    assert!(
        is_preview_after,
        "1.txt should still be a preview item after tab switcher confirm"
    );
}

#[gpui::test]
async fn test_toggle_preserves_preview_status(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open 1.txt as a preview item
    let project_path = ProjectPath {
        worktree_id: project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        }),
        path: rel_path("1.txt").into(),
    };
    workspace
        .update_in(cx, |workspace, window, cx| {
            workspace.open_path_preview(project_path, None, true, true, true, window, cx)
        })
        .await
        .unwrap();

    let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

    // Verify 1.txt is a preview
    let is_preview_before = pane.read_with(cx, |pane, _cx| {
        pane.active_item()
            .map(|item| pane.is_active_preview_item(item.item_id()))
            .unwrap_or(false)
    });
    assert!(is_preview_before, "1.txt should be a preview item initially");

    // Open regular tab switcher (non-follow mode) and confirm
    let _tab_switcher = open_tab_switcher(false, &workspace, cx);
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Verify 1.txt is still a preview in non-follow mode
    let is_preview_after = pane.read_with(cx, |pane, _cx| {
        pane.active_item()
            .map(|item| pane.is_active_preview_item(item.item_id()))
            .unwrap_or(false)
    });
    assert!(
        is_preview_after,
        "1.txt should still be a preview item after regular tab switcher confirm"
    );
}

#[gpui::test]
async fn test_toggle_all_follow_mode_non_file_item_focus_stays_in_current_pane(
    cx: &mut gpui::TestAppContext,
) {
    let app_state = init_test(cx);

    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Create a non-file item in first pane
    let test_item = cx.new(|cx| TestItem::new(cx).with_label("test-terminal"));
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_item_to_active_pane(Box::new(test_item.clone()), None, true, window, cx);
    });

    // Split and create second pane
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.split_pane(
            workspace.active_pane().clone(),
            workspace::SplitDirection::Right,
            window,
            cx,
        );
    });

    let panes = workspace.read_with(cx, |workspace, _| workspace.panes().to_vec());
    let pane_2 = panes[1].clone();

    // Verify we're in pane 2
    let initial_active_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    assert_eq!(initial_active_pane.entity_id(), pane_2.entity_id());

    // Open follow mode picker in pane 2 and select the test item from pane 1
    let tab_switcher = open_tab_switcher_follow_mode(&workspace, cx);
    let item_index = tab_switcher.read_with(cx, |picker, _cx| {
        picker
            .delegate
            .matches
            .iter()
            .position(|m| m.item.item_id() == test_item.item_id())
            .expect("test item should be in matches")
    });

    tab_switcher.update_in(cx, |picker, window, cx| {
        picker.delegate.set_selected_index(item_index, window, cx);
    });

    // Confirm selection
    cx.dispatch_action(menu::Confirm);
    cx.run_until_parked();

    // Verify focus stayed in pane 2
    let final_active_pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
    assert_eq!(
        final_active_pane.entity_id(),
        pane_2.entity_id(),
        "focus should remain in pane 2 (current pane) after confirming non-file item from pane 1"
    );
}
