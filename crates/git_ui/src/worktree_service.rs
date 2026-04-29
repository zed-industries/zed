use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use collections::HashSet;
use fs::Fs;
use gpui::{AsyncWindowContext, Entity, SharedString, WeakEntity};
use project::Project;
use project::git_store::Repository;
use project::project_settings::ProjectSettings;
use project::trusted_worktrees::{PathTrust, TrustedWorktrees};
use remote::RemoteConnectionOptions;
use settings::Settings;
use workspace::{MultiWorkspace, OpenMode, PreviousWorkspaceState, Workspace, dock::DockPosition};
use zed_actions::NewWorktreeBranchTarget;

use util::ResultExt as _;

use crate::git_panel::show_error_toast;
use crate::worktree_names;

/// Whether a worktree operation is creating a new one or switching to an
/// existing one. Controls whether the source workspace's state (dock layout,
/// open files, agent panel draft) is inherited by the destination.
enum WorktreeOperation {
    Create,
    Switch,
}

/// Classifies the project's visible worktrees into git-managed repositories
/// and non-git paths. Each unique repository is returned only once.
pub fn classify_worktrees(
    project: &Project,
    cx: &gpui::App,
) -> (Vec<Entity<Repository>>, Vec<PathBuf>) {
    let repositories = project.repositories(cx).clone();
    let mut git_repos: Vec<Entity<Repository>> = Vec::new();
    let mut non_git_paths: Vec<PathBuf> = Vec::new();
    let mut seen_repo_ids = HashSet::default();

    for worktree in project.visible_worktrees(cx) {
        let wt_path = worktree.read(cx).abs_path();

        let matching_repo = repositories
            .iter()
            .filter_map(|(id, repo)| {
                let work_dir = repo.read(cx).work_directory_abs_path.clone();
                if wt_path.starts_with(work_dir.as_ref()) {
                    Some((*id, repo.clone(), work_dir.as_ref().components().count()))
                } else {
                    None
                }
            })
            .max_by(
                |(left_id, _left_repo, left_depth), (right_id, _right_repo, right_depth)| {
                    left_depth
                        .cmp(right_depth)
                        .then_with(|| left_id.cmp(right_id))
                },
            );

        if let Some((id, repo, _)) = matching_repo {
            if seen_repo_ids.insert(id) {
                git_repos.push(repo);
            }
        } else {
            non_git_paths.push(wt_path.to_path_buf());
        }
    }

    (git_repos, non_git_paths)
}

/// Resolves a branch target into the ref the new worktree should be based on.
/// Returns `None` for `CurrentBranch`, meaning "use the current HEAD".
pub fn resolve_worktree_branch_target(branch_target: &NewWorktreeBranchTarget) -> Option<String> {
    match branch_target {
        NewWorktreeBranchTarget::CurrentBranch => None,
        NewWorktreeBranchTarget::ExistingBranch { name } => Some(name.clone()),
    }
}

/// Kicks off an async git-worktree creation for each repository. Returns:
///
/// - `creation_infos`: a vec of `(repo, new_path, receiver)` tuples.
/// - `path_remapping`: `(old_work_dir, new_worktree_path)` pairs for remapping editor tabs.
fn start_worktree_creations(
    git_repos: &[Entity<Repository>],
    worktree_name: Option<String>,
    existing_worktree_names: &[String],
    existing_worktree_paths: &HashSet<PathBuf>,
    base_ref: Option<String>,
    worktree_directory_setting: &str,
    rng: &mut impl rand::Rng,
    cx: &mut gpui::App,
) -> anyhow::Result<(
    Vec<(
        Entity<Repository>,
        PathBuf,
        futures::channel::oneshot::Receiver<anyhow::Result<()>>,
    )>,
    Vec<(PathBuf, PathBuf)>,
)> {
    let mut creation_infos = Vec::new();
    let mut path_remapping = Vec::new();

    let worktree_name = worktree_name.unwrap_or_else(|| {
        let existing_refs: Vec<&str> = existing_worktree_names.iter().map(|s| s.as_str()).collect();
        worktree_names::generate_worktree_name(&existing_refs, rng)
            .unwrap_or_else(|| "worktree".to_string())
    });

    for repo in git_repos {
        let (work_dir, new_path, receiver) = repo.update(cx, |repo, _cx| {
            let new_path =
                repo.path_for_new_linked_worktree(&worktree_name, worktree_directory_setting)?;
            if existing_worktree_paths.contains(&new_path) {
                anyhow::bail!("A worktree already exists at {}", new_path.display());
            }
            let target = git::repository::CreateWorktreeTarget::Detached {
                base_sha: base_ref.clone(),
            };
            let receiver = repo.create_worktree(target, new_path.clone());
            let work_dir = repo.work_directory_abs_path.clone();
            anyhow::Ok((work_dir, new_path, receiver))
        })?;
        path_remapping.push((work_dir.to_path_buf(), new_path.clone()));
        creation_infos.push((repo.clone(), new_path, receiver));
    }

    Ok((creation_infos, path_remapping))
}

/// Waits for every in-flight worktree creation to complete. If any
/// creation fails, all successfully-created worktrees are rolled back
/// (removed) so the project isn't left in a half-migrated state.
pub async fn await_and_rollback_on_failure(
    creation_infos: Vec<(
        Entity<Repository>,
        PathBuf,
        futures::channel::oneshot::Receiver<anyhow::Result<()>>,
    )>,
    fs: Arc<dyn Fs>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut created_paths: Vec<PathBuf> = Vec::new();
    let mut repos_and_paths: Vec<(Entity<Repository>, PathBuf)> = Vec::new();
    let mut first_error: Option<anyhow::Error> = None;

    for (repo, new_path, receiver) in creation_infos {
        repos_and_paths.push((repo.clone(), new_path.clone()));
        match receiver.await {
            Ok(Ok(())) => {
                created_paths.push(new_path);
            }
            Ok(Err(err)) => {
                if first_error.is_none() {
                    first_error = Some(err);
                }
            }
            Err(_canceled) => {
                if first_error.is_none() {
                    first_error = Some(anyhow!("Worktree creation was canceled"));
                }
            }
        }
    }

    let Some(err) = first_error else {
        return Ok(created_paths);
    };

    // Rollback all attempted worktrees
    let mut rollback_futures = Vec::new();
    for (rollback_repo, rollback_path) in &repos_and_paths {
        let receiver = cx
            .update(|_, cx| {
                rollback_repo.update(cx, |repo, _cx| {
                    repo.remove_worktree(rollback_path.clone(), true)
                })
            })
            .ok();

        rollback_futures.push((rollback_path.clone(), receiver));
    }

    let mut rollback_failures: Vec<String> = Vec::new();
    for (path, receiver_opt) in rollback_futures {
        let mut git_remove_failed = false;

        if let Some(receiver) = receiver_opt {
            match receiver.await {
                Ok(Ok(())) => {}
                Ok(Err(rollback_err)) => {
                    log::error!(
                        "git worktree remove failed for {}: {rollback_err}",
                        path.display()
                    );
                    git_remove_failed = true;
                }
                Err(canceled) => {
                    log::error!(
                        "git worktree remove failed for {}: {canceled}",
                        path.display()
                    );
                    git_remove_failed = true;
                }
            }
        } else {
            log::error!(
                "failed to dispatch git worktree remove for {}",
                path.display()
            );
            git_remove_failed = true;
        }

        if git_remove_failed {
            if let Err(fs_err) = fs
                .remove_dir(
                    &path,
                    fs::RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
            {
                let msg = format!("{}: failed to remove directory: {fs_err}", path.display());
                log::error!("{}", msg);
                rollback_failures.push(msg);
            }
        }
    }
    let mut error_message = format!("Failed to create worktree: {err}");
    if !rollback_failures.is_empty() {
        error_message.push_str("\n\nFailed to clean up: ");
        error_message.push_str(&rollback_failures.join(", "));
    }
    Err(anyhow!(error_message))
}

/// Propagates worktree trust from the source workspace to the new workspace.
/// If the source project's worktrees are all trusted, the new worktree paths
/// will also be trusted automatically.
fn maybe_propagate_worktree_trust(
    source_workspace: &WeakEntity<Workspace>,
    new_workspace: &Entity<Workspace>,
    paths: &[PathBuf],
    cx: &mut AsyncWindowContext,
) {
    cx.update(|_, cx| {
        if ProjectSettings::get_global(cx).session.trust_all_worktrees {
            return;
        }
        let Some(trusted_store) = TrustedWorktrees::try_get_global(cx) else {
            return;
        };

        let source_is_trusted = source_workspace
            .upgrade()
            .map(|workspace| {
                let source_worktree_store = workspace.read(cx).project().read(cx).worktree_store();
                !trusted_store
                    .read(cx)
                    .has_restricted_worktrees(&source_worktree_store, cx)
            })
            .unwrap_or(false);

        if !source_is_trusted {
            return;
        }

        let worktree_store = new_workspace.read(cx).project().read(cx).worktree_store();
        let paths_to_trust: HashSet<_> = paths
            .iter()
            .filter_map(|path| {
                let (worktree, _) = worktree_store.read(cx).find_worktree(path, cx)?;
                Some(PathTrust::Worktree(worktree.read(cx).id()))
            })
            .collect();

        if !paths_to_trust.is_empty() {
            trusted_store.update(cx, |store, cx| {
                store.trust(&worktree_store, paths_to_trust, cx);
            });
        }
    })
    .ok();
}

/// Handles the `CreateWorktree` action generically, without any agent panel involvement.
/// Creates a new git worktree, opens the workspace, restores layout and files.
pub fn handle_create_worktree(
    workspace: &mut Workspace,
    action: &zed_actions::CreateWorktree,
    window: &mut gpui::Window,
    fallback_focused_dock: Option<DockPosition>,
    cx: &mut gpui::Context<Workspace>,
) {
    let project = workspace.project().clone();

    if project.read(cx).repositories(cx).is_empty() {
        log::error!("create_worktree: no git repository in the project");
        return;
    }
    if project.read(cx).is_via_collab() {
        log::error!("create_worktree: not supported in collab projects");
        return;
    }

    // Guard against concurrent creation
    if workspace.active_worktree_creation().label.is_some() {
        return;
    }

    let previous_state =
        workspace.capture_state_for_worktree_switch(window, fallback_focused_dock, cx);
    let workspace_handle = workspace.weak_handle();
    let window_handle = window.window_handle().downcast::<MultiWorkspace>();
    let remote_connection_options = project.read(cx).remote_connection_options(cx);

    let (git_repos, non_git_paths) = classify_worktrees(project.read(cx), cx);

    if git_repos.is_empty() {
        show_error_toast(
            cx.entity(),
            "worktree create",
            anyhow!("No git repositories found in the project"),
            cx,
        );
        return;
    }

    if remote_connection_options.is_some() {
        let is_disconnected = project
            .read(cx)
            .remote_client()
            .is_some_and(|client| client.read(cx).is_disconnected());
        if is_disconnected {
            show_error_toast(
                cx.entity(),
                "worktree create",
                anyhow!("Cannot create worktree: remote connection is not active"),
                cx,
            );
            return;
        }
    }

    let worktree_name = action.worktree_name.clone();
    let branch_target = action.branch_target.clone();
    let display_name: SharedString = worktree_name
        .as_deref()
        .unwrap_or("worktree")
        .to_string()
        .into();

    workspace.set_active_worktree_creation(Some(display_name), false, cx);

    cx.spawn_in(window, async move |_workspace_entity, mut cx| {
        let result = do_create_worktree(
            git_repos,
            non_git_paths,
            worktree_name,
            branch_target,
            previous_state,
            workspace_handle.clone(),
            window_handle,
            remote_connection_options,
            &mut cx,
        )
        .await;

        if let Err(err) = &result {
            log::error!("Failed to create worktree: {err}");
            workspace_handle
                .update(cx, |workspace, cx| {
                    workspace.set_active_worktree_creation(None, false, cx);
                    show_error_toast(cx.entity(), "worktree create", anyhow!("{err:#}"), cx);
                })
                .ok();
        }

        result
    })
    .detach_and_log_err(cx);
}

pub fn handle_switch_worktree(
    workspace: &mut Workspace,
    action: &zed_actions::SwitchWorktree,
    window: &mut gpui::Window,
    fallback_focused_dock: Option<DockPosition>,
    cx: &mut gpui::Context<Workspace>,
) {
    let project = workspace.project().clone();

    if project.read(cx).repositories(cx).is_empty() {
        log::error!("switch_to_worktree: no git repository in the project");
        return;
    }
    if project.read(cx).is_via_collab() {
        log::error!("switch_to_worktree: not supported in collab projects");
        return;
    }

    // Guard against concurrent creation
    if workspace.active_worktree_creation().label.is_some() {
        return;
    }

    let previous_state =
        workspace.capture_state_for_worktree_switch(window, fallback_focused_dock, cx);
    let workspace_handle = workspace.weak_handle();
    let window_handle = window.window_handle().downcast::<MultiWorkspace>();
    let remote_connection_options = project.read(cx).remote_connection_options(cx);

    let (git_repos, non_git_paths) = classify_worktrees(project.read(cx), cx);

    let git_repo_work_dirs: Vec<PathBuf> = git_repos
        .iter()
        .map(|repo| repo.read(cx).work_directory_abs_path.to_path_buf())
        .collect();

    let display_name: SharedString = action.display_name.clone().into();

    workspace.set_active_worktree_creation(Some(display_name), true, cx);

    let worktree_path = action.path.clone();

    cx.spawn_in(window, async move |_workspace_entity, mut cx| {
        let result = do_switch_worktree(
            worktree_path,
            git_repo_work_dirs,
            non_git_paths,
            previous_state,
            workspace_handle.clone(),
            window_handle,
            remote_connection_options,
            &mut cx,
        )
        .await;

        if let Err(err) = &result {
            log::error!("Failed to switch worktree: {err}");
            workspace_handle
                .update(cx, |workspace, cx| {
                    workspace.set_active_worktree_creation(None, false, cx);
                    show_error_toast(cx.entity(), "worktree switch", anyhow!("{err:#}"), cx);
                })
                .ok();
        }

        result
    })
    .detach_and_log_err(cx);
}

async fn do_create_worktree(
    git_repos: Vec<Entity<Repository>>,
    non_git_paths: Vec<PathBuf>,
    worktree_name: Option<String>,
    branch_target: NewWorktreeBranchTarget,
    previous_state: PreviousWorkspaceState,
    workspace: WeakEntity<Workspace>,
    window_handle: Option<gpui::WindowHandle<MultiWorkspace>>,
    remote_connection_options: Option<RemoteConnectionOptions>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    // List existing worktrees from all repos to detect name collisions
    let worktree_receivers: Vec<_> = cx.update(|_, cx| {
        git_repos
            .iter()
            .map(|repo| repo.update(cx, |repo, _cx| repo.worktrees()))
            .collect()
    })?;
    let worktree_directory_setting = cx.update(|_, cx| {
        ProjectSettings::get_global(cx)
            .git
            .worktree_directory
            .clone()
    })?;

    let mut existing_worktree_names = Vec::new();
    let mut existing_worktree_paths = HashSet::default();
    for result in futures::future::join_all(worktree_receivers).await {
        match result {
            Ok(Ok(worktrees)) => {
                for worktree in worktrees {
                    if let Some(name) = worktree
                        .path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                    {
                        existing_worktree_names.push(name.to_string());
                    }
                    existing_worktree_paths.insert(worktree.path.clone());
                }
            }
            Ok(Err(err)) => {
                Err::<(), _>(err).log_err();
            }
            Err(_) => {}
        }
    }

    let mut rng = rand::rng();

    let base_ref = resolve_worktree_branch_target(&branch_target);

    let (creation_infos, path_remapping) = cx.update(|_, cx| {
        start_worktree_creations(
            &git_repos,
            worktree_name,
            &existing_worktree_names,
            &existing_worktree_paths,
            base_ref,
            &worktree_directory_setting,
            &mut rng,
            cx,
        )
    })??;

    let fs = cx.update(|_, cx| <dyn Fs>::global(cx))?;

    let created_paths = await_and_rollback_on_failure(creation_infos, fs, cx).await?;

    let mut all_paths = created_paths;
    let has_non_git = !non_git_paths.is_empty();
    all_paths.extend(non_git_paths.iter().cloned());

    open_worktree_workspace(
        all_paths,
        path_remapping,
        non_git_paths,
        has_non_git,
        previous_state,
        workspace,
        window_handle,
        remote_connection_options,
        WorktreeOperation::Create,
        cx,
    )
    .await
}

async fn do_switch_worktree(
    worktree_path: PathBuf,
    git_repo_work_dirs: Vec<PathBuf>,
    non_git_paths: Vec<PathBuf>,
    previous_state: PreviousWorkspaceState,
    workspace: WeakEntity<Workspace>,
    window_handle: Option<gpui::WindowHandle<MultiWorkspace>>,
    remote_connection_options: Option<RemoteConnectionOptions>,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    let path_remapping: Vec<(PathBuf, PathBuf)> = git_repo_work_dirs
        .iter()
        .map(|work_dir| (work_dir.clone(), worktree_path.clone()))
        .collect();

    let mut all_paths = vec![worktree_path];
    let has_non_git = !non_git_paths.is_empty();
    all_paths.extend(non_git_paths.iter().cloned());

    open_worktree_workspace(
        all_paths,
        path_remapping,
        non_git_paths,
        has_non_git,
        previous_state,
        workspace,
        window_handle,
        remote_connection_options,
        WorktreeOperation::Switch,
        cx,
    )
    .await
}

/// Core workspace opening logic shared by both create and switch flows.
async fn open_worktree_workspace(
    all_paths: Vec<PathBuf>,
    path_remapping: Vec<(PathBuf, PathBuf)>,
    non_git_paths: Vec<PathBuf>,
    has_non_git: bool,
    previous_state: PreviousWorkspaceState,
    workspace: WeakEntity<Workspace>,
    window_handle: Option<gpui::WindowHandle<MultiWorkspace>>,
    remote_connection_options: Option<RemoteConnectionOptions>,
    operation: WorktreeOperation,
    cx: &mut AsyncWindowContext,
) -> anyhow::Result<()> {
    let window_handle = window_handle
        .ok_or_else(|| anyhow!("No window handle available for workspace creation"))?;

    let focused_dock = previous_state.focused_dock;

    let is_creating_new_worktree = matches!(operation, WorktreeOperation::Create);

    let source_for_transfer = if is_creating_new_worktree {
        Some(workspace.clone())
    } else {
        None
    };

    let (workspace_task, modal_workspace) =
        window_handle.update(cx, |multi_workspace, window, cx| {
            let path_list = util::path_list::PathList::new(&all_paths);
            let active_workspace = multi_workspace.workspace().clone();
            let modal_workspace = active_workspace.clone();

            let init: Option<
                Box<
                    dyn FnOnce(&mut Workspace, &mut gpui::Window, &mut gpui::Context<Workspace>)
                        + Send,
                >,
            > = if is_creating_new_worktree {
                let dock_structure = previous_state.dock_structure;
                Some(Box::new(
                    move |workspace: &mut Workspace,
                          window: &mut gpui::Window,
                          cx: &mut gpui::Context<Workspace>| {
                        workspace.set_dock_structure(dock_structure, window, cx);
                    },
                ))
            } else {
                None
            };

            let task = multi_workspace.find_or_create_workspace_with_source_workspace(
                path_list,
                remote_connection_options,
                None,
                move |connection_options, window, cx| {
                    remote_connection::connect_with_modal(
                        &active_workspace,
                        connection_options,
                        window,
                        cx,
                    )
                },
                &[],
                init,
                OpenMode::Add,
                source_for_transfer.clone(),
                window,
                cx,
            );
            (task, modal_workspace)
        })?;

    let result = workspace_task.await;
    remote_connection::dismiss_connection_modal(&modal_workspace, cx);
    let new_workspace = result?;

    let panels_task = new_workspace.update(cx, |workspace, _cx| workspace.take_panels_task());

    if let Some(task) = panels_task {
        task.await.log_err();
    }

    new_workspace
        .update(cx, |workspace, cx| {
            workspace.project().read(cx).wait_for_initial_scan(cx)
        })
        .await;

    new_workspace
        .update(cx, |workspace, cx| {
            let repos = workspace
                .project()
                .read(cx)
                .repositories(cx)
                .values()
                .cloned()
                .collect::<Vec<_>>();

            let tasks = repos
                .into_iter()
                .map(|repo| repo.update(cx, |repo, _| repo.barrier()));
            futures::future::join_all(tasks)
        })
        .await;

    maybe_propagate_worktree_trust(&workspace, &new_workspace, &all_paths, cx);

    if is_creating_new_worktree {
        window_handle.update(cx, |_multi_workspace, window, cx| {
            new_workspace.update(cx, |workspace, cx| {
                if has_non_git {
                    struct WorktreeCreationToast;
                    let toast_id =
                        workspace::notifications::NotificationId::unique::<WorktreeCreationToast>();
                    workspace.show_toast(
                        workspace::Toast::new(
                            toast_id,
                            "Some project folders are not git repositories. \
                             They were included as-is without creating a worktree.",
                        ),
                        cx,
                    );
                }

                // Remap every previously-open file path into the new worktree.
                let remap_path = |original_path: PathBuf| -> Option<PathBuf> {
                    let best_match = path_remapping
                        .iter()
                        .filter_map(|(old_root, new_root)| {
                            original_path.strip_prefix(old_root).ok().map(|relative| {
                                (old_root.components().count(), new_root.join(relative))
                            })
                        })
                        .max_by_key(|(depth, _)| *depth);

                    if let Some((_, remapped_path)) = best_match {
                        return Some(remapped_path);
                    }

                    for non_git in &non_git_paths {
                        if original_path.starts_with(non_git) {
                            return Some(original_path);
                        }
                    }
                    None
                };

                let remapped_active_path =
                    previous_state.active_file_path.and_then(|p| remap_path(p));

                let mut paths_to_open: Vec<PathBuf> = Vec::new();
                let mut seen = HashSet::default();
                for path in previous_state.open_file_paths {
                    if let Some(remapped) = remap_path(path) {
                        if remapped_active_path.as_ref() != Some(&remapped)
                            && seen.insert(remapped.clone())
                        {
                            paths_to_open.push(remapped);
                        }
                    }
                }

                if let Some(active) = &remapped_active_path {
                    if seen.insert(active.clone()) {
                        paths_to_open.push(active.clone());
                    }
                }

                if !paths_to_open.is_empty() {
                    let should_focus_center = focused_dock.is_none();
                    let open_task = workspace.open_paths(
                        paths_to_open,
                        workspace::OpenOptions {
                            focus: Some(false),
                            ..Default::default()
                        },
                        None,
                        window,
                        cx,
                    );
                    cx.spawn_in(window, async move |workspace, cx| {
                        for item in open_task.await.into_iter().flatten() {
                            item.log_err();
                        }
                        if should_focus_center {
                            workspace.update_in(cx, |workspace, window, cx| {
                                workspace.focus_center_pane(window, cx);
                            })?;
                        }
                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
            });
        })?;
    }

    // Clear the creation status on the SOURCE workspace so its title bar
    // stops showing the loading indicator immediately.
    workspace
        .update(cx, |ws, cx| {
            ws.set_active_worktree_creation(None, false, cx);
        })
        .ok();

    window_handle.update(cx, |multi_workspace, window, cx| {
        multi_workspace.activate(new_workspace.clone(), source_for_transfer, window, cx);

        if is_creating_new_worktree {
            new_workspace.update(cx, |workspace, cx| {
                workspace.run_create_worktree_tasks(window, cx);

                if let Some(dock_position) = focused_dock {
                    let dock = workspace.dock_at_position(dock_position);
                    if let Some(panel) = dock.read(cx).active_panel() {
                        panel.panel_focus_handle(cx).focus(window, cx);
                    }
                }
            });
        }
    })?;

    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::Fs;
    use gpui::{App, Task, TestAppContext};
    use language::language_settings::AllLanguageSettings;
    use project::project_settings::ProjectSettings;
    use project::task_store::{TaskSettingsLocation, TaskStore};
    use project::{FakeFs, WorktreeSettings};
    use serde_json::json;
    use settings::{SettingsLocation, SettingsStore};
    use std::path::{Path, PathBuf};
    use std::process::ExitStatus;
    use std::sync::Mutex;
    use task::SpawnInTerminal;
    use theme::LoadThemes;
    use util::path;
    use util::rel_path::rel_path;
    use workspace::{TerminalProvider, WorkspaceSettings};

    struct CountingTerminalProvider {
        spawned_task_labels: Arc<Mutex<Vec<String>>>,
    }

    impl TerminalProvider for CountingTerminalProvider {
        fn spawn(
            &self,
            task: SpawnInTerminal,
            _window: &mut ui::Window,
            _cx: &mut App,
        ) -> Task<Option<anyhow::Result<ExitStatus>>> {
            self.spawned_task_labels
                .lock()
                .expect("terminal spawn mutex should not be poisoned")
                .push(task.label);
            Task::ready(Some(Ok(ExitStatus::default())))
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(LoadThemes::JustBase, cx);
            AllLanguageSettings::register(cx);
            editor::init(cx);
            ProjectSettings::register(cx);
            WorktreeSettings::register(cx);
            WorkspaceSettings::register(cx);
            TaskStore::init(None);
        });
    }

    fn install_counting_provider_and_worktree_hook(
        workspace: &Entity<Workspace>,
        spawned_task_labels: &Arc<Mutex<Vec<String>>>,
        main_project_root: &Path,
        hook_tasks_json: &str,
        cx: &mut App,
    ) {
        workspace.update(cx, |workspace, cx| {
            workspace.set_terminal_provider(CountingTerminalProvider {
                spawned_task_labels: spawned_task_labels.clone(),
            });

            let project = workspace.project().clone();
            let Some(worktree) = project.read(cx).worktrees(cx).next() else {
                return;
            };
            let worktree = worktree.read(cx);
            let worktree_id = worktree.id();
            let worktree_root = worktree.abs_path().to_path_buf();
            if worktree_root == main_project_root {
                return;
            }

            let Some(task_inventory) = project
                .read(cx)
                .task_store()
                .read(cx)
                .task_inventory()
                .cloned()
            else {
                return;
            };
            task_inventory.update(cx, |inventory, _| {
                inventory
                    .update_file_based_tasks(
                        TaskSettingsLocation::Worktree(SettingsLocation {
                            worktree_id,
                            path: rel_path(".zed"),
                        }),
                        Some(hook_tasks_json),
                    )
                    .expect("should inject create_worktree hook tasks for linked worktree");
            });
        });
    }

    #[gpui::test]
    async fn test_create_worktree_hook_does_not_run_when_switching_back_to_main_worktree(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let hook_tasks_json = r#"[{"label":"setup worktree","command":"echo","hide":"never","hooks":["create_worktree"]}]"#;
        let fs = FakeFs::new(cx.background_executor.clone());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        fs.insert_tree(
            "/root",
            json!({
                "project": {
                    ".git": {},
                    ".zed": {
                        "tasks.json": hook_tasks_json,
                    },
                    "src": {
                        "main.rs": "fn main() {}",
                    },
                },
            }),
        )
        .await;

        let main_project_root = PathBuf::from(path!("/root/project"));
        let project = Project::test(fs.clone(), [main_project_root.as_path()], cx).await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));

        let spawned_task_labels = Arc::new(Mutex::new(Vec::new()));
        multi_workspace.update(cx, |multi_workspace, cx| {
            multi_workspace.retain_active_workspace(cx);
            let active_workspace = multi_workspace.workspace().clone();
            install_counting_provider_and_worktree_hook(
                &active_workspace,
                &spawned_task_labels,
                &main_project_root,
                hook_tasks_json,
                cx,
            );
        });

        let main_workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        main_workspace.update_in(cx, |workspace, window, cx| {
            handle_create_worktree(
                workspace,
                &zed_actions::CreateWorktree {
                    worktree_name: Some("feature".to_string()),
                    branch_target: NewWorktreeBranchTarget::CurrentBranch,
                },
                window,
                None,
                cx,
            );
        });
        cx.run_until_parked();

        let active_workspace =
            multi_workspace.read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone());
        cx.update(|_, cx| {
            install_counting_provider_and_worktree_hook(
                &active_workspace,
                &spawned_task_labels,
                &main_project_root,
                hook_tasks_json,
                cx,
            );
        });
        active_workspace.update_in(cx, |workspace, window, cx| {
            workspace.run_create_worktree_tasks(window, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            spawned_task_labels
                .lock()
                .expect("terminal spawn mutex should not be poisoned")
                .as_slice(),
            ["setup worktree"],
            "create_worktree hook should run once for the created linked worktree"
        );

        active_workspace.update_in(cx, |workspace, window, cx| {
            handle_switch_worktree(
                workspace,
                &zed_actions::SwitchWorktree {
                    path: main_project_root.clone(),
                    display_name: "project".to_string(),
                },
                window,
                None,
                cx,
            );
        });
        cx.run_until_parked();

        assert_eq!(
            spawned_task_labels
                .lock()
                .expect("terminal spawn mutex should not be poisoned")
                .as_slice(),
            ["setup worktree"],
            "switching back to the main worktree should not rerun create_worktree hooks"
        );
    }
}
