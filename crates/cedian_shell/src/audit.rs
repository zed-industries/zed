//! `audit.jsonl` in the state dir (ADR-0044; §64 mechanism 4 envelope): `kind: "tool"` rows for
//! each OMP tool execution start and end (ADR-0035 decision 4), and
//! `kind: "gate"` rows for each decision cedian's own gate makes (S3 gate
//! item 4): a host-tool call it serves (`allow`) and every dialog answered,
//! by the person in the app or by headless fail-closed (`answered_by`), or
//! left open when nobody could answer (`abstain`). `decision_source`
//! says which profile let a tool run: `omp` under `policy = "omp"`, `cedian`
//! under the default profile. Append-only.

use cedian_omp::{Approvals, RouterEvent};
use serde_json::{Value, json};
use std::io::Write as _;
use std::path::Path;

pub const AUDIT_FILE: &str = "audit.jsonl";

/// How much of a reviewer's tool arguments a row keeps.
const MAX_ARGS_CHARS: usize = 500;

pub struct AuditLog {
    path: std::path::PathBuf,
    file: std::fs::File,
    source: &'static str,
    reviewer: bool,
}

/// Next ordinal per audit file. A reviewer's log is open while the
/// implementer's is, so both draw from one sequence in this process.
static NEXT_ORDINAL: std::sync::Mutex<Option<std::collections::HashMap<std::path::PathBuf, u64>>> =
    std::sync::Mutex::new(None);

impl AuditLog {
    /// The log in `state_dir`, the workspace's state dir (`crate::state`).
    pub fn open(state_dir: &Path, approvals: Approvals) -> Result<Self, String> {
        let path = state_dir.join(AUDIT_FILE);
        let complete_rows = match std::fs::read_to_string(&path) {
            Ok(text) => text.lines().filter(|l| !l.trim().is_empty()).count() as u64,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
            Err(e) => return Err(format!("audit log {}: {e}", path.display())),
        };
        let mut next = NEXT_ORDINAL.lock().unwrap_or_else(|e| e.into_inner());
        let entry = next
            .get_or_insert_with(Default::default)
            .entry(path.clone())
            .or_insert(0);
        *entry = (*entry).max(complete_rows);
        drop(next);
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("audit log {}: {e}", path.display()))?;
        Ok(Self {
            path,
            file,
            source: match approvals {
                Approvals::Omp => "omp",
                Approvals::Cedian(_) | Approvals::Reviewer => "cedian",
            },
            reviewer: approvals == Approvals::Reviewer,
        })
    }

    /// Append the row for `event` if it is a tool start or end.
    pub fn record(&mut self, event: &RouterEvent) -> Result<(), String> {
        let item = match event {
            RouterEvent::ToolStart {
                tool_call_id,
                tool_name,
                args_preview,
                ..
            } => {
                let mut item = json!({"kind": "tool", "event": "start", "tool": tool_name, "tool_call_id": tool_call_id});
                // A reviewer's row says what it touched, not only which tool
                // (ADR-0043): a read of a credential path shows up here.
                if self.reviewer {
                    item["args"] = json!(
                        args_preview
                            .chars()
                            .take(MAX_ARGS_CHARS)
                            .collect::<String>()
                    );
                }
                self.append(item, None)?;
                if let Some((host_tool, true)) =
                    cedian_agent_ui::host_device(tool_name, args_preview)
                {
                    let gate = json!({
                        "kind": "gate", "tool": host_tool, "command": tool_call_id,
                        "decision": "allow", "scope": "once",
                    });
                    self.append(gate, None)?;
                }
                return Ok(());
            }
            RouterEvent::ToolEnd {
                tool_call_id,
                tool_name,
                is_error,
                ..
            } => json!({
                "kind": "tool", "event": "end", "tool": tool_name, "tool_call_id": tool_call_id, "is_error": is_error,
            }),
            _ => return Ok(()),
        };
        self.append(item, None)
    }

    /// The gate row for one dialog's outcome, stamped when it was answered.
    pub fn dialog(&mut self, record: &cedian_omp::DialogRecord) -> Result<(), String> {
        let item = json!({
            "kind": "gate", "tool": record.tool, "command": record.label,
            "decision": record.decision.as_str(), "scope": "once",
            "answered_by": record.answered_by.as_str(),
        });
        self.append(item, Some(record.at_ms).filter(|ms| *ms != 0))
    }

    /// One review cedian ran: the role, both sides' models and whether it
    /// was independent (ADR-0039 decision 4).
    pub fn review(
        &mut self,
        role: &str,
        reviewer_models: &[String],
        implementer_models: &[String],
        independent: bool,
    ) -> Result<(), String> {
        let item = json!({
            "kind": "review", "tool": "cedian_review_request", "role": role,
            "reviewer_models": reviewer_models, "implementer_models": implementer_models,
            "independent": independent,
        });
        self.append(item, None)
    }

    /// A review cedian refused to run because the workspace carries OMP
    /// system-prompt files (ADR-0053).
    pub fn review_refused(&mut self, files: &[String]) -> Result<(), String> {
        let item = json!({
            "kind": "review", "tool": "cedian_review_request", "refused": files,
            "outcome": "inconclusive",
        });
        self.append(item, None)
    }

    /// The person cancelled OMP subagent `id` (ADR-0050 decision 4):
    /// OMP's answer, `false` when it had already ended, or why the cancel
    /// did not reach OMP.
    pub fn subagent_cancel(
        &mut self,
        id: &str,
        outcome: &Result<bool, String>,
    ) -> Result<(), String> {
        let mut item = json!({
            "kind": "gate", "tool": "cancel_subagent", "command": id,
            "decision": "cancel", "scope": "once", "answered_by": "user",
        });
        match outcome {
            Ok(cancelled) => item["cancelled"] = json!(cancelled),
            Err(e) => item["error"] = json!(e),
        }
        self.append(item, None)
    }

    /// A person dismissed a review finding: they let the change through
    /// over it, with a reason (ADR-0011).
    pub fn dismissal(&mut self, finding: &str, reason: &str) -> Result<(), String> {
        let item = json!({
            "kind": "gate", "tool": "cedian_review_finding",
            "command": format!("dismiss {finding}: {reason}"), "decision": "allow", "scope": "once",
        });
        self.append(item, None)
    }

    /// A changed file outside the running workers' briefs (ADR-0033
    /// decision 3, ADR-0063 decision 4): the call that wrote it, the
    /// worktree-relative path, and the workers the turn's scope was built
    /// from (the interim names all of them, not the one it belongs to).
    pub fn out_of_scope(
        &mut self,
        tool_call_id: &str,
        path: &str,
        workers: &[String],
    ) -> Result<(), String> {
        let item = json!({
            "kind": "scope", "event": "out_of_scope", "tool_call_id": tool_call_id,
            "path": path, "workers": workers,
        });
        self.append(item, None)
    }

    fn append(&mut self, mut item: Value, at_ms: Option<u64>) -> Result<(), String> {
        item["decision_source"] = json!(self.source);
        if self.reviewer {
            item["actor"] = json!("reviewer");
        }
        let timestamp_ms = at_ms.unwrap_or_else(cedian_omp::now_ms);
        // Hold the lock across the write so ordinals land in file order.
        let mut next = NEXT_ORDINAL.lock().unwrap_or_else(|e| e.into_inner());
        let ordinal = next
            .get_or_insert_with(Default::default)
            .entry(self.path.clone())
            .or_insert(0);
        let row = json!({"timestamp_ms": timestamp_ms, "ordinal": *ordinal, "item": item});
        writeln!(self.file, "{row}").map_err(|e| format!("audit log append: {e}"))?;
        *ordinal += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh, canonical temp directory per test, removed on drop.
    struct TestDir(std::path::PathBuf);

    impl TestDir {
        fn new(tag: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("cedian-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            Self(std::fs::canonicalize(&dir).unwrap())
        }
    }

    impl std::ops::Deref for TestDir {
        type Target = Path;
        fn deref(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Read the file back: every line parses and ordinals run 0, 1, 2, ….
    fn replay(dir: &Path) -> Vec<Value> {
        let rows: Vec<Value> = std::fs::read_to_string(dir.join(AUDIT_FILE))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row["ordinal"], i as u64, "ordinals are contiguous");
        }
        rows
    }

    #[test]
    fn rows_carry_envelope_source_and_continue_ordinals() {
        let dir = TestDir::new("audit");
        let start = RouterEvent::ToolStart {
            tool_call_id: "c1".into(),
            tool_name: "bash".into(),
            args_preview: "touch probe".into(),
            paths: Vec::new(),
        };
        let end = RouterEvent::ToolEnd {
            tool_call_id: "c1".into(),
            tool_name: "bash".into(),
            result_summary: String::new(),
            is_error: false,
            before: Vec::new(),
        };
        let mut log = AuditLog::open(&dir, Approvals::Omp).unwrap();
        log.record(&start).unwrap();
        log.record(&RouterEvent::Settled).unwrap();
        log.record(&end).unwrap();
        drop(log);
        let mut log = AuditLog::open(&dir, Approvals::Cedian(Default::default())).unwrap();
        log.record(&start).unwrap();
        drop(log);

        let rows = replay(&dir);
        assert_eq!(rows.len(), 3, "non-tool events write nothing");
        let ordinals: Vec<_> = rows
            .iter()
            .map(|r| r["ordinal"].as_u64().unwrap())
            .collect();
        assert_eq!(ordinals, [0, 1, 2]);
        assert_eq!(rows[0]["item"]["decision_source"], "omp");
        assert_eq!(rows[1]["item"]["event"], "end");
        assert_eq!(rows[1]["item"]["is_error"], false);
        assert_eq!(rows[2]["item"]["decision_source"], "cedian");
        assert!(rows[0]["timestamp_ms"].as_u64().unwrap() > 0);
    }

    #[test]
    fn two_logs_on_one_file_share_the_ordinal_sequence() {
        let dir = TestDir::new("audit-two");
        let start = |id: &str| RouterEvent::ToolStart {
            tool_call_id: id.into(),
            tool_name: "read".into(),
            args_preview: "notes.txt".into(),
            paths: Vec::new(),
        };
        let mut implementer = AuditLog::open(&dir, Approvals::Cedian(Default::default())).unwrap();
        implementer.record(&start("c1")).unwrap();
        let mut reviewer = AuditLog::open(&dir, Approvals::Reviewer).unwrap();
        reviewer.record(&start("r1")).unwrap();
        implementer.record(&start("c2")).unwrap();
        drop((implementer, reviewer));
        let rows = replay(&dir);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1]["item"]["actor"], "reviewer");
        assert_eq!(
            rows[1]["item"]["args"], "notes.txt",
            "a reviewer row says what it read"
        );
        assert_eq!(rows[2]["item"].get("actor"), None);
        assert_eq!(rows[2]["item"].get("args"), None);
    }

    #[test]
    fn gate_rows_for_a_host_tool_call_and_a_refused_dialog() {
        let dir = TestDir::new("audit-gate");
        let mut log = AuditLog::open(&dir, Approvals::Cedian(Default::default())).unwrap();
        log.record(&RouterEvent::ToolStart {
            tool_call_id: "c1".into(),
            tool_name: "write".into(),
            args_preview: "xd://cedian_worktree_request".into(),
            paths: Vec::new(),
        })
        .unwrap();
        log.dialog(&cedian_omp::DialogRecord {
            label: "Allow tool: bash — Command: rm -rf x".into(),
            tool: Some("bash".into()),
            decision: cedian_omp::GateDecision::Deny,
            answered_by: cedian_omp::Answerer::Cedian,
            at_ms: 42,
        })
        .unwrap();
        log.dialog(&cedian_omp::DialogRecord {
            label: "Name?".into(),
            tool: None,
            decision: cedian_omp::GateDecision::Abstain,
            answered_by: cedian_omp::Answerer::Cedian,
            at_ms: 43,
        })
        .unwrap();
        log.dialog(&cedian_omp::DialogRecord {
            label: "Allow tool: bash — Command: ls".into(),
            tool: Some("bash".into()),
            decision: cedian_omp::GateDecision::Allow,
            answered_by: cedian_omp::Answerer::User,
            at_ms: 44,
        })
        .unwrap();
        drop(log);

        let rows = replay(&dir);
        let gates: Vec<&Value> = rows
            .iter()
            .filter(|r| r["item"]["kind"] == "gate")
            .collect();
        assert_eq!(rows[0]["item"]["kind"], "tool");
        assert_eq!(gates.len(), 4, "{rows:?}");
        assert_eq!(gates[0]["item"]["tool"], "cedian_worktree_request");
        assert_eq!(
            gates[0]["item"]["decision"], "allow",
            "cedian served its host tool"
        );
        assert_eq!(gates[0]["item"]["scope"], "once");
        assert_eq!(gates[1]["item"]["tool"], "bash");
        assert_eq!(
            gates[1]["item"]["command"],
            "Allow tool: bash — Command: rm -rf x"
        );
        assert_eq!(gates[1]["item"]["decision"], "deny");
        assert_eq!(
            gates[1]["timestamp_ms"], 42,
            "stamped when headless replied"
        );
        assert_eq!(gates[1]["item"]["answered_by"], "cedian");
        assert_eq!(gates[2]["item"]["decision"], "abstain");
        assert_eq!(gates[2]["item"]["tool"], Value::Null);
        assert_eq!(
            (
                &gates[3]["item"]["decision"],
                &gates[3]["item"]["answered_by"]
            ),
            (&json!("allow"), &json!("user")),
            "the person approved it in the app"
        );
    }

    /// ADR-0063 decision 4: an out-of-scope write is one row naming the call,
    /// the path, and the worker(s) the turn's scope was built from.
    #[test]
    fn an_out_of_scope_write_is_one_row() {
        let dir = TestDir::new("audit-scope");
        let mut log = AuditLog::open(&dir, Approvals::Omp).unwrap();
        log.out_of_scope("c7", "/src/main.rs", &["w1".to_string()])
            .unwrap();
        drop(log);
        let rows = replay(&dir);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["item"]["kind"], "scope");
        assert_eq!(rows[0]["item"]["event"], "out_of_scope");
        assert_eq!(rows[0]["item"]["tool_call_id"], "c7");
        assert_eq!(rows[0]["item"]["path"], "/src/main.rs");
        assert_eq!(rows[0]["item"]["workers"], json!(["w1"]));
    }
}
