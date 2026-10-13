//! `cedian worker spawn` through the real CLI binary: the ADR-0033 brief is
//! checked before anything is made, and a good brief yields a worktree and a
//! `running` row titled with its goal. Hermetic: a temp repo and state dir.
#![allow(
    clippy::disallowed_methods,
    reason = "headless, synchronous process control (OMP, LSP, DAP, Chrome, git, sandbox-exec): Zed's async spawn helpers do not apply"
)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Root(PathBuf);

impl Root {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("cedian-worker-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let ws = dir.join("ws");
        std::fs::create_dir_all(&ws).unwrap();
        for args in [
            &["init", "-q", "-b", "main"][..],
            &["config", "user.email", "t@t"],
            &["config", "user.name", "t"],
        ] {
            assert!(git(&ws, args).status().unwrap().success(), "git {args:?}");
        }
        std::fs::write(ws.join("notes.txt"), "alpha\nbeta\n").unwrap();
        assert!(git(&ws, &["add", "."]).status().unwrap().success());
        assert!(
            git(&ws, &["commit", "-q", "-m", "base"])
                .status()
                .unwrap()
                .success()
        );
        std::fs::write(dir.join("cedian.toml"), "schema = 1\n").unwrap();
        Self(dir.canonicalize().unwrap())
    }

    fn ws(&self) -> PathBuf {
        self.0.join("ws")
    }

    fn brief(&self, name: &str, json: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, json).unwrap();
        path
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_cedian"))
            .args(args)
            .env("CEDIAN_CONFIG", self.0.join("cedian.toml"))
            .env("CEDIAN_WORKDIR", self.ws())
            .env("CEDIAN_SESSION_DIR", self.0.join("sessions"))
            .env("CEDIAN_STATE_DIR", self.0.join("state"))
            .env("CEDIAN_OMP_BINARY", "/nonexistent/omp")
            .env("HOME", self.0.join("home"))
            .env("PATH", "/usr/bin:/bin")
            .output()
            .unwrap()
    }

    fn registry(&self) -> PathBuf {
        let workspaces = self.0.join("state/workspaces");
        let mut dirs: Vec<PathBuf> = std::fs::read_dir(&workspaces)
            .map(|it| it.flatten().map(|e| e.path()).collect())
            .unwrap_or_default();
        assert!(dirs.len() <= 1, "one workspace per root: {dirs:?}");
        dirs.pop()
            .unwrap_or_else(|| workspaces.join("none-yet"))
            .join("workers.json")
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn git(dir: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(dir)
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE");
    cmd
}

fn text(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

const WHOLE_BRIEF: &str = r#"{
  "goal": "fix the casing",
  "scope": {"write": ["notes.txt"]},
  "acceptance": ["notes.txt keeps its casing"],
  "verify": ["cat notes.txt"],
  "timebox_min": 15
}"#;

#[test]
fn spawn_creates_the_tree_and_a_running_row_titled_by_the_goal() {
    let root = Root::new("ok");
    let brief = root.brief("b.json", WHOLE_BRIEF);
    let out = root.run(&["worker", "spawn", "w1", "--brief", brief.to_str().unwrap()]);
    assert!(out.status.success(), "{}", text(&out));
    assert!(root.ws().join(".worktrees/w1/notes.txt").exists());

    let reg: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.registry()).unwrap()).unwrap();
    let row = &reg["workers"]["w1"];
    assert_eq!(row["status"], "running");
    assert_eq!(row["task_title"], "fix the casing");
    assert_eq!(row["kind"], "worker", "--kind defaults to worker");
    assert_eq!(row["timebox_min"], 15.0);
    assert!(row["last_progress"].is_u64(), "spawn records progress");
    assert!(
        root.registry()
            .parent()
            .unwrap()
            .join("briefs/w1.1.json")
            .exists(),
        "the brief is stored beside the row"
    );
}

#[test]
fn a_kind_flag_names_the_row_kind() {
    let root = Root::new("kind");
    let brief = root.brief("b.json", WHOLE_BRIEF);
    let out = root.run(&[
        "worker",
        "spawn",
        "w1",
        "--kind",
        "bug_fix",
        "--brief",
        brief.to_str().unwrap(),
    ]);
    assert!(out.status.success(), "{}", text(&out));
    let reg: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.registry()).unwrap()).unwrap();
    assert_eq!(reg["workers"]["w1"]["kind"], "bug_fix");
}

#[test]
fn a_brief_without_verify_is_refused_and_creates_nothing() {
    let root = Root::new("nov");
    let brief = root.brief(
        "bad.json",
        r#"{"goal":"g","scope":{"write":["notes.txt"]},
            "acceptance":["done"],"timebox_min":5}"#,
    );
    let out = root.run(&["worker", "spawn", "w2", "--brief", brief.to_str().unwrap()]);
    assert!(!out.status.success());
    let err = text(&out);
    assert!(err.contains("verify"), "{err}");
    assert!(!root.ws().join(".worktrees/w2").exists());
    assert!(!root.registry().exists(), "no row was written");
}
