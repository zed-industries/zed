use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::terminal_job_manager::{TerminalJobManager, TerminalJobStatus};
use crate::{AgentTool, ToolCallEventStream};

/// Get status and output of a background terminal job.
///
/// This tool retrieves the current status, exit code, and output of a terminal command
/// that was started with the `async: true` flag.
///
/// By default, returns only new output since the last check (incremental mode).
/// Set `incremental: false` to get the full output from the beginning.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalJobStatusToolInput {
    /// The job ID returned when starting an async terminal command
    pub job_id: String,
    /// If true, return only new output since last check. If false, return full output.
    /// Default is true (incremental).
    #[serde(default = "default_incremental")]
    pub incremental: bool,
}

fn default_incremental() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalJobStatusOutput {
    pub job_id: String,
    pub command: String,
    pub working_dir: String,
    pub status: TerminalJobStatus,
    pub exit_code: Option<i32>,
    pub duration: String,
    pub output: String,
    pub is_running: bool,
}

impl From<TerminalJobStatusOutput> for language_model::LanguageModelToolResultContent {
    fn from(output: TerminalJobStatusOutput) -> Self {
        let status_str = match output.status {
            TerminalJobStatus::Running => "running",
            TerminalJobStatus::Completed => "completed",
            TerminalJobStatus::Failed => "failed",
            TerminalJobStatus::Canceled => "canceled",
        };

        let mut content = format!(
            "Job ID: {}\nCommand: {}\nWorking Directory: {}\nStatus: {}\nDuration: {}",
            output.job_id, output.command, output.working_dir, status_str, output.duration
        );

        if let Some(exit_code) = output.exit_code {
            content.push_str(&format!("\nExit Code: {}", exit_code));
        }

        if !output.output.is_empty() {
            content.push_str(&format!("\n\nOutput:\n```\n{}\n```", output.output));
        } else {
            content.push_str("\n\nNo output yet.");
        }

        if output.is_running {
            content.push_str("\n\n(Job is still running. Check again later for more output.)");
        }

        language_model::LanguageModelToolResultContent::Text(content.into())
    }
}

pub struct TerminalJobStatusTool;

impl AgentTool for TerminalJobStatusTool {
    type Input = TerminalJobStatusToolInput;
    type Output = TerminalJobStatusOutput;

    fn name() -> &'static str {
        "terminal_job_status"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Checking job status: {}", input.job_id).into()
        } else {
            "Checking job status".into()
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
        let incremental = input.incremental;
        let job_manager = TerminalJobManager::global(cx);

        cx.foreground_executor().spawn(async move {
            authorize.await?;

            let job = job_manager
                .get_job(&job_id)
                .ok_or_else(|| anyhow::anyhow!("Job not found: {}", job_id))?;

            let output = if incremental {
                job_manager
                    .get_incremental_output(&job_id)
                    .map(|(output, _)| output)
                    .unwrap_or_default()
            } else {
                job_manager
                    .get_full_output(&input.job_id)
                    .unwrap_or_default()
            };

            let is_running = matches!(job.status, TerminalJobStatus::Running);

            Ok(TerminalJobStatusOutput {
                job_id: job.job_id.clone(),
                command: job.command.clone(),
                working_dir: job.working_dir.clone(),
                status: job.status.clone(),
                exit_code: job.exit_code,
                duration: job.duration_string(),
                output,
                is_running,
            })
        })
    }
}
