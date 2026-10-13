//! Worker worktrees: `spawn` / `remove` plus merge `preview` / `back`.
//!
//! All `git` goes through blocking `Command` subprocess calls with an
//! explicit `-C <repo>` (never the caller cwd), mirroring the one-shot
//! discipline in `lib.rs`: each call shells out, then returns.
//!
//! Merge previews use a throwaway local clone (`git clone -s -b <base>`)
//! and run a REAL `git merge` there, so conflict detection has exact
//! merge semantics with zero output parsing. The clone is deleted before
//! returning; the real repo is never touched by a preview. `merge_back`
//! pre-checks the preview and refuses `Conflicted` work instead of
//! leaving a half-merged index behind (never `--force`).

use crate::registry::{WorkerError, WorkerHead, WorkerStatus};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// Uniquifies temp dirs so parallel `cargo test` threads never share one.
static TMP_SEQ: AtomicU32 = AtomicU32::new(0);

fn unique_tag() -> String {
    let n = TMP_SEQ.fetch_add(1, Ordering::SeqCst);
    format!("{}-{n}", std::process::id())
}

/// A `git -C dir` command with the hook environment scrubbed.
pub(crate) fn git_command(dir: &Path) -> Command {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(dir)
        // Scrub GIT_* env: under `git commit` hooks git exports GIT_DIR (and
        // friends), which makes `git -C <other-dir>` resolve the index
        // against the WRONG repo ("Not a directory").
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE");
    command
}

/// Run `git -C dir args`, returning trimmed stdout; failures become
/// `WorkerError::Git` carrying trimmed stderr.
pub(crate) fn git(dir: &Path, args: &[&str]) -> Result<String, WorkerError> {
    let out = git_command(dir)
        .args(args)
        .output()
        .map_err(|e| WorkerError::Git(e.to_string()))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(WorkerError::Git(err));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Spawn a worker: new branch `cedian-worker/<id>` from `base`, checked
/// out at `.worktrees/<id>`.
/// Worker ids become a path segment (`.worktrees/<id>`) and a branch name
/// (`cedian-worker/<id>`): only a conservative charset is allowed, so an id
/// can never escape `.worktrees/` or form an invalid ref.
pub fn validate_id(id: &str) -> Result<(), WorkerError> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && !id.starts_with('.')
        && !id.contains("..")
        && !id.ends_with(".lock")
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if ok {
        Ok(())
    } else {
        Err(WorkerError::BadId(id.to_string()))
    }
}

pub fn spawn(repo: &Path, id: &str, base: &str) -> Result<WorkerHead, WorkerError> {
    validate_id(id)?;
    let branch = format!("cedian-worker/{id}");
    let worktree = format!(".worktrees/{id}");
    git(repo, &["worktree", "add", &worktree, "-b", &branch, base])?;
    Ok(WorkerHead {
        id: id.to_string(),
        branch,
        worktree,
        task_title: String::new(),
        kind: String::new(),
        status: WorkerStatus::Ready,
        note: String::new(),
        timebox_min: 0.0,
        // The spawn is the row's first side effect (ADR-0033 decision 5).
        last_progress: Some(crate::registry::now_ms()),
    })
}

/// The committer time of `branch`'s newest commit in epoch milliseconds, or
/// `None` for a branch with no commits yet (an unborn ref).
pub fn last_commit_ms(repo: &Path, branch: &str) -> Result<Option<u64>, WorkerError> {
    let tip = format!("{branch}^{{commit}}");
    let born = git_command(repo)
        .args(["rev-parse", "--verify", "--quiet", &tip])
        .output()
        .map_err(|e| WorkerError::Git(e.to_string()))?
        .status
        .success();
    if !born {
        return Ok(None);
    }
    let seconds: u64 = git(repo, &["log", "-1", "--format=%ct", branch])?
        .parse()
        .map_err(|e| WorkerError::Git(format!("unreadable commit time: {e}")))?;
    Ok(Some(seconds * 1_000))
}

/// Remove a worker: plain `worktree remove` first (no `--force`, so a
/// dirty tree fails loud and the branch is kept), then `branch -D`.
///
/// `-D` (not `-d`): teardown must succeed even when the worker was never
/// merged — the committed-conflict test removes after a refused
/// `merge_back`, and `-d` would fail there on the unmerged branch.
/// Uncommitted work is still safe: plain `worktree remove` already fails
/// loud on a dirty tree before the branch is touched.
pub fn remove(repo: &Path, head: &WorkerHead) -> Result<(), WorkerError> {
    // Refuse BEFORE touching the worktree: unmerged commits would be lost.
    // (`branch -D` used to delete them silently.)
    let merged = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["merge-base", "--is-ancestor", &head.branch, "HEAD"])
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .status()
        .map_err(|e| WorkerError::Git(e.to_string()))?;
    if !merged.success() {
        return Err(WorkerError::NotMerged(head.branch.clone()));
    }
    git(repo, &["worktree", "remove", &head.worktree])?;
    git(repo, &["branch", "-d", &head.branch])?;
    Ok(())
}

/// Throw away a tree [`spawn`] just made and its branch, which hold no work
/// yet; used when the request it was made for fails after it.
pub(crate) fn discard(repo: &Path, head: &WorkerHead) -> Result<(), WorkerError> {
    force_remove(repo, Path::new(&head.worktree), &head.branch)
}

/// Drop a worktree and its branch unconditionally: `--force`/`-D` are right
/// for trees that are meant to be thrown away (a decided `Wip`, a `Scratch`,
/// or a just-spawned tree). Unlike [`remove`] this never refuses over
/// uncommitted or unmerged work — the caller owns that decision.
pub(crate) fn force_remove(repo: &Path, worktree: &Path, branch: &str) -> Result<(), WorkerError> {
    git(
        repo,
        &["worktree", "remove", "--force", &worktree.to_string_lossy()],
    )?;
    git(repo, &["branch", "-D", branch])?;
    Ok(())
}

/// What merging the worker branch into `base` would do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergePlan {
    /// Changed files (`base...branch`) that merge cleanly.
    pub clean: Vec<String>,
    /// Changed files that conflict (STALE: resolve in the worktree first).
    pub conflicted: Vec<String>,
}

/// Preview a merge without touching the repo: trial-merge the worker
/// branch into `base` inside a throwaway local clone.
pub fn merge_preview(repo: &Path, head: &WorkerHead, base: &str) -> Result<MergePlan, WorkerError> {
    let range = format!("{base}...{}", head.branch);
    let changed = git(repo, &["diff", "--name-only", &range])?;
    let tmp = std::env::temp_dir().join(format!("cedian-merge-{}", unique_tag()));
    let _ = std::fs::remove_dir_all(&tmp);
    let preview = trial_merge(repo, &tmp, head, base);
    let _ = std::fs::remove_dir_all(&tmp);
    let conflicted = preview?;
    let clean = changed
        .lines()
        .map(str::to_string)
        .filter(|f| !conflicted.contains(f))
        .collect();
    Ok(MergePlan { clean, conflicted })
}

/// Clone `base` to `tmp` and really merge the worker branch there;
/// return the unmerged (`U`) files. `tmp` cleanup stays with the caller.
/// `base` may be a rev (`HEAD`); it is resolved to a branch name first
/// because `clone -b` needs one.
fn trial_merge(
    repo: &Path,
    tmp: &Path,
    head: &WorkerHead,
    base: &str,
) -> Result<Vec<String>, WorkerError> {
    let from = repo.to_string_lossy().to_string();
    let dest = tmp.to_string_lossy().to_string();
    let base_branch = git(repo, &["rev-parse", "--abbrev-ref", base])?;
    // `-s`: share objects locally — hermetic, no network, cheap.
    git(
        &std::env::temp_dir(),
        &["clone", "-s", "-b", &base_branch, &from, &dest],
    )?;
    // The clone tracks the worker branch as `origin/<branch>` only; the
    // trial merge needs a local ref under its real name.
    git(
        tmp,
        &["branch", &head.branch, &format!("origin/{}", head.branch)],
    )?;
    // Identity is repo-local config, which clones do not copy — supply it
    // inline so the trial merge can commit on the clean path.
    let merge = Command::new("git")
        .arg("-C")
        .arg(tmp)
        .arg("-c")
        .arg("user.email=cedian@t")
        .arg("-c")
        .arg("user.name=cedian")
        .arg("merge")
        .arg("--no-edit")
        .arg(&head.branch)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .output()
        .map_err(|e| WorkerError::Git(e.to_string()))?;
    let raw = git(tmp, &["diff", "--name-only", "--diff-filter=U"])?;
    let conflicted: Vec<String> = raw.lines().map(str::to_string).collect();
    if !merge.status.success() && conflicted.is_empty() {
        let err = String::from_utf8_lossy(&merge.stderr).trim().to_string();
        return Err(WorkerError::Git(err));
    }
    Ok(conflicted)
}

/// Merge the worker branch into `base`. Refuses `Conflicted` work BEFORE
/// touching the repo, so a conflict never leaves a half-merged index.
pub fn merge_back(repo: &Path, head: &WorkerHead, base: &str) -> Result<(), WorkerError> {
    let plan = merge_preview(repo, head, base)?;
    if !plan.conflicted.is_empty() {
        return Err(WorkerError::Conflicted(plan.conflicted));
    }
    // `git merge` targets whatever is checked out — it must BE the base the
    // preview was computed against, or we'd merge into the wrong branch.
    let want = git(repo, &["rev-parse", "--abbrev-ref", base])?;
    let checked_out = git(repo, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    if want != checked_out || checked_out == "HEAD" {
        return Err(WorkerError::WrongBase {
            base: base.to_string(),
            checked_out,
        });
    }
    // The pre-check guarantees clean, so this fast-forwards or merges.
    git(repo, &["merge", &head.branch, "--no-edit"])?;
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{merge_back, merge_preview, remove, spawn};
    use crate::registry::WorkerError;
    use std::path::{Path, PathBuf};

    fn git(dir: &Path, args: &[&str]) -> Result<String, String> {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            // Same scrub as the crate helper: the `git commit` hook exports
            // relative `GIT_DIR`, which breaks `git -C <other-dir>`.
            .env_remove("GIT_DIR")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("GIT_WORK_TREE")
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
        }
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }

    /// Fresh repo on `main` with one commit (shared with `host_tool` tests).
    pub(crate) fn fixture() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("cedian-wt-test-{}", super::unique_tag()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        git(&dir, &["init", "-b", "main"]).unwrap();
        git(&dir, &["config", "user.email", "t@t"]).unwrap();
        git(&dir, &["config", "user.name", "t"]).unwrap();
        std::fs::write(dir.join("a.txt"), "base\n").unwrap();
        git(&dir, &["add", "."]).unwrap();
        git(&dir, &["commit", "-m", "base"]).unwrap();
        dir
    }

    #[test]
    fn spawn_preview_merge_clean() {
        let repo = fixture();
        let head = spawn(&repo, "w1", "main").unwrap();
        assert_eq!(head.branch, "cedian-worker/w1");
        // Worker edits a disjoint file.
        std::fs::write(repo.join(".worktrees/w1/b.txt"), "worker\n").unwrap();
        git(&repo.join(".worktrees/w1"), &["add", "."]).unwrap();
        git(&repo.join(".worktrees/w1"), &["commit", "-m", "w"]).unwrap();
        let plan = merge_preview(&repo, &head, "main").unwrap();
        assert_eq!(plan.clean, vec!["b.txt".to_string()]);
        assert!(plan.conflicted.is_empty());
        merge_back(&repo, &head, "main").unwrap();
        assert!(repo.join("b.txt").exists());
        remove(&repo, &head).unwrap();
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn conflicting_edit_refuses_merge() {
        let repo = fixture();
        let head = spawn(&repo, "w1", "main").unwrap();
        std::fs::write(repo.join(".worktrees/w1/a.txt"), "worker\n").unwrap();
        git(&repo.join(".worktrees/w1"), &["commit", "-am", "w"]).unwrap();
        std::fs::write(repo.join("a.txt"), "base-edit\n").unwrap();
        git(&repo, &["commit", "-am", "b"]).unwrap();
        let plan = merge_preview(&repo, &head, "main").unwrap();
        assert_eq!(plan.conflicted, vec!["a.txt".to_string()]);
        assert!(matches!(
            merge_back(&repo, &head, "main"),
            Err(WorkerError::Conflicted(_))
        ));
        // Unmerged work: remove refuses and leaves worktree + branch intact.
        assert!(matches!(
            remove(&repo, &head),
            Err(WorkerError::NotMerged(_))
        ));
        assert!(repo.join(".worktrees/w1/a.txt").exists());
        assert!(git(&repo, &["rev-parse", "--verify", "cedian-worker/w1"]).is_ok());
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn merge_back_refuses_when_base_not_checked_out() {
        let repo = fixture();
        git(&repo, &["branch", "other"]).unwrap();
        let head = spawn(&repo, "w2", "other").unwrap();
        std::fs::write(repo.join(".worktrees/w2/c.txt"), "w\n").unwrap();
        git(&repo.join(".worktrees/w2"), &["add", "."]).unwrap();
        git(&repo.join(".worktrees/w2"), &["commit", "-m", "w"]).unwrap();
        // Main checkout is on `main`; asking to merge into `other` must refuse.
        assert!(matches!(
            merge_back(&repo, &head, "other"),
            Err(WorkerError::WrongBase { .. })
        ));
        assert!(!repo.join("c.txt").exists());
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn unsafe_ids_rejected() {
        for bad in ["", "../x", "a/b", ".hidden", "a..b", "x.lock", "sp ace"] {
            assert!(super::validate_id(bad).is_err(), "{bad:?} must be rejected");
        }
        for good in ["w1", "fix-login_2", "v1.2"] {
            assert!(super::validate_id(good).is_ok(), "{good:?} must be allowed");
        }
    }

    #[test]
    fn last_commit_ms_is_none_for_an_unborn_branch() {
        let repo = fixture();
        let head = spawn(&repo, "w1", "main").unwrap();
        assert!(
            super::last_commit_ms(&repo, &head.branch)
                .unwrap()
                .is_some(),
            "a spawned branch points at the base commit"
        );
        // A repository with no commits has an unborn `main`.
        let empty = std::env::temp_dir().join(format!("cedian-wt-empty-{}", super::unique_tag()));
        let _ = std::fs::remove_dir_all(&empty);
        std::fs::create_dir_all(&empty).unwrap();
        git(&empty, &["init", "-b", "main"]).unwrap();
        assert!(super::last_commit_ms(&empty, "main").unwrap().is_none());
        std::fs::remove_dir_all(&repo).unwrap();
        std::fs::remove_dir_all(&empty).unwrap();
    }

    /// Regression: `spawn` works when the parent env carries hook-inherited
    /// `GIT_*` (the `git commit` hook exports relative `GIT_DIR`, which used
    /// to break `git -C <other-dir>` with "Not a directory").
    ///
    /// NOTE: `set_var` here is process-global, so this test takes the
    /// `serial` burden explicitly: it saves/restores every var it touches,
    /// and the suite has no other env-mutating test. If a second one ever
    /// appears, gate both behind a shared mutex.
    #[test]
    fn git_scrubs_hook_env() {
        let saved: Vec<(&str, Option<std::ffi::OsString>)> =
            ["GIT_DIR", "GIT_INDEX_FILE", "GIT_WORK_TREE"]
                .iter()
                .map(|k| (*k, std::env::var_os(k)))
                .collect();
        std::env::set_var("GIT_DIR", ".git");
        let repo = fixture();
        let head = spawn(&repo, "w1", "main").unwrap();
        remove(&repo, &head).unwrap();
        std::fs::remove_dir_all(&repo).unwrap();
        for (k, v) in saved {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
    }
}
