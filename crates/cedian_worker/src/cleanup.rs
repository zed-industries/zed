//! Worker cleanup (S5 exit): classify each cedian worktree and clean one at a time.
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

/// What [`cleanup_one`] did (or declined to do) with one tree.
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

/// Clean up exactly one tree, this call being its explicit per-tree decision
/// when it is a `Wip` (§43). `decided` is what the caller decided; a tree that
/// is not there is a refusal naming the id, never a silent success.
pub fn cleanup_one(
    repo: &Path,
    base: &str,
    id: &str,
    decided: bool,
) -> Result<CleanupOutcome, WorkerError> {
    let tree = trees(repo, base)?
        .into_iter()
        .find(|tree| tree.id == id)
        .ok_or_else(|| WorkerError::NoSuch(id.to_string()))?;
    Ok(decide(repo, base, tree, decided))
}

/// What one tree's verdict is, and do it. One home for the rule: `Merged` and
/// `Scratch` go, a `Wip` goes only when the caller decided, `Unmerged` is left
/// for merge-back, and a removal git refuses is reported rather than fatal.
fn decide(repo: &Path, base: &str, tree: TreeState, decided: bool) -> CleanupOutcome {
    let (removed, reason) = match tree.class {
        TreeClass::Merged | TreeClass::Scratch => {
            match force_remove(repo, &tree.path, &tree.branch) {
                Ok(()) => (true, String::new()),
                Err(e) => (false, format!("git refused: {e}")),
            }
        }
        TreeClass::Unmerged => (
            false,
            format!(
                "commits not in {base}: merge-back first, then remove {}",
                tree.branch
            ),
        ),
        TreeClass::Wip if decided => match force_remove(repo, &tree.path, &tree.branch) {
            Ok(()) => (true, String::new()),
            Err(e) => (false, format!("git refused: {e}")),
        },
        TreeClass::Wip => (
            false,
            format!(
                "tracked edits: needs an explicit per-tree decision for {}",
                tree.id
            ),
        ),
    };
    CleanupOutcome {
        id: tree.id,
        path: tree.path,
        class: tree.class,
        removed,
        reason,
    }
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
    use super::{TreeClass, cleanup_one, trees};
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
    fn trees_classify_the_four_and_cleanup_one_leaves_the_rest_alone() {
        let (repo, _, foreign) = scene();
        let mut got: Vec<(String, TreeClass)> = trees(&repo, "main")
            .unwrap()
            .into_iter()
            .map(|tree| (tree.id, tree.class))
            .collect();
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

        // Merged and scratch go when their own row is cleaned; unmerged waits
        // for merge-back.
        assert!(cleanup_one(&repo, "main", "m1", false).unwrap().removed);
        assert!(cleanup_one(&repo, "main", "s1", false).unwrap().removed);
        let unmerged = cleanup_one(&repo, "main", "u1", true).unwrap();
        assert!(!unmerged.removed);
        assert!(unmerged.reason.contains("merge-back"), "{unmerged:?}");

        // The last branch merges back; then its own row cleans it and the repo
        // holds no cedian tree.
        let head = crate::registry::WorkerHead {
            id: "u1".to_string(),
            branch: "cedian-worker/u1".to_string(),
            worktree: ".worktrees/u1".to_string(),
            task_title: String::new(),
            kind: String::new(),
            status: crate::registry::WorkerStatus::Running,
            note: String::new(),
            timebox_min: 0.0,
            last_progress: None,
        };
        crate::worktree::merge_back(&repo, &head, "main").unwrap();
        assert!(cleanup_one(&repo, "main", "u1", false).unwrap().removed);
        let left = trees(&repo, "main").unwrap();
        assert_eq!(left.len(), 1, "only the wip tree is left: {left:?}");
        assert_eq!(left[0].id, "w1");

        std::fs::remove_dir_all(&repo).unwrap();
        let _ = std::fs::remove_dir_all(&foreign);
    }

    #[test]
    fn cleanup_one_acts_on_the_named_tree_and_leaves_its_siblings() {
        let (repo, _, foreign) = scene();

        // The clicked tree is `wip`: nothing is removed without the decision,
        // and the clean trees beside it stay, which is what one row's click
        // decides (the repo-wide sweep is `cleanup`).
        let refused = cleanup_one(&repo, "main", "w1", false).unwrap();
        assert_eq!(refused.class, TreeClass::Wip);
        assert!(!refused.removed);
        assert!(refused.reason.contains("per-tree decision"), "{refused:?}");
        for kept in ["w1", "m1", "s1", "u1"] {
            assert!(
                repo.join(".worktrees").join(kept).exists(),
                "{kept} is untouched"
            );
        }

        let decided = cleanup_one(&repo, "main", "w1", true).unwrap();
        assert!(decided.removed, "{decided:?}");
        assert!(!repo.join(".worktrees/w1").exists());
        for kept in ["m1", "s1", "u1"] {
            assert!(
                repo.join(".worktrees").join(kept).exists(),
                "{kept} is still there after one tree's cleanup"
            );
        }

        // An unmerged tree is never the decision's to remove, and an id with
        // no tree is a refusal, not a silent success.
        let unmerged = cleanup_one(&repo, "main", "u1", true).unwrap();
        assert!(!unmerged.removed);
        assert!(unmerged.reason.contains("merge-back"), "{unmerged:?}");
        assert!(matches!(
            cleanup_one(&repo, "main", "nope", true),
            Err(crate::registry::WorkerError::NoSuch(id)) if id == "nope"
        ));

        std::fs::remove_dir_all(&repo).unwrap();
        let _ = std::fs::remove_dir_all(&foreign);
    }
}
