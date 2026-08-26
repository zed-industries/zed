use super::*;

use editor::Editor;
use gpui::{Entity, TestAppContext, VisualTestContext};
use menu::{Confirm, SecondaryConfirm};
use project::ProjectPath;
use serde_json::json;
use text::Point;
use util::{path, rel_path::rel_path};
use workspace::{AppState, MultiWorkspace};

#[ctor::ctor(unsafe)]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
async fn test_project_bookmarks_picker_matches_and_confirms(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "one.txt": "alpha\nbeta\ngamma\n",
                "two.txt": "delta\nepsilon\nzeta\n",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });

    add_bookmark(&project, worktree_id, "one.txt", 0, "alpha mark", cx).await;
    add_bookmark(&project, worktree_id, "one.txt", 2, "gamma mark", cx).await;
    add_bookmark(&project, worktree_id, "two.txt", 1, "epsilon mark", cx).await;

    let (picker, workspace, cx) = build_bookmarks_picker(project, cx);

    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 3);
    });

    simulate_input(cx, "gamma");
    picker.update(cx, |picker, _| {
        assert_eq!(picker.delegate.matches.len(), 1);
    });

    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    let active_editor = workspace.read_with(cx, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });
    active_editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.target_file_abs_path(cx).unwrap(),
            std::path::PathBuf::from(path!("/root/one.txt"))
        );

        let selection = editor
            .selections
            .newest_adjusted(&editor.display_snapshot(cx));
        assert_eq!(selection.start, selection.end);
        assert_eq!(selection.start, Point::new(2, 0));
    });
}

#[gpui::test]
async fn test_project_bookmarks_confirm_reuses_preview_and_secondary_confirm_opens_new_tabs(
    cx: &mut TestAppContext,
) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a.txt": "alpha\nbeta\ngamma\n",
                "b.txt": "delta\nepsilon\nzeta\n",
                "c.txt": "eta\ntheta\niota\n",
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });

    add_bookmark(&project, worktree_id, "a.txt", 0, "alpha mark", cx).await;
    add_bookmark(&project, worktree_id, "b.txt", 1, "epsilon mark", cx).await;
    add_bookmark(&project, worktree_id, "c.txt", 2, "iota mark", cx).await;

    let (picker, workspace, cx) = build_bookmarks_picker(project, cx);
    select_bookmark_with_label(&picker, "alpha mark", cx);
    cx.dispatch_action(Confirm);
    cx.run_until_parked();
    assert_active_pane_items_len(&workspace, 1, cx);

    let picker = open_bookmarks_picker(&workspace, cx);
    select_bookmark_with_label(&picker, "epsilon mark", cx);
    cx.dispatch_action(Confirm);
    cx.run_until_parked();
    assert_active_pane_items_len(&workspace, 1, cx);

    let picker = open_bookmarks_picker(&workspace, cx);
    select_bookmark_with_label(&picker, "alpha mark", cx);
    cx.dispatch_action(SecondaryConfirm);
    cx.run_until_parked();
    assert_active_pane_items_len(&workspace, 2, cx);

    let picker = open_bookmarks_picker(&workspace, cx);
    select_bookmark_with_label(&picker, "iota mark", cx);
    cx.dispatch_action(SecondaryConfirm);
    cx.run_until_parked();
    assert_active_pane_items_len(&workspace, 3, cx);
}

fn init_test(cx: &mut TestAppContext) -> std::sync::Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        super::init(cx);
        editor::init(cx);
        state
    })
}

async fn add_bookmark(
    project: &Entity<Project>,
    worktree_id: project::WorktreeId,
    path: &str,
    row: u32,
    label: &str,
    cx: &mut TestAppContext,
) {
    let project_path = ProjectPath {
        worktree_id,
        path: rel_path(path).into(),
    };
    let buffer = project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))
        .await
        .unwrap();
    let anchor = buffer.read_with(cx, |buffer, _cx| {
        buffer.text_snapshot().anchor_before(Point::new(row, 0))
    });
    let bookmark_store = project.read_with(cx, |project, _| project.bookmark_store());
    bookmark_store.update(cx, |bookmark_store, cx| {
        bookmark_store.toggle_bookmark(buffer, anchor, label.to_string(), cx);
    });
}

fn build_bookmarks_picker(
    project: Entity<Project>,
    cx: &mut TestAppContext,
) -> (
    Entity<Picker<ProjectBookmarksDelegate>>,
    Entity<workspace::Workspace>,
    &mut VisualTestContext,
) {
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    let picker = open_bookmarks_picker(&workspace, cx);
    cx.run_until_parked();
    (picker, workspace, cx)
}

fn open_bookmarks_picker(
    workspace: &Entity<workspace::Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<Picker<ProjectBookmarksDelegate>> {
    cx.dispatch_action(ToggleProjectBookmarks);
    cx.run_until_parked();
    workspace.update(cx, |workspace, cx| {
        workspace
            .active_modal::<Picker<ProjectBookmarksDelegate>>(cx)
            .expect("project bookmarks picker is not open")
    })
}

fn simulate_input(cx: &mut VisualTestContext, input: &str) {
    cx.simulate_input(input);
    cx.run_until_parked();
}

fn select_bookmark_with_label(
    picker: &Entity<Picker<ProjectBookmarksDelegate>>,
    label: &str,
    cx: &mut VisualTestContext,
) {
    picker.update_in(cx, |picker, window, cx| {
        let match_index = picker
            .delegate
            .matches
            .iter()
            .position(|search_match| search_match.label.as_ref() == label)
            .expect("bookmark label should be present");
        let entry_index = picker
            .delegate
            .entries
            .iter()
            .position(|entry| matches!(entry, Entry::Match(ix) if *ix == match_index))
            .expect("bookmark entry should be present");
        picker.set_selected_index(entry_index, None, true, window, cx);
    });
}

fn assert_active_pane_items_len(
    workspace: &Entity<workspace::Workspace>,
    expected_len: usize,
    cx: &mut VisualTestContext,
) {
    workspace.read_with(cx, |workspace, cx| {
        let actual_len = workspace.active_pane().read(cx).items_len();
        assert_eq!(actual_len, expected_len);
    });
}
