//! `cedian_worktree_request` (P5, ADR-0022 / ADR-0009): OMP asks, cedian
//! creates. Mechanism only — the same `spawn` + [`Registry`] row the
//! `cedian worker spawn` verb uses; OMP keeps the orchestration policy.

use crate::brief::{Brief, BriefError, REQUIRED_FIELDS};
use crate::worktree::discard;
use crate::{Registry, WorkerError, WorkerStatus, spawn, validate_id};
use omp_rpc::HostTool;
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Host tool name (listed in `cedian_workflow::CHANNEL_TOOLS`).
pub const WORKTREE_REQUEST_TOOL: &str = "cedian_worktree_request";

/// Who answers a worktree request when the project's writes need a person
/// (ADR-0012). `ask` blocks its caller, which is a host-tool thread of its
/// own; a timeout or a dropped channel is a denial.
pub trait WorktreeGate: Send + Sync {
    fn allow(&self, id: &str, goal: &str) -> bool;

    /// The refusal OMP sees for a denied request. The default names the
    /// person a panel asked; a headless caller says nobody was there.
    fn refused(&self, id: &str) -> String {
        format!("no worktree {id}: a person denied the request")
    }
}

/// The gate `worktree_request_tool` uses: today's `allow` behaviour.
struct AlwaysAllow;

impl WorktreeGate for AlwaysAllow {
    fn allow(&self, _id: &str, _goal: &str) -> bool {
        true
    }
}

/// Create worktree `id` off `base` (default `HEAD`) for the brief in
/// `args`, record it in the registry under `state` (the repo's cedian state
/// dir) and store the brief beside it as revision 1. A request without a
/// whole brief, or whose id the registry already has, is refused BEFORE
/// `git worktree add`; the brief is stored before the tree is made, and a
/// later failure takes tree and brief back, so a failed request leaves
/// neither and every row has its brief (ADR-0033). The registry lock is
/// held throughout: OMP runs host tool calls in parallel.
pub fn request_worktree(
    repo: &Path,
    state: &Path,
    args: &Map<String, Value>,
) -> Result<String, String> {
    let id = args.get("id").and_then(Value::as_str).unwrap_or("");
    let brief = checked_brief(id, args)?;
    let text = |key: &str| args.get(key).and_then(Value::as_str).unwrap_or("");
    let base = match text("base") {
        "" => "HEAD",
        b => b,
    };
    let _lock = Registry::lock(state).map_err(|e| e.to_string())?;
    let (mut reg, _) = Registry::open(state).map_err(|e| e.to_string())?;
    if reg.get(id).is_some() {
        return Err(WorkerError::Exists(id.to_string()).to_string());
    }
    validate_id(id).map_err(|e| e.to_string())?;
    let brief_path = brief.store(state, id, 1)?;
    let mut head = match spawn(repo, id, base) {
        Ok(head) => head,
        Err(e) => {
            let _ = std::fs::remove_file(&brief_path);
            return Err(e.to_string());
        }
    };
    head.status = WorkerStatus::Running;
    head.task_title = brief.goal.clone();
    head.kind = text("kind").to_string();
    head.timebox_min = brief.timebox_min;
    let reply = format!(
        "worktree {} at {} (branch {}, base {base})",
        head.id,
        repo.join(&head.worktree).display(),
        head.branch
    );
    let saved = reg
        .insert(head.clone())
        .and_then(|()| reg.save(state))
        .map_err(|e| e.to_string());
    if let Err(e) = saved {
        let _ = std::fs::remove_file(&brief_path);
        return Err(match discard(repo, &head) {
            Ok(()) => e,
            Err(left) => format!("{e}; its tree was left behind: {left}"),
        });
    }
    Ok(reply)
}

/// The brief in `args`, refused with the brief's error before anything else
/// (ADR-0033 decision 1).
fn checked_brief(id: &str, args: &Map<String, Value>) -> Result<Brief, String> {
    let brief = match Brief::from_json(&Value::Object(args.clone()).to_string()) {
        Ok(brief) => brief,
        Err(BriefError::Missing(mut fields)) => {
            // `id` is the tunnel that names the worker, not a brief field.
            if id.is_empty() {
                fields.insert(0, "id".to_string());
            }
            return Err(BriefError::Missing(fields).to_string());
        }
        Err(e) => return Err(e.to_string()),
    };
    if id.is_empty() {
        return Err(BriefError::Missing(vec!["id".to_string()]).to_string());
    }
    Ok(brief)
}

/// One request through `gate`: the brief is checked first (a missing field
/// is the brief's error, never a person's question), then the gate, then
/// `request_worktree`. A denied request creates nothing.
fn request_with_gate(
    repo: &Path,
    state: &Path,
    gate: &dyn WorktreeGate,
    args: &Map<String, Value>,
) -> Result<String, String> {
    let id = args.get("id").and_then(Value::as_str).unwrap_or("");
    let brief = checked_brief(id, args)?;
    if !gate.allow(id, &brief.goal) {
        return Err(gate.refused(id));
    }
    request_worktree(repo, state, args)
}

/// The host tool over `repo` (the repo root, like `cedian worker`), with a
/// gate that asks before a tree is made (ADR-0012).
pub fn worktree_request_tool_with_gate(
    repo: PathBuf,
    state: PathBuf,
    gate: Arc<dyn WorktreeGate>,
) -> HostTool {
    let lines = |what: &str| json!({"type": "array", "items": {"type": "string"}, "minItems": 1, "description": what});
    // The `required` list is built from the same fields the refusal checks,
    // so the schema cannot drift from the validator (ADR-0033 decision 1).
    let mut required = vec!["id".to_string()];
    for field in REQUIRED_FIELDS {
        let top = field.split('.').next().unwrap_or(field);
        if !required.iter().any(|r| r == top) {
            required.push(top.to_string());
        }
    }
    let params = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "description": "worker id: [A-Za-z0-9._-]"},
            "goal": {"type": "string", "description": "one sentence: what the worker is for"},
            "scope": {
                "type": "object",
                "properties": {
                    "write": lines("path globs the worker may write"),
                    "deny": lines("path globs it must not write")
                },
                "required": ["write"]
            },
            "acceptance": lines("checkable lines that say it is done"),
            "verify": lines("commands or verification-profile ids that check it"),
            "timebox_min": {"type": "number", "description": "minutes before it shows as stuck"},
            "context": lines("file and PR pointers"),
            "forbidden": lines("what it must not do"),
            "kind": {"type": "string"},
            "base": {"type": "string", "description": "branch or commit (default HEAD)"}
        },
        "required": required,
        "additionalProperties": false
    })
    .as_object()
    .cloned()
    .unwrap_or_default();
    HostTool::new(
        WORKTREE_REQUEST_TOOL,
        "cedian's own host tool (trusted). Ask the cedian IDE for an isolated git worktree for a \
         parallel worker, with its brief (goal, scope.write, acceptance, verify, timebox_min); \
         never run `git worktree add` yourself. Returns the worktree id and path.",
        params,
        move |args, _ctx| {
            request_with_gate(&repo, &state, &*gate, &args)
                .map(Into::into)
                .map_err(Into::into)
        },
    )
}

/// The host tool whose gate always allows: today's `allow` behaviour.
pub fn worktree_request_tool(repo: PathBuf, state: PathBuf) -> HostTool {
    worktree_request_tool_with_gate(repo, state, Arc::new(AlwaysAllow))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::worktree::tests::fixture;

    fn state(repo: &Path) -> PathBuf {
        repo.with_extension("state")
    }

    fn args(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    fn brief(id: &str) -> Value {
        json!({
            "id": id, "goal": "try it", "scope": {"write": ["src/**"]},
            "acceptance": ["tests pass"], "verify": ["cargo test"], "timebox_min": 30,
        })
    }

    #[test]
    fn request_creates_registers_and_stores_the_brief() {
        let repo = fixture();
        let out = request_worktree(&repo, &state(&repo), &args(brief("w1"))).unwrap();
        assert!(out.contains("branch cedian-worker/w1"), "{out}");
        assert!(repo.join(".worktrees/w1/a.txt").exists());
        let (reg, existed) = Registry::open(&state(&repo)).unwrap();
        assert!(existed);
        let head = reg.get("w1").unwrap();
        assert_eq!(head.task_title, "try it");
        assert_eq!(head.status, WorkerStatus::Running);
        assert_eq!(head.timebox_min, 30.0);
        assert!(head.last_progress.is_some(), "spawn is the first progress");
        // The stored revision is the typed brief's canonical serialization,
        // not the request envelope (no id/kind/base).
        let canonical =
            serde_json::to_value(Brief::from_json(&brief("w1").to_string()).unwrap()).unwrap();
        let stored = std::fs::read_to_string(state(&repo).join("briefs/w1.1.json")).unwrap();
        assert_eq!(serde_json::from_str::<Value>(&stored).unwrap(), canonical);
    }

    #[test]
    fn a_brief_less_request_names_every_required_field_and_makes_no_tree() {
        let repo = fixture();
        let err = request_worktree(
            &repo,
            &state(&repo),
            &args(json!({"id": "bare", "goal": "no brief"})),
        )
        .unwrap_err();
        assert_eq!(
            err,
            "no worktree: the brief is missing scope.write, acceptance, verify, timebox_min (ADR-0033)"
        );
        assert!(!repo.join(".worktrees/bare").exists());
        assert!(
            Registry::open(&state(&repo))
                .unwrap()
                .0
                .get("bare")
                .is_none()
        );
    }

    #[test]
    fn a_wrong_typed_brief_is_refused_and_makes_nothing() {
        let repo = fixture();
        let mut bad = brief("w1");
        bad.as_object_mut()
            .unwrap()
            .insert("scope".into(), json!({"write": "src/**"}));
        let err = request_worktree(&repo, &state(&repo), &args(bad)).unwrap_err();
        assert!(err.contains("malformed"), "{err}");
        assert!(!repo.join(".worktrees/w1").exists());
        assert!(Registry::open(&state(&repo)).unwrap().0.get("w1").is_none());
        assert!(!state(&repo).join("briefs/w1.1.json").exists());
    }

    #[test]
    fn a_request_without_a_whole_brief_names_every_missing_field_and_makes_no_tree() {
        let repo = fixture();
        let err = request_worktree(
            &repo,
            &state(&repo),
            &args(json!({"id": "w1", "goal": "try it", "scope": {"write": []}, "verify": [""]})),
        )
        .unwrap_err();
        assert!(
            err.contains("missing scope.write, acceptance, verify, timebox_min"),
            "{err}"
        );
        assert!(!repo.join(".worktrees/w1").exists());
        assert!(Registry::open(&state(&repo)).unwrap().0.get("w1").is_none());
        assert!(!state(&repo).join("briefs/w1.1.json").exists());
    }

    #[test]
    fn parallel_requests_each_keep_their_row() {
        let repo = fixture();
        let ids: Vec<String> = (0..8).map(|n| format!("p{n}")).collect();
        let results: Vec<Result<String, String>> = std::thread::scope(|s| {
            let handles: Vec<_> = ids
                .iter()
                .map(|id| {
                    let repo = &repo;
                    s.spawn(move || request_worktree(repo, &state(repo), &args(brief(id))))
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for result in &results {
            assert!(result.is_ok(), "{result:?}");
        }
        let (reg, _) = Registry::open(&state(&repo)).unwrap();
        let rows: Vec<&str> = reg.all().map(|head| head.id.as_str()).collect();
        assert_eq!(rows, ids, "a parallel request lost its row");
    }

    #[test]
    fn a_brief_that_cannot_be_stored_leaves_no_tree_and_no_row() {
        let repo = fixture();
        let stuck = state(&repo).join("briefs/w1.1.json");
        std::fs::create_dir_all(&stuck).unwrap();
        let err = request_worktree(&repo, &state(&repo), &args(brief("w1"))).unwrap_err();
        assert!(err.contains("brief"), "{err}");
        assert!(
            !repo.join(".worktrees/w1").exists(),
            "no tree without its brief"
        );
        assert!(Registry::open(&state(&repo)).unwrap().0.get("w1").is_none());
        std::fs::remove_dir(&stuck).unwrap();
        request_worktree(&repo, &state(&repo), &args(brief("w1"))).expect("a retry succeeds");
        assert!(stuck.is_file());
    }

    #[test]
    fn a_stored_brief_is_never_overwritten() {
        let repo = fixture();
        let kept = state(&repo).join("briefs/w1.1.json");
        std::fs::create_dir_all(kept.parent().unwrap()).unwrap();
        std::fs::write(&kept, "the removed worker's brief").unwrap();
        let err = request_worktree(&repo, &state(&repo), &args(brief("w1"))).unwrap_err();
        assert!(err.contains("exists"), "{err}");
        assert_eq!(
            std::fs::read_to_string(&kept).unwrap(),
            "the removed worker's brief"
        );
        assert!(!repo.join(".worktrees/w1").exists());
    }

    #[test]
    fn duplicate_and_bad_ids_are_refused() {
        let repo = fixture();
        request_worktree(&repo, &state(&repo), &args(brief("w1"))).unwrap();
        let dup = request_worktree(&repo, &state(&repo), &args(brief("w1"))).unwrap_err();
        assert!(dup.contains("w1"), "{dup}");
        assert!(request_worktree(&repo, &state(&repo), &args(brief("../x"))).is_err());
        assert!(!repo.join(".worktrees/x").exists());
        let mut no_id = brief("");
        no_id.as_object_mut().unwrap().remove("id");
        let err = request_worktree(&repo, &state(&repo), &args(no_id)).unwrap_err();
        assert!(err.contains("missing id"), "{err}");
    }

    /// Counts the questions it is asked, so a test can prove the brief is
    /// validated before the gate.
    struct RecordingGate {
        allow: bool,
        asked: std::sync::atomic::AtomicUsize,
    }

    impl RecordingGate {
        fn new(allow: bool) -> Self {
            Self {
                allow,
                asked: std::sync::atomic::AtomicUsize::new(0),
            }
        }
    }

    impl WorktreeGate for RecordingGate {
        fn allow(&self, _id: &str, _goal: &str) -> bool {
            self.asked
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.allow
        }
    }

    #[test]
    fn a_gate_that_allows_makes_the_tree_and_row() {
        let repo = fixture();
        let gate = RecordingGate::new(true);
        let out = request_with_gate(&repo, &state(&repo), &gate, &args(brief("w1"))).unwrap();
        assert!(out.contains("branch cedian-worker/w1"), "{out}");
        assert!(repo.join(".worktrees/w1/a.txt").exists());
        assert!(Registry::open(&state(&repo)).unwrap().0.get("w1").is_some());
        assert_eq!(gate.asked.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[test]
    fn a_gate_that_denies_names_the_id_and_makes_nothing() {
        let repo = fixture();
        let gate = RecordingGate::new(false);
        let err = request_with_gate(&repo, &state(&repo), &gate, &args(brief("w1"))).unwrap_err();
        assert!(err.contains("w1"), "{err}");
        assert!(!repo.join(".worktrees/w1").exists());
        assert!(Registry::open(&state(&repo)).unwrap().0.get("w1").is_none());
        assert!(!state(&repo).join("briefs/w1.1.json").exists());
    }

    #[test]
    fn a_missing_brief_field_is_refused_before_the_gate_is_asked() {
        let repo = fixture();
        let gate = RecordingGate::new(true);
        let err = request_with_gate(
            &repo,
            &state(&repo),
            &gate,
            &args(json!({"id": "bare", "goal": "no brief"})),
        )
        .unwrap_err();
        assert!(
            err.contains("scope.write, acceptance, verify, timebox_min"),
            "{err}"
        );
        assert_eq!(
            gate.asked.load(std::sync::atomic::Ordering::Relaxed),
            0,
            "the brief is checked before the gate"
        );
        assert!(!repo.join(".worktrees/bare").exists());
    }
}
