use anyhow::{Context, Result};
use fuzzy::{StringMatch, StringMatchCandidate};
use git::repository::Branch;
use gpui::{
    actions, rems, AnyElement, AppContext, DismissEvent, Element, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Subscription, Task, View, ViewContext, VisualContext, WindowContext,
};
use picker::{Picker, PickerDelegate};
use std::{ops::Not, sync::Arc};
use ui::{
    h_flex, v_flex, Button, ButtonCommon, Clickable, Color, HighlightedLabel, Label, LabelCommon,
    LabelSize, ListItem, ListItemSpacing, Selectable,
};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::{ModalView, Toast, Workspace};

actions!(branches, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, action, cx| {
            BranchList::toggle_modal(workspace, action, cx).log_err();
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
    fn toggle_modal(
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

pub fn build_branch_list(
    workspace: View<Workspace>,
    cx: &mut WindowContext<'_>,
) -> Result<View<BranchList>> {
    let delegate = workspace.update(cx, |workspace, cx| {
        BranchListDelegate::new(workspace, cx.view().clone(), 29, cx)
    })?;
    Ok(cx.new_view(move |cx| BranchList::new(delegate, 20., cx)))
}

pub struct BranchListDelegate {
    matches: Vec<StringMatch>,
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
        let project = workspace.project().read(&cx);
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

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        let current_pick = self.selected_index();
        let Some(current_pick) = self
            .matches
            .get(current_pick)
            .map(|pick| pick.string.clone())
        else {
            return;
        };
        cx.spawn(|picker, mut cx| async move {
            picker
                .update(&mut cx, |this, cx| {
                    let project = this.delegate.workspace.read(cx).project().read(cx);
                    let repo = project
                        .get_first_worktree_root_repo(cx)
                        .context("failed to get root repository for first worktree")?;
                    let status = repo
                        .change_branch(&current_pick);
                    if status.is_err() {
                        this.delegate.display_error_toast(format!("Failed to checkout branch '{current_pick}', check for conflicts or unstashed files"), cx);
                        status?;
                    }
                    cx.emit(DismissEvent);

                    Ok::<(), anyhow::Error>(())
                })
                .log_err();
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
            util::truncate_and_trailoff(&hit.string, self.branch_name_trailoff_after);
        let highlights: Vec<_> = hit
            .positions
            .iter()
            .filter(|index| index < &&self.branch_name_trailoff_after)
            .copied()
            .collect();
        Some(
            ListItem::new(SharedString::from(format!("vcs-menu-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .start_slot(HighlightedLabel::new(shortened_branch_name, highlights)),
        )
    }
    fn render_header(&self, _: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        let label = if self.last_query.is_empty() {
            h_flex()
                .ml_3()
                .child(Label::new("Recent Branches").size(LabelSize::Small))
        } else {
            let match_label = self.matches.is_empty().not().then(|| {
                let suffix = if self.matches.len() == 1 { "" } else { "es" };
                Label::new(format!("{} match{}", self.matches.len(), suffix))
                    .color(Color::Muted)
                    .size(LabelSize::Small)
            });
            h_flex()
                .px_3()
                .h_full()
                .justify_between()
                .child(Label::new("Branches").size(LabelSize::Small))
                .children(match_label)
        };
        Some(label.mt_1().into_any())
    }
    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<AnyElement> {
        if self.last_query.is_empty() {
            return None;
        }

        Some(
            h_flex().mr_3().pb_2().child(h_flex().w_full()).child(
            Button::new("branch-picker-create-branch-button", "Create branch").on_click(
                cx.listener(|_, _, cx| {
                    cx.spawn(|picker, mut cx| async move {
                                        picker.update(&mut cx, |this, cx| {
                                            let project = this.delegate.workspace.read(cx).project().read(cx);
                                            let current_pick = &this.delegate.last_query;
                                            let repo = project
                                                .get_first_worktree_root_repo(cx)
                                                .context("failed to get root repository for first worktree")?;
                                            let status = repo
                                                .create_branch(&current_pick);
                                            if status.is_err() {
                                                this.delegate.display_error_toast(format!("Failed to create branch '{current_pick}', check for conflicts or unstashed files"), cx);
                                                status?;
                                            }
                                            let status = repo.change_branch(&current_pick);
                                            if status.is_err() {
                                                this.delegate.display_error_toast(format!("Failed to check branch '{current_pick}', check for conflicts or unstashed files"), cx);
                                                status?;
                                            }
                                            this.cancel(&Default::default(), cx);
                                            Ok::<(), anyhow::Error>(())
                                })

                    }).detach_and_log_err(cx);
                }),
            ).style(ui::ButtonStyle::Filled)).into_any_element(),
        )
    }
}
