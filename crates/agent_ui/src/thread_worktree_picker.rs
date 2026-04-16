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

use crate::{CreateWorktree, NewWorktreeBranchTarget, SwitchWorktree};

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
                    Some(req) => match req.await {
                        Ok(Ok(worktrees)) => {
                            worktrees.into_iter().filter(|wt| !wt.is_bare).collect()
                        }
                        Ok(Err(err)) => {
                            log::warn!("ThreadWorktreePicker: git worktree list failed: {err}");
                            return anyhow::Ok(());
                        }
                        Err(_) => {
                            log::warn!("ThreadWorktreePicker: worktree request was cancelled");
                            return anyhow::Ok(());
                        }
                    },
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
            let picker_entity = picker.downgrade();
            subscriptions.push(cx.subscribe_in(
                repo,
                window,
                move |_this, repo, event: &RepositoryEvent, window, cx| {
                    if matches!(event, RepositoryEvent::GitWorktreeListChanged) {
                        let worktrees_request = repo.update(cx, |repo, _| repo.worktrees());
                        let picker = picker_entity.clone();
                        cx.spawn_in(window, async move |_, cx| {
                            let all_worktrees: Vec<_> = worktrees_request
                                .await??
                                .into_iter()
                                .filter(|wt| !wt.is_bare)
                                .collect();
                            picker.update_in(cx, |picker, window, cx| {
                                picker.delegate.all_worktrees = all_worktrees;
                                picker.refresh(window, cx);
                            })?;
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
        /// When Some, create from this branch name (e.g. "main"). When None, create from current branch.
        from_branch: Option<String>,
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
        let create_named_disabled_reason: Option<String> = if self.has_multiple_repositories {
            Some("Cannot create a named worktree in a project with multiple repositories".into())
        } else if has_named_worktree {
            Some("A worktree with this name already exists".into())
        } else {
            None
        };

        let show_default_branch_create = !self.has_multiple_repositories
            && self.default_branch_name.as_ref().is_some_and(|default| {
                self.current_branch_name
                    .as_ref()
                    .is_none_or(|current| current != default)
            });
        let default_branch_name = self.default_branch_name.clone();

        if query.is_empty() {
            let mut matches = self.build_fixed_entries();

            if !repo_worktrees.is_empty() {
                let main_worktree_path = repo_worktrees
                    .iter()
                    .find(|wt| wt.is_main)
                    .map(|wt| wt.path.clone());

                let mut sorted = repo_worktrees;
                let project_paths = &self.project_worktree_paths;

                sorted.sort_by(|a, b| {
                    let a_is_current = project_paths.contains(&a.path);
                    let b_is_current = project_paths.contains(&b.path);
                    b_is_current.cmp(&a_is_current).then_with(|| {
                        a.directory_name(main_worktree_path.as_deref())
                            .cmp(&b.directory_name(main_worktree_path.as_deref()))
                    })
                });

                matches.push(ThreadWorktreeEntry::Separator);
                for worktree in sorted {
                    matches.push(ThreadWorktreeEntry::Worktree {
                        worktree,
                        positions: Vec::new(),
                    });
                }
            }

            self.matches = matches;
            self.sync_selected_index(false);
            return Task::ready(());
        }

        // When the user is typing, fuzzy-match worktree names using display_name
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

        let task = cx.background_executor().spawn(async move {
            fuzzy::match_strings(
                &candidates,
                &query,
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

                    if !new_matches.is_empty() {
                        new_matches.push(ThreadWorktreeEntry::Separator);
                    }
                    new_matches.push(ThreadWorktreeEntry::CreateNamed {
                        name: normalized_query.clone(),
                        from_branch: None,
                        disabled_reason: create_named_disabled_reason.clone(),
                    });
                    if show_default_branch_create {
                        if let Some(ref default_branch) = default_branch_name {
                            new_matches.push(ThreadWorktreeEntry::CreateNamed {
                                name: normalized_query.clone(),
                                from_branch: Some(default_branch.clone()),
                                disabled_reason: create_named_disabled_reason.clone(),
                            });
                        }
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
                    Box::new(CreateWorktree {
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
                    Box::new(CreateWorktree {
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
                    let main_worktree_path = self
                        .all_worktrees
                        .iter()
                        .find(|wt| wt.is_main)
                        .map(|wt| wt.path.as_path());
                    window.dispatch_action(
                        Box::new(SwitchWorktree {
                            path: worktree.path.clone(),
                            display_name: worktree.directory_name(main_worktree_path),
                        }),
                        cx,
                    );
                }
            }

            ThreadWorktreeEntry::CreateNamed {
                name,
                from_branch,
                disabled_reason: None,
            } => {
                let branch_target = match from_branch {
                    Some(branch) => NewWorktreeBranchTarget::ExistingBranch {
                        name: branch.clone(),
                    },
                    None => NewWorktreeBranchTarget::CurrentBranch,
                };
                window.dispatch_action(
                    Box::new(CreateWorktree {
                        worktree_name: Some(name.clone()),
                        branch_target,
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

        let no_git_reason: SharedString = "Requires a Git repository in the project".into();

        let create_new_list_item = |id: SharedString,
                                    label: SharedString,
                                    disabled_tooltip: Option<SharedString>,
                                    selected: bool| {
            let is_disabled = disabled_tooltip.is_some();
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
                            Label::new(label).when(is_disabled, |this| this.color(Color::Disabled)),
                        ),
                )
                .when_some(disabled_tooltip, |this, reason| {
                    this.tooltip(Tooltip::text(reason))
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

                let disabled_tooltip = is_create_disabled.then(|| no_git_reason.clone());

                let item = create_new_list_item(
                    "create-from-current".to_string().into(),
                    label.into(),
                    disabled_tooltip,
                    selected,
                );

                Some(item.into_any_element())
            }

            ThreadWorktreeEntry::CreateFromDefaultBranch {
                default_branch_name,
            } => {
                let label = format!("Create new worktree based on {default_branch_name}");

                let disabled_tooltip = is_create_disabled.then(|| no_git_reason.clone());

                let item = create_new_list_item(
                    "create-from-main".to_string().into(),
                    label.into(),
                    disabled_tooltip,
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
                from_branch,
                disabled_reason,
            } => {
                let branch_label = from_branch
                    .as_deref()
                    .unwrap_or(self.current_branch_name.as_deref().unwrap_or("HEAD"));
                let label = format!("Create \"{name}\" based on {branch_label}");
                let element_id = match from_branch {
                    Some(branch) => format!("create-named-from-{branch}"),
                    None => "create-named-from-current".to_string(),
                };

                let item = create_new_list_item(
                    element_id.into(),
                    label.into(),
                    disabled_reason.clone().map(SharedString::from),
                    selected,
                );

                Some(item.into_any_element())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            release_channel::init("0.0.0".parse().unwrap(), cx);
            crate::agent_panel::init(cx);
        });
    }

    fn make_worktree(path: &str, branch: &str, is_main: bool) -> GitWorktree {
        GitWorktree {
            path: PathBuf::from(path),
            ref_name: Some(format!("refs/heads/{branch}").into()),
            sha: "abc1234".into(),
            is_main,
            is_bare: false,
        }
    }

    fn build_delegate(
        project: Entity<Project>,
        all_worktrees: Vec<GitWorktree>,
        project_worktree_paths: HashSet<PathBuf>,
        current_branch_name: Option<String>,
        default_branch_name: Option<String>,
        has_multiple_repositories: bool,
    ) -> ThreadWorktreePickerDelegate {
        ThreadWorktreePickerDelegate {
            matches: vec![ThreadWorktreeEntry::CreateFromCurrentBranch],
            all_worktrees,
            project_worktree_paths,
            selected_index: 0,
            project,
            current_branch_name,
            default_branch_name,
            has_multiple_repositories,
        }
    }

    fn entry_names(delegate: &ThreadWorktreePickerDelegate) -> Vec<String> {
        delegate
            .matches
            .iter()
            .map(|entry| match entry {
                ThreadWorktreeEntry::CreateFromCurrentBranch => {
                    "CreateFromCurrentBranch".to_string()
                }
                ThreadWorktreeEntry::CreateFromDefaultBranch {
                    default_branch_name,
                } => format!("CreateFromDefaultBranch({default_branch_name})"),
                ThreadWorktreeEntry::Separator => "---".to_string(),
                ThreadWorktreeEntry::Worktree { worktree, .. } => {
                    format!("Worktree({})", worktree.path.display())
                }
                ThreadWorktreeEntry::CreateNamed {
                    name,
                    from_branch,
                    disabled_reason,
                } => {
                    let branch = from_branch
                        .as_deref()
                        .map(|b| format!("from {b}"))
                        .unwrap_or_else(|| "from current".to_string());
                    if disabled_reason.is_some() {
                        format!("CreateNamed({name}, {branch}, disabled)")
                    } else {
                        format!("CreateNamed({name}, {branch})")
                    }
                }
            })
            .collect()
    }

    type PickerWindow = gpui::WindowHandle<Picker<ThreadWorktreePickerDelegate>>;

    async fn make_picker(
        cx: &mut TestAppContext,
        all_worktrees: Vec<GitWorktree>,
        project_worktree_paths: HashSet<PathBuf>,
        current_branch_name: Option<String>,
        default_branch_name: Option<String>,
        has_multiple_repositories: bool,
    ) -> PickerWindow {
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        cx.add_window(|window, cx| {
            let delegate = build_delegate(
                project,
                all_worktrees,
                project_worktree_paths,
                current_branch_name,
                default_branch_name,
                has_multiple_repositories,
            );
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .modal(false)
        })
    }

    #[gpui::test]
    async fn test_empty_query_entries(cx: &mut TestAppContext) {
        init_test(cx);

        // When on `main` with default branch also `main`, only CreateFromCurrentBranch
        // is shown as a fixed entry. Worktrees are listed with the current one first.
        let worktrees = vec![
            make_worktree("/repo", "main", true),
            make_worktree("/repo-feature", "feature", false),
            make_worktree("/repo-bugfix", "bugfix", false),
        ];
        let project_paths: HashSet<PathBuf> = [PathBuf::from("/repo")].into_iter().collect();

        let picker = make_picker(
            cx,
            worktrees,
            project_paths,
            Some("main".into()),
            Some("main".into()),
            false,
        )
        .await;

        picker
            .update(cx, |picker, window, cx| picker.refresh(window, cx))
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();

        assert_eq!(
            names,
            vec![
                "CreateFromCurrentBranch",
                "---",
                "Worktree(/repo)",
                "Worktree(/repo-bugfix)",
                "Worktree(/repo-feature)",
            ]
        );

        // When current branch differs from default, CreateFromDefaultBranch appears.
        picker
            .update(cx, |picker, _window, cx| {
                picker.delegate.current_branch_name = Some("feature".into());
                picker.delegate.default_branch_name = Some("main".into());
                cx.notify();
            })
            .unwrap();
        picker
            .update(cx, |picker, window, cx| picker.refresh(window, cx))
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();

        assert!(names.contains(&"CreateFromDefaultBranch(main)".to_string()));
    }

    #[gpui::test]
    async fn test_query_filtering_and_create_entries(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = make_picker(
            cx,
            vec![
                make_worktree("/repo", "main", true),
                make_worktree("/repo-feature", "feature", false),
                make_worktree("/repo-bugfix", "bugfix", false),
                make_worktree("/my-worktree", "experiment", false),
            ],
            HashSet::default(),
            Some("dev".into()),
            Some("main".into()),
            false,
        )
        .await;

        // Partial match filters to matching worktrees and offers to create
        // from both current branch and default branch.
        picker
            .update(cx, |picker, window, cx| {
                picker.set_query("feat", window, cx)
            })
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();
        assert!(names.contains(&"Worktree(/repo-feature)".to_string()));
        assert!(
            names.contains(&"CreateNamed(feat, from current)".to_string()),
            "should offer to create from current branch, got: {names:?}"
        );
        assert!(
            names.contains(&"CreateNamed(feat, from main)".to_string()),
            "should offer to create from default branch, got: {names:?}"
        );
        assert!(!names.contains(&"Worktree(/repo-bugfix)".to_string()));

        // Exact match: both create entries appear but are disabled.
        picker
            .update(cx, |picker, window, cx| {
                picker.set_query("repo-feature", window, cx)
            })
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();
        assert!(
            names.contains(&"CreateNamed(repo-feature, from current, disabled)".to_string()),
            "exact name match should show disabled create entries, got: {names:?}"
        );

        // Spaces are normalized to hyphens: "my worktree" matches "my-worktree".
        picker
            .update(cx, |picker, window, cx| {
                picker.set_query("my worktree", window, cx)
            })
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();
        assert!(
            names.contains(&"CreateNamed(my-worktree, from current, disabled)".to_string()),
            "spaces should normalize to hyphens and detect existing worktree, got: {names:?}"
        );
    }

    #[gpui::test]
    async fn test_multi_repo_hides_worktrees_and_disables_create_named(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = make_picker(
            cx,
            vec![
                make_worktree("/repo", "main", true),
                make_worktree("/repo-feature", "feature", false),
            ],
            HashSet::default(),
            Some("main".into()),
            Some("main".into()),
            true,
        )
        .await;

        picker
            .update(cx, |picker, window, cx| picker.refresh(window, cx))
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();
        assert_eq!(names, vec!["CreateFromCurrentBranch"]);

        picker
            .update(cx, |picker, window, cx| {
                picker.set_query("new-thing", window, cx)
            })
            .unwrap();
        cx.run_until_parked();

        let names = picker
            .read_with(cx, |picker, _| entry_names(&picker.delegate))
            .unwrap();
        assert!(
            names.contains(&"CreateNamed(new-thing, from current, disabled)".to_string()),
            "multi-repo should disable create named, got: {names:?}"
        );
    }
}
