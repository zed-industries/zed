use std::collections::{HashMap, HashSet};

use collections::HashSet as CollectionsHashSet;
use std::path::PathBuf;
use std::sync::Arc;

use fuzzy::StringMatchCandidate;
use git::repository::Branch as GitBranch;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use ui::{
    HighlightedLabel, Icon, IconName, Label, LabelCommon, ListItem, ListItemSpacing, Tooltip,
    prelude::*,
};
use util::ResultExt as _;

use crate::{NewWorktreeBranchTarget, StartThreadIn};

pub(crate) struct ThreadBranchPicker {
    picker: Entity<Picker<ThreadBranchPickerDelegate>>,
    focus_handle: FocusHandle,
    _subscription: gpui::Subscription,
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
        let branches_request = repository
            .clone()
            .map(|repo| repo.update(cx, |repo, _| repo.branches()));
        let default_branch_request = repository
            .clone()
            .map(|repo| repo.update(cx, |repo, _| repo.default_branch(false)));
        let worktrees_request = repository.map(|repo| repo.update(cx, |repo, _| repo.worktrees()));

        let (worktree_name, branch_target) = match current_target {
            StartThreadIn::NewWorktree {
                worktree_name,
                branch_target,
            } => (worktree_name.clone(), branch_target.clone()),
            _ => (None, NewWorktreeBranchTarget::default()),
        };

        let delegate = ThreadBranchPickerDelegate {
            matches: vec![ThreadBranchEntry::CurrentBranch],
            all_branches: None,
            occupied_branches: None,
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

        if let (Some(branches_request), Some(default_branch_request), Some(worktrees_request)) =
            (branches_request, default_branch_request, worktrees_request)
        {
            let picker_handle = picker.downgrade();
            cx.spawn_in(window, async move |_this, cx| {
                let branches = branches_request.await??;
                let default_branch = default_branch_request.await.ok().and_then(Result::ok).flatten();
                let worktrees = worktrees_request.await??;

                let remote_upstreams: CollectionsHashSet<_> = branches
                    .iter()
                    .filter_map(|branch| {
                        branch
                            .upstream
                            .as_ref()
                            .filter(|upstream| upstream.is_remote())
                            .map(|upstream| upstream.ref_name.clone())
                    })
                    .collect();

                let mut occupied_branches = HashMap::new();
                for worktree in worktrees {
                    let Some(branch_name) = worktree.branch_name().map(ToOwned::to_owned) else {
                        continue;
                    };

                    let reason = if picker_handle
                        .read_with(cx, |picker, _| {
                            picker
                                .delegate
                                .project_worktree_paths
                                .contains(&worktree.path)
                        })
                        .unwrap_or(false)
                    {
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

                let mut all_branches: Vec<_> = branches
                    .into_iter()
                    .filter(|branch| !remote_upstreams.contains(&branch.ref_name))
                    .collect();
                all_branches.sort_by_key(|branch| {
                    (
                        branch.is_remote(),
                        !branch.is_head,
                        branch
                            .most_recent_commit
                            .as_ref()
                            .map(|commit| 0 - commit.commit_timestamp),
                    )
                });

                picker_handle.update_in(cx, |picker, window, cx| {
                    picker.delegate.all_branches = Some(all_branches);
                    picker.delegate.occupied_branches = Some(occupied_branches);
                    picker.delegate.default_branch_name = default_branch.map(|branch| branch.to_string());
                    picker.refresh(window, cx);
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        let subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            focus_handle,
            _subscription: subscription,
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
    ExistingBranch {
        branch: GitBranch,
        positions: Vec<usize>,
        occupied_reason: Option<String>,
    },
    CreateNamed {
        name: String,
    },
}

pub(crate) struct ThreadBranchPickerDelegate {
    matches: Vec<ThreadBranchEntry>,
    all_branches: Option<Vec<GitBranch>>,
    occupied_branches: Option<HashMap<String, String>>,
    selected_index: usize,
    worktree_name: Option<String>,
    branch_target: NewWorktreeBranchTarget,
    project_worktree_paths: HashSet<PathBuf>,
    current_branch_name: String,
    default_branch_name: Option<String>,
    has_multiple_repositories: bool,
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

    fn current_branch_label(&self) -> SharedString {
        if self.has_multiple_repositories {
            SharedString::from("New branch from: current branches")
        } else {
            SharedString::from(format!("New branch from: {}", self.current_branch_name))
        }
    }

    fn default_branch_label(&self) -> Option<SharedString> {
        let default_branch_name = self
            .default_branch_name
            .as_ref()
            .filter(|name| *name != &self.current_branch_name)?;
        let is_occupied = self
            .occupied_branches
            .as_ref()
            .is_some_and(|occupied| occupied.contains_key(default_branch_name));
        let prefix = if is_occupied {
            "New branch from"
        } else {
            "From"
        };
        Some(SharedString::from(format!(
            "{prefix}: {default_branch_name}"
        )))
    }

    fn branch_label_prefix(&self, branch_name: &str) -> &'static str {
        let is_occupied = self
            .occupied_branches
            .as_ref()
            .is_some_and(|occupied| occupied.contains_key(branch_name));
        if is_occupied {
            "New branch from: "
        } else {
            "From: "
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
    type ListItem = ListItem;

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
                        matches.push(ThreadBranchEntry::CreateNamed { name });
                    }
                }
            } else {
                matches.push(ThreadBranchEntry::CreateNamed {
                    name: query.replace(' ', "-"),
                });
            }

            self.matches = matches;
            self.sync_selected_index();
            return Task::ready(());
        }

        let Some(all_branches) = self.all_branches.clone() else {
            self.matches = self.fixed_matches();
            self.selected_index = 0;
            return Task::ready(());
        };
        let occupied_branches = self.occupied_branches.clone().unwrap_or_default();

        if query.is_empty() {
            let mut matches = self.fixed_matches();
            for branch in all_branches.into_iter().filter(|branch| {
                branch.name() != self.current_branch_name
                    && self
                        .default_branch_name
                        .as_ref()
                        .is_none_or(|default_branch_name| branch.name() != default_branch_name)
            }) {
                matches.push(ThreadBranchEntry::ExistingBranch {
                    occupied_reason: occupied_branches.get(branch.name()).cloned(),
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

                    for candidate in &fuzzy_matches {
                        let branch = all_branches_clone[candidate.candidate_id].clone();
                        if branch.name() == picker.delegate.current_branch_name
                            || picker.delegate.default_branch_name.as_ref().is_some_and(
                                |default_branch_name| branch.name() == default_branch_name,
                            )
                        {
                            continue;
                        }
                        let occupied_reason = occupied_branches.get(branch.name()).cloned();
                        matches.push(ThreadBranchEntry::ExistingBranch {
                            branch,
                            positions: candidate.positions.clone(),
                            occupied_reason,
                        });
                    }

                    if fuzzy_matches.is_empty() {
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

    fn separators_after_indices(&self) -> Vec<usize> {
        let fixed_count = self.fixed_matches().len();
        if self.matches.len() > fixed_count {
            vec![fixed_count - 1]
        } else {
            Vec::new()
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.matches.get(ix)?;

        match entry {
            ThreadBranchEntry::CurrentBranch => Some(
                ListItem::new("current-branch")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .start_slot(Icon::new(IconName::GitBranch).color(Color::Muted))
                    .child(Label::new(self.current_branch_label())),
            ),
            ThreadBranchEntry::DefaultBranch => Some(
                ListItem::new("default-branch")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .start_slot(Icon::new(IconName::GitBranch).color(Color::Muted))
                    .child(Label::new(self.default_branch_label()?)),
            ),
            ThreadBranchEntry::ExistingBranch {
                branch,
                positions,
                occupied_reason,
            } => {
                let prefix = self.branch_label_prefix(branch.name());
                let branch_name = branch.name().to_string();
                let full_label = format!("{prefix}{branch_name}");
                let adjusted_positions: Vec<usize> =
                    positions.iter().map(|&p| p + prefix.len()).collect();

                let item = ListItem::new(SharedString::from(format!("branch-{ix}")))
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .start_slot(Icon::new(IconName::GitBranch).color(Color::Muted))
                    .child(HighlightedLabel::new(full_label, adjusted_positions).truncate());

                Some(if let Some(reason) = occupied_reason.clone() {
                    item.tooltip(Tooltip::text(reason))
                } else if branch.is_remote() {
                    item.tooltip(Tooltip::text(
                        "Create a new local branch from this remote branch",
                    ))
                } else {
                    item
                })
            }
            ThreadBranchEntry::CreateNamed { name } => Some(
                ListItem::new("create-named-branch")
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .start_slot(Icon::new(IconName::Plus).color(Color::Accent))
                    .child(Label::new(format!("Create Branch: \"{name}\"…"))),
            ),
        }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        None
    }
}
