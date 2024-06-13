use dev_server_projects::DevServer;
use gpui::{ClickEvent, DismissEvent, EventEmitter, FocusHandle, FocusableView, Render, WeakView};
use ui::{
    div, h_flex, rems, Button, ButtonCommon, ButtonStyle, Clickable, ElevationIndex, FluentBuilder,
    Headline, HeadlineSize, IconName, IconPosition, InteractiveElement, IntoElement, Label, Modal,
    ModalFooter, ModalHeader, ParentElement, Section, Styled, StyledExt, ViewContext,
};
use workspace::{notifications::DetachAndPromptErr, ModalView, Workspace};

use crate::{
    dev_servers::reconnect_to_dev_server_project, open_dev_server_project, DevServerProjects,
};

pub struct DisconnectedOverlay {
    workspace: WeakView<Workspace>,
    dev_server: Option<DevServer>,
    focus_handle: FocusHandle,
}

impl EventEmitter<DismissEvent> for DisconnectedOverlay {}
impl FocusableView for DisconnectedOverlay {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl ModalView for DisconnectedOverlay {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl DisconnectedOverlay {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        cx.subscribe(workspace.project(), |workspace, project, event, cx| {
            if !matches!(event, project::Event::DisconnectedFromHost) {
                return;
            }
            let handle = cx.view().downgrade();
            let dev_server = project
                .read(cx)
                .dev_server_project_id()
                .and_then(|id| {
                    dev_server_projects::Store::global(cx)
                        .read(cx)
                        .dev_server_for_project(id)
                })
                .cloned();
            workspace.toggle_modal(cx, |cx| DisconnectedOverlay {
                workspace: handle,
                dev_server,
                focus_handle: cx.focus_handle(),
            });
        })
        .detach();
    }

    fn handle_reconnect(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(dev_server) = self.dev_server.clone() else {
            return;
        };
        let Some(dev_server_project_id) = workspace
            .read(cx)
            .project()
            .read(cx)
            .dev_server_project_id()
        else {
            return;
        };

        if let Some(project_id) = dev_server_projects::Store::global(cx)
            .read(cx)
            .dev_server_project(dev_server_project_id)
            .and_then(|project| project.project_id)
        {
            return workspace.update(cx, move |_, cx| {
                open_dev_server_project(true, dev_server_project_id, project_id, cx)
                    .detach_and_prompt_err("Failed to reconnect", cx, |_, _| None)
            });
        }

        if dev_server.ssh_connection_string.is_some() {
            let task = workspace.update(cx, |_, cx| {
                reconnect_to_dev_server_project(
                    cx.view().clone(),
                    dev_server,
                    dev_server_project_id,
                    true,
                    cx,
                )
            });

            task.detach_and_prompt_err("Failed to reconnect", cx, |_, _| None);
        } else {
            return workspace.update(cx, |workspace, cx| {
                let handle = cx.view().downgrade();
                workspace.toggle_modal(cx, |cx| DevServerProjects::new(cx, handle))
            });
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for DisconnectedOverlay {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
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
                    .section(Section::new().child(Label::new(
                        "Your connection to the remote project has been lost.",
                    )))
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("close-window", "Close Window")
                                        .style(ButtonStyle::Filled)
                                        .layer(ElevationIndex::ModalSurface)
                                        .on_click(cx.listener(move |_, _, cx| {
                                            cx.remove_window();
                                        })),
                                )
                                .when_some(self.dev_server.clone(), |el, _| {
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
