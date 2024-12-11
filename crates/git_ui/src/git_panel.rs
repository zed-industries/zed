use gpui::*;
use ui::{prelude::*, Divider, DividerColor, ElevationIndex};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::item::TabContentParams;
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<GitPanel>(cx);
        });
    })
    .detach();
}

actions!(git_panel, [Deploy, ToggleFocus]);

#[derive(Clone)]
pub struct GitPanel {
    id: ElementId,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
}

impl GitPanel {
    pub fn new(id: impl Into<ElementId>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            id: id.into(),
            focus_handle: cx.focus_handle(),
            width: Some(px(400.).into()),
        }
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .font_buffer(cx)
            .py_1()
            .id(self.id.clone())
            .key_context("git_panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_hidden()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(
                h_flex()
                    .items_center()
                    .h(px(8.))
                    .child(Divider::horizontal_dashed().color(DividerColor::Border)),
            )
            .child(div().flex_1())
            .child(
                h_flex()
                    .items_center()
                    .h(px(8.))
                    .child(Divider::horizontal_dashed().color(DividerColor::Border)),
            )
    }
}

impl EventEmitter<()> for GitPanel {}
impl EventEmitter<PanelEvent> for GitPanel {}

impl FocusableView for GitPanel {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for GitPanel {
    fn position(&self, _cx: &gpui::WindowContext) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _cx: &mut ViewContext<Self>) {}

    fn size(&self, _cx: &gpui::WindowContext) -> Pixels {
        self.width.unwrap_or(px(360.))
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        cx.notify();
    }

    fn icon(&self, _cx: &gpui::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Git")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn persistent_name() -> &'static str {
        "GitPanel"
    }
}
