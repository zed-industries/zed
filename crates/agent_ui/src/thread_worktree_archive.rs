use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, AsyncApp, Entity, Task};
use project::{
    LocalProjectFlags, Project, WorktreeId,
    git_store::{Repository, resolve_git_worktree_to_main_repo},
};
use util::ResultExt;
use workspace::{AppState, MultiWorkspace, Workspace};

use crate::thread_metadata_store::{ArchivedGitWorktree, ThreadMetadataStore};

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
    /// The `Repository` entity for this worktree, used to run git commands
    /// (create WIP commits, stage files, reset) during
    /// [`persist_worktree_state`]. `None` when the `GitStore` hasn't created
    /// a `Repository` for this worktree yet — in that case,
    /// `persist_worktree_state` falls back to creating a temporary headless
    /// project to obtain one.
    pub worktree_repo: Option<Entity<Repository>>,
    /// The branch the worktree was on, so it can be restored later.
    /// `None` if the worktree was in detached HEAD state or if no
    /// `Repository` entity was available at planning time (in which case
    /// `persist_worktree_state` reads it from the repo snapshot instead).
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
/// When no `Repository` entity is available (e.g. the `GitStore` hasn't
/// finished scanning), the function falls back to deriving `main_repo_path`
/// from the worktree snapshot's `root_repo_common_dir`. In that case
/// `worktree_repo` is `None` and [`persist_worktree_state`] will create a
/// temporary headless project to obtain one.
///
/// Returns `None` if no open project has this path as a visible worktree.
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

    let matching_worktree_snapshot = workspaces.iter().find_map(|workspace| {
        workspace
            .read(cx)
            .project()
            .read(cx)
            .visible_worktrees(cx)
            .find(|worktree| worktree.read(cx).abs_path().as_ref() == path.as_path())
            .map(|worktree| worktree.read(cx).snapshot())
    });

    let (main_repo_path, worktree_repo, branch_name) =
        if let Some((linked_snapshot, repo)) = linked_repo {
            (
                linked_snapshot.original_repo_abs_path.to_path_buf(),
                Some(repo),
                linked_snapshot
                    .branch
                    .as_ref()
                    .map(|branch| branch.name().to_string()),
            )
        } else {
            let main_repo_path = matching_worktree_snapshot
                .as_ref()?
                .root_repo_common_dir()
                .and_then(|dir| dir.parent())?
                .to_path_buf();
            (main_repo_path, None, None)
        };

    Some(RootPlan {
        root_path: path,
        main_repo_path,
        affected_projects,
        worktree_repo,
        branch_name,
    })
}

/// Returns `true` if any unarchived thread other than `current_session_id`
/// references `path` in its folder paths. Used to determine whether a
/// worktree can safely be removed from disk.
pub fn path_is_referenced_by_other_unarchived_threads(
    current_session_id: &acp::SessionId,
    path: &Path,
    cx: &App,
) -> bool {
    ThreadMetadataStore::global(cx)
        .read(cx)
        .entries()
        .filter(|thread| thread.session_id != *current_session_id)
        .filter(|thread| !thread.archived)
        .any(|thread| {
            thread
                .folder_paths
                .paths()
                .iter()
                .any(|other_path| other_path.as_path() == path)
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
pub async fn remove_root(root: RootPlan, cx: &mut AsyncApp) -> Result<()> {
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

    if let Err(error) = remove_root_after_worktree_removal(&root, release_tasks, cx).await {
        rollback_root(&root, cx).await;
        return Err(error);
    }

    Ok(())
}

async fn remove_root_after_worktree_removal(
    root: &RootPlan,
    release_tasks: Vec<Task<Result<()>>>,
    cx: &mut AsyncApp,
) -> Result<()> {
    for task in release_tasks {
        if let Err(error) = task.await {
            log::error!("Failed waiting for worktree release: {error:#}");
        }
    }

    let (repo, _temp_project) = find_or_create_repository(&root.main_repo_path, cx).await?;
    // force=true is required because the working directory is still dirty
    // — persist_worktree_state captures state into detached commits without
    // modifying the real index or working tree, so git refuses to delete
    // the worktree without --force.
    let receiver = repo.update(cx, |repo: &mut Repository, _cx| {
        repo.remove_worktree(root.root_path.clone(), true)
    });
    let result = receiver
        .await
        .map_err(|_| anyhow!("git worktree removal was canceled"))?;
    // Keep _temp_project alive until after the await so the headless project isn't dropped mid-operation
    drop(_temp_project);
    result
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
    let (worktree_repo, _temp_worktree_project) = match &root.worktree_repo {
        Some(worktree_repo) => (worktree_repo.clone(), None),
        None => find_or_create_repository(&root.root_path, cx).await?,
    };

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
    let session_ids: Vec<acp::SessionId> = store.read_with(cx, |store, _cx| {
        store
            .entries()
            .filter(|thread| {
                thread
                    .folder_paths
                    .paths()
                    .iter()
                    .any(|p| p.as_path() == root.root_path)
            })
            .map(|thread| thread.session_id.clone())
            .collect()
    });

    for session_id in &session_ids {
        let link_result = store
            .read_with(cx, |store, cx| {
                store.link_thread_to_archived_worktree(
                    session_id.0.to_string(),
                    archived_worktree_id,
                    cx,
                )
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

    // Switch to the branch. Since the branch was never moved during
    // archival (WIP commits are detached), it still points at
    // original_commit_hash, so this is essentially a no-op for HEAD.
    if let Some(branch_name) = &row.branch_name {
        let rx = wt_repo.update(cx, |repo, _cx| repo.change_branch(branch_name.clone()));
        if let Err(checkout_error) = rx.await.map_err(|e| anyhow!("{e}")).and_then(|r| r) {
            log::debug!(
                "change_branch('{}') failed: {checkout_error:#}, trying create_branch",
                branch_name
            );
            let rx = wt_repo.update(cx, |repo, _cx| {
                repo.create_branch(branch_name.clone(), None)
            });
            if let Ok(Err(error)) | Err(error) = rx.await.map_err(|e| anyhow!("{e}")) {
                log::warn!(
                    "Could not create branch '{}': {error} — \
                     restored worktree will be in detached HEAD state.",
                    branch_name
                );
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
pub async fn cleanup_thread_archived_worktrees(session_id: &acp::SessionId, cx: &mut AsyncApp) {
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));

    let archived_worktrees = store
        .read_with(cx, |store, cx| {
            store.get_archived_worktrees_for_thread(session_id.0.to_string(), cx)
        })
        .await;
    let archived_worktrees = match archived_worktrees {
        Ok(rows) => rows,
        Err(error) => {
            log::error!(
                "Failed to fetch archived worktrees for thread {}: {error:#}",
                session_id.0
            );
            return;
        }
    };

    if archived_worktrees.is_empty() {
        return;
    }

    if let Err(error) = store
        .read_with(cx, |store, cx| {
            store.unlink_thread_from_all_archived_worktrees(session_id.0.to_string(), cx)
        })
        .await
    {
        log::error!(
            "Failed to unlink thread {} from archived worktrees: {error:#}",
            session_id.0
        );
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
