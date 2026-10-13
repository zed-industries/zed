//! Worker registry: persisted heads at `workers.json` in the repo's cedian
//! state dir (ADR-0044), which the caller resolves and passes in.
//!
//! The registry is a MECHANISM record (§88): which workers exist, which
//! branch/worktree serves each task, and their lifecycle state. No
//! scheduling, no queue — OMP stays the only orchestrator.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Lifecycle state of one worker, serialized snake_case (`ready`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerStatus {
    Ready,
    Running,
    Done,
    Stale,
    Failed,
}

/// One row in the registry: which branch/worktree serves a task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerHead {
    pub id: String,
    pub branch: String,
    /// Repo-relative worktree path (`.worktrees/<id>`).
    pub worktree: String,
    pub task_title: String,
    pub kind: String,
    pub status: WorkerStatus,
    pub note: String,
    /// Minutes before a running worker with no progress shows `stale`
    /// (ADR-0033 decision 5). Defaulted so a pre-existing `workers.json`
    /// keeps loading without a version bump.
    #[serde(default)]
    pub timebox_min: f64,
    /// Epoch milliseconds of the last side effect (ADR-0033 decision 5), set
    /// to the spawn time when the row is inserted. The same shape as an audit
    /// row's `timestamp_ms`. Defaulted for the same reason.
    #[serde(default)]
    pub last_progress: Option<u64>,
}

impl WorkerHead {
    /// Whether a `running` worker has passed its timebox with no progress:
    /// the later of `last_progress` and the branch's newest commit is older
    /// than `timebox_min` (ADR-0033 decision 5). A commit on the worker
    /// branch counts as progress.
    pub fn is_stuck(&self, now_ms: u64, repo: &Path) -> bool {
        if self.status != WorkerStatus::Running || self.timebox_min <= 0.0 {
            return false;
        }
        let progress = self
            .last_progress
            .into_iter()
            .chain(
                crate::worktree::last_commit_ms(repo, &self.branch)
                    .ok()
                    .flatten(),
            )
            .max();
        progress.is_some_and(|progress| {
            now_ms.saturating_sub(progress) as f64 > self.timebox_min * 60_000.0
        })
    }
}

/// Epoch milliseconds now, the shape `last_progress` and an audit row's
/// `timestamp_ms` share.
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_millis() as u64)
        .unwrap_or(0)
}

/// Registry errors: duplicate/unknown ids, or registry file I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkerError {
    Exists(String),
    NoSuch(String),
    Io(String),
    Git(String),
    Conflicted(Vec<String>),
    /// Worker id is not a safe path/branch segment.
    BadId(String),
    /// `workers.json` is corrupt or another snapshot version (fail closed).
    Snapshot(String),
    /// Branch has commits not in the main checkout — removing would lose them.
    NotMerged(String),
    /// The main checkout is not on the requested merge base.
    WrongBase {
        base: String,
        checked_out: String,
    },
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exists(id) => write!(f, "worker {id:?} already exists"),
            Self::NoSuch(id) => write!(f, "no worker {id:?}"),
            Self::Io(e) => write!(f, "worker registry io: {e}"),
            Self::Snapshot(e) => write!(f, "worker registry: {e}"),
            Self::Git(e) => write!(f, "worker git failed: {e}"),
            Self::Conflicted(files) => {
                write!(f, "worker merge refused (STALE): {}", files.join(", "))
            }
            Self::BadId(id) => write!(
                f,
                "bad worker id {id:?} (use 1-64 chars of [A-Za-z0-9._-], not starting with '.')"
            ),
            Self::NotMerged(branch) => write!(
                f,
                "branch {branch} has unmerged commits — merge-back first, or delete it with git yourself"
            ),
            Self::WrongBase { base, checked_out } => write!(
                f,
                "merge base {base:?} is not checked out (main checkout is on {checked_out:?})"
            ),
        }
    }
}

fn registry_path(state: &Path) -> PathBuf {
    state.join("workers.json")
}

/// `workers.json` schema version (ADR-0016 / P3). Bump on any shape change.
pub const REGISTRY_SNAPSHOT_VERSION: u32 = 1;

/// Persisted worker heads, shaped `{"snapshot_version": N, "workers": {id: head}}`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    /// Missing in pre-P3 files → 0 → rejected.
    #[serde(default)]
    snapshot_version: u32,
    workers: BTreeMap<String, WorkerHead>,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            snapshot_version: REGISTRY_SNAPSHOT_VERSION,
            workers: BTreeMap::new(),
        }
    }
}

impl Registry {
    /// Take the registry's lock under `state` (created if missing), held
    /// until the file is dropped. Every open-change-save holds it, the app's
    /// host tool and the CLI alike, so no writer saves over another's row.
    pub fn lock(state: &Path) -> Result<std::fs::File, WorkerError> {
        std::fs::create_dir_all(state).map_err(|e| WorkerError::Io(e.to_string()))?;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(state.join("workers.lock"))
            .map_err(|e| WorkerError::Io(e.to_string()))?;
        file.lock().map_err(|e| WorkerError::Io(e.to_string()))?;
        Ok(file)
    }

    /// Open the registry; a missing file yields an empty one. Returns the
    /// registry plus whether the file already existed. An unreadable, corrupt
    /// or other-version file fails closed — never silently emptied, since the
    /// next save would forget live worktrees.
    pub fn open(state: &Path) -> Result<(Self, bool), WorkerError> {
        let path = registry_path(state);
        if !path.exists() {
            return Ok((Self::default(), false));
        }
        let raw = std::fs::read_to_string(&path).map_err(|e| WorkerError::Io(e.to_string()))?;
        let reg: Self = serde_json::from_str(&raw)
            .map_err(|e| WorkerError::Snapshot(format!("corrupt {}: {e}", path.display())))?;
        if reg.snapshot_version != REGISTRY_SNAPSHOT_VERSION {
            return Err(WorkerError::Snapshot(format!(
                "workers.json state too old (got v{}, want v{REGISTRY_SNAPSHOT_VERSION}), \
                 re-baseline: inspect `git worktree list`, then remove {}",
                reg.snapshot_version,
                path.display()
            )));
        }
        Ok((reg, true))
    }

    /// Persist into `state` (created if missing).
    pub fn save(&self, state: &Path) -> Result<(), WorkerError> {
        std::fs::create_dir_all(state).map_err(|e| WorkerError::Io(e.to_string()))?;
        let raw = serde_json::to_string_pretty(self).map_err(|e| WorkerError::Io(e.to_string()))?;
        std::fs::write(registry_path(state), raw).map_err(|e| WorkerError::Io(e.to_string()))?;
        Ok(())
    }

    /// Insert a head; duplicate ids are rejected (record, not a queue).
    pub fn insert(&mut self, head: WorkerHead) -> Result<(), WorkerError> {
        if self.workers.contains_key(&head.id) {
            return Err(WorkerError::Exists(head.id));
        }
        self.workers.insert(head.id.clone(), head);
        Ok(())
    }

    /// Look up a head by id.
    pub fn get(&self, id: &str) -> Option<&WorkerHead> {
        self.workers.get(id)
    }

    /// All heads by id order — the CLI `list` view.
    pub fn all(&self) -> impl Iterator<Item = &WorkerHead> {
        self.workers.values()
    }

    /// Mark status + note; unknown ids fail (no implicit creation).
    pub fn set_status(
        &mut self,
        id: &str,
        status: WorkerStatus,
        note: String,
    ) -> Result<(), WorkerError> {
        let head = self
            .workers
            .get_mut(id)
            .ok_or_else(|| WorkerError::NoSuch(id.to_string()))?;
        head.status = status;
        head.note = note;
        Ok(())
    }

    /// Drop a head, returning it when present.
    pub fn remove(&mut self, id: &str) -> Option<WorkerHead> {
        self.workers.remove(id)
    }

    /// Flip every `running` row whose timebox has passed to `stale`
    /// (ADR-0033 decision 5) and return how many changed. A later unit feeds
    /// imported edits and evidence into `last_progress`; the caller saves.
    pub fn refresh_liveness(&mut self, now_ms: u64, repo: &Path) -> usize {
        let mut changed = 0;
        for head in self.workers.values_mut() {
            if head.status == WorkerStatus::Running && head.is_stuck(now_ms, repo) {
                head.status = WorkerStatus::Stale;
                changed += 1;
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worktree::tests::fixture;
    use std::path::Path;

    fn head(id: &str, branch: &str, status: WorkerStatus) -> WorkerHead {
        WorkerHead {
            id: id.into(),
            branch: branch.into(),
            worktree: format!(".worktrees/{id}"),
            task_title: "t".into(),
            kind: "k".into(),
            status,
            note: String::new(),
            timebox_min: 0.0,
            last_progress: None,
        }
    }

    /// Commit `file` in `dir`, optionally back-dating the commit. Goes through
    /// the crate's scrubbed command: with GIT_DIR set (a git hook runs this
    /// suite under `script/cedian-check`) a raw `git commit` here would write
    /// to the wrong repository.
    fn commit(dir: &Path, file: &str, date: Option<&str>) {
        std::fs::write(dir.join(file), "x\n").unwrap();
        let run = |args: &[&str], date: Option<&str>| {
            let mut cmd = crate::worktree::git_command(dir);
            cmd.args(args);
            if let Some(date) = date {
                cmd.env("GIT_COMMITTER_DATE", date)
                    .env("GIT_AUTHOR_DATE", date);
            }
            assert!(cmd.status().unwrap().success(), "git {args:?}");
        };
        run(&["add", "."], None);
        run(&["commit", "-m", "c"], date);
    }

    #[test]
    fn registry_roundtrip() {
        let repo = std::env::temp_dir().join(format!("cedian-reg-test-{}", std::process::id()));
        std::fs::create_dir_all(&repo).unwrap();
        let (mut reg, existed) = Registry::open(&repo).unwrap();
        assert!(!existed);
        let mut first = head("w1", "cedian-worker/w1", WorkerStatus::Ready);
        first.last_progress = Some(1_700_000_000_000);
        first.timebox_min = 15.0;
        reg.insert(first).unwrap();
        assert!(reg.insert(head("w1", "b", WorkerStatus::Ready)).is_err());
        reg.save(&repo).unwrap();
        let (reg2, existed2) = Registry::open(&repo).unwrap();
        assert!(existed2);
        let back = reg2.get("w1").unwrap();
        assert_eq!(back.task_title, "t");
        assert_eq!(back.last_progress, Some(1_700_000_000_000));
        assert_eq!(back.timebox_min, 15.0);
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn is_stuck_respects_the_timebox_and_a_newer_commit() {
        let repo = fixture();
        // w1: the worker branch's newest commit is old, so progress is the
        // (equally old) last_progress.
        let mut w1 = crate::worktree::spawn(&repo, "w1", "main").unwrap();
        commit(
            &repo.join(".worktrees/w1"),
            "old.txt",
            Some("2020-01-01T00:00:00+00:00"),
        );
        w1.status = WorkerStatus::Running;
        w1.timebox_min = 60.0;
        let progress = 1_700_000_000_000u64;
        w1.last_progress = Some(progress);
        assert!(!w1.is_stuck(progress + 60_000, &repo), "inside the timebox");
        assert!(
            w1.is_stuck(progress + 61 * 60_000, &repo),
            "past the timebox"
        );

        // w2: a commit newer than last_progress keeps it alive.
        let mut w2 = crate::worktree::spawn(&repo, "w2", "main").unwrap();
        commit(
            &repo.join(".worktrees/w2"),
            "new.txt",
            Some("2024-01-01T00:00:00+00:00"),
        );
        w2.status = WorkerStatus::Running;
        w2.timebox_min = 1.0;
        w2.last_progress = Some(progress);
        assert!(
            !w2.is_stuck(progress + 61 * 60_000, &repo),
            "a newer commit is progress"
        );
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn refresh_liveness_flips_exactly_the_stuck_row() {
        let repo = fixture();
        let stuck = crate::worktree::spawn(&repo, "w1", "main").unwrap();
        commit(
            &repo.join(".worktrees/w1"),
            "old.txt",
            Some("2020-01-01T00:00:00+00:00"),
        );
        let live = crate::worktree::spawn(&repo, "w2", "main").unwrap();
        commit(
            &repo.join(".worktrees/w2"),
            "new.txt",
            Some("2024-01-01T00:00:00+00:00"),
        );

        let progress = 1_700_000_000_000u64;
        let mut reg = Registry::default();
        for mut row in [stuck, live] {
            row.status = WorkerStatus::Running;
            row.timebox_min = 60.0;
            row.last_progress = Some(progress);
            reg.insert(row).unwrap();
        }
        let now = progress + 2 * 60 * 60_000;
        assert_eq!(reg.refresh_liveness(now, &repo), 1);
        assert_eq!(reg.get("w1").unwrap().status, WorkerStatus::Stale);
        assert_eq!(reg.get("w2").unwrap().status, WorkerStatus::Running);
        assert_eq!(reg.refresh_liveness(now, &repo), 0, "idempotent");
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn stale_or_corrupt_registry_fails_closed() {
        let repo = std::env::temp_dir().join(format!("cedian-reg-stale-{}", std::process::id()));
        std::fs::create_dir_all(&repo).unwrap();
        // Pre-P3 shape: no snapshot_version.
        std::fs::write(registry_path(&repo), r#"{"workers":{}}"#).unwrap();
        assert!(matches!(
            Registry::open(&repo),
            Err(WorkerError::Snapshot(_))
        ));
        std::fs::write(
            registry_path(&repo),
            r#"{"snapshot_version":99,"workers":{}}"#,
        )
        .unwrap();
        assert!(matches!(
            Registry::open(&repo),
            Err(WorkerError::Snapshot(_))
        ));
        std::fs::write(registry_path(&repo), "{not json").unwrap();
        assert!(matches!(
            Registry::open(&repo),
            Err(WorkerError::Snapshot(_))
        ));
        let _ = std::fs::remove_dir_all(&repo);
    }
}
