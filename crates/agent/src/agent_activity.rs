use gpui::{Context, EventEmitter, SharedString};
use language_model::LanguageModel;
use std::sync::Arc;

/// Tracks the activity status of an AI agent for collaborative features.
/// This entity emits `ActivityStatusChanged` events when the agent's state changes.
pub struct AgentActivityTracker {
    status: AgentActivityStatus,
    agent_type: Option<SharedString>,
    prompt_summary: Option<SharedString>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AgentActivityStatus {
    #[default]
    Idle,
    Active,
}

/// Event emitted when the agent's activity status changes.
/// Subscribers can listen for these events to react to agent state changes.
#[derive(Clone, Debug)]
pub struct ActivityStatusChanged {
    pub status: AgentActivityStatus,
    pub agent_type: Option<SharedString>,
    pub prompt_summary: Option<SharedString>,
}

impl EventEmitter<ActivityStatusChanged> for AgentActivityTracker {}

impl AgentActivityTracker {
    /// Creates a new activity tracker in the idle state.
    pub fn new() -> Self {
        Self {
            status: AgentActivityStatus::Idle,
            agent_type: None,
            prompt_summary: None,
        }
    }

    /// Sets the tracker to active state and records the agent type from the model.
    /// Emits an `ActivityStatusChanged` event and notifies GPUI.
    pub fn set_active(&mut self, model: &Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
        self.status = AgentActivityStatus::Active;
        self.agent_type = Some(SharedString::from(model.upstream_provider_name().0.clone()));

        cx.emit(ActivityStatusChanged {
            status: self.status.clone(),
            agent_type: self.agent_type.clone(),
            prompt_summary: self.prompt_summary.clone(),
        });
        cx.notify();
    }

    /// Sets the tracker to idle state.
    /// Emits an `ActivityStatusChanged` event and notifies GPUI.
    pub fn set_idle(&mut self, cx: &mut Context<Self>) {
        self.status = AgentActivityStatus::Idle;
        self.agent_type = None;

        cx.emit(ActivityStatusChanged {
            status: self.status.clone(),
            agent_type: self.agent_type.clone(),
            prompt_summary: self.prompt_summary.clone(),
        });
        cx.notify();
    }

    /// Sets the agent type independently (useful for initialization).
    pub fn set_agent_type(&mut self, agent_type: SharedString) {
        self.agent_type = Some(agent_type);
    }

    /// Returns the current activity status.
    pub fn status(&self) -> &AgentActivityStatus {
        &self.status
    }

    /// Returns the current agent type (e.g., "Anthropic", "OpenAI").
    pub fn agent_type(&self) -> Option<&SharedString> {
        self.agent_type.as_ref()
    }

    /// Returns the current prompt summary (for future use).
    pub fn prompt_summary(&self) -> Option<&SharedString> {
        self.prompt_summary.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    async fn test_activity_tracker_state_transitions(cx: &mut TestAppContext) {
        let tracker = cx.new(|_| AgentActivityTracker::new());

        // Initial state should be idle
        tracker.read_with(cx, |tracker, _| {
            assert_eq!(*tracker.status(), AgentActivityStatus::Idle);
            assert!(tracker.agent_type().is_none());
        });

        // Set active with a model
        let model = Arc::new(language_model::FakeLanguageModel::default());
        tracker.update(cx, |tracker, cx| {
            tracker.set_active(&model, cx);
        });

        tracker.read_with(cx, |tracker, _| {
            assert_eq!(*tracker.status(), AgentActivityStatus::Active);
            assert!(tracker.agent_type().is_some());
        });

        // Set back to idle
        tracker.update(cx, |tracker, cx| {
            tracker.set_idle(cx);
        });

        tracker.read_with(cx, |tracker, _| {
            assert_eq!(*tracker.status(), AgentActivityStatus::Idle);
        });
    }
}
