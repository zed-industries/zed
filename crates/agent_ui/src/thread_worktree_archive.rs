use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use fs::{Fs, RemoveOptions};
use gpui::{App, AsyncApp, Entity, Task};
use project::{
    LocalProjectFlags, Project, WorktreeId,
    git_store::{Repository, resolve_git_worktree_to_main_repo, worktrees_directory_for_repo},
    project_settings::ProjectSettings,
};
use settings::Settings;
use util::ResultExt;
use workspace::{AppState, MultiWorkspace, Workspace};

use crate::thread_metadata_store::{ArchivedGitWorktree, ThreadId, ThreadMetadataStore};

/// The plan for archiving a single git worktree root.
///
/// A thread can have multiple folder paths open, so there may be multiple
/// `RootPlan`s per archival operation. Each one captures everything needed to
/// persist the worktree's git state and then remove it from disk.
///
/// All fields are gathered synchronously by [`build_root_plan`] while the
/// worktree is still loaded in open projects. This is important because
/// workspace removal tears down project and repository entities, making
/// them unavailable for the later async persist/remove steps.
#[derive(Clone)]
pub struct RootPlan {
    /// Absolute path of the git worktree on disk.
    pub root_path: PathBuf,
    /// Absolute path to the main git repository this worktree is linked to.
    /// Used both for creating a git ref to prevent GC of WIP commits during
    /// [`persist_worktree_state`], and for `git worktree remove` during
    /// [`remove_root`].
    pub main_repo_path: PathBuf,
    /// Every open `Project` that has this worktree loaded, so they can all
    /// call `remove_worktree` and release it during [`remove_root`].
    /// Multiple projects can reference the same path when the user has the
    /// worktree open in more than one workspace.
    pub affected_projects: Vec<AffectedProject>,
    /// The `Repository` entity for this linked worktree, used to run git
    /// commands (create WIP commits, stage files, reset) during
    /// [`persist_worktree_state`].
    pub worktree_repo: Entity<Repository>,
    /// The branch the worktree was on, so it can be restored later.
    /// `None` if the worktree was in detached HEAD state.
    pub branch_name: Option<String>,
}

/// A `Project` that references a worktree being archived, paired with the
/// `WorktreeId` it uses for that worktree.
///
/// The same worktree path can appear in multiple open workspaces/projects
/// (e.g. when the user has two windows open that both include the same
/// linked worktree). Each one needs to call `remove_worktree` and wait for
/// the release during [`remove_root`], otherwise the project would still
/// hold a reference to the directory and `git worktree remove` would fail.
#[derive(Clone)]
pub struct AffectedProject {
    pub project: Entity<Project>,
    pub worktree_id: WorktreeId,
}

fn archived_worktree_ref_name(id: i64) -> String {
    format!("refs/archived-worktrees/{}", id)
}

/// Resolves the Zed-managed worktrees base directory for a given repo.
///
/// This intentionally reads the *global* `git.worktree_directory` setting
/// rather than any project-local override, because Zed always uses the
/// global value when creating worktrees and the archive check must match.
fn worktrees_base_for_repo(main_repo_path: &Path, cx: &App) -> Option<PathBuf> {
    let setting = &ProjectSettings::get_global(cx).git.worktree_directory;
    worktrees_directory_for_repo(main_repo_path, setting).log_err()
}

/// Builds a [`RootPlan`] for archiving the git worktree at `path`.
///
/// This is a synchronous planning step that must run *before* any workspace
/// removal, because it needs live project and repository entities that are
/// torn down when a workspace is removed. It does three things:
///
/// 1. Finds every `Project` across all open workspaces that has this
///    worktree loaded (`affected_projects`).
/// 2. Looks for a `Repository` entity whose snapshot identifies this path
///    as a linked worktree (`worktree_repo`), which is needed for the git
///    operations in [`persist_worktree_state`].
/// 3. Determines the `main_repo_path` — the parent repo that owns this
///    linked worktree — needed for both git ref creation and
///    `git worktree remove`.
///
/// Returns `None` if the path is not a linked worktree (main worktrees
/// cannot be archived to disk) or if no open project has it loaded.
pub fn build_root_plan(
    path: &Path,
    workspaces: &[Entity<Workspace>],
    cx: &App,
) -> Option<RootPlan> {
    let path = path.to_path_buf();

    let affected_projects = workspaces
        .iter()
        .filter_map(|workspace| {
            let project = workspace.read(cx).project().clone();
            let worktree = project
                .read(cx)
                .visible_worktrees(cx)
                .find(|worktree| worktree.read(cx).abs_path().as_ref() == path.as_path())?;
            let worktree_id = worktree.read(cx).id();
            Some(AffectedProject {
                project,
                worktree_id,
            })
        })
        .collect::<Vec<_>>();

    if affected_projects.is_empty() {
        return None;
    }

    let linked_repo = workspaces
        .iter()
        .flat_map(|workspace| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .repositories(cx)
                .values()
                .cloned()
                .collect::<Vec<_>>()
        })
        .find_map(|repo| {
            let snapshot = repo.read(cx).snapshot();
            (snapshot.is_linked_worktree()
                && snapshot.work_directory_abs_path.as_ref() == path.as_path())
            .then_some((snapshot, repo))
        });

    // Only linked worktrees can be archived to disk via `git worktree remove`.
    // Main worktrees must be left alone — git refuses to remove them.
    let (linked_snapshot, repo) = linked_repo?;
    let main_repo_path = linked_snapshot.original_repo_abs_path.to_path_buf();

    // Only archive worktrees that live inside the Zed-managed worktrees
    // directory (configured via `git.worktree_directory`). Worktrees the
    // user created outside that directory should be left untouched.
    let worktrees_base = worktrees_base_for_repo(&main_repo_path, cx)?;
    if !path.starts_with(&worktrees_base) {
        return None;
    }

    let branch_name = linked_snapshot
        .branch
        .as_ref()
        .map(|branch| branch.name().to_string());
    Some(RootPlan {
        root_path: path,
        main_repo_path,
        affected_projects,
        worktree_repo: repo,
        branch_name,
    })
}

/// Removes a worktree from all affected projects and deletes it from disk
/// via `git worktree remove`.
///
/// This is the destructive counterpart to [`persist_worktree_state`]. It
/// first detaches the worktree from every [`AffectedProject`], waits for
/// each project to fully release it, then asks the main repository to
/// delete the worktree directory. If the git removal fails, the worktree
/// is re-added to each project via [`rollback_root`].
pub async fn remove_root(root: RootPlan, fs: Arc<dyn Fs>, cx: &mut AsyncApp) -> Result<()> {
    let release_tasks: Vec<_> = root
        .affected_projects
        .iter()
        .map(|affected| {
            let project = affected.project.clone();
            let worktree_id = affected.worktree_id;
            project.update(cx, |project, cx| {
                let wait = project.wait_for_worktree_release(worktree_id, cx);
                project.remove_worktree(worktree_id, cx);
                wait
            })
        })
        .collect();

    if let Err(error) = remove_root_after_worktree_removal(&root, fs, release_tasks, cx).await {
        rollback_root(&root, cx).await;
        return Err(error);
    }

    Ok(())
}

async fn remove_root_after_worktree_removal(
    root: &RootPlan,
    fs: Arc<dyn Fs>,
    release_tasks: Vec<Task<Result<()>>>,
    cx: &mut AsyncApp,
) -> Result<()> {
    for task in release_tasks {
        if let Err(error) = task.await {
            log::error!("Failed waiting for worktree release: {error:#}");
        }
    }

    // Delete the directory ourselves first, then tell git to clean up the
    // metadata. This avoids a problem where `git worktree remove` can
    // remove the metadata in `.git/worktrees/<name>` but fail to delete
    // the directory (git continues past directory-removal errors), leaving
    // an orphaned folder on disk. By deleting the directory first, we
    // guarantee it's gone, and `git worktree remove --force` with a
    // missing working tree just cleans up the admin entry.
    fs.remove_dir(
        &root.root_path,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await
    .with_context(|| {
        format!(
            "failed to delete worktree directory '{}'",
            root.root_path.display()
        )
    })?;

    let (repo, _temp_project) = find_or_create_repository(&root.main_repo_path, cx).await?;
    let receiver = repo.update(cx, |repo: &mut Repository, _cx| {
        repo.remove_worktree(root.root_path.clone(), true)
    });
    let result = receiver
        .await
        .map_err(|_| anyhow!("git worktree metadata cleanup was canceled"))?;
    // Keep _temp_project alive until after the await so the headless project isn't dropped mid-operation
    drop(_temp_project);
    result.context("git worktree metadata cleanup failed")?;

    remove_empty_parent_dirs_up_to_worktrees_base(
        root.root_path.clone(),
        root.main_repo_path.clone(),
        cx,
    )
    .await;

    Ok(())
}

/// After `git worktree remove` deletes the worktree directory, clean up any
/// empty parent directories between it and the Zed-managed worktrees base
/// directory (configured via `git.worktree_directory`). The base directory
/// itself is never removed.
///
/// If the base directory is not an ancestor of `root_path`, no parent
/// directories are removed.
async fn remove_empty_parent_dirs_up_to_worktrees_base(
    root_path: PathBuf,
    main_repo_path: PathBuf,
    cx: &mut AsyncApp,
) {
    let worktrees_base = cx.update(|cx| worktrees_base_for_repo(&main_repo_path, cx));

    if let Some(worktrees_base) = worktrees_base {
        cx.background_executor()
            .spawn(async move {
                remove_empty_ancestors(&root_path, &worktrees_base);
            })
            .await;
    }
}

/// Removes empty directories between `child_path` and `base_path`.
///
/// Walks upward from `child_path`, removing each empty parent directory,
/// stopping before `base_path` itself is removed. If `base_path` is not
/// an ancestor of `child_path`, nothing is removed. If any directory is
/// non-empty (i.e. `std::fs::remove_dir` fails), the walk stops.
fn remove_empty_ancestors(child_path: &Path, base_path: &Path) {
    let mut current = child_path;
    while let Some(parent) = current.parent() {
        if parent == base_path {
            break;
        }
        if !parent.starts_with(base_path) {
            break;
        }
        match std::fs::remove_dir(parent) {
            Ok(()) => {
                log::info!("Removed empty parent directory: {}", parent.display());
            }
            Err(err) if err.kind() == std::io::ErrorKind::DirectoryNotEmpty => break,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                // Already removed by a concurrent process; keep walking upward.
            }
            Err(err) => {
                log::error!(
                    "Failed to remove parent directory {}: {err}",
                    parent.display()
                );
                break;
            }
        }
        current = parent;
    }
}

/// Finds a live `Repository` entity for the given path, or creates a temporary
/// `Project::local` to obtain one.
///
/// `Repository` entities can only be obtained through a `Project` because
/// `GitStore` (which creates and manages `Repository` entities) is owned by
/// `Project`. When no open workspace contains the repo we need, we spin up a
/// headless `Project::local` just to get a `Repository` handle. The caller
/// keeps the returned `Option<Entity<Project>>` alive for the duration of the
/// git operations, then drops it.
///
/// Future improvement: decoupling `GitStore` from `Project` so that
/// `Repository` entities can be created standalone would eliminate this
/// temporary-project workaround.
async fn find_or_create_repository(
    repo_path: &Path,
    cx: &mut AsyncApp,
) -> Result<(Entity<Repository>, Option<Entity<Project>>)> {
    let repo_path_owned = repo_path.to_path_buf();
    let live_repo = cx.update(|cx| {
        all_open_workspaces(cx)
            .into_iter()
            .flat_map(|workspace| {
                workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .repositories(cx)
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .find(|repo| {
                repo.read(cx).snapshot().work_directory_abs_path.as_ref()
                    == repo_path_owned.as_path()
            })
    });

    if let Some(repo) = live_repo {
        return Ok((repo, None));
    }

    let app_state =
        current_app_state(cx).context("no app state available for temporary project")?;
    let temp_project = cx.update(|cx| {
        Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            LocalProjectFlags::default(),
            cx,
        )
    });

    let repo_path_for_worktree = repo_path.to_path_buf();
    let create_worktree = temp_project.update(cx, |project, cx| {
        project.create_worktree(repo_path_for_worktree, true, cx)
    });
    let _worktree = create_worktree.await?;
    let initial_scan = temp_project.read_with(cx, |project, cx| project.wait_for_initial_scan(cx));
    initial_scan.await;

    let repo_path_for_find = repo_path.to_path_buf();
    let repo = temp_project
        .update(cx, |project, cx| {
            project
                .repositories(cx)
                .values()
                .find(|repo| {
                    repo.read(cx).snapshot().work_directory_abs_path.as_ref()
                        == repo_path_for_find.as_path()
                })
                .cloned()
        })
        .context("failed to resolve temporary repository handle")?;

    let barrier = repo.update(cx, |repo: &mut Repository, _cx| repo.barrier());
    barrier
        .await
        .map_err(|_| anyhow!("temporary repository barrier canceled"))?;
    Ok((repo, Some(temp_project)))
}

/// Re-adds the worktree to every affected project after a failed
/// [`remove_root`].
async fn rollback_root(root: &RootPlan, cx: &mut AsyncApp) {
    for affected in &root.affected_projects {
        let task = affected.project.update(cx, |project, cx| {
            project.create_worktree(root.root_path.clone(), true, cx)
        });
        task.await.log_err();
    }
}

/// Saves the worktree's full git state so it can be restored later.
///
/// This creates two detached commits (via [`create_archive_checkpoint`] on
/// the `GitRepository` trait) that capture the staged and unstaged state
/// without moving any branch ref. The commits are:
///   - "WIP staged": a tree matching the current index, parented on HEAD
///   - "WIP unstaged": a tree with all files (including untracked),
///     parented on the staged commit
///
/// After creating the commits, this function:
///   1. Records the commit SHAs, branch name, and paths in a DB record.
///   2. Links every thread referencing this worktree to that record.
///   3. Creates a git ref on the main repo to prevent GC of the commits.
///
/// On success, returns the archived worktree DB row ID for rollback.
pub async fn persist_worktree_state(root: &RootPlan, cx: &mut AsyncApp) -> Result<i64> {
    let worktree_repo = root.worktree_repo.clone();

    let original_commit_hash = worktree_repo
        .update(cx, |repo, _cx| repo.head_sha())
        .await
        .map_err(|_| anyhow!("head_sha canceled"))?
        .context("failed to read original HEAD SHA")?
        .context("HEAD SHA is None")?;

    // Create two detached WIP commits without moving the branch.
    let checkpoint_rx = worktree_repo.update(cx, |repo, _cx| repo.create_archive_checkpoint());
    let (staged_commit_hash, unstaged_commit_hash) = checkpoint_rx
        .await
        .map_err(|_| anyhow!("create_archive_checkpoint canceled"))?
        .context("failed to create archive checkpoint")?;

    // Create DB record
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));
    let worktree_path_str = root.root_path.to_string_lossy().to_string();
    let main_repo_path_str = root.main_repo_path.to_string_lossy().to_string();
    let branch_name = root.branch_name.clone().or_else(|| {
        worktree_repo.read_with(cx, |repo, _cx| {
            repo.snapshot()
                .branch
                .as_ref()
                .map(|branch| branch.name().to_string())
        })
    });

    let db_result = store
        .read_with(cx, |store, cx| {
            store.create_archived_worktree(
                worktree_path_str.clone(),
                main_repo_path_str.clone(),
                branch_name.clone(),
                staged_commit_hash.clone(),
                unstaged_commit_hash.clone(),
                original_commit_hash.clone(),
                cx,
            )
        })
        .await
        .context("failed to create archived worktree DB record");
    let archived_worktree_id = match db_result {
        Ok(id) => id,
        Err(error) => {
            return Err(error);
        }
    };

    // Link all threads on this worktree to the archived record
    let thread_ids: Vec<ThreadId> = store.read_with(cx, |store, _cx| {
        store
            .entries()
            .filter(|thread| {
                thread
                    .folder_paths()
                    .paths()
                    .iter()
                    .any(|p| p.as_path() == root.root_path)
            })
            .map(|thread| thread.thread_id)
            .collect()
    });

    for thread_id in &thread_ids {
        let link_result = store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(*thread_id, archived_worktree_id, cx)
            })
            .await;
        if let Err(error) = link_result {
            if let Err(delete_error) = store
                .read_with(cx, |store, cx| {
                    store.delete_archived_worktree(archived_worktree_id, cx)
                })
                .await
            {
                log::error!(
                    "Failed to delete archived worktree DB record during link rollback: \
                     {delete_error:#}"
                );
            }
            return Err(error.context("failed to link thread to archived worktree"));
        }
    }

    // Create git ref on main repo to prevent GC of the detached commits.
    // This is fatal: without the ref, git gc will eventually collect the
    // WIP commits and a later restore will silently fail.
    let ref_name = archived_worktree_ref_name(archived_worktree_id);
    let (main_repo, _temp_project) = find_or_create_repository(&root.main_repo_path, cx)
        .await
        .context("could not open main repo to create archive ref")?;
    let rx = main_repo.update(cx, |repo, _cx| {
        repo.update_ref(ref_name.clone(), unstaged_commit_hash.clone())
    });
    rx.await
        .map_err(|_| anyhow!("update_ref canceled"))
        .and_then(|r| r)
        .with_context(|| format!("failed to create ref {ref_name} on main repo"))?;
    drop(_temp_project);

    Ok(archived_worktree_id)
}

/// Undoes a successful [`persist_worktree_state`] by deleting the git ref
/// on the main repo and removing the DB record. Since the WIP commits are
/// detached (they don't move any branch), no git reset is needed — the
/// commits will be garbage-collected once the ref is removed.
pub async fn rollback_persist(archived_worktree_id: i64, root: &RootPlan, cx: &mut AsyncApp) {
    // Delete the git ref on main repo
    if let Ok((main_repo, _temp_project)) =
        find_or_create_repository(&root.main_repo_path, cx).await
    {
        let ref_name = archived_worktree_ref_name(archived_worktree_id);
        let rx = main_repo.update(cx, |repo, _cx| repo.delete_ref(ref_name));
        rx.await.ok().and_then(|r| r.log_err());
        drop(_temp_project);
    }

    // Delete the DB record
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));
    if let Err(error) = store
        .read_with(cx, |store, cx| {
            store.delete_archived_worktree(archived_worktree_id, cx)
        })
        .await
    {
        log::error!("Failed to delete archived worktree DB record during rollback: {error:#}");
    }
}

/// Restores a previously archived worktree back to disk from its DB record.
///
/// Creates the git worktree at the original commit (the branch never moved
/// during archival since WIP commits are detached), switches to the branch,
/// then uses [`restore_archive_checkpoint`] to reconstruct the staged/
/// unstaged state from the WIP commit trees.
pub async fn restore_worktree_via_git(
    row: &ArchivedGitWorktree,
    cx: &mut AsyncApp,
) -> Result<PathBuf> {
    let (main_repo, _temp_project) = find_or_create_repository(&row.main_repo_path, cx).await?;

    let worktree_path = &row.worktree_path;
    let app_state = current_app_state(cx).context("no app state available")?;
    let already_exists = app_state.fs.metadata(worktree_path).await?.is_some();

    let created_new_worktree = if already_exists {
        let is_git_worktree =
            resolve_git_worktree_to_main_repo(app_state.fs.as_ref(), worktree_path)
                .await
                .is_some();

        if !is_git_worktree {
            let rx = main_repo.update(cx, |repo, _cx| repo.repair_worktrees());
            rx.await
                .map_err(|_| anyhow!("worktree repair was canceled"))?
                .context("failed to repair worktrees")?;
        }
        false
    } else {
        // Create worktree at the original commit — the branch still points
        // here because archival used detached commits.
        let rx = main_repo.update(cx, |repo, _cx| {
            repo.create_worktree_detached(worktree_path.clone(), row.original_commit_hash.clone())
        });
        rx.await
            .map_err(|_| anyhow!("worktree creation was canceled"))?
            .context("failed to create worktree")?;
        true
    };

    let (wt_repo, _temp_wt_project) = match find_or_create_repository(worktree_path, cx).await {
        Ok(result) => result,
        Err(error) => {
            remove_new_worktree_on_error(created_new_worktree, &main_repo, worktree_path, cx).await;
            return Err(error);
        }
    };

    if let Some(branch_name) = &row.branch_name {
        // Attempt to check out the branch the worktree was previously on.
        let checkout_result = wt_repo
            .update(cx, |repo, _cx| repo.change_branch(branch_name.clone()))
            .await;

        match checkout_result.map_err(|e| anyhow!("{e}")).flatten() {
            Ok(()) => {
                // Branch checkout succeeded. Check whether the branch has moved since
                // we archived the worktree, by comparing HEAD to the expected SHA.
                let head_sha = wt_repo
                    .update(cx, |repo, _cx| repo.head_sha())
                    .await
                    .map_err(|e| anyhow!("{e}"))
                    .and_then(|r| r);

                match head_sha {
                    Ok(Some(sha)) if sha == row.original_commit_hash => {
                        // Branch still points at the original commit; we're all done!
                    }
                    Ok(Some(sha)) => {
                        // The branch has moved. We don't want to restore the worktree to
                        // a different filesystem state, so checkout the original commit
                        // in detached HEAD state.
                        log::info!(
                            "Branch '{branch_name}' has moved since archival (now at {sha}); \
                             restoring worktree in detached HEAD at {}",
                            row.original_commit_hash
                        );
                        let detach_result = main_repo
                            .update(cx, |repo, _cx| {
                                repo.checkout_branch_in_worktree(
                                    row.original_commit_hash.clone(),
                                    row.worktree_path.clone(),
                                    false,
                                )
                            })
                            .await;

                        if let Err(error) = detach_result.map_err(|e| anyhow!("{e}")).flatten() {
                            log::warn!(
                                "Failed to detach HEAD at {}: {error:#}",
                                row.original_commit_hash
                            );
                        }
                    }
                    Ok(None) => {
                        log::warn!(
                            "head_sha unexpectedly returned None after checking out \"{branch_name}\"; \
                             proceeding in current HEAD state."
                        );
                    }
                    Err(error) => {
                        log::warn!(
                            "Failed to read HEAD after checking out \"{branch_name}\": {error:#}"
                        );
                    }
                }
            }
            Err(checkout_error) => {
                // We weren't able to check out the branch, most likely because it was deleted.
                // This is fine; users will often delete old branches! We'll try to recreate it.
                log::debug!(
                    "change_branch('{branch_name}') failed: {checkout_error:#}, trying create_branch"
                );
                let create_result = wt_repo
                    .update(cx, |repo, _cx| {
                        repo.create_branch(branch_name.clone(), None)
                    })
                    .await;

                if let Err(error) = create_result.map_err(|e| anyhow!("{e}")).flatten() {
                    log::warn!(
                        "Failed to create branch '{branch_name}': {error:#}; \
                         restored worktree will be in detached HEAD state."
                    );
                }
            }
        }
    }

    // Restore the staged/unstaged state from the WIP commit trees.
    // read-tree --reset -u applies the unstaged tree (including deletions)
    // to the working directory, then a bare read-tree sets the index to
    // the staged tree without touching the working directory.
    let restore_rx = wt_repo.update(cx, |repo, _cx| {
        repo.restore_archive_checkpoint(
            row.staged_commit_hash.clone(),
            row.unstaged_commit_hash.clone(),
        )
    });
    if let Err(error) = restore_rx
        .await
        .map_err(|_| anyhow!("restore_archive_checkpoint canceled"))
        .and_then(|r| r)
    {
        remove_new_worktree_on_error(created_new_worktree, &main_repo, worktree_path, cx).await;
        return Err(error.context("failed to restore archive checkpoint"));
    }

    Ok(worktree_path.clone())
}

async fn remove_new_worktree_on_error(
    created_new_worktree: bool,
    main_repo: &Entity<Repository>,
    worktree_path: &PathBuf,
    cx: &mut AsyncApp,
) {
    if created_new_worktree {
        let rx = main_repo.update(cx, |repo, _cx| {
            repo.remove_worktree(worktree_path.clone(), true)
        });
        rx.await.ok().and_then(|r| r.log_err());
    }
}

/// Deletes the git ref and DB records for a single archived worktree.
/// Used when an archived worktree is no longer referenced by any thread.
pub async fn cleanup_archived_worktree_record(row: &ArchivedGitWorktree, cx: &mut AsyncApp) {
    // Delete the git ref from the main repo
    if let Ok((main_repo, _temp_project)) = find_or_create_repository(&row.main_repo_path, cx).await
    {
        let ref_name = archived_worktree_ref_name(row.id);
        let rx = main_repo.update(cx, |repo, _cx| repo.delete_ref(ref_name));
        match rx.await {
            Ok(Ok(())) => {}
            Ok(Err(error)) => log::warn!("Failed to delete archive ref: {error}"),
            Err(_) => log::warn!("Archive ref deletion was canceled"),
        }
        // Keep _temp_project alive until after the await so the headless project isn't dropped mid-operation
        drop(_temp_project);
    }

    // Delete the DB records
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));
    store
        .read_with(cx, |store, cx| store.delete_archived_worktree(row.id, cx))
        .await
        .log_err();
}

/// Cleans up all archived worktree data associated with a thread being deleted.
///
/// This unlinks the thread from all its archived worktrees and, for any
/// archived worktree that is no longer referenced by any other thread,
/// deletes the git ref and DB records.
pub async fn cleanup_thread_archived_worktrees(thread_id: ThreadId, cx: &mut AsyncApp) {
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));

    let archived_worktrees = store
        .read_with(cx, |store, cx| {
            store.get_archived_worktrees_for_thread(thread_id, cx)
        })
        .await;
    let archived_worktrees = match archived_worktrees {
        Ok(rows) => rows,
        Err(error) => {
            log::error!("Failed to fetch archived worktrees for thread {thread_id:?}: {error:#}");
            return;
        }
    };

    if archived_worktrees.is_empty() {
        return;
    }

    if let Err(error) = store
        .read_with(cx, |store, cx| {
            store.unlink_thread_from_all_archived_worktrees(thread_id, cx)
        })
        .await
    {
        log::error!("Failed to unlink thread {thread_id:?} from archived worktrees: {error:#}");
        return;
    }

    for row in &archived_worktrees {
        let still_referenced = store
            .read_with(cx, |store, cx| {
                store.is_archived_worktree_referenced(row.id, cx)
            })
            .await;
        match still_referenced {
            Ok(true) => {}
            Ok(false) => {
                cleanup_archived_worktree_record(row, cx).await;
            }
            Err(error) => {
                log::error!(
                    "Failed to check if archived worktree {} is still referenced: {error:#}",
                    row.id
                );
            }
        }
    }
}

/// Collects every `Workspace` entity across all open `MultiWorkspace` windows.
pub fn all_open_workspaces(cx: &App) -> Vec<Entity<Workspace>> {
    cx.windows()
        .into_iter()
        .filter_map(|window| window.downcast::<MultiWorkspace>())
        .flat_map(|multi_workspace| {
            multi_workspace
                .read(cx)
                .map(|multi_workspace| multi_workspace.workspaces().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .collect()
}

fn current_app_state(cx: &mut AsyncApp) -> Option<Arc<AppState>> {
    cx.update(|cx| {
        all_open_workspaces(cx)
            .into_iter()
            .next()
            .map(|workspace| workspace.read(cx).app_state().clone())
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use git::repository::Worktree as GitWorktree;
    use gpui::{BorrowAppContext, TestAppContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use tempfile::TempDir;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
    }

    #[test]
    fn test_remove_empty_ancestors_single_empty_parent() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        let branch_dir = base.join("my-branch");
        let child = branch_dir.join("zed");

        std::fs::create_dir_all(&child).unwrap();
        // Simulate git worktree remove having deleted the child.
        std::fs::remove_dir(&child).unwrap();

        assert!(branch_dir.exists());
        remove_empty_ancestors(&child, &base);
        assert!(!branch_dir.exists(), "empty parent should be removed");
        assert!(base.exists(), "base directory should be preserved");
    }

    #[test]
    fn test_remove_empty_ancestors_nested_empty_parents() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        // Branch name with slash creates nested dirs: fix/thing/zed
        let child = base.join("fix").join("thing").join("zed");

        std::fs::create_dir_all(&child).unwrap();
        std::fs::remove_dir(&child).unwrap();

        assert!(base.join("fix").join("thing").exists());
        remove_empty_ancestors(&child, &base);
        assert!(!base.join("fix").join("thing").exists());
        assert!(
            !base.join("fix").exists(),
            "all empty ancestors should be removed"
        );
        assert!(base.exists(), "base directory should be preserved");
    }

    #[test]
    fn test_remove_empty_ancestors_stops_at_non_empty_parent() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        let branch_dir = base.join("my-branch");
        let child = branch_dir.join("zed");
        let sibling = branch_dir.join("other-file.txt");

        std::fs::create_dir_all(&child).unwrap();
        std::fs::write(&sibling, "content").unwrap();
        std::fs::remove_dir(&child).unwrap();

        remove_empty_ancestors(&child, &base);
        assert!(branch_dir.exists(), "non-empty parent should be preserved");
        assert!(sibling.exists());
    }

    #[test]
    fn test_remove_empty_ancestors_not_an_ancestor() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        let unrelated = tmp.path().join("other-place").join("branch").join("zed");

        std::fs::create_dir_all(&base).unwrap();
        std::fs::create_dir_all(&unrelated).unwrap();
        std::fs::remove_dir(&unrelated).unwrap();

        let parent = unrelated.parent().unwrap();
        assert!(parent.exists());
        remove_empty_ancestors(&unrelated, &base);
        assert!(parent.exists(), "should not remove dirs outside base");
    }

    #[test]
    fn test_remove_empty_ancestors_child_is_direct_child_of_base() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        let child = base.join("zed");

        std::fs::create_dir_all(&child).unwrap();
        std::fs::remove_dir(&child).unwrap();

        remove_empty_ancestors(&child, &base);
        assert!(base.exists(), "base directory should be preserved");
    }

    #[test]
    fn test_remove_empty_ancestors_partially_non_empty_chain() {
        let tmp = TempDir::new().unwrap();
        let base = tmp.path().join("worktrees");
        // Structure: base/a/b/c/zed where a/ has another child besides b/
        let child = base.join("a").join("b").join("c").join("zed");
        let other_in_a = base.join("a").join("other-branch");

        std::fs::create_dir_all(&child).unwrap();
        std::fs::create_dir_all(&other_in_a).unwrap();
        std::fs::remove_dir(&child).unwrap();

        remove_empty_ancestors(&child, &base);
        assert!(
            !base.join("a").join("b").join("c").exists(),
            "c/ should be removed (empty)"
        );
        assert!(
            !base.join("a").join("b").exists(),
            "b/ should be removed (empty)"
        );
        assert!(
            base.join("a").exists(),
            "a/ should be preserved (has other-branch sibling)"
        );
        assert!(other_in_a.exists());
    }

    #[gpui::test]
    async fn test_build_root_plan_returns_none_for_main_worktree(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));

        let project = Project::test(fs.clone(), [Path::new("/project")], cx).await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        // The main worktree should NOT produce a root plan.
        workspace.read_with(cx, |_workspace, cx| {
            let plan = build_root_plan(Path::new("/project"), std::slice::from_ref(&workspace), cx);
            assert!(
                plan.is_none(),
                "build_root_plan should return None for a main worktree",
            );
        });
    }

    #[gpui::test]
    async fn test_build_root_plan_returns_some_for_linked_worktree(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature"]);

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/worktrees/project/feature/project"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/project"),
                Path::new("/worktrees/project/feature/project"),
            ],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        workspace.read_with(cx, |_workspace, cx| {
            // The linked worktree SHOULD produce a root plan.
            let plan = build_root_plan(
                Path::new("/worktrees/project/feature/project"),
                std::slice::from_ref(&workspace),
                cx,
            );
            assert!(
                plan.is_some(),
                "build_root_plan should return Some for a linked worktree",
            );
            let plan = plan.unwrap();
            assert_eq!(
                plan.root_path,
                PathBuf::from("/worktrees/project/feature/project")
            );
            assert_eq!(plan.main_repo_path, PathBuf::from("/project"));

            // The main worktree should still return None.
            let main_plan =
                build_root_plan(Path::new("/project"), std::slice::from_ref(&workspace), cx);
            assert!(
                main_plan.is_none(),
                "build_root_plan should return None for the main worktree \
                 even when a linked worktree exists",
            );
        });
    }

    #[gpui::test]
    async fn test_build_root_plan_returns_none_for_external_linked_worktree(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature"]);

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/external-worktree"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [Path::new("/project"), Path::new("/external-worktree")],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        workspace.read_with(cx, |_workspace, cx| {
            let plan = build_root_plan(
                Path::new("/external-worktree"),
                std::slice::from_ref(&workspace),
                cx,
            );
            assert!(
                plan.is_none(),
                "build_root_plan should return None for a linked worktree \
                 outside the Zed-managed worktrees directory",
            );
        });
    }

    #[gpui::test]
    async fn test_build_root_plan_with_custom_worktree_directory(cx: &mut TestAppContext) {
        init_test(cx);

        // Override the worktree_directory setting to a non-default location.
        // With main repo at /project and setting "../custom-worktrees", the
        // resolved base is /custom-worktrees/project.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |s| {
                    s.git.get_or_insert(Default::default()).worktree_directory =
                        Some("../custom-worktrees".to_string());
                });
            });
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature", "feature2"]);

        // Worktree inside the custom managed directory.
        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/custom-worktrees/project/feature/project"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        // Worktree outside the custom managed directory (at the default
        // `../worktrees` location, which is not what the setting says).
        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/worktrees/project/feature2/project"),
                ref_name: Some("refs/heads/feature2".into()),
                sha: "def456".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/project"),
                Path::new("/custom-worktrees/project/feature/project"),
                Path::new("/worktrees/project/feature2/project"),
            ],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        workspace.read_with(cx, |_workspace, cx| {
            // Worktree inside the custom managed directory SHOULD be archivable.
            let plan = build_root_plan(
                Path::new("/custom-worktrees/project/feature/project"),
                std::slice::from_ref(&workspace),
                cx,
            );
            assert!(
                plan.is_some(),
                "build_root_plan should return Some for a worktree inside \
                 the custom worktree_directory",
            );

            // Worktree at the default location SHOULD NOT be archivable
            // because the setting points elsewhere.
            let plan = build_root_plan(
                Path::new("/worktrees/project/feature2/project"),
                std::slice::from_ref(&workspace),
                cx,
            );
            assert!(
                plan.is_none(),
                "build_root_plan should return None for a worktree outside \
                 the custom worktree_directory, even if it would match the default",
            );
        });
    }

    #[gpui::test]
    async fn test_remove_root_deletes_directory_and_git_metadata(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature"]);

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/worktrees/project/feature/project"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/project"),
                Path::new("/worktrees/project/feature/project"),
            ],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        // Build the root plan while the worktree is still loaded.
        let root = workspace
            .read_with(cx, |_workspace, cx| {
                build_root_plan(
                    Path::new("/worktrees/project/feature/project"),
                    std::slice::from_ref(&workspace),
                    cx,
                )
            })
            .expect("should produce a root plan for the linked worktree");

        assert!(
            fs.is_dir(Path::new("/worktrees/project/feature/project"))
                .await
        );

        // Remove the root.
        let fs_clone = fs.clone();
        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, fs_clone, cx).await));
        task.await.expect("remove_root should succeed");

        cx.run_until_parked();

        // The FakeFs directory should be gone.
        assert!(
            !fs.is_dir(Path::new("/worktrees/project/feature/project"))
                .await,
            "linked worktree directory should be removed from FakeFs"
        );
    }

    #[gpui::test]
    async fn test_remove_root_succeeds_when_directory_already_gone(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature"]);

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/worktrees/project/feature/project"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/project"),
                Path::new("/worktrees/project/feature/project"),
            ],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        let root = workspace
            .read_with(cx, |_workspace, cx| {
                build_root_plan(
                    Path::new("/worktrees/project/feature/project"),
                    std::slice::from_ref(&workspace),
                    cx,
                )
            })
            .expect("should produce a root plan for the linked worktree");

        // Manually remove the worktree directory from FakeFs before calling
        // remove_root, simulating the directory being deleted externally.
        fs.as_ref()
            .remove_dir(
                Path::new("/worktrees/project/feature/project"),
                fs::RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: false,
                },
            )
            .await
            .unwrap();
        assert!(
            !fs.as_ref()
                .is_dir(Path::new("/worktrees/project/feature/project"))
                .await
        );

        // remove_root should still succeed — fs.remove_dir with
        // ignore_if_not_exists handles NotFound, and git worktree remove
        // handles a missing working tree directory.
        let fs_clone = fs.clone();
        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, fs_clone, cx).await));
        task.await
            .expect("remove_root should succeed even when directory is already gone");
    }

    #[gpui::test]
    async fn test_remove_root_returns_error_and_rolls_back_on_remove_dir_failure(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                ".git": {},
                "src": { "main.rs": "fn main() {}" }
            }),
        )
        .await;
        fs.set_branch_name(Path::new("/project/.git"), Some("main"));
        fs.insert_branches(Path::new("/project/.git"), &["main", "feature"]);

        fs.add_linked_worktree_for_repo(
            Path::new("/project/.git"),
            true,
            GitWorktree {
                path: PathBuf::from("/worktrees/project/feature/project"),
                ref_name: Some("refs/heads/feature".into()),
                sha: "abc123".into(),
                is_main: false,
                is_bare: false,
            },
        )
        .await;

        let project = Project::test(
            fs.clone(),
            [
                Path::new("/project"),
                Path::new("/worktrees/project/feature/project"),
            ],
            cx,
        )
        .await;
        project
            .update(cx, |project, cx| project.git_scans_complete(cx))
            .await;

        let multi_workspace =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace
            .read_with(cx, |mw, _cx| mw.workspace().clone())
            .unwrap();

        cx.run_until_parked();

        let root = workspace
            .read_with(cx, |_workspace, cx| {
                build_root_plan(
                    Path::new("/worktrees/project/feature/project"),
                    std::slice::from_ref(&workspace),
                    cx,
                )
            })
            .expect("should produce a root plan for the linked worktree");

        // Replace the worktree directory with a file so that fs.remove_dir
        // fails with a "not a directory" error.
        let worktree_path = Path::new("/worktrees/project/feature/project");
        fs.remove_dir(
            worktree_path,
            fs::RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        fs.create_file(worktree_path, fs::CreateOptions::default())
            .await
            .unwrap();
        assert!(
            fs.is_file(worktree_path).await,
            "path should now be a file, not a directory"
        );

        let fs_clone = fs.clone();
        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, fs_clone, cx).await));
        let result = task.await;

        assert!(
            result.is_err(),
            "remove_root should return an error when fs.remove_dir fails"
        );
        let error_message = format!("{:#}", result.unwrap_err());
        assert!(
            error_message.contains("failed to delete worktree directory"),
            "error should mention the directory deletion failure, got: {error_message}"
        );

        cx.run_until_parked();

        // After rollback, the worktree should be re-added to the project.
        let has_worktree = project.read_with(cx, |project, cx| {
            project
                .worktrees(cx)
                .any(|wt| wt.read(cx).abs_path().as_ref() == worktree_path)
        });
        assert!(
            has_worktree,
            "rollback should have re-added the worktree to the project"
        );
    }
}
