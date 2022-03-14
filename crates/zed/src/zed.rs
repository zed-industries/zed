pub mod assets;
pub mod language;
pub mod menus;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use chat_panel::ChatPanel;
pub use client;
pub use contacts_panel;
use contacts_panel::ContactsPanel;
pub use editor;
use gpui::{
    action,
    geometry::vector::vec2f,
    keymap::Binding,
    platform::{WindowBounds, WindowOptions},
    ModelHandle, ViewContext,
};
use lazy_static::lazy_static;
pub use lsp;
use project::Project;
pub use project::{self, fs};
use project_panel::ProjectPanel;
use std::{path::PathBuf, sync::Arc};
pub use workspace;
use workspace::{AppState, Settings, Workspace, WorkspaceParams};

action!(About);
action!(Quit);
action!(OpenSettings);
action!(AdjustBufferFontSize, f32);

const MIN_FONT_SIZE: f32 = 6.0;

lazy_static! {
    pub static ref ROOT_PATH: PathBuf = dirs::home_dir()
        .expect("failed to determine home directory")
        .join(".zed");
    pub static ref SETTINGS_PATH: PathBuf = ROOT_PATH.join("settings.json");
}

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_global_action(quit);
    cx.add_global_action({
        move |action: &AdjustBufferFontSize, cx| {
            cx.update_app_state::<Settings, _, _>(|settings, cx| {
                settings.buffer_font_size =
                    (settings.buffer_font_size + action.0).max(MIN_FONT_SIZE);
                cx.refresh_windows();
            });
        }
    });

    cx.add_action({
        let fs = app_state.fs.clone();
        move |_: &mut Workspace, _: &OpenSettings, cx: &mut ViewContext<Workspace>| {
            let fs = fs.clone();
            cx.spawn(move |workspace, mut cx| async move {
                if !fs.is_file(&SETTINGS_PATH).await {
                    fs.create_dir(&ROOT_PATH).await?;
                    fs.create_file(&SETTINGS_PATH, Default::default()).await?;
                }
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.open_paths(&[SETTINGS_PATH.clone()], cx)
                    })
                    .await;
                Ok::<_, anyhow::Error>(())
            })
            .detach_and_log_err(cx);
        }
    });

    workspace::lsp_status::init(cx);

    cx.add_bindings(vec![
        Binding::new("cmd-=", AdjustBufferFontSize(1.), None),
        Binding::new("cmd--", AdjustBufferFontSize(-1.), None),
        Binding::new("cmd-,", OpenSettings, None),
    ])
}

pub fn build_workspace(
    project: ModelHandle<Project>,
    app_state: &Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) -> Workspace {
    let workspace_params = WorkspaceParams {
        project,
        client: app_state.client.clone(),
        fs: app_state.fs.clone(),
        languages: app_state.languages.clone(),
        user_store: app_state.user_store.clone(),
        channel_list: app_state.channel_list.clone(),
        path_openers: app_state.path_openers.clone(),
    };
    let mut workspace = Workspace::new(&workspace_params, cx);
    let project = workspace.project().clone();

    project.update(cx, |project, _| {
        project.set_language_server_settings(serde_json::json!({
            "json": {
                "schemas": [
                    {
                        "fileMatch": "**/.zed/settings.json",
                        "schema": Settings::file_json_schema(),
                    }
                ]
            }
        }));
    });

    workspace.left_sidebar_mut().add_item(
        "icons/folder-tree-16.svg",
        ProjectPanel::new(project, cx).into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/user-16.svg",
        cx.add_view(|cx| ContactsPanel::new(app_state.clone(), cx))
            .into(),
    );
    workspace.right_sidebar_mut().add_item(
        "icons/comment-16.svg",
        cx.add_view(|cx| {
            ChatPanel::new(app_state.client.clone(), app_state.channel_list.clone(), cx)
        })
        .into(),
    );

    let diagnostic_message = cx.add_view(|_| editor::items::DiagnosticMessage::new());
    let diagnostic_summary =
        cx.add_view(|cx| diagnostics::items::DiagnosticSummary::new(workspace.project(), cx));
    let lsp_status = cx.add_view(|cx| {
        workspace::lsp_status::LspStatus::new(workspace.project(), app_state.languages.clone(), cx)
    });
    let cursor_position = cx.add_view(|_| editor::items::CursorPosition::new());
    workspace.status_bar().update(cx, |status_bar, cx| {
        status_bar.add_left_item(diagnostic_summary, cx);
        status_bar.add_left_item(diagnostic_message, cx);
        status_bar.add_left_item(lsp_status, cx);
        status_bar.add_right_item(cursor_position, cx);
    });

    workspace
}

pub fn build_window_options() -> WindowOptions<'static> {
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

#[cfg(test)]
mod tests {
    use crate::assets::Assets;

    use super::*;
    use editor::{DisplayPoint, Editor};
    use gpui::{AssetSource, MutableAppContext, TestAppContext, ViewHandle};
    use project::{Fs, ProjectPath};
    use serde_json::json;
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };
    use test::test_app_state;
    use theme::{Theme, ThemeRegistry, DEFAULT_THEME_NAME};
    use util::test::temp_tree;
    use workspace::{
        open_paths, pane, ItemView, ItemViewHandle, OpenNew, Pane, SplitDirection, WorkspaceHandle,
    };

    #[gpui::test]
    async fn test_open_paths_action(cx: &mut TestAppContext) {
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
                &[
                    dir.path().join("a").to_path_buf(),
                    dir.path().join("b").to_path_buf(),
                ],
                &app_state,
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| open_paths(&[dir.path().join("a").to_path_buf()], &app_state, cx))
            .await;
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_1 = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        workspace_1.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.worktrees(cx).count(), 2)
        });

        cx.update(|cx| {
            open_paths(
                &[
                    dir.path().join("b").to_path_buf(),
                    dir.path().join("c").to_path_buf(),
                ],
                &app_state,
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 2);
    }

    #[gpui::test]
    async fn test_new_empty_workspace(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        cx.update(|cx| {
            workspace::init(cx);
        });
        cx.dispatch_global_action(workspace::OpenNew(app_state.clone()));
        let window_id = *cx.window_ids().first().unwrap();
        let workspace = cx.root_view::<Workspace>(window_id).unwrap();
        let editor = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<editor::Editor>()
                .unwrap()
        });

        editor.update(cx, |editor, cx| {
            assert!(editor.text(cx).is_empty());
        });

        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(cx));
        app_state.fs.as_fake().insert_dir("/root").await;
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name")));
        save_task.await.unwrap();
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name");
        });
    }

    #[gpui::test]
    async fn test_open_entry(cx: &mut TestAppContext) {
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
        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
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
        let entry_1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.item_views().count(), 1);
        });

        // Open the second entry
        workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.item_views().count(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        let entry_1b = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), cx))
            .await
            .unwrap();
        assert_eq!(entry_1.id(), entry_1b.id());

        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.item_views().count(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        workspace
            .update(cx, |w, cx| {
                w.split_pane(w.active_pane().clone(), SplitDirection::Right, cx);
                w.open_path(file2.clone(), cx)
            })
            .await
            .unwrap();

        workspace.read_with(cx, |w, cx| {
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
        let (t1, t2) = workspace.update(cx, |w, cx| {
            (
                w.open_path(file3.clone(), cx),
                w.open_path(file3.clone(), cx),
            )
        });
        t1.await.unwrap();
        t2.await.unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file3.clone())
            );
            let pane_entries = pane
                .item_views()
                .map(|i| i.project_path(cx).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pane_entries, &[file1, file2, file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        let fs = app_state.fs.as_fake();
        fs.insert_dir("/dir1").await;
        fs.insert_dir("/dir2").await;
        fs.insert_file("/dir1/a.txt", "".into()).await;
        fs.insert_file("/dir2/b.txt", "".into()).await;

        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/dir1", true, cx)
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
                    .to_any()
                    .downcast::<Editor>()
                    .unwrap()
                    .read(cx)
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
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
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
                    .to_any()
                    .downcast::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "b.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        let fs = app_state.fs.as_fake();
        fs.insert_tree("/root", json!({ "a.txt": "" })).await;

        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
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
            item.downcast::<Editor>().unwrap()
        });

        cx.update(|cx| {
            editor.update(cx, |editor, cx| {
                editor.handle_input(&editor::Input("x".into()), cx)
            })
        });
        fs.insert_file("/root/a.txt", "changed".to_string()).await;
        editor
            .condition(&cx, |editor, cx| editor.has_conflict(cx))
            .await;
        cx.read(|cx| assert!(editor.is_dirty(cx)));

        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(cx));
        cx.simulate_prompt_answer(window_id, 0);
        save_task.await.unwrap();
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert!(!editor.has_conflict(cx));
        });
    }

    #[gpui::test]
    async fn test_open_and_save_new_file(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        app_state.fs.as_fake().insert_dir("/root").await;
        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        let worktree = cx.read(|cx| workspace.read(cx).worktrees(cx).next().unwrap());

        // Create a new untitled buffer
        cx.dispatch_action(
            window_id,
            vec![workspace.entity_id()],
            OpenNew(app_state.clone()),
        );
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "untitled");
            assert!(Arc::ptr_eq(
                editor.language(cx).unwrap(),
                &language::PLAIN_TEXT
            ));
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx));
        });

        // Save the buffer. This prompts for a filename.
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(cx));
        cx.simulate_new_path_selection(|parent_dir| {
            assert_eq!(parent_dir, Path::new("/root"));
            Some(parent_dir.join("the-new-name.rs"))
        });
        cx.read(|cx| {
            assert!(editor.is_dirty(cx));
            assert_eq!(editor.read(cx).title(cx), "untitled");
        });

        // When the save completes, the buffer's title is updated and the language is assigned based
        // on the path.
        save_task.await.unwrap();
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name.rs");
            assert_eq!(editor.language(cx).unwrap().name().as_ref(), "Rust");
        });

        // Edit the file and save it again. This time, there is no filename prompt.
        editor.update(cx, |editor, cx| {
            editor.handle_input(&editor::Input(" there".into()), cx);
            assert_eq!(editor.is_dirty(cx.as_ref()), true);
        });
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(cx));
        save_task.await.unwrap();
        assert!(!cx.did_prompt_for_new_path());
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name.rs")
        });

        // Open the same newly-created file in another pane item. The new editor should reuse
        // the same buffer.
        cx.dispatch_action(
            window_id,
            vec![workspace.entity_id()],
            OpenNew(app_state.clone()),
        );
        workspace
            .update(cx, |workspace, cx| {
                workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
                workspace.open_path(
                    ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: Path::new("the-new-name.rs").into(),
                    },
                    cx,
                )
            })
            .await
            .unwrap();
        let editor2 = workspace.update(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        cx.read(|cx| {
            assert_eq!(
                editor2.read(cx).buffer().read(cx).as_singleton().unwrap(),
                editor.read(cx).buffer().read(cx).as_singleton().unwrap()
            );
        })
    }

    #[gpui::test]
    async fn test_setting_language_when_saving_as_single_file_worktree(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        app_state.fs.as_fake().insert_dir("/root").await;
        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));

        // Create a new untitled buffer
        cx.dispatch_action(
            window_id,
            vec![workspace.entity_id()],
            OpenNew(app_state.clone()),
        );
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(cx, |editor, cx| {
            assert!(Arc::ptr_eq(
                editor.language(cx).unwrap(),
                &language::PLAIN_TEXT
            ));
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx.as_ref()));
        });

        // Save the buffer. This prompts for a filename.
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(cx));
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name.rs")));
        save_task.await.unwrap();
        // The buffer is not dirty anymore and the language is assigned based on the path.
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.language(cx).unwrap().name().as_ref(), "Rust")
        });
    }

    #[gpui::test]
    async fn test_pane_actions(cx: &mut TestAppContext) {
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

        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), cx))
            .await
            .unwrap();
        cx.read(|cx| {
            assert_eq!(
                pane_1.read(cx).active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
        });

        cx.dispatch_action(
            window_id,
            vec![pane_1.entity_id()],
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
    async fn test_navigation(cx: &mut TestAppContext) {
        let app_state = cx.update(test_app_state);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "file1": "contents 1\n".repeat(20),
                        "file2": "contents 2\n".repeat(20),
                        "file3": "contents 3\n".repeat(20),
                    },
                }),
            )
            .await;
        let params = cx.update(|cx| WorkspaceParams::local(&app_state, cx));
        let (_, workspace) = cx.add_window(|cx| Workspace::new(&params, cx));
        params
            .project
            .update(cx, |project, cx| {
                project.find_or_create_local_worktree("/root", true, cx)
            })
            .await
            .unwrap();
        cx.read(|cx| workspace.read(cx).worktree_scans_complete(cx))
            .await;
        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        let editor1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        editor1.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(10, 0)..DisplayPoint::new(10, 0)], cx);
        });
        let editor2 = workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let editor3 = workspace
            .update(cx, |w, cx| w.open_path(file3.clone(), cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        editor3.update(cx, |editor, cx| {
            editor.select_display_ranges(&[DisplayPoint::new(15, 0)..DisplayPoint::new(15, 0)], cx);
        });
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(15, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0))
        );

        // Go back one more time and ensure we don't navigate past the first item in the history.
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0))
        );

        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0))
        );

        // Go forward to an item that has been closed, ensuring it gets re-opened at the same
        // location.
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_pane()
                .update(cx, |pane, cx| pane.close_item(editor3.entity_id(), cx));
            drop(editor3);
        });
        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0))
        );

        // Go back to an item that has been closed and removed from disk, ensuring it gets skipped.
        workspace
            .update(cx, |workspace, cx| {
                workspace
                    .active_pane()
                    .update(cx, |pane, cx| pane.close_item(editor2.entity_id(), cx));
                drop(editor2);
                app_state
                    .fs
                    .as_fake()
                    .remove_file(Path::new("/root/a/file2"), Default::default())
            })
            .await
            .unwrap();
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0))
        );
        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0))
        );

        fn active_location(
            workspace: &ViewHandle<Workspace>,
            cx: &mut TestAppContext,
        ) -> (ProjectPath, DisplayPoint) {
            workspace.update(cx, |workspace, cx| {
                let item = workspace.active_item(cx).unwrap();
                let editor = item.downcast::<Editor>().unwrap();
                let selections = editor.update(cx, |editor, cx| editor.selected_display_ranges(cx));
                (item.project_path(cx).unwrap(), selections[0].start)
            })
        }
    }

    #[gpui::test]
    fn test_bundled_themes(cx: &mut MutableAppContext) {
        let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());

        lazy_static::lazy_static! {
            static ref DEFAULT_THEME: parking_lot::Mutex<Option<Arc<Theme>>> = Default::default();
            static ref FONTS: Vec<Arc<Vec<u8>>> = vec![
                Assets.load("fonts/zed-sans/zed-sans-extended.ttf").unwrap().to_vec().into()
            ];
        }

        cx.platform().fonts().add_fonts(&FONTS).unwrap();

        let mut has_default_theme = false;
        for theme_name in themes.list() {
            let theme = themes.get(&theme_name).unwrap();
            if theme.name == DEFAULT_THEME_NAME {
                has_default_theme = true;
            }
            assert_eq!(theme.name, theme_name);
        }
        assert!(has_default_theme);
    }
}
