mod runnables_settings;

use gpui::{
    actions, div, px, red, AppContext, EventEmitter, FocusHandle, FocusableView, IntoElement,
    ParentElement as _, Render, Styled as _, View, ViewContext, VisualContext as _, WindowContext,
};
use ui::h_flex;
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<RunnablesPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct RunnablesPanel {
    focus_handle: FocusHandle,
}

impl RunnablesPanel {
    pub fn new(cx: &mut WindowContext<'_>) -> View<Self> {
        cx.new_view(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}
actions!(runnables_panel, [ToggleFocus]);
impl FocusableView for RunnablesPanel {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<PanelEvent> for RunnablesPanel {}

impl Panel for RunnablesPanel {
    fn persistent_name() -> &'static str {
        "RunnablesPanel"
    }

    fn position(&self, cx: &ui::prelude::WindowContext) -> workspace::dock::DockPosition {
        workspace::dock::DockPosition::Right
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        matches!(
            position,
            workspace::dock::DockPosition::Left | workspace::dock::DockPosition::Right
        )
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) {
    }

    fn size(&self, cx: &ui::prelude::WindowContext) -> ui::prelude::Pixels {
        px(400.)
    }

    fn set_size(
        &mut self,
        size: Option<ui::prelude::Pixels>,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) {
    }

    fn icon(&self, cx: &ui::prelude::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::Return)
    }

    fn icon_tooltip(&self, cx: &ui::prelude::WindowContext) -> Option<&'static str> {
        Some("Runnables panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl Render for RunnablesPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex().bg(red()).w_full().h_full().min_w(px(400.))
        // .child("Hey there little man")
    }
}
