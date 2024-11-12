use super::*;
use editor::Editor;
use gpui::{TestAppContext, VisualTestContext};
use menu::SelectPrev;
use project::{Project, ProjectPath};
use serde_json::json;
use std::path::Path;
use workspace::{AppState, Workspace};

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
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
            "/root",
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
                "4.txt": "Fourth file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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

    cx.dispatch_action(SelectPrev);
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
            "/root",
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
                "3.txt": "Third file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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
            "/root",
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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
        .insert_tree("/root", json!({"1.txt": "Single file"}))
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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
            "/root",
            json!({
                "1.txt": "First file",
                "2.txt": "Second file",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
    let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project.clone(), cx));

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

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        language::init(cx);
        super::init(cx);
        editor::init(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        state
    })
}

#[track_caller]
fn open_tab_switcher(
    select_last: bool,
    workspace: &View<Workspace>,
    cx: &mut VisualTestContext,
) -> View<Picker<TabSwitcherDelegate>> {
    cx.dispatch_action(Toggle { select_last });
    get_active_tab_switcher(workspace, cx)
}

#[track_caller]
fn get_active_tab_switcher(
    workspace: &View<Workspace>,
    cx: &mut VisualTestContext,
) -> View<Picker<TabSwitcherDelegate>> {
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
    workspace: &View<Workspace>,
    cx: &mut gpui::VisualTestContext,
) -> Box<dyn ItemHandle> {
    let project = workspace.update(cx, |workspace, _| workspace.project().clone());
    let worktree_id = project.update(cx, |project, cx| {
        let worktree = project.worktrees(cx).last().expect("worktree not found");
        worktree.read(cx).id()
    });
    let project_path = ProjectPath {
        worktree_id,
        path: Arc::from(Path::new(file_path)),
    };
    workspace
        .update(cx, move |workspace, cx| {
            workspace.open_path(project_path, None, true, cx)
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
fn assert_tab_switcher_is_closed(workspace: View<Workspace>, cx: &mut VisualTestContext) {
    workspace.update(cx, |workspace, cx| {
        assert!(
            workspace.active_modal::<TabSwitcher>(cx).is_none(),
            "tab switcher is still open"
        );
    });
}
