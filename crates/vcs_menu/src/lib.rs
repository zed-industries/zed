use anyhow::{anyhow, Context, Result};
use fuzzy::{StringMatch, StringMatchCandidate};
use git::repository::Branch;
use gpui::{
    rems, AnyElement, AppContext, AsyncAppContext, DismissEvent, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Task, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use picker::{Picker, PickerDelegate};
use project::ProjectPath;
use std::{ops::Not, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};
use zed_actions::branches::OpenRecent;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(BranchList::open);
    })
    .detach();
}

pub struct BranchList {
    pub picker: View<Picker<BranchListDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl BranchList {
    pub fn open(_: &mut Workspace, _: &OpenRecent, cx: &mut ViewContext<Workspace>) {
        let this = cx.view().clone();
        cx.spawn(|_, mut cx| async move {
            // Modal branch picker has a longer trailoff than a popover one.
            let delegate = BranchListDelegate::new(this.clone(), 70, &cx).await?;

            this.update(&mut cx, |workspace, cx| {
                workspace.toggle_modal(cx, |cx| BranchList::new(delegate, 34., cx))
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to read branches", cx, |_, _| None)
    }

    fn new(delegate: BranchListDelegate, rem_width: f32, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
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

impl FocusableView for BranchList {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BranchList {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(self.rem_width))
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|this, _, cx| {
                this.picker.update(cx, |this, cx| {
                    this.cancel(&Default::default(), cx);
                })
            }))
    }
}

#[derive(Debug, Clone)]
enum BranchEntry {
    Branch(StringMatch),
    NewBranch { name: String },
}

impl BranchEntry {
    fn name(&self) -> &str {
        match self {
            Self::Branch(branch) => &branch.string,
            Self::NewBranch { name } => &name,
        }
    }
}

pub struct BranchListDelegate {
    matches: Vec<BranchEntry>,
    all_branches: Vec<Branch>,
    workspace: WeakView<Workspace>,
    selected_index: usize,
    last_query: String,
    /// Max length of branch name before we truncate it and add a trailing `...`.
    branch_name_trailoff_after: usize,
}

impl BranchListDelegate {
    async fn new(
        workspace: View<Workspace>,
        branch_name_trailoff_after: usize,
        cx: &AsyncAppContext,
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
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select branch...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        cx.spawn(move |picker, mut cx| async move {
            let candidates = picker.update(&mut cx, |view, _| {
                const RECENT_BRANCHES_COUNT: usize = 10;
                let mut branches = view.delegate.all_branches.clone();
                if query.is_empty() {
                    if branches.len() > RECENT_BRANCHES_COUNT {
                        // Truncate list of recent branches
                        // Do a partial sort to show recent-ish branches first.
                        branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT - 1, |lhs, rhs| {
                            rhs.is_head
                                .cmp(&lhs.is_head)
                                .then(rhs.unix_timestamp.cmp(&lhs.unix_timestamp))
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
                    .map(|(ix, command)| StringMatchCandidate {
                        id: ix,
                        char_bag: command.name.chars().collect(),
                        string: command.name.into(),
                    })
                    .collect::<Vec<StringMatchCandidate>>()
            });
            let Some(candidates) = candidates.log_err() else {
                return;
            };
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
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
            };
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches.into_iter().map(BranchEntry::Branch).collect();
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

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        let Some(branch) = self.matches.get(self.selected_index()) else {
            return;
        };
        cx.spawn({
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
        .detach_and_prompt_err("Failed to change branch", cx, |_, _| None);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let hit = &self.matches[ix];
        let shortened_branch_name =
            util::truncate_and_trailoff(&hit.name(), self.branch_name_trailoff_after);

        Some(
            ListItem::new(SharedString::from(format!("vcs-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .map(|parent| match hit {
                    BranchEntry::Branch(branch) => {
                        let highlights: Vec<_> = branch
                            .positions
                            .iter()
                            .filter(|index| index < &&self.branch_name_trailoff_after)
                            .copied()
                            .collect();

                        parent.child(HighlightedLabel::new(shortened_branch_name, highlights))
                    }
                    BranchEntry::NewBranch { name } => {
                        parent.child(Label::new(format!("Create branch '{name}'")))
                    }
                }),
        )
    }

    fn render_header(&self, _: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        let label = if self.last_query.is_empty() {
            Label::new("Recent Branches")
                .size(LabelSize::Small)
                .mt_1()
                .ml_3()
                .into_any_element()
        } else {
            let match_label = self.matches.is_empty().not().then(|| {
                let suffix = if self.matches.len() == 1 { "" } else { "es" };
                Label::new(format!("{} match{}", self.matches.len(), suffix))
                    .color(Color::Muted)
                    .size(LabelSize::Small)
            });
            h_flex()
                .px_3()
                .justify_between()
                .child(Label::new("Branches").size(LabelSize::Small))
                .children(match_label)
                .into_any_element()
        };
        Some(v_flex().mt_1().child(label).into_any_element())
    }
}
