use std::{
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result, anyhow};
use fs::{Fs, RemoveOptions, RenameOptions};
use git::repository::NotAWorktreeError;
use gpui::{App, AppContext as _, AsyncApp, Entity, Task};
use project::{
    LocalProjectFlags, Project, WorktreeId,
    git_store::{Repository, worktrees_directory_for_repo},
    project_settings::ProjectSettings,
};
use remote::{RemoteConnectionOptions, same_remote_connection_identity};
use settings::Settings;
use util::{ResultExt, paths::PathStyle};
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
    /// Remote connection options for the project that owns this worktree,
    /// used to create temporary remote projects when the main repo isn't
    /// loaded in any open workspace.
    pub remote_connection: Option<RemoteConnectionOptions>,
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
fn worktrees_base_for_repo(
    main_repo_path: &Path,
    path_style: PathStyle,
    cx: &App,
) -> Option<PathBuf> {
    let setting = &ProjectSettings::get_global(cx).git.worktree_directory;
    worktrees_directory_for_repo(main_repo_path, setting, path_style).log_err()
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
    remote_connection: Option<&RemoteConnectionOptions>,
    workspaces: &[Entity<Workspace>],
    cx: &App,
) -> Option<RootPlan> {
    let path = path.to_path_buf();

    let matches_target_connection = |project: &Entity<Project>, cx: &App| {
        same_remote_connection_identity(
            project.read(cx).remote_connection_options(cx).as_ref(),
            remote_connection,
        )
    };

    let affected_projects = workspaces
        .iter()
        .filter_map(|workspace| {
            let project = workspace.read(cx).project().clone();
            if !matches_target_connection(&project, cx) {
                return None;
            }
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
        .filter(|workspace| matches_target_connection(workspace.read(cx).project(), cx))
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
    let main_repo_path = linked_snapshot.main_worktree_abs_path()?.to_path_buf();

    // Only archive worktrees that live inside the Zed-managed worktrees
    // directory (configured via `git.worktree_directory`). Worktrees the
    // user created outside that directory should be left untouched.
    let worktrees_base = worktrees_base_for_repo(&main_repo_path, linked_snapshot.path_style, cx)?;
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
        remote_connection: remote_connection.cloned(),
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

    let (repo, project) =
        find_or_create_repository(&root.main_repo_path, root.remote_connection.as_ref(), cx)
            .await?;

    // `Repository::remove_worktree` with `force = true` deletes the working
    // directory before running `git worktree remove --force`, so there's no
    // need to touch the filesystem here. For remote projects that cleanup
    // runs on the headless server via the `GitRemoveWorktree` RPC, which is
    // the only code path with access to the remote machine's filesystem.
    let receiver = repo.update(cx, |repo: &mut Repository, _cx| {
        repo.remove_worktree(root.root_path.clone(), true)
    });
    let result = receiver
        .await
        .map_err(|_| anyhow!("git worktree metadata cleanup was canceled"))?;
    // `project` may be a live workspace project or a temporary one created
    // by `find_or_create_repository`. In the temporary case we must keep it
    // alive until the repo removes the worktree
    drop(project);
    result.context("git worktree metadata cleanup failed")?;
    Ok(())
}

/// Finds a live `Repository` entity for the given path, or creates a temporary
/// project to obtain one.
///
/// `Repository` entities can only be obtained through a `Project` because
/// `GitStore` (which creates and manages `Repository` entities) is owned by
/// `Project`. When no open workspace contains the repo we need, we spin up a
/// headless project just to get a `Repository` handle. For local paths this is
/// a `Project::local`; for remote paths we build a `Project::remote` through
/// the connection pool (reusing the existing SSH transport), which requires
/// the caller to pass the matching `RemoteConnectionOptions` so we only match
/// and fall back onto projects that share the same remote identity. The
/// caller keeps the returned `Entity<Project>` alive for the duration of the
/// git operations, then drops it.
///
/// Future improvement: decoupling `GitStore` from `Project` so that
/// `Repository` entities can be created standalone would eliminate this
/// temporary-project workaround.
async fn find_or_create_repository(
    repo_path: &Path,
    remote_connection: Option<&RemoteConnectionOptions>,
    cx: &mut AsyncApp,
) -> Result<(Entity<Repository>, Entity<Project>)> {
    let repo_path_owned = repo_path.to_path_buf();
    let remote_connection_owned = remote_connection.cloned();

    // First, try to find a live repository in any open workspace whose
    // remote connection matches (so a local `/project` and a remote
    // `/project` are not confused).
    let live_repo = cx.update(|cx| {
        all_open_workspaces(cx)
            .into_iter()
            .filter_map(|workspace| {
                let project = workspace.read(cx).project().clone();
                let project_connection = project.read(cx).remote_connection_options(cx);
                if !same_remote_connection_identity(
                    project_connection.as_ref(),
                    remote_connection_owned.as_ref(),
                ) {
                    return None;
                }
                Some((
                    project
                        .read(cx)
                        .repositories(cx)
                        .values()
                        .find(|repo| {
                            repo.read(cx).snapshot().work_directory_abs_path.as_ref()
                                == repo_path_owned.as_path()
                        })
                        .cloned()?,
                    project.clone(),
                ))
            })
            .next()
    });

    if let Some((repo, project)) = live_repo {
        return Ok((repo, project));
    }

    let app_state =
        current_app_state(cx).context("no app state available for temporary project")?;

    // For remote paths, create a fresh RemoteClient through the connection
    // pool (reusing the existing SSH transport) and build a temporary
    // remote project. Each RemoteClient gets its own server-side headless
    // project, so there are no RPC routing conflicts with other projects.
    let temp_project = if let Some(connection) = remote_connection_owned {
        let remote_client = cx
            .update(|cx| {
                if !remote::has_active_connection(&connection, cx) {
                    anyhow::bail!("cannot open repository on disconnected remote machine");
                }
                Ok(remote_connection::connect_reusing_pool(connection, cx))
            })?
            .await?
            .context("remote connection was canceled")?;

        cx.update(|cx| {
            Project::remote(
                remote_client,
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                false,
                cx,
            )
        })
    } else {
        cx.update(|cx| {
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
        })
    };

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
    Ok((repo, temp_project))
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
    let (main_repo, _temp_project) =
        find_or_create_repository(&root.main_repo_path, root.remote_connection.as_ref(), cx)
            .await
            .context("could not open main repo to create archive ref")?;
    let rx = main_repo.update(cx, |repo, _cx| {
        repo.update_ref(ref_name.clone(), unstaged_commit_hash.clone())
    });
    rx.await
        .map_err(|_| anyhow!("update_ref canceled"))
        .and_then(|r| r)
        .with_context(|| format!("failed to create ref {ref_name} on main repo"))?;
    // `_temp_project` is held until the end of this scope so that the
    // temporary project (when one was created by
    // `find_or_create_repository`) stays alive while the repo runs git
    // commands; the leading underscore already enforces end-of-scope drop.

    Ok(archived_worktree_id)
}

/// Undoes a successful [`persist_worktree_state`] by deleting the git ref
/// on the main repo and removing the DB record. Since the WIP commits are
/// detached (they don't move any branch), no git reset is needed — the
/// commits will be garbage-collected once the ref is removed.
pub async fn rollback_persist(archived_worktree_id: i64, root: &RootPlan, cx: &mut AsyncApp) {
    // Delete the git ref on main repo
    if let Ok((main_repo, _temp_project)) =
        find_or_create_repository(&root.main_repo_path, root.remote_connection.as_ref(), cx).await
    {
        let ref_name = archived_worktree_ref_name(archived_worktree_id);
        let rx = main_repo.update(cx, |repo, _cx| repo.delete_ref(ref_name));
        rx.await.ok().and_then(|r| r.log_err());
        // `_temp_project` lives to end of this block so a temporary
        // project (if `find_or_create_repository` made one) stays alive
        // through the await above.
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
///
/// **Destructive**: the final step (`restore_archive_checkpoint`) clobbers the
/// working directory unconditionally via `git read-tree --reset -u`. If the
/// path has any pre-existing content (a non-empty directory, a file, or a
/// symlink) it is moved aside into a `zed-restore-backup-<uuid>` directory
/// before the rest of the destructive work runs. We try to place the backup
/// next to `worktree_path` so the rename stays on the same filesystem
/// (atomic and fast), and fall back to the system temp directory if a
/// sibling cannot be created. If a later step fails, the backup is moved
/// back over `worktree_path` so the user does not lose their content. On
/// success the backup directory is deleted asynchronously so a multi-GB
/// cleanup does not block the caller.
///
/// Callers MUST first call [`restore_would_overwrite`] and confirm with the
/// user before invoking this function — there is no preflight or refusal
/// mode here, only a best-effort backup of whatever happens to be at the
/// path when we run.
pub async fn restore_worktree_via_git(
    row: &ArchivedGitWorktree,
    remote_connection: Option<&RemoteConnectionOptions>,
    cx: &mut AsyncApp,
) -> Result<PathBuf> {
    if remote_connection.is_some() {
        anyhow::bail!("restoring archived worktrees on remote machines is not yet supported");
    }
    let app_state = current_app_state(cx).context("no app state available")?;
    let fs = app_state.fs.as_ref();
    let worktree_path = &row.worktree_path;

    let (main_repo, _temp_project) =
        find_or_create_repository(&row.main_repo_path, remote_connection, cx).await?;

    // Always restore by recreating the worktree from scratch. This collapses
    // every messy intermediate state into one clean flow:
    //
    //   - Path missing or empty dir, no registration:  plain add.
    //   - Path missing or empty dir, stale registration: scoped remove → add.
    //   - Path present with content, no registration
    //     (the original Windows file-lock bug):        rename → add.
    //   - Path present with content, stale registration: rename → scoped remove → add.
    //   - Path present as a fully valid worktree:      rename → scoped remove → add.
    //
    // Any pre-existing content at `worktree_path` is moved aside into a
    // backup directory rather than deleted up-front. If any destructive
    // step below fails, [`rollback_backup`] restores the backup over
    // `worktree_path` so the user does not lose content they confirmed
    // they wanted overwritten only on the assumption that the archived
    // state would replace it. On success the backup is deleted at the end
    // of this function.
    //
    // An empty directory at `worktree_path` is left in place: `git worktree
    // add` is happy to adopt one and we don't want to round-trip through a
    // backup for it. This matches the preflight behaviour in
    // [`restore_would_overwrite`] (which also treats empty dirs as
    // "no content") so the destructive pass cannot silently delete
    // something the user wasn't warned about.
    let session = BackupSession::take(fs, worktree_path).await?;

    // From here on, every destructive step either (a) calls
    // `session.try_step(...)` so a failure rolls the backup back, or
    // (b) handles rollback inline via
    // `session.rollback_with_annotation(...)` because it shares the `cx`
    // borrow with its own cleanup.

    // Drop any stale registration in the main repo. Without this, the
    // `git worktree add` below would fail with "already assigned but
    // missing".
    session
        .try_step(async {
            remove_worktree_registration_if_present(&main_repo, worktree_path, cx)
                .await
                .with_context(|| {
                    format!(
                        "failed to remove stale worktree registration for '{}'",
                        worktree_path.display()
                    )
                })
        })
        .await?;

    // Recreate the worktree at the original commit. The branch still
    // points here because archival used detached commits.
    session
        .try_step(async {
            create_worktree_with_partial_cleanup(
                fs,
                &main_repo,
                worktree_path,
                &row.original_commit_hash,
                cx,
            )
            .await
        })
        .await?;

    // Resolve the worktree's own `Repository` entity. Can't use
    // `try_step` because the rollback path needs the same `cx` borrow
    // the step itself uses; call into the session directly instead.
    let wt_repo = match find_or_create_repository(worktree_path, remote_connection, cx).await {
        Ok((repo, _temp_wt_project)) => repo,
        Err(error) => {
            remove_new_worktree_on_error(fs, &main_repo, worktree_path, cx).await;
            return Err(session.rollback_with_annotation(error).await);
        }
    };

    // Best-effort branch checkout. Non-fatal: if the branch was deleted
    // or has moved since archival, we fall back to a detached checkout at
    // the original commit so the worktree's filesystem state matches what
    // was captured.
    if let Some(branch_name) = &row.branch_name {
        checkout_branch_after_restore(&wt_repo, &main_repo, branch_name, row, cx).await;
    }

    // Reconstruct staged/unstaged state from the WIP commit trees.
    // Inlined for the same `cx`-borrow reason as `wt_repo` above.
    if let Err(error) = run_restore_checkpoint(&wt_repo, row, cx).await {
        remove_new_worktree_on_error(fs, &main_repo, worktree_path, cx).await;
        return Err(session.rollback_with_annotation(error).await);
    }

    session.commit_async(app_state.fs.clone(), cx);

    Ok(worktree_path.clone())
}

/// Fixed leaf filename inside a backup directory where the original
/// `worktree_path` entry is renamed to. Kept as a single constant so the
/// rename target and the rollback rename source can never drift.
const BACKUP_ENTRY_NAME: &str = "worktree";

/// Pre-existing content at `worktree_path` that was moved aside before the
/// destructive parts of [`restore_worktree_via_git`] ran. If anything goes
/// wrong, [`rollback_backup`] uses this to put the user's content back.
struct Backup {
    /// The backup directory holding the moved content, always created
    /// as a sibling of `worktree_path` so the rename stays same-volume.
    /// The moved entry lives at `dir/BACKUP_ENTRY_NAME` — see
    /// [`Backup::target`].
    dir: PathBuf,
}

impl Backup {
    /// Path inside [`Backup::dir`] where the original `worktree_path` entry
    /// was renamed to. Computed rather than stored so the two can't drift.
    fn target(&self) -> PathBuf {
        self.dir.join(BACKUP_ENTRY_NAME)
    }
}

/// Owns the entire "move aside any existing content, undo on failure,
/// clean up on success" lifecycle for a single call to
/// [`restore_worktree_via_git`]. Bundles the `fs` / `worktree_path` /
/// `backup` triple so individual restore steps don't have to thread them
/// through.
///
/// Typical use:
///
/// 1. `let session = BackupSession::take(fs, worktree_path).await?;`
/// 2. `session.try_step(some_destructive_step()).await?;` for each step
///    that doesn't need its own access to the borrow held by the step.
/// 3. For steps that share a `cx` borrow with their cleanup, call
///    [`Self::rollback_with_annotation`] directly on the error path.
/// 4. On success, `session.commit_async(fs_owned, cx)` schedules
///    background cleanup of the backup directory. Dropping the session
///    without `commit_async` will leak the backup directory — on
///    purpose, since reaching that code path means something panicked.
struct BackupSession<'a> {
    fs: &'a dyn Fs,
    worktree_path: &'a Path,
    backup: Option<Backup>,
}

impl<'a> BackupSession<'a> {
    /// Captures any pre-existing content at `worktree_path` into a
    /// sibling backup directory and returns a session that can roll it
    /// back. If the path was already missing or an empty directory, the
    /// session holds no backup and rollback / commit become no-ops.
    async fn take(fs: &'a dyn Fs, worktree_path: &'a Path) -> Result<Self> {
        let backup = take_backup_if_needed(fs, worktree_path).await?;
        Ok(Self {
            fs,
            worktree_path,
            backup,
        })
    }

    /// Awaits `step` and rolls back the backup on failure, returning the
    /// original error annotated with the backup path if rollback strands
    /// the user's content for manual recovery.
    async fn try_step<F>(&self, step: F) -> Result<()>
    where
        F: Future<Output = Result<()>>,
    {
        match step.await {
            Ok(()) => Ok(()),
            Err(error) => Err(self.rollback_with_annotation(error).await),
        }
    }

    /// Rolls back the backup (if any) and returns the original `error`
    /// annotated with the backup path if rollback couldn't put the
    /// user's content back. Use directly when a destructive step shares
    /// its `cx` borrow with the cleanup that has to follow the failure.
    async fn rollback_with_annotation(&self, error: anyhow::Error) -> anyhow::Error {
        match rollback_backup(self.fs, self.backup.as_ref(), self.worktree_path, &error).await {
            Some(path) => annotate_with_stranded_backup(error, path),
            None => error,
        }
    }

    /// Consumes the session and schedules cleanup of the (now-unused)
    /// backup directory on a background task. Failures are logged but
    /// not surfaced; the `zed-restore-backup-<uuid>` naming makes any
    /// orphans easy to spot manually.
    fn commit_async(self, fs: Arc<dyn Fs>, cx: &mut AsyncApp) {
        schedule_backup_cleanup(fs, self.backup, cx);
    }
}

/// Detects whether `worktree_path` currently holds content the caller has
/// agreed to overwrite (anything that isn't a missing path or empty
/// directory) and, if so, renames it aside into a freshly-created backup
/// directory. Returns `None` when there was nothing worth backing up.
///
/// Used by [`BackupSession::take`]; not called directly from the restore
/// flow.
async fn take_backup_if_needed(fs: &dyn Fs, worktree_path: &Path) -> Result<Option<Backup>> {
    if !worktree_path_has_content(fs, worktree_path).await? {
        return Ok(None);
    }
    let backup_dir = create_backup_dir(fs, worktree_path).await?;
    let backup = Backup { dir: backup_dir };
    let target = backup.target();
    // `rename` works for both directories and files, so we don't need to
    // dispatch on the entry kind. A stray regular file or symlink at
    // `worktree_path` is moved aside the same way as a directory.
    fs.rename(
        worktree_path,
        &target,
        RenameOptions {
            overwrite: false,
            ignore_if_exists: false,
            create_parents: false,
        },
    )
    .await
    .with_context(|| {
        format!(
            "failed to move existing path '{}' to backup '{}'",
            worktree_path.display(),
            target.display()
        )
    })?;
    Ok(Some(backup))
}

/// Runs `git worktree add --detach` at `worktree_path` and, on failure,
/// cleans up any partial directory or stale registration so the caller's
/// rollback rename has a free target. Returns the underlying error when
/// creation fails.
async fn create_worktree_with_partial_cleanup(
    fs: &dyn Fs,
    main_repo: &Entity<Repository>,
    worktree_path: &Path,
    original_commit_hash: &str,
    cx: &mut AsyncApp,
) -> Result<()> {
    let create_rx = main_repo.update(cx, |repo, _cx| {
        repo.create_worktree_detached(
            worktree_path.to_path_buf(),
            original_commit_hash.to_string(),
        )
    });
    let create_result = match create_rx.await {
        Ok(result) => result.context("failed to create worktree"),
        Err(_) => Err(anyhow!("worktree creation was canceled")),
    };
    if let Err(error) = create_result {
        // `create_worktree_detached` may have left a partial directory
        // and/or a stale registration behind; `remove_new_worktree_on_error`
        // clears both so the caller's rollback rename has somewhere to put
        // the backup back.
        remove_new_worktree_on_error(fs, main_repo, worktree_path, cx).await;
        return Err(error);
    }
    Ok(())
}

/// Best-effort checkout of `branch_name` in the freshly restored worktree.
///
/// * If the branch was deleted while the thread was archived, recreates it.
/// * If the branch has moved since archival, falls back to a detached
///   checkout at the original commit so the worktree's filesystem state
///   matches what was captured.
///
/// All failures are logged; nothing here is fatal to the restore.
async fn checkout_branch_after_restore(
    wt_repo: &Entity<Repository>,
    main_repo: &Entity<Repository>,
    branch_name: &str,
    row: &ArchivedGitWorktree,
    cx: &mut AsyncApp,
) {
    let checkout_result = wt_repo
        .update(cx, |repo, _cx| repo.change_branch(branch_name.to_string()))
        .await;

    match checkout_result.map_err(|e| anyhow!("{e}")).flatten() {
        Ok(()) => {
            // Branch checkout succeeded. Check whether the branch has moved
            // since archival by comparing HEAD to the expected SHA.
            let head_sha = wt_repo
                .update(cx, |repo, _cx| repo.head_sha())
                .await
                .map_err(|e| anyhow!("{e}"))
                .and_then(|r| r);

            match head_sha {
                Ok(Some(sha)) if sha == row.original_commit_hash => {
                    // Branch still points at the original commit; done.
                }
                Ok(Some(sha)) => {
                    // Branch has moved — restore in detached HEAD at the
                    // original commit so the worktree state matches what
                    // was captured.
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
            // change_branch failed, most likely because the branch was
            // deleted. Users often delete old branches; try to recreate.
            log::debug!(
                "change_branch('{branch_name}') failed: {checkout_error:#}, trying create_branch"
            );
            let create_result = wt_repo
                .update(cx, |repo, _cx| {
                    repo.create_branch(branch_name.to_string(), None)
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

/// Runs the staged/unstaged read-tree pair from the WIP commit trees to
/// reconstruct the worktree's pre-archive index + working-tree state.
/// `read-tree --reset -u` applies the unstaged tree (including deletions)
/// to the working directory, then a bare `read-tree` sets the index to
/// the staged tree without touching the working directory.
async fn run_restore_checkpoint(
    wt_repo: &Entity<Repository>,
    row: &ArchivedGitWorktree,
    cx: &mut AsyncApp,
) -> Result<()> {
    let restore_rx = wt_repo.update(cx, |repo, _cx| {
        repo.restore_archive_checkpoint(
            row.staged_commit_hash.clone(),
            row.unstaged_commit_hash.clone(),
        )
    });
    restore_rx
        .await
        .map_err(|_| anyhow!("restore_archive_checkpoint canceled"))
        .and_then(|r| r)
        .context("failed to restore archive checkpoint")
}

/// Schedules cleanup of the (now-unused after success) backup directory
/// on a background task so the caller doesn't pay for a potentially
/// multi-GB cleanup synchronously. Failures are logged but not surfaced;
/// the `zed-restore-backup-<uuid>` naming makes any orphans easy to spot
/// manually.
fn schedule_backup_cleanup(fs: Arc<dyn Fs>, backup: Option<Backup>, cx: &mut AsyncApp) {
    let Some(backup) = backup else {
        return;
    };
    cx.background_spawn(async move {
        if let Err(error) = fs
            .remove_dir(
                &backup.dir,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
        {
            log::warn!(
                "failed to clean up backup directory '{}' after successful restore: {error:#}",
                backup.dir.display()
            );
        }
    })
    .detach();
}

/// If the main repo has a worktree registration pointing at
/// `worktree_path`, calls `git worktree remove --force` on it. Otherwise
/// returns `Ok(())`.
///
/// `GitRepository::remove_worktree` already pre-checks the registry and
/// returns [`NotAWorktreeError`] when nothing is registered at the path,
/// so we just call it and treat that specific error as a no-op. (Note: the
/// RPC arm of `Repository::remove_worktree` does not preserve the error
/// type; restoring archived worktrees rejects remote connections at its
/// entry point, so we only ever reach this helper for local repos.)
async fn remove_worktree_registration_if_present(
    main_repo: &Entity<Repository>,
    worktree_path: &Path,
    cx: &mut AsyncApp,
) -> Result<()> {
    let remove_rx = main_repo.update(cx, |repo, _cx| {
        repo.remove_worktree(worktree_path.to_path_buf(), true)
    });
    match remove_rx
        .await
        .map_err(|_| anyhow!("worktree remove was canceled"))
        .and_then(|r| r)
    {
        Ok(()) => Ok(()),
        Err(error) => {
            if error.downcast_ref::<NotAWorktreeError>().is_some() {
                log::debug!(
                    "no stale worktree registration to clean up for '{}'",
                    worktree_path.display()
                );
                Ok(())
            } else {
                Err(error)
            }
        }
    }
}

/// Creates the backup directory used by [`restore_worktree_via_git`].
///
/// Always places it as a sibling of `worktree_path` so the subsequent
/// rename stays on the same filesystem (atomic, and crucially does not
/// hit `EXDEV`/`ERROR_NOT_SAME_DEVICE`). Returns an error if a sibling
/// cannot be created — we used to fall back to `std::env::temp_dir()`,
/// but on Linux (tmpfs `/tmp`) and Windows (project on `D:` vs temp on
/// `C:`) that fallback would silently cross volumes, and `Fs::rename`
/// has no copy fallback for cross-device renames. Failing here, before
/// any destructive step in the restore runs, surfaces the real problem
/// (typically a read-only parent) instead of leaving the user with a
/// half-finished restore.
async fn create_backup_dir(fs: &dyn Fs, worktree_path: &Path) -> Result<PathBuf> {
    let parent = worktree_path.parent().with_context(|| {
        format!(
            "cannot create backup for worktree path '{}': path has no parent directory",
            worktree_path.display()
        )
    })?;
    let sibling = parent.join(format!("zed-restore-backup-{}", uuid::Uuid::new_v4()));
    fs.create_dir(&sibling).await.with_context(|| {
        format!(
            "failed to create backup directory '{}'; check that the parent directory is writable",
            sibling.display()
        )
    })?;
    Ok(sibling)
}

/// Restores the user's pre-existing content from a backup created by
/// [`restore_worktree_via_git`] back to `worktree_path`, then deletes the
/// now-empty backup directory.
///
/// Returns `Some(path)` if the rollback could **not** put the user's
/// content back (e.g. unexpected new content at `worktree_path`, or the
/// rename itself failed) and the content remains stranded at that path
/// for manual recovery. Returns `None` if the rollback succeeded (or
/// there was no backup to roll back). Callers should weave the returned
/// path into the user-facing error so the toast can tell the user where
/// their files are.
///
/// On any rollback failure we also log loudly with the original error and
/// the backup path. The original `restore_worktree_via_git` error is the
/// user-visible cause; the rollback error is intentionally not propagated.
#[must_use = "if rollback strands the backup, callers must surface the path to the user"]
async fn rollback_backup(
    fs: &dyn Fs,
    backup: Option<&Backup>,
    worktree_path: &Path,
    original_error: &anyhow::Error,
) -> Option<PathBuf> {
    let backup = backup?;
    let target = backup.target();
    if let Ok(Some(metadata)) = fs.metadata(worktree_path).await {
        // Treat symlinks as content even if they happen to resolve to a
        // directory: the worktree path was guaranteed to be either
        // missing or an empty directory when we moved the original
        // aside, so a symlink here is unexpected and could point
        // anywhere. Mirrors `worktree_path_has_content` so the
        // preflight and rollback agree on what counts as content.
        let emptiness = if metadata.is_symlink {
            DirEmptiness::HasContent
        } else if metadata.is_dir {
            classify_dir_emptiness(fs, worktree_path).await
        } else {
            // Anything that's not a directory at the worktree path counts
            // as unexpected content for rollback purposes (the path was
            // empty / missing when we moved the original aside).
            DirEmptiness::HasContent
        };
        match emptiness {
            DirEmptiness::Empty => {
                if let Err(clear_error) = fs
                    .remove_dir(
                        worktree_path,
                        RemoveOptions {
                            recursive: false,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await
                {
                    log::warn!(
                        "failed to clear empty '{}' before rollback rename: {clear_error:#}",
                        worktree_path.display()
                    );
                }
            }
            DirEmptiness::HasContent => {
                log::error!(
                    "cannot rollback: '{}' has unexpected content after restore failure; \
                     original error: {original_error:#}; \
                     user's pre-existing content remains at '{}' for manual recovery",
                    worktree_path.display(),
                    target.display(),
                );
                return Some(target);
            }
            DirEmptiness::Unknown(read_error) => {
                // We couldn't tell if the directory is empty (e.g. a
                // transient permission error reading it). Don't blindly
                // delete it — it might hold user content the partial
                // restore created. Surface this as its own log line so the
                // distinction from "non-empty" is clear when triaging.
                log::error!(
                    "cannot rollback: failed to read '{}' to check whether it's empty: {read_error:#}; \
                     original error: {original_error:#}; \
                     user's pre-existing content remains at '{}' for manual recovery",
                    worktree_path.display(),
                    target.display(),
                );
                return Some(target);
            }
        }
    }
    if let Err(rollback_error) = fs
        .rename(
            &target,
            worktree_path,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
                create_parents: false,
            },
        )
        .await
    {
        log::error!(
            "failed to restore backup '{}' to '{}' after restore error: {rollback_error:#}; \
             original restore error: {original_error:#}; \
             user content remains at '{}' for manual recovery",
            target.display(),
            worktree_path.display(),
            target.display(),
        );
        return Some(target);
    }
    if let Err(cleanup_error) = fs
        .remove_dir(
            &backup.dir,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await
    {
        log::warn!(
            "failed to clean up empty backup directory '{}' after rollback: {cleanup_error:#}",
            backup.dir.display()
        );
    }
    None
}

/// Wraps the original restore error with a message pointing the user at
/// `stranded_path` for manual recovery. Used by callers of
/// [`rollback_backup`] to surface the backup path through the existing
/// `anyhow` error chain (which the sidebar toast renders verbatim).
fn annotate_with_stranded_backup(error: anyhow::Error, stranded_path: PathBuf) -> anyhow::Error {
    error.context(format!(
        "Your pre-existing files have been preserved at '{}' for manual recovery.",
        stranded_path.display()
    ))
}

/// Result of attempting to determine whether a directory is empty for
/// rollback purposes. Distinguishes "definitely has content" from "could
/// not tell", which require different handling: the former means there's
/// new content we shouldn't blow away; the latter is a read failure where
/// the safer choice is also to leave the directory alone.
enum DirEmptiness {
    Empty,
    HasContent,
    Unknown(anyhow::Error),
}

async fn classify_dir_emptiness(fs: &dyn Fs, path: &Path) -> DirEmptiness {
    use futures::stream::StreamExt as _;

    match fs.read_dir(path).await {
        Ok(mut entries) => {
            if entries.next().await.is_some() {
                DirEmptiness::HasContent
            } else {
                DirEmptiness::Empty
            }
        }
        Err(error) => DirEmptiness::Unknown(error),
    }
}

/// Returns whether restoring this archived worktree would clobber any
/// pre-existing content on disk at the worktree's path.
///
/// Callers must invoke this **before** [`restore_worktree_via_git`] and prompt
/// the user for confirmation if it returns `true`, since the restore will
/// otherwise destroy that content.
pub async fn restore_would_overwrite(
    row: &ArchivedGitWorktree,
    remote_connection: Option<&RemoteConnectionOptions>,
    cx: &mut AsyncApp,
) -> Result<bool> {
    if remote_connection.is_some() {
        anyhow::bail!("restoring archived worktrees on remote machines is not yet supported");
    }
    let app_state = current_app_state(cx).context("no app state available")?;
    worktree_path_has_content(app_state.fs.as_ref(), &row.worktree_path).await
}

/// Returns whether the worktree path has any content that a restore would
/// destroy. A path that doesn't exist or that is an empty directory has no
/// content; anything else (a non-empty directory, or a file at this path)
/// counts as content.
async fn worktree_path_has_content(fs: &dyn Fs, path: &Path) -> Result<bool> {
    use futures::stream::StreamExt;

    let Some(metadata) = fs.metadata(path).await? else {
        return Ok(false);
    };

    if metadata.is_symlink {
        return Ok(true);
    }

    if !metadata.is_dir {
        return Ok(true);
    }

    let mut entries = fs.read_dir(path).await?;
    Ok(entries.next().await.is_some())
}

async fn remove_new_worktree_on_error(
    fs: &dyn Fs,
    main_repo: &Entity<Repository>,
    worktree_path: &Path,
    cx: &mut AsyncApp,
) {
    // Use the registration check rather than calling `remove_worktree`
    // directly so a rollback at a point where no registration exists
    // (e.g. when `create_worktree_detached` failed before git could
    // register the path) doesn't log a misleading `NotAWorktreeError`.
    if let Err(error) = remove_worktree_registration_if_present(main_repo, worktree_path, cx).await
    {
        log::warn!(
            "failed to clean up worktree registration for '{}' during rollback: {error:#}",
            worktree_path.display()
        );
    }

    // `git worktree add` creates the directory before registering it, so a
    // failure between mkdir and registration can leave a partial directory
    // with no registry entry. The registration cleanup above is a no-op in
    // that case; without this fs-level cleanup, the partial directory
    // would strand the rollback's backup rename (`fs.rename` with
    // `overwrite: false` fails if the target path exists), leaving the
    // user's pre-existing content in the `zed-restore-backup-<uuid>`
    // directory and forcing manual recovery.
    if let Err(error) = fs
        .remove_dir(
            worktree_path,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: true,
            },
        )
        .await
    {
        log::warn!(
            "failed to remove partial worktree directory '{}' during rollback: {error:#}",
            worktree_path.display()
        );
    }
}

/// Deletes the git ref and DB records for a single archived worktree.
/// Used when an archived worktree is no longer referenced by any thread.
pub async fn cleanup_archived_worktree_record(
    row: &ArchivedGitWorktree,
    remote_connection: Option<&RemoteConnectionOptions>,
    cx: &mut AsyncApp,
) {
    // Delete the git ref from the main repo
    if let Ok((main_repo, _temp_project)) =
        find_or_create_repository(&row.main_repo_path, remote_connection, cx).await
    {
        let ref_name = archived_worktree_ref_name(row.id);
        let rx = main_repo.update(cx, |repo, _cx| repo.delete_ref(ref_name));
        match rx.await {
            Ok(Ok(())) => {}
            Ok(Err(error)) => log::warn!("Failed to delete archive ref: {error}"),
            Err(_) => log::warn!("Archive ref deletion was canceled"),
        }
        // `_temp_project` lives to end of this block so a temporary
        // project (if `find_or_create_repository` made one) stays alive
        // through the await above.
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
    let remote_connection = store.read_with(cx, |store, _cx| {
        store
            .entry(thread_id)
            .and_then(|t| t.remote_connection.clone())
    });

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
                cleanup_archived_worktree_record(row, remote_connection.as_ref(), cx).await;
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
    use remote::SshConnectionOptions;
    use serde_json::json;
    use settings::SettingsStore;
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
            let plan = build_root_plan(
                Path::new("/project"),
                None,
                std::slice::from_ref(&workspace),
                cx,
            );
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
                None,
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
            let main_plan = build_root_plan(
                Path::new("/project"),
                None,
                std::slice::from_ref(&workspace),
                cx,
            );
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
                None,
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
                None,
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
                None,
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
                    None,
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
        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, cx).await));
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
                    None,
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
        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, cx).await));
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
                    None,
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

        let task = cx.update(|cx| cx.spawn(async move |cx| remove_root(root, cx).await));
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

    /// Case B (the original bug): the worktree directory exists with leftover
    /// content (e.g. files that Windows couldn't fully delete during archival
    /// because they were locked by another process), but git has no
    /// registration for it. The pre-flight check must report content so the
    /// caller can prompt before the leftover files get clobbered.
    #[gpui::test]
    async fn test_has_content_leftover_dir_no_registration(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // Worktree directory has leftover content but no .git linkage and no
        // entry in any main repo's .git/worktrees/.
        fs.insert_tree(
            "/wt-orphaned",
            json!({
                "leftover.txt": "important user data",
            }),
        )
        .await;

        let has_content = worktree_path_has_content(fs.as_ref(), Path::new("/wt-orphaned")).await;

        assert_eq!(
            has_content.unwrap(),
            true,
            "orphaned dir from a partial Windows archive must report content"
        );
        assert!(
            fs.is_file(Path::new("/wt-orphaned/leftover.txt")).await,
            "the check must not touch any files"
        );
    }

    /// Case D: the worktree directory exists with content but the `.git` file
    /// in the worktree itself is missing. The restore would still clobber
    /// any user files on disk, so we must report content here.
    #[gpui::test]
    async fn test_has_content_dir_with_broken_dot_git(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        // /wt-broken has files but no `.git` file — the linkage is broken.
        fs.insert_tree(
            "/wt-broken",
            json!({
                "src": { "lib.rs": "// existing user file" },
            }),
        )
        .await;

        let has_content = worktree_path_has_content(fs.as_ref(), Path::new("/wt-broken")).await;

        assert_eq!(
            has_content.unwrap(),
            true,
            "a directory with broken git linkage but real files must report content"
        );
    }

    /// Case E: the worktree directory is fully valid — `.git` file points back
    /// to the main repo. Even so, the restore will overwrite any uncommitted
    /// work the user has in there via `git read-tree --reset -u`, so we must
    /// still report content.
    #[gpui::test]
    async fn test_has_content_fully_valid_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/wt-valid",
            json!({
                ".git": "gitdir: /project/.git/worktrees/feature",
                "src": { "lib.rs": "// uncommitted local work" },
            }),
        )
        .await;

        let has_content = worktree_path_has_content(fs.as_ref(), Path::new("/wt-valid")).await;

        assert_eq!(
            has_content.unwrap(),
            true,
            "a valid worktree with uncommitted work must report content \
             (read-tree --reset -u would clobber it)"
        );
    }

    /// Case A: nothing exists at the worktree path. The check must report
    /// no content — there's nothing to lose.
    #[gpui::test]
    async fn test_has_content_when_path_missing(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        assert!(
            fs.metadata(Path::new("/wt-missing"))
                .await
                .unwrap()
                .is_none(),
            "precondition: worktree path must not exist"
        );

        let has_content = worktree_path_has_content(fs.as_ref(), Path::new("/wt-missing")).await;

        assert_eq!(
            has_content.unwrap(),
            false,
            "missing dir must not report content"
        );
    }

    /// An empty directory at the worktree path has no content to lose.
    #[gpui::test]
    async fn test_has_content_for_empty_dir(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.create_dir(Path::new("/wt-empty")).await.unwrap();
        assert!(
            fs.is_dir(Path::new("/wt-empty")).await,
            "precondition: empty dir must exist"
        );

        let has_content = worktree_path_has_content(fs.as_ref(), Path::new("/wt-empty")).await;

        assert_eq!(
            has_content.unwrap(),
            false,
            "empty dir must not report content"
        );
    }

    #[gpui::test]
    async fn test_restore_rejects_remote_connection(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            "/project",
            json!({
                ".git": {
                    "worktrees": {},
                },
                "src": {},
            }),
        )
        .await;
        cx.update(|cx| <dyn fs::Fs>::set_global(fs.clone(), cx));

        let remote_connection = RemoteConnectionOptions::Ssh(SshConnectionOptions {
            host: "test-host".into(),
            ..Default::default()
        });

        let overwrite_row = ArchivedGitWorktree {
            id: 1,
            worktree_path: PathBuf::from("/remote/worktree"),
            main_repo_path: PathBuf::from("/remote/project"),
            branch_name: Some("feature".to_string()),
            staged_commit_hash: "abc123".to_string(),
            unstaged_commit_hash: "def456".to_string(),
            original_commit_hash: "789abc".to_string(),
        };

        let overwrite_result = cx
            .spawn(|mut cx| {
                let remote_connection = remote_connection.clone();
                async move {
                    restore_would_overwrite(&overwrite_row, Some(&remote_connection), &mut cx).await
                }
            })
            .await;

        assert!(
            overwrite_result.is_err(),
            "restore_would_overwrite should reject remote connections"
        );
        assert!(
            format!("{:#}", overwrite_result.unwrap_err()).contains("remote machines"),
            "error message should mention remote machines"
        );

        let restore_row = ArchivedGitWorktree {
            id: 1,
            worktree_path: PathBuf::from("/remote/worktree"),
            main_repo_path: PathBuf::from("/remote/project"),
            branch_name: Some("feature".to_string()),
            staged_commit_hash: "abc123".to_string(),
            unstaged_commit_hash: "def456".to_string(),
            original_commit_hash: "789abc".to_string(),
        };

        let restore_result = cx
            .spawn(|mut cx| {
                let remote_connection = remote_connection.clone();
                async move {
                    restore_worktree_via_git(&restore_row, Some(&remote_connection), &mut cx).await
                }
            })
            .await;

        assert!(
            restore_result.is_err(),
            "restore_worktree_via_git should reject remote connections"
        );
        assert!(
            format!("{:#}", restore_result.unwrap_err()).contains("remote machines"),
            "error message should mention remote machines"
        );
    }
}
