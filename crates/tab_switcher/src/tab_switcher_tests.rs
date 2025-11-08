use super::*;
use editor::Editor;
use gpui::{TestAppContext, VisualTestContext};
use menu::SelectPrevious;
use project::{Project, ProjectPath};
use serde_json::json;

use util::{path, rel_path::rel_path};
use workspace::{AppState, Workspace};

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

    cx.simulate_modifiers_change(Modifiers::control());
    cx.dispatch_action(CloseSelectedItem);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 1);
        assert_match_selection(tab_switcher, 0, tab_2);
    });

    // Still switches tab on modifiers release
    cx.simulate_modifiers_change(Modifiers::none());
    cx.read(|cx| {
        let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "2.txt");
    });
    assert_tab_switcher_is_closed(workspace, cx);
}

#[gpui::test]
async fn test_close_preserves_selected_position(cx: &mut gpui::TestAppContext) {
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

    let tab_switcher = open_tab_switcher(false, &workspace, cx);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 3);
        assert_match_at_position(tab_switcher, 0, tab_3.boxed_clone());
        assert_match_selection(tab_switcher, 1, tab_2.boxed_clone());
        assert_match_at_position(tab_switcher, 2, tab_1.boxed_clone());
    });

    // Verify that if the selected tab was closed, tab at the same position is selected.
    cx.dispatch_action(CloseSelectedItem);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 2);
        assert_match_at_position(tab_switcher, 0, tab_3.boxed_clone());
        assert_match_selection(tab_switcher, 1, tab_1.boxed_clone());
    });

    // But if the position is no longer valid, fall back to the position above.
    cx.dispatch_action(CloseSelectedItem);
    tab_switcher.update(cx, |tab_switcher, _| {
        assert_eq!(tab_switcher.delegate.matches.len(), 1);
        assert_match_selection(tab_switcher, 0, tab_3.boxed_clone());
    });
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

#[gpui::test]
async fn test_tab_switcher_search_functionality(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "main.rs": "Main Rust file",
                "component.tsx": "React component",
                "readme.md": "Documentation",
                "test_file.js": "JavaScript test",
                "lib_utils.rs": "Utility library",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    // Open files in specific order to test search behavior
    let main_rs = open_buffer("main.rs", &workspace, cx).await;
    let component_tsx = open_buffer("component.tsx", &workspace, cx).await;
    let readme_md = open_buffer("readme.md", &workspace, cx).await;
    let test_js = open_buffer("test_file.js", &workspace, cx).await;
    let lib_rs = open_buffer("lib_utils.rs", &workspace, cx).await;

    let tab_switcher = open_tab_switcher(false, &workspace, cx);

    // Test initial searchable state
    tab_switcher.update_in(cx, |tab_switcher, window, cx| {
        let query = tab_switcher.query(cx);
        assert_eq!(query, "");

        // Should show all tabs initially (5 files)
        assert_eq!(tab_switcher.delegate.matches.len(), 5);

        // Verify placeholder text
        let placeholder = tab_switcher.delegate.placeholder_text(window, cx);
        assert_eq!(placeholder.as_ref(), "Search tabs…");
    });

    // Test search for Rust files
    test_search_query(
        &tab_switcher,
        "rs",
        &[main_rs.as_ref(), lib_rs.as_ref()],
        cx,
    )
    .await;

    // Test search for specific filename
    test_search_query(&tab_switcher, "component", &[component_tsx.as_ref()], cx).await;

    // Test search with no results
    test_search_query(&tab_switcher, "nonexistent", &[], cx).await;

    // Test search that matches multiple files
    test_search_query(
        &tab_switcher,
        "e",
        &[readme_md.as_ref(), test_js.as_ref(), component_tsx.as_ref()],
        cx,
    )
    .await;

    // Test clearing search returns all results
    test_search_query(
        &tab_switcher,
        "",
        &[
            lib_rs.as_ref(),
            test_js.as_ref(),
            readme_md.as_ref(),
            component_tsx.as_ref(),
            main_rs.as_ref(),
        ],
        cx,
    )
    .await;
}

async fn test_search_query(
    tab_switcher: &Entity<Picker<TabSwitcherDelegate>>,
    query: &str,
    expected_matches: &[&dyn ItemHandle],
    cx: &mut VisualTestContext,
) {
    // Set the search query
    tab_switcher.update_in(cx, |tab_switcher, window, cx| {
        tab_switcher.set_query(query, window, cx);
    });

    // Allow time for updates to process
    cx.executor().run_until_parked();

    // Verify results
    tab_switcher.update(cx, |tab_switcher, cx| {
        let query_result = tab_switcher.query(cx);
        assert_eq!(query_result, query, "Query should be set correctly");

        assert_eq!(
            tab_switcher.delegate.matches.len(),
            expected_matches.len(),
            "Should have {} matches for query '{}'",
            expected_matches.len(),
            query
        );

        // Verify all expected items are present (order may vary due to scoring)
        for expected_item in expected_matches {
            let found = tab_switcher
                .delegate
                .matches
                .iter()
                .any(|match_item| match_item.item.item_id() == expected_item.item_id());
            assert!(
                found,
                "Expected item should be found in search results for query '{}'",
                query
            );
        }
    });
}

#[gpui::test]
async fn test_global_tab_switcher_placeholder(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);

    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({"test.txt": "content"}))
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (workspace, cx) =
        cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

    let _tab = open_buffer("test.txt", &workspace, cx).await;

    // Test global tab switcher (all panes mode)
    cx.dispatch_action(ToggleAll);

    let tab_switcher = get_active_tab_switcher(&workspace, cx);
    tab_switcher.update_in(cx, |tab_switcher, window, cx| {
        let placeholder = tab_switcher.delegate.placeholder_text(window, cx);
        assert_eq!(placeholder.as_ref(), "Search all tabs…");
    });
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
