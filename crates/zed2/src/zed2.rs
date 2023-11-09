#![allow(unused_variables, dead_code, unused_mut)]
// todo!() this is to make transition easier.

mod assets;
pub mod languages;
mod only_instance;
mod open_listener;

pub use assets::*;
use gpui::{
    point, px, AppContext, AsyncWindowContext, Task, TitlebarOptions, WeakView, WindowBounds,
    WindowKind, WindowOptions,
};
pub use only_instance::*;
pub use open_listener::*;

use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;
use workspace::{AppState, Workspace};

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

pub fn initialize_workspace(
    workspace_handle: WeakView<Workspace>,
    was_deserialized: bool,
    app_state: Arc<AppState>,
    cx: AsyncWindowContext,
) -> Task<Result<()>> {
    cx.spawn(|mut cx| async move {
        workspace_handle.update(&mut cx, |workspace, cx| {
            let workspace_handle = cx.view().clone();
            cx.subscribe(&workspace_handle, {
                move |workspace, _, event, cx| {
                    if let workspace::Event::PaneAdded(pane) = event {
                        pane.update(cx, |pane, cx| {
                            pane.toolbar().update(cx, |toolbar, cx| {
                                // todo!()
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
            workspace.status_bar().update(cx, |status_bar, cx| {
                // status_bar.add_left_item(diagnostic_summary, cx);
                // status_bar.add_left_item(activity_indicator, cx);

                // status_bar.add_right_item(feedback_button, cx);
                // status_bar.add_right_item(copilot, cx);
                // status_bar.add_right_item(active_buffer_language, cx);
                // status_bar.add_right_item(vim_mode_indicator, cx);
                // status_bar.add_right_item(cursor_position, cx);
            });

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
