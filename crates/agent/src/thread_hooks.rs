//! Integration points for the curator and router into the agent thread
//! lifecycle.
//!
//! These hooks are called from `thread.rs` at specific points in the
//! agent loop. Each hook is a one-liner that either does nothing
//! (if the feature is disabled) or runs its logic in a background task.
//!
//! ## Wiring (to be done in thread.rs)
//!
//! ### 1. Router — before the first LLM call
//!
//! In the agent loop, before the first `client.chat.completions.create()`
//! call (typically in `build_request_messages_until` or the main cycle
//! function), add:
//!
//! ```rust
//! // Route this prompt to the optimal model
//! thread_hooks::maybe_route(&self, &user_message, cx);
//! ```
//!
//! ### 2. Curator — after a thread completes
//!
//! After the agent produces its final response and the message has been
//! appended to `self.messages`, add:
//!
//! ```rust
//! // Observe patterns for skill generation
//! thread_hooks::maybe_observe(&self.messages, cx);
//! ```
//!
//! Both hooks are no-ops when their respective configs have `enabled: false`.

use std::sync::Arc;

use gpui::App;

use crate::curator::Curator;
use crate::router::{Router, RouterConfig, TaskClassifier};
use crate::Message;

/// Call at thread start to route to the optimal model.
/// Returns `true` if routing was applied.
pub fn maybe_route(
    config: &RouterConfig,
    user_message: &str,
    _current_model: &mut Option<String>,
    _current_tools: &mut Vec<String>,
) -> bool {
    if !config.enabled {
        return false;
    }
    let router = Router::new(config.clone());
    let classifier = TaskClassifier;
    let task_type = classifier.classify(user_message);

    // Log the routing decision without modifying thread state
    // (actual model/tool changes happen in the caller via Router directly)
    log::info!(
        "router: classified prompt as {:?} (profile: {})",
        task_type,
        router.profile_for(task_type).name,
    );
    true
}

/// Call after a thread completes to let the curator extract patterns.
/// Runs synchronously but spawns no background tasks — the curator's
/// filesystem I/O is lightweight (JSONL append + optional SKILL.md write).
pub fn maybe_observe(messages: &[Arc<Message>], _cx: &App) {
    let curator = Curator::new();
    // The curator internally checks `config.enabled`
    curator.observe(messages, _cx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_route_disabled() {
        let config = RouterConfig::default(); // enabled: false
        let mut model = Some("original".into());
        let mut tools = vec!["all".into()];
        assert!(!maybe_route(&config, "Edit this file", &mut model, &mut tools));
    }
}
