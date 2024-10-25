mod active_toolchain;

pub use active_toolchain::ActiveToolchain;
use editor::Editor;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use language::{LanguageName, ToolchainList};
use picker::{Picker, PickerDelegate};
use project::{Project, WorktreeId};
use std::sync::Arc;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(toolchain_selector, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(ToolchainSelector::register).detach();
}

pub struct ToolchainSelector {
    picker: View<Picker<ToolchainSelectorDelegate>>,
}

impl ToolchainSelector {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &Toggle, cx| {
            Self::toggle(workspace, cx);
        });
    }

    fn toggle(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Option<()> {
        let (_, buffer, _) = workspace
            .active_item(cx)?
            .act_as::<Editor>(cx)?
            .read(cx)
            .active_excerpt(cx)?;
        let project = workspace.project().clone();

        let language_name = buffer.read(cx).language()?.name();
        let worktree_id = buffer.read(cx).file()?.worktree_id(cx);
        let weak = workspace.weak_handle();
        workspace.toggle_modal(cx, move |cx| {
            ToolchainSelector::new(weak, project, worktree_id, language_name, cx)
        });
        Some(())
    }

    fn new(
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let view = cx.view().downgrade();
        let picker = cx.new_view(|cx| {
            let delegate = ToolchainSelectorDelegate::new(
                view,
                workspace,
                worktree_id,
                project,
                language_name,
                cx,
            );
            Picker::uniform_list(delegate, cx)
        });
        Self { picker }
    }
}

impl Render for ToolchainSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl FocusableView for ToolchainSelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ToolchainSelector {}
impl ModalView for ToolchainSelector {}

pub struct ToolchainSelectorDelegate {
    toolchain_selector: WeakView<ToolchainSelector>,
    candidates: ToolchainList,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakView<Workspace>,
    worktree_id: WorktreeId,
    _fetch_candidates_task: Task<Option<()>>,
}

impl ToolchainSelectorDelegate {
    fn new(
        language_selector: WeakView<ToolchainSelector>,
        workspace: WeakView<Workspace>,
        worktree_id: WorktreeId,
        project: Model<Project>,
        language_name: LanguageName,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Self {
        let _fetch_candidates_task = cx.spawn({
            let project = project.clone();
            move |this, mut cx| async move {
                let available_toolchains = project
                    .update(&mut cx, |this, cx| {
                        this.available_toolchains(worktree_id, language_name, cx)
                    })
                    .ok()?
                    .await?;

                let _ = this.update(&mut cx, move |this, cx| {
                    this.delegate.candidates = available_toolchains;
                    this.update_matches(this.query(cx), cx);
                });

                Some(())
            }
        });

        Self {
            toolchain_selector: language_selector,
            candidates: Default::default(),
            matches: vec![],
            selected_index: 0,
            workspace,
            worktree_id,
            _fetch_candidates_task,
        }
    }
}

impl PickerDelegate for ToolchainSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a toolchain...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(string_match) = self.matches.get(self.selected_index) {
            let toolchain = self.candidates.toolchains[string_match.candidate_id].clone();
            if let Some(workspace_id) = self
                .workspace
                .update(cx, |this, _| this.database_id())
                .ok()
                .flatten()
            {
                let workspace = self.workspace.clone();
                cx.spawn(|this, mut cx| async move {
                    let worktree_id = this
                        .update(&mut cx, |this, _| this.delegate.worktree_id)
                        .ok()?;
                    workspace::WORKSPACE_DB
                        .set_toolchain(workspace_id, worktree_id, toolchain.clone())
                        .await
                        .ok()?;

                    workspace
                        .update(&mut cx, |this, cx| {
                            this.project().update(cx, |this, cx| {
                                this.activate_toolchain(worktree_id, toolchain, cx)
                            })
                        })
                        .ok()?
                        .await;
                    Some(())
                })
                .detach();
            }
        }
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.toolchain_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn(|this, mut cx| async move {
            let matches = if query.is_empty() {
                candidates
                    .toolchains
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let string = format!("{}{}", candidate.label, candidate.path);
                        StringMatch {
                            candidate_id: index,
                            string,
                            positions: Vec::new(),
                            score: 0.0,
                        }
                    })
                    .collect()
            } else {
                let candidates = candidates
                    .toolchains
                    .into_iter()
                    .enumerate()
                    .map(|(candidate_id, toolchain)| {
                        let string = format!("{}{}", toolchain.label, toolchain.path);
                        StringMatchCandidate::new(candidate_id, string)
                    })
                    .collect::<Vec<_>>();
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(&mut cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let toolchain = &self.candidates.toolchains[mat.candidate_id];

        let label = toolchain.label.clone();
        let path = toolchain.path.clone();
        let (name_highlights, mut path_highlights) = mat
            .positions
            .iter()
            .cloned()
            .partition::<Vec<_>, _>(|index| *index < label.len());
        path_highlights.iter_mut().for_each(|index| {
            *index -= label.len();
        });
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(label, name_highlights))
                .child(
                    HighlightedLabel::new(path, path_highlights)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}
