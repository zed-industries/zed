use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::terminal_job_manager::{TerminalJobManager, TerminalJobStatus};
use crate::{AgentTool, ToolCallEventStream};

/// Cancel a running background terminal job.
///
/// This tool attempts to cancel a terminal command that was started with the `async: true` flag.
/// The command will be killed and marked as canceled.
///
/// Only running jobs can be canceled. Already completed, failed, or canceled jobs cannot be canceled again.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalJobCancelToolInput {
    /// The job ID of the running job to cancel
    pub job_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalJobCancelOutput {
    pub job_id: String,
    pub success: bool,
    pub message: String,
    pub previous_status: String,
}

impl From<TerminalJobCancelOutput> for language_model::LanguageModelToolResultContent {
    fn from(output: TerminalJobCancelOutput) -> Self {
        let content = if output.success {
            format!(
                "Job `{}` has been canceled successfully.\n\nPrevious Status: {}\n\n{}",
                output.job_id, output.previous_status, output.message
            )
        } else {
            format!(
                "Failed to cancel job `{}`.\n\nReason: {}",
                output.job_id, output.message
            )
        };

        language_model::LanguageModelToolResultContent::Text(content.into())
    }
}

pub struct TerminalJobCancelTool;

impl AgentTool for TerminalJobCancelTool {
    type Input = TerminalJobCancelToolInput;
    type Output = TerminalJobCancelOutput;

    fn name() -> &'static str {
        "terminal_job_cancel"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Canceling job: {}", input.job_id).into()
        } else {
            "Canceling job".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let authorize = event_stream.authorize(self.initial_title(Ok(input.clone()), cx), cx);
        let job_id = input.job_id.clone();
        let job_manager = TerminalJobManager::global(cx);

        let cx_async = cx.to_async();
        cx.foreground_executor().spawn(async move {
            authorize.await?;

            // Get job info before canceling
            let job = job_manager
                .get_job(&job_id)
                .ok_or_else(|| anyhow::anyhow!("Job not found: {}", job_id))?;

            let previous_status = match job.status {
                TerminalJobStatus::Running => "running",
                TerminalJobStatus::Completed => "completed",
                TerminalJobStatus::Failed => "failed",
                TerminalJobStatus::Canceled => "canceled",
            }
            .to_string();

            // Actually kill the terminal process
            let result = job_manager.cancel_job(&job_id, &cx_async);

            match result {
                Ok(()) => Ok(TerminalJobCancelOutput {
                    job_id: job_id.clone(),
                    success: true,
                    message: "Job canceled and terminal process killed successfully.".to_string(),
                    previous_status,
                }),
                Err(e) => Ok(TerminalJobCancelOutput {
                    job_id: job_id.clone(),
                    success: false,
                    message: format!("Failed to cancel job: {:?}", e),
                    previous_status,
                }),
            }
        })
    }
}
