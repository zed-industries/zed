use anyhow::Context as _;
use fuzzy::StringMatchCandidate;

use git::repository::Worktree as GitWorktree;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use std::{path::PathBuf, sync::Arc};
use ui::{HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace, notifications::DetachAndPromptErr};

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::Worktree,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx);
    let workspace_handle = workspace.weak_handle();
    workspace.toggle_modal(window, cx, |window, cx| {
        WorktreeList::new(repository, workspace_handle, rems(34.), window, cx)
    })
}

pub struct WorktreeList {
    width: Rems,
    pub picker: Entity<Picker<WorktreeListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscription: Subscription,
}

impl WorktreeList {
    fn new(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let all_worktrees_request = repository
            .map(|repository| repository.update(cx, |repository, _| repository.worktrees()));

        cx.spawn_in(window, async move |this, cx| {
            let all_worktrees = all_worktrees_request
                .context("No active repository")?
                .await??;

            this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.all_worktrees = Some(all_worktrees);
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = WorktreeListDelegate::new(workspace, window, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscription,
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker
            .update(cx, |picker, _| picker.delegate.modifiers = ev.modifiers)
    }
}
impl ModalView for WorktreeList {}
impl EventEmitter<DismissEvent> for WorktreeList {}

impl Focusable for WorktreeList {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for WorktreeList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitWorktreeSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
            .on_mouse_down_out({
                cx.listener(move |this, _, window, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.cancel(&Default::default(), window, cx);
                    })
                })
            })
    }
}

#[derive(Debug, Clone)]
struct WorktreeEntry {
    worktree: GitWorktree,
    positions: Vec<usize>,
}

pub struct WorktreeListDelegate {
    matches: Vec<WorktreeEntry>,
    all_worktrees: Option<Vec<GitWorktree>>,
    workspace: WeakEntity<Workspace>,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
    focus_handle: FocusHandle,
}

impl WorktreeListDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        _window: &mut Window,
        cx: &mut Context<WorktreeList>,
    ) -> Self {
        Self {
            matches: vec![],
            all_worktrees: None,
            workspace,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn open_worktree(
        &self,
        worktree_path: &PathBuf,
        replace_current_window: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let workspace = self.workspace.clone();
        let path = worktree_path.clone();

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_workspace_for_paths(replace_current_window, vec![path], window, cx)
        });
        cx.spawn(async move |_, _| {
            open_task?.await?;
            anyhow::Ok(())
        })
        .detach_and_prompt_err("Failed open worktree", window, cx, |e, _, _| {
            Some(e.to_string())
        });

        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for WorktreeListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select worktree…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::Start
    }

    fn match_count(&self) -> usize {
        self.matches.len()
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
    ) -> Task<()> {
        let Some(all_worktrees) = self.all_worktrees.clone() else {
            return Task::ready(());
        };

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<WorktreeEntry> = if query.is_empty() {
                all_worktrees
                    .into_iter()
                    .map(|worktree| WorktreeEntry {
                        worktree,
                        positions: Vec::new(),
                    })
                    .collect()
            } else {
                let candidates = all_worktrees
                    .iter()
                    .enumerate()
                    .map(|(ix, worktree)| StringMatchCandidate::new(ix, worktree.name()))
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| WorktreeEntry {
                    worktree: all_worktrees[candidate.candidate_id].clone(),
                    positions: candidate.positions,
                })
                .collect()
            };
            picker
                .update(cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                    delegate.last_query = query;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index()) else {
            return;
        };

        // If secondary click, we open on a new window
        self.open_worktree(&entry.worktree.path, secondary, window, cx);

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = &self.matches.get(ix)?;
        let path = entry.worktree.path.to_string_lossy().to_string();
        let sha = entry
            .worktree
            .sha
            .clone()
            .chars()
            .take(7)
            .collect::<String>();

        let worktree_name =
            HighlightedLabel::new(entry.worktree.name().to_owned(), entry.positions.clone())
                .truncate()
                .into_any_element();

        Some(
            ListItem::new(SharedString::from(format!("worktree-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .tooltip({
                    let worktree_name = entry.worktree.name().to_string();
                    Tooltip::text(worktree_name)
                })
                .child(
                    v_flex()
                        .w_full()
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .gap_6()
                                .justify_between()
                                .overflow_x_hidden()
                                .child(worktree_name)
                                .child(
                                    Label::new(sha)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                        .into_element(),
                                ),
                        )
                        .child(div().max_w_96().child({
                            Label::new(path)
                                .size(LabelSize::Small)
                                .truncate()
                                .color(Color::Muted)
                        })),
                ),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No worktrees found".into())
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            Button::new("open-in-new-window", "Open in new window")
                                .key_binding(
                                    KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        )
                        .child(
                            Button::new("open-in-window", "Open")
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &menu::SecondaryConfirm,
                                        &focus_handle,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                                }),
                        ),
                )
                .into_any(),
        )
    }
}
