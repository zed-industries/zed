use std::sync::Arc;
use util::TryFutureExt;

use db::kvp::KEY_VALUE_STORE;
use gpui::*;
use project::{Fs, Project};
use serde::{Deserialize, Serialize};
use settings::Settings as _;
use ui::{prelude::*, Checkbox, Divider, DividerColor, ElevationIndex, Tooltip};
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::Workspace;

use crate::settings::GitPanelSettings;
use crate::{CommitAllChanges, CommitStagedChanges, DiscardAll, StageAll, UnstageAll};

actions!(git_panel, [ToggleFocus]);

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

pub struct GitPanel {
    _workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
    fs: Arc<dyn Fs>,
    pending_serialization: Task<Option<()>>,
    project: Model<Project>,
    width: Option<Pixels>,

    current_modifiers: Modifiers,
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
        let project = workspace.project().clone();
        let fs = workspace.app_state().fs.clone();
        let weak_workspace = workspace.weak_handle();

        cx.new_view(|cx| Self {
            _workspace: weak_workspace,
            focus_handle: cx.focus_handle(),
            fs,
            pending_serialization: Task::ready(None),
            project,

            current_modifiers: cx.modifiers(),

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

    fn dispatch_context(&self) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("GitPanel");
        dispatch_context.add("menu");

        dispatch_context
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        self.current_modifiers = event.modifiers;
        cx.notify();
    }
}

impl GitPanel {
    fn stage_all(&mut self, _: &StageAll, _cx: &mut ViewContext<Self>) {
        // todo!(): Implement stage all
        println!("Stage all triggered");
    }

    fn unstage_all(&mut self, _: &UnstageAll, _cx: &mut ViewContext<Self>) {
        // todo!(): Implement unstage all
        println!("Unstage all triggered");
    }

    fn discard_all(&mut self, _: &DiscardAll, _cx: &mut ViewContext<Self>) {
        // todo!(): Implement discard all
        println!("Discard all triggered");
    }

    /// Commit all staged changes
    fn commit_staged_changes(&mut self, _: &CommitStagedChanges, _cx: &mut ViewContext<Self>) {
        // todo!(): Implement commit all staged
        println!("Commit staged changes triggered");
    }

    /// Commit all changes, regardless of whether they are staged or not
    fn commit_all_changes(&mut self, _: &CommitAllChanges, _cx: &mut ViewContext<Self>) {
        // todo!(): Implement commit all changes
        println!("Commit all changes triggered");
    }

    fn all_staged(&self) -> bool {
        // todo!(): Implement all_staged
        true
    }
}

impl GitPanel {
    pub fn panel_button(
        &self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
    ) -> Button {
        let id = id.into().clone();
        let label = label.into().clone();

        Button::new(id, label)
            .label_size(LabelSize::Small)
            .layer(ElevationIndex::ElevatedSurface)
            .size(ButtonSize::Compact)
            .style(ButtonStyle::Filled)
    }

    pub fn render_divider(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .items_center()
            .h(px(8.))
            .child(Divider::horizontal_dashed().color(DividerColor::Border))
    }

    pub fn render_panel_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();

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
                            .tooltip(move |cx| {
                                let focus_handle = focus_handle.clone();

                                Tooltip::for_action_in(
                                    "Discard all changes",
                                    &DiscardAll,
                                    &focus_handle,
                                    cx,
                                )
                            })
                            .icon_size(IconSize::Small)
                            .disabled(true),
                    )
                    .child(if self.all_staged() {
                        self.panel_button("unstage-all", "Unstage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(DiscardAll))),
                        )
                    } else {
                        self.panel_button("stage-all", "Stage All").on_click(
                            cx.listener(move |_, _, cx| cx.dispatch_action(Box::new(StageAll))),
                        )
                    }),
            )
    }

    pub fn render_commit_editor(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let focus_handle_1 = self.focus_handle(cx).clone();
        let focus_handle_2 = self.focus_handle(cx).clone();

        let commit_staged_button = self
            .panel_button("commit-staged-changes", "Commit")
            .tooltip(move |cx| {
                let focus_handle = focus_handle_1.clone();
                Tooltip::for_action_in(
                    "Commit all staged changes",
                    &CommitStagedChanges,
                    &focus_handle,
                    cx,
                )
            })
            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                this.commit_staged_changes(&CommitStagedChanges, cx)
            }));

        let commit_all_button = self
            .panel_button("commit-all-changes", "Commit All")
            .tooltip(move |cx| {
                let focus_handle = focus_handle_2.clone();
                Tooltip::for_action_in(
                    "Commit all changes, including unstaged changes",
                    &CommitAllChanges,
                    &focus_handle,
                    cx,
                )
            })
            .on_click(cx.listener(|this, _: &ClickEvent, cx| {
                this.commit_all_changes(&CommitAllChanges, cx)
            }));

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
                .child(h_flex().child(div().gap_1().flex_grow()).child(
                    if self.current_modifiers.alt {
                        commit_all_button
                    } else {
                        commit_staged_button
                    },
                ))
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
        let project = self.project.read(cx);

        v_flex()
            .id("git_panel")
            .key_context(self.dispatch_context())
            .track_focus(&self.focus_handle)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .when(!project.is_read_only(cx), |this| {
                this.on_action(cx.listener(|this, &StageAll, cx| this.stage_all(&StageAll, cx)))
                    .on_action(
                        cx.listener(|this, &UnstageAll, cx| this.unstage_all(&UnstageAll, cx)),
                    )
                    .on_action(
                        cx.listener(|this, &DiscardAll, cx| this.discard_all(&DiscardAll, cx)),
                    )
                    .on_action(cx.listener(|this, &CommitStagedChanges, cx| {
                        this.commit_staged_changes(&CommitStagedChanges, cx)
                    }))
                    .on_action(cx.listener(|this, &CommitAllChanges, cx| {
                        this.commit_all_changes(&CommitAllChanges, cx)
                    }))
            })
            .size_full()
            .overflow_hidden()
            .font_buffer(cx)
            .py_1()
            .bg(ElevationIndex::Surface.bg(cx))
            .child(self.render_panel_header(cx))
            .child(self.render_divider(cx))
            .child(self.render_empty_state(cx))
            .child(self.render_divider(cx))
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
