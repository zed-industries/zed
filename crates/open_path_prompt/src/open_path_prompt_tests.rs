use std::{
    path::PathBuf,
    sync::{Arc, atomic},
};

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AppContext, Entity, KeyBinding, PathPromptOptions, TestAppContext, VisualTestContext};
use picker::{Picker, PickerDelegate};
use project::{DirectoryLister, DisableAiSettings, Project};
use serde_json::json;
use settings::{Settings, SettingsStore};
use util::path;
use workspace::{
    AppState, MultiWorkspace, OpenOptions, OpenPathPromptMode, Workspace, item::test::TestItem,
    prompt_for_open_path_and_open,
};

use crate::{CandidateInfo, DirectoryState, OpenPathDelegate, OpenPathPrompt};

#[gpui::test]
async fn test_open_path_prompt(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                ".a1": ".A1",
                ".b1": ".B1",
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

    let (picker, cx) = build_open_path_prompt(project, false, false, cx);

    insert_query(path!("sadjaoislkdjasldj"), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), Vec::<String>::new());

    let query = path!("/root");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["root"]);

    #[cfg(not(windows))]
    let expected_separator = "./";
    #[cfg(windows)]
    let expected_separator = ".\\";

    // If the query ends with a slash, the picker should show the contents of the directory and not show any of the hidden entries.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![expected_separator, "dir1", "dir2", "a1", "a2", "a3"]
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
        vec![expected_separator, "dir3", "dir4", "c", "d1", "d2", "d3"]
    );

    // Show candidates for the query "d".
    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["dir3", "dir4", "d1", "d2", "d3"]
    );

    let query = path!("/root/dir2/di");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir3", "dir4"]);

    // Don't show candidates for the query ".".
    let query = path!("/root/.");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), Vec::<String>::new());

    // Don't show any candidates for the query ".a".
    let query = path!("/root/.a");
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), Vec::<String>::new());

    // Show candidates for the query "./".
    // Should show current directory and contents.
    let query = path!("/root/./");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![expected_separator, "dir1", "dir2", "a1", "a2", "a3"]
    );

    // Show candidates for the query "../". Show parent contents.
    let query = path!("/root/dir1/../");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![expected_separator, "dir1", "dir2", "a1", "a2", "a3"]
    );
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

    let (picker, cx) = build_open_path_prompt(project, false, false, cx);

    // Confirm completion for the query "/root", since it's a directory, it should add a trailing slash.
    let query = path!("/root");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        path!("/root/")
    );

    // Confirm completion for the query "/root/", selecting the first candidate "dir1", since it's a directory, it should add a trailing slash.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        None,
        "First entry is `./` and when we confirm completion, it is tabbed below"
    );
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        path!("/root/dir1/"),
        "Second entry is the first directory"
    );

    // Confirm completion for the query "/root/", selecting the third candidate "a", since it's a file, it should not add a trailing slash.
    let query = path!("/root/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 3, &picker, cx).unwrap(),
        path!("/root/a")
    );

    let query = path!("/root/a");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        path!("/root/a")
    );

    let query = path!("/root/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        path!("/root/dir2/")
    );

    let query = path!("/root/dir2");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        path!("/root/dir2/")
    );

    let query = path!("/root/dir2/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        path!("/root/dir2/dir3/")
    );

    let query = path!("/root/dir2/");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 3, &picker, cx).unwrap(),
        path!("/root/dir2/c")
    );

    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        path!("/root/dir2/dir3/")
    );

    let query = path!("/root/dir2/d");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        path!("/root/dir2/dir4/")
    );

    let query = path!("/root/dir2/di");
    insert_query(query, &picker, cx).await;
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        path!("/root/dir2/dir4/")
    );
}

#[gpui::test]
#[cfg_attr(not(target_os = "windows"), ignore)]
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

    let (picker, cx) = build_open_path_prompt(project, false, false, cx);

    // Support both forward and backward slashes.
    let query = "C:/root/";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![".\\", "dir1", "dir2", "a"]
    );
    assert_eq!(
        confirm_completion(query, 0, &picker, cx),
        None,
        "First entry is `.\\` and when we confirm completion, it is tabbed below"
    );
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        "C:/root/dir1\\",
        "Second entry is the first directory"
    );

    let query = "C:\\root/";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![".\\", "dir1", "dir2", "a"]
    );
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        "C:\\root/dir1\\"
    );

    let query = "C:\\root\\";
    insert_query(query, &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![".\\", "dir1", "dir2", "a"]
    );
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        "C:\\root\\dir1\\"
    );

    // Confirm completion for the query "C:/root/d", selecting the second candidate "dir2", since it's a directory, it should add a trailing slash.
    let query = "C:/root/d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(
        confirm_completion(query, 1, &picker, cx).unwrap(),
        "C:/root/dir2\\"
    );

    let query = "C:\\root/d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        "C:\\root/dir1\\"
    );

    let query = "C:\\root\\d";
    insert_query(query, &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1", "dir2"]);
    assert_eq!(
        confirm_completion(query, 0, &picker, cx).unwrap(),
        "C:\\root\\dir1\\"
    );
}

#[gpui::test]
async fn test_new_path_prompt(cx: &mut TestAppContext) {
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

    let (picker, cx) = build_open_path_prompt(project, true, false, cx);

    insert_query(path!("/root"), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["root"]);

    insert_query(path!("/root/d"), &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec!["d", "dir1", "dir2"]
    );

    insert_query(path!("/root/dir1"), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1"]);

    insert_query(path!("/root/dir12"), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir12"]);

    insert_query(path!("/root/dir1"), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec!["dir1"]);
}

#[gpui::test]
async fn test_open_path_prompt_panics_with_stale_highlight_positions(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({}))
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (picker, cx) = build_open_path_prompt(project, false, false, cx);

    picker.update_in(cx, |picker, window, cx| {
        picker.delegate.prompt_root = "/".to_string();
        picker.delegate.directory_state = DirectoryState::List {
            parent_path: picker.delegate.prompt_root.clone(),
            entries: vec![CandidateInfo {
                path: StringMatchCandidate::new(0, "éclair"),
                is_dir: false,
            }],
            error: None,
        };
        picker.delegate.string_matches = vec![StringMatch {
            candidate_id: 0,
            score: 0.0,
            positions: vec![1],
            string: "ab".to_string(),
        }];

        picker.delegate.render_match(0, false, window, cx);
    });
}

#[gpui::test]
async fn test_open_path_prompt_with_show_hidden(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/root"),
            json!({
                "directory_1": {},
                "directory_2": {},
                ".hidden": {}
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;

    let (picker, cx) = build_open_path_prompt(project, false, true, cx);

    #[cfg(not(windows))]
    let expected_separator = "./";
    #[cfg(windows)]
    let expected_separator = ".\\";

    insert_query(path!("/root/"), &picker, cx).await;
    assert_eq!(
        collect_match_candidates(&picker, cx),
        vec![expected_separator, ".hidden", "directory_1", "directory_2"]
    );

    insert_query(path!("/root/."), &picker, cx).await;
    assert_eq!(collect_match_candidates(&picker, cx), vec![".hidden"]);
}

#[gpui::test]
async fn test_dismiss_cancels_in_flight_match(cx: &mut TestAppContext) {
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
            }),
        )
        .await;

    let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
    let (picker, cx) = build_open_path_prompt(project, false, false, cx);

    insert_query(path!("/root/a"), &picker, cx).await;

    let cancel_flag = picker.read_with(cx, |picker, _| picker.delegate.cancel_flag.clone());
    assert!(
        !cancel_flag.load(atomic::Ordering::Acquire),
        "cancel flag should be clear while the prompt is active"
    );

    picker.update_in(cx, |picker, window, cx| {
        picker.delegate.dismissed(window, cx);
    });

    assert!(
        cancel_flag.load(atomic::Ordering::Acquire),
        "dismissing the prompt must cancel the in-flight fuzzy match"
    );
}

#[gpui::test]
async fn test_open_path_prompt_initial_query_for_file_and_directory_roots(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/"),
            json!({ "root": { "first.txt": "" }, "root-file.txt": "" }),
        )
        .await;

    for (root, expected_query) in [
        (path!("/root/first.txt"), path!("/root/")),
        (path!("/root"), path!("/root/")),
        (path!("/root-file.txt"), path!("/")),
        (path!("/"), path!("/")),
    ] {
        let project = Project::test(app_state.fs.clone(), [root.as_ref()], cx).await;
        assert_eq!(
            cx.update(|cx| DirectoryLister::Project(project).default_query(cx)),
            expected_query
        );
    }
}

#[gpui::test]
async fn test_open_file_selections_preserve_workspace_and_dirty_items(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/"),
            json!({
                "root": {
                    "first.txt": "first",
                    "second.txt": "second",
                    "third.txt": "third",
                },
                "outside.txt": "outside",
            }),
        )
        .await;
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.update_user_settings(cx, |settings| {
            settings.workspace.use_system_path_prompts = Some(true);
        });
    });

    for (root, disable_ai) in [
        (path!("/root/first.txt"), false),
        (path!("/root/first.txt"), true),
        (path!("/root"), false),
        (path!("/root"), true),
    ] {
        cx.update(|cx| DisableAiSettings::override_global(DisableAiSettings { disable_ai }, cx));
        let project = Project::test(app_state.fs.clone(), [root.as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |multi_workspace, cx| {
            assert_eq!(multi_workspace.multi_workspace_enabled(cx), !disable_ai);
            multi_workspace.workspace().clone()
        });
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/root/first.txt")),
                    OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        let dirty_item = cx.new(|cx| TestItem::new(cx).with_dirty(true));
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_item_to_active_pane(Box::new(dirty_item.clone()), None, true, window, cx);
        });
        let window_count = cx.update(|_, cx| cx.windows().len());
        let mut expected_paths = vec![PathBuf::from(path!("/root/first.txt"))];

        for selected_paths in [
            vec![PathBuf::from(path!("/root/second.txt"))],
            vec![
                PathBuf::from(path!("/root/third.txt")),
                PathBuf::from(path!("/outside.txt")),
            ],
        ] {
            expected_paths.extend(selected_paths.iter().cloned());
            expected_paths.sort();
            prompt_and_select_paths(&workspace, selected_paths, false, cx);

            assert!(!cx.has_pending_prompt());
            assert_eq!(cx.update(|_, cx| cx.windows().len()), window_count);
            multi_workspace.read_with(cx, |multi_workspace, _| {
                assert_eq!(multi_workspace.workspace(), &workspace);
                assert_eq!(multi_workspace.workspaces().count(), 1);
            });
            workspace.read_with(cx, |workspace, cx| {
                let mut paths = workspace.open_item_abs_paths(cx);
                paths.sort();
                assert_eq!(paths, expected_paths);
                assert_eq!(
                    workspace.root_paths(cx),
                    vec![Arc::<std::path::Path>::from(PathBuf::from(root))]
                );

                assert_eq!(
                    workspace.items_of_type::<TestItem>(cx).collect::<Vec<_>>(),
                    vec![dirty_item.clone()]
                );
                assert!(dirty_item.read(cx).is_dirty);
            });
        }
    }
}

#[gpui::test]
async fn test_open_project_stays_in_window(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    app_state
        .fs
        .as_fake()
        .insert_tree(
            path!("/"),
            json!({ "root": { "file.txt": "" }, "other": {} }),
        )
        .await;
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store.update_user_settings(cx, |settings| {
            settings.workspace.use_system_path_prompts = Some(true);
        });
    });

    for (mut paths, disable_ai) in [
        (vec![PathBuf::from(path!("/other"))], false),
        (
            vec![
                PathBuf::from(path!("/other")),
                PathBuf::from(path!("/root/file.txt")),
            ],
            false,
        ),
        (vec![PathBuf::from(path!("/other"))], true),
    ] {
        cx.update(|cx| DisableAiSettings::override_global(DisableAiSettings { disable_ai }, cx));
        let project = Project::test(app_state.fs.clone(), [path!("/root").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        let window_count = cx.update(|_, cx| cx.windows().len());
        prompt_and_select_paths(&workspace, paths.clone(), false, cx);
        assert_eq!(cx.update(|_, cx| cx.windows().len()), window_count);
        let opened_workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        assert_ne!(opened_workspace, workspace);
        paths.sort();
        opened_workspace.read_with(cx, |workspace, cx| {
            let mut roots = workspace
                .root_paths(cx)
                .iter()
                .map(|path| path.to_path_buf())
                .collect::<Vec<_>>();
            roots.sort();
            assert_eq!(roots, paths);
        });
        prompt_and_select_paths(&opened_workspace, paths, false, cx);
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert_eq!(multi_workspace.workspace(), &opened_workspace);
        });
        if disable_ai {
            prompt_and_select_paths(
                &opened_workspace,
                vec![
                    PathBuf::from(path!("/other")),
                    PathBuf::from(path!("/root/file.txt")),
                ],
                false,
                cx,
            );
            multi_workspace.read_with(cx, |multi_workspace, _| {
                assert_eq!(multi_workspace.workspace(), &opened_workspace);
            });
        }
    }
}

#[gpui::test]
async fn test_open_path_prompt_window_choices(cx: &mut TestAppContext) {
    let app_state = init_test(cx);
    cx.update(|cx| {
        cx.bind_keys([
            KeyBinding::new("enter", menu::Confirm, None),
            KeyBinding::new("ctrl-enter", menu::SecondaryConfirm, None),
            KeyBinding::new("escape", menu::Cancel, None),
        ])
    });
    app_state
        .fs
        .as_fake()
        .insert_tree(path!("/root"), json!({ "file.txt": "" }))
        .await;
    let project = Project::test(app_state.fs.clone(), [], cx).await;
    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace =
        multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
    workspace.update_in(cx, |workspace, window, cx| {
        OpenPathPrompt::register(workspace, Some(window), cx)
    });
    let mut expected_paths = Vec::new();

    for (system_prompt, default_new_window, key, new_window) in [
        (false, false, "ctrl-enter", true),
        (false, false, "enter", false),
        (false, true, "enter", true),
        (false, true, "ctrl-enter", false),
        (false, false, "escape", false),
        (true, false, "enter", false),
        (true, true, "enter", true),
        (true, false, "escape", false),
    ] {
        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.workspace.use_system_path_prompts = Some(system_prompt)
            });
        });
        let window_count = cx.update(|_, cx| cx.windows().len());
        open_path_prompt(&workspace, default_new_window, cx);
        if system_prompt {
            cx.simulate_path_prompt_response(|options| {
                assert_eq!(
                    options.prompt.as_deref(),
                    Some(if default_new_window {
                        "Open in New Window"
                    } else {
                        "Open in This Window"
                    })
                );
                (key != "escape").then(|| vec![PathBuf::from(path!("/root/file.txt"))])
            });
        } else {
            let picker = workspace.read_with(cx, |workspace, cx| {
                workspace
                    .active_modal::<Picker<OpenPathDelegate>>(cx)
                    .unwrap()
            });
            picker.update_in(cx, |picker, window, cx| {
                assert!(picker.delegate.render_footer(window, cx).is_some());
                picker.set_query(path!("/root/file.txt"), window, cx);
            });
            cx.simulate_keystrokes(key);
        }
        cx.run_until_parked();
        assert_eq!(
            cx.update(|_, cx| cx.windows().len()),
            window_count + usize::from(new_window)
        );
        multi_workspace.read_with(cx, |multi_workspace, _| {
            assert_eq!(multi_workspace.workspace(), &workspace)
        });
        if key != "escape" && !new_window {
            expected_paths = vec![PathBuf::from(path!("/root/file.txt"))];
        }
        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.open_item_abs_paths(cx), expected_paths);
            assert!(
                workspace
                    .active_modal::<Picker<OpenPathDelegate>>(cx)
                    .is_none()
            );
        });
    }
}

fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
    cx.update(|cx| {
        let state = AppState::test(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);

        editor::init(cx);
        state
    })
}

fn build_open_path_prompt(
    project: Entity<Project>,
    creating_path: bool,
    show_hidden: bool,
    cx: &mut TestAppContext,
) -> (Entity<Picker<OpenPathDelegate>>, &mut VisualTestContext) {
    let (tx, _) = futures::channel::oneshot::channel();
    let lister = project::DirectoryLister::Project(project.clone());

    let (multi_workspace, cx) =
        cx.add_window_view(|window, cx| MultiWorkspace::test_new(project, window, cx));
    let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
    (
        workspace.update_in(cx, |_, window, cx| {
            let delegate = OpenPathDelegate::new(tx, lister.clone(), creating_path, cx);
            let delegate = if show_hidden {
                delegate.show_hidden()
            } else {
                delegate
            };
            cx.new(|cx| {
                let picker = Picker::uniform_list(delegate, window, cx).embedded();
                let query = lister.default_query(cx);
                picker.set_query(&query, window, cx);
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
) -> Option<String> {
    picker.update_in(cx, |f, window, cx| {
        if f.delegate.selected_index() != select {
            f.delegate.set_selected_index(select, window, cx);
        }
        f.delegate.confirm_completion(query.to_string(), window, cx)
    })
}

fn collect_match_candidates(
    picker: &Entity<Picker<OpenPathDelegate>>,
    cx: &mut VisualTestContext,
) -> Vec<String> {
    picker.update(cx, |f, _| f.delegate.collect_match_candidates())
}

fn prompt_and_select_paths(
    workspace: &Entity<Workspace>,
    paths: Vec<PathBuf>,
    create_new_window: bool,
    cx: &mut VisualTestContext,
) {
    open_path_prompt(workspace, create_new_window, cx);
    cx.simulate_path_prompt_response(|_| Some(paths));
    cx.run_until_parked();
}

fn open_path_prompt(
    workspace: &Entity<Workspace>,
    create_new_window: bool,
    cx: &mut VisualTestContext,
) {
    workspace.update_in(cx, |workspace, window, cx| {
        let app_state = workspace.app_state().clone();
        prompt_for_open_path_and_open(
            workspace,
            app_state,
            PathPromptOptions {
                files: true,
                directories: true,
                multiple: true,
                prompt: None,
            },
            if create_new_window {
                OpenPathPromptMode::NewWindow
            } else {
                OpenPathPromptMode::CurrentWindow
            },
            window,
            cx,
        );
    });
    cx.run_until_parked();
}
