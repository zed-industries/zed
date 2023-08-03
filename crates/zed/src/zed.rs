pub mod assets;
pub mod languages;
pub mod menus;
pub mod only_instance;
#[cfg(any(test, feature = "test-support"))]
pub mod test;

use ai::AssistantPanel;
use anyhow::Context;
use assets::Assets;
use breadcrumbs::Breadcrumbs;
pub use client;
use collab_ui::{CollabTitlebarItem, ToggleContactsMenu};
use collections::VecDeque;
pub use editor;
use editor::{Editor, MultiBuffer};

use anyhow::anyhow;
use feedback::{
    feedback_info_text::FeedbackInfoText, submit_feedback_button::SubmitFeedbackButton,
};
use futures::{channel::mpsc, StreamExt};
use gpui::{
    anyhow::{self, Result},
    geometry::vector::vec2f,
    impl_actions,
    platform::{Platform, PromptLevel, TitlebarOptions, WindowBounds, WindowKind, WindowOptions},
    AppContext, AsyncAppContext, Task, ViewContext, WeakViewHandle,
};
pub use lsp;
pub use project;
use project_panel::ProjectPanel;
use search::{BufferSearchBar, ProjectSearchBar};
use serde::Deserialize;
use serde_json::to_string_pretty;
use settings::{initial_local_settings_content, KeymapFile, SettingsStore};
use std::{borrow::Cow, str, sync::Arc};
use terminal_view::terminal_panel::{self, TerminalPanel};
use util::{
    asset_str,
    channel::ReleaseChannel,
    paths::{self, LOCAL_SETTINGS_RELATIVE_PATH},
    ResultExt,
};
use uuid::Uuid;
use welcome::BaseKeymap;
pub use workspace;
use workspace::{
    create_and_open_local_file, dock::PanelHandle,
    notifications::simple_message_notification::MessageNotification, open_new, AppState, NewFile,
    NewWindow, Workspace, WorkspaceSettings,
};
use zed_actions::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct OpenBrowser {
    url: Arc<str>,
}

impl_actions!(zed, [OpenBrowser]);

pub fn init(app_state: &Arc<AppState>, cx: &mut gpui::AppContext) {
    cx.add_action(about);
    cx.add_global_action(|_: &Hide, cx: &mut gpui::AppContext| {
        cx.platform().hide();
    });
    cx.add_global_action(|_: &HideOthers, cx: &mut gpui::AppContext| {
        cx.platform().hide_other_apps();
    });
    cx.add_global_action(|_: &ShowAll, cx: &mut gpui::AppContext| {
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
                cx.defer(move |_, cx| {
                    item.update(cx, |item, cx| {
                        item.toggle_contacts_popover(&Default::default(), cx);
                    });
                });
            }
        },
    );
    cx.add_global_action(quit);
    cx.add_global_action(move |action: &OpenBrowser, cx| cx.platform().open_url(&action.url));
    cx.add_global_action(move |_: &IncreaseBufferFontSize, cx| {
        theme::adjust_font_size(cx, |size| *size += 1.0)
    });
    cx.add_global_action(move |_: &DecreaseBufferFontSize, cx| {
        theme::adjust_font_size(cx, |size| *size -= 1.0)
    });
    cx.add_global_action(move |_: &ResetBufferFontSize, cx| theme::reset_font_size(cx));
    cx.add_global_action(move |_: &install_cli::Install, cx| {
        cx.spawn(|cx| async move {
            install_cli::install_cli(&cx)
                .await
                .context("error creating CLI symlink")
        })
        .detach_and_log_err(cx);
    });
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenLog, cx: &mut ViewContext<Workspace>| {
            open_log_file(workspace, cx);
        },
    );
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenLicenses, cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                workspace,
                asset_str::<Assets>("licenses.md"),
                "Open Source License Attribution",
                "Markdown",
                cx,
            );
        },
    );
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenTelemetryLog, cx: &mut ViewContext<Workspace>| {
            open_telemetry_log_file(workspace, cx);
        },
    );
    cx.add_action(
        move |_: &mut Workspace, _: &OpenKeymap, cx: &mut ViewContext<Workspace>| {
            create_and_open_local_file(&paths::KEYMAP, cx, Default::default).detach_and_log_err(cx);
        },
    );
    cx.add_action(
        move |_: &mut Workspace, _: &OpenSettings, cx: &mut ViewContext<Workspace>| {
            create_and_open_local_file(&paths::SETTINGS, cx, || {
                settings::initial_user_settings_content().as_ref().into()
            })
            .detach_and_log_err(cx);
        },
    );
    cx.add_action(open_local_settings_file);
    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenDefaultKeymap, cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                workspace,
                settings::default_keymap(),
                "Default Key Bindings",
                "JSON",
                cx,
            );
        },
    );
    cx.add_action(
        move |workspace: &mut Workspace,
              _: &OpenDefaultSettings,
              cx: &mut ViewContext<Workspace>| {
            open_bundled_file(
                workspace,
                settings::default_settings(),
                "Default Settings",
                "JSON",
                cx,
            );
        },
    );
    cx.add_action({
        move |workspace: &mut Workspace, _: &DebugElements, cx: &mut ViewContext<Workspace>| {
            let app_state = workspace.app_state().clone();
            let markdown = app_state.languages.language_for_name("JSON");
            let window_id = cx.window_id();
            cx.spawn(|workspace, mut cx| async move {
                let markdown = markdown.await.log_err();
                let content = to_string_pretty(
                    &cx.debug_elements(window_id)
                        .ok_or_else(|| anyhow!("could not debug elements for {window_id}"))?,
                )
                .unwrap();
                workspace
                    .update(&mut cx, |workspace, cx| {
                        workspace.with_local_workspace(cx, move |workspace, cx| {
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
                    })?
                    .await
            })
            .detach_and_log_err(cx);
        }
    });
    cx.add_action(
        |workspace: &mut Workspace,
         _: &project_panel::ToggleFocus,
         cx: &mut ViewContext<Workspace>| {
            workspace.toggle_panel_focus::<ProjectPanel>(cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace,
         _: &terminal_panel::ToggleFocus,
         cx: &mut ViewContext<Workspace>| {
            workspace.toggle_panel_focus::<TerminalPanel>(cx);
        },
    );
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &NewWindow, cx: &mut AppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_new(&app_state, cx, |workspace, cx| {
                    Editor::new_file(workspace, &Default::default(), cx)
                })
                .detach();
            }
        }
    });
    cx.add_global_action({
        let app_state = Arc::downgrade(&app_state);
        move |_: &NewFile, cx: &mut AppContext| {
            if let Some(app_state) = app_state.upgrade() {
                open_new(&app_state, cx, |workspace, cx| {
                    Editor::new_file(workspace, &Default::default(), cx)
                })
                .detach();
            }
        }
    });
    load_default_keymap(cx);
}

pub fn initialize_workspace(
    workspace_handle: WeakViewHandle<Workspace>,
    was_deserialized: bool,
    app_state: Arc<AppState>,
    cx: AsyncAppContext,
) -> Task<Result<()>> {
    cx.spawn(|mut cx| async move {
        workspace_handle.update(&mut cx, |workspace, cx| {
            let workspace_handle = cx.handle();
            cx.subscribe(&workspace_handle, {
                move |workspace, _, event, cx| {
                    if let workspace::Event::PaneAdded(pane) = event {
                        pane.update(cx, |pane, cx| {
                            pane.toolbar().update(cx, |toolbar, cx| {
                                let breadcrumbs = cx.add_view(|_| Breadcrumbs::new(workspace));
                                toolbar.add_item(breadcrumbs, cx);
                                let buffer_search_bar = cx.add_view(BufferSearchBar::new);
                                toolbar.add_item(buffer_search_bar, cx);
                                let project_search_bar = cx.add_view(|_| ProjectSearchBar::new());
                                toolbar.add_item(project_search_bar, cx);
                                let submit_feedback_button =
                                    cx.add_view(|_| SubmitFeedbackButton::new());
                                toolbar.add_item(submit_feedback_button, cx);
                                let feedback_info_text = cx.add_view(|_| FeedbackInfoText::new());
                                toolbar.add_item(feedback_info_text, cx);
                                let lsp_log_item =
                                    cx.add_view(|_| language_tools::LspLogToolbarItemView::new());
                                toolbar.add_item(lsp_log_item, cx);
                                let syntax_tree_item = cx
                                    .add_view(|_| language_tools::SyntaxTreeToolbarItemView::new());
                                toolbar.add_item(syntax_tree_item, cx);
                            })
                        });
                    }
                }
            })
            .detach();

            cx.emit(workspace::Event::PaneAdded(workspace.active_pane().clone()));

            let collab_titlebar_item =
                cx.add_view(|cx| CollabTitlebarItem::new(workspace, &workspace_handle, cx));
            workspace.set_titlebar_item(collab_titlebar_item.into_any(), cx);

            let copilot =
                cx.add_view(|cx| copilot_button::CopilotButton::new(app_state.fs.clone(), cx));
            let diagnostic_summary =
                cx.add_view(|cx| diagnostics::items::DiagnosticIndicator::new(workspace, cx));
            let activity_indicator = activity_indicator::ActivityIndicator::new(
                workspace,
                app_state.languages.clone(),
                cx,
            );
            let active_buffer_language =
                cx.add_view(|_| language_selector::ActiveBufferLanguage::new(workspace));
            let vim_mode_indicator = cx.add_view(|cx| vim::ModeIndicator::new(cx));
            let feedback_button = cx.add_view(|_| {
                feedback::deploy_feedback_button::DeployFeedbackButton::new(workspace)
            });
            let cursor_position = cx.add_view(|_| editor::items::CursorPosition::new());
            workspace.status_bar().update(cx, |status_bar, cx| {
                status_bar.add_left_item(diagnostic_summary, cx);
                status_bar.add_left_item(activity_indicator, cx);

                status_bar.add_right_item(feedback_button, cx);
                status_bar.add_right_item(copilot, cx);
                status_bar.add_right_item(active_buffer_language, cx);
                status_bar.add_right_item(vim_mode_indicator, cx);
                status_bar.add_right_item(cursor_position, cx);
            });

            auto_update::notify_of_any_new_update(cx.weak_handle(), cx);

            vim::observe_keystrokes(cx);

            cx.on_window_should_close(|workspace, cx| {
                if let Some(task) = workspace.close(&Default::default(), cx) {
                    task.detach_and_log_err(cx);
                }
                false
            });
        })?;

        let project_panel = ProjectPanel::load(workspace_handle.clone(), cx.clone());
        let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
        let assistant_panel = AssistantPanel::load(workspace_handle.clone(), cx.clone());
        let (project_panel, terminal_panel, assistant_panel) =
            futures::try_join!(project_panel, terminal_panel, assistant_panel)?;

        workspace_handle.update(&mut cx, |workspace, cx| {
            let project_panel_position = project_panel.position(cx);
            workspace.add_panel_with_extra_event_handler(
                project_panel,
                cx,
                |workspace, _, event, cx| match event {
                    project_panel::Event::NewSearchInDirectory { dir_entry } => {
                        search::ProjectSearchView::new_search_in_directory(workspace, dir_entry, cx)
                    }
                    project_panel::Event::ActivatePanel => {
                        workspace.focus_panel::<ProjectPanel>(cx);
                    }
                    _ => {}
                },
            );
            workspace.add_panel(terminal_panel, cx);
            workspace.add_panel(assistant_panel, cx);

            if !was_deserialized
                && workspace
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .any(|tree| {
                        tree.read(cx)
                            .root_entry()
                            .map_or(false, |entry| entry.is_dir())
                    })
            {
                workspace.toggle_dock(project_panel_position, cx);
            }
            cx.focus_self();
        })?;
        Ok(())
    })
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
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        bounds,
        screen,
    }
}

fn quit(_: &Quit, cx: &mut gpui::AppContext) {
    let should_confirm = settings::get::<WorkspaceSettings>(cx).confirm_quit;
    cx.spawn(|mut cx| async move {
        let mut workspaces = cx
            .window_ids()
            .into_iter()
            .filter_map(|window_id| {
                Some(
                    cx.root_view(window_id)?
                        .clone()
                        .downcast::<Workspace>()?
                        .downgrade(),
                )
            })
            .collect::<Vec<_>>();

        // If multiple windows have unsaved changes, and need a save prompt,
        // prompt in the active window before switching to a different window.
        workspaces.sort_by_key(|workspace| !cx.window_is_active(workspace.window_id()));

        if let (true, Some(workspace)) = (should_confirm, workspaces.first()) {
            let answer = cx.prompt(
                workspace.window_id(),
                PromptLevel::Info,
                "Are you sure you want to quit?",
                &["Quit", "Cancel"],
            );

            if let Some(mut answer) = answer {
                let answer = answer.next().await;
                if answer != Some(0) {
                    return Ok(());
                }
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for workspace in workspaces {
            if !workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.prepare_to_close(true, cx)
                })?
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
    cx.prompt(PromptLevel::Info, &format!("{app_name} {version}"), &["OK"]);
}

fn open_log_file(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    const MAX_LINES: usize = 1000;

    workspace
        .with_local_workspace(cx, move |workspace, cx| {
            let fs = workspace.app_state().fs.clone();
            cx.spawn(|workspace, mut cx| async move {
                let (old_log, new_log) =
                    futures::join!(fs.load(&paths::OLD_LOG), fs.load(&paths::LOG));

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

                workspace
                    .update(&mut cx, |workspace, cx| {
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
                    })
                    .log_err();
            })
            .detach();
        })
        .detach();
}

pub fn load_default_keymap(cx: &mut AppContext) {
    for path in ["keymaps/default.json", "keymaps/vim.json"] {
        KeymapFile::load_asset(path, cx).unwrap();
    }

    if let Some(asset_path) = settings::get::<BaseKeymap>(cx).asset_path() {
        KeymapFile::load_asset(asset_path, cx).unwrap();
    }
}

pub fn handle_keymap_file_changes(
    mut user_keymap_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    cx.spawn(move |mut cx| async move {
        let mut settings_subscription = None;
        while let Some(user_keymap_content) = user_keymap_file_rx.next().await {
            if let Ok(keymap_content) = KeymapFile::parse(&user_keymap_content) {
                cx.update(|cx| reload_keymaps(cx, &keymap_content));

                let mut old_base_keymap = cx.read(|cx| *settings::get::<BaseKeymap>(cx));
                drop(settings_subscription);
                settings_subscription = Some(cx.update(|cx| {
                    cx.observe_global::<SettingsStore, _>(move |cx| {
                        let new_base_keymap = *settings::get::<BaseKeymap>(cx);
                        if new_base_keymap != old_base_keymap {
                            old_base_keymap = new_base_keymap.clone();
                            reload_keymaps(cx, &keymap_content);
                        }
                    })
                }));
            }
        }
    })
    .detach();
}

fn reload_keymaps(cx: &mut AppContext, keymap_content: &KeymapFile) {
    cx.clear_bindings();
    load_default_keymap(cx);
    keymap_content.clone().add_to_cx(cx).log_err();
    cx.set_menus(menus::menus());
}

fn open_local_settings_file(
    workspace: &mut Workspace,
    _: &OpenLocalSettings,
    cx: &mut ViewContext<Workspace>,
) {
    let project = workspace.project().clone();
    let worktree = project
        .read(cx)
        .visible_worktrees(cx)
        .find_map(|tree| tree.read(cx).root_entry()?.is_dir().then_some(tree));
    if let Some(worktree) = worktree {
        let tree_id = worktree.read(cx).id();
        cx.spawn(|workspace, mut cx| async move {
            let file_path = &*LOCAL_SETTINGS_RELATIVE_PATH;

            if let Some(dir_path) = file_path.parent() {
                if worktree.read_with(&cx, |tree, _| tree.entry_for_path(dir_path).is_none()) {
                    project
                        .update(&mut cx, |project, cx| {
                            project.create_entry((tree_id, dir_path), true, cx)
                        })
                        .ok_or_else(|| anyhow!("worktree was removed"))?
                        .await?;
                }
            }

            if worktree.read_with(&cx, |tree, _| tree.entry_for_path(file_path).is_none()) {
                project
                    .update(&mut cx, |project, cx| {
                        project.create_entry((tree_id, file_path), false, cx)
                    })
                    .ok_or_else(|| anyhow!("worktree was removed"))?
                    .await?;
            }

            let editor = workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.open_path((tree_id, file_path), None, true, cx)
                })?
                .await?
                .downcast::<Editor>()
                .ok_or_else(|| anyhow!("unexpected item type"))?;

            editor
                .downgrade()
                .update(&mut cx, |editor, cx| {
                    if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                        if buffer.read(cx).is_empty() {
                            buffer.update(cx, |buffer, cx| {
                                buffer.edit([(0..0, initial_local_settings_content())], None, cx)
                            });
                        }
                    }
                })
                .ok();

            anyhow::Ok(())
        })
        .detach();
    } else {
        workspace.show_notification(0, cx, |cx| {
            cx.add_view(|_| MessageNotification::new("This project has no folders open."))
        })
    }
}

fn open_telemetry_log_file(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.with_local_workspace(cx, move |workspace, cx| {
        let app_state = workspace.app_state().clone();
        cx.spawn(|workspace, mut cx| async move {
            async fn fetch_log_string(app_state: &Arc<AppState>) -> Option<String> {
                let path = app_state.client.telemetry().log_file_path()?;
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
            }).log_err()?;

            Some(())
        })
        .detach();
    }).detach();
}

fn open_bundled_file(
    workspace: &mut Workspace,
    text: Cow<'static, str>,
    title: &'static str,
    language: &'static str,
    cx: &mut ViewContext<Workspace>,
) {
    let language = workspace.app_state().languages.language_for_name(language);
    cx.spawn(|workspace, mut cx| async move {
        let language = language.await.log_err();
        workspace
            .update(&mut cx, |workspace, cx| {
                workspace.with_local_workspace(cx, |workspace, cx| {
                    let project = workspace.project();
                    let buffer = project.update(cx, move |project, cx| {
                        project
                            .create_buffer(text.as_ref(), language, cx)
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
            })?
            .await
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assets::Assets;
    use editor::{scroll::autoscroll::Autoscroll, DisplayPoint, Editor};
    use fs::{FakeFs, Fs};
    use gpui::{
        actions, elements::Empty, executor::Deterministic, Action, AnyElement, AppContext,
        AssetSource, Element, Entity, TestAppContext, View, ViewHandle,
    };
    use language::LanguageRegistry;
    use node_runtime::NodeRuntime;
    use project::{Project, ProjectPath};
    use serde_json::json;
    use settings::{handle_settings_file_changes, watch_config_file, SettingsStore};
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };
    use theme::{ThemeRegistry, ThemeSettings};
    use util::http::FakeHttpClient;
    use workspace::{
        item::{Item, ItemHandle},
        open_new, open_paths, pane, NewFile, SplitDirection, WorkspaceHandle,
    };

    #[gpui::test]
    async fn test_open_paths_action(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
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
                    "d": {
                        "da": null,
                        "db": null,
                    },
                }),
            )
            .await;

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/a"), PathBuf::from("/root/b")],
                &app_state,
                None,
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.window_ids().len(), 1);

        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, None, cx))
            .await
            .unwrap();
        assert_eq!(cx.window_ids().len(), 1);
        let workspace_1 = cx
            .read_window(cx.window_ids()[0], |cx| cx.root_view().clone())
            .unwrap()
            .downcast::<Workspace>()
            .unwrap();
        workspace_1.update(cx, |workspace, cx| {
            assert_eq!(workspace.worktrees(cx).count(), 2);
            assert!(workspace.left_dock().read(cx).is_open());
            assert!(workspace.active_pane().is_focused(cx));
        });

        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/b"), PathBuf::from("/root/c")],
                &app_state,
                None,
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.window_ids().len(), 2);

        // Replace existing windows
        let window_id = cx.window_ids()[0];
        cx.update(|cx| {
            open_paths(
                &[PathBuf::from("/root/c"), PathBuf::from("/root/d")],
                &app_state,
                Some(window_id),
                cx,
            )
        })
        .await
        .unwrap();
        assert_eq!(cx.window_ids().len(), 2);
        let workspace_1 = cx
            .read_window(cx.window_ids()[0], |cx| cx.root_view().clone())
            .unwrap()
            .clone()
            .downcast::<Workspace>()
            .unwrap();
        workspace_1.update(cx, |workspace, cx| {
            assert_eq!(
                workspace
                    .worktrees(cx)
                    .map(|w| w.read(cx).abs_path())
                    .collect::<Vec<_>>(),
                &[Path::new("/root/c").into(), Path::new("/root/d").into()]
            );
            assert!(workspace.left_dock().read(cx).is_open());
            assert!(workspace.active_pane().is_focused(cx));
        });
    }

    #[gpui::test]
    async fn test_window_edit_state(executor: Arc<Deterministic>, cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({"a": "hey"}))
            .await;

        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, None, cx))
            .await
            .unwrap();
        assert_eq!(cx.window_ids().len(), 1);

        // When opening the workspace, the window is not in a edited state.
        let workspace = cx
            .read_window(cx.window_ids()[0], |cx| cx.root_view().clone())
            .unwrap()
            .downcast::<Workspace>()
            .unwrap();
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());
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
        let close = pane.update(cx, |pane, cx| {
            drop(editor);
            pane.close_active_item(&Default::default(), cx).unwrap()
        });
        executor.run_until_parked();
        cx.simulate_prompt_answer(workspace.window_id(), 1);
        close.await.unwrap();
        assert!(!cx.is_window_edited(workspace.window_id()));

        // Opening the buffer again doesn't impact the window's edited state.
        cx.update(|cx| open_paths(&[PathBuf::from("/root/a")], &app_state, None, cx))
            .await
            .unwrap();
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
        let app_state = init_test(cx);
        cx.update(|cx| {
            open_new(&app_state, cx, |workspace, cx| {
                Editor::new_file(workspace, &Default::default(), cx)
            })
        })
        .await;

        let window_id = *cx.window_ids().first().unwrap();
        let workspace = cx
            .read_window(window_id, |cx| cx.root_view().clone())
            .unwrap()
            .downcast::<Workspace>()
            .unwrap();

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
        let app_state = init_test(cx);
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

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
                w.split_and_clone(w.active_pane().clone(), SplitDirection::Right, cx);
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
                    .project_path(cx),
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
        let app_state = init_test(cx);

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

        cx.update(|cx| open_paths(&[PathBuf::from("/dir1/")], &app_state, None, cx))
            .await
            .unwrap();
        assert_eq!(cx.window_ids().len(), 1);
        let workspace = cx
            .read_window(cx.window_ids()[0], |cx| cx.root_view().clone())
            .unwrap()
            .downcast::<Workspace>()
            .unwrap();

        #[track_caller]
        fn assert_project_panel_selection(
            workspace: &Workspace,
            expected_worktree_path: &Path,
            expected_entry_path: &Path,
            cx: &AppContext,
        ) {
            let project_panel = [
                workspace.left_dock().read(cx).panel::<ProjectPanel>(),
                workspace.right_dock().read(cx).panel::<ProjectPanel>(),
                workspace.bottom_dock().read(cx).panel::<ProjectPanel>(),
            ]
            .into_iter()
            .find_map(std::convert::identity)
            .expect("found no project panels")
            .read(cx);
            let (selected_worktree, selected_entry) = project_panel
                .selected_entry(cx)
                .expect("project panel should have a selected entry");
            assert_eq!(
                selected_worktree.abs_path().as_ref(),
                expected_worktree_path,
                "Unexpected project panel selected worktree path"
            );
            assert_eq!(
                selected_entry.path.as_ref(),
                expected_entry_path,
                "Unexpected project panel selected entry path"
            );
        }

        // Open a file within an existing worktree.
        workspace
            .update(cx, |view, cx| {
                view.open_paths(vec!["/dir1/a.txt".into()], true, cx)
            })
            .await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(workspace, Path::new("/dir1"), Path::new("a.txt"), cx);
            assert_eq!(
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "a.txt"
            );
        });

        // Open a file outside of any existing worktree.
        workspace
            .update(cx, |view, cx| {
                view.open_paths(vec!["/dir2/b.txt".into()], true, cx)
            })
            .await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(workspace, Path::new("/dir2/b.txt"), Path::new(""), cx);
            let worktree_roots = workspace
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
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "b.txt"
            );
        });

        // Ensure opening a directory and one of its children only adds one worktree.
        workspace
            .update(cx, |view, cx| {
                view.open_paths(vec!["/dir3".into(), "/dir3/c.txt".into()], true, cx)
            })
            .await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(workspace, Path::new("/dir3"), Path::new("c.txt"), cx);
            let worktree_roots = workspace
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
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "c.txt"
            );
        });

        // Ensure opening invisibly a file outside an existing worktree adds a new, invisible worktree.
        workspace
            .update(cx, |view, cx| {
                view.open_paths(vec!["/d.txt".into()], false, cx)
            })
            .await;
        cx.read(|cx| {
            let workspace = workspace.read(cx);
            assert_project_panel_selection(workspace, Path::new("/d.txt"), Path::new(""), cx);
            let worktree_roots = workspace
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
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "d.txt"
            );
        });
    }

    #[gpui::test]
    async fn test_save_conflicting_item(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "a.txt": "" }))
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        // Open a file within an existing worktree.
        workspace
            .update(cx, |view, cx| {
                view.open_paths(vec![PathBuf::from("/root/a.txt")], true, cx)
            })
            .await;
        let editor = cx.read(|cx| {
            let pane = workspace.read(cx).active_pane().read(cx);
            let item = pane.active_item().unwrap();
            item.downcast::<Editor>().unwrap()
        });

        editor.update(cx, |editor, cx| editor.handle_input("x", cx));
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
        let app_state = init_test(cx);
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
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
            assert!(editor.is_dirty(cx));
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
                workspace.split_and_clone(
                    workspace.active_pane().clone(),
                    SplitDirection::Right,
                    cx,
                );
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
        let app_state = init_test(cx);
        app_state.fs.create_dir(Path::new("/root")).await.unwrap();

        let project = Project::test(app_state.fs.clone(), [], cx).await;
        project.update(cx, |project, _| project.languages().add(rust_lang()));
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

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
            assert!(editor.is_dirty(cx));
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
        let app_state = init_test(cx);
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

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
            assert_eq!(pane2_item.project_path(cx), Some(file1.clone()));

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
            assert_eq!(workspace.panes().len(), 1);
            assert!(workspace.active_item(cx).is_none());
        });

        cx.assert_dropped(editor_1);
        cx.assert_dropped(editor_2);
        cx.assert_dropped(buffer);
    }

    #[gpui::test]
    async fn test_navigation(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));
        let pane = workspace.read_with(cx, |workspace, _| workspace.active_pane().clone());

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
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go back one more time and ensure we don't navigate past the first item in the history.
        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_forward(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_forward(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file2.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go forward to an item that has been closed, ensuring it gets re-opened at the same
        // location.
        pane.update(cx, |pane, cx| {
            let editor3_id = editor3.id();
            drop(editor3);
            pane.close_item_by_id(editor3_id, cx)
        })
        .await
        .unwrap();
        workspace
            .update(cx, |w, cx| w.go_forward(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        workspace
            .update(cx, |w, cx| w.go_forward(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(16, 0), 12.5)
        );

        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file3.clone(), DisplayPoint::new(0, 0), 0.)
        );

        // Go back to an item that has been closed and removed from disk, ensuring it gets skipped.
        pane.update(cx, |pane, cx| {
            let editor2_id = editor2.id();
            drop(editor2);
            pane.close_item_by_id(editor2_id, cx)
        })
        .await
        .unwrap();
        app_state
            .fs
            .remove_file(Path::new("/root/a/file2"), Default::default())
            .await
            .unwrap();
        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(10, 0), 0.)
        );
        workspace
            .update(cx, |w, cx| w.go_forward(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
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
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
        assert_eq!(
            active_location(&workspace, cx),
            (file1.clone(), DisplayPoint::new(2, 0), 0.)
        );
        workspace
            .update(cx, |w, cx| w.go_back(w.active_pane().downgrade(), cx))
            .await
            .unwrap();
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
        let app_state = init_test(cx);
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
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
        pane.update(cx, |pane, cx| pane.close_item_by_id(file1_item_id, cx))
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        pane.update(cx, |pane, cx| pane.close_item_by_id(file4_item_id, cx))
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        pane.update(cx, |pane, cx| pane.close_item_by_id(file2_item_id, cx))
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        pane.update(cx, |pane, cx| pane.close_item_by_id(file3_item_id, cx))
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), None);

        // Reopen all the closed items, ensuring they are reopened in the same order
        // in which they were closed.
        workspace
            .update(cx, Workspace::reopen_closed_item)
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, Workspace::reopen_closed_item)
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening past the last closed item is a no-op.
        workspace
            .update(cx, Workspace::reopen_closed_item)
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        // Reopening closed items doesn't interfere with navigation history.
        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file4.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file3.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file2.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
        assert_eq!(active_path(&workspace, cx), Some(file1.clone()));

        workspace
            .update(cx, |workspace, cx| {
                workspace.go_back(workspace.active_pane().downgrade(), cx)
            })
            .await
            .unwrap();
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
    async fn test_base_keymap(cx: &mut gpui::TestAppContext) {
        struct TestView;

        impl Entity for TestView {
            type Event = ();
        }

        impl View for TestView {
            fn ui_name() -> &'static str {
                "TestView"
            }

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
        }

        let executor = cx.background();
        let fs = FakeFs::new(executor.clone());

        actions!(test, [A, B]);
        // From the Atom keymap
        actions!(workspace, [ActivatePreviousPane]);
        // From the JetBrains keymap
        actions!(pane, [ActivatePrevItem]);

        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "Atom"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": "test::A"
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            theme::init(Assets, cx);
            welcome::init(cx);

            cx.add_global_action(|_: &A, _cx| {});
            cx.add_global_action(|_: &B, _cx| {});
            cx.add_global_action(|_: &ActivatePreviousPane, _cx| {});
            cx.add_global_action(|_: &ActivatePrevItem, _cx| {});

            let settings_rx = watch_config_file(
                executor.clone(),
                fs.clone(),
                PathBuf::from("/settings.json"),
            );
            let keymap_rx =
                watch_config_file(executor.clone(), fs.clone(), PathBuf::from("/keymap.json"));

            handle_keymap_file_changes(keymap_rx, cx);
            handle_settings_file_changes(settings_rx, cx);
        });

        cx.foreground().run_until_parked();

        let (window_id, _view) = cx.add_window(|_| TestView);

        // Test loading the keymap base at all
        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &A), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the users keymap, while retaining the base keymap
        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": "test::B"
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &B), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test modifying the base, while retaining the users keymap
        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "JetBrains"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &B), ("[", &ActivatePrevItem)],
            line!(),
        );

        #[track_caller]
        fn assert_key_bindings_for<'a>(
            window_id: usize,
            cx: &TestAppContext,
            actions: Vec<(&'static str, &'a dyn Action)>,
            line: u32,
        ) {
            for (key, action) in actions {
                // assert that...
                assert!(
                    cx.available_actions(window_id, 0)
                        .into_iter()
                        .any(|(_, bound_action, b)| {
                            // action names match...
                            bound_action.name() == action.name()
                        && bound_action.namespace() == action.namespace()
                        // and key strokes contain the given key
                        && b.iter()
                            .any(|binding| binding.keystrokes().iter().any(|k| k.key == key))
                        }),
                    "On {} Failed to find {} with key binding {}",
                    line,
                    action.name(),
                    key
                );
            }
        }
    }

    #[gpui::test]
    async fn test_disabled_keymap_binding(cx: &mut gpui::TestAppContext) {
        struct TestView;

        impl Entity for TestView {
            type Event = ();
        }

        impl View for TestView {
            fn ui_name() -> &'static str {
                "TestView"
            }

            fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
                Empty::new().into_any()
            }
        }

        let executor = cx.background();
        let fs = FakeFs::new(executor.clone());

        actions!(test, [A, B]);
        // From the Atom keymap
        actions!(workspace, [ActivatePreviousPane]);
        // From the JetBrains keymap
        actions!(pane, [ActivatePrevItem]);

        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "Atom"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": "test::A"
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.update(|cx| {
            cx.set_global(SettingsStore::test(cx));
            theme::init(Assets, cx);
            welcome::init(cx);

            cx.add_global_action(|_: &A, _cx| {});
            cx.add_global_action(|_: &B, _cx| {});
            cx.add_global_action(|_: &ActivatePreviousPane, _cx| {});
            cx.add_global_action(|_: &ActivatePrevItem, _cx| {});

            let settings_rx = watch_config_file(
                executor.clone(),
                fs.clone(),
                PathBuf::from("/settings.json"),
            );
            let keymap_rx =
                watch_config_file(executor.clone(), fs.clone(), PathBuf::from("/keymap.json"));

            handle_keymap_file_changes(keymap_rx, cx);
            handle_settings_file_changes(settings_rx, cx);
        });

        cx.foreground().run_until_parked();

        let (window_id, _view) = cx.add_window(|_| TestView);

        // Test loading the keymap base at all
        assert_key_bindings_for(
            window_id,
            cx,
            vec![("backspace", &A), ("k", &ActivatePreviousPane)],
            line!(),
        );

        // Test disabling the key binding for the base keymap
        fs.save(
            "/keymap.json".as_ref(),
            &r#"
            [
                {
                    "bindings": {
                        "backspace": null
                    }
                }
            ]
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(window_id, cx, vec![("k", &ActivatePreviousPane)], line!());

        // Test modifying the base, while retaining the users keymap
        fs.save(
            "/settings.json".as_ref(),
            &r#"
            {
                "base_keymap": "JetBrains"
            }
            "#
            .into(),
            Default::default(),
        )
        .await
        .unwrap();

        cx.foreground().run_until_parked();

        assert_key_bindings_for(window_id, cx, vec![("[", &ActivatePrevItem)], line!());

        #[track_caller]
        fn assert_key_bindings_for<'a>(
            window_id: usize,
            cx: &TestAppContext,
            actions: Vec<(&'static str, &'a dyn Action)>,
            line: u32,
        ) {
            for (key, action) in actions {
                // assert that...
                assert!(
                    cx.available_actions(window_id, 0)
                        .into_iter()
                        .any(|(_, bound_action, b)| {
                            // action names match...
                            bound_action.name() == action.name()
                        && bound_action.namespace() == action.namespace()
                        // and key strokes contain the given key
                        && b.iter()
                            .any(|binding| binding.keystrokes().iter().any(|k| k.key == key))
                        }),
                    "On {} Failed to find {} with key binding {}",
                    line,
                    action.name(),
                    key
                );
            }
        }
    }

    #[gpui::test]
    fn test_bundled_settings_and_themes(cx: &mut AppContext) {
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
        let mut settings = SettingsStore::default();
        settings
            .set_default_settings(&settings::default_settings(), cx)
            .unwrap();
        cx.set_global(settings);
        theme::init(Assets, cx);

        let mut has_default_theme = false;
        for theme_name in themes.list(false).map(|meta| meta.name) {
            let theme = themes.get(&theme_name).unwrap();
            assert_eq!(theme.meta.name, theme_name);
            if theme.meta.name == settings::get::<ThemeSettings>(cx).theme.meta.name {
                has_default_theme = true;
            }
        }
        assert!(has_default_theme);
    }

    #[gpui::test]
    fn test_bundled_languages(cx: &mut AppContext) {
        let mut languages = LanguageRegistry::test();
        languages.set_executor(cx.background().clone());
        let languages = Arc::new(languages);
        let http = FakeHttpClient::with_404_response();
        let node_runtime = NodeRuntime::instance(http);
        languages::init(languages.clone(), node_runtime);
        for name in languages.language_names() {
            languages.language_for_name(&name);
        }
        cx.foreground().run_until_parked();
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let mut app_state = AppState::test(cx);
            let state = Arc::get_mut(&mut app_state).unwrap();
            state.initialize_workspace = initialize_workspace;
            state.build_window_options = build_window_options;
            theme::init((), cx);
            audio::init((), cx);
            call::init(app_state.client.clone(), app_state.user_store.clone(), cx);
            workspace::init(app_state.clone(), cx);
            Project::init_settings(cx);
            language::init(cx);
            editor::init(cx);
            project_panel::init_settings(cx);
            pane::init(cx);
            project_panel::init((), cx);
            terminal_view::init(cx);
            ai::init(cx);
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
