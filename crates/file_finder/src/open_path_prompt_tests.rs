use std::sync::Arc;

use gpui::{AppContext, Entity, TestAppContext, VisualTestContext};
use picker::{Picker, PickerDelegate};
use project::Project;
use serde_json::json;
use ui::rems;
use util::path;
use workspace::{AppState, Workspace};

use crate::OpenPathDelegate;

#[gpui::test]
async fn test_open_path_prompt(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a1": "A1",
                "a2": "A2",
                "a3": "A3",
                "dir1": {},
                "dir2": {
                    "c": "C",
                    "d1": "D1",
                    "d2": "D2",
                    "d3": "D3",
                    "dir3": {},
                    "dir4": {}
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, cx) = build_open_path_prompt(project, cx);

    let query = path!("/root");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["root"]);

    // If the query ends with a slash, the picker should show the contents of the directory.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["a1", "a2", "a3", "dir1", "dir2"]
    );

    // Show candidates for the query "a".
    let query = path!("/root/a");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["a1", "a2", "a3"]
    );

    // Show candidates for the query "d".
    let query = path!("/root/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);

    let query = path!("/root/dir2");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir2"]);

    let query = path!("/root/dir2/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["c", "d1", "d2", "d3", "dir3", "dir4"]
    );

    // Show candidates for the query "d".
    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["d1", "d2", "d3", "dir3", "dir4"]
    );

    let query = path!("/root/dir2/di");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir3", "dir4"]);
}

#[gpui::test]
async fn test_open_path_prompt_completion(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a": "A",
                "dir1": {},
                "dir2": {
                    "c": "C",
                    "d": "D",
                    "dir3": {},
                    "dir4": {}
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, cx) = build_open_path_prompt(project, cx);

    // Confirm completion for the query "/root", since it's a directory, it should add a trailing slash.
    let query = path!("/root");
    insert_query(query, &picker, cx).await;
    assert_eq!(confirm_completion(query, 0, &picker, cx), path!("/root/"));

    // Confirm completion for the query "/root/", selecting the first candidate "a", since it's a file, it should not add a trailing slash.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(confirm_completion(query, 0, &picker, cx), path!("/root/a"));

    // Confirm completion for the query "/root/", selecting the second candidate "dir1", since it's a directory, it should add a trailing slash.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx),
        path!("/root/dir1/")
    );

    let query = path!("/root/a");
    insert_query(query, &picker, cx).await;
    assert_eq!(confirm_completion(query, 0, &picker, cx), path!("/root/a"));

    let query = path!("/root/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx),
        path!("/root/dir2/")
    );

    let query = path!("/root/dir2");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        path!("/root/dir2/")
    );

    let query = path!("/root/dir2/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        path!("/root/dir2/c")
    );

    let query = path!("/root/dir2/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 2, &picker, cx),
        path!("/root/dir2/dir3/")
    );

    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        path!("/root/dir2/d")
    );

    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx),
        path!("/root/dir2/dir3/")
    );

    let query = path!("/root/dir2/di");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx),
        path!("/root/dir2/dir4/")
    );
}

#[gpui::test]
#[cfg(target_os = "windows")]
async fn test_open_path_prompt_on_windows(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "a": "A",
                "dir1": {},
                "dir2": {}
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, cx) = build_open_path_prompt(project, cx);

    // Support both forward and backward slashes.
    let query = "C:/root/";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["a", "dir1", "dir2"]
    );
    assert_eq!(confirm_completion(query, 0, &picker, cx), "C:/root/a");

    let query = "C:\\root/";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["a", "dir1", "dir2"]
    );
    assert_eq!(confirm_completion(query, 0, &picker, cx), "C:\\root/a");

    let query = "C:\\root\\";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["a", "dir1", "dir2"]
    );
    assert_eq!(confirm_completion(query, 0, &picker, cx), "C:\\root\\a");

    // Confirm completion for the query "C:/root/d", selecting the second candidate "dir2", since it's a directory, it should add a trailing slash.
    let query = "C:/root/d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(confirm_completion(query, 1, &picker, cx), "C:/root/dir2\\");

    let query = "C:\\root/d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(confirm_completion(query, 0, &picker, cx), "C:\\root/dir1\\");

    let query = "C:\\root\\d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        "C:\\root\\dir1\\"
    );
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

fn build_open_path_prompt(
    project: Entity<Project>,
    cx: &mut TestAppContext,
) -> (Entity<Picker<OpenPathDelegate>>, &mut VisualTestContext) {
    let (tx, _) = futures::channel::oneshot::channel();
    let lister = project::DirectoryLister::Project(project.clone());
    let delegate = OpenPathDelegate::new(tx, lister.clone());

    let (workspace, cx) = cx.add_window_view(|window, cx| Workspace::test_new(project, window, cx));
    (
        workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                let picker = Picker::uniform_list(delegate, window, cx)
                    .width(rems(34.))
                    .modal(false);
                let query = lister.default_query(cx);
                picker.set_query(query, window, cx);
                picker
            })
        }),
        cx,
    )
}

async fn insert_query(
    query: &str,
    picker: &Entity<Picker<OpenPathDelegate>>,
    cx: &mut VisualTestContext,
) {
    picker
        .update_in(cx, |f, window, cx| {
            f.delegate.update_matches(query.to_string(), window, cx)
        })
        .await;
}

fn confirm_completion(
    query: &str,
    select: usize,
    picker: &Entity<Picker<OpenPathDelegate>>,
    cx: &mut VisualTestContext,
) -> String {
    picker
        .update_in(cx, |f, window, cx| {
            if f.delegate.selected_index() != select {
                f.delegate.set_selected_index(select, window, cx);
            }
            f.delegate.confirm_completion(query.to_string(), window, cx)
        })
        .unwrap()
}

fn collect_match_candidates(
    picker: &Entity<Picker<OpenPathDelegate>>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    picker.update(cx, |f, _| f.delegate.collect_match_candidates())
}
