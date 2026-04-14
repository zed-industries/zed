use std::rc::Rc;

use collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use fuzzy::StringMatchCandidate;
use git::repository::{Branch as GitBranch, Worktree as GitWorktree};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use project::git_store::RepositoryEvent;
use ui::{
    Divider, DocumentationAside, HighlightedLabel, Icon, IconName, Label, LabelCommon, ListItem,
    ListItemSpacing, prelude::*,
};
use util::ResultExt as _;

use crate::{NewWorktreeBranchTarget, StartThreadIn};

pub(crate) struct ThreadBranchPicker {
    picker: Entity<Picker<ThreadBranchPickerDelegate>>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ThreadBranchPicker {
    pub fn new(
        project: Entity<Project>,
        current_target: &StartThreadIn,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_worktree_paths: HashSet<PathBuf> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .collect();

        let has_multiple_repositories = project.read(cx).repositories(cx).len() > 1;
        let current_branch_name = project
            .read(cx)
            .active_repository(cx)
            .and_then(|repo| {
                repo.read(cx)
                    .branch
                    .as_ref()
                    .map(|branch| branch.name().to_string())
            })
            .unwrap_or_else(|| "HEAD".to_string());

        let repository = if has_multiple_repositories {
            None
        } else {
            project.read(cx).active_repository(cx)
        };

        let (all_branches, occupied_branches) = repository
            .as_ref()
            .map(|repo| {
                let snapshot = repo.read(cx);
                let branches = process_branches(&snapshot.branch_list);
                let occupied =
                    compute_occupied_branches(&snapshot.linked_worktrees, &project_worktree_paths);
                (branches, occupied)
            })
            .unwrap_or_default();

        let default_branch_request = repository
            .clone()
            .map(|repo| repo.update(cx, |repo, _| repo.default_branch(false)));

        let (worktree_name, branch_target) = match current_target {
            StartThreadIn::NewWorktree {
                worktree_name,
                branch_target,
            } => (worktree_name.clone(), branch_target.clone()),
            _ => (None, NewWorktreeBranchTarget::default()),
        };

        let delegate = ThreadBranchPickerDelegate {
            matches: vec![ThreadBranchEntry::CurrentBranch],
            all_branches,
            occupied_branches,
            selected_index: 0,
            worktree_name,
            branch_target,
            project_worktree_paths,
            current_branch_name,
            default_branch_name: None,
            has_multiple_repositories,
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .modal(false)
                .max_height(Some(rems(20.).into()))
        });

        let focus_handle = picker.focus_handle(cx);

        let mut subscriptions = Vec::new();

        if let Some(repo) = &repository {
            subscriptions.push(cx.subscribe_in(
                repo,
                window,
                |this, repo, event: &RepositoryEvent, window, cx| match event {
                    RepositoryEvent::BranchListChanged => {
                        let all_branches = process_branches(&repo.read(cx).branch_list);
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.all_branches = all_branches;
                            picker.refresh(window, cx);
                        });
                    }
                    RepositoryEvent::GitWorktreeListChanged => {
                        let project_worktree_paths =
                            this.picker.read(cx).delegate.project_worktree_paths.clone();
                        let occupied = compute_occupied_branches(
                            &repo.read(cx).linked_worktrees,
                            &project_worktree_paths,
                        );
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.occupied_branches = occupied;
                            picker.refresh(window, cx);
                        });
                    }
                    _ => {}
                },
            ));
        }

        // Fetch default branch asynchronously since it requires a git operation
        if let Some(default_branch_request) = default_branch_request {
            let picker_handle = picker.downgrade();
            cx.spawn_in(window, async move |_this, cx| {
                let default_branch = default_branch_request
                    .await
                    .ok()
                    .and_then(Result::ok)
                    .flatten();

                picker_handle.update_in(cx, |picker, window, cx| {
                    picker.delegate.default_branch_name =
                        default_branch.map(|branch| branch.to_string());
                    picker.refresh(window, cx);
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        subscriptions.push(cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));

        Self {
            picker,
            focus_handle,
            _subscriptions: subscriptions,
        }
    }
}

impl Focusable for ThreadBranchPicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ThreadBranchPicker {}

impl Render for ThreadBranchPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(22.))
            .elevation_3(cx)
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
    }
}

#[derive(Clone)]
enum ThreadBranchEntry {
    CurrentBranch,
    DefaultBranch,
    Separator,
    ExistingBranch {
        branch: GitBranch,
        positions: Vec<usize>,
    },
    CreateNamed {
        name: String,
    },
}

pub(crate) struct ThreadBranchPickerDelegate {
    matches: Vec<ThreadBranchEntry>,
    all_branches: Vec<GitBranch>,
    occupied_branches: HashMap<String, String>,
    selected_index: usize,
    worktree_name: Option<String>,
    branch_target: NewWorktreeBranchTarget,
    project_worktree_paths: HashSet<PathBuf>,
    current_branch_name: String,
    default_branch_name: Option<String>,
    has_multiple_repositories: bool,
}

fn process_branches(branches: &Arc<[GitBranch]>) -> Vec<GitBranch> {
    let remote_upstreams: HashSet<_> = branches
        .iter()
        .filter_map(|branch| {
            branch
                .upstream
                .as_ref()
                .filter(|upstream| upstream.is_remote())
                .map(|upstream| upstream.ref_name.clone())
        })
        .collect();

    let mut result: Vec<GitBranch> = branches
        .iter()
        .filter(|branch| !remote_upstreams.contains(&branch.ref_name))
        .cloned()
        .collect();

    result.sort_by_key(|branch| {
        (
            branch.is_remote(),
            !branch.is_head,
            branch
                .most_recent_commit
                .as_ref()
                .map(|commit| 0 - commit.commit_timestamp),
        )
    });

    result
}

fn compute_occupied_branches(
    worktrees: &[GitWorktree],
    project_worktree_paths: &HashSet<PathBuf>,
) -> HashMap<String, String> {
    let mut occupied_branches = HashMap::default();
    for worktree in worktrees {
        let Some(branch_name) = worktree.branch_name().map(ToOwned::to_owned) else {
            continue;
        };

        let reason = if project_worktree_paths.contains(&worktree.path) {
            format!(
                "This branch is already checked out in the current project worktree at {}.",
                worktree.path.display()
            )
        } else {
            format!(
                "This branch is already checked out in a linked worktree at {}.",
                worktree.path.display()
            )
        };

        occupied_branches.insert(branch_name, reason);
    }
    occupied_branches
}

impl ThreadBranchPickerDelegate {
    fn new_worktree_action(&self, branch_target: NewWorktreeBranchTarget) -> StartThreadIn {
        StartThreadIn::NewWorktree {
            worktree_name: self.worktree_name.clone(),
            branch_target,
        }
    }

    fn selected_entry_name(&self) -> Option<&str> {
        match &self.branch_target {
            NewWorktreeBranchTarget::CurrentBranch => None,
            NewWorktreeBranchTarget::ExistingBranch { name } => Some(name),
            NewWorktreeBranchTarget::CreateBranch {
                from_ref: Some(from_ref),
                ..
            } => Some(from_ref),
            NewWorktreeBranchTarget::CreateBranch { name, .. } => Some(name),
        }
    }

    fn prefer_create_entry(&self) -> bool {
        matches!(
            &self.branch_target,
            NewWorktreeBranchTarget::CreateBranch { from_ref: None, .. }
        )
    }

    fn fixed_matches(&self) -> Vec<ThreadBranchEntry> {
        let mut matches = vec![ThreadBranchEntry::CurrentBranch];
        if !self.has_multiple_repositories
            && self
                .default_branch_name
                .as_ref()
                .is_some_and(|default_branch_name| default_branch_name != &self.current_branch_name)
        {
            matches.push(ThreadBranchEntry::DefaultBranch);
        }
        matches
    }

    fn is_branch_occupied(&self, branch_name: &str) -> bool {
        self.occupied_branches.contains_key(branch_name)
    }

    fn branch_aside_text(&self, branch_name: &str, is_remote: bool) -> Option<SharedString> {
        if self.is_branch_occupied(branch_name) {
            Some(
                "This branch is already checked out in another worktree. \
                 The new worktree will start in detached HEAD state."
                    .into(),
            )
        } else if is_remote {
            Some("A new local branch will be created from this remote branch.".into())
        } else {
            None
        }
    }

    fn entry_branch_name(&self, entry: &ThreadBranchEntry) -> Option<SharedString> {
        match entry {
            ThreadBranchEntry::CurrentBranch => {
                Some(SharedString::from(self.current_branch_name.clone()))
            }
            ThreadBranchEntry::DefaultBranch => {
                self.default_branch_name.clone().map(SharedString::from)
            }
            ThreadBranchEntry::ExistingBranch { branch, .. } => {
                Some(SharedString::from(branch.name().to_string()))
            }
            _ => None,
        }
    }

    fn entry_aside_text(&self, entry: &ThreadBranchEntry) -> Option<SharedString> {
        match entry {
            ThreadBranchEntry::CurrentBranch => Some(SharedString::from(
                "A new branch will be created from the current branch.",
            )),
            ThreadBranchEntry::DefaultBranch => {
                let default_branch_name = self
                    .default_branch_name
                    .as_ref()
                    .filter(|name| *name != &self.current_branch_name)?;
                self.branch_aside_text(default_branch_name, false)
            }
            ThreadBranchEntry::ExistingBranch { branch, .. } => {
                self.branch_aside_text(branch.name(), branch.is_remote())
            }
            _ => None,
        }
    }

    fn sync_selected_index(&mut self) {
        let selected_entry_name = self.selected_entry_name().map(ToOwned::to_owned);
        let prefer_create = self.prefer_create_entry();

        if prefer_create {
            if let Some(ref selected_entry_name) = selected_entry_name {
                if let Some(index) = self.matches.iter().position(|entry| {
                    matches!(
                        entry,
                        ThreadBranchEntry::CreateNamed { name } if name == selected_entry_name
                    )
                }) {
                    self.selected_index = index;
                    return;
                }
            }
        } else if let Some(ref selected_entry_name) = selected_entry_name {
            if selected_entry_name == &self.current_branch_name {
                if let Some(index) = self
                    .matches
                    .iter()
                    .position(|entry| matches!(entry, ThreadBranchEntry::CurrentBranch))
                {
                    self.selected_index = index;
                    return;
                }
            }

            if self
                .default_branch_name
                .as_ref()
                .is_some_and(|default_branch_name| default_branch_name == selected_entry_name)
            {
                if let Some(index) = self
                    .matches
                    .iter()
                    .position(|entry| matches!(entry, ThreadBranchEntry::DefaultBranch))
                {
                    self.selected_index = index;
                    return;
                }
            }

            if let Some(index) = self.matches.iter().position(|entry| {
                matches!(
                    entry,
                    ThreadBranchEntry::ExistingBranch { branch, .. }
                        if branch.name() == selected_entry_name.as_str()
                )
            }) {
                self.selected_index = index;
                return;
            }
        }

        if self.matches.len() > 1
            && self
                .matches
                .iter()
                .skip(1)
                .all(|entry| matches!(entry, ThreadBranchEntry::CreateNamed { .. }))
        {
            self.selected_index = 1;
            return;
        }

        self.selected_index = 0;
    }
}

impl PickerDelegate for ThreadBranchPickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search branches…".into()
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
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        !matches!(self.matches.get(ix), Some(ThreadBranchEntry::Separator))
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if self.has_multiple_repositories {
            let mut matches = self.fixed_matches();

            if query.is_empty() {
                if let Some(name) = self.selected_entry_name().map(ToOwned::to_owned) {
                    if self.prefer_create_entry() {
                        matches.push(ThreadBranchEntry::Separator);
                        matches.push(ThreadBranchEntry::CreateNamed { name });
                    }
                }
            } else {
                matches.push(ThreadBranchEntry::Separator);
                matches.push(ThreadBranchEntry::CreateNamed {
                    name: query.replace(' ', "-"),
                });
            }

            self.matches = matches;
            self.sync_selected_index();
            return Task::ready(());
        }

        let all_branches = self.all_branches.clone();

        if query.is_empty() {
            let mut matches = self.fixed_matches();
            let filtered_branches: Vec<_> = all_branches
                .into_iter()
                .filter(|branch| {
                    branch.name() != self.current_branch_name
                        && self
                            .default_branch_name
                            .as_ref()
                            .is_none_or(|default_branch_name| branch.name() != default_branch_name)
                })
                .collect();

            if !filtered_branches.is_empty() {
                matches.push(ThreadBranchEntry::Separator);
            }
            for branch in filtered_branches {
                matches.push(ThreadBranchEntry::ExistingBranch {
                    branch,
                    positions: Vec::new(),
                });
            }

            if let Some(selected_entry_name) = self.selected_entry_name().map(ToOwned::to_owned) {
                let has_existing = matches.iter().any(|entry| {
                    matches!(
                        entry,
                        ThreadBranchEntry::ExistingBranch { branch, .. }
                            if branch.name() == selected_entry_name
                    )
                });
                if self.prefer_create_entry() && !has_existing {
                    matches.push(ThreadBranchEntry::CreateNamed {
                        name: selected_entry_name,
                    });
                }
            }

            self.matches = matches;
            self.sync_selected_index();
            return Task::ready(());
        }

        let candidates: Vec<_> = all_branches
            .iter()
            .enumerate()
            .map(|(ix, branch)| StringMatchCandidate::new(ix, branch.name()))
            .collect();
        let executor = cx.background_executor().clone();
        let query_clone = query.clone();
        let normalized_query = query.replace(' ', "-");

        let task = cx.background_executor().spawn(async move {
            fuzzy::match_strings(
                &candidates,
                &query_clone,
                true,
                true,
                10000,
                &Default::default(),
                executor,
            )
            .await
        });

        let all_branches_clone = all_branches;
        cx.spawn_in(window, async move |picker, cx| {
            let fuzzy_matches = task.await;

            picker
                .update_in(cx, |picker, _window, cx| {
                    let mut matches = picker.delegate.fixed_matches();
                    let mut has_dynamic_entries = false;

                    for candidate in &fuzzy_matches {
                        let branch = all_branches_clone[candidate.candidate_id].clone();
                        if branch.name() == picker.delegate.current_branch_name
                            || picker.delegate.default_branch_name.as_ref().is_some_and(
                                |default_branch_name| branch.name() == default_branch_name,
                            )
                        {
                            continue;
                        }
                        if !has_dynamic_entries {
                            matches.push(ThreadBranchEntry::Separator);
                            has_dynamic_entries = true;
                        }
                        matches.push(ThreadBranchEntry::ExistingBranch {
                            branch,
                            positions: candidate.positions.clone(),
                        });
                    }

                    if fuzzy_matches.is_empty() {
                        if !has_dynamic_entries {
                            matches.push(ThreadBranchEntry::Separator);
                        }
                        matches.push(ThreadBranchEntry::CreateNamed {
                            name: normalized_query.clone(),
                        });
                    }

                    picker.delegate.matches = matches;
                    if let Some(index) =
                        picker.delegate.matches.iter().position(|entry| {
                            matches!(entry, ThreadBranchEntry::ExistingBranch { .. })
                        })
                    {
                        picker.delegate.selected_index = index;
                    } else if !fuzzy_matches.is_empty() {
                        picker.delegate.selected_index = 0;
                    } else if let Some(index) =
                        picker.delegate.matches.iter().position(|entry| {
                            matches!(entry, ThreadBranchEntry::CreateNamed { .. })
                        })
                    {
                        picker.delegate.selected_index = index;
                    } else {
                        picker.delegate.sync_selected_index();
                    }
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        match entry {
            ThreadBranchEntry::Separator => return,
            ThreadBranchEntry::CurrentBranch => {
                window.dispatch_action(
                    Box::new(self.new_worktree_action(NewWorktreeBranchTarget::CurrentBranch)),
                    cx,
                );
            }
            ThreadBranchEntry::DefaultBranch => {
                let Some(default_branch_name) = self.default_branch_name.clone() else {
                    return;
                };
                window.dispatch_action(
                    Box::new(
                        self.new_worktree_action(NewWorktreeBranchTarget::ExistingBranch {
                            name: default_branch_name,
                        }),
                    ),
                    cx,
                );
            }
            ThreadBranchEntry::ExistingBranch { branch, .. } => {
                let branch_target = if branch.is_remote() {
                    let branch_name = branch
                        .ref_name
                        .as_ref()
                        .strip_prefix("refs/remotes/")
                        .and_then(|stripped| stripped.split_once('/').map(|(_, name)| name))
                        .unwrap_or(branch.name())
                        .to_string();
                    NewWorktreeBranchTarget::CreateBranch {
                        name: branch_name,
                        from_ref: Some(branch.name().to_string()),
                    }
                } else {
                    NewWorktreeBranchTarget::ExistingBranch {
                        name: branch.name().to_string(),
                    }
                };
                window.dispatch_action(Box::new(self.new_worktree_action(branch_target)), cx);
            }
            ThreadBranchEntry::CreateNamed { name } => {
                window.dispatch_action(
                    Box::new(
                        self.new_worktree_action(NewWorktreeBranchTarget::CreateBranch {
                            name: name.clone(),
                            from_ref: None,
                        }),
                    ),
                    cx,
                );
            }
        }

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.matches.get(ix)?;

        match entry {
            ThreadBranchEntry::Separator => Some(
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),
            ThreadBranchEntry::CurrentBranch => {
                let branch_name = if self.has_multiple_repositories {
                    SharedString::from("current branches")
                } else {
                    SharedString::from(self.current_branch_name.clone())
                };

                Some(
                    ListItem::new("current-branch")
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(Label::new(branch_name))
                        .into_any_element(),
                )
            }
            ThreadBranchEntry::DefaultBranch => {
                let default_branch_name = self
                    .default_branch_name
                    .as_ref()
                    .filter(|name| *name != &self.current_branch_name)?;

                let is_occupied = self.is_branch_occupied(default_branch_name);

                let item = ListItem::new("default-branch")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(default_branch_name.clone()));

                Some(
                    if is_occupied {
                        item.start_slot(Icon::new(IconName::GitBranchPlus).color(Color::Muted))
                    } else {
                        item
                    }
                    .into_any_element(),
                )
            }
            ThreadBranchEntry::ExistingBranch {
                branch, positions, ..
            } => {
                let branch_name = branch.name().to_string();
                let needs_new_branch = self.is_branch_occupied(&branch_name) || branch.is_remote();

                Some(
                    ListItem::new(SharedString::from(format!("branch-{ix}")))
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_1()
                                .child(
                                    HighlightedLabel::new(branch_name, positions.clone())
                                        .truncate(),
                                )
                                .when(needs_new_branch, |item| {
                                    item.child(
                                        Icon::new(IconName::GitBranchPlus)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    )
                                }),
                        )
                        .into_any_element(),
                )
            }
            ThreadBranchEntry::CreateNamed { name } => Some(
                ListItem::new("create-named-branch")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(format!("Create Branch: \"{name}\"…")))
                    .into_any_element(),
            ),
        }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<DocumentationAside> {
        let entry = self.matches.get(self.selected_index)?;
        let branch_name = self.entry_branch_name(entry);
        let aside_text = self.entry_aside_text(entry);

        if branch_name.is_none() && aside_text.is_none() {
            return None;
        }

        let side = crate::ui::documentation_aside_side(cx);

        Some(DocumentationAside::new(
            side,
            Rc::new(move |cx| {
                v_flex()
                    .gap_1()
                    .when_some(branch_name.clone(), |this, name| {
                        this.child(Label::new(name))
                    })
                    .when_some(aside_text.clone(), |this, text| {
                        this.child(
                            div()
                                .when(branch_name.is_some(), |this| {
                                    this.pt_1()
                                        .border_t_1()
                                        .border_color(cx.theme().colors().border_variant)
                                })
                                .child(Label::new(text).color(Color::Muted)),
                        )
                    })
                    .into_any_element()
            }),
        ))
    }

    fn documentation_aside_index(&self) -> Option<usize> {
        let entry = self.matches.get(self.selected_index)?;
        if self.entry_branch_name(entry).is_some() || self.entry_aside_text(entry).is_some() {
            Some(self.selected_index)
        } else {
            None
        }
    }
}
