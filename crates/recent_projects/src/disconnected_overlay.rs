use std::time::Duration;

use dev_server_projects::DevServer;
use gpui::{AnyElement, Bounds, GlobalElementId, LayoutId, WeakView};
use rpc::proto;
use ui::{
    div, ActiveTheme, Button, ButtonCommon, ButtonStyle, Clickable, Element, ElementId,
    ElevationIndex, FluentBuilder, IconName, IconPosition, InteractiveElement, IntoElement, Label,
    ParentElement, Pixels, Styled, ViewContext, WindowContext,
};
use workspace::{notifications::DetachAndPromptErr, Workspace};

use crate::{
    dev_servers::reconnect_to_dev_server_project, open_dev_server_project, DevServerProjects,
};

pub struct DisconnectedOverlay {
    workspace: WeakView<Workspace>,
    dev_server: Option<DevServer>,
}

impl DisconnectedOverlay {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        workspace.set_render_disconnected_overlay(|workspace, cx| {
            let dev_server = workspace
                .project()
                .read(cx)
                .dev_server_project_id()
                .and_then(|id| {
                    dev_server_projects::Store::global(cx)
                        .read(cx)
                        .dev_server_for_project(id)
                })
                .cloned();
            DisconnectedOverlay {
                workspace: cx.view().downgrade(),
                dev_server,
            }
            .into_any_element()
        })
    }

    fn handle_reconnect(
        workspace: WeakView<Workspace>,
        dev_server: DevServer,
        cx: &mut WindowContext,
    ) {
        let Some(workspace) = workspace.upgrade() else {
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
                open_dev_server_project(true, project_id, cx).detach_and_prompt_err(
                    "Failed to reconnect",
                    cx,
                    |_, _| None,
                )
            });
        }

        let reset_window = Workspace::new_local(
            vec![],
            workspace.read(cx).app_state().clone(),
            cx.window_handle().downcast::<Workspace>(),
            cx,
        );
        cx.spawn(|mut cx| async move {
            let (window, _) = reset_window.await?;

            // TODO. Hopefully this is long enough for the panels to load...
            cx.background_executor()
                .timer(Duration::from_millis(1000))
                .await;

            let (workspace, mut cx) =
                window.update(&mut cx, |_, cx| (cx.view().clone(), cx.to_async()))?;

            if dev_server.status == proto::DevServerStatus::Online
                || dev_server.ssh_connection_string.is_none()
            {
                return workspace.update(&mut cx, |workspace, cx| {
                    let handle = cx.view().downgrade();
                    workspace.toggle_modal(cx, |cx| DevServerProjects::new(cx, handle))
                });
            }
            workspace
                .update(&mut cx, |_, cx| {
                    reconnect_to_dev_server_project(
                        cx.view().clone(),
                        dev_server.clone(),
                        dev_server_project_id,
                        true,
                        cx,
                    )
                })?
                .await?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to reconnect", cx, |_, _| None);
        return;
    }
}

impl Element for DisconnectedOverlay {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut background = cx.theme().colors().elevated_surface_background;
        background.fade_out(0.2);

        let mut overlay = div()
            .bg(background)
            .absolute()
            .left_0()
            .top(ui::TitleBar::height(cx))
            .size_full()
            .flex()
            .gap_2()
            .items_center()
            .justify_center()
            // .capture_any_mouse_down(|_, cx| cx.stop_propagation())
            // .capture_any_mouse_up(|_, cx| cx.stop_propagation())
            .child(Label::new(
                "Your connection to the remote project has been lost.",
            ))
            .when_some(self.dev_server.take(), |el, dev_server| {
                el.child(
                    Button::new("reconnect", "Reconnect")
                        .style(ButtonStyle::Filled)
                        .layer(ElevationIndex::ModalSurface)
                        .icon(IconName::ArrowCircle)
                        .icon_position(IconPosition::Start)
                        .on_click({
                            let workspace = self.workspace.clone();
                            move |_, cx| {
                                Self::handle_reconnect(workspace.clone(), dev_server.clone(), cx)
                            }
                        }),
                )
            })
            .into_any();
        (overlay.request_layout(cx), overlay)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        overlay: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) {
        cx.insert_hitbox(bounds, true);
        overlay.prepaint(cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        overlay: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        overlay.paint(cx)
    }
}

impl IntoElement for DisconnectedOverlay {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
