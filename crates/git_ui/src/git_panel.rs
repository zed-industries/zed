use std::sync::Arc;
use util::TryFutureExt;

use db::kvp::KEY_VALUE_STORE;
use gpui::*;
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use ui::{prelude::*, Checkbox, Divider, DividerColor, ElevationIndex};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::settings::GitPanelSettings;

const GIT_PANEL_KEY: &str = "GitPanel";

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

#[derive(Serialize, Deserialize)]
struct SerializedGitPanel {
    width: Option<Pixels>,
}

actions!(git_panel, [Deploy, ToggleFocus]);

pub struct GitPanel {
    _workspace: WeakView<Workspace>,
    pending_serialization: Task<Option<()>>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
}

impl GitPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            // Clippy incorrectly classifies this as a redundant closure
            #[allow(clippy::redundant_closure)]
            workspace.update(&mut cx, |workspace, cx| Self::new(workspace, cx))
        })
    }

    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let weak_workspace = workspace.weak_handle();

        cx.new_view(|cx| Self {
            fs,
            _workspace: weak_workspace,
            pending_serialization: Task::ready(None),
            focus_handle: cx.focus_handle(),
            width: Some(px(360.)),
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        GIT_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedGitPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    pub fn render_panel_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h(px(32.))
            .items_center()
            .px_3()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(
                h_flex()
                    .gap_2()
                    .child(Checkbox::new("all-changes", true.into()).disabled(true))
                    .child(div().text_buffer(cx).text_ui_sm(cx).child("0 changes")),
            )
            .child(div().flex_grow())
            .child(
                h_flex()
                    .gap_2()
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

    fn render_empty_state(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .h_full()
            .flex_1()
            .justify_center()
            .items_center()
            .child(
                v_flex()
                    .gap_3()
                    .child("No changes to commit")
                    .text_ui_sm(cx)
                    .mx_auto()
                    .text_color(Color::Placeholder.color(cx)),
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
            .child(self.render_empty_state(cx))
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

    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        GitPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<GitPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| settings.dock = Some(position),
        );
    }

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| GitPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::GitBranch).filter(|_| GitPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Git Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}
