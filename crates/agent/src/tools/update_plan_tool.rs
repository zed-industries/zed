use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task, WeakEntity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Updates the live plan checklist shown in the activity bar as you work through
/// a plan.
///
/// Call this whenever the set of steps changes or a step's status moves from
/// pending to in-progress to completed. The checklist is purely a progress aid —
/// it makes no changes on its own. Pass the full, current state of the plan each
/// time; earlier steps are replaced by the new list (preserving completion order).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdatePlanToolInput {
    /// An optional short title for the plan.
    #[serde(default)]
    pub title: Option<String>,
    /// The full, current list of plan steps, in order.
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PlanStep {
    /// A short description of the step.
    pub title: String,
    /// The current status of the step.
    pub status: PlanStepStatus,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlanStepStatus {
    Pending,
    InProgress,
    Completed,
}

const NAME: &str = "update_plan";

pub struct UpdatePlanTool {
    thread: WeakEntity<Thread>,
}

impl UpdatePlanTool {
    pub fn new(thread: WeakEntity<Thread>) -> Self {
        Self { thread }
    }
}

impl AgentTool for UpdatePlanTool {
    type Input = UpdatePlanToolInput;
    type Output = String;

    const NAME: &'static str = NAME;

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Update plan".into()
    }

    fn allow_in_restricted_mode() -> bool {
        true
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let thread = self.thread.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;

            let entries: Vec<acp::PlanEntry> = input
                .steps
                .iter()
                .map(|step| {
                    let status = match step.status {
                        PlanStepStatus::Pending => acp::PlanEntryStatus::Pending,
                        PlanStepStatus::InProgress => acp::PlanEntryStatus::InProgress,
                        PlanStepStatus::Completed => acp::PlanEntryStatus::Completed,
                    };
                    acp::PlanEntry::new(step.title.as_str(), acp::PlanEntryPriority::Medium, status)
                })
                .collect();

            let plan_title = input.title;
            let step_count = entries.len();
            let completed = entries
                .iter()
                .filter(|e| matches!(e.status, acp::PlanEntryStatus::Completed))
                .count();

            let plan = acp::Plan::new(entries);
            thread
                .update(cx, |thread, cx| thread.update_plan(plan, cx))
                .map_err(|e| e.to_string())?;

            Ok(format!(
                "Updated the plan{} with {step_count} step(s), {completed} completed.",
                plan_title
                    .map(|title| format!(" \"{title}\""))
                    .unwrap_or_default()
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Thread, ToolCallEventStream};
    use gpui::{AppContext as _, Entity, TestAppContext};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });
    }

    async fn build_thread(cx: &mut TestAppContext) -> (Entity<Project>, Entity<Thread>) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree("/root", json!({"main.rs": "fn main() {}"}))
            .await;
        let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

        let context_server_registry = cx.new(|cx| {
            crate::ContextServerRegistry::new(project.read(cx).context_server_store(), cx)
        });
        let templates = crate::Templates::new();
        let thread = cx.new(|cx| {
            Thread::new(
                project.clone(),
                cx.new(|_cx| prompt_store::ProjectContext::default()),
                context_server_registry,
                templates,
                None,
                cx,
            )
        });
        (project, thread)
    }

    #[gpui::test]
    async fn test_update_plan_replaces_checklist(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, thread) = build_thread(cx).await;
        let tool = Arc::new(UpdatePlanTool::new(thread.downgrade()));
        let (event_stream, _event_rx) = ToolCallEventStream::test();

        let task = cx.update(|cx| {
            Arc::clone(&tool).run(
                ToolInput::resolved(UpdatePlanToolInput {
                    title: Some("Refactor parser".into()),
                    steps: vec![
                        PlanStep {
                            title: "Find call sites".into(),
                            status: PlanStepStatus::Completed,
                        },
                        PlanStep {
                            title: "Add helper".into(),
                            status: PlanStepStatus::InProgress,
                        },
                        PlanStep {
                            title: "Wire it up".into(),
                            status: PlanStepStatus::Pending,
                        },
                    ],
                }),
                event_stream,
                cx,
            )
        });

        let result = task.await.unwrap();
        assert!(result.contains("3 step(s)"));
        assert!(result.contains("1 completed"));

        let entries = thread.read_with(cx, |thread, _| thread.plan().entries.clone());
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].status, acp::PlanEntryStatus::Completed);
        assert_eq!(entries[1].status, acp::PlanEntryStatus::InProgress);
        assert_eq!(entries[2].status, acp::PlanEntryStatus::Pending);

        // A second call replaces rather than appends.
        let (event_stream, _event_rx) = ToolCallEventStream::test();
        let task = cx.update(|cx| {
            Arc::clone(&tool).run(
                ToolInput::resolved(UpdatePlanToolInput {
                    title: None,
                    steps: vec![PlanStep {
                        title: "Single remaining step".into(),
                        status: PlanStepStatus::Pending,
                    }],
                }),
                event_stream,
                cx,
            )
        });
        task.await.unwrap();

        let entries = thread.read_with(cx, |thread, _| thread.plan().entries.clone());
        assert_eq!(entries.len(), 1);
        drop(project);
    }
}
