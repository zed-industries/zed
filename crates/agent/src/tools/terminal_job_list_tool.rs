use agent_client_protocol as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::terminal_job_manager::{TerminalJobManager, TerminalJobStatus};
use crate::{AgentTool, ToolCallEventStream};

/// List background terminal jobs with optional filtering.
///
/// This tool lists all terminal jobs that were started with the `async: true` flag.
/// You can filter by status and limit the number of results.
///
/// Jobs are returned sorted by start time (most recent first).
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalJobListToolInput {
    /// Optional status filter. Can be one or more of: "running", "completed", "failed", "canceled"
    #[serde(default)]
    pub status_filter: Option<Vec<String>>,
    /// Maximum number of jobs to return. Default is 50.
    #[serde(default = "default_limit")]
    pub limit: Option<usize>,
}

fn default_limit() -> Option<usize> {
    Some(50)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobListEntry {
    pub job_id: String,
    pub command: String,
    pub working_dir: String,
    pub status: String,
    pub exit_code: Option<i32>,
    pub duration: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalJobListOutput {
    pub jobs: Vec<JobListEntry>,
    pub total_count: usize,
    pub running_count: usize,
}

impl From<TerminalJobListOutput> for language_model::LanguageModelToolResultContent {
    fn from(output: TerminalJobListOutput) -> Self {
        let mut content = format!(
            "Total Jobs: {} | Running: {}\n\n",
            output.total_count, output.running_count
        );

        if output.jobs.is_empty() {
            content.push_str("No jobs found.");
        } else {
            for job in output.jobs {
                content.push_str(&format!(
                    "Job ID: `{}`\n  Command: {}\n  Working Dir: {}\n  Status: {}\n  Duration: {}",
                    job.job_id, job.command, job.working_dir, job.status, job.duration
                ));
                if let Some(exit_code) = job.exit_code {
                    content.push_str(&format!("\n  Exit Code: {}", exit_code));
                }
                content.push_str("\n\n");
            }
        }

        language_model::LanguageModelToolResultContent::Text(content.into())
    }
}

pub struct TerminalJobListTool;

impl AgentTool for TerminalJobListTool {
    type Input = TerminalJobListToolInput;
    type Output = TerminalJobListOutput;

    fn name() -> &'static str {
        "terminal_job_list"
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
            if let Some(ref filter) = input.status_filter {
                format!("Listing {} jobs", filter.join(", ")).into()
            } else {
                "Listing all jobs".into()
            }
        } else {
            "Listing jobs".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let authorize = event_stream.authorize(self.initial_title(Ok(input.clone()), cx), cx);
        let status_filter = input.status_filter.clone();
        let limit = input.limit;
        let job_manager = TerminalJobManager::global(cx);

        cx.foreground_executor().spawn(async move {
            authorize.await?;

            // Parse status filter
            let status_filter: Option<Vec<TerminalJobStatus>> =
                status_filter.as_ref().map(|filters| {
                    filters
                        .iter()
                        .filter_map(|s| match s.to_lowercase().as_str() {
                            "running" => Some(TerminalJobStatus::Running),
                            "completed" => Some(TerminalJobStatus::Completed),
                            "failed" => Some(TerminalJobStatus::Failed),
                            "canceled" => Some(TerminalJobStatus::Canceled),
                            _ => None,
                        })
                        .collect()
                });

            let jobs = job_manager.list_jobs_filtered(status_filter.as_deref(), limit.or(Some(50)));

            let running_count = job_manager.running_count();
            let total_count = jobs.len();

            let job_entries: Vec<JobListEntry> = jobs
                .into_iter()
                .map(|job| {
                    let status_str = match job.status {
                        TerminalJobStatus::Running => "running",
                        TerminalJobStatus::Completed => "completed",
                        TerminalJobStatus::Failed => "failed",
                        TerminalJobStatus::Canceled => "canceled",
                    };

                    JobListEntry {
                        job_id: job.job_id.clone(),
                        command: job.command.clone(),
                        working_dir: job.working_dir.clone(),
                        status: status_str.to_string(),
                        exit_code: job.exit_code,
                        duration: job.duration_string(),
                    }
                })
                .collect();

            Ok(TerminalJobListOutput {
                jobs: job_entries,
                total_count,
                running_count,
            })
        })
    }
}
