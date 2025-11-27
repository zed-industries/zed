use gpui::{Context, EventEmitter, SharedString, Task};
use language_model::LanguageModel;
use std::sync::Arc;

/// Tracks agent activity status for a session and handles prompt summarization.
/// This entity is used to broadcast agent activity to room participants.
pub struct AgentActivityTracker {
    status: AgentActivityStatus,
    agent_type: Option<SharedString>,
    prompt_summary: Option<SharedString>,
    #[allow(dead_code)]
    pending_summarization: Option<Task<()>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum AgentActivityStatus {
    #[default]
    Idle,
    Active,
}

/// Event emitted when agent activity status changes.
pub struct ActivityStatusChanged {
    pub status: AgentActivityStatus,
    pub agent_type: Option<SharedString>,
    pub prompt_summary: Option<SharedString>,
}

impl EventEmitter<ActivityStatusChanged> for AgentActivityTracker {}

impl AgentActivityTracker {
    pub fn new() -> Self {
        Self {
            status: AgentActivityStatus::Idle,
            agent_type: None,
            prompt_summary: None,
            pending_summarization: None,
        }
    }

    pub fn status(&self) -> &AgentActivityStatus {
        &self.status
    }

    pub fn agent_type(&self) -> Option<&SharedString> {
        self.agent_type.as_ref()
    }

    pub fn prompt_summary(&self) -> Option<&SharedString> {
        self.prompt_summary.as_ref()
    }

    /// Set the agent type (provider name) for this tracker.
    /// This should be called when the model is first set on a thread.
    pub fn set_agent_type(&mut self, agent_type: SharedString) {
        self.agent_type = Some(agent_type);
    }

    /// Set the tracker to active state with the given model.
    pub fn set_active(&mut self, model: &Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
        self.status = AgentActivityStatus::Active;
        self.agent_type = Some(model.upstream_provider_name().0);
        cx.emit(ActivityStatusChanged {
            status: self.status.clone(),
            agent_type: self.agent_type.clone(),
            prompt_summary: self.prompt_summary.clone(),
        });
        cx.notify();
    }

    /// Set the tracker to active state without updating agent type.
    /// Use this when activity changes but agent type is already set.
    pub fn set_active_status(&mut self, cx: &mut Context<Self>) {
        self.status = AgentActivityStatus::Active;
        cx.emit(ActivityStatusChanged {
            status: self.status.clone(),
            agent_type: self.agent_type.clone(),
            prompt_summary: self.prompt_summary.clone(),
        });
        cx.notify();
    }

    /// Set the tracker to idle state.
    pub fn set_idle(&mut self, cx: &mut Context<Self>) {
        self.status = AgentActivityStatus::Idle;
        self.pending_summarization = None;
        cx.emit(ActivityStatusChanged {
            status: self.status.clone(),
            agent_type: self.agent_type.clone(),
            prompt_summary: self.prompt_summary.clone(),
        });
        cx.notify();
    }
}

impl Default for AgentActivityTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, TestAppContext};
    use language_model::fake_provider::FakeLanguageModel;

    #[gpui::test]
    async fn test_activity_tracker_state_transitions(cx: &mut TestAppContext) {
        let tracker = cx.new(|_| AgentActivityTracker::new());

        // Initial state should be idle
        tracker.read_with(cx, |tracker, _| {
            assert_eq!(*tracker.status(), AgentActivityStatus::Idle);
            assert!(tracker.agent_type().is_none());
        });

        // Set active with a model
        let model = Arc::new(FakeLanguageModel::default());
        tracker.update(cx, |tracker, cx| {
            tracker.set_active(&(model.clone() as Arc<dyn LanguageModel>), cx);
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
