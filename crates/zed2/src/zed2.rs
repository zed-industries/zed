#![allow(unused_variables, unused_mut)]
//todo!()

mod assets;
pub mod languages;
mod only_instance;
mod open_listener;

pub use assets::*;
use breadcrumbs::Breadcrumbs;
use collections::VecDeque;
use editor::{Editor, MultiBuffer};
use gpui::{
    actions, point, px, AppContext, Context, FocusableView, PromptLevel, TitlebarOptions,
    ViewContext, VisualContext, WindowBounds, WindowKind, WindowOptions,
};
pub use only_instance::*;
pub use open_listener::*;

use anyhow::{anyhow, Context as _};
use project_panel::ProjectPanel;
use settings::{initial_local_settings_content, Settings};
use std::{borrow::Cow, ops::Deref, sync::Arc};
use terminal_view::terminal_panel::TerminalPanel;
use util::{
    asset_str,
    channel::{AppCommitSha, ReleaseChannel},
    paths::{self, LOCAL_SETTINGS_RELATIVE_PATH},
    ResultExt,
};
use uuid::Uuid;
use workspace::{
    create_and_open_local_file, dock::PanelHandle,
    notifications::simple_message_notification::MessageNotification, open_new, AppState, NewFile,
    NewWindow, Workspace, WorkspaceSettings,
};
use zed_actions::{OpenBrowser, OpenZedURL};

actions!(
    About,
    DebugElements,
    DecreaseBufferFontSize,
    Hide,
    HideOthers,
    IncreaseBufferFontSize,
    Minimize,
    OpenDefaultKeymap,
    OpenDefaultSettings,
    OpenKeymap,
    OpenLicenses,
    OpenLocalSettings,
    OpenLog,
    OpenSettings,
    OpenTelemetryLog,
    Quit,
    ResetBufferFontSize,
    ResetDatabase,
    ShowAll,
    ToggleFullScreen,
    Zoom,
);

pub fn build_window_options(
    bounds: Option<WindowBounds>,
    display_uuid: Option<Uuid>,
    cx: &mut AppContext,
) -> WindowOptions {
    let bounds = bounds.unwrap_or(WindowBounds::Maximized);
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    WindowOptions {
        bounds,
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(8.), px(8.))),
        }),
        center: false,
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
    }
}

pub fn initialize_workspace(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, cx| {
        let workspace_handle = cx.view().clone();
        cx.subscribe(&workspace_handle, {
            move |workspace, _, event, cx| {
                if let workspace::Event::PaneAdded(pane) = event {
                    pane.update(cx, |pane, cx| {
                        pane.toolbar().update(cx, |toolbar, cx| {
                            let breadcrumbs = cx.build_view(|_| Breadcrumbs::new(workspace));
                            toolbar.add_item(breadcrumbs, cx);
                            let buffer_search_bar = cx.build_view(search::BufferSearchBar::new);
                            toolbar.add_item(buffer_search_bar.clone(), cx);
                            // todo!()
                            //     let quick_action_bar = cx.add_view(|_| {
                            //         QuickActionBar::new(buffer_search_bar, workspace)
                            //     });
                            //     toolbar.add_item(quick_action_bar, cx);
                            let diagnostic_editor_controls =
                                cx.build_view(|_| diagnostics::ToolbarControls::new());
                            //     toolbar.add_item(diagnostic_editor_controls, cx);
                            //     let project_search_bar = cx.add_view(|_| ProjectSearchBar::new());
                            //     toolbar.add_item(project_search_bar, cx);
                            //     let submit_feedback_button =
                            //         cx.add_view(|_| SubmitFeedbackButton::new());
                            //     toolbar.add_item(submit_feedback_button, cx);
                            //     let feedback_info_text = cx.add_view(|_| FeedbackInfoText::new());
                            //     toolbar.add_item(feedback_info_text, cx);
                            //     let lsp_log_item =
                            //         cx.add_view(|_| language_tools::LspLogToolbarItemView::new());
                            //     toolbar.add_item(lsp_log_item, cx);
                            //     let syntax_tree_item = cx
                            //         .add_view(|_| language_tools::SyntaxTreeToolbarItemView::new());
                            //     toolbar.add_item(syntax_tree_item, cx);
                        })
                    });
                }
            }
        })
        .detach();

        //     cx.emit(workspace2::Event::PaneAdded(
        //         workspace.active_pane().clone(),
        //     ));

        //     let collab_titlebar_item =
        //         cx.add_view(|cx| CollabTitlebarItem::new(workspace, &workspace_handle, cx));
        //     workspace.set_titlebar_item(collab_titlebar_item.into_any(), cx);

        //     let copilot =
        //         cx.add_view(|cx| copilot_button::CopilotButton::new(app_state.fs.clone(), cx));
        let diagnostic_summary =
            cx.build_view(|cx| diagnostics::items::DiagnosticIndicator::new(workspace, cx));
        //     let activity_indicator = activity_indicator::ActivityIndicator::new(
        //         workspace,
        //         app_state.languages.clone(),
        //         cx,
        //     );
        //     let active_buffer_language =
        //         cx.add_view(|_| language_selector::ActiveBufferLanguage::new(workspace));
        //     let vim_mode_indicator = cx.add_view(|cx| vim::ModeIndicator::new(cx));
        //     let feedback_button = cx.add_view(|_| {
        //         feedback::deploy_feedback_button::DeployFeedbackButton::new(workspace)
        //     });
        //     let cursor_position = cx.add_view(|_| editor::items::CursorPosition::new());
        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_left_item(diagnostic_summary, cx);
            // status_bar.add_left_item(activity_indicator, cx);

            // status_bar.add_right_item(feedback_button, cx);
            // status_bar.add_right_item(copilot, cx);
            // status_bar.add_right_item(active_buffer_language, cx);
            // status_bar.add_right_item(vim_mode_indicator, cx);
            // status_bar.add_right_item(cursor_position, cx);
        });

        auto_update::notify_of_any_new_update(cx);

        //     vim::observe_keystrokes(cx);

        //     cx.on_window_should_close(|workspace, cx| {
        //         if let Some(task) = workspace.close(&Default::default(), cx) {
        //             task.detach_and_log_err(cx);
        //         }
        //         false
        //     });

        cx.spawn(|workspace_handle, mut cx| async move {
            let project_panel = ProjectPanel::load(workspace_handle.clone(), cx.clone());
            let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
            // let assistant_panel = AssistantPanel::load(workspace_handle.clone(), cx.clone());
            let channels_panel =
                collab_ui::collab_panel::CollabPanel::load(workspace_handle.clone(), cx.clone());
            // let chat_panel =
            //     collab_ui::chat_panel::ChatPanel::load(workspace_handle.clone(), cx.clone());
            // let notification_panel = collab_ui::notification_panel::NotificationPanel::load(
            //     workspace_handle.clone(),
            //     cx.clone(),
            // );
            let (
                project_panel,
                terminal_panel,
                //     assistant_panel,
                channels_panel,
                //     chat_panel,
                //     notification_panel,
            ) = futures::try_join!(
                project_panel,
                terminal_panel,
                //     assistant_panel,
                channels_panel,
                //     chat_panel,
                //     notification_panel,
            )?;

            workspace_handle.update(&mut cx, |workspace, cx| {
                let project_panel_position = project_panel.position(cx);
                workspace.add_panel(project_panel, cx);
                workspace.add_panel(terminal_panel, cx);
                //     workspace.add_panel(assistant_panel, cx);
                workspace.add_panel(channels_panel, cx);
                //     workspace.add_panel(chat_panel, cx);
                //     workspace.add_panel(notification_panel, cx);

                //     if !was_deserialized
                //         && workspace
                //             .project()
                //             .read(cx)
                //             .visible_worktrees(cx)
                //             .any(|tree| {
                //                 tree.read(cx)
                //                     .root_entry()
                //                     .map_or(false, |entry| entry.is_dir())
                //             })
                //     {
                // workspace.toggle_dock(project_panel_position, cx);
                //     }
                // cx.focus_self();
            })
        })
        .detach();

        workspace
            .register_action(about)
            .register_action(|_, _: &Hide, cx| {
                cx.hide();
            })
            .register_action(|_, _: &HideOthers, cx| {
                cx.hide_other_apps();
            })
            .register_action(|_, _: &ShowAll, cx| {
                cx.unhide_other_apps();
            })
            .register_action(|_, _: &Minimize, cx| {
                cx.minimize_window();
            })
            .register_action(|_, _: &Zoom, cx| {
                cx.zoom_window();
            })
            .register_action(|_, _: &ToggleFullScreen, cx| {
                cx.toggle_full_screen();
            })
            .register_action(quit)
            .register_action(|_, action: &OpenZedURL, cx| {
                cx.global::<Arc<OpenListener>>()
                    .open_urls(&[action.url.clone()])
            })
            .register_action(|_, action: &OpenBrowser, cx| cx.open_url(&action.url))
            //todo!(buffer font size)
            // cx.add_global_action(move |_: &IncreaseBufferFontSize, cx| {
            //     theme::adjust_font_size(cx, |size| *size += 1.0)
            // });
            // cx.add_global_action(move |_: &DecreaseBufferFontSize, cx| {
            //     theme::adjust_font_size(cx, |size| *size -= 1.0)
            // });
            // cx.add_global_action(move |_: &ResetBufferFontSize, cx| theme::reset_font_size(cx));
            .register_action(|_, _: &install_cli::Install, cx| {
                cx.spawn(|_, cx| async move {
                    install_cli::install_cli(cx.deref())
                        .await
                        .context("error creating CLI symlink")
                })
                .detach_and_log_err(cx);
            })
            .register_action(|workspace, _: &OpenLog, cx| {
                open_log_file(workspace, cx);
            })
            .register_action(|workspace, _: &OpenLicenses, cx| {
                open_bundled_file(
                    workspace,
                    asset_str::<Assets>("licenses.md"),
                    "Open Source License Attribution",
                    "Markdown",
                    cx,
                );
            })
            .register_action(
                move |workspace: &mut Workspace,
                      _: &OpenTelemetryLog,
                      cx: &mut ViewContext<Workspace>| {
                    open_telemetry_log_file(workspace, cx);
                },
            )
            .register_action(
                move |_: &mut Workspace, _: &OpenKeymap, cx: &mut ViewContext<Workspace>| {
                    create_and_open_local_file(&paths::KEYMAP, cx, Default::default)
                        .detach_and_log_err(cx);
                },
            )
            .register_action(
                move |_: &mut Workspace, _: &OpenSettings, cx: &mut ViewContext<Workspace>| {
                    create_and_open_local_file(&paths::SETTINGS, cx, || {
                        settings::initial_user_settings_content().as_ref().into()
                    })
                    .detach_and_log_err(cx);
                },
            )
            .register_action(open_local_settings_file)
            .register_action(
                move |workspace: &mut Workspace,
                      _: &OpenDefaultKeymap,
                      cx: &mut ViewContext<Workspace>| {
                    open_bundled_file(
                        workspace,
                        settings::default_keymap(),
                        "Default Key Bindings",
                        "JSON",
                        cx,
                    );
                },
            )
            .register_action(
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
            )
            //todo!()
            // cx.add_action({
            //     move |workspace: &mut Workspace, _: &DebugElements, cx: &mut ViewContext<Workspace>| {
            //         let app_state = workspace.app_state().clone();
            //         let markdown = app_state.languages.language_for_name("JSON");
            //         let window = cx.window();
            //         cx.spawn(|workspace, mut cx| async move {
            //             let markdown = markdown.await.log_err();
            //             let content = to_string_pretty(&window.debug_elements(&cx).ok_or_else(|| {
            //                 anyhow!("could not debug elements for window {}", window.id())
            //             })?)
            //             .unwrap();
            //             workspace
            //                 .update(&mut cx, |workspace, cx| {
            //                     workspace.with_local_workspace(cx, move |workspace, cx| {
            //                         let project = workspace.project().clone();
            //                         let buffer = project
            //                             .update(cx, |project, cx| {
            //                                 project.create_buffer(&content, markdown, cx)
            //                             })
            //                             .expect("creating buffers on a local workspace always succeeds");
            //                         let buffer = cx.add_model(|cx| {
            //                             MultiBuffer::singleton(buffer, cx)
            //                                 .with_title("Debug Elements".into())
            //                         });
            //                         workspace.add_item(
            //                             Box::new(cx.add_view(|cx| {
            //                                 Editor::for_multibuffer(buffer, Some(project.clone()), cx)
            //                             })),
            //                             cx,
            //                         );
            //                     })
            //                 })?
            //                 .await
            //         })
            //         .detach_and_log_err(cx);
            //     }
            // });
            // .register_action(
            //     |workspace: &mut Workspace,
            //      _: &project_panel::ToggleFocus,
            //      cx: &mut ViewContext<Workspace>| {
            //         workspace.toggle_panel_focus::<ProjectPanel>(cx);
            //     },
            // );
            // cx.add_action(
            //     |workspace: &mut Workspace,
            //      _: &collab_ui::collab_panel::ToggleFocus,
            //      cx: &mut ViewContext<Workspace>| {
            //         workspace.toggle_panel_focus::<collab_ui::collab_panel::CollabPanel>(cx);
            //     },
            // );
            // cx.add_action(
            //     |workspace: &mut Workspace,
            //      _: &collab_ui::chat_panel::ToggleFocus,
            //      cx: &mut ViewContext<Workspace>| {
            //         workspace.toggle_panel_focus::<collab_ui::chat_panel::ChatPanel>(cx);
            //     },
            // );
            // cx.add_action(
            //     |workspace: &mut Workspace,
            //      _: &collab_ui::notification_panel::ToggleFocus,
            //      cx: &mut ViewContext<Workspace>| {
            //         workspace.toggle_panel_focus::<collab_ui::notification_panel::NotificationPanel>(cx);
            //     },
            // );
            // cx.add_action(
            //     |workspace: &mut Workspace,
            //      _: &terminal_panel::ToggleFocus,
            //      cx: &mut ViewContext<Workspace>| {
            //         workspace.toggle_panel_focus::<TerminalPanel>(cx);
            //     },
            // );
            .register_action({
                let app_state = Arc::downgrade(&app_state);
                move |_, _: &NewWindow, cx| {
                    if let Some(app_state) = app_state.upgrade() {
                        open_new(&app_state, cx, |workspace, cx| {
                            Editor::new_file(workspace, &Default::default(), cx)
                        })
                        .detach();
                    }
                }
            })
            .register_action({
                let app_state = Arc::downgrade(&app_state);
                move |_, _: &NewFile, cx| {
                    if let Some(app_state) = app_state.upgrade() {
                        open_new(&app_state, cx, |workspace, cx| {
                            Editor::new_file(workspace, &Default::default(), cx)
                        })
                        .detach();
                    }
                }
            });

        workspace.focus_handle(cx).focus(cx);
        //todo!()
        // load_default_keymap(cx);
    })
    .detach();
}

fn about(_: &mut Workspace, _: &About, cx: &mut gpui::ViewContext<Workspace>) {
    use std::fmt::Write as _;

    let app_name = cx.global::<ReleaseChannel>().display_name();
    let version = env!("CARGO_PKG_VERSION");
    let mut message = format!("{app_name} {version}");
    if let Some(sha) = cx.try_global::<AppCommitSha>() {
        write!(&mut message, "\n\n{}", sha.0).unwrap();
    }

    let prompt = cx.prompt(PromptLevel::Info, &message, &["OK"]);
    cx.foreground_executor()
        .spawn(async {
            prompt.await.ok();
        })
        .detach();
}

fn quit(_: &mut Workspace, _: &Quit, cx: &mut gpui::ViewContext<Workspace>) {
    let should_confirm = WorkspaceSettings::get_global(cx).confirm_quit;
    cx.spawn(|_, mut cx| async move {
        let mut workspace_windows = cx.update(|_, cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<Workspace>())
                .collect::<Vec<_>>()
        })?;

        // If multiple windows have unsaved changes, and need a save prompt,
        // prompt in the active window before switching to a different window.
        cx.update(|_, cx| {
            workspace_windows.sort_by_key(|window| window.is_active(&cx) == Some(false));
        })
        .log_err();

        if let (true, Some(window)) = (should_confirm, workspace_windows.first().copied()) {
            let answer = cx
                .update(|_, cx| {
                    cx.prompt(
                        PromptLevel::Info,
                        "Are you sure you want to quit?",
                        &["Quit", "Cancel"],
                    )
                })
                .log_err();

            if let Some(mut answer) = answer {
                let answer = answer.await.ok();
                if answer != Some(0) {
                    return Ok(());
                }
            }
        }

        // If the user cancels any save prompt, then keep the app open.
        for window in workspace_windows {
            if let Some(should_close) = window
                .update(&mut cx, |workspace, cx| {
                    workspace.prepare_to_close(true, cx)
                })
                .log_err()
            {
                if !should_close.await? {
                    return Ok(());
                }
            }
        }
        cx.update(|_, cx| {
            cx.quit();
        })?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
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

                        let buffer = cx.build_model(|cx| {
                            MultiBuffer::singleton(buffer, cx).with_title("Log".into())
                        });
                        workspace.add_item(
                            Box::new(cx.build_view(|cx| {
                                Editor::for_multibuffer(buffer, Some(project), cx)
                            })),
                            cx,
                        );
                    })
                    .log_err();
            })
            .detach();
        })
        .detach();
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
                if worktree.update(&mut cx, |tree, _| tree.entry_for_path(dir_path).is_none())? {
                    project
                        .update(&mut cx, |project, cx| {
                            project.create_entry((tree_id, dir_path), true, cx)
                        })?
                        .ok_or_else(|| anyhow!("worktree was removed"))?
                        .await?;
                }
            }

            if worktree.update(&mut cx, |tree, _| tree.entry_for_path(file_path).is_none())? {
                project
                    .update(&mut cx, |project, cx| {
                        project.create_entry((tree_id, file_path), false, cx)
                    })?
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
            cx.build_view(|_| MessageNotification::new("This project has no folders open."))
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

                let buffer = cx.build_model(|cx| {
                    MultiBuffer::singleton(buffer, cx).with_title("Telemetry Log".into())
                });
                workspace.add_item(
                    Box::new(cx.build_view(|cx| Editor::for_multibuffer(buffer, Some(project), cx))),
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
                    let buffer = cx.build_model(|cx| {
                        MultiBuffer::singleton(buffer, cx).with_title(title.into())
                    });
                    workspace.add_item(
                        Box::new(cx.build_view(|cx| {
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
