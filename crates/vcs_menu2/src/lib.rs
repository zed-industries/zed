use anyhow::{anyhow, bail, Result};
use fs::repository::Branch;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions, rems, AppContext, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView,
    ParentElement, Render, SharedString, Styled, Task, View, ViewContext, VisualContext,
    WindowContext,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{v_stack, HighlightedLabel, ListItem, Selectable};
use util::ResultExt;
use workspace::{ModalView, Toast, Workspace};

actions!(branches, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    // todo!() po
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, action, cx| {
            ModalBranchList::toggle_modal(workspace, action, cx).log_err();
        });
    })
    .detach();
}
pub type BranchList = Picker<BranchListDelegate>;

pub struct ModalBranchList {
    pub picker: View<Picker<BranchListDelegate>>,
}

impl ModalView for ModalBranchList {}
impl EventEmitter<DismissEvent> for ModalBranchList {}

impl FocusableView for ModalBranchList {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ModalBranchList {
    type Element = Div;

    fn render(&mut self, _cx: &mut ViewContext<Self>) -> Self::Element {
        v_stack().w(rems(34.)).child(self.picker.clone())
    }
}

pub fn build_branch_list(
    workspace: View<Workspace>,
    cx: &mut WindowContext<'_>,
) -> Result<View<BranchList>> {
    let delegate = workspace.update(cx, |workspace, cx| {
        BranchListDelegate::new(workspace, cx.view().clone(), 29, cx)
    })?;

    Ok(cx.build_view(|cx| Picker::new(delegate, cx)))
}

impl ModalBranchList {
    fn toggle_modal(
        workspace: &mut Workspace,
        _: &OpenRecent,
        cx: &mut ViewContext<Workspace>,
    ) -> Result<()> {
        // Modal branch picker has a longer trailoff than a popover one.
        let delegate = BranchListDelegate::new(workspace, cx.view().clone(), 70, cx)?;
        workspace.toggle_modal(cx, |cx| {
            let modal = ModalBranchList {
                picker: cx.build_view(|cx| Picker::new(delegate, cx)),
            };
            cx.subscribe(&modal.picker, |_, _, _, cx| cx.emit(DismissEvent))
                .detach();
            modal
        });

        Ok(())
    }
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
        let Some(worktree) = project.visible_worktrees(cx).next() else {
            bail!("Cannot update branch list as there are no visible worktrees")
        };

        let mut cwd = worktree.read(cx).abs_path().to_path_buf();
        cwd.push(".git");
        let Some(repo) = project.fs().open_repo(&cwd) else {
            bail!("Project does not have associated git repository.")
        };
        let all_branches = repo.lock().branches()?;
        Ok(Self {
            matches: vec![],
            workspace: handle,
            all_branches,
            selected_index: 0,
            last_query: Default::default(),
            branch_name_trailoff_after,
        })
    }

    fn display_error_toast(&self, message: String, cx: &mut ViewContext<BranchList>) {
        const GIT_CHECKOUT_FAILURE_ID: usize = 2048;
        self.workspace.update(cx, |model, ctx| {
            model.show_toast(Toast::new(GIT_CHECKOUT_FAILURE_ID, message), ctx)
        });
    }
}

impl PickerDelegate for BranchListDelegate {
    type ListItem = ListItem;
    fn placeholder_text(&self) -> Arc<str> {
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
                if query.is_empty() && branches.len() > RECENT_BRANCHES_COUNT {
                    // Truncate list of recent branches
                    // Do a partial sort to show recent-ish branches first.
                    branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT - 1, |lhs, rhs| {
                        rhs.unix_timestamp.cmp(&lhs.unix_timestamp)
                    });
                    branches.truncate(RECENT_BRANCHES_COUNT);
                    branches.sort_unstable_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
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
                    let mut cwd = project
                        .visible_worktrees(cx)
                        .next()
                        .ok_or_else(|| anyhow!("There are no visisible worktrees."))?
                        .read(cx)
                        .abs_path()
                        .to_path_buf();
                    cwd.push(".git");
                    let status = project
                        .fs()
                        .open_repo(&cwd)
                        .ok_or_else(|| {
                            anyhow!(
                                "Could not open repository at path `{}`",
                                cwd.as_os_str().to_string_lossy()
                            )
                        })?
                        .lock()
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
                .start_slot(HighlightedLabel::new(shortened_branch_name, highlights))
                .selected(selected),
        )
    }
    // fn render_header(
    //     &self,
    //     cx: &mut ViewContext<Picker<Self>>,
    // ) -> Option<AnyElement<Picker<Self>>> {
    //     let theme = &theme::current(cx);
    //     let style = theme.picker.header.clone();
    //     let label = if self.last_query.is_empty() {
    //         Flex::row()
    //             .with_child(Label::new("Recent branches", style.label.clone()))
    //             .contained()
    //             .with_style(style.container)
    //     } else {
    //         Flex::row()
    //             .with_child(Label::new("Branches", style.label.clone()))
    //             .with_children(self.matches.is_empty().not().then(|| {
    //                 let suffix = if self.matches.len() == 1 { "" } else { "es" };
    //                 Label::new(
    //                     format!("{} match{}", self.matches.len(), suffix),
    //                     style.label,
    //                 )
    //                 .flex_float()
    //             }))
    //             .contained()
    //             .with_style(style.container)
    //     };
    //     Some(label.into_any())
    // }
    // fn render_footer(
    //     &self,
    //     cx: &mut ViewContext<Picker<Self>>,
    // ) -> Option<AnyElement<Picker<Self>>> {
    //     if !self.last_query.is_empty() {
    //         let theme = &theme::current(cx);
    //         let style = theme.picker.footer.clone();
    //         enum BranchCreateButton {}
    //         Some(
    //             Flex::row().with_child(MouseEventHandler::new::<BranchCreateButton, _>(0, cx, |state, _| {
    //                 let style = style.style_for(state);
    //                 Label::new("Create branch", style.label.clone())
    //                     .contained()
    //                     .with_style(style.container)
    //             })
    //             .with_cursor_style(CursorStyle::PointingHand)
    //             .on_down(MouseButton::Left, |_, _, cx| {
    //                 cx.spawn(|picker, mut cx| async move {
    //                     picker.update(&mut cx, |this, cx| {
    //                         let project = this.delegate().workspace.read(cx).project().read(cx);
    //                         let current_pick = &this.delegate().last_query;
    //                         let mut cwd = project
    //                         .visible_worktrees(cx)
    //                         .next()
    //                         .ok_or_else(|| anyhow!("There are no visisible worktrees."))?
    //                         .read(cx)
    //                         .abs_path()
    //                         .to_path_buf();
    //                         cwd.push(".git");
    //                         let repo = project
    //                             .fs()
    //                             .open_repo(&cwd)
    //                             .ok_or_else(|| anyhow!("Could not open repository at path `{}`", cwd.as_os_str().to_string_lossy()))?;
    //                         let repo = repo
    //                             .lock();
    //                         let status = repo
    //                             .create_branch(&current_pick);
    //                         if status.is_err() {
    //                             this.delegate().display_error_toast(format!("Failed to create branch '{current_pick}', check for conflicts or unstashed files"), cx);
    //                             status?;
    //                         }
    //                         let status = repo.change_branch(&current_pick);
    //                         if status.is_err() {
    //                             this.delegate().display_error_toast(format!("Failed to chec branch '{current_pick}', check for conflicts or unstashed files"), cx);
    //                             status?;
    //                         }
    //                         cx.emit(PickerEvent::Dismiss);
    //                         Ok::<(), anyhow::Error>(())
    //             })
    //                 }).detach();
    //             })).aligned().right()
    //             .into_any(),
    //         )
    //     } else {
    //         None
    //     }
    // }
}
