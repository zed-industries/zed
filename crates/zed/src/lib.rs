pub mod assets;
pub mod language;
pub mod menus;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use self::language::LanguageRegistry;
use chat_panel::ChatPanel;
pub use client;
pub use editor;
use gpui::{
    action,
    geometry::vector::vec2f,
    keymap::Binding,
    platform::{WindowBounds, WindowOptions},
    ModelHandle, MutableAppContext, PathPromptOptions, Task, ViewContext,
};
pub use lsp;
use parking_lot::Mutex;
pub use people_panel;
use people_panel::PeoplePanel;
use postage::watch;
pub use project::{self, fs};
use project_panel::ProjectPanel;
use std::{path::PathBuf, sync::Arc};
use theme::ThemeRegistry;
use theme_selector::ThemeSelectorParams;
pub use workspace;
use workspace::{OpenNew, Settings, Workspace, WorkspaceParams};

action!(About);
action!(Open, Arc<AppState>);
action!(OpenPaths, OpenParams);
action!(Quit);
action!(AdjustBufferFontSize, f32);

const MIN_FONT_SIZE: f32 = 6.0;

pub struct AppState {
    pub settings_tx: Arc<Mutex<watch::Sender<Settings>>>,
    pub settings: watch::Receiver<Settings>,
    pub languages: Arc<LanguageRegistry>,
    pub themes: Arc<ThemeRegistry>,
    pub client: Arc<client::Client>,
    pub user_store: ModelHandle<client::UserStore>,
    pub fs: Arc<dyn fs::Fs>,
    pub channel_list: ModelHandle<client::ChannelList>,
    pub entry_openers: Arc<[Box<dyn workspace::EntryOpener>]>,
}

#[derive(Clone)]
pub struct OpenParams {
    pub paths: Vec<PathBuf>,
    pub app_state: Arc<AppState>,
}

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_global_action(open);
    cx.add_global_action(|action: &OpenPaths, cx: &mut MutableAppContext| {
        open_paths(action, cx).detach()
    });
    cx.add_global_action(open_new);
    cx.add_global_action(quit);

    cx.add_global_action({
        let settings_tx = app_state.settings_tx.clone();

        move |action: &AdjustBufferFontSize, cx| {
            let mut settings_tx = settings_tx.lock();
            let new_size = (settings_tx.borrow().buffer_font_size + action.0).max(MIN_FONT_SIZE);
            settings_tx.borrow_mut().buffer_font_size = new_size;
            cx.refresh_windows();
        }
    });

    cx.add_bindings(vec![
        Binding::new("cmd-=", AdjustBufferFontSize(1.), None),
        Binding::new("cmd--", AdjustBufferFontSize(-1.), None),
    ])
}

fn open(action: &Open, cx: &mut MutableAppContext) {
    let app_state = action.0.clone();
    cx.prompt_for_paths(
        PathPromptOptions {
            files: true,
            directories: true,
            multiple: true,
        },
        move |paths, cx| {
            if let Some(paths) = paths {
                cx.dispatch_global_action(OpenPaths(OpenParams { paths, app_state }));
            }
        },
    );
}

fn open_paths(action: &OpenPaths, cx: &mut MutableAppContext) -> Task<()> {
    log::info!("open paths {:?}", action.0.paths);

    // Open paths in existing workspace if possible
    for window_id in cx.window_ids().collect::<Vec<_>>() {
        if let Some(handle) = cx.root_view::<Workspace>(window_id) {
            let task = handle.update(cx, |view, cx| {
                if view.contains_paths(&action.0.paths, cx.as_ref()) {
                    log::info!("open paths on existing workspace");
                    Some(view.open_paths(&action.0.paths, cx))
                } else {
                    None
                }
            });

            if let Some(task) = task {
                return task;
            }
        }
    }

    log::info!("open new workspace");

    // Add a new workspace if necessary
    let app_state = &action.0.app_state;
    let (_, workspace) = cx.add_window(window_options(), |cx| {
        build_workspace(&WorkspaceParams::from(app_state.as_ref()), cx)
    });
    // cx.resize_window(window_id);
    workspace.update(cx, |workspace, cx| {
        workspace.open_paths(&action.0.paths, cx)
    })
}

fn open_new(action: &OpenNew, cx: &mut MutableAppContext) {
    let (window_id, workspace) =
        cx.add_window(window_options(), |cx| build_workspace(&action.0, cx));
    cx.dispatch_action(window_id, vec![workspace.id()], action);
}

fn build_workspace(params: &WorkspaceParams, cx: &mut ViewContext<Workspace>) -> Workspace {
    let mut workspace = Workspace::new(params, cx);
    let project = workspace.project().clone();
    workspace.left_sidebar_mut().add_item(
        "icons/folder-tree-16.svg",
        ProjectPanel::new(project, params.settings.clone(), cx).into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/user-16.svg",
        cx.add_view(|cx| PeoplePanel::new(params.user_store.clone(), params.settings.clone(), cx))
            .into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/comment-16.svg",
        cx.add_view(|cx| {
            ChatPanel::new(
                params.client.clone(),
                params.channel_list.clone(),
                params.settings.clone(),
                cx,
            )
        })
        .into(),
    );

    let diagnostic =
        cx.add_view(|_| editor::items::DiagnosticMessage::new(params.settings.clone()));
    let cursor_position =
        cx.add_view(|_| editor::items::CursorPosition::new(params.settings.clone()));
    workspace.status_bar().update(cx, |status_bar, cx| {
        status_bar.add_left_item(diagnostic, cx);
        status_bar.add_right_item(cursor_position, cx);
    });

    workspace
}

fn window_options() -> WindowOptions<'static> {
    WindowOptions {
        bounds: WindowBounds::Maximized,
        title: None,
        titlebar_appears_transparent: true,
        traffic_light_position: Some(vec2f(8., 8.)),
    }
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}

impl<'a> From<&'a AppState> for WorkspaceParams {
    fn from(state: &'a AppState) -> Self {
        Self {
            client: state.client.clone(),
            fs: state.fs.clone(),
            languages: state.languages.clone(),
            settings: state.settings.clone(),
            user_store: state.user_store.clone(),
            channel_list: state.channel_list.clone(),
            entry_openers: state.entry_openers.clone(),
        }
    }
}

impl<'a> From<&'a AppState> for ThemeSelectorParams {
    fn from(state: &'a AppState) -> Self {
        Self {
            settings_tx: state.settings_tx.clone(),
            settings: state.settings.clone(),
            themes: state.themes.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::Editor;
    use project::ProjectPath;
    use serde_json::json;
    use std::{collections::HashSet, path::Path};
    use test::test_app_state;
    use theme::DEFAULT_THEME_NAME;
    use util::test::temp_tree;
    use workspace::{pane, ItemView, ItemViewHandle, SplitDirection, WorkspaceHandle};

    #[gpui::test]
    async fn test_open_paths_action(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        let dir = temp_tree(json!({
            "a": {
                "aa": null,
                "ab": null,
            },
            "b": {
                "ba": null,
                "bb": null,
            },
            "c": {
                "ca": null,
                "cb": null,
            },
        }));

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![
                        dir.path().join("a").to_path_buf(),
                        dir.path().join("b").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![dir.path().join("a").to_path_buf()],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_1 = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        workspace_1.read_with(&cx, |workspace, cx| {
            assert_eq!(workspace.worktrees(cx).len(), 2)
        });

        cx.update(|cx| {
            open_paths(
                &OpenPaths(OpenParams {
                    paths: vec![
                        dir.path().join("b").to_path_buf(),
                        dir.path().join("c").to_path_buf(),
                    ],
                    app_state: app_state.clone(),
                }),
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 2);
    }

    #[gpui::test]
    async fn test_new_empty_workspace(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        cx.update(|cx| init(&app_state, cx));
        cx.dispatch_global_action(workspace::OpenNew(app_state.as_ref().into()));
        let window_id = *cx.window_ids().first().unwrap();
        let workspace = cx.root_view::<Workspace>(window_id).unwrap();
        let editor = workspace.update(&mut cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<editor::Editor>()
                .unwrap()
        });

        editor.update(&mut cx, |editor, cx| {
            assert!(editor.text(cx).is_empty());
        });

        workspace.update(&mut cx, |workspace, cx| {
            workspace.save_active_item(&workspace::Save, cx)
        });

        app_state.fs.as_fake().insert_dir("/root").await.unwrap();
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name")));

        editor
            .condition(&cx, |editor, cx| editor.title(cx) == "the-new-name")
            .await;
        editor.update(&mut cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
        });
    }

    #[gpui::test]
    async fn test_open_entry(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "file1": "contents 1",
                        "file2": "contents 2",
                        "file3": "contents 3",
                    },
                }),
            )
            .await;

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state.as_ref().into(), cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(Path::new("/root"), cx)
            })
            .await
            .unwrap();

        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        // Open the first entry
        workspace
            .update(&mut cx, |w, cx| w.open_entry(file1.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().len(), 1);
        });

        // Open the second entry
        workspace
            .update(&mut cx, |w, cx| w.open_entry(file2.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.items().len(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        workspace.update(&mut cx, |w, cx| {
            assert!(w.open_entry(file1.clone(), cx).is_none())
        });
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().len(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        workspace.update(&mut cx, |w, cx| {
            w.split_pane(w.active_pane().clone(), SplitDirection::Right, cx);
            assert!(w.open_entry(file2.clone(), cx).is_none());
            assert_eq!(
                w.active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .project_path(cx.as_ref()),
                Some(file2.clone())
            );
        });

        // Open the third entry twice concurrently. Only one pane item is added.
        let (t1, t2) = workspace.update(&mut cx, |w, cx| {
            (
                w.open_entry(file3.clone(), cx).unwrap(),
                w.open_entry(file3.clone(), cx).unwrap(),
            )
        });
        t1.await;
        t2.await;
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file3.clone())
            );
            let pane_entries = pane
                .items()
                .iter()
                .map(|i| i.project_path(cx).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pane_entries, &[file1, file2, file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        let fs = app_state.fs.as_fake();
        fs.insert_dir("/dir1").await.unwrap();
        fs.insert_dir("/dir2").await.unwrap();
        fs.insert_file("/dir1/a.txt", "".into()).await.unwrap();
        fs.insert_file("/dir2/b.txt", "".into()).await.unwrap();

        let (_, workspace) = cx.add_window(|cx| Workspace::new(&app_state.as_ref().into(), cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree("/dir1".as_ref(), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| view.open_paths(&["/dir1/a.txt".into()], cx))
        })
        .await;
        cx.read(|cx| {
            assert_eq!(
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .title(cx),
                "a.txt"
            );
        });

        // Open a file outside of any existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| view.open_paths(&["/dir2/b.txt".into()], cx))
        })
        .await;
        cx.read(|cx| {
            let worktree_roots = workspace
                .read(cx)
                .worktrees(cx)
                .iter()
                .map(|w| w.read(cx).as_local().unwrap().abs_path())
                .collect::<HashSet<_>>();
            assert_eq!(
                worktree_roots,
                vec!["/dir1", "/dir2/b.txt"]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );
            assert_eq!(
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .title(cx),
                "b.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        let fs = app_state.fs.as_fake();
        fs.insert_tree("/root", json!({ "a.txt": "" })).await;

        let (window_id, workspace) =
            cx.add_window(|cx| Workspace::new(&app_state.as_ref().into(), cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(Path::new("/root"), cx)
            })
            .await
            .unwrap();

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(&[PathBuf::from("/root/a.txt")], cx)
            })
        })
        .await;
        let editor = cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            let item = pane.active_item().unwrap();
            item.to_any().downcast::<Editor>().unwrap()
        });

        cx.update(|cx| {
            editor.update(cx, |editor, cx| {
                editor.handle_input(&editor::Input("x".into()), cx)
            })
        });
        fs.insert_file("/root/a.txt", "changed".to_string())
            .await
            .unwrap();
        editor
            .condition(&cx, |editor, cx| editor.has_conflict(cx))
            .await;
        cx.read(|cx| assert!(editor.is_dirty(cx)));

        cx.update(|cx| workspace.update(cx, |w, cx| w.save_active_item(&workspace::Save, cx)));
        cx.simulate_prompt_answer(window_id, 0);
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| assert!(!editor.has_conflict(cx)));
    }

    #[gpui::test]
    async fn test_open_and_save_new_file(mut cx: gpui::TestAppContext) {
        let app_state = cx.update(test_app_state);
        app_state.fs.as_fake().insert_dir("/root").await.unwrap();
        let params = app_state.as_ref().into();
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(Path::new("/root"), cx)
            })
            .await
            .unwrap();
        let worktree = cx.read(|cx| {
            workspace
                .read(cx)
                .worktrees(cx)
                .iter()
                .next()
                .unwrap()
                .clone()
        });

        // Create a new untitled buffer
        cx.dispatch_action(window_id, vec![workspace.id()], OpenNew(params.clone()));
        let editor = workspace.read_with(&cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(&mut cx, |editor, cx| {
            assert!(!editor.is_dirty(cx.as_ref()));
            assert_eq!(editor.title(cx.as_ref()), "untitled");
            assert!(editor.language(cx).is_none());
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx.as_ref()));
        });

        // Save the buffer. This prompts for a filename.
        workspace.update(&mut cx, |workspace, cx| {
            workspace.save_active_item(&workspace::Save, cx)
        });
        cx.simulate_new_path_selection(|parent_dir| {
            assert_eq!(parent_dir, Path::new("/root"));
            Some(parent_dir.join("the-new-name.rs"))
        });
        cx.read(|cx| {
            assert!(editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "untitled");
        });

        // When the save completes, the buffer's title is updated.
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name.rs");
        });
        // The language is assigned based on the path
        editor.read_with(&cx, |editor, cx| {
            assert_eq!(editor.language(cx).unwrap().name(), "Rust")
        });

        // Edit the file and save it again. This time, there is no filename prompt.
        editor.update(&mut cx, |editor, cx| {
            editor.handle_input(&editor::Input(" there".into()), cx);
            assert_eq!(editor.is_dirty(cx.as_ref()), true);
        });
        workspace.update(&mut cx, |workspace, cx| {
            workspace.save_active_item(&workspace::Save, cx)
        });
        assert!(!cx.did_prompt_for_new_path());
        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;
        cx.read(|cx| assert_eq!(editor.title(cx), "the-new-name.rs"));

        // Open the same newly-created file in another pane item. The new editor should reuse
        // the same buffer.
        cx.dispatch_action(window_id, vec![workspace.id()], OpenNew(params.clone()));
        workspace.update(&mut cx, |workspace, cx| {
            workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
            assert!(workspace
                .open_entry(
                    ProjectPath {
                        worktree_id: worktree.id(),
                        path: Path::new("the-new-name.rs").into()
                    },
                    cx
                )
                .is_none());
        });
        let editor2 = workspace.update(&mut cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap()
        });
        cx.read(|cx| {
            assert_eq!(editor2.read(cx).buffer(), editor.read(cx).buffer());
        })
    }

    #[gpui::test]
    async fn test_setting_language_when_saving_as_single_file_worktree(
        mut cx: gpui::TestAppContext,
    ) {
        let app_state = cx.update(test_app_state);
        app_state.fs.as_fake().insert_dir("/root").await.unwrap();
        let params = app_state.as_ref().into();
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));

        // Create a new untitled buffer
        cx.dispatch_action(window_id, vec![workspace.id()], OpenNew(params.clone()));
        let editor = workspace.read_with(&cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .to_any()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(&mut cx, |editor, cx| {
            assert!(editor.language(cx).is_none());
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx.as_ref()));
        });

        // Save the buffer. This prompts for a filename.
        workspace.update(&mut cx, |workspace, cx| {
            workspace.save_active_item(&workspace::Save, cx)
        });
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name.rs")));

        editor
            .condition(&cx, |editor, cx| !editor.is_dirty(cx))
            .await;

        // The language is assigned based on the path
        editor.read_with(&cx, |editor, cx| {
            assert_eq!(editor.language(cx).unwrap().name(), "Rust")
        });
    }

    #[gpui::test]
    async fn test_pane_actions(mut cx: gpui::TestAppContext) {
        cx.update(|cx| pane::init(cx));
        let app_state = cx.update(test_app_state);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "file1": "contents 1",
                        "file2": "contents 2",
                        "file3": "contents 3",
                    },
                }),
            )
            .await;

        let (window_id, workspace) =
            cx.add_window(|cx| Workspace::new(&app_state.as_ref().into(), cx));
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.add_worktree(Path::new("/root"), cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        workspace
            .update(&mut cx, |w, cx| w.open_entry(file1.clone(), cx))
            .unwrap()
            .await;
        cx.read(|cx| {
            assert_eq!(
                pane_1.read(cx).active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
        });

        cx.dispatch_action(
            window_id,
            vec![pane_1.id()],
            pane::Split(SplitDirection::Right),
        );
        cx.update(|cx| {
            let pane_2 = workspace.read(cx).active_pane().clone();
            assert_ne!(pane_1, pane_2);

            let pane2_item = pane_2.read(cx).active_item().unwrap();
            assert_eq!(pane2_item.project_path(cx.as_ref()), Some(file1.clone()));

            cx.dispatch_action(window_id, vec![pane_2.id()], &workspace::CloseActiveItem);
            let workspace = workspace.read(cx);
            assert_eq!(workspace.panes().len(), 1);
            assert_eq!(workspace.active_pane(), &pane_1);
        });
    }

    #[gpui::test]
    fn test_bundled_themes(cx: &mut MutableAppContext) {
        let app_state = test_app_state(cx);
        let mut has_default_theme = false;
        for theme_name in app_state.themes.list() {
            let theme = app_state.themes.get(&theme_name).unwrap();
            if theme.name == DEFAULT_THEME_NAME {
                has_default_theme = true;
            }
            assert_eq!(theme.name, theme_name);
        }
        assert!(has_default_theme);
    }
}
