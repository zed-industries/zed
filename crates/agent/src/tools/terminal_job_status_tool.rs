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
///
/// Supports pagination via `page_number` and `page_size` parameters for large outputs.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalJobStatusToolInput {
    /// The job ID returned when starting an async terminal command
    pub job_id: String,
    /// If true, return only new output since last check. If false, return full output.
    /// Default is true (incremental).
    /// Note: Pagination (page_number/page_size) only works when incremental is false.
    #[serde(default = "default_incremental")]
    pub incremental: bool,
    /// Page number to retrieve (1-based). Only used when incremental is false.
    /// Requires page_size to be set.
    #[serde(default)]
    pub page_number: Option<usize>,
    /// Number of bytes per page. Only used when incremental is false.
    /// If set without page_number, defaults to page 1.
    #[serde(default)]
    pub page_size: Option<usize>,
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
    /// Total length of output in bytes (useful for pagination)
    pub total_output_length: usize,
    /// Current page number (1-based, only set when pagination is used)
    pub page_number: Option<usize>,
    /// Total number of pages available (only set when pagination is used)
    pub total_pages: Option<usize>,
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

            if let (Some(page_num), Some(total)) = (output.page_number, output.total_pages) {
                content.push_str(&format!(
                    "\n\n(Page {} of {}. Total size: {} bytes.)",
                    page_num, total, output.total_output_length
                ));
            }
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
        let page_number = input.page_number;
        let page_size = input.page_size;
        let job_manager = TerminalJobManager::global(cx);
        let cx_async = cx.to_async();

        cx.foreground_executor().spawn(async move {
            authorize.await?;

            let job = job_manager
                .get_job(&job_id)
                .ok_or_else(|| anyhow::anyhow!("Job not found: {}", job_id))?;

            let is_running = matches!(job.status, TerminalJobStatus::Running);

            // Get current output from live terminal if running, otherwise use stored output
            let full_output = if is_running {
                // Fetch live output and update stored output
                match job_manager.get_current_terminal_output(&job_id, &cx_async) {
                    Ok(output) => output,
                    Err(_) => {
                        // Fallback to stored output if terminal read fails
                        job_manager.get_full_output(&job_id).unwrap_or_default()
                    }
                }
            } else {
                // For finished jobs, use stored output
                job_manager.get_full_output(&job_id).unwrap_or_default()
            };

            let total_length = full_output.len();

            // Apply incremental or pagination logic
            let (output, current_page, total_pages_count) = if incremental {
                // Incremental mode: return only new output since last read
                let new_output = job_manager
                    .get_incremental_output(&job_id)
                    .map(|(output, _)| output)
                    .unwrap_or_default();
                (new_output, None, None)
            } else if let Some(page_size_val) = page_size {
                // Pagination mode
                let page_num = page_number.unwrap_or(1).max(1); // Default to page 1, minimum page 1
                let total_pages = if total_length == 0 {
                    1
                } else {
                    (total_length + page_size_val - 1) / page_size_val // Ceiling division
                };

                let start = (page_num - 1) * page_size_val;
                let start = start.min(total_length);
                let end = (start + page_size_val).min(total_length);

                let paginated = full_output[start..end].to_string();
                (paginated, Some(page_num), Some(total_pages))
            } else {
                // No pagination: return all output
                (full_output, None, None)
            };

            Ok(TerminalJobStatusOutput {
                job_id: job.job_id.clone(),
                command: job.command.clone(),
                working_dir: job.working_dir.clone(),
                status: job.status.clone(),
                exit_code: job.exit_code,
                duration: job.duration_string(),
                output,
                is_running,
                total_output_length: total_length,
                page_number: current_page,
                total_pages: total_pages_count,
            })
        })
    }
}
