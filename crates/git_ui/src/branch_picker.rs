use anyhow::{anyhow, Context as _, Result};
use fuzzy::{StringMatch, StringMatchCandidate};

use git::repository::Branch;
use gpui::{
    rems, App, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Task, WeakEntity, Window,
};
use picker::{Picker, PickerDelegate};
use project::ProjectPath;
use std::sync::Arc;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(open);
    })
    .detach();
}

pub fn open(
    _: &mut Workspace,
    _: &zed_actions::git::Branch,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let this = cx.entity().clone();
    cx.spawn_in(window, |_, mut cx| async move {
        // Modal branch picker has a longer trailoff than a popover one.
        let delegate = BranchListDelegate::new(this.clone(), 70, &cx).await?;

        this.update_in(&mut cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                BranchList::new(delegate, 34., window, cx)
            })
        })?;

        Ok(())
    })
    .detach_and_prompt_err("Failed to read branches", window, cx, |_, _, _| None)
}

pub struct BranchList {
    pub picker: Entity<Picker<BranchListDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl BranchList {
    pub fn new(
        delegate: BranchListDelegate,
        rem_width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        Self {
            picker,
            rem_width,
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
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), window, cx);
                })
            }))
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
    all_branches: Vec<Branch>,
    workspace: WeakEntity<Workspace>,
    selected_index: usize,
    last_query: String,
    /// Max length of branch name before we truncate it and add a trailing `...`.
    branch_name_trailoff_after: usize,
}

impl BranchListDelegate {
    pub async fn new(
        workspace: Entity<Workspace>,
        branch_name_trailoff_after: usize,
        cx: &AsyncApp,
    ) -> Result<Self> {
        let all_branches_request = cx.update(|cx| {
            let project = workspace.read(cx).project().read(cx);
            let first_worktree = project
                .visible_worktrees(cx)
                .next()
                .context("No worktrees found")?;
            let project_path = ProjectPath::root_path(first_worktree.read(cx).id());
            anyhow::Ok(project.branches(project_path, cx))
        })??;

        let all_branches = all_branches_request.await?;

        Ok(Self {
            matches: vec![],
            workspace: workspace.downgrade(),
            all_branches,
            selected_index: 0,
            last_query: Default::default(),
            branch_name_trailoff_after,
        })
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
        cx.spawn_in(window, move |picker, mut cx| async move {
            let candidates = picker.update(&mut cx, |picker, _| {
                const RECENT_BRANCHES_COUNT: usize = 10;
                let mut branches = picker.delegate.all_branches.clone();
                if query.is_empty() {
                    if branches.len() > RECENT_BRANCHES_COUNT {
                        // Truncate list of recent branches
                        // Do a partial sort to show recent-ish branches first.
                        branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT - 1, |lhs, rhs| {
                            rhs.priority_key().cmp(&lhs.priority_key())
                        });
                        branches.truncate(RECENT_BRANCHES_COUNT);
                    }
                    branches.sort_unstable_by(|lhs, rhs| {
                        rhs.is_head.cmp(&lhs.is_head).then(lhs.name.cmp(&rhs.name))
                    });
                }
                branches
                    .into_iter()
                    .enumerate()
                    .map(|(ix, command)| StringMatchCandidate::new(ix, &command.name))
                    .collect::<Vec<StringMatchCandidate>>()
            });
            let Some(candidates) = candidates.log_err() else {
                return;
            };
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

        let current_branch = self
            .workspace
            .update(cx, |workspace, cx| {
                workspace
                    .project()
                    .read(cx)
                    .active_repository(cx)
                    .and_then(|repo| repo.read(cx).branch())
                    .map(|branch| branch.name.to_string())
            })
            .ok()
            .flatten();

        if current_branch == Some(branch.name().to_string()) {
            cx.emit(DismissEvent);
            return;
        }

        cx.spawn_in(window, {
            let branch = branch.clone();
            |picker, mut cx| async move {
                let branch_change_task = picker.update(&mut cx, |this, cx| {
                    let workspace = this
                        .delegate
                        .workspace
                        .upgrade()
                        .ok_or_else(|| anyhow!("workspace was dropped"))?;

                    let project = workspace.read(cx).project().read(cx);
                    let branch_to_checkout = match branch {
                        BranchEntry::Branch(branch) => branch.string,
                        BranchEntry::History(string) => string,
                        BranchEntry::NewBranch { name: branch_name } => branch_name,
                    };
                    let worktree = project
                        .visible_worktrees(cx)
                        .next()
                        .context("worktree disappeared")?;
                    let repository = ProjectPath::root_path(worktree.read(cx).id());

                    anyhow::Ok(project.update_or_create_branch(repository, branch_to_checkout, cx))
                })??;

                branch_change_task.await?;

                picker.update(&mut cx, |_, cx| {
                    cx.emit(DismissEvent);

                    Ok::<(), anyhow::Error>(())
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
