use gpui::{
    actions, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView, View,
    ViewContext, WeakView,
};
use ui::{
    div, h_flex, prelude, px, ButtonCommon, Element, IconButton, ParentElement, Pixels, Render,
    Styled, Tooltip, VisualContext, WindowContext,
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
}

impl DebugPanel {
    pub fn new(position: DockPosition, cx: &mut WindowContext) -> Self {
        Self {
            position,
            zoomed: false,
            active: false,
            focus_handle: cx.focus_handle(),
            size: px(300.),
        }
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<View<Self>> {
        workspace.update(&mut cx, |workspace, cx| {
            cx.new_view(|cx| DebugPanel::new(workspace::dock::DockPosition::Bottom, cx))
        })
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

    fn position(&self, _cx: &prelude::WindowContext) -> workspace::dock::DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        _cx: &mut ViewContext<Self>,
    ) {
        self.position = position;
        // TODO:
        // cx.update_global::<SettingsStore>(f)
    }

    fn size(&self, _cx: &prelude::WindowContext) -> prelude::Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<prelude::Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &prelude::WindowContext) -> Option<ui::IconName> {
        None
    }

    fn icon_tooltip(&self, _cx: &prelude::WindowContext) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
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
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl prelude::IntoElement {
        div()
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        IconButton::new("debug-play", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Start debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-in", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-out", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-stop", ui::IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Stop", cx)),
                    ),
            )
            .into_any()
    }
}
