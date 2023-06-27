use client::{ContactRequestStatus, User, UserStore};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext};
use picker::{Picker, PickerDelegate, PickerEvent};
use project::Project;
use std::sync::Arc;
use util::{ResultExt, TryFutureExt};

pub fn init(cx: &mut AppContext) {
    Picker::<BranchListDelegate>::init(cx);
}

pub type BranchList = Picker<BranchListDelegate>;

pub fn build_branch_list(
    project: ModelHandle<Project>,
    cx: &mut ViewContext<BranchList>,
) -> BranchList {
    Picker::new(
        BranchListDelegate {
            branches: vec!["Foo".into(), "bar/baz".into()],
            matches: vec![],
            project,
            selected_index: 0,
        },
        cx,
    )
    .with_theme(|theme| theme.picker.clone())
}

pub struct BranchListDelegate {
    branches: Vec<String>,
    matches: Vec<StringMatch>,
    project: ModelHandle<Project>,
    selected_index: usize,
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
            let candidates = picker
                .read_with(&mut cx, |view, cx| {
                    let delegate = view.delegate();
                    let project = delegate.project.read(&cx);
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
                    let branches = project.fs().open_repo(&cwd).unwrap().lock().branches();
                    branches
                        .unwrap()
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(ix, command)| StringMatchCandidate {
                            id: ix,
                            string: command.clone(),
                            char_bag: command.chars().collect(),
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap();
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
                    //delegate.branches = actions;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                })
                .log_err();
        })
    }

    fn confirm(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        log::error!("confirm {}", self.selected_index());
        let current_pick = self.selected_index();
        let current_pick = self.matches[current_pick].string.clone();
        log::error!("Hi? {current_pick}");
        let project = self.project.read(cx);
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
        log::error!("{current_pick}");
        project
            .fs()
            .open_repo(&cwd)
            .unwrap()
            .lock()
            .change_branch(&current_pick)
            .log_err();
        cx.emit(PickerEvent::Dismiss);
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
        let user = &self.matches[ix];
        let style = theme.picker.item.in_state(selected).style_for(mouse_state);
        Flex::row()
            .with_child(
                Label::new(user.string.clone(), style.label.clone())
                    .with_highlights(user.positions.clone())
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
}
