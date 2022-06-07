mod feedback;
pub mod languages;
pub mod menus;
pub mod settings_file;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use anyhow::{anyhow, Context, Result};
use breadcrumbs::Breadcrumbs;
pub use client;
pub use contacts_panel;
use contacts_panel::ContactsPanel;
pub use editor;
use editor::Editor;
use gpui::{
    actions,
    geometry::vector::vec2f,
    impl_actions,
    platform::{WindowBounds, WindowOptions},
    AsyncAppContext, ViewContext,
};
use lazy_static::lazy_static;
pub use lsp;
use project::Project;
pub use project::{self, fs};
use project_panel::ProjectPanel;
use search::{BufferSearchBar, ProjectSearchBar};
use serde::Deserialize;
use serde_json::to_string_pretty;
use settings::{keymap_file_json_schema, settings_file_json_schema, Settings};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;
pub use workspace;
use workspace::{AppState, Workspace};

#[derive(Deserialize, Clone, PartialEq)]
struct OpenBrowser {
    url: Arc<str>,
}

impl_actions!(zed, [OpenBrowser]);

actions!(
    zed,
    [
        About,
        Quit,
        DebugElements,
        OpenSettings,
        OpenKeymap,
        IncreaseBufferFontSize,
        DecreaseBufferFontSize,
        ResetBufferFontSize,
        InstallCommandLineInterface,
    ]
);

const MIN_FONT_SIZE: f32 = 6.0;

lazy_static! {
    pub static ref ROOT_PATH: PathBuf = dirs::home_dir()
        .expect("failed to determine home directory")
        .join(".zed");
    pub static ref SETTINGS_PATH: PathBuf = ROOT_PATH.join("settings.json");
    pub static ref KEYMAP_PATH: PathBuf = ROOT_PATH.join("keymap.json");
}

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_action(about);
    cx.add_global_action(quit);
    cx.add_global_action(move |action: &OpenBrowser, cx| cx.platform().open_url(&action.url));
    cx.add_global_action(move |_: &IncreaseBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = (settings.buffer_font_size + 1.0).max(MIN_FONT_SIZE);
            cx.refresh_windows();
        });
    });
    cx.add_global_action(move |_: &DecreaseBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = (settings.buffer_font_size - 1.0).max(MIN_FONT_SIZE);
            cx.refresh_windows();
        });
    });
    cx.add_global_action(move |_: &ResetBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = settings.default_buffer_font_size;
            cx.refresh_windows();
        });
    });
    cx.add_global_action(move |_: &InstallCommandLineInterface, cx| {
        cx.spawn(|cx| async move { install_cli(&cx).await.context("error creating CLI symlink") })
            .detach_and_log_err(cx);
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenSettings, cx: &mut ViewContext<Workspace>| {
            open_config_file(&SETTINGS_PATH, app_state.clone(), cx);
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenKeymap, cx: &mut ViewContext<Workspace>| {
            open_config_file(&KEYMAP_PATH, app_state.clone(), cx);
        }
    });
    cx.add_action(
        |workspace: &mut Workspace, _: &DebugElements, cx: &mut ViewContext<Workspace>| {
            let content = to_string_pretty(&cx.debug_elements()).unwrap();
            let project = workspace.project().clone();
            let json_language = project.read(cx).languages().get_language("JSON").unwrap();
            if project.read(cx).is_remote() {
                cx.propagate_action();
            } else if let Some(buffer) = project
                .update(cx, |project, cx| {
                    project.create_buffer(&content, Some(json_language), cx)
                })
                .log_err()
            {
                workspace.add_item(
                    Box::new(
                        cx.add_view(|cx| Editor::for_buffer(buffer, Some(project.clone()), cx)),
                    ),
                    cx,
                );
            }
        },
    );

    workspace::lsp_status::init(cx);
    settings::KeymapFileContent::load_defaults(cx);
}

pub fn initialize_workspace(
    workspace: &mut Workspace,
    app_state: &Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) {
    cx.subscribe(&cx.handle(), {
        let project = workspace.project().clone();
        move |_, _, event, cx| {
            if let workspace::Event::PaneAdded(pane) = event {
                pane.update(cx, |pane, cx| {
                    pane.toolbar().update(cx, |toolbar, cx| {
                        let breadcrumbs = cx.add_view(|_| Breadcrumbs::new(project.clone()));
                        toolbar.add_item(breadcrumbs, cx);
                        let buffer_search_bar = cx.add_view(|cx| BufferSearchBar::new(cx));
                        toolbar.add_item(buffer_search_bar, cx);
                        let project_search_bar = cx.add_view(|_| ProjectSearchBar::new());
                        toolbar.add_item(project_search_bar, cx);
                    })
                });
            }
        }
    })
    .detach();

    cx.emit(workspace::Event::PaneAdded(workspace.active_pane().clone()));

    let theme_names = app_state.themes.list().collect();
    let language_names = app_state.languages.language_names();

    workspace.project().update(cx, |project, cx| {
        let action_names = cx.all_action_names().collect::<Vec<_>>();
        project.set_language_server_settings(serde_json::json!({
            "json": {
                "schemas": [
                    {
                        "fileMatch": [".zed/settings.json"],
                        "schema": settings_file_json_schema(theme_names, language_names),
                    },
                    {
                        "fileMatch": [".zed/keymap.json"],
                        "schema": keymap_file_json_schema(&action_names),
                    }
                ]
            }
        }));
    });

    let project_panel = ProjectPanel::new(workspace.project().clone(), cx);
    let contact_panel = cx.add_view(|cx| {
        ContactsPanel::new(
            app_state.user_store.clone(),
            app_state.project_store.clone(),
            workspace.weak_handle(),
            cx,
        )
    });

    workspace.left_sidebar().update(cx, |sidebar, cx| {
        sidebar.add_item("icons/folder-tree-solid-14.svg", project_panel.into(), cx)
    });
    workspace.right_sidebar().update(cx, |sidebar, cx| {
        sidebar.add_item("icons/contacts-solid-14.svg", contact_panel.into(), cx)
    });

    let diagnostic_summary =
        cx.add_view(|cx| diagnostics::items::DiagnosticIndicator::new(workspace.project(), cx));
    let lsp_status = cx.add_view(|cx| {
        workspace::lsp_status::LspStatus::new(workspace.project(), app_state.languages.clone(), cx)
    });
    let cursor_position = cx.add_view(|_| editor::items::CursorPosition::new());
    let auto_update = cx.add_view(|cx| auto_update::AutoUpdateIndicator::new(cx));
    let feedback_link = cx.add_view(|_| feedback::FeedbackLink);
    workspace.status_bar().update(cx, |status_bar, cx| {
        status_bar.add_left_item(diagnostic_summary, cx);
        status_bar.add_left_item(lsp_status, cx);
        status_bar.add_right_item(cursor_position, cx);
        status_bar.add_right_item(auto_update, cx);
        status_bar.add_right_item(feedback_link, cx);
    });

    auto_update::notify_of_any_new_update(cx.weak_handle(), cx);
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

fn about(_: &mut Workspace, _: &About, cx: &mut gpui::ViewContext<Workspace>) {
    cx.prompt(
        gpui::PromptLevel::Info,
        &format!("Zed {}", env!("CARGO_PKG_VERSION")),
        &["OK"],
    );
}

async fn install_cli(cx: &AsyncAppContext) -> Result<()> {
    let cli_path = cx.platform().path_for_auxiliary_executable("cli")?;
    let link_path = Path::new("/usr/local/bin/zed");
    let bin_dir_path = link_path.parent().unwrap();

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(());
    }

    // If the symlink is not there or is outdated, first try replacing it
    // without escalating.
    smol::fs::remove_file(link_path).await.log_err();
    if smol::fs::unix::symlink(&cli_path, link_path)
        .await
        .log_err()
        .is_some()
    {
        return Ok(());
    }

    // The symlink could not be created, so use osascript with admin privileges
    // to create it.
    let status = smol::process::Command::new("osascript")
        .args([
            "-e",
            &format!(
                "do shell script \" \
                    mkdir -p \'{}\' && \
                    ln -sf \'{}\' \'{}\' \
                \" with administrator privileges",
                bin_dir_path.to_string_lossy(),
                cli_path.to_string_lossy(),
                link_path.to_string_lossy(),
            ),
        ])
        .stdout(smol::process::Stdio::inherit())
        .stderr(smol::process::Stdio::inherit())
        .output()
        .await?
        .status;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("error running osascript"))
    }
}

fn open_config_file(
    path: &'static Path,
    app_state: Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) {
    cx.spawn(|workspace, mut cx| async move {
        let fs = &app_state.fs;
        if !fs.is_file(path).await {
            fs.create_dir(&ROOT_PATH).await?;
            fs.create_file(path, Default::default()).await?;
        }

        workspace
            .update(&mut cx, |workspace, cx| {
                if workspace.project().read(cx).is_local() {
                    workspace.open_paths(vec![path.to_path_buf()], cx)
                } else {
                    let (_, workspace) = cx.add_window((app_state.build_window_options)(), |cx| {
                        let mut workspace = Workspace::new(
                            Project::local(
                                false,
                                app_state.client.clone(),
                                app_state.user_store.clone(),
                                app_state.project_store.clone(),
                                app_state.languages.clone(),
                                app_state.fs.clone(),
                                cx,
                            ),
                            cx,
                        );
                        (app_state.initialize_workspace)(&mut workspace, &app_state, cx);
                        workspace
                    });
                    workspace.update(cx, |workspace, cx| {
                        workspace.open_paths(vec![path.to_path_buf()], cx)
                    })
                }
            })
            .await;
        Ok::<_, anyhow::Error>(())
    })
    .detach_and_log_err(cx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assets::Assets;
    use editor::{Autoscroll, DisplayPoint, Editor};
    use gpui::{AssetSource, MutableAppContext, TestAppContext, ViewHandle};
    use project::ProjectPath;
    use serde_json::json;
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };
    use theme::{Theme, ThemeRegistry, DEFAULT_THEME_NAME};
    use workspace::{
        open_paths, pane, Item, ItemHandle, NewFile, Pane, SplitDirection, WorkspaceHandle,
    };

    #[gpui::test]
    async fn test_open_paths_action(cx: &mut TestAppContext) {
        let app_state = init(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
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
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/a"), PathBuf::from("/root/b")],
                &app_state,
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, cx))
            .await;
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_1 = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        workspace_1.update(cx, |workspace, cx| {
            assert_eq!(workspace.worktrees(cx).count(), 2);
            assert!(workspace.left_sidebar().read(cx).active_item().is_some());
            assert!(workspace.active_pane().is_focused(cx));
        });

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/b"), PathBuf::from("/root/c")],
                &app_state,
                cx,
            )
        })
        .await;
        assert_eq!(cx.window_ids().len(), 2);
    }

    #[gpui::test]
    async fn test_new_empty_workspace(cx: &mut TestAppContext) {
        let app_state = init(cx);
        cx.dispatch_global_action(workspace::NewFile);
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

        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(false, cx));
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
        let app_state = init(cx);
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

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        // Open the first entry
        let entry_1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), true, cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().count(), 1);
        });

        // Open the second entry
        workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), true, cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.items().count(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        let entry_1b = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), true, cx))
            .await
            .unwrap();
        assert_eq!(entry_1.id(), entry_1b.id());

        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items().count(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        workspace
            .update(cx, |w, cx| {
                w.split_pane(w.active_pane().clone(), SplitDirection::Right, cx);
                w.open_path(file2.clone(), true, cx)
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
                w.open_path(file3.clone(), true, cx),
                w.open_path(file3.clone(), true, cx),
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
                .items()
                .map(|i| i.project_path(cx).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(pane_entries, &[file1, file2, file3]);
        });
    }

    #[gpui::test]
    async fn test_open_paths(cx: &mut TestAppContext) {
        let app_state = init(cx);

        let fs = app_state.fs.as_fake();
        fs.insert_dir("/dir1").await;
        fs.insert_dir("/dir2").await;
        fs.insert_dir("/dir3").await;
        fs.insert_file("/dir1/a.txt", "".into()).await;
        fs.insert_file("/dir2/b.txt", "".into()).await;
        fs.insert_file("/dir3/c.txt", "".into()).await;

        let project = Project::test(app_state.fs.clone(), ["/dir1".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec!["/dir1/a.txt".into()], cx)
            })
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
            workspace.update(cx, |view, cx| {
                view.open_paths(vec!["/dir2/b.txt".into()], cx)
            })
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

        // Ensure opening a directory and one of its children only adds one worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec!["/dir3".into(), "/dir3/c.txt".into()], cx)
            })
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
                vec!["/dir1", "/dir2/b.txt", "/dir3"]
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
                "c.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(cx: &mut TestAppContext) {
        let app_state = init(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "a.txt": "" }))
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec![PathBuf::from("/root/a.txt")], cx)
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
        app_state
            .fs
            .as_fake()
            .insert_file("/root/a.txt", "changed".to_string())
            .await;
        editor
            .condition(&cx, |editor, cx| editor.has_conflict(cx))
            .await;
        cx.read(|cx| assert!(editor.is_dirty(cx)));

        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(false, cx));
        cx.simulate_prompt_answer(window_id, 0);
        save_task.await.unwrap();
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert!(!editor.has_conflict(cx));
        });
    }

    #[gpui::test]
    async fn test_open_and_save_new_file(cx: &mut TestAppContext) {
        let app_state = init(cx);
        app_state.fs.as_fake().insert_dir("/root").await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));
        let worktree = cx.read(|cx| workspace.read(cx).worktrees(cx).next().unwrap());

        // Create a new untitled buffer
        cx.dispatch_action(window_id, NewFile);
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
                editor.language_at(0, cx).unwrap(),
                &languages::PLAIN_TEXT
            ));
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx));
        });

        // Save the buffer. This prompts for a filename.
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(false, cx));
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
            assert_eq!(editor.language_at(0, cx).unwrap().name().as_ref(), "Rust");
        });

        // Edit the file and save it again. This time, there is no filename prompt.
        editor.update(cx, |editor, cx| {
            editor.handle_input(&editor::Input(" there".into()), cx);
            assert_eq!(editor.is_dirty(cx.as_ref()), true);
        });
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(false, cx));
        save_task.await.unwrap();
        assert!(!cx.did_prompt_for_new_path());
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.title(cx), "the-new-name.rs")
        });

        // Open the same newly-created file in another pane item. The new editor should reuse
        // the same buffer.
        cx.dispatch_action(window_id, NewFile);
        workspace
            .update(cx, |workspace, cx| {
                workspace.split_pane(workspace.active_pane().clone(), SplitDirection::Right, cx);
                workspace.open_path((worktree.read(cx).id(), "the-new-name.rs"), true, cx)
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
        let app_state = init(cx);
        app_state.fs.as_fake().insert_dir("/root").await;

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        // Create a new untitled buffer
        cx.dispatch_action(window_id, NewFile);
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });

        editor.update(cx, |editor, cx| {
            assert!(Arc::ptr_eq(
                editor.language_at(0, cx).unwrap(),
                &languages::PLAIN_TEXT
            ));
            editor.handle_input(&editor::Input("hi".into()), cx);
            assert!(editor.is_dirty(cx.as_ref()));
        });

        // Save the buffer. This prompts for a filename.
        let save_task = workspace.update(cx, |workspace, cx| workspace.save_active_item(false, cx));
        cx.simulate_new_path_selection(|_| Some(PathBuf::from("/root/the-new-name.rs")));
        save_task.await.unwrap();
        // The buffer is not dirty anymore and the language is assigned based on the path.
        editor.read_with(cx, |editor, cx| {
            assert!(!editor.is_dirty(cx));
            assert_eq!(editor.language_at(0, cx).unwrap().name().as_ref(), "Rust")
        });
    }

    #[gpui::test]
    async fn test_pane_actions(cx: &mut TestAppContext) {
        init(cx);

        let app_state = cx.update(AppState::test);
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

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::new(project, cx));

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), true, cx))
            .await
            .unwrap();

        let (editor_1, buffer) = pane_1.update(cx, |pane_1, cx| {
            let editor = pane_1.active_item().unwrap().downcast::<Editor>().unwrap();
            assert_eq!(editor.project_path(cx), Some(file1.clone()));
            let buffer = editor.update(cx, |editor, cx| {
                editor.insert("dirt", cx);
                editor.buffer().downgrade()
            });
            (editor.downgrade(), buffer)
        });

        cx.dispatch_action(window_id, pane::Split(SplitDirection::Right));
        let editor_2 = cx.update(|cx| {
            let pane_2 = workspace.read(cx).active_pane().clone();
            assert_ne!(pane_1, pane_2);

            let pane2_item = pane_2.read(cx).active_item().unwrap();
            assert_eq!(pane2_item.project_path(cx.as_ref()), Some(file1.clone()));

            pane2_item.downcast::<Editor>().unwrap().downgrade()
        });
        cx.dispatch_action(window_id, workspace::CloseActiveItem);

        cx.foreground().run_until_parked();
        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.panes().len(), 1);
            assert_eq!(workspace.active_pane(), &pane_1);
        });

        cx.dispatch_action(window_id, workspace::CloseActiveItem);
        cx.foreground().run_until_parked();
        cx.simulate_prompt_answer(window_id, 1);
        cx.foreground().run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.active_item(cx).is_none());
        });

        cx.assert_dropped(editor_1);
        cx.assert_dropped(editor_2);
        cx.assert_dropped(buffer);
    }

    #[gpui::test]
    async fn test_navigation(cx: &mut TestAppContext) {
        let app_state = init(cx);
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

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project.clone(), cx));

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        let editor1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        editor1.update(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                s.select_display_ranges([DisplayPoint::new(10, 0)..DisplayPoint::new(10, 0)])
            });
        });
        let editor2 = workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let editor3 = workspace
            .update(cx, |w, cx| w.open_path(file3.clone(), true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        editor3
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::Fit), cx, |s| {
                    s.select_display_ranges([DisplayPoint::new(12, 0)..DisplayPoint::new(12, 0)])
                });
                editor.newline(&Default::default(), cx);
                editor.newline(&Default::default(), cx);
                editor.move_down(&Default::default(), cx);
                editor.move_down(&Default::default(), cx);
                editor.save(project.clone(), cx)
            })
            .await
            .unwrap();
        editor3.update(cx, |editor, cx| {
            editor.set_scroll_position(vec2f(0., 12.5), cx)
        });
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(16, 0), 12.5)
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go back one more time and ensure we don't navigate past the first item in the history.
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go forward to an item that has been closed, ensuring it gets re-opened at the same
        // location.
        workspace
            .update(cx, |workspace, cx| {
                let editor3_id = editor3.id();
                drop(editor3);
                Pane::close_item(workspace, workspace.active_pane().clone(), editor3_id, cx)
            })
            .await
            .unwrap();
        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(16, 0), 12.5)
        );

        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go back to an item that has been closed and removed from disk, ensuring it gets skipped.
        workspace
            .update(cx, |workspace, cx| {
                let editor2_id = editor2.id();
                drop(editor2);
                Pane::close_item(workspace, workspace.active_pane().clone(), editor2_id, cx)
            })
            .await
            .unwrap();
        app_state
            .fs
            .remove_file(Path::new("/root/a/file2"), Default::default())
            .await
            .unwrap();
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );
        workspace
            .update(cx, |w, cx| Pane::go_forward(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Modify file to collapse multiple nav history entries into the same location.
        // Ensure we don't visit the same location twice when navigating.
        editor1.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(15, 0)..DisplayPoint::new(15, 0)])
            })
        });

        for _ in 0..5 {
            editor1.update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.select_display_ranges([DisplayPoint::new(3, 0)..DisplayPoint::new(3, 0)])
                });
            });
            editor1.update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.select_display_ranges([DisplayPoint::new(13, 0)..DisplayPoint::new(13, 0)])
                })
            });
        }

        editor1.update(cx, |editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.select_display_ranges([DisplayPoint::new(2, 0)..DisplayPoint::new(14, 0)])
                });
                editor.insert("", cx);
            })
        });

        editor1.update(cx, |editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.select_display_ranges([DisplayPoint::new(1, 0)..DisplayPoint::new(1, 0)])
            })
        });
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(2, 0), 0.)
        );
        workspace
            .update(cx, |w, cx| Pane::go_back(w, None, cx))
            .await;
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(3, 0), 0.)
        );

        fn active_location(
            workspace: &ViewHandle<Workspace>,
            cx: &mut TestAppContext,
        ) -> (ProjectPath, DisplayPoint, f32) {
            workspace.update(cx, |workspace, cx| {
                let item = workspace.active_item(cx).unwrap();
                let editor = item.downcast::<Editor>().unwrap();
                let (selections, scroll_position) = editor.update(cx, |editor, cx| {
                    (
                        editor.selections.display_ranges(cx),
                        editor.scroll_position(cx),
                    )
                });
                (
                    item.project_path(cx).unwrap(),
                    selections[0].start,
                    scroll_position.y(),
                )
            })
        }
    }

    #[gpui::test]
    fn test_bundled_themes(cx: &mut MutableAppContext) {
        let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());

        lazy_static::lazy_static! {
            static ref DEFAULT_THEME: parking_lot::Mutex<Option<Arc<Theme>>> = Default::default();
            static ref FONTS: Vec<Arc<Vec<u8>>> = vec![
                Assets.load("fonts/zed-sans/zed-sans-extended.ttf").unwrap().to_vec().into(),
                Assets.load("fonts/zed-mono/zed-mono-extended.ttf").unwrap().to_vec().into(),
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

    fn init(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let mut app_state = AppState::test(cx);
            let state = Arc::get_mut(&mut app_state).unwrap();
            state.initialize_workspace = initialize_workspace;
            state.build_window_options = build_window_options;
            workspace::init(app_state.clone(), cx);
            editor::init(cx);
            pane::init(cx);
            app_state
        })
    }

    fn rust_lang() -> Arc<language::Language> {
        Arc::new(language::Language::new(
            language::LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            Some(tree_sitter_rust::language()),
        ))
    }
}
