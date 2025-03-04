use std::{path::PathBuf, sync::Arc};

use futures::channel::oneshot::Receiver;
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
                    "dir3": {
                        "b": "B"
                    }
                }
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, cx) = build_open_path_prompt(project, cx);

    let query = path!("/root");
    insert_query(query, &picker, cx).await;
    let canditates = collect_match_candidates(&picker, cx);
    println!("{:?}", canditates);
    let completion = confirm_completion(query, &picker, cx);
    println!("{:?}", completion);

    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    let canditates = collect_match_candidates(&picker, cx);
    println!("{:?}", canditates);

    let query = path!("/root/a");
    insert_query(query, &picker, cx).await;
    let canditates = collect_match_candidates(&picker, cx);
    println!("{:?}", canditates);

    let query = path!("/root/d");
    insert_query(query, &picker, cx).await;
    let canditates = collect_match_candidates(&picker, cx);
    println!("{:?}", canditates);

    // let query = path!("/root/d");
    // let result = query_path_prompt_result(&query, &picker, cx).await;
    // println!("{:?}", result);

    // let query = path!("/root/a");
    // let result = query_path_prompt_result(&query, &picker, cx).await;
    // println!("{:?}", result);

    // let query = path!("/root/dir2");
    // let result = query_path_prompt_result(&query, &picker, cx).await;
    // println!("{:?}", result);

    // let query = path!("/root/dir2/");
    // // let result = query_path_prompt_result(&query, &picker, cx).await;
    // picker
    //     .update_in(cx, |f, window, cx| {
    //         f.delegate.update_matches(query.to_string(), window, cx)
    //     })
    //     .await;
    // let result = picker.update_in(cx, |f, window, cx| f.delegate.confirm(false, window, cx));
    // println!("{:?}", rx.await);

    // let query = path!("/root/dir2/di");
    // let result = query_path_prompt_result(&query, &picker, cx).await;
    // println!("{:?}", result);
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
    picker: &Entity<Picker<OpenPathDelegate>>,
    cx: &mut VisualTestContext,
) -> String {
    picker
        .update_in(cx, |f, window, cx| {
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
