//! ADR-0033 decision 4: each brief `verify` item is a required gate for that
//! worker. Evaluation reuses [`cedian_workflow::Gate`] so pass / fail /
//! inconclusive and freshness match every other gate.

use cedian_worker::Brief;
use cedian_workflow::{CurrentState, Evidence, Gate, GateKind, GatePredicate, GateStatus};

/// Reasons the worker is not yet complete. Empty means every verify item has
/// fresh attributed `pass` evidence.
pub fn missing_verify(brief: &Brief, evidence: &[Evidence], current: &CurrentState) -> Vec<String> {
    let mut missing = Vec::new();
    for item in &brief.verify {
        let gate = match Gate::register(
            item.clone(),
            GateKind::Test,
            true,
            GatePredicate {
                kinds: Vec::new(),
                min_items: 1,
                require_ok: true,
                fresh: true,
                feature: None,
            },
            false,
        ) {
            Ok(gate) => gate,
            Err(e) => {
                missing.push(format!("worker verify {item:?}: {e:?}"));
                continue;
            }
        };
        let result = gate.evaluate(evidence, current);
        if result.status != GateStatus::Passed {
            missing.push(format!("worker verify {item:?}: {}", result.reason));
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use cedian_workflow::{EvidenceKind, Outcome};

    fn brief(verify: &[&str]) -> Brief {
        Brief {
            goal: "g".into(),
            scope: cedian_worker::Scope {
                write: vec!["notes.txt".into()],
                deny: vec![],
            },
            acceptance: vec!["done".into()],
            verify: verify.iter().map(|s| (*s).to_string()).collect(),
            timebox_min: 5.0,
            context: vec![],
            forbidden: vec![],
        }
    }

    fn current() -> CurrentState {
        CurrentState::from_files([("notes.txt".into(), b"x" as &[u8])])
    }

    fn pass(gate: &str, current: &CurrentState) -> Evidence {
        Evidence::attributed(
            "e1",
            EvidenceKind::Test,
            &[gate],
            "ok",
            Outcome::Pass,
            "task",
            "call-1",
        )
        .with_code_state(current.bind(&[]))
    }

    #[test]
    fn missing_verify_names_each_unmet_item() {
        let brief = brief(&["cat notes.txt", "cargo test"]);
        let missing = missing_verify(&brief, &[], &current());
        assert_eq!(missing.len(), 2, "{missing:?}");
        assert!(missing[0].contains("cat notes.txt"), "{missing:?}");
        assert!(missing[1].contains("cargo test"), "{missing:?}");
    }

    #[test]
    fn pass_evidence_clears_the_item() {
        let current = current();
        let brief = brief(&["cat notes.txt"]);
        let missing = missing_verify(&brief, &[pass("cat notes.txt", &current)], &current);
        assert!(missing.is_empty(), "{missing:?}");
    }

    #[test]
    fn inconclusive_does_not_complete() {
        let current = current();
        let brief = brief(&["cat notes.txt"]);
        let mut e = pass("cat notes.txt", &current);
        e.outcome = Outcome::Inconclusive;
        let missing = missing_verify(&brief, &[e], &current);
        assert_eq!(missing.len(), 1, "{missing:?}");
    }
}
