use anyhow::{anyhow, Context as _};
use fuzzy::{StringMatch, StringMatchCandidate};

use git::repository::Branch;
use gpui::{
    rems, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Task, Window,
};
use picker::{Picker, PickerDelegate};
use project::git::Repository;
use std::sync::Arc;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing, PopoverMenuHandle};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(open);
        workspace.register_action(switch);
        workspace.register_action(checkout_branch);
    })
    .detach();
}

pub fn checkout_branch(
    workspace: &mut Workspace,
    _: &zed_actions::git::CheckoutBranch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open(workspace, &zed_actions::git::Branch, window, cx);
}

pub fn switch(
    workspace: &mut Workspace,
    _: &zed_actions::git::Switch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    open(workspace, &zed_actions::git::Branch, window, cx);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::Branch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx).clone();
    let style = BranchListStyle::Modal;
    workspace.toggle_modal(window, cx, |window, cx| {
        BranchList::new(repository, style, 34., window, cx)
    })
}

pub fn popover(
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<BranchList> {
    cx.new(|cx| {
        let list = BranchList::new(repository, BranchListStyle::Popover, 15., window, cx);
        list.focus_handle(cx).focus(window);
        list
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BranchListStyle {
    Modal,
    Popover,
}

pub struct BranchList {
    rem_width: f32,
    pub popover_handle: PopoverMenuHandle<Self>,
    pub picker: Entity<Picker<BranchListDelegate>>,
    _subscription: Subscription,
}

impl BranchList {
    fn new(
        repository: Option<Entity<Repository>>,
        style: BranchListStyle,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let popover_handle = PopoverMenuHandle::default();
        let all_branches_request = repository
            .clone()
            .map(|repository| repository.read(cx).branches());

        cx.spawn_in(window, |this, mut cx| async move {
            let all_branches = all_branches_request
                .context("No active repository")?
                .await??;

            this.update_in(&mut cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.all_branches = Some(all_branches);
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = BranchListDelegate::new(repository.clone(), style, 20);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            rem_width,
            popover_handle,
            _subscription,
        }
    }
}
impl ModalView for BranchList {}
impl EventEmitter<DismissEvent> for BranchList {}

impl Focusable for BranchList {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BranchList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(self.rem_width))
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
enum BranchEntry {
    Branch(StringMatch),
    History(String),
    NewBranch { name: String },
}

impl BranchEntry {
    fn name(&self) -> &str {
        match self {
            Self::Branch(branch) => &branch.string,
            Self::History(branch) => &branch,
            Self::NewBranch { name } => &name,
        }
    }
}

pub struct BranchListDelegate {
    matches: Vec<BranchEntry>,
    all_branches: Option<Vec<Branch>>,
    repo: Option<Entity<Repository>>,
    style: BranchListStyle,
    selected_index: usize,
    last_query: String,
    /// Max length of branch name before we truncate it and add a trailing `...`.
    branch_name_trailoff_after: usize,
}

impl BranchListDelegate {
    fn new(
        repo: Option<Entity<Repository>>,
        style: BranchListStyle,
        branch_name_trailoff_after: usize,
    ) -> Self {
        Self {
            matches: vec![],
            repo,
            style,
            all_branches: None,
            selected_index: 0,
            last_query: Default::default(),
            branch_name_trailoff_after,
        }
    }

    pub fn branch_count(&self) -> usize {
        self.matches
            .iter()
            .filter(|item| matches!(item, BranchEntry::Branch(_)))
            .count()
    }
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select branch...".into()
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
        let Some(mut all_branches) = self.all_branches.clone() else {
            return Task::ready(());
        };

        cx.spawn_in(window, move |picker, mut cx| async move {
            const RECENT_BRANCHES_COUNT: usize = 10;
            if query.is_empty() {
                if all_branches.len() > RECENT_BRANCHES_COUNT {
                    // Truncate list of recent branches
                    // Do a partial sort to show recent-ish branches first.
                    all_branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT - 1, |lhs, rhs| {
                        rhs.priority_key().cmp(&lhs.priority_key())
                    });
                    all_branches.truncate(RECENT_BRANCHES_COUNT);
                }
                all_branches.sort_unstable_by(|lhs, rhs| {
                    rhs.is_head.cmp(&lhs.is_head).then(lhs.name.cmp(&rhs.name))
                });
            }

            let candidates = all_branches
                .into_iter()
                .enumerate()
                .map(|(ix, command)| StringMatchCandidate::new(ix, &command.name))
                .collect::<Vec<StringMatchCandidate>>();
            let matches: Vec<BranchEntry> = if query.is_empty() {
                candidates
                    .into_iter()
                    .map(|candidate| BranchEntry::History(candidate.string))
                    .collect()
            } else {
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .iter()
                .cloned()
                .map(BranchEntry::Branch)
                .collect()
            };
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        if !query.is_empty() {
                            delegate.matches.push(BranchEntry::NewBranch {
                                name: query.trim().replace(' ', "-"),
                            });
                        }

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

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(branch) = self.matches.get(self.selected_index()) else {
            return;
        };

        let current_branch = self.repo.as_ref().map(|repo| {
            repo.update(cx, |repo, _| {
                repo.current_branch().map(|branch| branch.name.clone())
            })
        });

        if current_branch
            .flatten()
            .is_some_and(|current_branch| current_branch == branch.name())
        {
            cx.emit(DismissEvent);
            return;
        }

        cx.spawn_in(window, {
            let branch = branch.clone();
            |picker, mut cx| async move {
                let branch_change_task = picker.update(&mut cx, |this, cx| {
                    let repo = this
                        .delegate
                        .repo
                        .as_ref()
                        .ok_or_else(|| anyhow!("No active repository"))?
                        .clone();

                    let cx = cx.to_async();

                    anyhow::Ok(async move {
                        match branch {
                            BranchEntry::Branch(StringMatch {
                                string: branch_name,
                                ..
                            })
                            | BranchEntry::History(branch_name) => {
                                cx.update(|cx| repo.read(cx).change_branch(branch_name))?
                                    .await?
                            }
                            BranchEntry::NewBranch { name: branch_name } => {
                                cx.update(|cx| repo.read(cx).create_branch(branch_name.clone()))?
                                    .await??;
                                cx.update(|cx| repo.read(cx).change_branch(branch_name))?
                                    .await?
                            }
                        }
                    })
                })??;

                branch_change_task.await?;

                picker.update(&mut cx, |_, cx| {
                    cx.emit(DismissEvent);

                    anyhow::Ok(())
                })
            }
        })
        .detach_and_prompt_err("Failed to change branch", window, cx, |_, _, _| None);
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
        let hit = &self.matches[ix];
        let shortened_branch_name =
            util::truncate_and_trailoff(&hit.name(), self.branch_name_trailoff_after);

        Some(
            ListItem::new(SharedString::from(format!("vcs-menu-{ix}")))
                .inset(true)
                .spacing(match self.style {
                    BranchListStyle::Modal => ListItemSpacing::default(),
                    BranchListStyle::Popover => ListItemSpacing::ExtraDense,
                })
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .when(matches!(hit, BranchEntry::History(_)), |el| {
                    el.end_slot(
                        Icon::new(IconName::HistoryRerun)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                })
                .map(|el| match hit {
                    BranchEntry::Branch(branch) => {
                        let highlights: Vec<_> = branch
                            .positions
                            .iter()
                            .filter(|index| index < &&self.branch_name_trailoff_after)
                            .copied()
                            .collect();

                        el.child(HighlightedLabel::new(shortened_branch_name, highlights))
                    }
                    BranchEntry::History(_) => el.child(Label::new(shortened_branch_name)),
                    BranchEntry::NewBranch { name } => {
                        el.child(Label::new(format!("Create branch '{name}'")))
                    }
                }),
        )
    }
}
