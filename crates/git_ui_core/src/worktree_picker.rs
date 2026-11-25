use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::{
    StreamExt as _,
    future::{self, Either},
    stream,
};
use fuzzy::StringMatchCandidate;
use git::repository::Worktree as GitWorktree;
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Context, DismissEvent, Entity, EntityId,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, Modifiers,
    ModifiersChangedEvent, ParentElement, PromptLevel, Render, SharedString, Styled, Subscription,
    Task, TaskExt, WeakEntity, Window, actions,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::{
    DeltaClone, Event as ProjectEvent, Project, delta_checkout_short_name,
    git_store::{GitStoreEvent, RepositoryEvent, RepositorySnapshot},
    repo_identity_path,
};
use ui::{
    Button, CommonAnimationExt as _, Divider, HighlightedLabel, IconButton, KeyBinding, ListItem,
    ListItemSpacing, ListSubHeader, Tooltip, prelude::*,
};
use util::{
    ResultExt as _,
    path_list::PathList,
    paths::{PathExt, PathStyle},
};
use workspace::{
    ModalView, MultiWorkspace, MultiWorkspaceEvent, OpenMode, RemovalIntent, Workspace,
    dock::DockPosition, notifications::DetachAndPromptErr,
};

use crate::notifications::show_error_toast;
use crate::worktree_labels::workspace_menu_worktree_labels;
use crate::worktree_service::{
    RemoteBranchName, WorktreeCreateTarget, classify_worktrees, switch_worktree_paths,
    worktree_create_targets,
};
use zed_actions::{
    CreateWorktree, NewWorktreeBranchTarget, OpenWorktreeInNewWindow, OpenWorktreeSetupTasks,
    SwitchWorktree,
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
    _load_group_checkouts: Option<Task<()>>,
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

        let active_worktree_paths: HashSet<PathBuf> = project_ref
            .visible_worktrees(cx)
            .map(|wt| wt.read(cx).abs_path().to_path_buf())
            .collect();

        let project_worktree_paths = active_worktree_paths.clone();

        let has_multiple_repositories = project_ref.repositories(cx).len() > 1;
        let path_style = project_ref.path_style(cx);
        let repository = project_ref.active_repository(cx);
        let repository_identity_path = repository.as_ref().map(|repository| {
            let repository = repository.read(cx);
            repo_identity_path(&repository.common_dir_abs_path, path_style).to_path_buf()
        });
        let worktree_paths = project_ref.worktree_paths(cx);
        let delta_clone_path = repository.as_ref().and_then(|repository| {
            let common_dir_abs_path = &repository.read(cx).common_dir_abs_path;
            worktree_paths
                .delta_clones()
                .values()
                .find(|clone| clone.path.as_path() == common_dir_abs_path.as_ref())
                .map(|clone| clone.path.clone())
        });
        let group_checkouts_request = repository
            .as_ref()
            .filter(|_| project_ref.is_local())
            .and_then(|repository| {
                let common_dir = repository.read(cx).common_dir_abs_path.clone();
                let is_delta_checkout = delta_clone_path.is_some();
                let source_common_dir = if is_delta_checkout {
                    project_ref.visible_worktrees(cx).find_map(|worktree| {
                        let snapshot = worktree.read(cx).snapshot();
                        if snapshot.root_repo_common_dir() == Some(&common_dir) {
                            snapshot.root_repo_source_common_dir().cloned()
                        } else {
                            None
                        }
                    })?
                } else {
                    common_dir.clone()
                };
                Some((
                    source_common_dir,
                    common_dir,
                    is_delta_checkout,
                    project_ref.fs().clone(),
                ))
            });

        let current_branch_name = repository.as_ref().and_then(|repo| {
            repo.read(cx)
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string())
        });

        let snapshot_worktrees = repository
            .as_ref()
            .and_then(|repository| snapshot_worktrees(repository.read(cx)));
        let has_snapshot_worktrees = snapshot_worktrees.is_some();
        let snapshot_default_branch = repository.as_ref().and_then(|repository| {
            repository
                .read(cx)
                .remote_default_branch
                .as_deref()
                .and_then(RemoteBranchName::parse)
        });

        let all_worktrees_request = repository
            .clone()
            .map(|repository| repository.update(cx, |repository, _| repository.worktrees()));

        let git_binary_path_request = group_checkouts_request
            .as_ref()
            .and(repository.as_ref())
            .map(|repository| repository.read(cx).git_binary_path(cx));

        let default_branch_request = repository.clone().map(|repository| {
            repository.update(cx, |repository, _| repository.default_branch(true))
        });

        let initial_matches = vec![WorktreeEntry::CreateFromCurrentBranch];

        let delegate = WorktreePickerDelegate {
            matches: initial_matches,
            all_worktrees: snapshot_worktrees.unwrap_or_default(),
            group_checkouts: Vec::new(),
            matches_query: String::new(),
            keep_selection_on_update: false,
            worktrees_loaded: false,
            observed_workspaces: Vec::new(),
            observed_git_stores: Vec::new(),
            _window_workspace_subscriptions: Vec::new(),
            repository_identity_path,
            delta_clone_path,
            project_worktree_paths,
            selected_index: 0,
            project,
            workspace,
            focused_dock,
            current_branch_name,
            default_branch: snapshot_default_branch,
            has_multiple_repositories,
            focus_handle: cx.focus_handle(),
            show_footer,
            modifiers: Modifiers::default(),
            active_worktree_paths,
            hovered_delete_index: None,
            deleting_worktree_paths: HashSet::default(),
            path_style,
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .show_scrollbar(true)
                .embedded()
        });

        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle;
        });
        if has_snapshot_worktrees {
            cx.defer_in(window, |worktree_picker, window, cx| {
                worktree_picker.picker.update(cx, |picker, cx| {
                    mark_worktrees_loaded(picker, window, cx);
                });
            });
        }

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
                            picker_handle.update_in(cx, mark_worktrees_loaded).ok();
                            return anyhow::Ok(());
                        }
                        Err(_) => {
                            log::warn!("WorktreePicker: worktree request was cancelled");
                            picker_handle.update_in(cx, mark_worktrees_loaded).ok();
                            return anyhow::Ok(());
                        }
                    },
                    None => Vec::new(),
                };

                let default_branch = match default_branch_request {
                    Some(req) => req.await.ok().and_then(Result::ok),
                    None => Some(None),
                };

                picker_handle.update_in(cx, |picker, window, cx| {
                    let default_branch = match default_branch {
                        Some(default_branch) => {
                            default_branch.as_deref().and_then(RemoteBranchName::parse)
                        }
                        None => picker.delegate.default_branch.clone(),
                    };
                    let is_changed = picker.delegate.default_branch != default_branch
                        || !same_worktrees(&picker.delegate.all_worktrees, &all_worktrees);
                    picker.delegate.all_worktrees = all_worktrees;
                    picker.delegate.default_branch = default_branch;
                    if !picker.delegate.worktrees_loaded {
                        mark_worktrees_loaded(picker, window, cx);
                    } else if is_changed {
                        refresh_keeping_selection(picker, window, cx);
                    }
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }

        if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() {
            subscriptions.push(cx.subscribe_in(
                &multi_workspace,
                window,
                |worktree_picker, _, _: &MultiWorkspaceEvent, window, cx| {
                    worktree_picker.picker.update(cx, |picker, cx| {
                        refresh_keeping_selection(picker, window, cx);
                    });
                },
            ));
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
                                picker.delegate.worktrees_loaded = true;
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

        let load_group_checkouts = group_checkouts_request.zip(git_binary_path_request).map(
            |((source_common_dir, listed_common_dir, include_source, fs), git_binary_path)| {
                let picker = picker.downgrade();
                cx.spawn_in(window, async move |_, cx| {
                    let git_binary_path = git_binary_path.await;
                    let source_checkout = include_source
                        .then(|| git_binary_path.clone())
                        .flatten()
                        .map(|git_binary_path| {
                            let source_common_dir = source_common_dir.clone();
                            let fs = fs.clone();
                            cx.background_spawn(async move {
                                load_source_checkout(
                                    &source_common_dir,
                                    &git_binary_path,
                                    fs.as_ref(),
                                )
                                .await
                            })
                        });
                    let mut group_checkouts = cx
                        .background_spawn(async move {
                            load_delta_checkouts(
                                &source_common_dir,
                                Some(&listed_common_dir),
                                git_binary_path,
                                fs,
                                path_style,
                            )
                            .await
                        })
                        .await;
                    let mut late_source_checkout = None;
                    if let Some(source_checkout) = source_checkout {
                        let source_checkout_wait =
                            cx.background_executor().timer(SOURCE_CHECKOUT_WAIT);
                        match future::select(source_checkout, source_checkout_wait).await {
                            Either::Left((source_checkout, _)) => {
                                group_checkouts.extend(source_checkout);
                            }
                            Either::Right((_, source_checkout)) => {
                                late_source_checkout = Some(source_checkout);
                            }
                        }
                    }
                    let Ok(()) = picker.update_in(cx, |picker, window, cx| {
                        apply_group_checkouts(picker, group_checkouts, window, cx);
                    }) else {
                        return;
                    };
                    if let Some(source_checkout) = late_source_checkout {
                        let source_checkout = Vec::from_iter(source_checkout.await);
                        picker
                            .update_in(cx, |picker, window, cx| {
                                apply_group_checkouts(picker, source_checkout, window, cx);
                            })
                            .ok();
                    }
                })
            },
        );

        Self {
            focus_handle: picker.focus_handle(cx),
            picker,
            _subscriptions: subscriptions,
            _load_group_checkouts: load_group_checkouts,
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
            .elevation_3(cx)
            .child(self.picker.clone())
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| {
                cx.emit(DismissEvent);
            }))
            .on_action(cx.listener(|_, _: &OpenWorktreeSetupTasks, _, cx| {
                cx.emit(DismissEvent);
                cx.propagate();
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
        default_branch: RemoteBranchName,
    },
    Separator,
    SectionHeader(SharedString),
    Worktree {
        worktree: GitWorktree,
        positions: Vec<usize>,
        branch_positions: Vec<usize>,
    },
    OpenWorkspace {
        open_workspace: OpenWorkspace,
        positions: Vec<usize>,
        branch_positions: Vec<usize>,
    },
    ClosedCheckout {
        checkout: GroupCheckout,
        positions: Vec<usize>,
        branch_positions: Vec<usize>,
    },
    CreateNamed {
        name: String,
        from_branch: Option<RemoteBranchName>,
        disabled_reason: Option<String>,
    },
}

impl WorktreeEntry {
    fn is_same_row(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CreateNamed { .. }, Self::CreateNamed { .. }) => true,
            (
                Self::Worktree { worktree, .. },
                Self::Worktree {
                    worktree: other_worktree,
                    ..
                },
            ) => worktree.path == other_worktree.path,
            (
                Self::OpenWorkspace { open_workspace, .. },
                Self::OpenWorkspace {
                    open_workspace: other_open_workspace,
                    ..
                },
            ) => open_workspace.workspace == other_open_workspace.workspace,
            (
                Self::ClosedCheckout { checkout, .. },
                Self::ClosedCheckout {
                    checkout: other_checkout,
                    ..
                },
            ) => checkout.path == other_checkout.path,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct OpenWorkspace {
    workspace: WeakEntity<Workspace>,
    name: String,
    branch: Option<String>,
    paths: String,
}

#[derive(Clone)]
struct GroupCheckout {
    path: PathBuf,
    name: String,
    branch: Option<String>,
    is_source: bool,
}

struct WorktreePickerDelegate {
    matches: Vec<WorktreeEntry>,
    matches_query: String,
    keep_selection_on_update: bool,
    all_worktrees: Vec<GitWorktree>,
    group_checkouts: Vec<GroupCheckout>,
    worktrees_loaded: bool,
    observed_workspaces: Vec<EntityId>,
    observed_git_stores: Vec<EntityId>,
    _window_workspace_subscriptions: Vec<Subscription>,
    repository_identity_path: Option<PathBuf>,
    delta_clone_path: Option<PathBuf>,
    project_worktree_paths: HashSet<PathBuf>,
    active_worktree_paths: HashSet<PathBuf>,
    selected_index: usize,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    focused_dock: Option<DockPosition>,
    current_branch_name: Option<String>,
    default_branch: Option<RemoteBranchName>,
    has_multiple_repositories: bool,
    focus_handle: FocusHandle,
    show_footer: bool,
    modifiers: Modifiers,
    hovered_delete_index: Option<usize>,
    deleting_worktree_paths: HashSet<PathBuf>,
    path_style: PathStyle,
}

fn worktree_name_anchor<'a>(
    worktrees: &'a [GitWorktree],
    repository_identity_path: Option<&'a Path>,
) -> Option<&'a Path> {
    worktrees
        .iter()
        .find(|worktree| worktree.is_main)
        .map(|worktree| worktree.path.as_path())
        .or(repository_identity_path)
}

fn mark_worktrees_loaded(
    picker: &mut Picker<WorktreePickerDelegate>,
    window: &mut Window,
    cx: &mut Context<Picker<WorktreePickerDelegate>>,
) {
    if picker.delegate.worktrees_loaded {
        return;
    }
    picker.delegate.worktrees_loaded = true;
    picker.delegate.refresh_project_worktree_paths(window, cx);
    picker.refresh(window, cx);
}

fn snapshot_worktrees(repository: &RepositorySnapshot) -> Option<Vec<GitWorktree>> {
    if repository.scan_id == 0 {
        return None;
    }
    let current_worktree = GitWorktree {
        path: repository.work_directory_abs_path.to_path_buf(),
        ref_name: repository
            .branch
            .as_ref()
            .map(|branch| branch.ref_name.clone()),
        sha: repository
            .head_commit
            .as_ref()
            .map(|head_commit| head_commit.sha.clone())
            .unwrap_or_default(),
        is_main: !repository.is_linked_worktree(),
        is_bare: false,
    };
    Some(
        std::iter::once(current_worktree)
            .chain(
                repository
                    .linked_worktrees()
                    .iter()
                    .filter(|worktree| !worktree.is_bare)
                    .cloned(),
            )
            .collect(),
    )
}

fn same_worktrees(shown: &[GitWorktree], refreshed: &[GitWorktree]) -> bool {
    shown.len() == refreshed.len() && refreshed.iter().all(|worktree| shown.contains(worktree))
}

fn apply_group_checkouts(
    picker: &mut Picker<WorktreePickerDelegate>,
    group_checkouts: Vec<GroupCheckout>,
    window: &mut Window,
    cx: &mut Context<Picker<WorktreePickerDelegate>>,
) {
    if group_checkouts.is_empty() {
        return;
    }
    let keeps_selection = picker.delegate.matches_query.is_empty()
        || picker.delegate.selected_index != picker.delegate.default_selected_index();
    picker.delegate.group_checkouts.extend(group_checkouts);
    picker
        .delegate
        .group_checkouts
        .sort_by_key(|checkout| !checkout.is_source);
    if keeps_selection {
        refresh_keeping_selection(picker, window, cx);
    } else {
        picker.refresh(window, cx);
    }
}

fn refresh_keeping_selection(
    picker: &mut Picker<WorktreePickerDelegate>,
    window: &mut Window,
    cx: &mut Context<Picker<WorktreePickerDelegate>>,
) {
    picker.delegate.refresh_project_worktree_paths(window, cx);
    picker.delegate.keep_selection_on_update = true;
    picker.refresh(window, cx);
}

fn worktree_row_name(
    worktree: &GitWorktree,
    name_anchor_path: Option<&Path>,
    delta_clone_path: Option<&Path>,
    path_style: PathStyle,
) -> String {
    delta_clone_path
        .and_then(|delta_clone_path| {
            delta_checkout_short_name(
                &DeltaClone {
                    path: delta_clone_path.to_path_buf(),
                    is_linked_worktree: !worktree.is_main,
                    checkout_path: Some(worktree.path.clone()),
                },
                &worktree.path,
                path_style,
            )
        })
        .map(String::from)
        .unwrap_or_else(|| worktree.directory_name(name_anchor_path, path_style))
}

async fn load_source_checkout(
    source_common_dir: &Path,
    git_binary_path: &Path,
    fs: &dyn Fs,
) -> Option<GroupCheckout> {
    let (path, branch) = source_checkout_path(source_common_dir, git_binary_path, fs).await?;
    Some(GroupCheckout {
        path,
        name: "main worktree".to_string(),
        branch,
        is_source: true,
    })
}

const MAX_CONCURRENT_BRANCH_LOOKUPS: usize = 16;
const SOURCE_CHECKOUT_WAIT: Duration = Duration::from_millis(500);

async fn load_delta_checkouts(
    source_common_dir: &Path,
    listed_common_dir: Option<&Path>,
    git_binary_path: Option<PathBuf>,
    fs: Arc<dyn Fs>,
    path_style: PathStyle,
) -> Vec<GroupCheckout> {
    let discovered_checkouts = project::git_store::discover_delta_checkouts(
        source_common_dir,
        listed_common_dir,
        fs.as_ref(),
    )
    .await;
    let mut delta_checkouts =
        stream::iter(discovered_checkouts.into_iter().filter_map(|checkout| {
            let checkout_path = checkout.clone.checkout_path.clone()?;
            let fs = fs.clone();
            let git_binary_path = git_binary_path.clone();
            Some(async move {
                let branch = checkout_branch_name(
                    &checkout_path,
                    &checkout.repository_dir,
                    git_binary_path.as_deref(),
                    fs.as_ref(),
                )
                .await;
                let name = if checkout.is_delta_clone {
                    delta_checkout_short_name(&checkout.clone, &checkout_path, path_style)
                        .map(String::from)
                        .unwrap_or_else(|| checkout_path.compact().to_string_lossy().into_owned())
                } else {
                    worktree_row_name(
                        &GitWorktree {
                            path: checkout_path.clone(),
                            ref_name: None,
                            sha: SharedString::default(),
                            is_main: false,
                            is_bare: false,
                        },
                        Some(repo_identity_path(source_common_dir, path_style)),
                        None,
                        path_style,
                    )
                };
                GroupCheckout {
                    path: checkout_path,
                    name,
                    branch,
                    is_source: false,
                }
            })
        }))
        .buffer_unordered(MAX_CONCURRENT_BRANCH_LOOKUPS)
        .collect::<Vec<_>>()
        .await;
    delta_checkouts.sort_by(|a, b| a.name.cmp(&b.name));
    delta_checkouts
}

async fn ensure_checkout_available(checkout_path: &Path, fs: &dyn Fs) -> anyhow::Result<()> {
    let is_available = fs
        .canonicalize(checkout_path)
        .await
        .is_ok_and(|canonical_path| canonical_path == checkout_path)
        && fs.is_dir(checkout_path).await;
    anyhow::ensure!(is_available, "{checkout_path:?} is no longer available");
    Ok(())
}

fn report_checkout_open_error(
    workspace: &WeakEntity<Workspace>,
    error: anyhow::Error,
    cx: &mut AsyncWindowContext,
) {
    workspace
        .update(cx, |_, cx| {
            show_error_toast(
                cx.entity(),
                "worktree switch",
                anyhow::anyhow!("{error:#}"),
                cx,
            );
        })
        .ok();
}

async fn source_checkout_path(
    source_common_dir: &Path,
    git_binary_path: &Path,
    fs: &dyn Fs,
) -> Option<(PathBuf, Option<String>)> {
    let identity_path = repo_identity_path(source_common_dir, PathStyle::local());
    if identity_path != source_common_dir
        && let Some(source_checkout) =
            verified_source_checkout(identity_path, source_common_dir, git_binary_path, fs).await
    {
        return Some(source_checkout);
    }
    let configured_worktree = configured_worktree(git_binary_path, source_common_dir, fs).await?;
    verified_source_checkout(&configured_worktree, source_common_dir, git_binary_path, fs).await
}

async fn configured_worktree(
    git_binary_path: &Path,
    git_dir: &Path,
    fs: &dyn Fs,
) -> Option<PathBuf> {
    let value = fs
        .git_command(
            git_binary_path,
            git_dir,
            [
                "--git-dir",
                ".",
                "config",
                "-z",
                "--default=",
                "--get",
                "core.worktree",
            ]
            .map(String::from)
            .to_vec(),
        )
        .await
        .log_with_level(log::Level::Debug)?;
    let value = value.trim_suffix('\0');
    (!value.is_empty()).then(|| git_dir.join(value))
}

async fn verified_source_checkout(
    candidate: &Path,
    source_common_dir: &Path,
    git_binary_path: &Path,
    fs: &dyn Fs,
) -> Option<(PathBuf, Option<String>)> {
    let candidate = fs.canonicalize(candidate).await.ok()?;
    let work_tree_output = git_output(
        git_binary_path,
        &candidate,
        &[
            "rev-parse",
            "--is-inside-work-tree",
            "--show-prefix",
            "--git-common-dir",
        ],
        fs,
    )
    .await?;
    let common_dir = work_tree_output.strip_prefix("true\n\n")?;
    let common_dir = fs
        .canonicalize(&candidate.join(last_output_field(common_dir)))
        .await
        .ok()?;
    if common_dir != source_common_dir {
        return None;
    }
    let git_dir = if candidate.join(".git") == source_common_dir {
        common_dir
    } else {
        let git_dir = git_output(
            git_binary_path,
            &candidate,
            &["rev-parse", "--absolute-git-dir"],
            fs,
        )
        .await?;
        fs.canonicalize(Path::new(last_output_field(&git_dir)))
            .await
            .ok()?
    };
    if git_dir != source_common_dir {
        return None;
    }
    let branch = checkout_branch_name(&candidate, &git_dir, Some(git_binary_path), fs).await;
    Some((candidate, branch))
}

async fn git_output(
    git_binary_path: &Path,
    work_directory: &Path,
    args: &[&str],
    fs: &dyn Fs,
) -> Option<String> {
    fs.git_command(
        git_binary_path,
        work_directory,
        args.iter().copied().map(String::from).collect(),
    )
    .await
    .log_with_level(log::Level::Debug)
}

fn last_output_field(output: &str) -> &str {
    output.trim_suffix('\n')
}

async fn current_branch_name(
    checkout_path: &Path,
    git_binary_path: &Path,
    fs: &dyn Fs,
) -> Option<String> {
    let branch = git_output(
        git_binary_path,
        checkout_path,
        &["symbolic-ref", "--quiet", "HEAD"],
        fs,
    )
    .await?;
    branch
        .trim_end_matches(['\n', '\r'])
        .strip_prefix("refs/heads/")
        .filter(|branch| !branch.is_empty())
        .map(str::to_owned)
}

async fn checkout_branch_name(
    checkout_path: &Path,
    git_dir: &Path,
    git_binary_path: Option<&Path>,
    fs: &dyn Fs,
) -> Option<String> {
    let head_path = git_dir.join("HEAD");
    if !fs.is_file(&head_path).await {
        return None;
    }
    let head = fs.load(&head_path).await.ok()?;
    match head.trim().strip_prefix("ref: refs/heads/") {
        Some(".invalid") => current_branch_name(checkout_path, git_binary_path?, fs).await,
        branch => branch.map(str::to_owned),
    }
}

fn single_repository_branch_name(workspace: &Entity<Workspace>, cx: &App) -> Option<String> {
    let (repositories, _) = classify_worktrees(workspace.read(cx).project().read(cx), cx);
    let [repository] = repositories.as_slice() else {
        return None;
    };
    repository
        .read(cx)
        .branch
        .as_ref()
        .map(|branch| branch.name().to_string())
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
                concat!("Hold ", ui::alt_key_name!(), " to force delete"),
                &self.focus_handle,
                cx,
            )
            .into_any_element()
        }
    }
}

impl WorktreePickerDelegate {
    fn worktree_name_anchor(&self) -> Option<&Path> {
        worktree_name_anchor(
            &self.all_worktrees,
            self.repository_identity_path.as_deref(),
        )
    }

    fn worktree_name(&self, worktree: &GitWorktree) -> String {
        worktree_row_name(
            worktree,
            self.worktree_name_anchor(),
            self.delta_clone_path.as_deref(),
            self.path_style,
        )
    }

    fn other_open_workspaces(&self, window: &Window, cx: &App) -> Vec<OpenWorkspace> {
        // `Picker::new` runs the first `update_matches`, possibly inside an update of the active workspace.
        // Before the Git list loads, a sibling that a Git row opens would also get its own row.
        if !self.worktrees_loaded {
            return Vec::new();
        }
        let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() else {
            return Vec::new();
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return Vec::new();
        };
        let group_key = workspace.read(cx).project_group_key(cx);
        let multi_workspace = multi_workspace.read(cx);
        let group_workspaces = multi_workspace
            .workspaces_for_project_group(&group_key, cx)
            .filter(|group_workspace| {
                *group_workspace != &workspace
                    && !group_workspace.read(cx).root_paths(cx).is_empty()
            })
            .collect::<Vec<_>>();
        if group_workspaces.is_empty() {
            return Vec::new();
        }
        let (_, non_git_paths) = classify_worktrees(self.project.read(cx), cx);
        let host = self.project.read(cx).remote_connection_options(cx).cloned();
        let switch_targets = self
            .all_worktrees
            .iter()
            .filter(|git_worktree| !self.active_worktree_paths.contains(&git_worktree.path))
            .filter_map(|git_worktree| {
                multi_workspace.workspace_for_paths(
                    &PathList::new(&switch_worktree_paths(&git_worktree.path, &non_git_paths)),
                    host.as_ref(),
                    cx,
                )
            })
            .collect::<Vec<_>>();
        let mut open_workspaces = group_workspaces
            .into_iter()
            .filter(|group_workspace| !switch_targets.contains(group_workspace))
            .map(|group_workspace| OpenWorkspace {
                workspace: group_workspace.downgrade(),
                name: workspace_menu_worktree_labels(group_workspace, cx)
                    .iter()
                    .map(|label| label.text())
                    .collect::<Vec<_>>()
                    .join(" • "),
                branch: single_repository_branch_name(group_workspace, cx),
                paths: group_workspace
                    .read(cx)
                    .root_paths(cx)
                    .iter()
                    .map(|root_path| root_path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join(", "),
            })
            .collect::<Vec<_>>();
        open_workspaces.sort_by(|a, b| a.name.cmp(&b.name));
        open_workspaces
    }

    fn closed_group_checkouts(&self) -> Vec<GroupCheckout> {
        if !self.worktrees_loaded {
            return Vec::new();
        }
        self.group_checkouts
            .iter()
            .filter(|checkout| {
                !self.project_worktree_paths.contains(&checkout.path)
                    && !self
                        .all_worktrees
                        .iter()
                        .any(|worktree| worktree.path == checkout.path)
            })
            .cloned()
            .collect()
    }

    fn observe_window_workspaces(
        &mut self,
        open_workspaces: &[OpenWorkspace],
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if !self.worktrees_loaded {
            return;
        }
        let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten() else {
            return;
        };
        // A workspace joins or leaves the group through `WorktreePathsChanged`, for which `MultiWorkspace` emits no event.
        let projects = multi_workspace
            .read(cx)
            .workspaces()
            .map(|workspace| workspace.read(cx).project().clone())
            .collect::<Vec<_>>();
        let git_stores = open_workspaces
            .iter()
            .filter_map(|open_workspace| open_workspace.workspace.upgrade())
            .map(|workspace| workspace.read(cx).project().read(cx).git_store().clone())
            .collect::<Vec<_>>();
        let project_ids = projects
            .iter()
            .map(|project| project.entity_id())
            .collect::<Vec<_>>();
        let git_store_ids = git_stores
            .iter()
            .map(|git_store| git_store.entity_id())
            .collect::<Vec<_>>();
        if project_ids == self.observed_workspaces && git_store_ids == self.observed_git_stores {
            return;
        }
        self.observed_workspaces = project_ids;
        self.observed_git_stores = git_store_ids;
        let project_subscriptions = projects.iter().map(|project| {
            cx.subscribe_in(
                project,
                window,
                |picker, _, event: &ProjectEvent, window, cx| {
                    if matches!(event, ProjectEvent::WorktreePathsChanged { .. }) {
                        refresh_keeping_selection(picker, window, cx);
                    }
                },
            )
        });
        let mut subscriptions = project_subscriptions.collect::<Vec<_>>();
        subscriptions.extend(git_stores.iter().map(|git_store| {
            cx.subscribe_in(
                git_store,
                window,
                |picker, _, event: &GitStoreEvent, window, cx| {
                    if matches!(
                        event,
                        GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::HeadChanged, _)
                            | GitStoreEvent::RepositoryAdded
                            | GitStoreEvent::RepositoryRemoved(_)
                    ) {
                        refresh_keeping_selection(picker, window, cx);
                    }
                },
            )
        }));
        self._window_workspace_subscriptions = subscriptions;
    }

    fn build_fixed_entries(&self) -> Vec<WorktreeEntry> {
        worktree_create_targets(
            self.has_multiple_repositories,
            self.default_branch.clone(),
            self.current_branch_name.as_deref(),
        )
        .into_iter()
        .map(|target| match target {
            WorktreeCreateTarget::CurrentBranch => WorktreeEntry::CreateFromCurrentBranch,
            WorktreeCreateTarget::DefaultBranch(default_branch) => {
                WorktreeEntry::CreateFromDefaultBranch { default_branch }
            }
        })
        .collect()
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

    fn refresh_project_worktree_paths(&mut self, window: &mut Window, cx: &mut App) {
        let mut paths = self.active_worktree_paths.clone();

        if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten()
            && let Some(workspace) = self.workspace.upgrade()
        {
            let group_key = workspace.read(cx).project_group_key(cx);
            for group_workspace in multi_workspace
                .read(cx)
                .workspaces_for_project_group(&group_key, cx)
            {
                for worktree in group_workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                {
                    paths.insert(worktree.read(cx).abs_path().to_path_buf());
                }
            }
        }

        self.project_worktree_paths = paths;
    }

    fn worktree_row_index(&self, worktree_path: &Path) -> Option<usize> {
        self.matches.iter().position(|entry| {
            matches!(entry, WorktreeEntry::Worktree { worktree, .. } if worktree.path == worktree_path)
        })
    }

    fn is_force_delete_hovering_index(&self, index: usize) -> bool {
        self.modifiers.alt && self.hovered_delete_index == Some(index)
    }

    fn delete_worktree(
        &mut self,
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
        if !self.can_delete_worktree(worktree)
            || self.deleting_worktree_paths.contains(&worktree.path)
        {
            return;
        }

        let repo = self.project.read(cx).active_repository(cx);
        let Some(repo) = repo else {
            return;
        };
        let path = worktree.path.clone();
        let display_name = self.worktree_name(worktree);
        let workspace = self.workspace.clone();

        self.deleting_worktree_paths.insert(path.clone());
        if self.hovered_delete_index == Some(ix) {
            self.hovered_delete_index = None;
        }
        cx.notify();

        cx.spawn_in(window, async move |picker, cx| {
            let initial_result = match repo
                .update(cx, |repo, _| repo.remove_worktree(path.clone(), force))
                .await
            {
                Ok(result) => result,
                Err(error) => {
                    picker.update_in(cx, |picker, _window, cx| {
                        if picker.delegate.deleting_worktree_paths.remove(&path) {
                            cx.notify();
                        }
                    })?;
                    return Err(error.into());
                }
            };

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
                        picker.update_in(cx, |picker, _window, cx| {
                            if picker.delegate.deleting_worktree_paths.remove(&path) {
                                cx.notify();
                            }
                        })?;

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

                        let should_retry = picker.update_in(cx, |picker, _window, cx| {
                            let worktree_still_exists = picker
                                .delegate
                                .all_worktrees
                                .iter()
                                .any(|worktree| worktree.path == path);
                            if !worktree_still_exists
                                || !picker.delegate.deleting_worktree_paths.insert(path.clone())
                            {
                                return false;
                            }
                            cx.notify();
                            true
                        })?;

                        if !should_retry {
                            return Ok(());
                        }

                        let retry = match repo
                            .update(cx, |repo, _| repo.remove_worktree(path.clone(), true))
                            .await
                        {
                            Ok(result) => result,
                            Err(error) => {
                                picker.update_in(cx, |picker, _window, cx| {
                                    if picker.delegate.deleting_worktree_paths.remove(&path) {
                                        cx.notify();
                                    }
                                })?;
                                return Err(error.into());
                            }
                        };

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
                picker.update_in(cx, |picker, _window, cx| {
                    if picker.delegate.deleting_worktree_paths.remove(&path) {
                        cx.notify();
                    }
                })?;

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
                picker.delegate.deleting_worktree_paths.remove(&path);
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

    /// Finds the workspace in this window (other than the picker's own
    /// workspace) that has `worktree_path` open as a visible worktree.
    fn workspace_for_open_worktree(
        &self,
        worktree_path: &Path,
        window: &Window,
        cx: &App,
    ) -> Option<Entity<Workspace>> {
        if self.active_worktree_paths.contains(worktree_path) {
            return None;
        }
        let multi_workspace = window.root::<MultiWorkspace>().flatten()?;
        let workspace = self.workspace.upgrade()?;
        let group_key = workspace.read(cx).project_group_key(cx);
        multi_workspace
            .read(cx)
            .workspaces_for_project_group(&group_key, cx)
            .find(|group_workspace| {
                *group_workspace != &workspace
                    && group_workspace
                        .read(cx)
                        .project()
                        .read(cx)
                        .visible_worktrees(cx)
                        .any(|worktree| worktree.read(cx).abs_path().as_ref() == worktree_path)
            })
            .cloned()
    }

    fn remove_worktree_from_window(
        &mut self,
        worktree_path: &Path,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.deleting_worktree_paths.contains(worktree_path) {
            return;
        }
        let Some(workspace_to_remove) = self.workspace_for_open_worktree(worktree_path, window, cx)
        else {
            return;
        };
        let Some(window_handle) = window.window_handle().downcast::<MultiWorkspace>() else {
            return;
        };

        cx.spawn_in(window, async move |picker, cx| {
            let removed = window_handle
                .update(cx, |multi_workspace, window, cx| {
                    multi_workspace.remove(
                        [workspace_to_remove.clone()],
                        RemovalIntent::CloseProject,
                        window,
                        cx,
                    )
                })?
                .await?;

            if removed {
                picker.update_in(cx, |picker, window, cx| {
                    picker.delegate.refresh_project_worktree_paths(window, cx);
                    picker.refresh(window, cx);
                })?;
            }

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn set_matches(&mut self, matches: Vec<WorktreeEntry>, query: String, keep_selection: bool) {
        let is_event_refresh = keep_selection && query == self.matches_query;
        let kept_index = is_event_refresh
            .then(|| self.matches.get(self.selected_index))
            .flatten()
            .and_then(|selected_row| {
                matches
                    .iter()
                    .position(|entry| entry.is_same_row(selected_row))
            });
        self.matches = matches;
        match kept_index {
            Some(index) => self.selected_index = index,
            None if is_event_refresh && query.is_empty() => {
                let last_index = self.matches.len().saturating_sub(1);
                self.selected_index = (0..=self.selected_index.min(last_index))
                    .rev()
                    .find(|index| {
                        !matches!(
                            self.matches.get(*index),
                            Some(WorktreeEntry::Separator | WorktreeEntry::SectionHeader(_))
                        )
                    })
                    .unwrap_or(0);
            }
            None => self.sync_selected_index(!query.is_empty()),
        }
        self.matches_query = query;
    }

    fn sync_selected_index(&mut self, has_query: bool) {
        if has_query {
            self.selected_index = self.default_selected_index();
        }
    }

    fn default_selected_index(&self) -> usize {
        self.matches
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    WorktreeEntry::Worktree { .. }
                        | WorktreeEntry::OpenWorkspace { .. }
                        | WorktreeEntry::ClosedCheckout { .. }
                )
            })
            .or_else(|| {
                self.matches
                    .iter()
                    .position(|entry| matches!(entry, WorktreeEntry::CreateNamed { .. }))
            })
            .unwrap_or(0)
    }
}

impl PickerDelegate for WorktreePickerDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "worktree picker"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Find a worktree or branch, or type to create…".into()
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
        !matches!(
            self.matches.get(ix),
            Some(WorktreeEntry::Separator | WorktreeEntry::SectionHeader(_))
        )
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let repo_worktrees = self.all_repo_worktrees().to_vec();
        let keep_selection = std::mem::take(&mut self.keep_selection_on_update);
        let open_workspaces = self.other_open_workspaces(window, cx);
        self.observe_window_workspaces(&open_workspaces, window, cx);
        let closed_checkouts = self.closed_group_checkouts();

        let normalized_query = query.replace(' ', "-");
        let worktree_name_anchor = self.worktree_name_anchor().map(Path::to_path_buf);
        let delta_clone_path = self.delta_clone_path.clone();
        let path_style = self.path_style;
        let row_name = move |worktree: &GitWorktree| {
            worktree_row_name(
                worktree,
                worktree_name_anchor.as_deref(),
                delta_clone_path.as_deref(),
                path_style,
            )
        };
        let has_named_worktree = self.all_worktrees.iter().any(|worktree| {
            worktree.directory_name(self.worktree_name_anchor(), self.path_style)
                == normalized_query
        });
        let create_named_disabled_reason: Option<String> = if self.has_multiple_repositories {
            Some("Cannot create a named worktree in a project with multiple repositories".into())
        } else if has_named_worktree {
            Some("A worktree with this name already exists".into())
        } else {
            None
        };

        let show_default_branch_create =
            !self.has_multiple_repositories && self.default_branch.is_some();
        let default_branch = self.default_branch.clone();

        if query.is_empty() {
            let mut matches = self.build_fixed_entries();

            if !repo_worktrees.is_empty()
                || !open_workspaces.is_empty()
                || !closed_checkouts.is_empty()
            {
                let project_paths = &self.project_worktree_paths;

                let sort_by_name = |a: &GitWorktree, b: &GitWorktree| row_name(a).cmp(&row_name(b));
                let has_open_workspaces = !open_workspaces.is_empty();
                let open_workspace_entries = open_workspaces.into_iter().map(|open_workspace| {
                    WorktreeEntry::OpenWorkspace {
                        open_workspace,
                        positions: Vec::new(),
                        branch_positions: Vec::new(),
                    }
                });
                let has_closed_checkouts = !closed_checkouts.is_empty();
                let closed_checkout_entries =
                    closed_checkouts
                        .into_iter()
                        .map(|checkout| WorktreeEntry::ClosedCheckout {
                            checkout,
                            positions: Vec::new(),
                            branch_positions: Vec::new(),
                        });

                let (mut open_here, mut others): (Vec<_>, Vec<_>) = repo_worktrees
                    .into_iter()
                    .partition(|worktree| project_paths.contains(&worktree.path));
                open_here.sort_by(sort_by_name);
                others.sort_by(sort_by_name);

                matches.push(WorktreeEntry::Separator);

                if open_here.len() > 1 || has_open_workspaces {
                    matches.push(WorktreeEntry::SectionHeader("This Window".into()));
                    for worktree in open_here {
                        matches.push(WorktreeEntry::Worktree {
                            worktree,
                            positions: Vec::new(),
                            branch_positions: Vec::new(),
                        });
                    }
                    matches.extend(open_workspace_entries);

                    if !others.is_empty() || has_closed_checkouts {
                        matches.push(WorktreeEntry::Separator);
                    }

                    for worktree in others {
                        matches.push(WorktreeEntry::Worktree {
                            worktree,
                            positions: Vec::new(),
                            branch_positions: Vec::new(),
                        });
                    }
                } else {
                    for worktree in open_here.into_iter().chain(others) {
                        matches.push(WorktreeEntry::Worktree {
                            worktree,
                            positions: Vec::new(),
                            branch_positions: Vec::new(),
                        });
                    }
                }
                matches.extend(closed_checkout_entries);
            }

            self.set_matches(matches, query, keep_selection);
            return Task::ready(());
        }

        let mut candidates = Vec::new();
        for (index, worktree) in repo_worktrees.iter().enumerate() {
            candidates.push(StringMatchCandidate::new(index * 2, &row_name(worktree)));
            if let Some(branch) = worktree.branch_name() {
                candidates.push(StringMatchCandidate::new(index * 2 + 1, branch));
            }
        }
        for (index, open_workspace) in open_workspaces.iter().enumerate() {
            let row_index = repo_worktrees.len() + index;
            candidates.push(StringMatchCandidate::new(
                row_index * 2,
                &open_workspace.name,
            ));
            if let Some(branch) = &open_workspace.branch {
                candidates.push(StringMatchCandidate::new(row_index * 2 + 1, branch));
            }
        }
        for (index, checkout) in closed_checkouts.iter().enumerate() {
            let row_index = repo_worktrees.len() + open_workspaces.len() + index;
            candidates.push(StringMatchCandidate::new(row_index * 2, &checkout.name));
            if let Some(branch) = &checkout.branch {
                candidates.push(StringMatchCandidate::new(row_index * 2 + 1, branch));
            }
        }

        let executor = cx.background_executor().clone();

        let matches_query = query.clone();
        let task = cx.background_executor().spawn(async move {
            fuzzy::match_strings(
                &candidates,
                &query,
                true,
                true,
                candidates.len(),
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

                    let mut matched_worktrees = HashMap::default();
                    for candidate in fuzzy_matches {
                        let worktree_index = candidate.candidate_id / 2;
                        let entry = if let Some(worktree) = repo_worktrees_clone.get(worktree_index)
                        {
                            WorktreeEntry::Worktree {
                                worktree: worktree.clone(),
                                positions: Vec::new(),
                                branch_positions: Vec::new(),
                            }
                        } else if let Some(open_workspace) = worktree_index
                            .checked_sub(repo_worktrees_clone.len())
                            .and_then(|index| open_workspaces.get(index))
                        {
                            WorktreeEntry::OpenWorkspace {
                                open_workspace: open_workspace.clone(),
                                positions: Vec::new(),
                                branch_positions: Vec::new(),
                            }
                        } else if let Some(checkout) = worktree_index
                            .checked_sub(repo_worktrees_clone.len() + open_workspaces.len())
                            .and_then(|index| closed_checkouts.get(index))
                        {
                            WorktreeEntry::ClosedCheckout {
                                checkout: checkout.clone(),
                                positions: Vec::new(),
                                branch_positions: Vec::new(),
                            }
                        } else {
                            continue;
                        };
                        // Matches are ranked, so the first field hit determines the row's rank.
                        let entry_index =
                            *matched_worktrees.entry(worktree_index).or_insert_with(|| {
                                let entry_index = new_matches.len();
                                new_matches.push(entry);
                                entry_index
                            });
                        if let Some(
                            WorktreeEntry::Worktree {
                                positions,
                                branch_positions,
                                ..
                            }
                            | WorktreeEntry::OpenWorkspace {
                                positions,
                                branch_positions,
                                ..
                            }
                            | WorktreeEntry::ClosedCheckout {
                                positions,
                                branch_positions,
                                ..
                            },
                        ) = new_matches.get_mut(entry_index)
                        {
                            if candidate.candidate_id % 2 == 0 {
                                *positions = candidate.positions;
                            } else {
                                *branch_positions = candidate.positions;
                            }
                        }
                    }
                    new_matches.truncate(10000);

                    if !new_matches.is_empty() {
                        new_matches.push(WorktreeEntry::Separator);
                    }
                    if show_default_branch_create {
                        if let Some(ref default_branch) = default_branch {
                            new_matches.push(WorktreeEntry::CreateNamed {
                                name: normalized_query.clone(),
                                from_branch: Some(default_branch.clone()),
                                disabled_reason: create_named_disabled_reason.clone(),
                            });
                        }
                    } else {
                        new_matches.push(WorktreeEntry::CreateNamed {
                            name: normalized_query.clone(),
                            from_branch: None,
                            disabled_reason: create_named_disabled_reason.clone(),
                        });
                    }

                    picker
                        .delegate
                        .set_matches(new_matches, matches_query, keep_selection);

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
            WorktreeEntry::Separator | WorktreeEntry::SectionHeader(_) => return,
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
            WorktreeEntry::CreateFromDefaultBranch { default_branch } => {
                if self.creation_blocked_reason(cx).is_some() {
                    return;
                }
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        crate::worktree_service::handle_create_worktree(
                            workspace,
                            &CreateWorktree {
                                worktree_name: None,
                                branch_target: NewWorktreeBranchTarget::RemoteBranch {
                                    remote_name: default_branch.remote_name.clone(),
                                    branch_name: default_branch.branch_name.clone(),
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
                if self.deleting_worktree_paths.contains(&worktree.path) {
                    return;
                }

                let is_current = self.active_worktree_paths.contains(&worktree.path);

                if !is_current {
                    if secondary {
                        window.dispatch_action(
                            Box::new(OpenWorktreeInNewWindow {
                                path: worktree.path.clone(),
                            }),
                            cx,
                        );
                    } else {
                        if let Some(workspace) = self.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                crate::worktree_service::handle_switch_worktree(
                                    workspace,
                                    &SwitchWorktree {
                                        path: worktree.path.clone(),
                                        display_name: self.worktree_name(worktree),
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
            WorktreeEntry::OpenWorkspace { open_workspace, .. } => {
                if let Some(open_workspace) = open_workspace.workspace.upgrade() {
                    if secondary {
                        let paths = open_workspace
                            .read(cx)
                            .root_paths(cx)
                            .iter()
                            .map(|root_path| root_path.to_path_buf())
                            .collect::<Vec<_>>();
                        if let Some(workspace) = self.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                open_worktree_paths_in_new_window(workspace, paths, window, cx);
                            });
                        }
                    } else if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten()
                    {
                        window.defer(cx, move |window, cx| {
                            multi_workspace.update(cx, |multi_workspace, cx| {
                                if multi_workspace
                                    .workspaces()
                                    .any(|workspace| workspace == &open_workspace)
                                {
                                    multi_workspace.activate(open_workspace, None, window, cx);
                                }
                            });
                        });
                    }
                }
            }
            WorktreeEntry::ClosedCheckout { checkout, .. } => {
                let checkout_path = checkout.path.clone();
                let fs = self.project.read(cx).fs().clone();
                if secondary {
                    let source_workspace = self.workspace.clone();
                    window
                        .spawn(cx, async move |cx| {
                            match ensure_checkout_available(&checkout_path, fs.as_ref()).await {
                                Ok(()) => {
                                    source_workspace
                                        .update_in(cx, |workspace, window, cx| {
                                            open_worktree_paths_in_new_window(
                                                workspace,
                                                vec![checkout_path],
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                                Err(error) => {
                                    report_checkout_open_error(&source_workspace, error, cx)
                                }
                            }
                        })
                        .detach();
                } else if let Some(multi_workspace) = window.root::<MultiWorkspace>().flatten()
                    && let Some(workspace) = self.workspace.upgrade()
                    && workspace
                        .read(cx)
                        .active_worktree_creation()
                        .label
                        .is_none()
                {
                    let (_, non_git_paths) = classify_worktrees(self.project.read(cx), cx);
                    let paths = switch_worktree_paths(&checkout_path, &non_git_paths);
                    workspace.update(cx, |workspace, cx| {
                        workspace.set_active_worktree_creation(
                            Some(SharedString::from(checkout.name.clone())),
                            true,
                            cx,
                        );
                    });
                    let source_workspace = workspace.downgrade();
                    let multi_workspace = multi_workspace.downgrade();
                    window
                        .spawn(cx, async move |cx| {
                            let opened = match ensure_checkout_available(
                                &checkout_path,
                                fs.as_ref(),
                            )
                            .await
                            {
                                Ok(()) => {
                                    let Ok(open_task) = multi_workspace.update_in(
                                        cx,
                                        |multi_workspace, window, cx| {
                                            multi_workspace.find_or_create_local_workspace(
                                                PathList::new(&paths),
                                                None,
                                                None,
                                                OpenMode::Add,
                                                None,
                                                window,
                                                cx,
                                            )
                                        },
                                    ) else {
                                        return;
                                    };
                                    open_task.await
                                }
                                Err(error) => Err(error),
                            };
                            source_workspace
                                .update(cx, |workspace, cx| {
                                    workspace.set_active_worktree_creation(None, false, cx);
                                })
                                .ok();
                            match opened {
                                Ok(workspace) => {
                                    multi_workspace
                                        .update_in(cx, |multi_workspace, window, cx| {
                                            multi_workspace.activate(workspace, None, window, cx);
                                        })
                                        .ok();
                                }
                                Err(error) => {
                                    report_checkout_open_error(&source_workspace, error, cx)
                                }
                            }
                        })
                        .detach();
                }
            }
            WorktreeEntry::CreateNamed {
                name,
                from_branch,
                disabled_reason: None,
            } => {
                let branch_target = match from_branch {
                    Some(branch) => NewWorktreeBranchTarget::RemoteBranch {
                        remote_name: branch.remote_name.clone(),
                        branch_name: branch.branch_name.clone(),
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
            WorktreeEntry::SectionHeader(label) => Some(
                ListSubHeader::new(label.clone())
                    .inset(true)
                    .into_any_element(),
            ),
            WorktreeEntry::CreateFromCurrentBranch => {
                let branch_label = WorktreeCreateTarget::CurrentBranch.branch_label(
                    self.has_multiple_repositories,
                    self.current_branch_name.as_deref(),
                );

                let label = format!("Create new worktree based on {branch_label}");

                let item = create_new_list_item(
                    SharedString::new_static("create-from-current"),
                    label.into(),
                    self.creation_blocked_reason(cx),
                    selected,
                );

                Some(item.into_any_element())
            }
            WorktreeEntry::CreateFromDefaultBranch { default_branch } => {
                let branch_label = WorktreeCreateTarget::DefaultBranch(default_branch.clone())
                    .branch_label(
                        self.has_multiple_repositories,
                        self.current_branch_name.as_deref(),
                    );
                let label = format!("Create new worktree based on {branch_label}");

                let item = create_new_list_item(
                    SharedString::new_static("create-from-main"),
                    label.into(),
                    self.creation_blocked_reason(cx),
                    selected,
                );

                Some(item.into_any_element())
            }
            WorktreeEntry::OpenWorkspace {
                open_workspace,
                positions,
                branch_positions,
            } => Some(render_workspace_row(
                SharedString::from(format!("open-workspace-{ix}")),
                open_workspace.name.clone(),
                positions.clone(),
                open_workspace.branch.clone(),
                branch_positions.clone(),
                open_workspace.paths.clone(),
                selected,
            )),
            WorktreeEntry::ClosedCheckout {
                checkout,
                positions,
                branch_positions,
            } => Some(render_workspace_row(
                SharedString::from(format!("closed-checkout-{ix}")),
                checkout.name.clone(),
                positions.clone(),
                checkout.branch.clone(),
                branch_positions.clone(),
                checkout.path.compact().to_string_lossy().into_owned(),
                selected,
            )),
            WorktreeEntry::Worktree {
                worktree,
                positions,
                branch_positions,
            } => {
                let display_name = self.worktree_name(worktree);
                let first_line = display_name.lines().next().unwrap_or(&display_name);
                let positions: Vec<_> = positions
                    .iter()
                    .copied()
                    .filter(|&pos| pos < first_line.len())
                    .collect();
                let path = worktree.path.compact().to_string_lossy().to_string();
                let sha = worktree.sha.chars().take(7).collect::<String>();

                let is_current = self.active_worktree_paths.contains(&worktree.path);
                let is_deleting = self.deleting_worktree_paths.contains(&worktree.path);
                let can_delete = self.can_delete_worktree(worktree);
                let can_remove_from_window =
                    !is_current && self.project_worktree_paths.contains(&worktree.path);

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
                                                            HighlightedLabel::new(
                                                                branch,
                                                                branch_positions.clone(),
                                                            )
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
                        .when(is_deleting, |this| {
                            this.end_slot(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::LoadCircle)
                                            .size(IconSize::Small)
                                            .color(Color::Muted)
                                            .with_rotate_animation(2),
                                    )
                                    .child(
                                        Label::new("Deleting…")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                        })
                        .when(!is_deleting && !is_current, |this| {
                            let open_path = worktree.path.clone();
                            let delete_path = worktree.path.clone();
                            let open_in_new_window_button =
                                IconButton::new(("open-new-window", ix), IconName::ArrowUpRight)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Open in New Window"))
                                    .on_click(cx.listener(move |picker, _, window, cx| {
                                        if picker.delegate.worktree_row_index(&open_path).is_none()
                                            || picker
                                                .delegate
                                                .deleting_worktree_paths
                                                .contains(&open_path)
                                        {
                                            return;
                                        }
                                        window.dispatch_action(
                                            Box::new(OpenWorktreeInNewWindow {
                                                path: open_path.clone(),
                                            }),
                                            cx,
                                        );
                                        cx.emit(DismissEvent);
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
                                            let force = picker.delegate.modifiers.alt;
                                            if let Some(ix) =
                                                picker.delegate.worktree_row_index(&delete_path)
                                            {
                                                picker
                                                    .delegate
                                                    .delete_worktree(ix, force, window, cx);
                                            }
                                        })),
                                );

                            this.end_slot(
                                h_flex()
                                    .gap_0p5()
                                    .child(open_in_new_window_button)
                                    .when(can_remove_from_window, |this| {
                                        let worktree_path = worktree.path.clone();
                                        this.child(
                                            IconButton::new(
                                                ("remove-worktree-from-window", ix),
                                                IconName::Close,
                                            )
                                            .icon_size(IconSize::Small)
                                            .tooltip(Tooltip::text("Remove Worktree from Window"))
                                            .on_click(
                                                cx.listener(move |picker, _, window, cx| {
                                                    picker.delegate.remove_worktree_from_window(
                                                        &worktree_path,
                                                        window,
                                                        cx,
                                                    );
                                                }),
                                            ),
                                        )
                                    })
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
                    .as_ref()
                    .map(RemoteBranchName::display_name)
                    .unwrap_or_else(|| {
                        self.current_branch_name
                            .clone()
                            .unwrap_or_else(|| "HEAD".to_string())
                    });
                let label = format!("Create \"{name}\" based on {branch_label}");
                let element_id = match from_branch {
                    Some(branch) => format!("create-named-from-{}", branch.display_name()),
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

    fn searchbar_trailer(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        if self.show_footer {
            return None;
        }

        let focus_handle = self.focus_handle.clone();

        Some(
            IconButton::new("configure-worktree-tasks", IconName::Settings)
                .icon_size(IconSize::Small)
                .tooltip(move |_window, cx| {
                    Tooltip::for_action_in(
                        "Automate Worktree Setup",
                        &OpenWorktreeSetupTasks,
                        &focus_handle,
                        cx,
                    )
                })
                .on_click(|_, window, cx| {
                    window.dispatch_action(OpenWorktreeSetupTasks.boxed_clone(), cx)
                })
                .into_any_element(),
        )
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
        let is_open_workspace = selected_entry.is_some_and(|e| {
            matches!(
                e,
                WorktreeEntry::OpenWorkspace { .. } | WorktreeEntry::ClosedCheckout { .. }
            )
        });

        let can_delete = selected_entry.is_some_and(|e| {
            matches!(e, WorktreeEntry::Worktree { worktree, .. } if self.can_delete_worktree(worktree))
        });

        let is_current = selected_entry.is_some_and(|e| {
            matches!(e, WorktreeEntry::Worktree { worktree, .. } if self.project_worktree_paths.contains(&worktree.path))
        });

        let is_deleting = selected_entry.is_some_and(|e| {
            matches!(e, WorktreeEntry::Worktree { worktree, .. } if self.deleting_worktree_paths.contains(&worktree.path))
        });

        let footer = h_flex()
            .w_full()
            .p_1p5()
            .gap_0p5()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Button::new("configure-worktree-tasks", "Automate Setup")
                    .key_binding(
                        KeyBinding::for_action_in(&OpenWorktreeSetupTasks, &focus_handle, cx)
                            .map(|kb| kb.size(rems_from_px(12_f32))),
                    )
                    .on_click(|_, window, cx| {
                        window.dispatch_action(OpenWorktreeSetupTasks.boxed_clone(), cx)
                    }),
            );

        if is_creating {
            Some(
                footer
                    .child(
                        Button::new("create-worktree", "Create")
                            .key_binding(
                                KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                    .map(|kb| kb.size(rems_from_px(12_f32))),
                            )
                            .on_click(|_, window, cx| {
                                window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                            }),
                    )
                    .into_any(),
            )
        } else if is_existing_worktree || is_open_workspace {
            Some(
                footer
                    .child(
                        h_flex()
                            .gap_0p5()
                            .when(is_deleting, |this| {
                                this.child(
                                    Button::new("delete-worktree", "Deleting…")
                                        .loading(true)
                                        .disabled(true),
                                )
                            })
                            .when(!is_deleting && can_delete, |this| {
                                let focus_handle = focus_handle.clone();
                                this.child(
                                    Button::new("delete-worktree", "Delete")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &DeleteWorktree,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12_f32))),
                                        )
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(DeleteWorktree.boxed_clone(), cx)
                                        }),
                                )
                            })
                            .when(!is_deleting && !is_current, |this| {
                                let focus_handle = focus_handle.clone();
                                this.child(
                                    Button::new("open-in-new-window", "Open in New Window")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::SecondaryConfirm,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12_f32))),
                                        )
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(
                                                menu::SecondaryConfirm.boxed_clone(),
                                                cx,
                                            )
                                        }),
                                )
                            })
                            .when(!is_deleting, |this| {
                                this.child(
                                    Button::new("open-worktree", "Open")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Confirm,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12_f32))),
                                        )
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                        }),
                                )
                            }),
                    )
                    .into_any(),
            )
        } else {
            None
        }
    }
}

fn render_workspace_row(
    id: SharedString,
    name: String,
    positions: Vec<usize>,
    branch: Option<String>,
    branch_positions: Vec<usize>,
    paths: String,
    selected: bool,
) -> AnyElement {
    ListItem::new(id)
        .inset(true)
        .spacing(ListItemSpacing::Sparse)
        .toggle_state(selected)
        .child(
            h_flex()
                .w_full()
                .gap_2p5()
                .child(
                    Icon::new(IconName::GitWorktree)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )
                .child(
                    v_flex()
                        .w_full()
                        .min_w_0()
                        .child(HighlightedLabel::new(name, positions).truncate())
                        .child(
                            h_flex()
                                .w_full()
                                .min_w_0()
                                .gap_1p5()
                                .when_some(branch, |this, branch| {
                                    this.child(
                                        HighlightedLabel::new(branch, branch_positions)
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
                                    Label::new(paths)
                                        .truncate_start()
                                        .color(Color::Muted)
                                        .size(LabelSize::Small)
                                        .flex_1(),
                                ),
                        ),
                ),
        )
        .into_any_element()
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

pub fn open_worktree_paths_in_new_window(
    workspace: &mut Workspace,
    paths: Vec<PathBuf>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let is_remote = !workspace.project().read(cx).is_local();

    if is_remote {
        let connection_options = workspace
            .project()
            .read(cx)
            .remote_connection_options(cx)
            .cloned();
        let app_state = workspace.app_state().clone();
        let workspace_handle = workspace.weak_handle();
        cx.spawn_in(window, async move |_, cx| {
            if let Some(connection_options) = connection_options {
                open_remote_worktree(connection_options, paths, app_state, workspace_handle, cx)
                    .await?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    } else {
        workspace
            .open_workspace_for_paths(OpenMode::NewWindow, paths, window, cx)
            .detach_and_log_err(cx);
    }
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
    use fs::{FakeFs, RemoveOptions, RenameOptions};
    use gpui::{AnyWindowHandle, AppContext, TestAppContext, VisualTestContext};
    use project::project_settings::ProjectSettings;
    use project::{Project, WorktreeSettings};
    use serde_json::json;
    use settings::Settings as _;
    use settings::SettingsStore;
    use util::path;
    use util::paths::SanitizedPath;
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

    #[test]
    fn test_bare_repository_worktree_uses_generated_name() {
        let worktree = GitWorktree {
            path: PathBuf::from("/worktrees/zed/plum-warbler/zed"),
            ref_name: None,
            sha: "8166e3d".into(),
            is_main: false,
            is_bare: false,
        };
        let worktrees = [worktree.clone()];
        let name_anchor = worktree_name_anchor(&worktrees, Some(Path::new("/repos/zed")));

        assert_eq!(
            worktree.directory_name(name_anchor, PathStyle::local()),
            "plum-warbler"
        );
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

        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);

        let worktree_picker = cx.update(|window, cx| {
            cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
        });

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

    fn picker_contains_worktree(
        worktree_picker: &Entity<WorktreePicker>,
        worktree_path: &Path,
        cx: &mut VisualTestContext,
    ) -> bool {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker.delegate.all_worktrees.iter().any(|worktree| {
                    worktree.path == *worktree_path
                }) && picker.delegate.matches.iter().any(|entry| {
                    matches!(entry, WorktreeEntry::Worktree { worktree, .. } if worktree.path == *worktree_path)
                })
            })
        })
    }

    fn deleting_worktree_paths(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> HashSet<PathBuf> {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker.delegate.deleting_worktree_paths.clone()
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
    async fn test_delete_worktree_marks_row_pending_immediately(cx: &mut TestAppContext) {
        let (_, worktree_picker, _repository, worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        let index = worktree_index(&worktree_picker, &worktree_path, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.delete_worktree(index, false, window, cx);
            })
        });

        let pending_paths = deleting_worktree_paths(&worktree_picker, &mut cx);
        assert_eq!(pending_paths.len(), 1);
        assert!(pending_paths.contains(&worktree_path));

        cx.run_until_parked();
    }

    #[gpui::test]
    async fn test_delete_worktree_clears_pending_and_removes_row_on_success(
        cx: &mut TestAppContext,
    ) {
        let (_, worktree_picker, repository, worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        let index = worktree_index(&worktree_picker, &worktree_path, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.delete_worktree(index, false, window, cx);
            })
        });
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).contains(&worktree_path));

        cx.run_until_parked();

        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).is_empty());
        assert!(!picker_contains_worktree(
            &worktree_picker,
            &worktree_path,
            &mut cx
        ));
        assert!(
            !repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "worktree should be removed after successful delete"
        );
    }

    #[gpui::test]
    async fn test_search_worktrees_by_name_and_branch(cx: &mut TestAppContext) {
        let (_fs, worktree_picker, _repository, _worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker.delegate.all_worktrees = [
                    ("blue-fox", Some("feature/search")),
                    ("same", Some("same")),
                    ("detached", None),
                    ("café", Some("fix/été")),
                ]
                .into_iter()
                .map(|(name, branch)| GitWorktree {
                    path: Path::new(path!("/root/worktrees")).join(name),
                    ref_name: branch.map(|branch| format!("refs/heads/{branch}").into()),
                    sha: "deadbeef".into(),
                    is_main: false,
                    is_bare: false,
                })
                .collect();
            });
        });

        for (query, name, expected_positions, expected_branch_positions) in [
            ("blue", "blue-fox", vec![0, 1, 2, 3], vec![]),
            ("feature/search", "blue-fox", vec![], (0..14).collect()),
            ("same", "same", vec![0, 1, 2, 3], vec![0, 1, 2, 3]),
            ("detached", "detached", (0..8).collect(), vec![]),
            ("café", "café", vec![0, 1, 2, 3], vec![]),
            ("été", "café", vec![], vec![4, 6, 7]),
        ] {
            worktree_picker
                .update_in(&mut cx, |worktree_picker, window, cx| {
                    worktree_picker.picker.update(cx, |picker, cx| {
                        picker.delegate.update_matches(query.into(), window, cx)
                    })
                })
                .await;

            worktree_picker.update(&mut cx, |worktree_picker, cx| {
                worktree_picker.picker.update(cx, |picker, _| {
                    let matches = &picker.delegate.matches;
                    let expected_path = Path::new(path!("/root/worktrees")).join(name);
                    assert_eq!(
                        matches
                            .iter()
                            .filter(|entry| matches!(entry, WorktreeEntry::Worktree { .. }))
                            .count(),
                        1,
                        "query: {query}",
                    );
                    assert!(
                        matches!(
                            matches.get(picker.delegate.selected_index),
                            Some(WorktreeEntry::Worktree { worktree, positions, branch_positions })
                                if worktree.path == expected_path
                                    && positions == &expected_positions
                                    && branch_positions == &expected_branch_positions
                        ),
                        "query: {query}"
                    );
                    assert!(
                        matches!(
                            matches.last(),
                            Some(WorktreeEntry::CreateNamed { disabled_reason, .. })
                                if disabled_reason.is_some() == (query == name)
                        ),
                        "branch matches must not disable worktree creation: {query}"
                    );
                });
            });
        }

        for query in ["bluefeature", "missing worktree"] {
            worktree_picker
                .update_in(&mut cx, |worktree_picker, window, cx| {
                    worktree_picker.picker.update(cx, |picker, cx| {
                        picker.delegate.update_matches(query.into(), window, cx)
                    })
                })
                .await;
            worktree_picker.update(&mut cx, |worktree_picker, cx| {
                worktree_picker.picker.update(cx, |picker, _| {
                    assert_eq!(picker.delegate.matches.len(), 1);
                    assert!(matches!(
                        picker.delegate.matches.get(picker.delegate.selected_index),
                        Some(WorktreeEntry::CreateNamed { name, disabled_reason: None, .. })
                            if name == &query.replace(' ', "-")
                    ));
                });
            });
        }
    }

    #[gpui::test]
    async fn test_search_worktrees_ranks_by_best_field(cx: &mut TestAppContext) {
        let (_fs, worktree_picker, _repository, _worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        worktree_picker
            .update_in(&mut cx, |worktree_picker, window, cx| {
                worktree_picker.picker.update(cx, |picker, cx| {
                    picker.delegate.all_worktrees = [
                        ("same-long", "unrelated"),
                        ("branch-only", "same"),
                        ("other/same", "same"),
                        ("same", "same"),
                    ]
                    .into_iter()
                    .map(|(name, branch)| GitWorktree {
                        path: Path::new(path!("/root/worktrees")).join(name),
                        ref_name: Some(format!("refs/heads/{branch}").into()),
                        sha: "deadbeef".into(),
                        is_main: false,
                        is_bare: false,
                    })
                    .collect();
                    picker.delegate.update_matches("same".into(), window, cx)
                })
            })
            .await;

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                let paths: Vec<_> = picker
                    .delegate
                    .matches
                    .iter()
                    .filter_map(|entry| match entry {
                        WorktreeEntry::Worktree { worktree, .. } => Some(worktree.path.clone()),
                        _ => None,
                    })
                    .collect();
                assert_eq!(
                    paths,
                    ["same", "other/same", "branch-only", "same-long"]
                        .map(|name| { Path::new(path!("/root/worktrees")).join(name) })
                );
            });
        });
    }

    #[gpui::test]
    async fn test_remote_default_branch_is_preferred_create_target(cx: &mut TestAppContext) {
        let (_fs, worktree_picker, _repository, _worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                assert_eq!(picker.delegate.selected_index, 0);
                match picker.delegate.matches.first() {
                    Some(WorktreeEntry::CreateFromDefaultBranch { default_branch }) => {
                        assert_eq!(default_branch.display_name(), "origin/main");
                    }
                    _ => panic!("remote default branch should be the first create target"),
                }
            })
        });

        let update_matches = worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker
                    .delegate
                    .update_matches("feature".to_string(), window, cx)
            })
        });
        update_matches.await;
        cx.run_until_parked();

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker
                .picker
                .update(cx, |picker, _| match picker.delegate.matches.first() {
                    Some(WorktreeEntry::CreateNamed {
                        from_branch: Some(default_branch),
                        ..
                    }) => {
                        assert_eq!(default_branch.display_name(), "origin/main");
                    }
                    _ => panic!("named worktree creation should prefer the remote default branch"),
                })
        });
    }

    #[gpui::test]
    async fn test_current_branch_create_target_is_shown_without_default_branch(
        cx: &mut TestAppContext,
    ) {
        let (_fs, worktree_picker, _repository, _worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.default_branch = None;
                picker.refresh(window, cx);
            });
        });
        cx.run_until_parked();

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                assert!(matches!(
                    picker.delegate.matches.first(),
                    Some(WorktreeEntry::CreateFromCurrentBranch)
                ));
                assert!(
                    !picker.delegate.matches.iter().any(|entry| matches!(
                        entry,
                        WorktreeEntry::CreateFromDefaultBranch { .. }
                    ))
                );
            });
        });
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
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).contains(&worktree_path));

        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        assert!(
            !deleting_worktree_paths(&worktree_picker, &mut cx).contains(&worktree_path),
            "pending delete state should clear while waiting for force-delete confirmation"
        );

        cx.simulate_prompt_answer("Force Delete");
        cx.run_until_parked();

        assert!(!cx.has_pending_prompt());
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).is_empty());
        assert!(!picker_contains_worktree(
            &worktree_picker,
            &worktree_path,
            &mut cx
        ));
        assert!(
            !repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "worktree should be removed after confirming force delete"
        );
    }

    #[gpui::test]
    async fn test_duplicate_delete_worktree_is_ignored_while_pending(cx: &mut TestAppContext) {
        let (fs, worktree_picker, _repository, worktree_path, mut cx) =
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
                picker.delegate.delete_worktree(index, false, window, cx);
            })
        });

        let pending_paths = deleting_worktree_paths(&worktree_picker, &mut cx);
        assert_eq!(pending_paths.len(), 1);
        assert!(pending_paths.contains(&worktree_path));

        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).is_empty());

        cx.simulate_prompt_answer("Cancel");
        cx.run_until_parked();

        assert!(!cx.has_pending_prompt());
        assert!(picker_contains_worktree(
            &worktree_picker,
            &worktree_path,
            &mut cx
        ));
    }

    #[gpui::test]
    async fn test_selected_deleting_worktree_cannot_be_opened(cx: &mut TestAppContext) {
        let (_, worktree_picker, _repository, worktree_path, mut cx) =
            init_worktree_picker_test(cx).await;

        let subscription = cx.update(|_, cx| {
            cx.subscribe(&worktree_picker, |_, _: &DismissEvent, _| {
                panic!("DismissEvent should not be emitted for a deleting worktree");
            })
        });

        let index = worktree_index(&worktree_picker, &worktree_path, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.selected_index = index;
                picker.delegate.delete_worktree(index, false, window, cx);
                picker.delegate.confirm(false, window, cx);
            })
        });

        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).contains(&worktree_path));

        drop(subscription);
        cx.run_until_parked();
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
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).contains(&worktree_path));

        cx.run_until_parked();

        assert!(!cx.has_pending_prompt());
        assert!(deleting_worktree_paths(&worktree_picker, &mut cx).is_empty());
        assert!(!picker_contains_worktree(
            &worktree_picker,
            &worktree_path,
            &mut cx
        ));
        assert!(
            !repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "worktree should be removed by explicit force delete"
        );
    }

    #[gpui::test]
    async fn test_open_worktrees_are_grouped_under_section_header(cx: &mut TestAppContext) {
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
        let second_worktree_path = PathBuf::from(path!("/root/worktrees/second-wt"));

        cx.update(|cx| {
            repository.update(cx, |repository, _| {
                repository.create_worktree(
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: "second-wt".to_string(),
                        base_sha: Some("deadbeef".to_string()),
                    },
                    second_worktree_path.clone(),
                )
            })
        })
        .await
        .unwrap()
        .unwrap();

        // Open the second worktree as a visible worktree of the active project so
        // that two worktrees of the same repo are open in this window.
        project
            .update(cx, |project, cx| {
                project.create_worktree(&second_worktree_path, true, cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();

        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);
        let worktree_picker = cx.update(|window, cx| {
            cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
        });
        cx.run_until_parked();

        let project_path = PathBuf::from(path!("/root/project"));
        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                let matches = &picker.delegate.matches;

                let header_index = matches
                    .iter()
                    .position(|entry| {
                        matches!(entry, WorktreeEntry::SectionHeader(label) if label.as_ref() == "This Window")
                    })
                    .expect("section header should be present when multiple worktrees are open");

                let grouped_paths: Vec<&Path> = matches[header_index + 1..]
                    .iter()
                    .map_while(|entry| match entry {
                        WorktreeEntry::Worktree { worktree, .. } => Some(worktree.path.as_path()),
                        _ => None,
                    })
                    .collect();

                assert!(
                    grouped_paths.contains(&project_path.as_path()),
                    "main worktree should be grouped under the header"
                );
                assert!(
                    grouped_paths.contains(&second_worktree_path.as_path()),
                    "second open worktree should be grouped under the header"
                );
            })
        });
    }

    #[gpui::test]
    async fn test_remove_open_worktree_workspace_from_window(cx: &mut TestAppContext) {
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
        let worktree_path = PathBuf::from(path!("/root/worktrees/open-wt"));
        cx.update(|cx| {
            repository.update(cx, |repository, _| {
                repository.create_worktree(
                    git::repository::CreateWorktreeTarget::NewBranch {
                        branch_name: "open-wt".to_string(),
                        base_sha: Some("deadbeef".to_string()),
                    },
                    worktree_path.clone(),
                )
            })
        })
        .await
        .unwrap()
        .unwrap();

        let worktree_project = Project::test(fs.clone(), [worktree_path.as_path()], cx).await;
        cx.executor().run_until_parked();

        let main_group_key = project.read_with(cx, |project, cx| project.project_group_key(cx));
        let worktree_group_key =
            worktree_project.read_with(cx, |project, cx| project.project_group_key(cx));
        assert_eq!(
            main_group_key, worktree_group_key,
            "the worktree workspace should belong to the same project group as the main repo"
        );

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let worktree_workspace = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let worktree_workspace =
                    cx.new(|cx| Workspace::test_new(worktree_project.clone(), window, cx));
                multi_workspace.add(worktree_workspace.clone(), window, cx);
                worktree_workspace
            })
            .unwrap();

        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);
        let worktree_picker = cx.update(|window, cx| {
            cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
        });
        cx.run_until_parked();

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                assert!(
                    picker
                        .delegate
                        .project_worktree_paths
                        .contains(&worktree_path),
                    "the worktree should be considered open in this window"
                );
            })
        });

        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker
                    .delegate
                    .remove_worktree_from_window(&worktree_path, window, cx);
            })
        });
        cx.run_until_parked();

        window_handle
            .read_with(&cx, |multi_workspace, _| {
                assert!(
                    multi_workspace
                        .workspaces()
                        .all(|workspace| *workspace != worktree_workspace),
                    "the worktree workspace should be removed from the window"
                );
            })
            .unwrap();

        worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                assert!(
                    !picker
                        .delegate
                        .project_worktree_paths
                        .contains(&worktree_path),
                    "the worktree should no longer be considered open in this window"
                );
            })
        });

        assert!(
            repo_contains_worktree(&repository, &worktree_path, &mut cx).await,
            "removing the worktree from the window should not delete the git worktree"
        );
    }

    #[test]
    fn test_worktree_row_name() {
        let worktree = |path: &str, is_main: bool| GitWorktree {
            path: PathBuf::from(path),
            ref_name: None,
            sha: "deadbeef".into(),
            is_main,
            is_bare: false,
        };
        let delta_clone_path = Path::new("/store/clones/thread-alpha/repo.git");
        let names = [
            (worktree("/repo", true), None),
            (
                worktree("/store/worktrees/thread-alpha/repo", true),
                Some(delta_clone_path),
            ),
            (worktree("/linked", false), Some(delta_clone_path)),
        ]
        .map(|(worktree, delta_clone_path)| {
            worktree_row_name(&worktree, None, delta_clone_path, PathStyle::Unix)
        });
        assert_eq!(
            names,
            [
                "main worktree".to_string(),
                "thread-alpha".to_string(),
                "thread-alpha/repo/linked".to_string(),
            ]
        );
    }

    #[gpui::test]
    async fn test_open_workspaces_of_the_project_group_are_listed(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    ".git": {},
                    "file.txt": "buffer_text",
                },
                "notes": { "todo.md": "" },
                "store": {
                    "clones": {
                        "thread-alpha": {
                            "project.git": {
                                "HEAD": "ref: refs/heads/feature-x\n",
                                "config": "[core]\n\tbare = false\n",
                                "delta": { "copy-baseline": "" },
                                "objects": {
                                    "info": {
                                        "alternates": format!(
                                            "{}\n",
                                            path!("/root/project/.git/objects")
                                        ),
                                    },
                                },
                            },
                        },
                    },
                    "worktrees": {
                        "thread-alpha": {
                            "project": {
                                ".git": format!(
                                    "gitdir: {}\n",
                                    path!("/root/store/clones/thread-alpha/project.git")
                                ),
                            },
                        },
                    },
                },
            }),
        )
        .await;
        let source_path = PathBuf::from(path!("/root/project"));
        let checkout_path = PathBuf::from(path!("/root/store/worktrees/thread-alpha/project"));
        let notes_path = PathBuf::from(path!("/root/notes"));
        fs.set_branch_name(&checkout_path.join(".git"), Some("feature-x"));
        let project = Project::test(
            fs.clone(),
            [source_path.as_path(), notes_path.as_path()],
            cx,
        )
        .await;
        let checkout_project = Project::test(
            fs.clone(),
            [checkout_path.as_path(), notes_path.as_path()],
            cx,
        )
        .await;
        let (worktree_picker, window_handle, checkout_workspace, mut cx) =
            init_worktree_picker_with_open_workspace(project, checkout_project, cx).await;

        assert_eq!(
            picker_rows(&worktree_picker, &mut cx),
            vec![
                "[This Window]".to_string(),
                format!("git: {}", source_path.display()),
                "open: project/thread-alpha • notes (Some(\"feature-x\"))".to_string(),
            ]
        );
        let (project, workspace) = worktree_picker.update(&mut cx, |worktree_picker, cx| {
            let delegate = &worktree_picker.picker.read(cx).delegate;
            (delegate.project.clone(), delegate.workspace.clone())
        });
        let reopened_picker = cx
            .update(|window, cx| cx.new(|cx| WorktreePicker::new(project, workspace, window, cx)));
        assert_eq!(
            picker_rows(&reopened_picker, &mut cx),
            picker_rows(&worktree_picker, &mut cx),
            "a picker opened after loading shows the same rows before any Git request returns"
        );

        set_picker_query(&worktree_picker, "tree", &mut cx);
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::OpenWorkspace { .. }),
            &mut cx,
        );
        fs.set_branch_name(&checkout_path.join(".git"), Some("feature-y"));
        cx.run_until_parked();
        let open_row = "open: project/thread-alpha • notes (Some(\"feature-y\"))".to_string();
        assert_eq!(
            picker_rows(&worktree_picker, &mut cx),
            vec![format!("git: {}", source_path.display()), open_row.clone()]
        );
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut cx),
            Some(open_row.clone())
        );
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::CreateNamed { .. }),
            &mut cx,
        );
        fs.set_branch_name(&checkout_path.join(".git"), Some("feature-z"));
        cx.run_until_parked();
        assert_eq!(
            open_workspace_branches(&worktree_picker, &mut cx),
            vec![Some("feature-z".to_string())]
        );
        fs.set_branch_name(&checkout_path.join(".git"), Some("feature-y"));
        cx.run_until_parked();
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut cx),
            Some("create: named".to_string())
        );
        set_picker_query(&worktree_picker, "tr", &mut cx);
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut cx),
            Some(format!("git: {}", source_path.display()))
        );

        for query in ["alpha", "feature-y"] {
            set_picker_query(&worktree_picker, query, &mut cx);
            assert_eq!(
                picker_rows(&worktree_picker, &mut cx),
                vec![open_row.clone()],
                "query: {query}"
            );
        }

        let source_workspace = window_handle
            .read_with(&cx, |multi_workspace, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.confirm(true, window, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            window_root_paths(&cx),
            vec![
                vec![source_path.clone(), notes_path.clone()],
                vec![checkout_path.clone(), notes_path.clone()]
            ]
        );
        assert_eq!(
            window_handle
                .read_with(&cx, |multi_workspace, _| multi_workspace
                    .workspace()
                    .clone())
                .unwrap(),
            source_workspace
        );

        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            window_handle
                .read_with(&cx, |multi_workspace, _| multi_workspace
                    .workspace()
                    .clone())
                .unwrap(),
            checkout_workspace
        );

        set_picker_query(&worktree_picker, "", &mut cx);
        fs.create_dir(&notes_path.join(".git")).await.unwrap();
        cx.run_until_parked();
        assert_eq!(
            open_workspace_branches(&worktree_picker, &mut cx),
            vec![None]
        );
        fs.remove_dir(
            &notes_path.join(".git"),
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        cx.run_until_parked();
        assert_eq!(
            open_workspace_branches(&worktree_picker, &mut cx),
            vec![Some("feature-y".to_string())]
        );
    }

    #[gpui::test]
    async fn test_open_workspace_is_listed_unless_a_git_row_opens_it(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/root"),
            json!({
                "a": { ".git": {}, "file.txt": "buffer_text" },
                "b": { ".git": {}, "file.txt": "buffer_text" },
                "docs": { "notes.md": "" },
                "worktrees": {},
            }),
        )
        .await;
        let root = Path::new(path!("/root"));
        let [a, b, docs] = ["a", "b", "docs"].map(|name| root.join(name));
        let [linked_a, linked_b] = ["a-1", "b-1"].map(|name| root.join("worktrees").join(name));
        for repository_path in [&a, &b] {
            fs.set_head_for_repo(
                &repository_path.join(".git"),
                &[("file.txt", "buffer_text".to_string())],
                "deadbeef",
            );
        }
        let two_repositories = Project::test(fs.clone(), [a.as_path(), b.as_path()], cx).await;
        cx.executor().run_until_parked();
        for (repository_path, linked_path) in [(&a, &linked_a), (&b, &linked_b)] {
            let repository = two_repositories.read_with(cx, |project, cx| {
                project
                    .repositories(cx)
                    .values()
                    .find(|repository| {
                        repository.read(cx).work_directory_abs_path.as_ref()
                            == repository_path.as_path()
                    })
                    .unwrap()
                    .clone()
            });
            cx.update(|cx| {
                repository.update(cx, |repository, _| {
                    repository.create_worktree(
                        git::repository::CreateWorktreeTarget::NewBranch {
                            branch_name: linked_path
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned(),
                            base_sha: Some("deadbeef".to_string()),
                        },
                        linked_path.clone(),
                    )
                })
            })
            .await
            .unwrap()
            .unwrap();
        }

        for linked_path in [&linked_a, &linked_b] {
            fs.set_branch_name(&linked_path.join(".git"), Some("feature-x"));
        }
        let two_linked_worktrees =
            Project::test(fs.clone(), [linked_a.as_path(), linked_b.as_path()], cx).await;
        let (worktree_picker, window_handle, open_workspace, mut window_cx) =
            init_worktree_picker_with_open_workspace(two_repositories, two_linked_worktrees, cx)
                .await;
        assert_eq!(
            open_workspace_rows(&worktree_picker, &mut window_cx),
            vec![open_workspace.entity_id()]
        );
        assert_eq!(
            open_workspace_branches(&worktree_picker, &mut window_cx),
            vec![None]
        );
        set_picker_query(&worktree_picker, "a", &mut window_cx);
        worktree_picker.update(&mut window_cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker.delegate.selected_index = 2;
            })
        });
        set_picker_query(&worktree_picker, "", &mut window_cx);
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut window_cx),
            Some("[This Window]".to_string())
        );
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::OpenWorkspace { .. }),
            &mut window_cx,
        );
        worktree_picker.update_in(&mut window_cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.confirm(true, window, cx);
            })
        });
        window_cx.run_until_parked();
        assert_eq!(
            window_root_paths(&window_cx),
            vec![
                vec![a.clone(), b.clone()],
                vec![linked_a.clone(), linked_b.clone()]
            ]
        );
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::Worktree { worktree, .. } if worktree.is_main),
            &mut window_cx,
        );
        let main_worktree_row = selected_picker_row(&worktree_picker, &mut window_cx);
        window_handle
            .update(&mut window_cx, |multi_workspace, window, cx| {
                multi_workspace.remove([open_workspace], RemovalIntent::CloseProject, window, cx)
            })
            .unwrap()
            .await
            .unwrap();
        window_cx.run_until_parked();
        let rows_without_open_workspaces = vec![
            format!("git: {}", a.display()),
            format!("git: {}", linked_a.display()),
        ];
        assert_eq!(
            picker_rows(&worktree_picker, &mut window_cx),
            rows_without_open_workspaces
        );
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut window_cx),
            main_worktree_row
        );
        let more_linked_worktrees =
            Project::test(fs.clone(), [linked_a.as_path(), linked_b.as_path()], cx).await;
        more_linked_worktrees
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        cx.executor().run_until_parked();
        let added_workspace = window_handle
            .update(&mut window_cx, |multi_workspace, window, cx| {
                let added_workspace =
                    cx.new(|cx| Workspace::test_new(more_linked_worktrees.clone(), window, cx));
                multi_workspace.add(added_workspace.clone(), window, cx);
                added_workspace
            })
            .unwrap();
        window_cx.run_until_parked();
        assert_eq!(
            open_workspace_rows(&worktree_picker, &mut window_cx),
            vec![added_workspace.entity_id()]
        );
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::OpenWorkspace { .. }),
            &mut window_cx,
        );
        more_linked_worktrees
            .update(cx, |project, cx| {
                project.find_or_create_worktree(&docs, true, cx)
            })
            .await
            .unwrap();
        window_cx.run_until_parked();
        assert_eq!(
            picker_rows(&worktree_picker, &mut window_cx),
            rows_without_open_workspaces
        );
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut window_cx),
            Some(format!("git: {}", linked_a.display()))
        );

        set_picker_query(&worktree_picker, "a-", &mut window_cx);
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::CreateNamed { .. }),
            &mut window_cx,
        );
        let repository_a = window_handle
            .read_with(&window_cx, |multi_workspace, cx| {
                multi_workspace
                    .workspace()
                    .read(cx)
                    .project()
                    .read(cx)
                    .repositories(cx)
                    .values()
                    .find(|repository| {
                        repository.read(cx).work_directory_abs_path.as_ref() == a.as_path()
                    })
                    .unwrap()
                    .clone()
            })
            .unwrap();
        window_cx
            .update(|_, cx| {
                repository_a.update(cx, |repository, _| {
                    repository.create_worktree(
                        git::repository::CreateWorktreeTarget::NewBranch {
                            branch_name: "a-2".to_string(),
                            base_sha: Some("deadbeef".to_string()),
                        },
                        root.join("worktrees").join("a-2"),
                    )
                })
            })
            .await
            .unwrap()
            .unwrap();
        window_cx.run_until_parked();
        let rows = picker_rows(&worktree_picker, &mut window_cx);
        assert_eq!(rows.len(), 2);
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut window_cx),
            rows.first().cloned()
        );

        let with_folder = Project::test(fs.clone(), [a.as_path(), docs.as_path()], cx).await;
        let linked_with_folder =
            Project::test(fs.clone(), [linked_a.as_path(), docs.as_path()], cx).await;
        let (worktree_picker, _window_handle, _open_workspace, mut window_cx) =
            init_worktree_picker_with_open_workspace(with_folder, linked_with_folder, cx).await;
        assert_eq!(
            picker_rows(&worktree_picker, &mut window_cx),
            vec![
                "[This Window]".to_string(),
                format!("git: {}", linked_a.display()),
                format!("git: {}", a.display()),
                format!("git: {}", root.join("worktrees").join("a-2").display()),
            ]
        );
    }

    #[gpui::test]
    async fn test_closed_delta_checkouts_of_the_source_are_listed(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let delta_clone = |branch: &str, source: &str| {
            json!({
                "HEAD": format!("ref: refs/heads/{branch}\n"),
                "config": "[core]\n\tbare = false\n",
                "delta": { "copy-baseline": "" },
                "objects": { "info": { "alternates": format!("{source}\n") } },
            })
        };
        let delta_checkout = |namespace: &str| {
            json!({
                ".git": format!(
                    "gitdir: {}/{namespace}/project.git\n",
                    path!("/root/project/.delta/clones")
                ),
            })
        };
        let source_objects = path!("/root/project/.git/objects");
        let mut gamma_clone = delta_clone("feature-g", source_objects);
        gamma_clone["worktrees"] = json!({
            "epsilon": {
                "HEAD": "ref: refs/heads/feature-e\n",
                "commondir": "../..",
                "gitdir": format!(
                    "{}\n",
                    path!("/root/project/.delta/worktrees/epsilon/project/.git")
                ),
            },
        });
        fs.insert_tree(
            path!("/root"),
            json!({
                "project": {
                    ".git": {
                        "HEAD": "ref: refs/heads/main\n",
                        "info": { "exclude": "/.delta/worktrees/\n/.delta/clones/\n" },
                        "worktrees": {
                            "self": {
                                "HEAD": "ref: refs/heads/main\n",
                                "commondir": "../..",
                                "gitdir": format!("{}\n", path!("/root/project/.git")),
                            },
                        },
                    },
                    ".delta": {
                        "clones": {
                            "alpha": { "project.git": delta_clone("feature-a", source_objects) },
                            "beta": { "project.git": delta_clone("feature-b", source_objects) },
                            "gamma": { "project.git": gamma_clone },
                            "zeta": { "project.git": delta_clone("feature-z", source_objects) },
                            "foreign": {
                                "project.git": delta_clone(
                                    "feature-f",
                                    path!("/root/other/.git/objects"),
                                ),
                            },
                        },
                        "mounted": { "zeta": { "project": delta_checkout("zeta") } },
                        "worktrees": {
                            "alpha": { "project": delta_checkout("alpha") },
                            "beta": { "project": delta_checkout("beta") },
                            "gamma": { "project": delta_checkout("gamma") },
                            "epsilon": {
                                "project": {
                                    ".git": format!(
                                        "gitdir: {}\n",
                                        path!(
                                            "/root/project/.delta/clones/gamma/project.git/worktrees/epsilon"
                                        )
                                    ),
                                },
                            },
                            "foreign": { "project": delta_checkout("foreign") },
                            "plain": { "project": { "file.txt": "" } },
                        },
                    },
                    "file.txt": "buffer_text",
                },
                "other": { ".git": {} },
            }),
        )
        .await;
        fs.create_symlink(
            Path::new(path!("/root/project/.delta/worktrees/zeta")),
            PathBuf::from(path!("/root/project/.delta/mounted/zeta")),
        )
        .await
        .unwrap();
        let alpha_path = PathBuf::from(path!("/root/project/.delta/worktrees/alpha/project"));
        let beta_path = PathBuf::from(path!("/root/project/.delta/worktrees/beta/project"));
        let gamma_path = PathBuf::from(path!("/root/project/.delta/worktrees/gamma/project"));
        fs.set_branch_name(&alpha_path.join(".git"), Some("feature-a"));
        fs.set_branch_name(&beta_path.join(".git"), Some("feature-b"));
        let beta_project = Project::test(fs.clone(), [beta_path.as_path()], cx).await;
        let alpha_project = Project::test(fs.clone(), [alpha_path.as_path()], cx).await;
        let (worktree_picker, window_handle, _, mut cx) =
            init_worktree_picker_with_open_workspace(beta_project, alpha_project, cx).await;

        assert_eq!(
            picker_rows(&worktree_picker, &mut cx),
            vec![
                "[This Window]".to_string(),
                format!("git: {}", beta_path.display()),
                "open: alpha (Some(\"feature-a\"))".to_string(),
                "closed: gamma (Some(\"feature-g\"))".to_string(),
                "closed: gamma/project/epsilon/project (Some(\"feature-e\"))".to_string(),
            ]
        );

        let group_checkouts = worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                std::mem::take(&mut picker.delegate.group_checkouts)
            })
        });
        set_picker_query(&worktree_picker, "feature-g", &mut cx);
        let selected_before_scan = selected_picker_row(&worktree_picker, &mut cx);
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                apply_group_checkouts(picker, group_checkouts, window, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            (
                selected_before_scan,
                selected_picker_row(&worktree_picker, &mut cx)
            ),
            (
                Some("create: named".to_string()),
                Some("closed: gamma (Some(\"feature-g\"))".to_string())
            ),
            "checkouts found after a query was typed take the selection from the create row"
        );

        let group_checkouts = worktree_picker.update(&mut cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                std::mem::take(&mut picker.delegate.group_checkouts)
            })
        });
        set_picker_query(&worktree_picker, "feature", &mut cx);
        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::Worktree { .. }),
            &mut cx,
        );
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                apply_group_checkouts(picker, group_checkouts, window, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            selected_picker_row(&worktree_picker, &mut cx),
            Some(format!("git: {}", beta_path.display())),
            "a row picked with the keyboard stays selected when checkouts are found"
        );

        set_picker_query(&worktree_picker, "feature-e", &mut cx);
        assert_eq!(
            picker_rows(&worktree_picker, &mut cx),
            vec!["closed: gamma/project/epsilon/project (Some(\"feature-e\"))".to_string()]
        );
        let epsilon_path = Path::new(path!("/root/project/.delta/worktrees/epsilon"));
        fs.rename(
            epsilon_path,
            Path::new(path!("/root/project/.delta/worktrees/epsilon-renamed")),
            RenameOptions::default(),
        )
        .await
        .unwrap();
        for secondary in [true, false] {
            worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
                worktree_picker.picker.update(cx, |picker, cx| {
                    picker.delegate.confirm(secondary, window, cx);
                })
            });
            cx.run_until_parked();
        }
        fs.insert_symlink(
            epsilon_path,
            PathBuf::from(path!("/root/project/.delta/worktrees/gamma")),
        )
        .await;
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            })
        });
        cx.run_until_parked();
        assert_eq!(
            window_root_paths(&cx),
            vec![vec![beta_path.clone()]],
            "a checkout renamed or replaced by a symlink after listing opens nothing in its place"
        );

        set_picker_query(&worktree_picker, "feature-g", &mut cx);
        assert_eq!(
            picker_rows(&worktree_picker, &mut cx),
            vec!["closed: gamma (Some(\"feature-g\"))".to_string()]
        );

        let beta_workspace = window_handle
            .read_with(&cx, |multi_workspace, _| {
                multi_workspace.workspace().clone()
            })
            .unwrap();
        for _ in 0..2 {
            worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
                worktree_picker.picker.update(cx, |picker, cx| {
                    picker.delegate.confirm(false, window, cx);
                })
            });
        }
        cx.run_until_parked();
        assert_eq!(window_root_paths(&cx), vec![vec![gamma_path.clone()]]);
        assert_eq!(
            window_handle
                .read_with(&cx, |multi_workspace, cx| multi_workspace
                    .workspaces()
                    .map(|workspace| workspace
                        .read(cx)
                        .root_paths(cx)
                        .iter()
                        .map(|root_path| root_path.to_path_buf())
                        .collect::<Vec<_>>())
                    .collect::<Vec<_>>())
                .unwrap(),
            vec![vec![beta_path], vec![alpha_path], vec![gamma_path]]
        );
        assert_eq!(
            beta_workspace.read_with(&cx, |workspace, _| workspace
                .active_worktree_creation()
                .label
                .clone()),
            None
        );
    }

    #[gpui::test]
    async fn test_opened_checkouts_stay_open_when_ai_is_disabled(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            project::DisableAiSettings::override_global(
                project::DisableAiSettings { disable_ai: true },
                cx,
            )
        });
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let delta_clone = |branch: &str| {
            json!({
                "HEAD": format!("ref: refs/heads/{branch}\n"),
                "config": "[core]\n\tbare = false\n",
                "delta": { "copy-baseline": "" },
                "objects": {
                    "info": {
                        "alternates": format!("{}\n", path!("/root/project/.git/objects")),
                    },
                },
            })
        };
        let delta_checkout = |namespace: &str| {
            json!({
                "project": {
                    ".git": format!(
                        "gitdir: {}/{namespace}/project.git\n",
                        path!("/root/project/.delta/clones")
                    ),
                },
            })
        };
        fs.insert_tree(
            path!("/root/project"),
            json!({
                ".git": { "HEAD": "ref: refs/heads/main\n" },
                ".delta": {
                    "clones": {
                        "alpha": { "project.git": delta_clone("feature-a") },
                        "beta": { "project.git": delta_clone("feature-b") },
                        "gamma": { "project.git": delta_clone("feature-g") },
                    },
                    "worktrees": {
                        "alpha": delta_checkout("alpha"),
                        "beta": delta_checkout("beta"),
                        "gamma": delta_checkout("gamma"),
                    },
                },
            }),
        )
        .await;
        let checkout_path = |namespace: &str| {
            PathBuf::from(path!("/root/project/.delta/worktrees"))
                .join(namespace)
                .join("project")
        };
        let alpha_project = Project::test(fs.clone(), [checkout_path("alpha").as_path()], cx).await;
        alpha_project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(alpha_project, window, cx));
        let mut cx = VisualTestContext::from_window(AnyWindowHandle::from(window_handle), cx);

        for namespace in ["beta", "gamma"] {
            let workspace = window_handle
                .read_with(&cx, |multi_workspace, _| {
                    multi_workspace.workspace().clone()
                })
                .unwrap();
            let project = workspace.read_with(&cx, |workspace, _| workspace.project().clone());
            project
                .update(&mut cx, |project, cx| project.git_scans_complete(cx))
                .await;
            let worktree_picker = cx.update(|window, cx| {
                cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
            });
            cx.run_until_parked();
            select_picker_row(
                &worktree_picker,
                |entry| {
                    matches!(
                        entry,
                        WorktreeEntry::ClosedCheckout { checkout, .. } if checkout.name == namespace
                    )
                },
                &mut cx,
            );
            worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
                worktree_picker.picker.update(cx, |picker, cx| {
                    picker.delegate.confirm(false, window, cx);
                })
            });
            cx.run_until_parked();
        }

        assert_eq!(
            window_handle
                .read_with(&cx, |multi_workspace, cx| {
                    multi_workspace
                        .workspaces()
                        .map(|workspace| workspace.read(cx).root_paths(cx).to_vec())
                        .collect::<Vec<_>>()
                })
                .unwrap(),
            vec![
                vec![Arc::from(checkout_path("beta").as_path())],
                vec![Arc::from(checkout_path("gamma").as_path())],
            ],
            "a checkout opened from the picker is kept when the next one is opened"
        );
    }

    #[gpui::test]
    async fn test_checkouts_of_a_dot_named_git_dir_source_are_listed_from_subfolders(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let delta_clone = |branch: &str| {
            json!({
                "HEAD": format!("ref: refs/heads/{branch}\n"),
                "config": "[core]\n\tbare = false\n",
                "delta": { "copy-baseline": "" },
                "objects": {
                    "info": {
                        "alternates": format!("{}\n", path!("/root/project/.metadata/objects")),
                    },
                },
            })
        };
        let mut rows_by_opened_folder = Vec::new();
        for opened_folder in [
            path!("/root/checkout/src"),
            path!("/root/project/src"),
            path!("/root/project/.metadata/.delta/worktrees/sibling/project/src"),
        ] {
            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                path!("/root"),
                json!({
                    "project": {
                        ".git": format!("gitdir: {}\n", path!("/root/project/.metadata")),
                        ".metadata": {
                            "HEAD": "ref: refs/heads/main\n",
                            ".delta": {
                                "clones": { "sibling": { "project.git": delta_clone(".invalid") } },
                                "worktrees": {
                                    "sibling": {
                                        "project": {
                                            ".git": format!(
                                                "gitdir: {}\n",
                                                path!(
                                                    "/root/project/.metadata/.delta/clones/sibling/project.git"
                                                )
                                            ),
                                            "src": {},
                                        },
                                    },
                                },
                            },
                        },
                        "src": {},
                    },
                    "clones": { "project.git": delta_clone("feature") },
                    "checkout": {
                        ".git": format!("gitdir: {}\n", path!("/root/clones/project.git")),
                        "src": {},
                    },
                }),
            )
            .await;
            let project = Project::test(fs.clone(), [Path::new(opened_folder)], cx).await;
            project
                .update(cx, |project, cx| project.git_scans_complete(cx))
                .await;
            let window_handle =
                cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
            let workspace = window_handle
                .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
                .unwrap();
            let mut visual_cx =
                VisualTestContext::from_window(AnyWindowHandle::from(window_handle), cx);
            let worktree_picker = visual_cx.update(|window, cx| {
                cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
            });
            visual_cx.run_until_parked();
            rows_by_opened_folder.push(picker_rows(&worktree_picker, &mut visual_cx));
        }

        let sibling_row = "closed: sibling (None)".to_string();
        assert_eq!(
            rows_by_opened_folder,
            vec![
                vec![
                    format!("git: {}", path!("/root/checkout")),
                    sibling_row.clone()
                ],
                vec![format!("git: {}", path!("/root/project")), sibling_row],
                vec![format!(
                    "git: {}",
                    path!("/root/project/.metadata/.delta/worktrees/sibling/project")
                )],
            ]
        );
    }

    #[gpui::test]
    async fn test_source_row_is_a_working_checkout_of_the_source(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let git_dir = |config: &str, extra_files: serde_json::Value| {
            let mut git_dir = json!({
                "HEAD": "ref: refs/heads/main\n",
                "objects": null,
                "refs": null,
                "config": config,
            });
            if let (Some(git_dir), serde_json::Value::Object(extra_files)) =
                (git_dir.as_object_mut(), extra_files)
            {
                git_dir.extend(extra_files);
            }
            git_dir
        };
        let dot_bare_layouts = [
            (
                "bare",
                "[core]\n\trepositoryformatversion = 0\n\tbare = true\n",
                false,
            ),
            (
                "non_bare",
                "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
                true,
            ),
        ];
        let mut layout = dot_bare_layouts
            .iter()
            .map(|(name, config, _)| {
                (
                    name.to_string(),
                    json!({
                        ".git": "gitdir: .bare\n",
                        ".bare": git_dir(config, json!({})),
                    }),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        let plain_config = "[core]\n\trepositoryformatversion = 0\n\tbare = false\n";
        layout.insert(
            "storage".to_string(),
            json!({
                "absolute.git": git_dir(plain_config, json!({})),
                "relative.git": git_dir(
                    "[core]\n\trepositoryformatversion = 1\n\tbare = false\n[extensions]\n\tworktreeConfig = true\n",
                    json!({ "config.worktree": "[core]\n\tworktree = ../../relative\n" }),
                ),
                "included.git": git_dir(
                    &format!("{plain_config}[include]\n\tpath = extra\n"),
                    json!({ "extra": "[core]\n\tworktree = ../../included\n" }),
                ),
                "linked.git": git_dir(
                    plain_config,
                    json!({
                        "worktrees": {
                            "linked": {
                                "HEAD": "ref: refs/heads/linked\n",
                                "commondir": "../..",
                                "gitdir": "",
                            },
                        },
                    }),
                ),
            }),
        );
        for checkout in ["absolute", "relative", "included", "linked"] {
            layout.insert(checkout.to_string(), json!({ ".git": "" }));
        }
        layout.insert(
            "unrelated".to_string(),
            json!({ ".git": {}, ".metadata": git_dir(plain_config, json!({})) }),
        );
        layout.insert(
            "unversioned_subdirectory_worktree".to_string(),
            json!({
                ".git": "gitdir: .bare\n",
                ".bare": git_dir("[core]\n\tbare = false\n\tworktree = ../sub\n", json!({})),
                "sub": {},
            }),
        );
        layout.insert(
            "outer".to_string(),
            json!({
                ".git": {},
                "damaged": { ".git": { "HEAD": "damaged\n" } },
            }),
        );
        let tree = util::test::TempTree::new(serde_json::Value::Object(layout));
        let fs = fs::RealFs::new(None, cx.executor());
        let root = SanitizedPath::new(&fs.canonicalize(tree.path()).await.unwrap()).to_path_buf();
        let storage = root.join("storage");
        let config_path = |path: &Path| {
            path.to_string_lossy()
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
        };
        std::fs::write(
            storage.join("absolute.git").join("config"),
            format!(
                "{plain_config}\tworktree = \"{}\"\n",
                config_path(&root.join("absolute"))
            ),
        )
        .unwrap();
        std::fs::write(
            storage.join("linked.git").join("config"),
            format!(
                "{plain_config}\tworktree = \"{}\"\n",
                config_path(&root.join("linked"))
            ),
        )
        .unwrap();
        let linked_admin_dir = storage.join("linked.git").join("worktrees").join("linked");
        std::fs::write(
            linked_admin_dir.join("gitdir"),
            format!("{}\n", root.join("linked").join(".git").display()),
        )
        .unwrap();
        for (checkout, git_dir) in [
            ("absolute", storage.join("absolute.git")),
            ("relative", storage.join("relative.git")),
            ("included", storage.join("included.git")),
            ("linked", linked_admin_dir),
        ] {
            std::fs::write(
                root.join(checkout).join(".git"),
                format!("gitdir: {}\n", git_dir.display()),
            )
            .unwrap();
        }

        let mut cases = dot_bare_layouts
            .iter()
            .map(|(name, _, is_working_checkout)| {
                (
                    root.join(name).join(".bare"),
                    is_working_checkout.then(|| root.join(name)),
                )
            })
            .collect::<Vec<_>>();
        cases.push((storage.join("absolute.git"), Some(root.join("absolute"))));
        cases.push((storage.join("relative.git"), Some(root.join("relative"))));
        cases.push((storage.join("included.git"), Some(root.join("included"))));
        cases.push((storage.join("linked.git"), None));
        cases.push((root.join("unrelated").join(".metadata"), None));
        cases.push((root.join("outer").join("damaged").join(".git"), None));
        cases.push((
            root.join("unversioned_subdirectory_worktree").join(".bare"),
            Some(root.join("unversioned_subdirectory_worktree")),
        ));
        #[cfg(unix)]
        {
            let symlinked = root.join("symlinked");
            std::fs::create_dir_all(symlinked.join(".metadata").join("objects")).unwrap();
            std::fs::create_dir_all(symlinked.join(".metadata").join("refs")).unwrap();
            std::fs::write(
                symlinked.join(".metadata").join("HEAD"),
                "ref: refs/heads/main\n",
            )
            .unwrap();
            std::fs::write(symlinked.join(".metadata").join("config"), plain_config).unwrap();
            std::os::unix::fs::symlink(".metadata", symlinked.join(".git")).unwrap();
            cases.push((symlinked.join(".metadata"), Some(symlinked)));

            let line_break = root.join("line\nbreak");
            let line_break_git_dir = storage.join("line_break.git");
            std::fs::create_dir_all(&line_break).unwrap();
            std::fs::create_dir_all(line_break_git_dir.join("objects")).unwrap();
            std::fs::create_dir_all(line_break_git_dir.join("refs")).unwrap();
            std::fs::write(line_break_git_dir.join("HEAD"), "ref: refs/heads/main\n").unwrap();
            std::fs::write(
                line_break_git_dir.join("config"),
                format!(
                    "{plain_config}\tworktree = \"{}\"\n",
                    config_path(&line_break)
                ),
            )
            .unwrap();
            std::fs::write(
                line_break.join(".git"),
                format!("gitdir: {}\n", line_break_git_dir.display()),
            )
            .unwrap();
            cases.push((line_break_git_dir, Some(line_break)));
        }
        let mut source_rows = Vec::new();
        for (source_common_dir, _) in &cases {
            let checkouts = load_group_checkouts(source_common_dir, true, fs.clone(), cx).await;
            source_rows.push(
                checkouts
                    .into_iter()
                    .find(|checkout| checkout.name == "main worktree")
                    .map(|checkout| checkout.path),
            );
        }
        assert_eq!(
            source_rows,
            cases
                .into_iter()
                .map(|(_, expected_row)| expected_row)
                .collect::<Vec<_>>()
        );
    }

    #[gpui::test]
    async fn test_checkouts_in_managed_stores_are_listed(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let source = || {
            json!({
                "HEAD": "ref: refs/heads/main\n",
                "config": "[core]\n\tbare = true\n",
                "objects": {},
            })
        };
        let delta_clone = |source_common_dir: &str| {
            json!({
                "HEAD": "ref: refs/heads/feature\n",
                "config": "[core]\n\tbare = false\n",
                "delta": { "copy-baseline": "" },
                "objects": {
                    "info": { "alternates": format!("{source_common_dir}/objects\n") },
                },
            })
        };
        let checkout = |clone_path: &str| json!({ ".git": format!("gitdir: {clone_path}\n") });
        let account_source =
            path!("/base/user_01g3vqv3ejt1em7rjrbwde86vm/managed-repositories/id/project.git");
        let store_source = path!("/other/store/managed-repositories/id/project.git");
        fs.insert_tree(
            path!("/base"),
            json!({
                "user_01g3vqv3ejt1em7rjrbwde86vm": {
                    "managed-repositories": {
                        "id": {
                            "project.git": source(),
                            "clones": { "legacy": { "project.git": delta_clone(account_source) } },
                            "worktrees": {
                                "legacy": {
                                    "project": checkout(path!(
                                        "/base/user_01g3vqv3ejt1em7rjrbwde86vm/managed-repositories/id/clones/legacy/project.git"
                                    )),
                                },
                            },
                        },
                    },
                },
                "worktrees": {
                    "allocation": {
                        "project.git": delta_clone(account_source),
                        "project": checkout(path!("/base/worktrees/allocation/project.git")),
                    },
                },
            }),
        )
        .await;
        fs.insert_tree(
            path!("/base/user_01g3vqv3ejt1em7rjrbwde86vm/worktrees"),
            json!({
                "custom": {
                    "project.git": delta_clone(account_source),
                    "project": checkout(path!(
                        "/base/user_01g3vqv3ejt1em7rjrbwde86vm/worktrees/custom/project.git"
                    )),
                },
            }),
        )
        .await;
        let bare_root_source = path!("/legacy/store/managed-repositories/id");
        fs.insert_tree(
            path!("/legacy/store"),
            json!({
                "managed-repositories": {
                    "id": {
                        "HEAD": "ref: refs/heads/main\n",
                        "config": "[core]\n\tbare = true\n",
                        "objects": {},
                        "clones": { "thread": { "project.git": delta_clone(bare_root_source) } },
                        "worktrees": {
                            "thread": {
                                "project": checkout(path!(
                                    "/legacy/store/managed-repositories/id/clones/thread/project.git"
                                )),
                            },
                        },
                    },
                },
                "worktrees": {
                    "allocation": {
                        "project.git": delta_clone(bare_root_source),
                        "project": checkout(path!("/legacy/store/worktrees/allocation/project.git")),
                    },
                },
            }),
        )
        .await;
        fs.insert_tree(
            path!("/other/store"),
            json!({ "managed-repositories": { "id": { "project.git": source() } } }),
        )
        .await;
        fs.insert_tree(
            path!("/ssd/checkouts"),
            json!({
                "8p2k4m6q9r1s": {
                    "project.git": delta_clone(store_source),
                    "project": checkout(path!("/other/store/worktrees/8p2k4m6q9r1s/project.git")),
                },
                "allocation": {
                    "project.git": delta_clone(store_source),
                    "project": checkout(path!("/other/store/worktrees/allocation/project.git")),
                },
            }),
        )
        .await;
        fs.insert_symlink(
            path!("/other/store/worktrees"),
            PathBuf::from(path!("/ssd/checkouts")),
        )
        .await;

        fs.insert_symlink(path!("/alias"), PathBuf::from(path!("/other")))
            .await;
        let aliased_store_source = path!("/alias/store/managed-repositories/id/project.git");

        let mut checkouts_by_source = Vec::new();
        for source_common_dir in [
            account_source,
            store_source,
            aliased_store_source,
            bare_root_source,
        ] {
            checkouts_by_source.push(
                load_group_checkouts(Path::new(source_common_dir), true, fs.clone(), cx)
                    .await
                    .into_iter()
                    .map(|checkout| checkout.path)
                    .collect::<Vec<_>>(),
            );
        }
        assert_eq!(
            checkouts_by_source,
            vec![
                vec![
                    PathBuf::from(path!("/base/worktrees/allocation/project")),
                    PathBuf::from(path!(
                        "/base/user_01g3vqv3ejt1em7rjrbwde86vm/worktrees/custom/project"
                    )),
                    PathBuf::from(path!(
                        "/base/user_01g3vqv3ejt1em7rjrbwde86vm/managed-repositories/id/worktrees/legacy/project"
                    )),
                ],
                vec![PathBuf::from(path!("/ssd/checkouts/8p2k4m6q9r1s/project"))],
                vec![PathBuf::from(path!("/ssd/checkouts/8p2k4m6q9r1s/project"))],
                vec![
                    PathBuf::from(path!("/legacy/store/worktrees/allocation/project")),
                    PathBuf::from(path!(
                        "/legacy/store/managed-repositories/id/worktrees/thread/project"
                    )),
                ],
            ]
        );
    }

    #[gpui::test]
    async fn test_registered_worktrees_of_the_source_and_its_clones_are_listed(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let registry_entry = |branch: &str, gitdir: &str| {
            json!({
                "HEAD": format!("ref: refs/heads/{branch}\n"),
                "commondir": "../..",
                "gitdir": format!("{gitdir}\n"),
            })
        };
        fs.insert_tree(
            path!("/code"),
            json!({
                "project.git": {
                    "HEAD": "ref: refs/heads/main\n",
                    "config": "[core]\n\tbare = true\n",
                    "objects": {},
                    "worktrees": {
                        "main": registry_entry("main", path!("/code/project/.git")),
                        "rel": registry_entry("rel", "../../../nested/rel/.git"),
                        "stale": registry_entry("stale", path!("/code/stale/.git")),
                        "foreign": registry_entry("foreign", path!("/code/other/.git")),
                    },
                    ".delta": {
                        "clones": {
                            "alpha": {
                                "project.git": {
                                    "HEAD": "ref: refs/heads/feature-a\n",
                                    "config": "[core]\n\tbare = false\n",
                                    "delta": { "copy-baseline": "" },
                                    "objects": {
                                        "info": {
                                            "alternates": format!(
                                                "{}\n",
                                                path!("/code/project.git/objects")
                                            ),
                                        },
                                    },
                                    "worktrees": {
                                        "beta": registry_entry(
                                            "feature-b",
                                            path!("/code/project.git/.delta/worktrees/beta/project/.git"),
                                        ),
                                        "feature": registry_entry(
                                            "feature-f",
                                            path!(
                                                "/code/project.git/.delta/worktrees/alpha/worktrees/project/feature/project/.git"
                                            ),
                                        ),
                                    },
                                },
                            },
                        },
                        "worktrees": {
                            "beta": {
                                "project": {
                                    ".git": format!(
                                        "gitdir: {}\n",
                                        path!(
                                            "/code/project.git/.delta/clones/alpha/project.git/worktrees/beta"
                                        )
                                    ),
                                },
                            },
                            "alpha": {
                                "project": {
                                    ".git": format!(
                                        "gitdir: {}\n",
                                        path!("/code/project.git/.delta/clones/alpha/project.git")
                                    ),
                                },
                                "worktrees": {
                                    "project": {
                                        "feature": {
                                            "project": {
                                                ".git": format!(
                                                    "gitdir: {}\n",
                                                    path!(
                                                        "/code/project.git/.delta/clones/alpha/project.git/worktrees/feature"
                                                    )
                                                ),
                                            },
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
                "project": {
                    ".git": format!("gitdir: {}\n", path!("/code/project.git/worktrees/main")),
                },
                "nested": {
                    "rel": { ".git": "gitdir: ../../project.git/worktrees/rel\n" },
                },
                "other": { ".git": { "HEAD": "ref: refs/heads/foreign\n" } },
            }),
        )
        .await;

        let checkouts =
            load_group_checkouts(Path::new(path!("/code/project.git")), true, fs.clone(), cx)
                .await
                .into_iter()
                .map(|checkout| (checkout.name, checkout.path, checkout.branch))
                .collect::<Vec<_>>();
        let row = |name: &str, path: &str, branch: &str| {
            (
                name.to_string(),
                PathBuf::from(path),
                Some(branch.to_string()),
            )
        };
        assert_eq!(
            checkouts,
            vec![
                row(
                    "alpha",
                    path!("/code/project.git/.delta/worktrees/alpha/project"),
                    "feature-a"
                ),
                row(
                    "alpha/project/beta/project",
                    path!("/code/project.git/.delta/worktrees/beta/project"),
                    "feature-b"
                ),
                row(
                    "alpha/project/feature/project",
                    path!(
                        "/code/project.git/.delta/worktrees/alpha/worktrees/project/feature/project"
                    ),
                    "feature-f"
                ),
                row("project", path!("/code/project"), "main"),
                row("rel", path!("/code/nested/rel"), "rel"),
            ]
        );

        fs.insert_symlink(path!("/link"), PathBuf::from(path!("/code")))
            .await;
        let mut listed_checkouts = Vec::new();
        for listed_common_dir in [
            path!("/link/project.git"),
            path!("/code/project.git/.delta/clones/alpha/project.git"),
        ] {
            let fs = fs.clone();
            listed_checkouts.push(
                cx.background_spawn(async move {
                    load_delta_checkouts(
                        Path::new(path!("/code/project.git")),
                        Some(Path::new(listed_common_dir)),
                        None,
                        fs,
                        PathStyle::local(),
                    )
                    .await
                })
                .await
                .into_iter()
                .map(|checkout| checkout.name)
                .collect::<Vec<_>>(),
            );
        }
        assert_eq!(
            listed_checkouts,
            vec![
                vec![
                    "alpha".to_string(),
                    "alpha/project/beta/project".to_string(),
                    "alpha/project/feature/project".to_string()
                ],
                vec![
                    "alpha".to_string(),
                    "project".to_string(),
                    "rel".to_string()
                ],
            ],
            "worktrees that the active repository's own list shows are left out"
        );
    }

    #[gpui::test]
    async fn test_source_row_is_listed_and_opened_with_real_git(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let (_tree, fs, source_path) = real_git_delta_source(cx).await;
        let checkout_path = source_path
            .join(".delta")
            .join("worktrees")
            .join("alpha")
            .join("project");
        for foreign_path in [
            checkout_path.with_file_name("project~"),
            source_path.join(".delta/worktrees/gamma/project"),
        ] {
            std::fs::create_dir_all(&foreign_path).unwrap();
            std::fs::write(
                foreign_path.join(".git"),
                format!(
                    "gitdir: {}\n",
                    source_path
                        .join(".delta/clones/alpha/project.git")
                        .display()
                ),
            )
            .unwrap();
        }
        let (worktree_picker, mut cx) = open_real_git_worktree_picker(fs, &checkout_path, cx).await;
        let shown_at_open = shown_git_state(&worktree_picker, &mut cx);
        wait_for_git_refresh(&worktree_picker, &mut cx).await;
        assert_eq!(shown_at_open, shown_git_state(&worktree_picker, &mut cx));
        let expected_rows = vec![
            format!("git: {}", checkout_path.display()),
            "closed: main worktree (Some(\"main\"))".to_string(),
            "closed: beta (Some(\"topic\"))".to_string(),
        ];
        for _ in 0..400 {
            cx.run_until_parked();
            if picker_rows(&worktree_picker, &mut cx) == expected_rows {
                break;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(50))
                .await;
        }
        assert_eq!(picker_rows(&worktree_picker, &mut cx), expected_rows);

        select_picker_row(
            &worktree_picker,
            |entry| matches!(entry, WorktreeEntry::ClosedCheckout { .. }),
            &mut cx,
        );
        worktree_picker.update_in(&mut cx, |worktree_picker, window, cx| {
            worktree_picker.picker.update(cx, |picker, cx| {
                picker.delegate.confirm(false, window, cx);
            })
        });
        for _ in 0..400 {
            cx.run_until_parked();
            if window_root_paths(&cx) == vec![vec![source_path.clone()]] {
                break;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(50))
                .await;
        }
        assert_eq!(window_root_paths(&cx), vec![vec![source_path]]);
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn test_closed_checkouts_wait_briefly_for_a_blocked_source_row(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let (_tree, fs, source_path) = real_git_delta_source(cx).await;
        #[allow(clippy::disallowed_methods)]
        let mkfifo = std::process::Command::new("mkfifo")
            .arg(source_path.join(".git/included"))
            .status()
            .unwrap();
        assert!(mkfifo.success());
        fs.git_command(
            Path::new("git"),
            &source_path,
            ["config", "include.path", "included"]
                .map(String::from)
                .to_vec(),
        )
        .await
        .unwrap();
        let checkout_path = source_path.join(".delta/worktrees/alpha/project");
        let (worktree_picker, mut cx) = open_real_git_worktree_picker(fs, &checkout_path, cx).await;

        let expected_rows = vec![
            format!("git: {}", checkout_path.display()),
            "closed: beta (Some(\"topic\"))".to_string(),
        ];
        let started = cx.background_executor.now();
        for _ in 0..200 {
            cx.run_until_parked();
            if picker_rows(&worktree_picker, &mut cx) == expected_rows {
                break;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(50))
                .await;
        }
        let elapsed = cx.background_executor.now() - started;
        let arrival = if elapsed < SOURCE_CHECKOUT_WAIT {
            "before the source row wait ends"
        } else if elapsed < Duration::from_secs(5) {
            "after the source row wait, before git gives up"
        } else {
            "after git gives up"
        };
        assert_eq!(
            (picker_rows(&worktree_picker, &mut cx), arrival),
            (
                expected_rows,
                "after the source row wait, before git gives up"
            ),
            "after {elapsed:?}"
        );
    }

    #[gpui::test]
    async fn test_picker_opens_with_the_rows_git_reports(cx: &mut TestAppContext) {
        let (fs, _source_picker, _repository, worktree_path, _source_cx) =
            init_worktree_picker_test(cx).await;
        fs.set_branch_name(&worktree_path.join(".git"), Some("dirty-wt"));
        let linked_project = Project::test(fs.clone(), [worktree_path.as_path()], cx).await;
        linked_project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let window_handle = cx
            .add_window(|window, cx| MultiWorkspace::test_new(linked_project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(AnyWindowHandle::from(window_handle), cx);
        let open_picker = |cx: &mut VisualTestContext| {
            let project = linked_project.clone();
            let workspace = workspace.downgrade();
            cx.update(|window, cx| cx.new(|cx| WorktreePicker::new(project, workspace, window, cx)))
        };
        let worktree_picker = open_picker(&mut cx);
        let shown_at_open = shown_git_state(&worktree_picker, &mut cx);
        wait_for_git_refresh(&worktree_picker, &mut cx).await;

        assert_eq!(
            (shown_at_open.0.clone(), shown_at_open.1.clone()),
            (
                Some("create: default branch".to_string()),
                vec![
                    format!("git: {}", worktree_path.display()),
                    format!("git: {}", path!("/root/project")),
                ]
            )
        );
        assert_eq!(shown_at_open, shown_git_state(&worktree_picker, &mut cx));

        fs.pause_events();
        fs.insert_tree(
            path!("/root/project/.git/worktrees/fresh-wt"),
            json!({
                "HEAD": "ref: refs/heads/fresh-wt\n",
                "gitdir": format!("{}\n", path!("/root/worktrees/fresh-wt/.git")),
            }),
        )
        .await;
        let stale_picker = open_picker(&mut cx);
        let rows_at_open = shown_git_state(&stale_picker, &mut cx).1;
        wait_for_git_refresh(&stale_picker, &mut cx).await;
        assert_eq!(
            (rows_at_open, shown_git_state(&stale_picker, &mut cx).1),
            (
                vec![
                    format!("git: {}", worktree_path.display()),
                    format!("git: {}", path!("/root/project")),
                ],
                vec![
                    format!("git: {}", worktree_path.display()),
                    format!("git: {}", path!("/root/worktrees/fresh-wt")),
                    format!("git: {}", path!("/root/project")),
                ]
            )
        );
    }

    #[gpui::test]
    async fn test_reftable_checkout_branch_is_read_through_git(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let tree = util::test::TempTree::new(json!({ "source": { ".git": {} } }));
        let fs = fs::RealFs::new(None, cx.executor());
        let root = SanitizedPath::new(&fs.canonicalize(tree.path()).await.unwrap()).to_path_buf();
        let source = root.join("source");
        let clone_path = source.join(".delta/clones/thread/source.git");
        let checkout_path = source.join(".delta/worktrees/thread/source");
        let supports_reftable = fs
            .git_command(
                Path::new("git"),
                &root,
                vec![
                    "init".to_string(),
                    "--quiet".to_string(),
                    "--ref-format=reftable".to_string(),
                    root.join("probe").display().to_string(),
                ],
            )
            .await
            .is_ok();
        if !supports_reftable {
            return;
        }
        std::fs::create_dir_all(clone_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(checkout_path.parent().unwrap()).unwrap();
        fs.git_command(
            Path::new("git"),
            &root,
            vec![
                "init".to_string(),
                "--quiet".to_string(),
                "--initial-branch=feature".to_string(),
                "--ref-format=reftable".to_string(),
                format!("--separate-git-dir={}", clone_path.display()),
                checkout_path.display().to_string(),
            ],
        )
        .await
        .unwrap();
        std::fs::create_dir_all(clone_path.join("delta")).unwrap();
        std::fs::write(clone_path.join("delta/copy-baseline"), "").unwrap();
        std::fs::create_dir_all(clone_path.join("objects/info")).unwrap();
        std::fs::write(
            clone_path.join("objects/info/alternates"),
            format!("{}\n", source.join(".git/objects").display()),
        )
        .unwrap();

        let checkouts = load_group_checkouts(&source.join(".git"), false, fs, cx).await;
        assert_eq!(
            checkouts
                .iter()
                .map(|checkout| (checkout.path.clone(), checkout.branch.clone()))
                .collect::<Vec<_>>(),
            vec![(checkout_path, Some("feature".to_string()))]
        );
    }

    #[cfg(unix)]
    #[gpui::test]
    async fn test_special_git_entries_do_not_block_discovery(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let tree = util::test::TempTree::new(json!({
            "source": {
                ".git": {},
                ".delta": {
                    "clones": {
                        "healthy": {
                            "source.git": {
                                "HEAD": "ref: refs/heads/feature\n",
                                "config": "[core]\n\tbare = false\n",
                                "delta": { "copy-baseline": "" },
                                "objects": { "info": null },
                                "refs": null,
                            },
                        },
                    },
                    "worktrees": {
                        "healthy": { "source": { ".git": "" } },
                        "special": { "source": null },
                        "special_commondir": { "source": { ".git": "" } },
                    },
                },
            },
            "special_git_dir": { "HEAD": "ref: refs/heads/main\n", "objects": null, "refs": null },
        }));
        let fs = fs::RealFs::new(None, cx.executor());
        let root = fs.canonicalize(tree.path()).await.unwrap();
        let source = root.join("source");
        let clone_path = source.join(".delta/clones/healthy/source.git");
        let checkout_path = source.join(".delta/worktrees/healthy/source");
        std::fs::write(
            clone_path.join("objects/info/alternates"),
            format!("{}\n", source.join(".git/objects").display()),
        )
        .unwrap();
        std::fs::write(
            checkout_path.join(".git"),
            format!("gitdir: {}\n", clone_path.display()),
        )
        .unwrap();
        std::fs::write(
            source.join(".delta/worktrees/special_commondir/source/.git"),
            format!("gitdir: {}\n", root.join("special_git_dir").display()),
        )
        .unwrap();
        for fifo_path in [
            source.join(".delta/worktrees/special/source/.git"),
            root.join("special_git_dir/commondir"),
        ] {
            #[allow(clippy::disallowed_methods)]
            let mkfifo = std::process::Command::new("mkfifo")
                .arg(fifo_path)
                .status()
                .unwrap();
            assert!(mkfifo.success());
        }

        let checkouts = load_group_checkouts(&source.join(".git"), false, fs, cx).await;
        assert_eq!(
            checkouts
                .iter()
                .map(|checkout| checkout.path.clone())
                .collect::<Vec<_>>(),
            vec![checkout_path]
        );
    }

    async fn init_worktree_picker_with_open_workspace(
        project: Entity<Project>,
        open_project: Entity<Project>,
        cx: &mut TestAppContext,
    ) -> (
        Entity<WorktreePicker>,
        gpui::WindowHandle<MultiWorkspace>,
        Entity<Workspace>,
        VisualTestContext,
    ) {
        for project in [&project, &open_project] {
            project
                .update(cx, |project, cx| project.git_scans_complete(cx))
                .await;
        }
        cx.executor().run_until_parked();
        assert_eq!(
            project.read_with(cx, |project, cx| project.project_group_key(cx)),
            open_project.read_with(cx, |project, cx| project.project_group_key(cx)),
        );

        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let open_workspace = window_handle
            .update(cx, |multi_workspace, window, cx| {
                let open_workspace =
                    cx.new(|cx| Workspace::test_new(open_project.clone(), window, cx));
                multi_workspace.add(open_workspace.clone(), window, cx);
                open_workspace
            })
            .unwrap();

        let mut cx = VisualTestContext::from_window(window_handle.into(), cx);
        let worktree_picker = cx.update(|window, cx| {
            cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
        });
        cx.run_until_parked();
        (worktree_picker, window_handle, open_workspace, cx)
    }

    fn window_root_paths(cx: &VisualTestContext) -> Vec<Vec<PathBuf>> {
        cx.read(|cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<MultiWorkspace>())
                .map(|window| {
                    window
                        .read(cx)
                        .unwrap()
                        .workspace()
                        .read(cx)
                        .root_paths(cx)
                        .iter()
                        .map(|root_path| root_path.to_path_buf())
                        .collect()
                })
                .collect()
        })
    }

    fn set_picker_query(
        worktree_picker: &Entity<WorktreePicker>,
        query: &str,
        cx: &mut VisualTestContext,
    ) {
        worktree_picker.update_in(cx, |worktree_picker, window, cx| {
            worktree_picker
                .picker
                .update(cx, |picker, cx| picker.set_query(query, window, cx))
        });
        cx.run_until_parked();
    }

    fn select_picker_row(
        worktree_picker: &Entity<WorktreePicker>,
        is_row: impl Fn(&WorktreeEntry) -> bool,
        cx: &mut VisualTestContext,
    ) {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker.delegate.selected_index =
                    picker.delegate.matches.iter().position(is_row).unwrap();
            })
        });
    }

    fn open_workspace_branches(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> Vec<Option<String>> {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .iter()
                    .filter_map(|entry| match entry {
                        WorktreeEntry::OpenWorkspace { open_workspace, .. } => {
                            Some(open_workspace.branch.clone())
                        }
                        _ => None,
                    })
                    .collect()
            })
        })
    }

    fn open_workspace_rows(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> Vec<EntityId> {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .iter()
                    .filter_map(|entry| match entry {
                        WorktreeEntry::OpenWorkspace { open_workspace, .. } => {
                            Some(open_workspace.workspace.entity_id())
                        }
                        _ => None,
                    })
                    .collect()
            })
        })
    }

    fn picker_rows(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> Vec<String> {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .iter()
                    .filter_map(picker_row_text)
                    .collect()
            })
        })
    }

    fn selected_picker_row(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> Option<String> {
        worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker.picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .matches
                    .get(picker.delegate.selected_index)
                    .and_then(|entry| match entry {
                        WorktreeEntry::CreateFromCurrentBranch => {
                            Some("create: current branch".to_string())
                        }
                        WorktreeEntry::CreateFromDefaultBranch { .. } => {
                            Some("create: default branch".to_string())
                        }
                        WorktreeEntry::CreateNamed { .. } => Some("create: named".to_string()),
                        entry => picker_row_text(entry),
                    })
            })
        })
    }

    async fn wait_for_git_refresh(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) {
        let repository = worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker
                .picker
                .read(cx)
                .delegate
                .project
                .read(cx)
                .active_repository(cx)
                .unwrap()
        });
        cx.update(|_, cx| repository.update(cx, |repository, _| repository.default_branch(true)))
            .await
            .unwrap()
            .unwrap();
        cx.run_until_parked();
    }

    fn shown_git_state(
        worktree_picker: &Entity<WorktreePicker>,
        cx: &mut VisualTestContext,
    ) -> (Option<String>, Vec<String>, Vec<(PathBuf, String, bool)>) {
        let git_rows = picker_rows(worktree_picker, cx)
            .into_iter()
            .filter(|row| row.starts_with("git: "))
            .collect();
        let mut worktrees = worktree_picker.update(cx, |worktree_picker, cx| {
            worktree_picker
                .picker
                .read(cx)
                .delegate
                .all_worktrees
                .iter()
                .map(|worktree| {
                    (
                        worktree.path.clone(),
                        worktree.display_name().to_string(),
                        worktree.is_main,
                    )
                })
                .collect::<Vec<_>>()
        });
        worktrees.sort();
        (
            selected_picker_row(worktree_picker, cx),
            git_rows,
            worktrees,
        )
    }

    fn picker_row_text(entry: &WorktreeEntry) -> Option<String> {
        match entry {
            WorktreeEntry::SectionHeader(label) => Some(format!("[{label}]")),
            WorktreeEntry::Worktree { worktree, .. } => {
                Some(format!("git: {}", worktree.path.display()))
            }
            WorktreeEntry::OpenWorkspace { open_workspace, .. } => Some(format!(
                "open: {} ({:?})",
                open_workspace.name, open_workspace.branch
            )),
            WorktreeEntry::ClosedCheckout { checkout, .. } => {
                Some(format!("closed: {} ({:?})", checkout.name, checkout.branch))
            }
            _ => None,
        }
    }

    async fn load_group_checkouts(
        source_common_dir: &Path,
        include_source: bool,
        fs: Arc<dyn Fs>,
        cx: &TestAppContext,
    ) -> Vec<GroupCheckout> {
        let source_common_dir = source_common_dir.to_path_buf();
        cx.background_spawn(async move {
            let git_binary_path = PathBuf::from("git");
            let source_checkout = if include_source {
                load_source_checkout(&source_common_dir, &git_binary_path, fs.as_ref()).await
            } else {
                None
            };
            source_checkout
                .into_iter()
                .chain(
                    load_delta_checkouts(
                        &source_common_dir,
                        None,
                        Some(git_binary_path),
                        fs,
                        PathStyle::local(),
                    )
                    .await,
                )
                .collect()
        })
        .await
    }

    async fn real_git_delta_source(
        cx: &mut TestAppContext,
    ) -> (util::test::TempTree, Arc<dyn Fs>, PathBuf) {
        let tree = util::test::TempTree::new(json!({
            "project": {
                ".git": {},
                "file.txt": "",
                ".delta": {
                    "clones": {
                        "alpha": {
                            "project.git": {
                                "HEAD": "ref: refs/heads/feature\n",
                                "config": "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
                                "delta": { "copy-baseline": "" },
                                "objects": { "info": null },
                                "refs": null,
                            },
                        },
                        "beta": {
                            "project.git": {
                                "HEAD": "ref: refs/heads/topic\n",
                                "config": "[core]\n\trepositoryformatversion = 0\n\tbare = false\n",
                                "delta": { "copy-baseline": "" },
                                "objects": { "info": null },
                                "refs": null,
                            },
                        },
                    },
                    "worktrees": {
                        "alpha": { "project": { ".git": "" } },
                        "beta": { "project": { ".git": "" } },
                    },
                },
            },
        }));
        let fs: Arc<dyn Fs> = fs::RealFs::new(None, cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let root = SanitizedPath::new(&fs.canonicalize(tree.path()).await.unwrap()).to_path_buf();
        let source_path = root.join("project");
        for name in ["alpha", "beta"] {
            let clone_path = source_path.join(format!(".delta/clones/{name}/project.git"));
            std::fs::write(
                clone_path.join("objects/info/alternates"),
                format!("{}\n", source_path.join(".git/objects").display()),
            )
            .unwrap();
            let checkout_path = source_path.join(format!(".delta/worktrees/{name}/project"));
            std::fs::write(
                checkout_path.join(".git"),
                format!("gitdir: {}\n", clone_path.display()),
            )
            .unwrap();
            fs.git_command(
                Path::new("git"),
                &source_path,
                vec![
                    "config".to_string(),
                    "--file".to_string(),
                    clone_path.join("config").display().to_string(),
                    "core.worktree".to_string(),
                    checkout_path.display().to_string(),
                ],
            )
            .await
            .unwrap();
        }
        (tree, fs, source_path)
    }

    async fn open_real_git_worktree_picker(
        fs: Arc<dyn Fs>,
        checkout_path: &Path,
        cx: &mut TestAppContext,
    ) -> (Entity<WorktreePicker>, VisualTestContext) {
        let project = Project::test(fs, [checkout_path], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;
        let window_handle =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window_handle
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(AnyWindowHandle::from(window_handle), cx);
        let worktree_picker = cx.update(|window, cx| {
            cx.new(|cx| WorktreePicker::new(project, workspace.downgrade(), window, cx))
        });
        (worktree_picker, cx)
    }
}
