use super::*;
use crate::file_finder_tests::init_test;
use editor::Editor;
use gpui::{Entity, VisualTestContext};
use menu::Confirm;
use project::{Project, ProjectPath};
use serde_json::json;
use util::{path, rel_path::rel_path};
use workspace::{CloseActiveItem, MultiWorkspace, Workspace};

/// Opens `file_name` in the active pane and waits for the workspace to settle.
async fn open_file(file_name: &str, workspace: &Entity<Workspace>, cx: &mut VisualTestContext) {
    workspace
        .update_in(cx, |workspace, window, cx| {
            let worktree_id = workspace.worktrees(cx).next().unwrap().read(cx).id();
            workspace.open_path(
                ProjectPath {
                    worktree_id,
                    path: rel_path(file_name).into(),
                },
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();
}

/// Closes the active item. Zed's navigation history only records an item once
/// the pane moves away from it, so tests close each file before opening the
/// next one to get a predictable history order (matches the pattern used by
/// `file_finder_tests::open_close_queried_buffer`).
fn close_active_item(cx: &mut VisualTestContext) {
    cx.dispatch_action(CloseActiveItem {
        save_intent: None,
        close_pinned: false,
    });
    cx.run_until_parked();
}

fn open_recent_files(cx: &mut VisualTestContext) {
    cx.dispatch_action(Toggle);
    cx.run_until_parked();
}

fn active_recent_files_picker(
    workspace: &Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<RecentFilesDelegate>> {
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<RecentFiles>(cx)
            .expect("recent files palette is not open")
            .read(cx)
            .picker
            .clone()
    })
}

fn matched_file_names(
    picker: &Entity<Picker<RecentFilesDelegate>>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    picker.read_with(cx, |picker, _| {
        picker
            .delegate
            .matches
            .iter()
            .filter_map(|m| picker.delegate.entries.get(m.entry_index))
            .map(|entry| entry.file_name.to_string())
            .collect()
    })
}

#[gpui::test]
async fn test_recent_files_shows_history_in_recency_order(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "first.rs": "// First Rust file",
                "second.rs": "// Second Rust file",
                "third.rs": "// Third Rust file",
            }),
        )
        .await;
    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    workspace.update_in(cx, |_workspace, window, cx| window.focused(cx));

    open_file("first.rs", &workspace, cx).await;
    close_active_item(cx);
    open_file("second.rs", &workspace, cx).await;
    close_active_item(cx);
    // third.rs stays open, mimicking "I'm in this file, show me my other recents".
    open_file("third.rs", &workspace, cx).await;

    open_recent_files(cx);
    let picker = active_recent_files_picker(&workspace, cx);
    assert_eq!(
        matched_file_names(&picker, cx),
        vec![
            "third.rs".to_string(),
            "second.rs".to_string(),
            "first.rs".to_string()
        ],
        "Recent files should list navigation history newest-first, including the \
         currently open file"
    );
    picker.read_with(cx, |picker, _| {
        assert_eq!(
            picker.delegate.selected_index, 1,
            "Selection should skip the currently open file (index 0) so pressing \
             Enter immediately switches away from it"
        );
    });
}

#[gpui::test]
async fn test_recent_files_filters_by_substring(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "first.rs": "// First Rust file",
                "second.rs": "// Second Rust file",
                "third.rs": "// Third Rust file",
            }),
        )
        .await;
    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    workspace.update_in(cx, |_workspace, window, cx| window.focused(cx));

    open_file("first.rs", &workspace, cx).await;
    close_active_item(cx);
    open_file("second.rs", &workspace, cx).await;
    close_active_item(cx);
    open_file("third.rs", &workspace, cx).await;

    open_recent_files(cx);
    let picker = active_recent_files_picker(&workspace, cx);
    cx.simulate_input("sec");
    cx.run_until_parked();

    assert_eq!(
        matched_file_names(&picker, cx),
        vec!["second.rs".to_string()],
        "Typing a substring should filter the recent files list to matching file names"
    );
}

#[gpui::test]
async fn test_recent_files_confirm_opens_file(cx: &mut gpui::TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/src"),
            json!({
                "first.rs": "// First Rust file",
                "second.rs": "// Second Rust file",
            }),
        )
        .await;
    let project = Project::test(app_state.fs.clone(), [path!("/src").as_ref()], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    workspace.update_in(cx, |_workspace, window, cx| window.focused(cx));

    open_file("first.rs", &workspace, cx).await;
    close_active_item(cx);
    open_file("second.rs", &workspace, cx).await;

    open_recent_files(cx);
    // second.rs is the currently open file, so selection should skip it and
    // land on first.rs; confirming should switch to first.rs.
    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    workspace.read_with(cx, |workspace, cx| {
        let active_editor = workspace.active_item_as::<Editor>(cx).unwrap();
        assert_eq!(active_editor.read(cx).title(cx), "first.rs");
    });
}
