use anyhow::{anyhow, bail};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{elements::*, AppContext, MouseState, Task, ViewContext, ViewHandle};
use picker::{Picker, PickerDelegate, PickerEvent};
use std::{ops::Not, sync::Arc};
use util::ResultExt;
use workspace::{Toast, Workspace};

pub fn init(cx: &mut AppContext) {
    Picker::<BranchListDelegate>::init(cx);
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
        },
        cx,
    )
    .with_theme(|theme| theme.picker.clone())
}

pub struct BranchListDelegate {
    matches: Vec<StringMatch>,
    workspace: ViewHandle<Workspace>,
    selected_index: usize,
    last_query: String,
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
                    let mut cwd = project
                        .visible_worktrees(cx)
                        .next()
                        .unwrap()
                        .read(cx)
                        .root_entry()
                        .unwrap()
                        .path
                        .to_path_buf();
                    cwd.push(".git");
                    let Some(repo) = project.fs().open_repo(&cwd) else {bail!("Project does not have associated git repository.")};
                    let mut branches = repo
                        .lock()
                        .branches()?;
                    if query.is_empty() {
                        const RECENT_BRANCHES_COUNT: usize = 10;
                        // Do a partial sort to show recent-ish branches first.
                        branches.select_nth_unstable_by(RECENT_BRANCHES_COUNT, |lhs, rhs| {
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

    fn confirm(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        let current_pick = self.selected_index();
        let current_pick = self.matches[current_pick].string.clone();
        cx.spawn(|picker, mut cx| async move {
            picker.update(&mut cx, |this, cx| {
                let project = this.delegate().workspace.read(cx).project().read(cx);
                let mut cwd = project
                .visible_worktrees(cx)
                .next()
                .ok_or_else(|| anyhow!("There are no visisible worktrees."))?
                .read(cx)
                .root_entry()
                .ok_or_else(|| anyhow!("Worktree has no root entry."))?
                .path
                .to_path_buf();
                cwd.push(".git");
                let status = project
                    .fs()
                    .open_repo(&cwd)
                    .ok_or_else(|| anyhow!("Could not open repository at path `{}`", cwd.as_os_str().to_string_lossy()))?
                    .lock()
                    .change_branch(&current_pick);
                if status.is_err() {
                    const GIT_CHECKOUT_FAILURE_ID: usize = 2048;
                    this.delegate().workspace.update(cx, |model, ctx| {
                        model.show_toast(
                            Toast::new(
                                GIT_CHECKOUT_FAILURE_ID,
                                format!("Failed to checkout branch '{current_pick}', check for conflicts or unstashed files"),
                            ),
                            ctx,
                        )
                    });
                    status?;
                }
                cx.emit(PickerEvent::Dismiss);

                Ok::<(), anyhow::Error>(())
            }).log_err();
        }).detach();
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
        const DISPLAYED_MATCH_LEN: usize = 29;
        let theme = &theme::current(cx);
        let hit = &self.matches[ix];
        let shortened_branch_name = util::truncate_and_trailoff(&hit.string, DISPLAYED_MATCH_LEN);
        let highlights = hit
            .positions
            .iter()
            .copied()
            .filter(|index| index < &DISPLAYED_MATCH_LEN)
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
            .with_height(theme.contact_finder.row_height)
            .into_any()
    }
    fn render_header(&self, cx: &AppContext) -> Option<AnyElement<Picker<Self>>> {
        let theme = &theme::current(cx);
        let style = theme.picker.header.clone();
        if self.last_query.is_empty() {
            Some(
                Flex::row()
                    .with_child(Label::new("Recent branches", style))
                    .into_any(),
            )
        } else {
            Some(
                Stack::new()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("Branches", style.clone()).aligned().left()),
                    )
                    .with_children(self.matches.is_empty().not().then(|| {
                        let suffix = if self.matches.len() == 1 { "" } else { "es" };
                        Flex::row()
                            .align_children_center()
                            .with_child(Label::new(
                                format!("{} match{}", self.matches.len(), suffix),
                                style,
                            ))
                            .aligned()
                            .right()
                    }))
                    .into_any(),
            )
        }
    }
}
