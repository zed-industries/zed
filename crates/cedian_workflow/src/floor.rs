//! Gate floor (§46, ADR-0010): the gates cedian policy requires per task
//! kind × risk. OMP may ADD gates; it can never remove or weaken a floor
//! gate. An empty floor is the fast lane (ADR-0026): nothing is required
//! unless OMP or the user starts a workflow.
//!
//! Pure data. The policy source is the user's `cedian.toml`
//! (`[[workflow.floor]]`, ADR-0018), parsed as [`FloorRuleSpec`]s so every
//! floor gate goes through [`GateSpec::build`].

use crate::gate::{Gate, GateSpec};
use crate::profile::{Risk, TaskKind, TaskProfile};
use serde::Deserialize;

/// Gates required for one task kind at or above a risk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloorRule {
    pub kind: TaskKind,
    pub min_risk: Risk,
    pub gates: Vec<Gate>,
    /// OMP model role names for the review panel (ADR-0039 decision 5).
    pub review_roles: Vec<String>,
    /// Cap on reviewers after same-model roles are collapsed. `None` means
    /// no floor cap beyond the role list.
    pub max_reviewers: Option<u32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GateFloor {
    pub rules: Vec<FloorRule>,
}

/// Roles a review panel should run for a task, before model resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPanel {
    pub roles: Vec<String>,
    /// After roles that resolve to the same model count once (ADR-0039).
    pub max_reviewers: u32,
}

/// One `[[workflow.floor]]` table as written in `cedian.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FloorRuleSpec {
    pub kind: TaskKind,
    pub min_risk: Risk,
    pub gates: Vec<GateSpec>,
    /// ADR-0039 decision 5: role names, never model ids.
    #[serde(default)]
    pub review_roles: Vec<String>,
    #[serde(default)]
    pub max_reviewers: Option<u32>,
}

impl GateFloor {
    /// Build the floor; the error names the rule (1-based) and the gate.
    pub fn from_specs(specs: Vec<FloorRuleSpec>) -> Result<Self, String> {
        let mut rules = Vec::with_capacity(specs.len());
        for (n, spec) in specs.into_iter().enumerate() {
            let gates = spec
                .gates
                .into_iter()
                .map(|g| {
                    let id = g.id.clone();
                    g.build()
                        .map_err(|e| format!("floor rule {}, gate {id:?}: {e:?}", n + 1))
                })
                .collect::<Result<_, _>>()?;
            let mut review_roles = Vec::new();
            for role in spec.review_roles {
                let role = role.trim().to_string();
                if role.is_empty() {
                    return Err(format!(
                        "floor rule {}: review_roles cannot hold an empty name",
                        n + 1
                    ));
                }
                if role.contains('/') {
                    return Err(format!(
                        "floor rule {}: review_roles names a model id {role:?}; use an OMP role name (ADR-0039)",
                        n + 1
                    ));
                }
                if !review_roles.iter().any(|r| r == &role) {
                    review_roles.push(role);
                }
            }
            rules.push(FloorRule {
                kind: spec.kind,
                min_risk: spec.min_risk,
                gates,
                review_roles,
                max_reviewers: spec.max_reviewers,
            });
        }
        Ok(Self { rules })
    }

    /// Floor gates for this task, all forced `required`.
    pub fn gates_for(&self, task: &TaskProfile) -> Vec<Gate> {
        self.rules
            .iter()
            .filter(|r| r.kind == task.kind && task.risk >= r.min_risk)
            .flat_map(|r| r.gates.iter().cloned())
            .map(|mut g| {
                g.required = true;
                g
            })
            .collect()
    }

    /// Review panel for this task (ADR-0039 decision 5). Matching rules
    /// union their roles; `max_reviewers` takes the strictest (minimum)
    /// among rules that set it. With no floor roles, `fallback_role` is the
    /// single-role panel (`[review] role`).
    pub fn review_panel_for(&self, task: &TaskProfile, fallback_role: &str) -> ReviewPanel {
        let mut roles = Vec::new();
        let mut max_reviewers: Option<u32> = None;
        for rule in self
            .rules
            .iter()
            .filter(|r| r.kind == task.kind && task.risk >= r.min_risk)
        {
            for role in &rule.review_roles {
                if !roles.iter().any(|r| r == role) {
                    roles.push(role.clone());
                }
            }
            if let Some(cap) = rule.max_reviewers {
                max_reviewers = Some(match max_reviewers {
                    Some(existing) => existing.min(cap),
                    None => cap,
                });
            }
        }
        if roles.is_empty() {
            let fallback = fallback_role.trim();
            if !fallback.is_empty() {
                roles.push(fallback.to_string());
            }
        }
        let max_reviewers = max_reviewers.unwrap_or(roles.len() as u32).max(1);
        ReviewPanel {
            roles,
            max_reviewers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::{GateKind, GatePredicate};

    fn floor() -> GateFloor {
        let gate = Gate::register(
            "build",
            GateKind::Build,
            false, // forced required by the floor
            GatePredicate {
                kinds: vec![],
                min_items: 1,
                require_ok: true,
                fresh: true,
                feature: None,
            },
            false,
        )
        .unwrap();
        GateFloor {
            rules: vec![FloorRule {
                kind: TaskKind::Feature,
                min_risk: Risk::Medium,
                gates: vec![gate],
                review_roles: Vec::new(),
                max_reviewers: None,
            }],
        }
    }

    #[test]
    fn floor_applies_by_kind_and_min_risk() {
        let mut task = TaskProfile::new("t", TaskKind::Feature);
        task.risk = Risk::Low;
        assert!(floor().gates_for(&task).is_empty());
        task.risk = Risk::High;
        let gates = floor().gates_for(&task);
        assert_eq!(gates.len(), 1);
        assert!(gates[0].required);
        task.kind = TaskKind::BugFix;
        assert!(floor().gates_for(&task).is_empty());
        assert!(GateFloor::default().gates_for(&task).is_empty());
    }

    fn specs(v: serde_json::Value) -> Result<GateFloor, String> {
        let specs: Vec<FloorRuleSpec> = serde_json::from_value(v).map_err(|e| e.to_string())?;
        GateFloor::from_specs(specs)
    }

    #[test]
    fn specs_build_required_gates_with_kind_rules() {
        let floor = specs(serde_json::json!([{
            "kind": "bug_fix", "min_risk": "low",
            "gates": [{"id": "repro", "gate_kind": "reproduction"},
                      {"id": "tests", "gate_kind": "test", "evidence_kinds": ["test"], "min_items": 0}]
        }]))
        .unwrap();
        let gates = &floor.rules[0].gates;
        assert!(gates.iter().all(|g| g.required));
        assert!(!gates[0].predicate.fresh && !gates[0].predicate.require_ok);
        assert!(gates[1].predicate.fresh && gates[1].predicate.require_ok);
        assert_eq!(gates[1].predicate.min_items, 1);
    }

    #[test]
    fn specs_cannot_carry_engine_fields() {
        for key in ["fresh", "require_ok", "required", "feature"] {
            let err = specs(serde_json::json!([{
                "kind": "feature", "min_risk": "low",
                "gates": [{"id": "g", "gate_kind": "test", key: false}]
            }]))
            .unwrap_err();
            assert!(err.contains("unknown field"), "{key}: {err}");
        }
    }

    #[test]
    fn review_panel_unions_roles_and_takes_the_strictest_cap() {
        let floor = specs(serde_json::json!([
            {
                "kind": "feature", "min_risk": "low",
                "gates": [{"id": "t", "gate_kind": "test"}],
                "review_roles": ["review", "review-alt"],
                "max_reviewers": 2
            },
            {
                "kind": "feature", "min_risk": "high",
                "gates": [{"id": "t2", "gate_kind": "test"}],
                "review_roles": ["security"],
                "max_reviewers": 1
            }
        ]))
        .unwrap();
        let mut task = TaskProfile::new("t", TaskKind::Feature);
        task.risk = Risk::High;
        let panel = floor.review_panel_for(&task, "review");
        assert_eq!(panel.roles, vec!["review", "review-alt", "security"]);
        assert_eq!(panel.max_reviewers, 1, "strictest matching cap wins");
        task.risk = Risk::Low;
        let panel = floor.review_panel_for(&task, "review");
        assert_eq!(panel.roles, vec!["review", "review-alt"]);
        assert_eq!(panel.max_reviewers, 2);
    }

    #[test]
    fn review_panel_falls_back_to_the_settings_role() {
        let floor = GateFloor::default();
        let task = TaskProfile::new("t", TaskKind::Feature);
        let panel = floor.review_panel_for(&task, "review");
        assert_eq!(panel.roles, vec!["review"]);
        assert_eq!(panel.max_reviewers, 1);
    }

    #[test]
    fn review_roles_refuse_a_model_id() {
        let err = specs(serde_json::json!([{
            "kind": "feature", "min_risk": "low",
            "gates": [{"id": "t", "gate_kind": "test"}],
            "review_roles": ["opencode-go/muse"]
        }]))
        .unwrap_err();
        assert!(err.contains("model id"), "{err}");
    }
}
