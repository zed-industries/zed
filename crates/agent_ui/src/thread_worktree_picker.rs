use std::path::PathBuf;
use std::sync::Arc;

use collections::HashSet;
use fuzzy::StringMatchCandidate;
use git::repository::Worktree as GitWorktree;
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    IntoElement, ParentElement, Render, SharedString, Styled, Subscription, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use project::git_store::RepositoryEvent;
use ui::{Divider, HighlightedLabel, ListItem, ListItemSpacing, Tooltip, prelude::*};
use util::ResultExt as _;
use util::paths::PathExt;

use crate::{CreateWorktreeImmediately, NewWorktreeBranchTarget, SwitchToLinkedWorktree};

pub(crate) struct ThreadWorktreePicker {
    picker: Entity<Picker<ThreadWorktreePickerDelegate>>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl ThreadWorktreePicker {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project_worktree_paths: HashSet<PathBuf> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|wt| wt.read(cx).abs_path().to_path_buf())
            .collect();

        let has_multiple_repositories = project.read(cx).repositories(cx).len() > 1;

        let current_branch_name = project.read(cx).active_repository(cx).and_then(|repo| {
            repo.read(cx)
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string())
        });

        let repository = if has_multiple_repositories {
            None
        } else {
            project.read(cx).active_repository(cx)
        };

        // Fetch worktrees from the git backend (includes main + all linked)
        let all_worktrees_request = repository
            .clone()
            .map(|repo| repo.update(cx, |repo, _| repo.worktrees()));

        let default_branch_request = repository
            .clone()
            .map(|repo| repo.update(cx, |repo, _| repo.default_branch(false)));

        // Start with just the fixed entries; worktree list populates async
        let initial_matches = vec![ThreadWorktreeEntry::CreateFromCurrentBranch];

        let delegate = ThreadWorktreePickerDelegate {
            matches: initial_matches,
            all_worktrees: Vec::new(),
            project_worktree_paths,
            selected_index: 0,
            project,
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

        let mut subscriptions = Vec::new();

        // Fetch worktrees and default branch asynchronously
        {
            let picker_handle = picker.downgrade();
            cx.spawn_in(window, async move |_this, cx| {
                let all_worktrees: Vec<_> = match all_worktrees_request {
                    Some(req) => req
                        .await
                        .ok()
                        .and_then(Result::ok)
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|wt| !wt.is_bare)
                        .collect(),
                    None => Vec::new(),
                };

                let default_branch = match default_branch_request {
                    Some(req) => req.await.ok().and_then(Result::ok).flatten(),
                    None => None,
                };

                picker_handle.update_in(cx, |picker, window, cx| {
                    picker.delegate.all_worktrees = all_worktrees;
                    picker.delegate.default_branch_name =
                        default_branch.map(|branch| branch.to_string());
                    picker.refresh(window, cx);
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        // Subscribe to repository events to live-update the worktree list
        if let Some(repo) = &repository {
            let picker_entity = picker.clone();
            subscriptions.push(cx.subscribe_in(
                repo,
                window,
                move |_this, repo, event: &RepositoryEvent, _window, cx| {
                    if matches!(event, RepositoryEvent::GitWorktreeListChanged) {
                        let worktrees_request = repo.update(cx, |repo, _| repo.worktrees());
                        let picker = picker_entity.clone();
                        cx.spawn(async move |_, cx| {
                            let all_worktrees: Vec<_> = worktrees_request
                                .await??
                                .into_iter()
                                .filter(|wt| !wt.is_bare)
                                .collect();
                            picker.update(cx, |picker, _cx| {
                                picker.delegate.all_worktrees = all_worktrees;
                            });
                            anyhow::Ok(())
                        })
                        .detach_and_log_err(cx);
                    }
                },
            ));
        }

        subscriptions.push(cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));

        Self {
            focus_handle: picker.focus_handle(cx),
            picker,
            _subscriptions: subscriptions,
        }
    }
}

impl Focusable for ThreadWorktreePicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ThreadWorktreePicker {}

impl Render for ThreadWorktreePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(rems(34.))
            .elevation_3(cx)
            .child(self.picker.clone())
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
    }
}

#[derive(Clone)]
enum ThreadWorktreeEntry {
    CreateFromCurrentBranch,
    CreateFromDefaultBranch {
        default_branch_name: String,
    },
    Separator,
    Worktree {
        worktree: GitWorktree,
        positions: Vec<usize>,
    },
    CreateNamed {
        name: String,
        disabled_reason: Option<String>,
    },
}

pub(crate) struct ThreadWorktreePickerDelegate {
    matches: Vec<ThreadWorktreeEntry>,
    all_worktrees: Vec<GitWorktree>,
    project_worktree_paths: HashSet<PathBuf>,
    selected_index: usize,
    project: Entity<Project>,
    current_branch_name: Option<String>,
    default_branch_name: Option<String>,
    has_multiple_repositories: bool,
}

impl ThreadWorktreePickerDelegate {
    fn build_fixed_entries(&self) -> Vec<ThreadWorktreeEntry> {
        let mut entries = Vec::new();

        entries.push(ThreadWorktreeEntry::CreateFromCurrentBranch);

        if !self.has_multiple_repositories {
            if let Some(ref default_branch) = self.default_branch_name {
                let is_different = self
                    .current_branch_name
                    .as_ref()
                    .is_none_or(|current| current != default_branch);
                if is_different {
                    entries.push(ThreadWorktreeEntry::CreateFromDefaultBranch {
                        default_branch_name: default_branch.clone(),
                    });
                }
            }
        }

        entries
    }

    fn all_repo_worktrees(&self) -> &[GitWorktree] {
        if self.has_multiple_repositories {
            &[]
        } else {
            &self.all_worktrees
        }
    }

    fn sync_selected_index(&mut self, has_query: bool) {
        if !has_query {
            return;
        }

        // When filtering, prefer selecting the first worktree match
        if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, ThreadWorktreeEntry::Worktree { .. }))
        {
            self.selected_index = index;
        } else if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, ThreadWorktreeEntry::CreateNamed { .. }))
        {
            self.selected_index = index;
        } else {
            self.selected_index = 0;
        }
    }
}

impl PickerDelegate for ThreadWorktreePickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a worktree for this thread…".into()
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
        !matches!(self.matches.get(ix), Some(ThreadWorktreeEntry::Separator))
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let repo_worktrees = self.all_repo_worktrees().to_vec();

        let normalized_query = query.replace(' ', "-");
        let main_worktree_path = self
            .all_worktrees
            .iter()
            .find(|wt| wt.is_main)
            .map(|wt| wt.path.clone());
        let has_named_worktree = self.all_worktrees.iter().any(|worktree| {
            worktree.directory_name(main_worktree_path.as_deref()) == normalized_query
        });
        let create_named_disabled_reason = if self.has_multiple_repositories {
            Some("Cannot create a named worktree in a project with multiple repositories".into())
        } else if has_named_worktree {
            Some("A worktree with this name already exists".into())
        } else {
            None
        };

        if query.is_empty() {
            let mut matches = self.build_fixed_entries();

            if !repo_worktrees.is_empty() {
                matches.push(ThreadWorktreeEntry::Separator);
                for worktree in &repo_worktrees {
                    matches.push(ThreadWorktreeEntry::Worktree {
                        worktree: worktree.clone(),
                        positions: Vec::new(),
                    });
                }
            }

            self.matches = matches;
            self.sync_selected_index(false);
            return Task::ready(());
        }

        // When the user is typing, fuzzy-match worktree names using display_name
        // For the main worktree, also match against "main"
        let main_worktree_path = repo_worktrees
            .iter()
            .find(|wt| wt.is_main)
            .map(|wt| wt.path.clone());
        let candidates: Vec<_> = repo_worktrees
            .iter()
            .enumerate()
            .map(|(ix, worktree)| {
                StringMatchCandidate::new(
                    ix,
                    &worktree.directory_name(main_worktree_path.as_deref()),
                )
            })
            .collect();

        let executor = cx.background_executor().clone();
        let query_clone = query.clone();

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

        let repo_worktrees_clone = repo_worktrees;
        cx.spawn_in(window, async move |picker, cx| {
            let fuzzy_matches = task.await;

            picker
                .update_in(cx, |picker, _window, cx| {
                    let mut new_matches: Vec<ThreadWorktreeEntry> = Vec::new();

                    for candidate in &fuzzy_matches {
                        new_matches.push(ThreadWorktreeEntry::Worktree {
                            worktree: repo_worktrees_clone[candidate.candidate_id].clone(),
                            positions: candidate.positions.clone(),
                        });
                    }

                    // If the typed text doesn't exactly match an existing worktree, offer to create one
                    let main_worktree_path = repo_worktrees_clone
                        .iter()
                        .find(|wt| wt.is_main)
                        .map(|wt| wt.path.clone());
                    let has_exact_match = repo_worktrees_clone.iter().any(|worktree| {
                        worktree.directory_name(main_worktree_path.as_deref()) == query
                    });

                    if !has_exact_match {
                        if !new_matches.is_empty() {
                            new_matches.push(ThreadWorktreeEntry::Separator);
                        }
                        new_matches.push(ThreadWorktreeEntry::CreateNamed {
                            name: normalized_query.clone(),
                            disabled_reason: create_named_disabled_reason.clone(),
                        });
                    }

                    picker.delegate.matches = new_matches;
                    picker.delegate.sync_selected_index(true);

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
            ThreadWorktreeEntry::Separator => return,

            ThreadWorktreeEntry::CreateFromCurrentBranch => {
                window.dispatch_action(
                    Box::new(CreateWorktreeImmediately {
                        worktree_name: None,
                        branch_target: NewWorktreeBranchTarget::CurrentBranch,
                    }),
                    cx,
                );
            }

            ThreadWorktreeEntry::CreateFromDefaultBranch {
                default_branch_name,
            } => {
                window.dispatch_action(
                    Box::new(CreateWorktreeImmediately {
                        worktree_name: None,
                        branch_target: NewWorktreeBranchTarget::ExistingBranch {
                            name: default_branch_name.clone(),
                        },
                    }),
                    cx,
                );
            }

            ThreadWorktreeEntry::Worktree { worktree, .. } => {
                let is_current = self.project_worktree_paths.contains(&worktree.path);

                if is_current {
                    // Already in this worktree — just dismiss
                } else {
                    window.dispatch_action(
                        Box::new(SwitchToLinkedWorktree {
                            path: worktree.path.clone(),
                        }),
                        cx,
                    );
                }
            }

            ThreadWorktreeEntry::CreateNamed {
                name,
                disabled_reason: None,
            } => {
                window.dispatch_action(
                    Box::new(CreateWorktreeImmediately {
                        worktree_name: Some(name.clone()),
                        branch_target: NewWorktreeBranchTarget::CurrentBranch,
                    }),
                    cx,
                );
            }

            ThreadWorktreeEntry::CreateNamed {
                disabled_reason: Some(_),
                ..
            } => {
                return;
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
        let project = self.project.read(cx);
        let is_create_disabled = project.repositories(cx).is_empty() || project.is_via_collab();

        let create_new_list_item =
            |id: SharedString, label: SharedString, is_disabled: bool, selected: bool| {
                ListItem::new(id)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_2p5()
                            .child(
                                Icon::new(IconName::Plus)
                                    .map(|this| {
                                        if is_disabled {
                                            this.color(Color::Disabled)
                                        } else {
                                            this.color(Color::Muted)
                                        }
                                    })
                                    .size(IconSize::Small),
                            )
                            .child(
                                Label::new(label)
                                    .when(is_disabled, |this| this.color(Color::Disabled)),
                            ),
                    )
                    .when(is_disabled, |this| {
                        this.tooltip(Tooltip::text("Requires a Git repository in the project"))
                    })
                    .into_any_element()
            };

        match entry {
            ThreadWorktreeEntry::Separator => Some(
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),

            ThreadWorktreeEntry::CreateFromCurrentBranch => {
                let branch_label = if self.has_multiple_repositories {
                    "current branches".to_string()
                } else {
                    self.current_branch_name
                        .clone()
                        .unwrap_or_else(|| "HEAD".to_string())
                };

                let label = format!("Create new worktree based on {branch_label}");

                let item = create_new_list_item(
                    "create-from-current".to_string().into(),
                    label.into(),
                    is_create_disabled,
                    selected,
                );

                Some(item.into_any_element())
            }

            ThreadWorktreeEntry::CreateFromDefaultBranch {
                default_branch_name,
            } => {
                let label = format!("Create new worktree based on {default_branch_name}");

                let item = create_new_list_item(
                    "create-from-main".to_string().into(),
                    label.into(),
                    is_create_disabled,
                    selected,
                );

                Some(item.into_any_element())
            }

            ThreadWorktreeEntry::Worktree {
                worktree,
                positions,
            } => {
                let main_worktree_path = self
                    .all_worktrees
                    .iter()
                    .find(|wt| wt.is_main)
                    .map(|wt| wt.path.as_path());
                let display_name = worktree.directory_name(main_worktree_path);
                let first_line = display_name.lines().next().unwrap_or(&display_name);
                let positions: Vec<_> = positions
                    .iter()
                    .copied()
                    .filter(|&pos| pos < first_line.len())
                    .collect();
                let path = worktree.path.compact().to_string_lossy().to_string();
                let sha = worktree.sha.chars().take(7).collect::<String>();

                let is_current = self.project_worktree_paths.contains(&worktree.path);

                let entry_icon = if is_current {
                    IconName::Check
                } else {
                    IconName::GitWorktree
                };

                Some(
                    ListItem::new(SharedString::from(format!("worktree-{ix}")))
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(
                            h_flex()
                                .w_full()
                                .gap_2p5()
                                .child(
                                    Icon::new(entry_icon)
                                        .color(if is_current {
                                            Color::Accent
                                        } else {
                                            Color::Muted
                                        })
                                        .size(IconSize::Small),
                                )
                                .child(
                                    v_flex()
                                        .w_full()
                                        .min_w_0()
                                        .child(
                                            HighlightedLabel::new(first_line.to_owned(), positions)
                                                .truncate(),
                                        )
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .min_w_0()
                                                .gap_1p5()
                                                .when_some(
                                                    worktree.branch_name().map(|b| b.to_string()),
                                                    |this, branch| {
                                                        this.child(
                                                            Label::new(branch)
                                                                .size(LabelSize::Small)
                                                                .color(Color::Muted),
                                                        )
                                                        .child(
                                                            Label::new("\u{2022}")
                                                                .alpha(0.5)
                                                                .color(Color::Muted)
                                                                .size(LabelSize::Small),
                                                        )
                                                    },
                                                )
                                                .when(!sha.is_empty(), |this| {
                                                    this.child(
                                                        Label::new(sha)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(
                                                        Label::new("\u{2022}")
                                                            .alpha(0.5)
                                                            .color(Color::Muted)
                                                            .size(LabelSize::Small),
                                                    )
                                                })
                                                .child(
                                                    Label::new(path)
                                                        .truncate_start()
                                                        .color(Color::Muted)
                                                        .size(LabelSize::Small)
                                                        .flex_1(),
                                                ),
                                        ),
                                ),
                        )
                        .into_any_element(),
                )
            }

            ThreadWorktreeEntry::CreateNamed {
                name,
                disabled_reason,
            } => {
                let is_disabled = disabled_reason.is_some();
                let label = format!("Create Worktree: \"{name}\"…");

                let item = create_new_list_item(
                    "create-fresh-new".to_string().into(),
                    label.into(),
                    is_disabled,
                    selected,
                );

                Some(item.into_any_element())
            }
        }
    }
}
