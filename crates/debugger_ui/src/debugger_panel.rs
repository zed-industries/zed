use std::sync::Arc;

use anyhow::{anyhow, Result};
use dap::{
    client::{DebugAdapterClient, DebugAdapterClientId, TransportType},
    transport::Payload,
};
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    View, ViewContext, WeakView,
};
use ui::{
    div, h_flex,
    prelude::{IntoElement, Pixels, WindowContext},
    px, ButtonCommon, Clickable, Element, IconButton, IconName, ParentElement, Render, Styled,
    Tooltip, VisualContext,
};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(debug, [TogglePanel]);

pub struct DebugPanel {
    pub position: DockPosition,
    pub zoomed: bool,
    pub active: bool,
    pub focus_handle: FocusHandle,
    pub size: Pixels,
    pub workspace: WeakView<Workspace>,
}

impl DebugPanel {
    pub fn new(
        position: DockPosition,
        workspace: WeakView<Workspace>,
        cx: &mut WindowContext,
    ) -> Self {
        Self {
            position,
            zoomed: false,
            active: false,
            focus_handle: cx.focus_handle(),
            size: px(300.),
            workspace,
        }
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        workspace.update(&mut cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.start_debug_adapter_client(DebugAdapterClientId(1), cx);
            });
        })?;
        cx.new_view(|cx| DebugPanel::new(DockPosition::Bottom, workspace, cx))
    }

    fn debug_adapter(&self, cx: &mut ViewContext<Self>) -> Arc<DebugAdapterClient> {
        self.workspace
            .update(cx, |this, cx| {
                this.project()
                    .read(cx)
                    .running_debug_adapters()
                    .next()
                    .unwrap()
            })
            .unwrap()
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _cx: &mut ViewContext<Self>) {
        self.position = position;
        // TODO:
        // cx.update_global::<SettingsStore>(f)
    }

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        None
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(TogglePanel)
    }

    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }

    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn starts_open(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        IconButton::new("debug-play", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                cx.background_executor()
                                    .spawn(async move { client.continue_thread().await })
                                    .detach();
                            }))
                            .tooltip(move |cx| Tooltip::text("Start debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-in", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                cx.background_executor()
                                    .spawn(async move { client.step_in().await })
                                    .detach();
                            }))
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-out", IconName::Play)
                            .on_click(cx.listener(|view, _, cx| {
                                let client = view.debug_adapter(cx);
                                cx.background_executor()
                                    .spawn(async move { client.step_out().await })
                                    .detach();
                            }))
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-stop", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Stop", cx)),
                    ),
            )
            .into_any()
    }
}
