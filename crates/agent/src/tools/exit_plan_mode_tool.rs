use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use chrono::Local;
use gpui::{App, AsyncApp, Entity, SharedString, Task, WeakEntity};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

/// Presents your implementation plan to the user for approval before making any changes.
///
/// Call this tool once — and only once — you have finished researching the codebase and
/// have a concrete, actionable plan for how to implement the user's request. Until then,
/// keep exploring with the read-only tools available to you.
///
/// This pauses the conversation so the user can review the plan. If they approve it, you
/// will be given full tool access (including editing files and running commands) to carry
/// it out, and the plan will be saved to a Markdown file under `.zed/plans/` for reference.
/// If they ask you to keep planning instead, do not make any edits — continue researching
/// and refining the plan based on their feedback, then call this tool again once it's ready.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExitPlanModeToolInput {
    /// A short (3-8 word) title for the plan, e.g. "Add dark mode toggle". Used only to
    /// name the file the plan is saved to; avoid punctuation other than spaces.
    pub title: String,
    /// The plan to present to the user, formatted as Markdown. Should be a concise,
    /// actionable description of the changes you intend to make.
    pub plan: String,
}

const APPROVE_OPTION_ID: &str = "approve";

pub struct ExitPlanModeTool {
    project: Entity<Project>,
    thread: WeakEntity<Thread>,
}

impl ExitPlanModeTool {
    pub fn new(project: Entity<Project>, thread: WeakEntity<Thread>) -> Self {
        Self { project, thread }
    }
}

impl AgentTool for ExitPlanModeTool {
    type Input = ExitPlanModeToolInput;
    type Output = String;

    const NAME: &'static str = "exit_plan_mode";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::SwitchMode
    }

    fn initial_title(
        &self,
        _input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        "Exit plan mode".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        let thread = self.thread.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|e| e.to_string())?;

            let approve = acp::PermissionOption::new(
                acp::PermissionOptionId::new(APPROVE_OPTION_ID),
                "Yes, start implementing",
                acp::PermissionOptionKind::AllowOnce,
            );
            let keep_planning = acp::PermissionOption::new(
                acp::PermissionOptionId::new("keep_planning"),
                "No, keep planning",
                acp::PermissionOptionKind::RejectOnce,
            );

            let option_id = cx
                .update(|cx| {
                    event_stream.prompt_for_decision(
                        Some("Ready to code?".to_string()),
                        Some(input.plan.clone()),
                        vec![approve, keep_planning],
                        cx,
                    )
                })
                .await
                .map_err(|e| e.to_string())?;

            if option_id.0.as_ref() == APPROVE_OPTION_ID {
                thread
                    .update(cx, |thread, cx| thread.exit_plan_mode(cx))
                    .map_err(|e| e.to_string())?;

                telemetry::event!("Plan Mode Exited", approved = true);

                let saved_note = match save_plan_to_file(&project, &input.title, &input.plan, cx).await
                {
                    Ok(Some(path)) => {
                        format!(" The plan has also been saved to `{}` for reference.", path.display())
                    }
                    Ok(None) => String::new(),
                    Err(error) => {
                        log::warn!("Failed to save approved plan to .zed/plans: {error:#}");
                        String::new()
                    }
                };

                Ok(format!(
                    "The user approved the plan. You now have full tool access — proceed with \
                      implementing it.{saved_note}"
                ))
            } else {
                telemetry::event!("Plan Mode Exited", approved = false);

                Ok("The user wants you to keep planning instead of implementing yet. Do not make \
                    any edits or run any commands. Continue researching, refine the plan based on \
                    any feedback in the conversation, and call exit_plan_mode again once it's ready."
                    .to_string())
            }
        })
    }
}

/// Saves an approved plan to `.zed/plans/<date>-<slug>.md` under the project's primary
/// worktree, so it survives after the thread is closed and can be picked up later (the
/// editor recognizes files under `.zed/plans/` and offers an "Implement Plan" action).
///
/// Returns `Ok(None)` when there's no worktree to save into (e.g. an empty workspace)
/// rather than treating that as an error, since plan approval should still succeed.
async fn save_plan_to_file(
    project: &Entity<Project>,
    title: &str,
    plan: &str,
    cx: &mut AsyncApp,
) -> Result<Option<PathBuf>> {
    let Some((fs, root)) = cx.update(|cx| {
        let project = project.read(cx);
        project.worktrees(cx).next().map(|worktree| {
            (
                project.fs().clone(),
                worktree.read(cx).abs_path().to_path_buf(),
            )
        })
    }) else {
        return Ok(None);
    };

    let slug = agent_skills::slugify_skill_name(title).unwrap_or_else(|| "plan".to_string());
    let plans_dir = root.join(paths::local_settings_folder_name()).join("plans");
    let stem = format!("{}-{}", Local::now().format("%Y-%m-%d-%H%M"), slug);

    let mut path = plans_dir.join(format!("{stem}.md"));
    let mut suffix = 2;
    while fs.is_file(&path).await {
        path = plans_dir.join(format!("{stem}-{suffix}.md"));
        suffix += 1;
    }

    let content = format!(
        "# {title}\n\n_Approved plan · saved {timestamp}_\n\n{plan}\n",
        timestamp = Local::now().format("%Y-%m-%d %H:%M"),
    );
    fs.write(&path, content.as_bytes()).await?;

    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToolCallEventStream;
    use agent_settings::{AgentProfileId, builtin_profiles};
    use futures::StreamExt as _;
    use gpui::{AppContext as _, TestAppContext};
    use project::FakeFs;
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
    async fn test_exit_plan_mode_approve_restores_previous_profile_and_saves_plan(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let (project, thread) = build_thread(cx).await;
        // Start from a non-default profile so we can confirm approval restores it
        // rather than assuming `write`.
        thread.update(cx, |thread, cx| {
            thread.set_profile(AgentProfileId(builtin_profiles::ASK.into()), cx);
            thread.set_profile(AgentProfileId(builtin_profiles::PLAN.into()), cx);
        });

        let tool = Arc::new(ExitPlanModeTool::new(project.clone(), thread.downgrade()));
        let (event_stream, mut event_rx) = ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(ExitPlanModeToolInput {
                    title: "Add dark mode toggle".into(),
                    plan: "1. Do the thing\n2. Do another thing".into(),
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        assert_eq!(
            auth.tool_call.fields.title.as_deref(),
            Some("Ready to code?")
        );
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new(APPROVE_OPTION_ID),
                acp::PermissionOptionKind::AllowOnce,
            ))
            .unwrap();

        let result = task.await.unwrap();
        assert!(result.contains("full tool access"));
        assert!(result.contains(".zed/plans"));

        // Restores the profile that was active before Plan mode, not `write`.
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.profile().clone()),
            AgentProfileId(builtin_profiles::ASK.into())
        );

        let fs = project.read_with(cx, |project, _| project.fs().clone());
        let plans_dir = std::path::Path::new("/root/.zed/plans");
        let entries = fs.read_dir(plans_dir).await.unwrap();
        let saved_files: Vec<_> = entries.collect().await;
        assert_eq!(saved_files.len(), 1, "expected exactly one saved plan file");
        let saved_content = fs.load(saved_files[0].as_ref().unwrap()).await.unwrap();
        assert!(saved_content.contains("Add dark mode toggle"));
        assert!(saved_content.contains("Do another thing"));
    }

    #[gpui::test]
    async fn test_exit_plan_mode_keep_planning_stays_in_plan_profile(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, thread) = build_thread(cx).await;
        thread.update(cx, |thread, cx| {
            thread.set_profile(AgentProfileId(builtin_profiles::PLAN.into()), cx);
        });

        let tool = Arc::new(ExitPlanModeTool::new(project, thread.downgrade()));
        let (event_stream, mut event_rx) = ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                ToolInput::resolved(ExitPlanModeToolInput {
                    title: "Add dark mode toggle".into(),
                    plan: "1. Do the thing".into(),
                }),
                event_stream,
                cx,
            )
        });

        let auth = event_rx.expect_authorization().await;
        auth.response
            .send(acp_thread::SelectedPermissionOutcome::new(
                acp::PermissionOptionId::new("keep_planning"),
                acp::PermissionOptionKind::RejectOnce,
            ))
            .unwrap();

        let result = task.await.unwrap();
        assert!(result.contains("keep planning"));
        assert_eq!(
            thread.read_with(cx, |thread, _| thread.profile().clone()),
            AgentProfileId(builtin_profiles::PLAN.into())
        );
    }
}
