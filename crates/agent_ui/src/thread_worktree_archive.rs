use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use agent_client_protocol as acp;
use anyhow::{Context as _, Result, anyhow};
use git::repository::{AskPassDelegate, CommitOptions, ResetMode};
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

/// The result of a successful [`persist_worktree_state`] call.
///
/// Carries exactly the information needed to roll back the persist via
/// [`rollback_persist`]: the DB row ID (to delete the record and the
/// corresponding `refs/archived-worktrees/<id>` git ref) and the staged
/// commit hash (to `git reset` back past both WIP commits).
pub struct PersistOutcome {
    pub archived_worktree_id: i64,
    pub staged_commit_hash: String,
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
    let receiver = repo.update(cx, |repo: &mut Repository, _cx| {
        repo.remove_worktree(root.root_path.clone(), false)
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
/// This is a multi-step operation:
/// 1. Records the original HEAD SHA.
/// 2. Creates WIP commit #1 ("staged") capturing the current index.
/// 3. Stages everything including untracked files, then creates WIP commit
///    #2 ("unstaged") capturing the full working directory.
/// 4. Creates a DB record (`ArchivedGitWorktree`) with all the SHAs, the
///    branch name, and both paths.
/// 5. Links every thread that references this worktree to the DB record.
/// 6. Creates a git ref (`refs/archived-worktrees/<id>`) on the main repo
///    pointing at the unstaged commit, preventing git from
///    garbage-collecting the WIP commits after the worktree is deleted.
///
/// Each step has rollback logic: if step N fails, steps 1..N-1 are undone.
/// On success, returns a [`PersistOutcome`] that can be passed to
/// [`rollback_persist`] if a later step in the archival pipeline fails.
pub async fn persist_worktree_state(root: &RootPlan, cx: &mut AsyncApp) -> Result<PersistOutcome> {
    let (worktree_repo, _temp_worktree_project) = match &root.worktree_repo {
        Some(worktree_repo) => (worktree_repo.clone(), None),
        None => find_or_create_repository(&root.root_path, cx).await?,
    };

    // Read original HEAD SHA before creating any WIP commits
    let original_commit_hash = worktree_repo
        .update(cx, |repo, _cx| repo.head_sha())
        .await
        .map_err(|_| anyhow!("head_sha canceled"))?
        .context("failed to read original HEAD SHA")?
        .context("HEAD SHA is None before WIP commits")?;

    // Create WIP commit #1 (staged state)
    let askpass = AskPassDelegate::new(cx, |_, _, _| {});
    let commit_rx = worktree_repo.update(cx, |repo, cx| {
        repo.commit(
            "WIP staged".into(),
            None,
            CommitOptions {
                allow_empty: true,
                ..Default::default()
            },
            askpass,
            cx,
        )
    });
    commit_rx
        .await
        .map_err(|_| anyhow!("WIP staged commit canceled"))??;

    // Read SHA after staged commit
    let staged_sha_result = worktree_repo
        .update(cx, |repo, _cx| repo.head_sha())
        .await
        .map_err(|_| anyhow!("head_sha canceled"))
        .and_then(|r| r.context("failed to read HEAD SHA after staged commit"))
        .and_then(|opt| opt.context("HEAD SHA is None after staged commit"));
    let staged_commit_hash = match staged_sha_result {
        Ok(sha) => sha,
        Err(error) => {
            let rx = worktree_repo.update(cx, |repo, cx| {
                repo.reset("HEAD~1".to_string(), ResetMode::Mixed, cx)
            });
            rx.await.ok().and_then(|r| r.log_err());
            return Err(error);
        }
    };

    // Stage all files including untracked
    let stage_rx = worktree_repo.update(cx, |repo, _cx| repo.stage_all_including_untracked());
    if let Err(error) = stage_rx
        .await
        .map_err(|_| anyhow!("stage all canceled"))
        .and_then(|inner| inner)
    {
        let rx = worktree_repo.update(cx, |repo, cx| {
            repo.reset("HEAD~1".to_string(), ResetMode::Mixed, cx)
        });
        rx.await.ok().and_then(|r| r.log_err());
        return Err(error.context("failed to stage all files including untracked"));
    }

    // Create WIP commit #2 (unstaged/untracked state)
    let askpass = AskPassDelegate::new(cx, |_, _, _| {});
    let commit_rx = worktree_repo.update(cx, |repo, cx| {
        repo.commit(
            "WIP unstaged".into(),
            None,
            CommitOptions {
                allow_empty: true,
                ..Default::default()
            },
            askpass,
            cx,
        )
    });
    if let Err(error) = commit_rx
        .await
        .map_err(|_| anyhow!("WIP unstaged commit canceled"))
        .and_then(|inner| inner)
    {
        let rx = worktree_repo.update(cx, |repo, cx| {
            repo.reset("HEAD~1".to_string(), ResetMode::Mixed, cx)
        });
        rx.await.ok().and_then(|r| r.log_err());
        return Err(error);
    }

    // Read HEAD SHA after WIP commits
    let head_sha_result = worktree_repo
        .update(cx, |repo, _cx| repo.head_sha())
        .await
        .map_err(|_| anyhow!("head_sha canceled"))
        .and_then(|r| r.context("failed to read HEAD SHA after WIP commits"))
        .and_then(|opt| opt.context("HEAD SHA is None after WIP commits"));
    let unstaged_commit_hash = match head_sha_result {
        Ok(sha) => sha,
        Err(error) => {
            let rx = worktree_repo.update(cx, |repo, cx| {
                repo.reset(format!("{}~1", staged_commit_hash), ResetMode::Mixed, cx)
            });
            rx.await.ok().and_then(|r| r.log_err());
            return Err(error);
        }
    };

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
            let rx = worktree_repo.update(cx, |repo, cx| {
                repo.reset(format!("{}~1", staged_commit_hash), ResetMode::Mixed, cx)
            });
            rx.await.ok().and_then(|r| r.log_err());
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
                    "Failed to delete archived worktree DB record during link rollback: {delete_error:#}"
                );
            }
            let rx = worktree_repo.update(cx, |repo, cx| {
                repo.reset(format!("{}~1", staged_commit_hash), ResetMode::Mixed, cx)
            });
            rx.await.ok().and_then(|r| r.log_err());
            return Err(error.context("failed to link thread to archived worktree"));
        }
    }

    // Create git ref on main repo (non-fatal)
    let ref_name = archived_worktree_ref_name(archived_worktree_id);
    let main_repo_result = find_or_create_repository(&root.main_repo_path, cx).await;
    match main_repo_result {
        Ok((main_repo, _temp_project)) => {
            let rx = main_repo.update(cx, |repo, _cx| {
                repo.update_ref(ref_name.clone(), unstaged_commit_hash.clone())
            });
            if let Err(error) = rx
                .await
                .map_err(|_| anyhow!("update_ref canceled"))
                .and_then(|r| r)
            {
                log::warn!(
                    "Failed to create ref {} on main repo (non-fatal): {error}",
                    ref_name
                );
            }
            // Keep _temp_project alive until after the await so the headless project isn't dropped mid-operation
            drop(_temp_project);
        }
        Err(error) => {
            log::warn!(
                "Could not find main repo to create ref {} (non-fatal): {error}",
                ref_name
            );
        }
    }

    Ok(PersistOutcome {
        archived_worktree_id,
        staged_commit_hash,
    })
}

/// Undoes a successful [`persist_worktree_state`] by resetting the WIP
/// commits, deleting the git ref on the main repo, and removing the DB
/// record.
pub async fn rollback_persist(outcome: &PersistOutcome, root: &RootPlan, cx: &mut AsyncApp) {
    // Undo WIP commits on the worktree repo
    if let Some(worktree_repo) = &root.worktree_repo {
        let rx = worktree_repo.update(cx, |repo, cx| {
            repo.reset(
                format!("{}~1", outcome.staged_commit_hash),
                ResetMode::Mixed,
                cx,
            )
        });
        rx.await.ok().and_then(|r| r.log_err());
    }

    // Delete the git ref on main repo
    if let Ok((main_repo, _temp_project)) =
        find_or_create_repository(&root.main_repo_path, cx).await
    {
        let ref_name = archived_worktree_ref_name(outcome.archived_worktree_id);
        let rx = main_repo.update(cx, |repo, _cx| repo.delete_ref(ref_name));
        rx.await.ok().and_then(|r| r.log_err());
        // Keep _temp_project alive until after the await so the headless project isn't dropped mid-operation
        drop(_temp_project);
    }

    // Delete the DB record
    let store = cx.update(|cx| ThreadMetadataStore::global(cx));
    if let Err(error) = store
        .read_with(cx, |store, cx| {
            store.delete_archived_worktree(outcome.archived_worktree_id, cx)
        })
        .await
    {
        log::error!("Failed to delete archived worktree DB record during rollback: {error:#}");
    }
}

/// Restores a previously archived worktree back to disk from its DB record.
///
/// Re-creates the git worktree (or adopts an existing directory), resets
/// past the two WIP commits to recover the original working directory
/// state, verifies HEAD matches the expected commit, and restores the
/// original branch if one was recorded.
pub async fn restore_worktree_via_git(
    row: &ArchivedGitWorktree,
    cx: &mut AsyncApp,
) -> Result<PathBuf> {
    let (main_repo, _temp_project) = find_or_create_repository(&row.main_repo_path, cx).await?;

    // Check if worktree path already exists on disk
    let worktree_path = &row.worktree_path;
    let app_state = current_app_state(cx).context("no app state available")?;
    let already_exists = app_state.fs.metadata(worktree_path).await?.is_some();

    if already_exists {
        let is_git_worktree =
            resolve_git_worktree_to_main_repo(app_state.fs.as_ref(), worktree_path)
                .await
                .is_some();

        if is_git_worktree {
            // Already a git worktree — another thread on the same worktree
            // already restored it. Reuse as-is.
            return Ok(worktree_path.clone());
        }

        // Path exists but isn't a git worktree. Ask git to adopt it.
        let rx = main_repo.update(cx, |repo, _cx| repo.repair_worktrees());
        rx.await
            .map_err(|_| anyhow!("worktree repair was canceled"))?
            .context("failed to repair worktrees")?;
    } else {
        // Create detached worktree at the unstaged commit
        let rx = main_repo.update(cx, |repo, _cx| {
            repo.create_worktree_detached(worktree_path.clone(), row.unstaged_commit_hash.clone())
        });
        rx.await
            .map_err(|_| anyhow!("worktree creation was canceled"))?
            .context("failed to create worktree")?;
    }

    // Get the worktree's repo entity
    let (wt_repo, _temp_wt_project) = find_or_create_repository(worktree_path, cx).await?;

    // Reset past the WIP commits to recover original state
    let mixed_reset_ok = {
        let rx = wt_repo.update(cx, |repo, cx| {
            repo.reset(row.staged_commit_hash.clone(), ResetMode::Mixed, cx)
        });
        match rx.await {
            Ok(Ok(())) => true,
            Ok(Err(error)) => {
                log::error!("Mixed reset to staged commit failed: {error:#}");
                false
            }
            Err(_) => {
                log::error!("Mixed reset to staged commit was canceled");
                false
            }
        }
    };

    let soft_reset_ok = if mixed_reset_ok {
        let rx = wt_repo.update(cx, |repo, cx| {
            repo.reset(row.original_commit_hash.clone(), ResetMode::Soft, cx)
        });
        match rx.await {
            Ok(Ok(())) => true,
            Ok(Err(error)) => {
                log::error!("Soft reset to original commit failed: {error:#}");
                false
            }
            Err(_) => {
                log::error!("Soft reset to original commit was canceled");
                false
            }
        }
    } else {
        false
    };

    // If either WIP reset failed, fall back to a mixed reset directly to
    // original_commit_hash so we at least land on the right commit.
    if !mixed_reset_ok || !soft_reset_ok {
        log::warn!(
            "WIP reset(s) failed (mixed_ok={mixed_reset_ok}, soft_ok={soft_reset_ok}); \
             falling back to mixed reset to original commit {}",
            row.original_commit_hash
        );
        let rx = wt_repo.update(cx, |repo, cx| {
            repo.reset(row.original_commit_hash.clone(), ResetMode::Mixed, cx)
        });
        match rx.await {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                return Err(error.context(format!(
                    "fallback reset to original commit {} also failed",
                    row.original_commit_hash
                )));
            }
            Err(_) => {
                return Err(anyhow!(
                    "fallback reset to original commit {} was canceled",
                    row.original_commit_hash
                ));
            }
        }
    }

    // Verify HEAD is at original_commit_hash
    let current_head = wt_repo
        .update(cx, |repo, _cx| repo.head_sha())
        .await
        .map_err(|_| anyhow!("post-restore head_sha was canceled"))?
        .context("failed to read HEAD after restore")?
        .context("HEAD is None after restore")?;

    if current_head != row.original_commit_hash {
        anyhow::bail!(
            "After restore, HEAD is at {current_head} but expected {}. \
             The worktree may be in an inconsistent state.",
            row.original_commit_hash
        );
    }

    // Restore the branch
    if let Some(branch_name) = &row.branch_name {
        // Check if the branch exists and points at original_commit_hash.
        // If it does, switch to it. If not, create a new branch there.
        let rx = wt_repo.update(cx, |repo, _cx| repo.change_branch(branch_name.clone()));
        if matches!(rx.await, Ok(Ok(()))) {
            // Verify the branch actually points at original_commit_hash after switching
            let head_after_switch = wt_repo
                .update(cx, |repo, _cx| repo.head_sha())
                .await
                .ok()
                .and_then(|r| r.ok())
                .flatten();

            if head_after_switch.as_deref() != Some(&row.original_commit_hash) {
                // Branch exists but doesn't point at the right commit.
                // Switch back to detached HEAD at original_commit_hash.
                log::warn!(
                    "Branch '{}' exists but points at {:?}, not {}. Creating fresh branch.",
                    branch_name,
                    head_after_switch,
                    row.original_commit_hash
                );
                let rx = wt_repo.update(cx, |repo, cx| {
                    repo.reset(row.original_commit_hash.clone(), ResetMode::Mixed, cx)
                });
                rx.await.ok().and_then(|r| r.log_err());
                // Delete the old branch and create fresh
                let rx = wt_repo.update(cx, |repo, _cx| {
                    repo.create_branch(branch_name.clone(), None)
                });
                rx.await.ok().and_then(|r| r.log_err());
            }
        } else {
            // Branch doesn't exist or can't be switched to — create it.
            let rx = wt_repo.update(cx, |repo, _cx| {
                repo.create_branch(branch_name.clone(), None)
            });
            if let Ok(Err(error)) | Err(error) = rx.await.map_err(|e| anyhow::anyhow!("{e}")) {
                log::warn!(
                    "Could not create branch '{}': {error} — \
                     restored worktree is in detached HEAD state.",
                    branch_name
                );
            }
        }
    }

    Ok(worktree_path.clone())
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
