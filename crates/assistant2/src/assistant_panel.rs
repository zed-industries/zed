use anyhow::Result;
use gpui::{
    prelude::*, px, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, Pixels, Task, View, ViewContext, WeakView, WindowContext,
};
use ui::prelude::*;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{Pane, Workspace};

use crate::{NewChat, ToggleFocus};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<AssistantPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct AssistantPanel {
    pane: View<Pane>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                cx.new_view(|cx| Self::new(workspace, cx))
            })
        })
    }

    fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewChat.boxed_clone(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(true, cx);

            pane
        });

        Self { pane }
    }
}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel2"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &WindowContext) -> Pixels {
        px(640.)
    }

    fn set_size(&mut self, _size: Option<Pixels>, _cx: &mut ViewContext<Self>) {}

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}

    fn pane(&self) -> Option<View<Pane>> {
        Some(self.pane.clone())
    }

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child(Label::new("Assistant II"))
    }
}
