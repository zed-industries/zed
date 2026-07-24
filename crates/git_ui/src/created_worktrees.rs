//! A persistent registry of git worktrees that Zed itself created.
//!
//! Thread archival deletes a worktree's directory from disk, so it must be
//! certain the worktree was created by Zed rather than by the user. This
//! module records each Zed-created worktree in the local database, keyed by
//! its path (and remote host, for remote projects), along with the creation
//! time of the worktree's git metadata directory at the time Zed created it.
//!
//! Before deleting a worktree, callers re-stat that directory and compare
//! against the recorded time. A mismatch means the worktree was removed and
//! recreated outside Zed, so deletion must be skipped. Every failure mode
//! (no record, unreadable creation time, mismatched time) fails safe by
//! leaving the directory untouched.
//!
//! Because the registry lives in the local database, worktrees created by a
//! different Zed install (e.g. another release channel, or another machine
//! connecting to the same remote host) are treated as manually created and
//! never archived. That is intentional: when in doubt, don't delete.

use std::{
    future::Future,
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context as _, Result};
use db::kvp::KeyValueStore;
use gpui::{App, AsyncApp, Entity};
use project::git_store::Repository;
use remote::{RemoteConnectionOptions, remote_connection_identity};
use serde::{Deserialize, Serialize};
use util::ResultExt as _;

const NAMESPACE: &str = "created_git_worktrees";

#[derive(Serialize, Deserialize)]
struct CreatedWorktreeRecord {
    created_at_seconds: u64,
    created_at_subsec_nanos: u32,
}

fn record_key(worktree_path: &Path, remote: Option<&RemoteConnectionOptions>) -> String {
    let host = match remote {
        None => "local".to_string(),
        Some(options) => remote_connection_identity(options).persistence_key(),
    };
    // Paths cannot contain newlines in practice, so this separator is
    // unambiguous.
    format!("{host}\n{}", worktree_path.display())
}

/// Records that Zed created the worktree at `worktree_path`, along with the
/// creation time of its git metadata directory.
pub fn record_created_worktree(
    worktree_path: &Path,
    remote: Option<&RemoteConnectionOptions>,
    created_at: SystemTime,
    cx: &App,
) -> impl Future<Output = Result<()>> + use<> {
    let store = KeyValueStore::global(cx);
    let key = record_key(worktree_path, remote);
    let value = created_at
        .duration_since(UNIX_EPOCH)
        .context("worktree creation time predates the unix epoch")
        .and_then(|duration| {
            serde_json::to_string(&CreatedWorktreeRecord {
                created_at_seconds: duration.as_secs(),
                created_at_subsec_nanos: duration.subsec_nanos(),
            })
            .context("failed to serialize created worktree record")
        });
    async move { store.scoped(NAMESPACE).write(key, value?).await }
}

/// Returns the recorded creation time for a worktree Zed created, or `None`
/// if Zed has no record of creating it.
pub fn recorded_created_at(
    worktree_path: &Path,
    remote: Option<&RemoteConnectionOptions>,
    cx: &App,
) -> Option<SystemTime> {
    let store = KeyValueStore::global(cx);
    let value = store
        .scoped(NAMESPACE)
        .read(&record_key(worktree_path, remote))
        .log_err()??;
    let record: CreatedWorktreeRecord = serde_json::from_str(&value).log_err()?;
    Some(UNIX_EPOCH + Duration::new(record.created_at_seconds, record.created_at_subsec_nanos))
}

/// Looks up the creation time of a freshly created worktree's git metadata
/// directory through the repository and records it in the registry.
///
/// Failures are logged and swallowed: the worktree just won't be eligible
/// for automatic archival.
pub async fn record_created_worktree_for_repo(
    repo: &Entity<Repository>,
    worktree_path: &Path,
    remote: Option<&RemoteConnectionOptions>,
    cx: &mut AsyncApp,
) {
    let receiver = repo.update(cx, |repo, _cx| {
        repo.worktree_created_at(worktree_path.to_path_buf())
    });
    let created_at = match receiver.await {
        Ok(Ok(Some(created_at))) => created_at,
        Ok(Ok(None)) => {
            log::warn!(
                "Newly created worktree {} not found on disk; \
                 it won't be eligible for automatic archival",
                worktree_path.display()
            );
            return;
        }
        Ok(Err(error)) => {
            log::warn!(
                "Couldn't determine creation time for worktree {}; \
                 it won't be eligible for automatic archival: {error:#}",
                worktree_path.display()
            );
            return;
        }
        Err(_) => {
            log::warn!(
                "Worktree creation time lookup was canceled for {}",
                worktree_path.display()
            );
            return;
        }
    };
    let record = cx.update(|cx| record_created_worktree(worktree_path, remote, created_at, cx));
    if let Err(error) = record.await {
        log::warn!(
            "Failed to record created worktree {}: {error:#}",
            worktree_path.display()
        );
    }
}

/// Removes the record for a worktree, either because Zed deleted it or
/// because the directory on disk turned out not to be the one Zed created.
pub fn forget_created_worktree(
    worktree_path: &Path,
    remote: Option<&RemoteConnectionOptions>,
    cx: &App,
) -> impl Future<Output = Result<()>> + use<> {
    let store = KeyValueStore::global(cx);
    let key = record_key(worktree_path, remote);
    async move { store.scoped(NAMESPACE).delete(key).await }
}
