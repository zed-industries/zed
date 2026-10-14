//! Evidence as first-class data (§53): every gate decision reads evidence
//! items already stored in the `WorkflowState` evidence map. Each item carries
//! provenance (§53 R2 fix): a tool-call link or `unattributed`; the code
//! state it verified and a three-way outcome (ADR-0024).

use crate::code_state::{CodeState, CurrentState};
use serde::{Deserialize, Serialize};

/// Provenance: link to the tool call that produced it, or unattributed.
///
/// Required gates REJECT unattributed evidence; optional gates accept it but
/// flag `unverified-origin` in the `GateResult`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    /// Produced by a tracked tool call (`task_id` + `tool_call_id`).
    Attributed {
        task_id: String,
        tool_call_id: String,
    },
    /// No tracked origin (hand-added note, external paste). Never passes a
    /// required gate.
    Unattributed,
}

/// Evidence kind (§53): command < test < browser/screenshot < simulator <
/// debugger < file. `Custom` covers future kinds without schema churn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Command,
    Test,
    Browser,
    Screenshot,
    Simulator,
    Debugger,
    File,
    Custom,
}

/// What the observation showed (ADR-0024 decision 2). "Could not run" is
/// `Inconclusive`: never a pass, never silently a fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Pass,
    Fail,
    Inconclusive,
}

impl Outcome {
    /// `ok: true|false` → `pass|fail` (the P5 wire form).
    pub fn from_ok(ok: bool) -> Self {
        if ok { Self::Pass } else { Self::Fail }
    }
}

/// A performance number (ADR-0024 decision 4). Every field is needed; a
/// performance gate treats a measurement with any field missing as
/// `inconclusive`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub runs: Option<u32>,
    pub median: Option<f64>,
    /// `[min, max]` over the runs.
    pub range: Option<[f64; 2]>,
    /// What bounds the number (cpu, io, network, lock, …).
    pub limiter: Option<String>,
    /// e.g. `release`; must be production-like to mean anything.
    pub build_profile: Option<String>,
}

impl Measurement {
    /// Names of the fields still missing (empty = complete).
    pub fn missing(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.runs.is_none_or(|n| n == 0) {
            out.push("runs");
        }
        if self.median.is_none() {
            out.push("median");
        }
        if self.range.is_none() {
            out.push("range");
        }
        if self.limiter.as_deref().is_none_or(str::is_empty) {
            out.push("limiter");
        }
        if self.build_profile.as_deref().is_none_or(str::is_empty) {
            out.push("build_profile");
        }
        out
    }
}

/// One evidence item: what was observed, which gate(s) it supports, where it
/// came from and which code it saw. A `Fail` outcome records a FAILING
/// observation (e.g. a reproduction that still fails) — gates read it, they
/// never re-run it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    /// Stable id within the workflow (dedup key for the continue rule).
    pub id: String,
    pub kind: EvidenceKind,
    /// Gate ids this item supports (empty = informational only).
    pub for_gates: Vec<String>,
    /// Human-readable observation ("repro fails with E0502", "cargo test
    /// auth: 14 passed"). Gates match on structured fields, not prose.
    pub summary: String,
    pub outcome: Outcome,
    pub provenance: Provenance,
    /// Code the producing call saw; `None` = unknown, which counts as stale.
    #[serde(default)]
    pub code_state: Option<CodeState>,
    /// Set when the item was stale on arrival (a mutating call ran between
    /// the observation and the report).
    #[serde(default)]
    pub born_stale: Option<String>,
    /// Performance numbers, when the item is a measurement.
    #[serde(default)]
    pub measurement: Option<Measurement>,
    /// The verification-profile feature this item proves (ADR-0025).
    #[serde(default)]
    pub feature: Option<crate::verification::FeatureRef>,
    /// The browser frame sequence a capture was taken at (§29 R4); a later
    /// navigation makes it `stale-frame`.
    #[serde(default)]
    pub frame_seq: Option<u64>,
    /// The producing call's command / args preview (bash, eval, …). Class
    /// proof (ADR-0032) requires both proof runs to carry the enforcer's
    /// command. Absent on old rows and on non-exec observations.
    #[serde(default)]
    pub command: Option<String>,
}

impl Evidence {
    /// New attributed item supporting the given gates.
    pub fn attributed(
        id: impl Into<String>,
        kind: EvidenceKind,
        for_gates: &[&str],
        summary: impl Into<String>,
        outcome: Outcome,
        task_id: impl Into<String>,
        tool_call_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            for_gates: for_gates.iter().map(|s| s.to_string()).collect(),
            summary: summary.into(),
            outcome,
            provenance: Provenance::Attributed {
                task_id: task_id.into(),
                tool_call_id: tool_call_id.into(),
            },
            code_state: None,
            born_stale: None,
            measurement: None,
            feature: None,
            frame_seq: None,
            command: None,
        }
    }

    /// New unattributed item (hand note / external paste).
    pub fn unattributed(
        id: impl Into<String>,
        kind: EvidenceKind,
        for_gates: &[&str],
        summary: impl Into<String>,
        outcome: Outcome,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            for_gates: for_gates.iter().map(|s| s.to_string()).collect(),
            summary: summary.into(),
            outcome,
            provenance: Provenance::Unattributed,
            code_state: None,
            born_stale: None,
            measurement: None,
            feature: None,
            frame_seq: None,
            command: None,
        }
    }

    /// Bind to the code state the producing call saw.
    pub fn with_code_state(mut self, state: CodeState) -> Self {
        self.code_state = Some(state);
        self
    }

    /// Record the producing call's command / args preview.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        let c = command.into();
        self.command = if c.is_empty() { None } else { Some(c) };
        self
    }

    pub fn is_attributed(&self) -> bool {
        matches!(self.provenance, Provenance::Attributed { .. })
    }

    /// `Some(reason)` when this item no longer describes the workspace.
    pub fn stale_reason(&self, current: &CurrentState) -> Option<String> {
        if let Some(reason) = &self.born_stale {
            return Some(reason.clone());
        }
        match (self.frame_seq, current.frame_seq) {
            (Some(at), Some(now)) if at < now => {
                return Some(format!(
                    "stale-frame: captured at frame {at}, the browser is at {now}"
                ));
            }
            // The captured page closed with the browser.
            (Some(at), None) => {
                return Some(format!(
                    "stale-frame: captured at frame {at}, the browser is not running"
                ));
            }
            _ => {}
        }
        match &self.code_state {
            None => Some("no code state recorded".to_string()),
            Some(state) => current.stale_reason(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_flag() {
        let a = Evidence::attributed(
            "e1",
            EvidenceKind::Test,
            &["g"],
            "ok",
            Outcome::Pass,
            "t",
            "c",
        );
        let u = Evidence::unattributed("e2", EvidenceKind::File, &["g"], "note", Outcome::Pass);
        assert!(a.is_attributed());
        assert!(!u.is_attributed());
    }

    #[test]
    fn measurement_lists_missing_fields() {
        let full = Measurement {
            runs: Some(5),
            median: Some(12.0),
            range: Some([11.0, 14.0]),
            limiter: Some("cpu".into()),
            build_profile: Some("release".into()),
        };
        assert!(full.missing().is_empty());
        let m = Measurement {
            runs: Some(0),
            limiter: Some(String::new()),
            ..full
        };
        assert_eq!(m.missing(), ["runs", "limiter"]);
        assert_eq!(Measurement::default().missing().len(), 5);
    }

    #[test]
    fn staleness_needs_a_matching_code_state() {
        let ws = CurrentState::from_files([("a.rs".to_string(), b"1".as_slice())]);
        let e = Evidence::attributed(
            "e1",
            EvidenceKind::Test,
            &["g"],
            "s",
            Outcome::Pass,
            "t",
            "c",
        );
        assert_eq!(
            e.stale_reason(&ws).as_deref(),
            Some("no code state recorded")
        );
        let mut e = e.with_code_state(ws.bind(&["a.rs".to_string()]));
        assert_eq!(e.stale_reason(&ws), None);
        e.born_stale = Some("edited after the call".to_string());
        assert!(e.stale_reason(&ws).is_some());
    }

    #[test]
    fn a_capture_from_an_earlier_frame_reads_stale_frame() {
        let mut ws = CurrentState::from_files([("a.rs".to_string(), b"1".as_slice())]);
        let mut e = Evidence::attributed(
            "shot",
            EvidenceKind::Screenshot,
            &["g"],
            "s",
            Outcome::Pass,
            "t",
            "c",
        )
        .with_code_state(ws.bind(&["a.rs".to_string()]));
        e.frame_seq = Some(3);
        ws.frame_seq = Some(3);
        assert_eq!(e.stale_reason(&ws), None, "same frame");
        ws.frame_seq = Some(4);
        let reason = e.stale_reason(&ws).unwrap();
        assert!(reason.starts_with("stale-frame"), "{reason}");
        // The captured page is gone with the browser: never a pass.
        ws.frame_seq = None;
        let reason = e.stale_reason(&ws).unwrap_or_default();
        assert!(reason.starts_with("stale-frame"), "{reason:?}");
    }
}
