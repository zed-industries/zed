mod active_toolchain;

pub use active_toolchain::ActiveToolchain;
use editor::Editor;
use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ParentElement, Render, Styled, Task, WeakEntity, Window,
};
use language::{LanguageName, Toolchain, ToolchainList};
use picker::{Picker, PickerDelegate};
use project::{Project, WorktreeId};
use std::{path::Path, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(toolchain, [Select]);

pub fn init(cx: &mut App) {
    cx.observe_new(ToolchainSelector::register).detach();
}

pub struct ToolchainSelector {
    picker: Entity<Picker<ToolchainSelectorDelegate>>,
}

impl ToolchainSelector {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(move |workspace, _: &Select, window, cx| {
            Self::toggle(workspace, window, cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Option<()> {
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
        cx.spawn_in(window, move |workspace, mut cx| async move {
            let active_toolchain = workspace::WORKSPACE_DB
                .toolchain(workspace_id, worktree_id, language_name.clone())
                .await
                .ok()
                .flatten();
            workspace
                .update_in(&mut cx, |this, window, cx| {
                    this.toggle_modal(window, cx, move |window, cx| {
                        ToolchainSelector::new(
                            weak,
                            project,
                            active_toolchain,
                            worktree_id,
                            worktree_root_path,
                            language_name,
                            window,
                            cx,
                        )
                    });
                })
                .ok();
        })
        .detach();

        Some(())
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        active_toolchain: Option<Toolchain>,
        worktree_id: WorktreeId,
        worktree_root: Arc<Path>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let toolchain_selector = cx.entity().downgrade();
        let picker = cx.new(|cx| {
            let delegate = ToolchainSelectorDelegate::new(
                active_toolchain,
                toolchain_selector,
                workspace,
                worktree_id,
                worktree_root,
                project,
                language_name,
                window,
                cx,
            );
            Picker::uniform_list(delegate, window, cx)
        });
        Self { picker }
    }
}

impl Render for ToolchainSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl Focusable for ToolchainSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for ToolchainSelector {}
impl ModalView for ToolchainSelector {}

pub struct ToolchainSelectorDelegate {
    toolchain_selector: WeakEntity<ToolchainSelector>,
    candidates: ToolchainList,
    matches: Vec<StringMatch>,
    selected_index: usize,
    workspace: WeakEntity<Workspace>,
    worktree_id: WorktreeId,
    worktree_abs_path_root: Arc<Path>,
    placeholder_text: Arc<str>,
    _fetch_candidates_task: Task<Option<()>>,
}

impl ToolchainSelectorDelegate {
    #[allow(clippy::too_many_arguments)]
    fn new(
        active_toolchain: Option<Toolchain>,
        toolchain_selector: WeakEntity<ToolchainSelector>,
        workspace: WeakEntity<Workspace>,
        worktree_id: WorktreeId,
        worktree_abs_path_root: Arc<Path>,
        project: Entity<Project>,
        language_name: LanguageName,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Self {
        let _fetch_candidates_task = cx.spawn_in(window, {
            let project = project.clone();
            move |this, mut cx| async move {
                let term = project
                    .update(&mut cx, |this, _| {
                        Project::toolchain_term(this.languages().clone(), language_name.clone())
                    })
                    .ok()?
                    .await?;
                let placeholder_text = format!("Select a {}…", term.to_lowercase()).into();
                let _ = this.update_in(&mut cx, move |this, window, cx| {
                    this.delegate.placeholder_text = placeholder_text;
                    this.refresh_placeholder(window, cx);
                });
                let available_toolchains = project
                    .update(&mut cx, |this, cx| {
                        this.available_toolchains(worktree_id, language_name, cx)
                    })
                    .ok()?
                    .await?;

                let _ = this.update_in(&mut cx, move |this, window, cx| {
                    this.delegate.candidates = available_toolchains;

                    if let Some(active_toolchain) = active_toolchain {
                        if let Some(position) = this
                            .delegate
                            .candidates
                            .toolchains
                            .iter()
                            .position(|toolchain| *toolchain == active_toolchain)
                        {
                            this.delegate.set_selected_index(position, window, cx);
                        }
                    }
                    this.update_matches(this.query(cx), window, cx);
                });

                Some(())
            }
        });
        let placeholder_text = "Select a toolchain…".to_string().into();
        Self {
            toolchain_selector,
            candidates: Default::default(),
            matches: vec![],
            selected_index: 0,
            workspace,
            worktree_id,
            worktree_abs_path_root,
            placeholder_text,
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

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        self.placeholder_text.clone()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
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
                cx.spawn_in(window, |_, mut cx| async move {
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
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.toolchain_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        let worktree_root_path = self.worktree_abs_path_root.clone();
        cx.spawn_in(window, |this, mut cx| async move {
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
                        StringMatchCandidate::new(candidate_id, &string)
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
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
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
                .toggle_state(selected)
                .child(HighlightedLabel::new(label, name_highlights))
                .child(
                    HighlightedLabel::new(path, path_highlights)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}
