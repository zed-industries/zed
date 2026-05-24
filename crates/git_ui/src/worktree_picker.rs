use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context as _;
use collections::HashSet;
use fuzzy::StringMatchCandidate;
use git::repository::Worktree as GitWorktree;
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, PromptLevel,
    Render, SharedString, Styled, Subscription, Task, TaskExt, WeakEntity, Window, actions, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::Project;
use project::git_store::RepositoryEvent;
use ui::{
    Button, Divider, HighlightedLabel, IconButton, KeyBinding, ListItem, ListItemSpacing, Tooltip,
    prelude::*,
};
use util::ResultExt as _;
use util::paths::PathExt;
use workspace::{
    ModalView, MultiWorkspace, Workspace, dock::DockPosition, notifications::DetachAndPromptErr,
};

use crate::git_panel::show_error_toast;
use zed_actions::{
    CreateWorktree, NewWorktreeBranchTarget, OpenWorktreeInNewWindow, SwitchWorktree,
};

actions!(
    worktree_picker,
    [
        /// Deletes the selected git worktree.
        DeleteWorktree,
        /// Force deletes the selected git worktree.
        ForceDeleteWorktree
    ]
);

pub struct WorktreePicker {
    picker: Entity<Picker<WorktreePickerDelegate>>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl WorktreePicker {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focused_dock = workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).focused_dock_position(window, cx));
        Self::new_inner(project, workspace, focused_dock, false, window, cx)
    }

    pub fn new_modal(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        focused_dock: Option<DockPosition>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new_inner(project, workspace, focused_dock, true, window, cx)
    }

    fn new_inner(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        focused_dock: Option<DockPosition>,
        show_footer: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project_ref = project.read(cx);
        let project_worktree_paths: HashSet<PathBuf> = project_ref
            .visible_worktrees(cx)
            .map(|wt| wt.read(cx).abs_path().to_path_buf())
            .collect();

        let has_multiple_repositories = project_ref.repositories(cx).len() > 1;
        let repository = project_ref.active_repository(cx);

        let current_branch_name = repository.as_ref().and_then(|repo| {
            repo.read(cx)
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string())
        });

        let all_worktrees_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.worktrees()));

        let default_branch_request = repository.clone().map(|repository| {
            repository.update(cx, |repository, _| repository.default_branch(false))
        });

        let initial_matches = vec![WorktreeEntry::CreateFromCurrentBranch];

        let delegate = WorktreePickerDelegate {
            matches: initial_matches,
            all_worktrees: Vec::new(),
            project_worktree_paths,
            selected_index: 0,
            project,
            workspace,
            focused_dock,
            current_branch_name,
            default_branch_name: None,
            has_multiple_repositories,
            focus_handle: cx.focus_handle(),
            show_footer,
            modifiers: Modifiers::default(),
            hovered_delete_index: None,
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .show_scrollbar(true)
                .modal(false)
                .max_height(Some(rems(20.).into()))
        });

        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle;
        });

        let mut subscriptions = Vec::new();

        {
            let picker_handle = picker.downgrade();
            cx.spawn_in(window, async move |_this, cx| {
                let all_worktrees: Vec<_> = match all_worktrees_request {
                    Some(req) => match req.await {
                        Ok(Ok(worktrees)) => {
                            worktrees.into_iter().filter(|wt| !wt.is_bare).collect()
                        }
                        Ok(Err(err)) => {
                            log::warn!("WorktreePicker: git worktree list failed: {err}");
                            return anyhow::Ok(());
                        }
                        Err(_) => {
                            log::warn!("WorktreePicker: worktree request was cancelled");
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

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.modifiers = ev.modifiers;
            cx.notify();
        });
    }
}

impl Focusable for WorktreePicker {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for WorktreePicker {}
impl EventEmitter<DismissEvent> for WorktreePicker {}

impl Render for WorktreePicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("WorktreePicker")
            .w(rems(34.))
            .elevation_3(cx)
            .child(self.picker.clone())
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|this, _: &DeleteWorktree, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    let ix = picker.delegate.selected_index;
                    picker.delegate.delete_worktree(ix, false, window, cx);
                });
            }))
            .on_action(cx.listener(|this, _: &ForceDeleteWorktree, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    let ix = picker.delegate.selected_index;
                    picker.delegate.delete_worktree(ix, true, window, cx);
                });
            }))
    }
}

#[derive(Clone)]
enum WorktreeEntry {
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
        from_branch: Option<String>,
        disabled_reason: Option<String>,
    },
}

struct WorktreePickerDelegate {
    matches: Vec<WorktreeEntry>,
    all_worktrees: Vec<GitWorktree>,
    project_worktree_paths: HashSet<PathBuf>,
    selected_index: usize,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focused_dock: Option<DockPosition>,
    current_branch_name: Option<String>,
    default_branch_name: Option<String>,
    has_multiple_repositories: bool,
    focus_handle: FocusHandle,
    show_footer: bool,
    modifiers: Modifiers,
    hovered_delete_index: Option<usize>,
}

fn remove_worktree_command(path: &Path, force: bool) -> String {
    if force {
        format!("worktree remove --force {}", path.display())
    } else {
        format!("worktree remove {}", path.display())
    }
}

struct WorktreeRemoveForceDeletePrompt {
    required_error_substrings: &'static [&'static str],
    message: fn(&str) -> String,
}

impl WorktreeRemoveForceDeletePrompt {
    fn matches(&self, normalized_error_message: &str) -> bool {
        self.required_error_substrings
            .iter()
            .all(|substring| normalized_error_message.contains(substring))
    }
}

const WORKTREE_REMOVE_FORCE_DELETE_PROMPTS: &[WorktreeRemoveForceDeletePrompt] =
    &[WorktreeRemoveForceDeletePrompt {
        required_error_substrings: &[
            "contains modified or untracked files",
            "use --force to delete it",
        ],
        message: dirty_worktree_force_delete_prompt,
    }];

fn dirty_worktree_force_delete_prompt(display_name: &str) -> String {
    format!("Worktree \"{display_name}\" contains modified or untracked files. Force delete it?")
}

fn force_delete_prompt_for_worktree_remove_error(
    error: &anyhow::Error,
    display_name: &str,
) -> Option<String> {
    let normalized_error_message = error.to_string().to_lowercase();
    WORKTREE_REMOVE_FORCE_DELETE_PROMPTS
        .iter()
        .find(|prompt| prompt.matches(&normalized_error_message))
        .map(|prompt| (prompt.message)(display_name))
}

struct DeleteWorktreeTooltip {
    picker: WeakEntity<Picker<WorktreePickerDelegate>>,
    focus_handle: FocusHandle,
    delete_index: usize,
    _subscription: Subscription,
}

impl DeleteWorktreeTooltip {
    fn new(
        picker: Entity<Picker<WorktreePickerDelegate>>,
        focus_handle: FocusHandle,
        delete_index: usize,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&picker, |_, _, cx| cx.notify());
        Self {
            picker: picker.downgrade(),
            focus_handle,
            delete_index,
            _subscription: subscription,
        }
    }
}

impl Render for DeleteWorktreeTooltip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let force_delete = self
            .picker
            .read_with(cx, |picker, _| {
                picker
                    .delegate
                    .is_force_delete_hovering_index(self.delete_index)
            })
            .unwrap_or(false);

        if force_delete {
            Tooltip::for_action_in(
                "Force Delete Worktree",
                &ForceDeleteWorktree,
                &self.focus_handle,
                cx,
            )
            .into_any_element()
        } else {
            Tooltip::with_meta_in(
                "Delete Worktree",
                Some(&DeleteWorktree),
                "Hold alt to force delete",
                &self.focus_handle,
                cx,
            )
            .into_any_element()
        }
    }
}

impl WorktreePickerDelegate {
    fn build_fixed_entries(&self) -> Vec<WorktreeEntry> {
        let mut entries = Vec::new();

        entries.push(WorktreeEntry::CreateFromCurrentBranch);

        if !self.has_multiple_repositories {
            if let Some(ref default_branch) = self.default_branch_name {
                let is_different = self
                    .current_branch_name
                    .as_ref()
                    .is_none_or(|current| current != default_branch);
                if is_different {
                    entries.push(WorktreeEntry::CreateFromDefaultBranch {
                        default_branch_name: default_branch.clone(),
                    });
                }
            }
        }

        entries
    }

    fn all_repo_worktrees(&self) -> &[GitWorktree] {
        &self.all_worktrees
    }

    fn creation_blocked_reason(&self, cx: &App) -> Option<SharedString> {
        let project = self.project.read(cx);
        if project.is_via_collab() {
            Some("Worktree creation is not supported in collaborative projects".into())
        } else if project.repositories(cx).is_empty() {
            Some("Requires a Git repository in the project".into())
        } else {
            None
        }
    }

    fn can_delete_worktree(&self, worktree: &GitWorktree) -> bool {
        !worktree.is_main && !self.project_worktree_paths.contains(&worktree.path)
    }

    fn is_force_delete_hovering_index(&self, index: usize) -> bool {
        self.modifiers.alt && self.hovered_delete_index == Some(index)
    }

    fn delete_worktree(
        &self,
        ix: usize,
        force: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(entry) = self.matches.get(ix) else {
            return;
        };
        let WorktreeEntry::Worktree { worktree, .. } = entry else {
            return;
        };
        if !self.can_delete_worktree(worktree) {
            return;
        }

        let repo = self.project.read(cx).active_repository(cx);
        let Some(repo) = repo else {
            return;
        };
        let path = worktree.path.clone();
        let display_name = worktree.directory_name(
            self.all_worktrees
                .iter()
                .find(|worktree| worktree.is_main)
                .map(|worktree| worktree.path.as_path()),
        );
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let initial_result = repo
                .update(cx, |repo, _| repo.remove_worktree(path.clone(), force))
                .await?;

            let (result, attempted_force) = match initial_result {
                Ok(()) => (Ok(()), force),
                Err(error) => {
                    log::error!("Failed to remove worktree: {}", error);

                    let force_delete_prompt = (!force)
                        .then(|| {
                            force_delete_prompt_for_worktree_remove_error(&error, &display_name)
                        })
                        .flatten();

                    if let Some(prompt_message) = force_delete_prompt {
                        let answer = cx.update(|window, cx| {
                            window.prompt(
                                PromptLevel::Warning,
                                &prompt_message,
                                None,
                                &["Force Delete", "Cancel"],
                                cx,
                            )
                        })?;

                        if answer.await != Ok(0) {
                            return Ok(());
                        }

                        let retry = repo
                            .update(cx, |repo, _| repo.remove_worktree(path.clone(), true))
                            .await?;

                        if let Err(error) = &retry {
                            log::error!("Failed to force remove worktree: {error}");
                        }

                        (retry, true)
                    } else {
                        (Err(error), force)
                    }
                }
            };

            if let Err(error) = result {
                if let Some(workspace) = workspace.upgrade() {
                    cx.update(|_window, cx| {
                        show_error_toast(
                            workspace,
                            remove_worktree_command(&path, attempted_force),
                            error,
                            cx,
                        )
                    })?;
                }

                return Ok(());
            }

            picker.update_in(cx, |picker, _window, cx| {
                picker.delegate.matches.retain(|e| {
                    !matches!(e, WorktreeEntry::Worktree { worktree, .. } if worktree.path == path)
                });
                picker.delegate.all_worktrees.retain(|w| w.path != path);
                if picker.delegate.matches.is_empty() {
                    picker.delegate.selected_index = 0;
                } else if picker.delegate.selected_index >= picker.delegate.matches.len() {
                    picker.delegate.selected_index = picker.delegate.matches.len() - 1;
                }
                picker.delegate.hovered_delete_index = None;
                cx.notify();
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn sync_selected_index(&mut self, has_query: bool) {
        if !has_query {
            return;
        }

        if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, WorktreeEntry::Worktree { .. }))
        {
            self.selected_index = index;
        } else if let Some(index) = self
            .matches
            .iter()
            .position(|entry| matches!(entry, WorktreeEntry::CreateNamed { .. }))
        {
            self.selected_index = index;
        } else {
            self.selected_index = 0;
        }
    }
}

impl PickerDelegate for WorktreePickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a worktree…".into()
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
        !matches!(self.matches.get(ix), Some(WorktreeEntry::Separator))
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

                matches.push(WorktreeEntry::Separator);
                for worktree in sorted {
                    matches.push(WorktreeEntry::Worktree {
                        worktree,
                        positions: Vec::new(),
                    });
                }
            }

            self.matches = matches;
            self.sync_selected_index(false);
            return Task::ready(());
        }

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
                    let mut new_matches: Vec<WorktreeEntry> = Vec::new();

                    for candidate in &fuzzy_matches {
                        new_matches.push(WorktreeEntry::Worktree {
                            worktree: repo_worktrees_clone[candidate.candidate_id].clone(),
                            positions: candidate.positions.clone(),
                        });
                    }

                    if !new_matches.is_empty() {
                        new_matches.push(WorktreeEntry::Separator);
                    }
                    new_matches.push(WorktreeEntry::CreateNamed {
                        name: normalized_query.clone(),
                        from_branch: None,
                        disabled_reason: create_named_disabled_reason.clone(),
                    });
                    if show_default_branch_create {
                        if let Some(ref default_branch) = default_branch_name {
                            new_matches.push(WorktreeEntry::CreateNamed {
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

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry) = self.matches.get(self.selected_index) else {
            return;
        };

        match entry {
            WorktreeEntry::Separator => return,
            WorktreeEntry::CreateFromCurrentBranch => {
                if self.creation_blocked_reason(cx).is_some() {
                    return;
                }
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        crate::worktree_service::handle_create_worktree(
                            workspace,
                            &CreateWorktree {
                                worktree_name: None,
                                branch_target: NewWorktreeBranchTarget::CurrentBranch,
                            },
                            window,
                            self.focused_dock,
                            cx,
                        );
                    });
                }
            }
            WorktreeEntry::CreateFromDefaultBranch {
                default_branch_name,
            } => {
                if self.creation_blocked_reason(cx).is_some() {
                    return;
                }
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        crate::worktree_service::handle_create_worktree(
                            workspace,
                            &CreateWorktree {
                                worktree_name: None,
                                branch_target: NewWorktreeBranchTarget::ExistingBranch {
                                    name: default_branch_name.clone(),
                                },
                            },
                            window,
                            self.focused_dock,
                            cx,
                        );
                    });
                }
            }
            WorktreeEntry::Worktree { worktree, .. } => {
                let is_current = self.project_worktree_paths.contains(&worktree.path);

                if !is_current {
                    if secondary {
                        window.dispatch_action(
                            Box::new(OpenWorktreeInNewWindow {
                                path: worktree.path.clone(),
                            }),
                            cx,
                        );
                    } else {
                        let main_worktree_path = self
                            .all_worktrees
                            .iter()
                            .find(|wt| wt.is_main)
                            .map(|wt| wt.path.as_path());
                        if let Some(workspace) = self.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                crate::worktree_service::handle_switch_worktree(
                                    workspace,
                                    &SwitchWorktree {
                                        path: worktree.path.clone(),
                                        display_name: worktree.directory_name(main_worktree_path),
                                    },
                                    window,
                                    self.focused_dock,
                                    cx,
                                );
                            });
                        }
                    }
                }
            }
            WorktreeEntry::CreateNamed {
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
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        crate::worktree_service::handle_create_worktree(
                            workspace,
                            &CreateWorktree {
                                worktree_name: Some(name.clone()),
                                branch_target,
                            },
                            window,
                            self.focused_dock,
                            cx,
                        );
                    });
                }
            }
            WorktreeEntry::CreateNamed {
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

        match entry {
            WorktreeEntry::Separator => Some(
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),
            WorktreeEntry::CreateFromCurrentBranch => {
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
                    self.creation_blocked_reason(cx),
                    selected,
                );

                Some(item.into_any_element())
            }
            WorktreeEntry::CreateFromDefaultBranch {
                default_branch_name,
            } => {
                let label = format!("Create new worktree based on {default_branch_name}");

                let item = create_new_list_item(
                    "create-from-main".to_string().into(),
                    label.into(),
                    self.creation_blocked_reason(cx),
                    selected,
                );

                Some(item.into_any_element())
            }
            WorktreeEntry::Worktree {
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
                let can_delete = self.can_delete_worktree(worktree);

                let entry_icon = if is_current {
                    IconName::Check
                } else {
                    IconName::GitWorktree
                };
                let picker = cx.entity();

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
                        .when(!is_current, |this| {
                            let open_in_new_window_button =
                                IconButton::new(("open-new-window", ix), IconName::ArrowUpRight)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Open in New Window"))
                                    .on_click(cx.listener(move |picker, _, window, cx| {
                                        let Some(entry) = picker.delegate.matches.get(ix) else {
                                            return;
                                        };
                                        if let WorktreeEntry::Worktree { worktree, .. } = entry {
                                            window.dispatch_action(
                                                Box::new(OpenWorktreeInNewWindow {
                                                    path: worktree.path.clone(),
                                                }),
                                                cx,
                                            );
                                            cx.emit(DismissEvent);
                                        }
                                    }));

                            let focus_handle_delete = self.focus_handle.clone();
                            let force_delete = self.is_force_delete_hovering_index(ix);
                            let delete_button = div()
                                .id(("delete-worktree-hover", ix))
                                .on_hover(cx.listener(move |picker, hovered: &bool, _, cx| {
                                    if *hovered {
                                        picker.delegate.hovered_delete_index = Some(ix);
                                    } else if picker.delegate.hovered_delete_index == Some(ix) {
                                        picker.delegate.hovered_delete_index = None;
                                    }
                                    cx.notify();
                                }))
                                .child(
                                    IconButton::new(("delete-worktree", ix), IconName::Trash)
                                        .icon_size(IconSize::Small)
                                        .when(force_delete, |this| this.icon_color(Color::Error))
                                        .tooltip(move |_, cx| {
                                            cx.new(|cx| {
                                                DeleteWorktreeTooltip::new(
                                                    picker.clone(),
                                                    focus_handle_delete.clone(),
                                                    ix,
                                                    cx,
                                                )
                                            })
                                            .into()
                                        })
                                        .on_click(cx.listener(move |picker, _, window, cx| {
                                            picker.delegate.delete_worktree(
                                                ix,
                                                picker.delegate.modifiers.alt,
                                                window,
                                                cx,
                                            );
                                        })),
                                );

                            this.end_slot(
                                h_flex()
                                    .gap_0p5()
                                    .child(open_in_new_window_button)
                                    .when(can_delete, |this| this.child(delete_button)),
                            )
                            .show_end_slot_on_hover()
                        })
                        .into_any_element(),
                )
            }
            WorktreeEntry::CreateNamed {
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

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        if !self.show_footer {
            return None;
        }

        let focus_handle = self.focus_handle.clone();
        let selected_entry = self.matches.get(self.selected_index);

        let is_creating = selected_entry.is_some_and(|e| {
            matches!(
                e,
                WorktreeEntry::CreateFromCurrentBranch
                    | WorktreeEntry::CreateFromDefaultBranch { .. }
                    | WorktreeEntry::CreateNamed { .. }
            )
        });

        let is_existing_worktree =
            selected_entry.is_some_and(|e| matches!(e, WorktreeEntry::Worktree { .. }));

        let can_delete = selected_entry.is_some_and(|e| {
            matches!(e, WorktreeEntry::Worktree { worktree, .. } if self.can_delete_worktree(worktree))
        });

        let is_current = selected_entry.is_some_and(|e| {
            matches!(e, WorktreeEntry::Worktree { worktree, .. } if self.project_worktree_paths.contains(&worktree.path))
        });

        let footer = h_flex()
            .w_full()
            .p_1p5()
            .gap_0p5()
            .justify_end()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant);

        if is_creating {
            Some(
                footer
                    .child(
                        Button::new("create-worktree", "Create")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                            }),
                    )
                    .into_any(),
            )
        } else if is_existing_worktree {
            Some(
                footer
                    .when(can_delete, |this| {
                        let focus_handle = focus_handle.clone();
                        this.child(
                            Button::new("delete-worktree", "Delete")
                                .key_binding(
                                    KeyBinding::for_action_in(&DeleteWorktree, &focus_handle, cx)
                                        .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(DeleteWorktree.boxed_clone(), cx)
                                }),
                        )
                    })
                    .when(!is_current, |this| {
                        let focus_handle = focus_handle.clone();
                        this.child(
                            Button::new("open-in-new-window", "Open in New Window")
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &menu::SecondaryConfirm,
                                        &focus_handle,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                                }),
                        )
                    })
                    .child(
                        Button::new("open-worktree", "Open")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12.))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                            }),
                    )
                    .into_any(),
            )
        } else {
            None
        }
    }
}

fn create_new_list_item(
    id: SharedString,
    label: SharedString,
    disabled_tooltip: Option<SharedString>,
    selected: bool,
) -> AnyElement {
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
                .child(Label::new(label).when(is_disabled, |this| this.color(Color::Disabled))),
        )
        .when_some(disabled_tooltip, |this, reason| {
            this.tooltip(Tooltip::text(reason))
        })
        .into_any_element()
}

pub async fn open_remote_worktree(
    connection_options: remote::RemoteConnectionOptions,
    paths: Vec<PathBuf>,
    app_state: Arc<workspace::AppState>,
    workspace: gpui::WeakEntity<Workspace>,
    cx: &mut gpui::AsyncWindowContext,
) -> anyhow::Result<()> {
    let connect_task = workspace.update_in(cx, |workspace, window, cx| {
        workspace.toggle_modal(window, cx, |window, cx| {
            remote_connection::RemoteConnectionModal::new(
                &connection_options,
                Vec::new(),
                window,
                cx,
            )
        });

        let prompt = workspace
            .active_modal::<remote_connection::RemoteConnectionModal>(cx)
            .expect("Modal just created")
            .read(cx)
            .prompt
            .clone();

        remote_connection::connect(
            remote::remote_client::ConnectionIdentifier::setup(),
            connection_options.clone(),
            prompt,
            window,
            cx,
        )
        .prompt_err("Failed to connect", window, cx, |_, _, _| None)
    })?;

    let session = connect_task.await;

    workspace
        .update_in(cx, |workspace, _window, cx| {
            if let Some(prompt) =
                workspace.active_modal::<remote_connection::RemoteConnectionModal>(cx)
            {
                prompt.update(cx, |prompt, cx| prompt.finished(cx))
            }
        })
        .ok();

    let Some(Some(session)) = session else {
        return Ok(());
    };

    let new_project = cx.update(|_, cx| {
        project::Project::remote(
            session,
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            true,
            cx,
        )
    })?;

    let workspace_position = cx
        .update(|_, cx| {
            workspace::remote_workspace_position_from_db(connection_options.clone(), &paths, cx)
        })?
        .await
        .context("fetching workspace position from db")?;

    let mut options =
        cx.update(|_, cx| (app_state.build_window_options)(workspace_position.display, cx))?;
    options.window_bounds = workspace_position.window_bounds;

    let new_window = cx.open_window(options, |window, cx| {
        let workspace = cx.new(|cx| {
            let mut workspace =
                Workspace::new(None, new_project.clone(), app_state.clone(), window, cx);
            workspace.centered_layout = workspace_position.centered_layout;
            workspace
        });
        cx.new(|cx| MultiWorkspace::new(workspace, window, cx))
    })?;

    workspace::open_remote_project_with_existing_connection(
        connection_options,
        new_project,
        paths,
        app_state,
        new_window,
        None,
        None,
        cx,
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{AppContext, TestAppContext, VisualTestContext};
    use project::project_settings::ProjectSettings;
    use project::{Project, WorktreeSettings};
    use serde_json::json;
    use settings::Settings as _;
    use settings::SettingsStore;
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            ProjectSettings::register(cx);
            WorktreeSettings::register(cx);
        });
    }

    async fn init_worktree_picker_test(
        cx: &mut TestAppContext,
    ) -> (
        Arc<FakeFs>,
        Entity<WorktreePicker>,
        Entity<project::git_store::Repository>,
        PathBuf,
        VisualTestContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    ".git": {},
                    "file.txt": "buffer_text",
                },
                "worktrees": {},
            }),
        )
        .await;
        fs.set_head_for_repo(
            path!("/root/project/.git").as_ref(),
            &[("file.txt", "buffer_text".to_string())],
            "deadbeef",
        );

        let project = Project::test(fs.clone(), [path!("/root/project").as_ref()], cx).await;
        cx.executor().run_until_parked();

        let repository = project.read_with(cx, |project, cx| {
            project.repositories(cx).values().next().unwrap().clone()
        });
        let worktree_path = PathBuf::from(path!("/root/worktrees/dirty-wt"));

        cx.update(|cx| {
            repository.update(cx, |repository, _| {
                repository.create_worktree(
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: "dirty-wt".to_string(),
                        base_sha: Some("deadbeef".to_string()),
                    },
                    worktree_path.clone(),
                )
            })
        })
        .await
        .unwrap()
        .unwrap();

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let worktree_picker = window_handle
            .update(cx, |_multi_workspace, window, cx| {
                cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
            })
            .unwrap();

        let cx = VisualTestContext::from_window(window_handle.into(), cx);
        cx.run_until_parked();

        (fs, worktree_picker, repository, worktree_path, cx)
    }

    fn worktree_index(
        worktree_picker: &Entity<WorktreePicker>,
        worktree_path: &Path,
        cx: &mut VisualTestContext,
    ) -> usize {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .iter()
                    .position(|entry| {
                        matches!(entry, WorktreeEntry::Worktree { worktree, .. } if worktree.path == *worktree_path)
                    })
                    .expect("worktree should appear in picker")
            })
        })
    }

    async fn repo_contains_worktree(
        repository: &Entity<project::git_store::Repository>,
        worktree_path: &Path,
        cx: &mut VisualTestContext,
    ) -> bool {
        let worktrees = repository
            .update(cx, |repository, _| repository.worktrees())
            .await
            .unwrap()
            .unwrap();
        worktrees
            .iter()
            .any(|worktree| worktree.path == *worktree_path)
    }

    #[gpui::test]
    async fn test_delete_dirty_worktree_prompts_for_force_delete(cx: &mut TestAppContext) {
        let (fs, worktree_picker, repository, worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        fs.with_git_state(path!("/root/project/.git").as_ref(), true, |state| {
            state
                .worktrees_requiring_force_delete
                .insert(worktree_path.clone());
        })
        .expect("failed to mark test worktree as requiring force delete");

        let index = worktree_index(&worktree_picker, &worktree_path, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.delete_worktree(index, false, window, cx);
            })
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());

        cx.simulate_prompt_answer("Force Delete");
        cx.run_until_parked();

        assert!(!cx.has_pending_prompt());
        assert!(
            !repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "worktree should be removed after confirming force delete"
        );
    }

    #[gpui::test]
    async fn test_force_delete_worktree_deletes_without_prompt(cx: &mut TestAppContext) {
        let (fs, worktree_picker, repository, worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        fs.with_git_state(path!("/root/project/.git").as_ref(), true, |state| {
            state
                .worktrees_requiring_force_delete
                .insert(worktree_path.clone());
        })
        .expect("failed to mark test worktree as requiring force delete");

        let index = worktree_index(&worktree_picker, &worktree_path, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.modifiers = Modifiers::alt();
                picker.delegate.delete_worktree(index, true, window, cx);
            })
        });
        cx.run_until_parked();

        assert!(!cx.has_pending_prompt());
        assert!(
            !repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "worktree should be removed by explicit force delete"
        );
    }
}
