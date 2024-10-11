use anyhow::{Context, Result};
use fuzzy::{StringMatch, StringMatchCandidate};
use git::repository::Branch;
use gpui::{
    actions, rems, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Subscription,
    Task, View, ViewContext, VisualContext, WindowContext,
};
use picker::{Picker, PickerDelegate};
use std::{ops::Not, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::{ModalView, Toast, Workspace};

actions!(branches, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, action, cx| {
            BranchList::open(workspace, action, cx).log_err();
        });
    })
    .detach();
}

pub struct BranchList {
    pub picker: View<Picker<BranchListDelegate>>,
    rem_width: f32,
    _subscription: Subscription,
}

impl BranchList {
    fn new(delegate: BranchListDelegate, rem_width: f32, cx: &mut ViewContext<Self>) -> Self {
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
        let _subscription = cx.subscribe(&picker, |_, _, _, cx| cx.emit(DismissEvent));
        Self {
            picker,
            rem_width,
            _subscription,
        }
    }
    pub fn open(
        workspace: &mut Workspace,
        _: &OpenRecent,
        cx: &mut ViewContext<Workspace>,
    ) -> Result<()> {
        // Modal branch picker has a longer trailoff than a popover one.
        let delegate = BranchListDelegate::new(workspace, cx.view().clone(), 70, cx)?;
        workspace.toggle_modal(cx, |cx| BranchList::new(delegate, 34., cx));

        Ok(())
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
    workspace: View<Workspace>,
    selected_index: usize,
    last_query: String,
    /// Max length of branch name before we truncate it and add a trailing `...`.
    branch_name_trailoff_after: usize,
}

impl BranchListDelegate {
    fn new(
        workspace: &Workspace,
        handle: View<Workspace>,
        branch_name_trailoff_after: usize,
        cx: &AppContext,
    ) -> Result<Self> {
        let project = workspace.project().read(cx);
        let repo = project
            .get_first_worktree_root_repo(cx)
            .context("failed to get root repository for first worktree")?;

        let all_branches = repo.branches()?;
        Ok(Self {
            matches: vec![],
            workspace: handle,
            all_branches,
            selected_index: 0,
            last_query: Default::default(),
            branch_name_trailoff_after,
        })
    }

    fn display_error_toast(&self, message: String, cx: &mut WindowContext<'_>) {
        self.workspace.update(cx, |model, ctx| {
            struct GitCheckoutFailure;
            let id = NotificationId::unique::<GitCheckoutFailure>();

            model.show_toast(Toast::new(id, message), ctx)
        });
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
                picker
                    .update(&mut cx, |this, cx| {
                        let project = this.delegate.workspace.read(cx).project().read(cx);
                        let repo = project
                            .get_first_worktree_root_repo(cx)
                            .context("failed to get root repository for first worktree")?;

                        let branch_to_checkout = match branch {
                            BranchEntry::Branch(branch) => branch.string,
                            BranchEntry::NewBranch { name: branch_name } => {
                                let status = repo.create_branch(&branch_name);
                                if status.is_err() {
                                    this.delegate.display_error_toast(format!("Failed to create branch '{branch_name}', check for conflicts or unstashed files"), cx);
                                    status?;
                                }

                                branch_name
                            }
                        };

                        let status = repo.change_branch(&branch_to_checkout);
                        if status.is_err() {
                            this.delegate.display_error_toast(format!("Failed to checkout branch '{branch_to_checkout}', check for conflicts or unstashed files"), cx);
                            status?;
                        }

                        cx.emit(DismissEvent);

                        Ok::<(), anyhow::Error>(())
                    })
                    .log_err();
            }
        })
        .detach();
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
