pub mod languages;
pub mod menus;
#[cfg(any(test, feature = "test-support"))]
pub mod test;
use anyhow::{anyhow, Context, Result};
use assets::Assets;
use breadcrumbs::Breadcrumbs;
pub use client;
use collab_ui::{CollabTitlebarItem, ToggleContactsMenu};
use collections::VecDeque;
pub use editor;
use editor::{Editor, MultiBuffer};

use feedback::{
    feedback_info_text::FeedbackInfoText, submit_feedback_button::SubmitFeedbackButton,
};
use futures::StreamExt;
use gpui::{
    actions,
    geometry::vector::vec2f,
    impl_actions,
    platform::{WindowBounds, WindowOptions},
    AssetSource, AsyncAppContext, Platform, PromptLevel, TitlebarOptions, ViewContext, WindowKind,
};
use language::Rope;
pub use lsp;
pub use project;
use project_panel::ProjectPanel;
use search::{BufferSearchBar, ProjectSearchBar};
use serde::Deserialize;
use serde_json::to_string_pretty;
use settings::{keymap_file_json_schema, settings_file_json_schema, Settings};
use std::{borrow::Cow, env, path::Path, str, sync::Arc};
use util::{channel::ReleaseChannel, paths, ResultExt, StaffMode};
use uuid::Uuid;
pub use workspace;
use workspace::{sidebar::SidebarSide, AppState, Restart, Workspace};

#[derive(Deserialize, Clone, PartialEq)]
pub struct OpenBrowser {
    url: Arc<str>,
}

impl_actions!(zed, [OpenBrowser]);

actions!(
    zed,
    [
        About,
        Hide,
        HideOthers,
        ShowAll,
        Minimize,
        Zoom,
        ToggleFullScreen,
        Quit,
        DebugElements,
        OpenSettings,
        OpenLog,
        OpenLicenses,
        OpenTelemetryLog,
        OpenKeymap,
        OpenDefaultSettings,
        OpenDefaultKeymap,
        IncreaseBufferFontSize,
        DecreaseBufferFontSize,
        ResetBufferFontSize,
        InstallCommandLineInterface,
        ResetDatabase,
    ]
);

const MIN_FONT_SIZE: f32 = 6.0;

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::MutableAppContext) {
    cx.add_action(about);
    cx.add_global_action(|_: &Hide, cx: &mut gpui::MutableAppContext| {
        cx.platform().hide();
    });
    cx.add_global_action(|_: &HideOthers, cx: &mut gpui::MutableAppContext| {
        cx.platform().hide_other_apps();
    });
    cx.add_global_action(|_: &ShowAll, cx: &mut gpui::MutableAppContext| {
        cx.platform().unhide_other_apps();
    });
    cx.add_action(
        |_: &mut Workspace, _: &Minimize, cx: &mut ViewContext<Workspace>| {
            cx.minimize_window();
        },
    );
    cx.add_action(
        |_: &mut Workspace, _: &Zoom, cx: &mut ViewContext<Workspace>| {
            cx.zoom_window();
        },
    );
    cx.add_action(
        |_: &mut Workspace, _: &ToggleFullScreen, cx: &mut ViewContext<Workspace>| {
            cx.toggle_full_screen();
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &ToggleContactsMenu, cx: &mut ViewContext<Workspace>| {
            if let Some(item) = workspace
                .titlebar_item()
                .and_then(|item| item.downcast::<CollabTitlebarItem>())
            {
                cx.as_mut().defer(move |cx| {
                    item.update(cx, |item, cx| {
                        item.toggle_contacts_popover(&Default::default(), cx);
                    });
                });
            }
        },
    );
    cx.add_global_action(quit);
    cx.add_global_action(restart);
    cx.add_global_action(move |action: &OpenBrowser, cx| cx.platform().open_url(&action.url));
    cx.add_global_action(move |_: &IncreaseBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = (settings.buffer_font_size + 1.0).max(MIN_FONT_SIZE);
            if let Some(terminal_font_size) = settings.terminal_overrides.font_size.as_mut() {
                *terminal_font_size = (*terminal_font_size + 1.0).max(MIN_FONT_SIZE);
            }
            cx.refresh_windows();
        });
    });
    cx.add_global_action(move |_: &DecreaseBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = (settings.buffer_font_size - 1.0).max(MIN_FONT_SIZE);
            if let Some(terminal_font_size) = settings.terminal_overrides.font_size.as_mut() {
                *terminal_font_size = (*terminal_font_size - 1.0).max(MIN_FONT_SIZE);
            }
            cx.refresh_windows();
        });
    });
    cx.add_global_action(move |_: &ResetBufferFontSize, cx| {
        cx.update_global::<Settings, _, _>(|settings, cx| {
            settings.buffer_font_size = settings.default_buffer_font_size;
            settings.terminal_overrides.font_size = settings.terminal_defaults.font_size;
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
            open_config_file(&paths::SETTINGS, app_state.clone(), cx, || {
                str::from_utf8(
                    Assets
                        .load("settings/initial_user_settings.json")
                        .unwrap()
                        .as_ref(),
                )
                .unwrap()
                .into()
            });
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |workspace: &mut Workspace, _: &OpenLog, cx: &mut ViewContext<Workspace>| {
            open_log_file(workspace, app_state.clone(), cx);
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenLicenses, cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                app_state.clone(),
                "licenses.md",
                "Open Source License Attribution",
                "Markdown",
                cx,
            );
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |workspace: &mut Workspace, _: &OpenTelemetryLog, cx: &mut ViewContext<Workspace>| {
            open_telemetry_log_file(workspace, app_state.clone(), cx);
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenKeymap, cx: &mut ViewContext<Workspace>| {
            open_config_file(&paths::KEYMAP, app_state.clone(), cx, Default::default);
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenDefaultKeymap, cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                app_state.clone(),
                "keymaps/default.json",
                "Default Key Bindings",
                "JSON",
                cx,
            );
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &OpenDefaultSettings, cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                app_state.clone(),
                "settings/default.json",
                "Default Settings",
                "JSON",
                cx,
            );
        }
    });
    cx.add_action({
        let app_state = app_state.clone();
        move |_: &mut Workspace, _: &DebugElements, cx: &mut ViewContext<Workspace>| {
            let app_state = app_state.clone();
            let markdown = app_state.languages.language_for_name("JSON");
            let content = to_string_pretty(&cx.debug_elements()).unwrap();
            cx.spawn(|workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.with_local_workspace(&app_state, cx, move |workspace, cx| {
                            let project = workspace.project().clone();

                            let buffer = project
                                .update(cx, |project, cx| {
                                    project.create_buffer(&content, markdown, cx)
                                })
                                .expect("creating buffers on a local workspace always succeeds");
                            let buffer = cx.add_model(|cx| {
                                MultiBuffer::singleton(buffer, cx)
                                    .with_title("Debug Elements".into())
                            });
                            workspace.add_item(
                                Box::new(cx.add_view(|cx| {
                                    Editor::for_multibuffer(buffer, Some(project.clone()), cx)
                                })),
                                cx,
                            );
                        })
                    })
                    .await;
            })
            .detach();
        }
    });
    cx.add_action(
        |workspace: &mut Workspace,
         _: &project_panel::ToggleFocus,
         cx: &mut ViewContext<Workspace>| {
            workspace.toggle_sidebar_item_focus(SidebarSide::Left, 0, cx);
        },
    );

    activity_indicator::init(cx);
    call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
    settings::KeymapFileContent::load_defaults(cx);
}

pub fn initialize_workspace(
    workspace: &mut Workspace,
    app_state: &Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) {
    let workspace_handle = cx.handle();
    cx.subscribe(&workspace_handle, {
        move |_, _, event, cx| {
            if let workspace::Event::PaneAdded(pane) = event {
                pane.update(cx, |pane, cx| {
                    pane.toolbar().update(cx, |toolbar, cx| {
                        let breadcrumbs = cx.add_view(|_| Breadcrumbs::new());
                        toolbar.add_item(breadcrumbs, cx);
                        let buffer_search_bar = cx.add_view(BufferSearchBar::new);
                        toolbar.add_item(buffer_search_bar, cx);
                        let project_search_bar = cx.add_view(|_| ProjectSearchBar::new());
                        toolbar.add_item(project_search_bar, cx);
                        let submit_feedback_button = cx.add_view(|_| SubmitFeedbackButton::new());
                        toolbar.add_item(submit_feedback_button, cx);
                        let feedback_info_text = cx.add_view(|_| FeedbackInfoText::new());
                        toolbar.add_item(feedback_info_text, cx);
                    })
                });
            }
        }
    })
    .detach();

    cx.emit(workspace::Event::PaneAdded(workspace.active_pane().clone()));
    cx.emit(workspace::Event::PaneAdded(workspace.dock_pane().clone()));

    let theme_names = app_state
        .themes
        .list(**cx.default_global::<StaffMode>())
        .map(|meta| meta.name)
        .collect();
    let language_names = app_state.languages.language_names();

    workspace.project().update(cx, |project, cx| {
        let action_names = cx.all_action_names().collect::<Vec<_>>();
        project.set_language_server_settings(serde_json::json!({
            "json": {
                "format": {
                    "enable": true,
                },
                "schemas": [
                    {
                        "fileMatch": [schema_file_match(&paths::SETTINGS)],
                        "schema": settings_file_json_schema(theme_names, &language_names),
                    },
                    {
                        "fileMatch": [schema_file_match(&paths::KEYMAP)],
                        "schema": keymap_file_json_schema(&action_names),
                    }
                ]
            }
        }));
    });

    let collab_titlebar_item =
        cx.add_view(|cx| CollabTitlebarItem::new(&workspace_handle, &app_state.user_store, cx));
    workspace.set_titlebar_item(collab_titlebar_item, cx);

    let project_panel = ProjectPanel::new(workspace.project().clone(), cx);
    workspace.left_sidebar().update(cx, |sidebar, cx| {
        sidebar.add_item(
            "icons/folder_tree_16.svg",
            "Project Panel".to_string(),
            project_panel,
            cx,
        )
    });

    let diagnostic_summary =
        cx.add_view(|cx| diagnostics::items::DiagnosticIndicator::new(workspace.project(), cx));
    let activity_indicator =
        activity_indicator::ActivityIndicator::new(workspace, app_state.languages.clone(), cx);
    let active_buffer_language = cx.add_view(|_| language_selector::ActiveBufferLanguage::new());
    let feedback_button =
        cx.add_view(|_| feedback::deploy_feedback_button::DeployFeedbackButton::new());
    let cursor_position = cx.add_view(|_| editor::items::CursorPosition::new());
    workspace.status_bar().update(cx, |status_bar, cx| {
        status_bar.add_left_item(diagnostic_summary, cx);
        status_bar.add_left_item(activity_indicator, cx);
        status_bar.add_right_item(feedback_button, cx);
        status_bar.add_right_item(active_buffer_language, cx);
        status_bar.add_right_item(cursor_position, cx);
    });

    auto_update::notify_of_any_new_update(cx.weak_handle(), cx);

    let window_id = cx.window_id();
    vim::observe_keystrokes(window_id, cx);

    cx.on_window_should_close(|workspace, cx| {
        if let Some(task) = workspace.close(&Default::default(), cx) {
            task.detach_and_log_err(cx);
        }
        false
    });
}

pub fn build_window_options(
    bounds: Option<WindowBounds>,
    display: Option<Uuid>,
    platform: &dyn Platform,
) -> WindowOptions<'static> {
    let bounds = bounds.unwrap_or(WindowBounds::Maximized);
    let screen = display.and_then(|display| platform.screen_by_id(display));

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(vec2f(8., 8.)),
        }),
        center: false,
        focus: true,
        kind: WindowKind::Normal,
        is_movable: true,
        bounds,
        screen,
    }
}

fn restart(_: &Restart, cx: &mut gpui::MutableAppContext) {
    let mut workspaces = cx
        .window_ids()
        .filter_map(|window_id| cx.root_view::<Workspace>(window_id))
        .collect::<Vec<_>>();

    // If multiple windows have unsaved changes, and need a save prompt,
    // prompt in the active window before switching to a different window.
    workspaces.sort_by_key(|workspace| !cx.window_is_active(workspace.window_id()));

    let should_confirm = cx.global::<Settings>().confirm_quit;
    cx.spawn(|mut cx| async move {
        if let (true, Some(workspace)) = (should_confirm, workspaces.first()) {
            let answer = cx
                .prompt(
                    workspace.window_id(),
                    PromptLevel::Info,
                    "Are you sure you want to restart?",
                    &["Restart", "Cancel"],
                )
                .next()
                .await;
            if answer != Some(0) {
                return Ok(());
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for workspace in workspaces {
            if !workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.prepare_to_close(true, cx)
                })
                .await?
            {
                return Ok(());
            }
        }
        cx.platform().restart();
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    let mut workspaces = cx
        .window_ids()
        .filter_map(|window_id| cx.root_view::<Workspace>(window_id))
        .collect::<Vec<_>>();

    // If multiple windows have unsaved changes, and need a save prompt,
    // prompt in the active window before switching to a different window.
    workspaces.sort_by_key(|workspace| !cx.window_is_active(workspace.window_id()));

    let should_confirm = cx.global::<Settings>().confirm_quit;
    cx.spawn(|mut cx| async move {
        if let (true, Some(workspace)) = (should_confirm, workspaces.first()) {
            let answer = cx
                .prompt(
                    workspace.window_id(),
                    PromptLevel::Info,
                    "Are you sure you want to quit?",
                    &["Quit", "Cancel"],
                )
                .next()
                .await;
            if answer != Some(0) {
                return Ok(());
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for workspace in workspaces {
            if !workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.prepare_to_close(true, cx)
                })
                .await?
            {
                return Ok(());
            }
        }
        cx.platform().quit();
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

fn about(_: &mut Workspace, _: &About, cx: &mut gpui::ViewContext<Workspace>) {
    let app_name = cx.global::<ReleaseChannel>().display_name();
    let version = env!("CARGO_PKG_VERSION");
    cx.prompt(
        gpui::PromptLevel::Info,
        &format!("{app_name} {version}"),
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
    default_content: impl 'static + Send + FnOnce() -> Rope,
) {
    cx.spawn(|workspace, mut cx| async move {
        let fs = &app_state.fs;
        if !fs.is_file(path).await {
            fs.create_file(path, Default::default()).await?;
            fs.save(path, &default_content(), Default::default())
                .await?;
        }

        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.with_local_workspace(&app_state, cx, |workspace, cx| {
                    workspace.open_paths(vec![path.to_path_buf()], false, cx)
                })
            })
            .await
            .await;
        Ok::<_, anyhow::Error>(())
    })
    .detach_and_log_err(cx)
}

fn open_log_file(
    workspace: &mut Workspace,
    app_state: Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) {
    const MAX_LINES: usize = 1000;

    workspace
        .with_local_workspace(&app_state.clone(), cx, move |_, cx| {
            cx.spawn_weak(|workspace, mut cx| async move {
                let (old_log, new_log) = futures::join!(
                    app_state.fs.load(&paths::OLD_LOG),
                    app_state.fs.load(&paths::LOG)
                );

                if let Some(workspace) = workspace.upgrade(&cx) {
                    let mut lines = VecDeque::with_capacity(MAX_LINES);
                    for line in old_log
                        .iter()
                        .flat_map(|log| log.lines())
                        .chain(new_log.iter().flat_map(|log| log.lines()))
                    {
                        if lines.len() == MAX_LINES {
                            lines.pop_front();
                        }
                        lines.push_back(line);
                    }
                    let log = lines
                        .into_iter()
                        .flat_map(|line| [line, "\n"])
                        .collect::<String>();

                    workspace.update(&mut cx, |workspace, cx| {
                        let project = workspace.project().clone();
                        let buffer = project
                            .update(cx, |project, cx| project.create_buffer("", None, cx))
                            .expect("creating buffers on a local workspace always succeeds");
                        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, log)], None, cx));

                        let buffer = cx.add_model(|cx| {
                            MultiBuffer::singleton(buffer, cx).with_title("Log".into())
                        });
                        workspace.add_item(
                            Box::new(
                                cx.add_view(|cx| {
                                    Editor::for_multibuffer(buffer, Some(project), cx)
                                }),
                            ),
                            cx,
                        );
                    });
                }
            })
            .detach();
        })
        .detach();
}

fn open_telemetry_log_file(
    workspace: &mut Workspace,
    app_state: Arc<AppState>,
    cx: &mut ViewContext<Workspace>,
) {
    workspace.with_local_workspace(&app_state.clone(), cx, move |_, cx| {
        cx.spawn_weak(|workspace, mut cx| async move {
            let workspace = workspace.upgrade(&cx)?;

            async fn fetch_log_string(app_state: &Arc<AppState>) -> Option<String> {
                let path = app_state.client.telemetry_log_file_path()?;
                app_state.fs.load(&path).await.log_err()
            }

            let log = fetch_log_string(&app_state).await.unwrap_or_else(|| "// No data has been collected yet".to_string());

            const MAX_TELEMETRY_LOG_LEN: usize = 5 * 1024 * 1024;
            let mut start_offset = log.len().saturating_sub(MAX_TELEMETRY_LOG_LEN);
            if let Some(newline_offset) = log[start_offset..].find('\n') {
                start_offset += newline_offset + 1;
            }
            let log_suffix = &log[start_offset..];
            let json = app_state.languages.language_for_name("JSON").await.log_err();

            workspace.update(&mut cx, |workspace, cx| {
                let project = workspace.project().clone();
                let buffer = project
                    .update(cx, |project, cx| project.create_buffer("", None, cx))
                    .expect("creating buffers on a local workspace always succeeds");
                buffer.update(cx, |buffer, cx| {
                    buffer.set_language(json, cx);
                    buffer.edit(
                        [(
                            0..0,
                            concat!(
                                "// Zed collects anonymous usage data to help us understand how people are using the app.\n",
                                "// Telemetry can be disabled via the `settings.json` file.\n",
                                "// Here is the data that has been reported for the current session:\n",
                                "\n"
                            ),
                        )],
                        None,
                        cx,
                    );
                    buffer.edit([(buffer.len()..buffer.len(), log_suffix)], None, cx);
                });

                let buffer = cx.add_model(|cx| {
                    MultiBuffer::singleton(buffer, cx).with_title("Telemetry Log".into())
                });
                workspace.add_item(
                    Box::new(cx.add_view(|cx| Editor::for_multibuffer(buffer, Some(project), cx))),
                    cx,
                );
            });

            Some(())
        })
        .detach();
    }).detach();
}

fn open_bundled_file(
    app_state: Arc<AppState>,
    asset_path: &'static str,
    title: &'static str,
    language: &'static str,
    cx: &mut ViewContext<Workspace>,
) {
    let language = app_state.languages.language_for_name(language);
    cx.spawn(|workspace, mut cx| async move {
        let language = language.await.log_err();
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.with_local_workspace(&app_state, cx, |workspace, cx| {
                    let project = workspace.project();
                    let buffer = project.update(cx, |project, cx| {
                        let text = Assets::get(asset_path)
                            .map(|f| f.data)
                            .unwrap_or_else(|| Cow::Borrowed(b"File not found"));
                        let text = str::from_utf8(text.as_ref()).unwrap();
                        project
                            .create_buffer(text, language, cx)
                            .expect("creating buffers on a local workspace always succeeds")
                    });
                    let buffer = cx.add_model(|cx| {
                        MultiBuffer::singleton(buffer, cx).with_title(title.into())
                    });
                    workspace.add_item(
                        Box::new(cx.add_view(|cx| {
                            Editor::for_multibuffer(buffer, Some(project.clone()), cx)
                        })),
                        cx,
                    );
                })
            })
            .await;
    })
    .detach();
}

fn schema_file_match(path: &Path) -> &Path {
    path.strip_prefix(path.parent().unwrap().parent().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assets::Assets;
    use editor::{scroll::autoscroll::Autoscroll, DisplayPoint, Editor};
    use gpui::{
        executor::Deterministic, AssetSource, MutableAppContext, Task, TestAppContext, ViewHandle,
    };
    use language::LanguageRegistry;
    use project::{Project, ProjectPath};
    use serde_json::json;
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };
    use theme::ThemeRegistry;
    use workspace::{
        item::{Item, ItemHandle},
        open_new, open_paths, pane, NewFile, Pane, SplitDirection, WorkspaceHandle,
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
            assert!(workspace.left_sidebar().read(cx).is_open());
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
    async fn test_window_edit_state(executor: Arc<Deterministic>, cx: &mut TestAppContext) {
        let app_state = init(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({"a": "hey"}))
            .await;

        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, cx))
            .await;
        assert_eq!(cx.window_ids().len(), 1);

        // When opening the workspace, the window is not in a edited state.
        let workspace = cx.root_view::<Workspace>(cx.window_ids()[0]).unwrap();
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        assert!(!cx.is_window_edited(workspace.window_id()));

        // Editing a buffer marks the window as edited.
        editor.update(cx, |editor, cx| editor.insert("EDIT", cx));
        assert!(cx.is_window_edited(workspace.window_id()));

        // Undoing the edit restores the window's edited state.
        editor.update(cx, |editor, cx| editor.undo(&Default::default(), cx));
        assert!(!cx.is_window_edited(workspace.window_id()));

        // Redoing the edit marks the window as edited again.
        editor.update(cx, |editor, cx| editor.redo(&Default::default(), cx));
        assert!(cx.is_window_edited(workspace.window_id()));

        // Closing the item restores the window's edited state.
        let close = workspace.update(cx, |workspace, cx| {
            drop(editor);
            Pane::close_active_item(workspace, &Default::default(), cx).unwrap()
        });
        executor.run_until_parked();
        cx.simulate_prompt_answer(workspace.window_id(), 1);
        close.await.unwrap();
        assert!(!cx.is_window_edited(workspace.window_id()));

        // Opening the buffer again doesn't impact the window's edited state.
        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, cx))
            .await;
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace
                .active_item(cx)
                .unwrap()
                .downcast::<Editor>()
                .unwrap()
        });
        assert!(!cx.is_window_edited(workspace.window_id()));

        // Editing the buffer marks the window as edited.
        editor.update(cx, |editor, cx| editor.insert("EDIT", cx));
        assert!(cx.is_window_edited(workspace.window_id()));

        // Ensure closing the window via the mouse gets preempted due to the
        // buffer having unsaved changes.
        assert!(!cx.simulate_window_close(workspace.window_id()));
        executor.run_until_parked();
        assert_eq!(cx.window_ids().len(), 1);

        // The window is successfully closed after the user dismisses the prompt.
        cx.simulate_prompt_answer(workspace.window_id(), 1);
        executor.run_until_parked();
        assert_eq!(cx.window_ids().len(), 0);
    }

    #[gpui::test]
    async fn test_new_empty_workspace(cx: &mut TestAppContext) {
        let app_state = init(cx);
        cx.update(|cx| open_new(&app_state, cx)).await;

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
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();
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
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        // Open the first entry
        let entry_1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), None, true, cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items_len(), 1);
        });

        // Open the second entry
        workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), None, true, cx))
            .await
            .unwrap();
        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file2.clone())
            );
            assert_eq!(pane.items_len(), 2);
        });

        // Open the first entry again. The existing pane item is activated.
        let entry_1b = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), None, true, cx))
            .await
            .unwrap();
        assert_eq!(entry_1.id(), entry_1b.id());

        cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            assert_eq!(
                pane.active_item().unwrap().project_path(cx),
                Some(file1.clone())
            );
            assert_eq!(pane.items_len(), 2);
        });

        // Split the pane with the first entry, then open the second entry again.
        workspace
            .update(cx, |w, cx| {
                w.split_pane(w.active_pane().clone(), SplitDirection::Right, cx);
                w.open_path(file2.clone(), None, true, cx)
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
                w.open_path(file3.clone(), None, true, cx),
                w.open_path(file3.clone(), None, true, cx),
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

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/",
                json!({
                    "dir1": {
                        "a.txt": ""
                    },
                    "dir2": {
                        "b.txt": ""
                    },
                    "dir3": {
                        "c.txt": ""
                    },
                    "d.txt": ""
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/dir1".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec!["/dir1/a.txt".into()], true, cx)
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
                view.open_paths(vec!["/dir2/b.txt".into()], true, cx)
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
                view.open_paths(vec!["/dir3".into(), "/dir3/c.txt".into()], true, cx)
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

        // Ensure opening invisibly a file outside an existing worktree adds a new, invisible worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec!["/d.txt".into()], false, cx)
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
                vec!["/dir1", "/dir2/b.txt", "/dir3", "/d.txt"]
                    .into_iter()
                    .map(Path::new)
                    .collect(),
            );

            let visible_worktree_roots = workspace
                .read(cx)
                .visible_worktrees(cx)
                .map(|w| w.read(cx).as_local().unwrap().abs_path().as_ref())
                .collect::<HashSet<_>>();
            assert_eq!(
                visible_worktree_roots,
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
                "d.txt"
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
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });

        // Open a file within an existing worktree.
        cx.update(|cx| {
            workspace.update(cx, |view, cx| {
                view.open_paths(vec![PathBuf::from("/root/a.txt")], true, cx)
            })
        })
        .await;
        let editor = cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            let item = pane.active_item().unwrap();
            item.downcast::<Editor>().unwrap()
        });

        cx.update(|cx| editor.update(cx, |editor, cx| editor.handle_input("x", cx)));
        app_state
            .fs
            .as_fake()
            .insert_file("/root/a.txt", "changed".to_string())
            .await;
        editor
            .condition(cx, |editor, cx| editor.has_conflict(cx))
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
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });
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
                &editor.language_at(0, cx).unwrap(),
                &languages::PLAIN_TEXT
            ));
            editor.handle_input("hi", cx);
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
            editor.handle_input(" there", cx);
            assert!(editor.is_dirty(cx.as_ref()));
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
                workspace.open_path((worktree.read(cx).id(), "the-new-name.rs"), None, true, cx)
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
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });

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
                &editor.language_at(0, cx).unwrap(),
                &languages::PLAIN_TEXT
            ));
            editor.handle_input("hi", cx);
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
        let (window_id, workspace) = cx.add_window(|cx| {
            Workspace::new(Default::default(), 0, project, |_, _| unimplemented!(), cx)
        });

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();

        let pane_1 = cx.read(|cx| workspace.read(cx).active_pane().clone());

        workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), None, true, cx))
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

        cx.dispatch_action(window_id, pane::SplitRight);
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
            assert_eq!(workspace.panes().len(), 2); //Center pane + Dock pane
            assert_eq!(workspace.active_pane(), &pane_1);
        });

        cx.dispatch_action(window_id, workspace::CloseActiveItem);
        cx.foreground().run_until_parked();
        cx.simulate_prompt_answer(window_id, 1);
        cx.foreground().run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert_eq!(workspace.panes().len(), 2);
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
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| unimplemented!(),
                cx,
            )
        });

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();

        let editor1 = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), None, true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        editor1.update(cx, |editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_display_ranges([DisplayPoint::new(10, 0)..DisplayPoint::new(10, 0)])
            });
        });
        let editor2 = workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), None, true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let editor3 = workspace
            .update(cx, |w, cx| w.open_path(file3.clone(), None, true, cx))
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        editor3
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
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
    async fn test_reopening_closed_items(cx: &mut TestAppContext) {
        let app_state = init(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "file1": "",
                        "file2": "",
                        "file3": "",
                        "file4": "",
                    },
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (_, workspace) = cx.add_window(|cx| {
            Workspace::new(
                Default::default(),
                0,
                project.clone(),
                |_, _| unimplemented!(),
                cx,
            )
        });
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

        let entries = cx.read(|cx| workspace.file_project_paths(cx));
        let file1 = entries[0].clone();
        let file2 = entries[1].clone();
        let file3 = entries[2].clone();
        let file4 = entries[3].clone();

        let file1_item_id = workspace
            .update(cx, |w, cx| w.open_path(file1.clone(), None, true, cx))
            .await
            .unwrap()
            .id();
        let file2_item_id = workspace
            .update(cx, |w, cx| w.open_path(file2.clone(), None, true, cx))
            .await
            .unwrap()
            .id();
        let file3_item_id = workspace
            .update(cx, |w, cx| w.open_path(file3.clone(), None, true, cx))
            .await
            .unwrap()
            .id();
        let file4_item_id = workspace
            .update(cx, |w, cx| w.open_path(file4.clone(), None, true, cx))
            .await
            .unwrap()
            .id();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        // Close all the pane items in some arbitrary order.
        workspace
            .update(cx, |workspace, cx| {
                Pane::close_item(workspace, pane.clone(), file1_item_id, cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, cx| {
                Pane::close_item(workspace, pane.clone(), file4_item_id, cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| {
                Pane::close_item(workspace, pane.clone(), file2_item_id, cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| {
                Pane::close_item(workspace, pane.clone(), file3_item_id, cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), None);

        // Reopen all the closed items, ensuring they are reopened in the same order
        // in which they were closed.
        workspace.update(cx, Pane::reopen_closed_item).await;
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace.update(cx, Pane::reopen_closed_item).await;
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace.update(cx, Pane::reopen_closed_item).await;
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace.update(cx, Pane::reopen_closed_item).await;
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening past the last closed item is a no-op.
        workspace.update(cx, Pane::reopen_closed_item).await;
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening closed items doesn't interfere with navigation history.
        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        workspace
            .update(cx, |workspace, cx| Pane::go_back(workspace, None, cx))
            .await;
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        fn active_path(
            workspace: &ViewHandle<Workspace>,
            cx: &TestAppContext,
        ) -> Option<ProjectPath> {
            workspace.read_with(cx, |workspace, cx| {
                let item = workspace.active_item(cx)?;
                item.project_path(cx)
            })
        }
    }

    #[gpui::test]
    fn test_bundled_settings_and_themes(cx: &mut MutableAppContext) {
        cx.platform()
            .fonts()
            .add_fonts(&[
                Assets
                    .load("fonts/zed-sans/zed-sans-extended.ttf")
                    .unwrap()
                    .to_vec()
                    .into(),
                Assets
                    .load("fonts/zed-mono/zed-mono-extended.ttf")
                    .unwrap()
                    .to_vec()
                    .into(),
            ])
            .unwrap();
        let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
        let settings = Settings::defaults(Assets, cx.font_cache(), &themes);

        let mut has_default_theme = false;
        for theme_name in themes.list(false).map(|meta| meta.name) {
            let theme = themes.get(&theme_name).unwrap();
            if theme.meta.name == settings.theme.meta.name {
                has_default_theme = true;
            }
            assert_eq!(theme.meta.name, theme_name);
        }
        assert!(has_default_theme);
    }

    #[gpui::test]
    fn test_bundled_languages(cx: &mut MutableAppContext) {
        let mut languages = LanguageRegistry::new(Task::ready(()));
        languages.set_executor(cx.background().clone());
        let languages = Arc::new(languages);
        languages::init(languages.clone());
        for name in languages.language_names() {
            languages.language_for_name(&name);
        }
        cx.foreground().run_until_parked();
    }

    fn init(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let mut app_state = AppState::test(cx);
            let state = Arc::get_mut(&mut app_state).unwrap();
            state.initialize_workspace = initialize_workspace;
            state.build_window_options = build_window_options;
            call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
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
