mod active_toolchain;

pub use active_toolchain::ActiveToolchain;
use editor::Editor;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use language::{LanguageName, Toolchain, ToolchainList};
use picker::{Picker, PickerDelegate};
use project::{Project, WorktreeId};
use std::{path::Path, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(toolchain, [Select]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(ToolchainSelector::register).detach();
}

pub struct ToolchainSelector {
    picker: View<Picker<ToolchainSelectorDelegate>>,
}

impl ToolchainSelector {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(move |workspace, _: &Select, cx| {
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
        let worktree_root_path = project
            .read(cx)
            .worktree_for_id(worktree_id, cx)?
            .read(cx)
            .abs_path();
        let workspace_id = workspace.database_id()?;
        let weak = workspace.weak_handle();
        cx.spawn(move |workspace, mut cx| async move {
            let active_toolchain = workspace::WORKSPACE_DB
                .toolchain(workspace_id, worktree_id, language_name.clone())
                .await
                .ok()
                .flatten();
            workspace
                .update(&mut cx, |this, cx| {
                    this.toggle_modal(cx, move |cx| {
                        ToolchainSelector::new(
                            weak,
                            project,
                            active_toolchain,
                            worktree_id,
                            worktree_root_path,
                            language_name,
                            cx,
                        )
                    });
                })
                .ok();
        })
        .detach();

        Some(())
    }

    fn new(
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        active_toolchain: Option<Toolchain>,
        worktree_id: WorktreeId,
        worktree_root: Arc<Path>,
        language_name: LanguageName,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let view = cx.view().downgrade();
        let picker = cx.new_view(|cx| {
            let delegate = ToolchainSelectorDelegate::new(
                active_toolchain,
                view,
                workspace,
                worktree_id,
                worktree_root,
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
    worktree_abs_path_root: Arc<Path>,
    _fetch_candidates_task: Task<Option<()>>,
}

impl ToolchainSelectorDelegate {
    #[allow(clippy::too_many_arguments)]
    fn new(
        active_toolchain: Option<Toolchain>,
        language_selector: WeakView<ToolchainSelector>,
        workspace: WeakView<Workspace>,
        worktree_id: WorktreeId,
        worktree_abs_path_root: Arc<Path>,
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
                    if let Some(active_toolchain) = active_toolchain {
                        if let Some(position) = this
                            .delegate
                            .candidates
                            .toolchains
                            .iter()
                            .position(|toolchain| *toolchain == active_toolchain)
                        {
                            this.delegate.set_selected_index(position, cx);
                        }
                    }
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
            worktree_abs_path_root,
            _fetch_candidates_task,
        }
    }
    fn relativize_path(path: SharedString, worktree_root: &Path) -> SharedString {
        Path::new(&path.as_ref())
            .strip_prefix(&worktree_root)
            .ok()
            .map(|suffix| Path::new(".").join(suffix))
            .and_then(|path| path.to_str().map(String::from).map(SharedString::from))
            .unwrap_or(path)
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
                let worktree_id = self.worktree_id;
                cx.spawn(|_, mut cx| async move {
                    workspace::WORKSPACE_DB
                        .set_toolchain(workspace_id, worktree_id, toolchain.clone())
                        .await
                        .log_err();
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
        let worktree_root_path = self.worktree_abs_path_root.clone();
        cx.spawn(|this, mut cx| async move {
            let matches = if query.is_empty() {
                candidates
                    .toolchains
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let path = Self::relativize_path(candidate.path, &worktree_root_path);
                        let string = format!("{}{}", candidate.name, path);
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
                        let path = Self::relativize_path(toolchain.path, &worktree_root_path);
                        let string = format!("{}{}", toolchain.name, path);
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

        let label = toolchain.name.clone();
        let path = Self::relativize_path(toolchain.path.clone(), &self.worktree_abs_path_root);
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
