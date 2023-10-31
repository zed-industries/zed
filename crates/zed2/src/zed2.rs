mod assets;
pub mod languages;
mod only_instance;
mod open_listener;

pub use assets::*;
use collections::HashMap;
use gpui2::{
    point, px, AppContext, AsyncAppContext, AsyncWindowContext, MainThread, Point, Task,
    TitlebarOptions, WeakView, WindowBounds, WindowKind, WindowOptions,
};
pub use only_instance::*;
pub use open_listener::*;

use anyhow::{Context, Result};
use cli::{
    ipc::{self, IpcSender},
    CliRequest, CliResponse, IpcHandshake,
};
use futures::{
    channel::{mpsc, oneshot},
    FutureExt, SinkExt, StreamExt,
};
use std::{path::Path, sync::Arc, thread, time::Duration};
use util::{paths::PathLikeWithPosition, ResultExt};
use uuid::Uuid;
use workspace2::{AppState, Workspace};

pub fn connect_to_cli(
    server_name: &str,
) -> Result<(mpsc::Receiver<CliRequest>, IpcSender<CliResponse>)> {
    let handshake_tx = cli::ipc::IpcSender::<IpcHandshake>::connect(server_name.to_string())
        .context("error connecting to cli")?;
    let (request_tx, request_rx) = ipc::channel::<CliRequest>()?;
    let (response_tx, response_rx) = ipc::channel::<CliResponse>()?;

    handshake_tx
        .send(IpcHandshake {
            requests: request_tx,
            responses: response_rx,
        })
        .context("error sending ipc handshake")?;

    let (mut async_request_tx, async_request_rx) =
        futures::channel::mpsc::channel::<CliRequest>(16);
    thread::spawn(move || {
        while let Ok(cli_request) = request_rx.recv() {
            if smol::block_on(async_request_tx.send(cli_request)).is_err() {
                break;
            }
        }
        Ok::<_, anyhow::Error>(())
    });

    Ok((async_request_rx, response_tx))
}

pub async fn handle_cli_connection(
    (mut requests, responses): (mpsc::Receiver<CliRequest>, IpcSender<CliResponse>),
    app_state: Arc<AppState>,
    mut cx: AsyncAppContext,
) {
    if let Some(request) = requests.next().await {
        match request {
            CliRequest::Open { paths, wait } => {
                let mut caret_positions = HashMap::default();

                let paths = if paths.is_empty() {
                    todo!()
                    //     workspace::last_opened_workspace_paths()
                    //         .await
                    //         .map(|location| location.paths().to_vec())
                    //         .unwrap_or_default()
                } else {
                    paths
                        .into_iter()
                        .filter_map(|path_with_position_string| {
                            let path_with_position = PathLikeWithPosition::parse_str(
                                &path_with_position_string,
                                |path_str| {
                                    Ok::<_, std::convert::Infallible>(
                                        Path::new(path_str).to_path_buf(),
                                    )
                                },
                            )
                            .expect("Infallible");
                            let path = path_with_position.path_like;
                            if let Some(row) = path_with_position.row {
                                if path.is_file() {
                                    let row = row.saturating_sub(1);
                                    let col =
                                        path_with_position.column.unwrap_or(0).saturating_sub(1);
                                    caret_positions.insert(path.clone(), Point::new(row, col));
                                }
                            }
                            Some(path)
                        })
                        .collect::<Vec<_>>()
                };

                let mut errored = false;

                if let Some(open_paths_task) = cx
                    .update(|cx| workspace2::open_paths(&paths, &app_state, None, cx))
                    .log_err()
                {
                    match open_paths_task.await {
                        Ok((workspace, items)) => {
                            let mut item_release_futures = Vec::new();

                            for (item, path) in items.into_iter().zip(&paths) {
                                match item {
                                    Some(Ok(mut item)) => {
                                        if let Some(point) = caret_positions.remove(path) {
                                            todo!()
                                            // if let Some(active_editor) = item.downcast::<Editor>() {
                                            //     active_editor
                                            //         .downgrade()
                                            //         .update(&mut cx, |editor, cx| {
                                            //             let snapshot =
                                            //                 editor.snapshot(cx).display_snapshot;
                                            //             let point = snapshot
                                            //                 .buffer_snapshot
                                            //                 .clip_point(point, Bias::Left);
                                            //             editor.change_selections(
                                            //                 Some(Autoscroll::center()),
                                            //                 cx,
                                            //                 |s| s.select_ranges([point..point]),
                                            //             );
                                            //         })
                                            //         .log_err();
                                            // }
                                        }

                                        let released = oneshot::channel();
                                        cx.update(move |cx| {
                                            item.on_release(
                                                cx,
                                                Box::new(move |_| {
                                                    let _ = released.0.send(());
                                                }),
                                            )
                                            .detach();
                                        });
                                        item_release_futures.push(released.1);
                                    }
                                    Some(Err(err)) => {
                                        responses
                                            .send(CliResponse::Stderr {
                                                message: format!(
                                                    "error opening {:?}: {}",
                                                    path, err
                                                ),
                                            })
                                            .log_err();
                                        errored = true;
                                    }
                                    None => {}
                                }
                            }

                            if wait {
                                let executor = cx.executor().clone();
                                let wait = async move {
                                    if paths.is_empty() {
                                        let (done_tx, done_rx) = oneshot::channel();
                                        let _subscription =
                                            cx.update_window_root(&workspace, move |_, cx| {
                                                cx.on_release(|_, _| {
                                                    let _ = done_tx.send(());
                                                })
                                            });
                                        drop(workspace);
                                        let _ = done_rx.await;
                                    } else {
                                        let _ = futures::future::try_join_all(item_release_futures)
                                            .await;
                                    };
                                }
                                .fuse();
                                futures::pin_mut!(wait);

                                loop {
                                    // Repeatedly check if CLI is still open to avoid wasting resources
                                    // waiting for files or workspaces to close.
                                    let mut timer = executor.timer(Duration::from_secs(1)).fuse();
                                    futures::select_biased! {
                                        _ = wait => break,
                                        _ = timer => {
                                            if responses.send(CliResponse::Ping).is_err() {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            errored = true;
                            responses
                                .send(CliResponse::Stderr {
                                    message: format!("error opening {:?}: {}", paths, error),
                                })
                                .log_err();
                        }
                    }

                    responses
                        .send(CliResponse::Exit {
                            status: i32::from(errored),
                        })
                        .log_err();
                }
            }
        }
    }
}

pub fn build_window_options(
    bounds: Option<WindowBounds>,
    display_uuid: Option<Uuid>,
    cx: MainThread<AppContext>,
) -> WindowOptions {
    let bounds = bounds.unwrap_or(WindowBounds::Maximized);
    let display = display_uuid.and_then(|uuid| cx.display_for_uuid(uuid));

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
        is_movable: false,
        display_id: display.map(|display| display.id()),
    }
}

pub fn initialize_workspace(
    workspace_handle: WeakView<Workspace>,
    was_deserialized: bool,
    app_state: Arc<AppState>,
    cx: AsyncWindowContext,
) -> Task<Result<()>> {
    cx.spawn(|mut cx| async move {
        workspace_handle.update(&mut cx, |workspace, cx| {
            let workspace_handle = cx.view();
            cx.subscribe(&workspace_handle, {
                move |workspace, _, event, cx| {
                    if let workspace2::Event::PaneAdded(pane) = event {
                        pane.update(cx, |pane, cx| {
                            // todo!()
                            // pane.toolbar().update(cx, |toolbar, cx| {
                            //     let breadcrumbs = cx.add_view(|_| Breadcrumbs::new(workspace));
                            //     toolbar.add_item(breadcrumbs, cx);
                            //     let buffer_search_bar = cx.add_view(BufferSearchBar::new);
                            //     toolbar.add_item(buffer_search_bar.clone(), cx);
                            //     let quick_action_bar = cx.add_view(|_| {
                            //         QuickActionBar::new(buffer_search_bar, workspace)
                            //     });
                            //     toolbar.add_item(quick_action_bar, cx);
                            //     let diagnostic_editor_controls =
                            //         cx.add_view(|_| diagnostics2::ToolbarControls::new());
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
                            // })
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
            //     let diagnostic_summary =
            //         cx.add_view(|cx| diagnostics::items::DiagnosticIndicator::new(workspace, cx));
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
            //     workspace.status_bar().update(cx, |status_bar, cx| {
            //         status_bar.add_left_item(diagnostic_summary, cx);
            //         status_bar.add_left_item(activity_indicator, cx);

            //         status_bar.add_right_item(feedback_button, cx);
            //         status_bar.add_right_item(copilot, cx);
            //         status_bar.add_right_item(active_buffer_language, cx);
            //         status_bar.add_right_item(vim_mode_indicator, cx);
            //         status_bar.add_right_item(cursor_position, cx);
            //     });

            //     auto_update::notify_of_any_new_update(cx.weak_handle(), cx);

            //     vim::observe_keystrokes(cx);

            //     cx.on_window_should_close(|workspace, cx| {
            //         if let Some(task) = workspace.close(&Default::default(), cx) {
            //             task.detach_and_log_err(cx);
            //         }
            //         false
            //     });
            // })?;

            // let project_panel = ProjectPanel::load(workspace_handle.clone(), cx.clone());
            // let terminal_panel = TerminalPanel::load(workspace_handle.clone(), cx.clone());
            // let assistant_panel = AssistantPanel::load(workspace_handle.clone(), cx.clone());
            // let channels_panel =
            //     collab_ui::collab_panel::CollabPanel::load(workspace_handle.clone(), cx.clone());
            // let chat_panel =
            //     collab_ui::chat_panel::ChatPanel::load(workspace_handle.clone(), cx.clone());
            // let notification_panel = collab_ui::notification_panel::NotificationPanel::load(
            //     workspace_handle.clone(),
            //     cx.clone(),
            // );
            // let (
            //     project_panel,
            //     terminal_panel,
            //     assistant_panel,
            //     channels_panel,
            //     chat_panel,
            //     notification_panel,
            // ) = futures::try_join!(
            //     project_panel,
            //     terminal_panel,
            //     assistant_panel,
            //     channels_panel,
            //     chat_panel,
            //     notification_panel,
            // )?;
            // workspace_handle.update(&mut cx, |workspace, cx| {
            //     let project_panel_position = project_panel.position(cx);
            //     workspace.add_panel_with_extra_event_handler(
            //         project_panel,
            //         cx,
            //         |workspace, _, event, cx| match event {
            //             project_panel::Event::NewSearchInDirectory { dir_entry } => {
            //                 search::ProjectSearchView::new_search_in_directory(workspace, dir_entry, cx)
            //             }
            //             project_panel::Event::ActivatePanel => {
            //                 workspace.focus_panel::<ProjectPanel>(cx);
            //             }
            //             _ => {}
            //         },
            //     );
            //     workspace.add_panel(terminal_panel, cx);
            //     workspace.add_panel(assistant_panel, cx);
            //     workspace.add_panel(channels_panel, cx);
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
            //         workspace.toggle_dock(project_panel_position, cx);
            //     }
            //     cx.focus_self();
        })?;
        Ok(())
    })
}
