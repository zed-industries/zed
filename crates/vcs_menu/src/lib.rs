use anyhow::{anyhow, bail, Result};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    AppContext, MouseState, Task, ViewContext, ViewHandle,
};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::{ops::Not, sync::Arc};
use util::ResultExt;
use workspace::{Toast, Workspace};

actions!(branches, [OpenRecent]);

pub fn init(cx: &mut AppContext) {
    Picker::<BranchListDelegate>::init(cx);
    cx.add_async_action(toggle);
}
pub type BranchList = Picker<BranchListDelegate>;

pub fn build_branch_list(
    workspace: ViewHandle<Workspace>,
    cx: &mut ViewContext<BranchList>,
) -> BranchList {
    Picker::new(
        BranchListDelegate {
            matches: vec![],
            workspace,
            selected_index: 0,
            last_query: String::default(),
            branch_name_trailoff_after: 29,
        },
        cx,
    )
    .with_theme(|theme| theme.picker.clone())
}

fn toggle(
    _: &mut Workspace,
    _: &OpenRecent,
    cx: &mut ViewContext<Workspace>,
) -> Option<Task<Result<()>>> {
    Some(cx.spawn(|workspace, mut cx| async move {
        workspace.update(&mut cx, |workspace, cx| {
            workspace.toggle_modal(cx, |_, cx| {
                let workspace = cx.handle();
                cx.add_view(|cx| {
                    Picker::new(
                        BranchListDelegate {
                            matches: vec![],
                            workspace,
                            selected_index: 0,
                            last_query: String::default(),
                            /// Modal branch picker has a longer trailoff than a popover one.
                            branch_name_trailoff_after: 70,
                        },
                        cx,
                    )
                    .with_theme(|theme| theme.picker.clone())
                    .with_max_size(800., 1200.)
                })
            });
        })?;
        Ok(())
    }))
}

pub struct BranchListDelegate {
    matches: Vec<StringMatch>,
    workspace: ViewHandle<Workspace>,
    selected_index: usize,
    last_query: String,
    /// Max length of branch name before we truncate it and add a trailing `...`.
    branch_name_trailoff_after: usize,
}

impl BranchListDelegate {
    fn display_error_toast(&self, message: String, cx: &mut ViewContext<BranchList>) {
        const GIT_CHECKOUT_FAILURE_ID: usize = 2048;
        self.workspace.update(cx, |model, ctx| {
            model.show_toast(Toast::new(GIT_CHECKOUT_FAILURE_ID, message), ctx)
        });
    }
}
impl PickerDelegate for BranchListDelegate {
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
            let Some(candidates) = picker
                .read_with(&mut cx, |view, cx| {
                    let delegate = view.delegate();
                    let project = delegate.workspace.read(cx).project().read(&cx);

                    let Some(worktree) = project
                        .visible_worktrees(cx)
                        .next()
                    else {
                        bail!("Cannot update branch list as there are no visible worktrees")
                    };
                    let mut cwd = worktree .read(cx)
                        .abs_path()
                        .to_path_buf();
                    cwd.push(".git");
                    let Some(repo) = project.fs().open_repo(&cwd) else {bail!("Project does not have associated git repository.")};
                    let mut branches = repo
                        .lock()
                        .branches()?;
                    const RECENT_BRANCHES_COUNT: usize = 10;
                    if query.is_empty() && branches.len() > RECENT_BRANCHES_COUNT {
                        // Truncate list of recent branches
                        // Do a partial sort to show recent-ish branches first.
                        branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT - 1, |lhs, rhs| {
                            rhs.unix_timestamp.cmp(&lhs.unix_timestamp)
                        });
                        branches.truncate(RECENT_BRANCHES_COUNT);
                        branches.sort_unstable_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    }
                    Ok(branches
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(ix, command)| StringMatchCandidate {
                            id: ix,
                            char_bag: command.name.chars().collect(),
                            string: command.name.into(),
                        })
                        .collect::<Vec<_>>())
                })
                .log_err() else { return; };
            let Some(candidates) = candidates.log_err() else {return;};
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
                    cx.background(),
                )
                .await
            };
            picker
                .update(&mut cx, |picker, _| {
                    let delegate = picker.delegate_mut();
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
        let Some(current_pick) = self.matches.get(current_pick).map(|pick| pick.string.clone()) else {
            return;
        };
        cx.spawn(|picker, mut cx| async move {
            picker
                .update(&mut cx, |this, cx| {
                    let project = this.delegate().workspace.read(cx).project().read(cx);
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
                        this.delegate().display_error_toast(format!("Failed to checkout branch '{current_pick}', check for conflicts or unstashed files"), cx);
                        status?;
                    }
                    cx.emit(PickerEvent::Dismiss);

                    Ok::<(), anyhow::Error>(())
                })
                .log_err();
        })
        .detach();
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        cx.emit(PickerEvent::Dismiss);
    }

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &gpui::AppContext,
    ) -> AnyElement<Picker<Self>> {
        let theme = &theme::current(cx);
        let hit = &self.matches[ix];
        let shortened_branch_name =
            util::truncate_and_trailoff(&hit.string, self.branch_name_trailoff_after);
        let highlights = hit
            .positions
            .iter()
            .copied()
            .filter(|index| index < &self.branch_name_trailoff_after)
            .collect();
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);
        Flex::row()
            .with_child(
                Label::new(shortened_branch_name.clone(), style.label.clone())
                    .with_highlights(highlights)
                    .contained()
                    .aligned()
                    .left(),
            )
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(theme.collab_panel.tabbed_modal.row_height)
            .into_any()
    }
    fn render_header(
        &self,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<AnyElement<Picker<Self>>> {
        let theme = &theme::current(cx);
        let style = theme.picker.header.clone();
        let label = if self.last_query.is_empty() {
            Flex::row()
                .with_child(Label::new("Recent branches", style.label.clone()))
                .contained()
                .with_style(style.container)
        } else {
            Flex::row()
                .with_child(Label::new("Branches", style.label.clone()))
                .with_children(self.matches.is_empty().not().then(|| {
                    let suffix = if self.matches.len() == 1 { "" } else { "es" };
                    Label::new(
                        format!("{} match{}", self.matches.len(), suffix),
                        style.label,
                    )
                    .flex_float()
                }))
                .contained()
                .with_style(style.container)
        };
        Some(label.into_any())
    }
    fn render_footer(
        &self,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<AnyElement<Picker<Self>>> {
        if !self.last_query.is_empty() {
            let theme = &theme::current(cx);
            let style = theme.picker.footer.clone();
            enum BranchCreateButton {}
            Some(
                Flex::row().with_child(MouseEventHandler::<BranchCreateButton, _>::new(0, cx, |state, _| {
                    let style = style.style_for(state);
                    Label::new("Create branch", style.label.clone())
                        .contained()
                        .with_style(style.container)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_down(MouseButton::Left, |_, _, cx| {
                    cx.spawn(|picker, mut cx| async move {
                        picker.update(&mut cx, |this, cx| {
                            let project = this.delegate().workspace.read(cx).project().read(cx);
                            let current_pick = &this.delegate().last_query;
                            let mut cwd = project
                            .visible_worktrees(cx)
                            .next()
                            .ok_or_else(|| anyhow!("There are no visisible worktrees."))?
                            .read(cx)
                            .abs_path()
                            .to_path_buf();
                            cwd.push(".git");
                            let repo = project
                                .fs()
                                .open_repo(&cwd)
                                .ok_or_else(|| anyhow!("Could not open repository at path `{}`", cwd.as_os_str().to_string_lossy()))?;
                            let repo = repo
                                .lock();
                            let status = repo
                                .create_branch(&current_pick);
                            if status.is_err() {
                                this.delegate().display_error_toast(format!("Failed to create branch '{current_pick}', check for conflicts or unstashed files"), cx);
                                status?;
                            }
                            let status = repo.change_branch(&current_pick);
                            if status.is_err() {
                                this.delegate().display_error_toast(format!("Failed to chec branch '{current_pick}', check for conflicts or unstashed files"), cx);
                                status?;
                            }
                            cx.emit(PickerEvent::Dismiss);
                            Ok::<(), anyhow::Error>(())
                })
                    }).detach();
                })).aligned().right()
                .into_any(),
            )
        } else {
            None
        }
    }
}
