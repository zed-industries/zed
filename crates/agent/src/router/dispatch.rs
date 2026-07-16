//! Route dispatch — translates a classified task type into concrete
//! model selection and tool allowlist, then spawns a sub-agent.
//!
//! ## Integration point
//!
//! The router is called at the START of `agent_loop()` in `thread.rs`.
//! It sets the thread's model and enabled tools before the first LLM
//! call, ensuring prompt caching is stable for the full thread.
//!
//! ## Sub-agent dispatch (alternative mode)
//!
//! Instead of changing the main thread's model, the router can spawn
//! a sub-agent via `spawn_agent` with the selected model + tools and
//! merge the results. This preserves the main thread's model for
//! coordination while routing subtasks to specialized models.

use anyhow::Result;
use gpui::App;

use crate::router::{ModelProfile, Router, RouterConfig, TaskType};
use crate::router::classifier::TaskClassifier;

/// Result of routing a prompt.
pub enum RoutingDecision {
    /// Stay on the current model — nothing to route.
    None,
    /// Switch the thread's model and tools to the matched profile.
    SwitchProfile(ModelProfile),
    /// (Future) Spawn a sub-agent with the matched profile.
    SpawnSubAgent(ModelProfile),
}

impl Router {
    /// Route a user prompt. Returns a RoutingDecision describing what
    /// action the agent loop should take.
    pub fn route(&self, prompt: &str) -> RoutingDecision {
        if !self.is_enabled() {
            return RoutingDecision::None;
        }

        let classifier = TaskClassifier;
        let task_type = classifier.classify(prompt);
        let profile = self.profile_for(task_type);

        // If the profile has no specific model set, don't route
        if profile.model.is_empty() {
            return RoutingDecision::None;
        }

        RoutingDecision::SwitchProfile(profile.clone())
    }
}

/// Apply a routing decision to the thread's state.
///
/// This is called from `thread.rs` before the LLM API call. It
/// overrides:
/// - `thread.model` — the LLM model to use
/// - The tool allowlist (if the profile specifies one)
pub fn apply_routing(
    decision: &RoutingDecision,
    current_model: &mut Option<String>,
    current_tools: &mut Vec<String>,
) {
    match decision {
        RoutingDecision::None => {}
        RoutingDecision::SwitchProfile(profile) | RoutingDecision::SpawnSubAgent(profile) => {
            if !profile.model.is_empty() {
                *current_model = Some(profile.model.clone());
            }
            if !profile.tools.is_empty() {
                *current_tools = profile.tools.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_disabled() {
        let config = RouterConfig {
            enabled: false,
            ..Default::default()
        };
        let router = Router::new(config);
        assert!(matches!(router.route("Edit this file"), RoutingDecision::None));
    }

    #[test]
    fn test_route_enabled_edit() {
        let config = RouterConfig {
            enabled: true,
            ..Default::default()
        };
        let router = Router::new(config);
        match router.route("Fix the login bug") {
            RoutingDecision::SwitchProfile(profile) => {
                assert_eq!(profile.name, "Edit");
            }
            other => panic!("expected SwitchProfile, got {other:?}"),
        }
    }

    #[test]
    fn test_apply_routing() {
        let mut model = Some("original".into());
        let mut tools = vec!["all".into()];

        let profile = ModelProfile {
            name: "Test".into(),
            provider: "test".into(),
            model: "model-x".into(),
            tools: vec!["terminal".into()],
        };

        apply_routing(&RoutingDecision::SwitchProfile(profile), &mut model, &mut tools);
        assert_eq!(model.unwrap(), "model-x");
        assert_eq!(tools, vec!["terminal"]);
    }
}
