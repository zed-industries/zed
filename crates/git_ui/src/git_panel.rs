use gpui::*;
use ui::{prelude::*, Checkbox, Divider, DividerColor, ElevationIndex};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<GitPanel>(cx);
            });
        },
    )
    .detach();
}

actions!(git_panel, [Deploy, ToggleFocus]);

#[derive(Clone)]
pub struct GitPanel {
    _workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
}

impl GitPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            workspace.update(&mut cx, |workspace, cx| {
                let workspace_handle = workspace.weak_handle();

                cx.new_view(|cx| Self::new(workspace_handle, cx))
            })
        })
    }

    pub fn new(workspace: WeakView<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        Self {
            _workspace: workspace,
            focus_handle: cx.focus_handle(),
            width: Some(px(360.)),
        }
    }

    pub fn render_panel_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h(px(32.))
            .items_center()
            .px_3()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(
                h_flex()
                    .gap_1()
                    .child(Checkbox::new("all-changes", true.into()).disabled(true))
                    .child(div().text_buffer(cx).text_ui_sm(cx).child("0 changes")),
            )
            .child(div().flex_grow())
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("discard-changes", IconName::Undo)
                            .icon_size(IconSize::Small)
                            .disabled(true),
                    )
                    .child(
                        Button::new("stage-all", "Stage All")
                            .label_size(LabelSize::Small)
                            .layer(ElevationIndex::ElevatedSurface)
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Filled)
                            .disabled(true),
                    ),
            )
    }

    pub fn render_commit_editor(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        div().w_full().h(px(140.)).px_2().pt_1().pb_2().child(
            v_flex()
                .h_full()
                .py_2p5()
                .px_3()
                .bg(cx.theme().colors().editor_background)
                .font_buffer(cx)
                .text_ui_sm(cx)
                .text_color(cx.theme().colors().text_muted)
                .child("Add a message")
                .gap_1()
                .child(div().flex_grow())
                .child(
                    h_flex().child(div().gap_1().flex_grow()).child(
                        Button::new("commit", "Commit")
                            .label_size(LabelSize::Small)
                            .layer(ElevationIndex::ElevatedSurface)
                            .size(ButtonSize::Compact)
                            .style(ButtonStyle::Filled)
                            .disabled(true),
                    ),
                )
                .cursor(CursorStyle::OperationNotAllowed)
                .opacity(0.5),
        )
    }
}

impl Render for GitPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitPanel")
            .font_buffer(cx)
            .py_1()
            .id("git_panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_hidden()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(self.render_panel_header(cx))
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
            .child(self.render_commit_editor(cx))
    }
}

impl FocusableView for GitPanel {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for GitPanel {}

impl Panel for GitPanel {
    fn persistent_name() -> &'static str {
        "GitPanel"
    }

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
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}
