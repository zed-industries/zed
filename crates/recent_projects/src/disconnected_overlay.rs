use gpui::{ClickEvent, DismissEvent, EventEmitter, FocusHandle, Focusable, Render, WeakEntity};
use project::project_settings::ProjectSettings;
use remote::RemoteConnectionOptions;
use settings::Settings;
use ui::{
    Button, ButtonCommon, ButtonStyle, Clickable, Context, ElevationIndex, FluentBuilder, Headline,
    HeadlineSize, IconName, IconPosition, InteractiveElement, IntoElement, Label, Modal,
    ModalFooter, ModalHeader, ParentElement, Section, Styled, StyledExt, Window, div, h_flex, rems,
};
use workspace::{ModalView, OpenOptions, Workspace, notifications::DetachAndPromptErr};

use crate::open_remote_project;

enum Host {
    CollabGuestProject,
    RemoteServerProject(RemoteConnectionOptions, bool),
}

pub struct DisconnectedOverlay {
    workspace: WeakEntity<Workspace>,
    host: Host,
    focus_handle: FocusHandle,
    finished: bool,
}

impl EventEmitter<DismissEvent> for DisconnectedOverlay {}
impl Focusable for DisconnectedOverlay {
    fn focus_handle(&self, _cx: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl ModalView for DisconnectedOverlay {
    fn on_before_dismiss(
        &mut self,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        workspace::DismissDecision::Dismiss(self.finished)
    }
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl DisconnectedOverlay {
    pub fn register(
        workspace: &mut Workspace,
        window: Option<&mut Window>,
        cx: &mut Context<Workspace>,
    ) {
        let Some(window) = window else {
            return;
        };
        cx.subscribe_in(
            workspace.project(),
            window,
            |workspace, project, event, window, cx| {
                if !matches!(
                    event,
                    project::Event::DisconnectedFromHost
                        | project::Event::DisconnectedFromRemote { .. }
                ) {
                    return;
                }
                let handle = cx.entity().downgrade();

                let remote_connection_options = project.read(cx).remote_connection_options(cx);
                let host = if let Some(remote_connection_options) = remote_connection_options {
                    Host::RemoteServerProject(
                        remote_connection_options,
                        matches!(
                            event,
                            project::Event::DisconnectedFromRemote {
                                server_not_running: true
                            }
                        ),
                    )
                } else {
                    Host::CollabGuestProject
                };

                workspace.toggle_modal(window, cx, |_, cx| DisconnectedOverlay {
                    finished: false,
                    workspace: handle,
                    host,
                    focus_handle: cx.focus_handle(),
                });
            },
        )
        .detach();
    }

    fn handle_reconnect(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.finished = true;
        cx.emit(DismissEvent);

        if let Host::RemoteServerProject(remote_connection_options, _) = &self.host {
            self.reconnect_to_remote_project(remote_connection_options.clone(), window, cx);
        }
    }

    fn reconnect_to_remote_project(
        &self,
        connection_options: RemoteConnectionOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let Some(window_handle) = window.window_handle().downcast::<Workspace>() else {
            return;
        };

        let app_state = workspace.read(cx).app_state().clone();
        let paths = workspace
            .read(cx)
            .root_paths(cx)
            .iter()
            .map(|path| path.to_path_buf())
            .collect();

        cx.spawn_in(window, async move |_, cx| {
            open_remote_project(
                connection_options,
                paths,
                app_state,
                OpenOptions {
                    replace_window: Some(window_handle),
                    ..Default::default()
                },
                cx,
            )
            .await?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to reconnect", window, cx, |_, _, _| None);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.finished = true;
        cx.emit(DismissEvent)
    }
}

impl Render for DisconnectedOverlay {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_reconnect = matches!(self.host, Host::RemoteServerProject(..));

        let message = match &self.host {
            Host::CollabGuestProject => {
                "Your connection to the remote project has been lost.".to_string()
            }
            Host::RemoteServerProject(options, server_not_running) => {
                let autosave = if ProjectSettings::get_global(cx)
                    .session
                    .restore_unsaved_buffers
                {
                    "\nUnsaved changes are stored locally."
                } else {
                    ""
                };
                let reason = if *server_not_running {
                    "process exiting unexpectedly"
                } else {
                    "not responding"
                };
                format!(
                    "Your connection to {} has been lost due to the server {reason}.{autosave}",
                    options.display_name(),
                )
            }
        };

        div()
            .track_focus(&self.focus_handle(cx))
            .elevation_3(cx)
            .on_action(cx.listener(Self::cancel))
            .occlude()
            .w(rems(24.))
            .max_h(rems(40.))
            .child(
                Modal::new("disconnected", None)
                    .header(
                        ModalHeader::new()
                            .show_dismiss_button(true)
                            .child(Headline::new("Disconnected").size(HeadlineSize::Small)),
                    )
                    .section(Section::new().child(Label::new(message)))
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("close-window", "Close Window")
                                        .style(ButtonStyle::Filled)
                                        .layer(ElevationIndex::ModalSurface)
                                        .on_click(cx.listener(move |_, _, window, _| {
                                            window.remove_window();
                                        })),
                                )
                                .when(can_reconnect, |el| {
                                    el.child(
                                        Button::new("reconnect", "Reconnect")
                                            .style(ButtonStyle::Filled)
                                            .layer(ElevationIndex::ModalSurface)
                                            .icon(IconName::ArrowCircle)
                                            .icon_position(IconPosition::Start)
                                            .on_click(cx.listener(Self::handle_reconnect)),
                                    )
                                }),
                        ),
                    ),
            )
    }
}
