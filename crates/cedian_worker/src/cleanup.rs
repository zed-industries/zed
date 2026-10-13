//! Worker cleanup (S5 exit): sweep the cedian worktrees and decide what may go.
//!
//! The source of truth is `git worktree list`, never the registry — a row can
//! outlive its tree or miss one made by hand. Only paths under `<repo>/.worktrees/<id>`
//! are cedian's; the main checkout and every foreign worktree are left alone.
//! A `Wip` tree (tracked uncommitted edits) is never removed without an
//! explicit per-tree decision; `Unmerged` commits likewise stay put.
//!
//! All `git` runs through the crate's blocking, env-scrubbed helpers in
//! [`crate::worktree`], matching the one-shot discipline the rest of the crate
//! uses (a raw `git` under a `git commit` hook would target the wrong repo).

use crate::registry::WorkerError;
use crate::worktree::{force_remove, git, git_command};
use std::path::{Path, PathBuf};

/// How a cedian worktree stands against `base`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeClass {
    /// Branch is an ancestor of `base`: its work is already in the base.
    Merged,
    /// Clean tree whose commits are not in `base`.
    Unmerged,
    /// Tracked uncommitted edits — destroying them needs a per-tree decision.
    Wip,
    /// Only untracked files: nothing of value, safe to throw away.
    Scratch,
}

impl std::fmt::Display for TreeClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Merged => "merged",
            Self::Unmerged => "unmerged",
            Self::Wip => "wip",
            Self::Scratch => "scratch",
        };
        f.write_str(name)
    }
}

/// One live cedian worktree, as read from `git worktree list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeState {
    pub id: String,
    pub path: PathBuf,
    pub branch: String,
    pub class: TreeClass,
}

/// What [`cleanup`] did (or declined to do) with one tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupOutcome {
    pub id: String,
    pub path: PathBuf,
    pub class: TreeClass,
    pub removed: bool,
    /// Why it was kept, or git's refusal when removal failed; empty when removed.
    pub reason: String,
}

/// Every cedian worktree in `repo`, classified against `base`.
///
/// Reads `git worktree list --porcelain` and keeps only entries whose path is
/// `<repo>/.worktrees/<id>` and whose branch is `cedian-worker/<id>` (the path
/// is compared after canonicalizing both sides, so a symlinked or
/// `/tmp`-vs-`/private/tmp` root still matches). The main checkout, foreign
/// worktrees and anything at a cedian path without cedian's branch are omitted,
/// never classified.
pub fn trees(repo: &Path, base: &str) -> Result<Vec<TreeState>, WorkerError> {
    let root = repo.join(".worktrees");
    // No `.worktrees/` yet means no cedian tree exists to classify.
    let Ok(root) = root.canonicalize() else {
        return Ok(Vec::new());
    };
    let listed = git(repo, &["worktree", "list", "--porcelain"])?;
    let mut out = Vec::new();
    for entry in parse_porcelain(&listed) {
        let Ok(path) = entry.path.canonicalize() else {
            continue;
        };
        if path.parent() != Some(root.as_path()) {
            continue;
        }
        // Cedian's own trees are `.worktrees/<id>` on `cedian-worker/<id>`: both
        // halves must agree, or it is not ours to classify or remove (a tree
        // parked at that path on `main` would put `main` in reach).
        let Some(branch) = entry.branch else {
            continue;
        };
        let Some(id) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if branch != format!("cedian-worker/{id}") {
            continue;
        }
        let class = classify(repo, &path, &branch, base)?;
        out.push(TreeState {
            id: id.to_string(),
            path,
            branch,
            class,
        });
    }
    Ok(out)
}

/// Sweep every cedian tree: remove what is safe, report what is not.
///
/// `Merged` and `Scratch` trees are removed; `Unmerged` is skipped so commits
/// not in `base` are never lost; a `Wip` tree is removed only when its id is in
/// `decided` — an explicit per-tree decision. A removal git refuses yields
/// `removed: false` with git's message and the sweep continues to the rest.
pub fn cleanup(
    repo: &Path,
    base: &str,
    decided: &[String],
) -> Result<Vec<CleanupOutcome>, WorkerError> {
    let mut outcomes = Vec::new();
    // Same removal for every class we are allowed to drop; a refused git call
    // is reported, not fatal, so the sweep reaches the remaining trees.
    let remove = |tree: &TreeState| match force_remove(repo, &tree.path, &tree.branch) {
        Ok(()) => (true, String::new()),
        Err(e) => (false, format!("git refused: {e}")),
    };
    for tree in trees(repo, base)? {
        let (removed, reason) = match tree.class {
            TreeClass::Merged | TreeClass::Scratch => remove(&tree),
            TreeClass::Unmerged => (
                false,
                format!(
                    "commits not in {base}: merge-back first, then remove {}",
                    tree.branch
                ),
            ),
            TreeClass::Wip if decided.iter().any(|id| id == &tree.id) => remove(&tree),
            TreeClass::Wip => (
                false,
                format!(
                    "tracked edits: needs an explicit per-tree decision (pass --decide {})",
                    tree.id
                ),
            ),
        };
        outcomes.push(CleanupOutcome {
            id: tree.id,
            path: tree.path,
            class: tree.class,
            removed,
            reason,
        });
    }
    Ok(outcomes)
}

/// One `worktree <path>` record from `--porcelain` output (branch optional).
struct Porcelain {
    path: PathBuf,
    branch: Option<String>,
}

/// Split `git worktree list --porcelain` into per-tree records. Records start
/// at a `worktree` line and run to the next one; blank lines and the
/// `HEAD`/`detached`/`locked` lines are ignored.
fn parse_porcelain(listed: &str) -> Vec<Porcelain> {
    let mut entries = Vec::new();
    let mut current: Option<Porcelain> = None;
    for line in listed.lines() {
        if let Some(rest) = line.strip_prefix("worktree ") {
            if let Some(done) = current.take() {
                entries.push(done);
            }
            current = Some(Porcelain {
                path: PathBuf::from(rest),
                branch: None,
            });
        } else if let Some(rest) = line.strip_prefix("branch refs/heads/") {
            if let Some(entry) = current.as_mut() {
                entry.branch = Some(rest.to_string());
            }
        }
    }
    if let Some(done) = current.take() {
        entries.push(done);
    }
    entries
}

/// True when `branch` is an ancestor of `base` (`merge-base --is-ancestor`,
/// exit 0). Run from `repo`, not the tree: a `base` of `HEAD` must resolve to
/// the main checkout's branch, never the worker's own HEAD.
fn is_ancestor(repo: &Path, branch: &str, base: &str) -> Result<bool, WorkerError> {
    let status = git_command(repo)
        .args(["merge-base", "--is-ancestor", branch, base])
        .status()
        .map_err(|e| WorkerError::Git(e.to_string()))?;
    Ok(status.success())
}

/// Classify one tree.
///
/// Uncommitted edits decide first: a tree with tracked changes is `Wip` and a
/// tree with only untracked files is `Scratch`, even when its branch is an
/// ancestor of `base` — a freshly spawned worker is exactly that, and calling
/// it `Merged` would let cleanup throw its edits away. Only a clean tree is
/// then `Merged` (branch already in `base`) or `Unmerged` (commits not in base).
fn classify(repo: &Path, tree: &Path, branch: &str, base: &str) -> Result<TreeClass, WorkerError> {
    let status = git(tree, &["status", "--porcelain"])?;
    let mut tracked = false;
    let mut any = false;
    for line in status.lines() {
        any = true;
        // `??` is an untracked file; anything else edits tracked content.
        if !line.starts_with("??") {
            tracked = true;
        }
    }
    if tracked {
        return Ok(TreeClass::Wip);
    }
    if any {
        return Ok(TreeClass::Scratch);
    }
    Ok(if is_ancestor(repo, branch, base)? {
        TreeClass::Merged
    } else {
        TreeClass::Unmerged
    })
}

#[cfg(test)]
mod tests {
    use super::{TreeClass, cleanup, trees};
    use crate::worktree::{merge_back, spawn};
    use std::path::{Path, PathBuf};

    fn git(dir: &Path, args: &[&str]) -> Result<String, String> {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            // Same scrub as the crate helper: under `git commit`, git exports
            // GIT_DIR, which breaks `git -C <other-dir>`.
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

    /// A repo with one merged, one wip, one scratch and one unmerged cedian
    /// tree, plus a foreign worktree outside `.worktrees/`. Returns the repo,
    /// the unmerged head (so a test can merge it later) and the foreign path
    /// (outside the repo, so the caller must clean it up itself).
    fn scene() -> (PathBuf, crate::registry::WorkerHead, PathBuf) {
        let repo = crate::worktree::tests::fixture();

        // Merged: a real commit landed on `main`.
        let merged = spawn(&repo, "m1", "main").unwrap();
        std::fs::write(repo.join(".worktrees/m1/m.txt"), "m\n").unwrap();
        git(&repo.join(".worktrees/m1"), &["add", "."]).unwrap();
        git(&repo.join(".worktrees/m1"), &["commit", "-m", "m"]).unwrap();
        merge_back(&repo, &merged, "main").unwrap();

        // Wip: a tracked file edited, never committed.
        spawn(&repo, "w1", "main").unwrap();
        std::fs::write(repo.join(".worktrees/w1/a.txt"), "edited\n").unwrap();

        // Scratch: one untracked file, nothing else.
        spawn(&repo, "s1", "main").unwrap();
        std::fs::write(repo.join(".worktrees/s1/scratch.txt"), "s\n").unwrap();

        // Unmerged: a commit not on `main`.
        let unmerged = spawn(&repo, "u1", "main").unwrap();
        std::fs::write(repo.join(".worktrees/u1/u.txt"), "u\n").unwrap();
        git(&repo.join(".worktrees/u1"), &["add", "."]).unwrap();
        git(&repo.join(".worktrees/u1"), &["commit", "-m", "u"]).unwrap();

        // A cedian path on a branch cedian never makes: omitted, never removed,
        // so a hand-made tree there (on `main`, say) is out of reach.
        git(
            &repo,
            &[
                "worktree",
                "add",
                repo.join(".worktrees/odd").to_str().unwrap(),
                "-b",
                "odd",
            ],
        )
        .unwrap();

        // Foreign: a worktree cedian did not make, outside `.worktrees/`.
        let name = repo.file_name().unwrap().to_string_lossy().to_string();
        let foreign = repo.with_file_name(format!("{name}-foreign"));
        let _ = std::fs::remove_dir_all(&foreign);
        git(
            &repo,
            &[
                "worktree",
                "add",
                foreign.to_str().unwrap(),
                "-b",
                "foreign",
            ],
        )
        .unwrap();

        (repo, unmerged, foreign)
    }

    fn classes(states: &[super::TreeState]) -> Vec<(String, TreeClass)> {
        states.iter().map(|s| (s.id.clone(), s.class)).collect()
    }

    #[test]
    fn classifies_the_four_and_omits_the_foreign_tree() {
        let (repo, _, foreign) = scene();
        let found = trees(&repo, "main").unwrap();
        let mut got = classes(&found);
        got.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            got,
            vec![
                ("m1".to_string(), TreeClass::Merged),
                ("s1".to_string(), TreeClass::Scratch),
                ("u1".to_string(), TreeClass::Unmerged),
                ("w1".to_string(), TreeClass::Wip),
            ]
        );
        assert!(
            !found.iter().any(|s| s.id == "foreign"),
            "the foreign worktree must be omitted, not classified"
        );
        assert!(
            !found.iter().any(|s| s.id == "odd"),
            "a cedian path on another branch is omitted too: {found:?}"
        );
        std::fs::remove_dir_all(&repo).unwrap();
        let _ = std::fs::remove_dir_all(&foreign);
    }

    #[test]
    fn cleanup_removes_merged_and_scratch_then_a_decided_wip() {
        let (repo, unmerged, foreign) = scene();

        // No decision: merged + scratch go, wip + unmerged stay.
        let out = cleanup(&repo, "main", &[]).unwrap();
        let removed: Vec<&str> = out
            .iter()
            .filter(|o| o.removed)
            .map(|o| o.id.as_str())
            .collect();
        let mut sorted = removed.clone();
        sorted.sort();
        assert_eq!(sorted, vec!["m1", "s1"]);
        assert!(!repo.join(".worktrees/m1").exists());
        assert!(!repo.join(".worktrees/s1").exists());
        assert!(repo.join(".worktrees/w1/a.txt").exists());
        assert!(repo.join(".worktrees/u1/u.txt").exists());

        let wip = out.iter().find(|o| o.id == "w1").unwrap();
        assert!(!wip.removed);
        assert!(
            wip.reason.contains("per-tree decision"),
            "wip reason names the decision: {:?}",
            wip.reason
        );
        let unmerged_out = out.iter().find(|o| o.id == "u1").unwrap();
        assert!(!unmerged_out.removed);
        assert!(
            unmerged_out.reason.contains("merge-back"),
            "unmerged reason names merge-back: {:?}",
            unmerged_out.reason
        );

        // An explicit decision removes the wip tree too.
        let out = cleanup(&repo, "main", &["w1".to_string()]).unwrap();
        let wip = out.iter().find(|o| o.id == "w1").unwrap();
        assert!(wip.removed, "decided wip is removed: {:?}", wip.reason);
        assert!(!repo.join(".worktrees/w1").exists());
        assert!(repo.join(".worktrees/u1/u.txt").exists());

        // Merge the last branch back; now a plain sweep clears it and a
        // repeated sweep finds nothing and removes nothing.
        merge_back(&repo, &unmerged, "main").unwrap();
        let out = cleanup(&repo, "main", &[]).unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].removed, "merged now: {:?}", out[0].reason);
        let repeat = cleanup(&repo, "main", &[]).unwrap();
        assert!(repeat.is_empty(), "second run finds nothing: {repeat:?}");
        assert!(trees(&repo, "main").unwrap().is_empty());

        std::fs::remove_dir_all(&repo).unwrap();
        let _ = std::fs::remove_dir_all(&foreign);
    }
}
