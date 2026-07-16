//! Agent tools for managing cron jobs.
//! Provides `cron_list`, `cron_add`, and `cron_remove` so the agent
//! can schedule and manage background tasks autonomously.

use std::sync::Arc;

use crate::scheduler::{CronJob, global_store};
use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use gpui::{App, SharedString, Task};
use language_model::LanguageModelToolResultContent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// cron_add
// ---------------------------------------------------------------------------

/// Schedule a recurring agent task. The agent will run the given prompt
/// automatically on the specified schedule. Use this for periodic checks,
/// nightly builds, daily summaries, or any recurring automation.
///
/// Schedule formats:
/// - `30m` — every 30 minutes
/// - `2h` — every 2 hours
/// - `@daily` — once a day at midnight
/// - `@weekly` — once a week on Sunday
/// - `0 9 * * 1-5` — weekdays at 9am (standard 5-field cron)
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CronAddToolInput {
    /// Schedule expression (e.g. "30m", "2h", "@daily", "0 9 * * 1-5").
    pub schedule: String,
    /// The prompt to run when the job fires.
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronAddToolOutput {
    Success { id: String, next_run_at: u64 },
    Error { error: String },
}

impl From<CronAddToolOutput> for LanguageModelToolResultContent {
    fn from(output: CronAddToolOutput) -> Self {
        match output {
            CronAddToolOutput::Success { id, next_run_at } => {
                format!("Cron job '{id}' created (next run: timestamp {next_run_at})").into()
            }
            CronAddToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct CronAddTool;

impl AgentTool for CronAddTool {
    type Input = CronAddToolInput;
    type Output = CronAddToolOutput;

    const NAME: &'static str = "cron_add";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Write
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>, _cx: &mut App) -> SharedString {
        "Scheduling cron job…".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| CronAddToolOutput::Error {
                error: format!("Failed to receive input: {e}"),
            })?;

            let id = slugify(&input.prompt);
            let store = global_store();
            let job = CronJob::new(id.clone(), input.schedule, input.prompt);
            let next = job.next_run_at;
            store.add(job);

            Ok(CronAddToolOutput::Success {
                id,
                next_run_at: next,
            })
        })
    }
}

// ---------------------------------------------------------------------------
// cron_list
// ---------------------------------------------------------------------------

/// List all scheduled cron jobs with their schedule, status, and next run time.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CronListToolInput;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronListToolOutput {
    Success { jobs: Vec<serde_json::Value>, total: usize },
    Error { error: String },
}

impl From<CronListToolOutput> for LanguageModelToolResultContent {
    fn from(output: CronListToolOutput) -> Self {
        match output {
            CronListToolOutput::Success { jobs, total } => {
                if jobs.is_empty() {
                    "No cron jobs scheduled.".into()
                } else {
                    let mut lines = format!("**{total} cron job(s):**\n\n");
                    for (i, job) in jobs.iter().enumerate() {
                        let id = job.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                        let sched = job.get("schedule").and_then(|v| v.as_str()).unwrap_or("");
                        let paused = job.get("paused").and_then(|v| v.as_bool()).unwrap_or(false);
                        let runs = job.get("run_count").and_then(|v| v.as_u64()).unwrap_or(0);
                        let status = if paused { "⏸ paused" } else { "▶ active" };
                        lines.push_str(&format!("{i}. **{id}** — `{sched}` ({status}, {runs} runs)\n"));
                    }
                    lines.into()
                }
            }
            CronListToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct CronListTool;

impl AgentTool for CronListTool {
    type Input = CronListToolInput;
    type Output = CronListToolOutput;

    const NAME: &'static str = "cron_list";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>, _cx: &mut App) -> SharedString {
        "Listing cron jobs…".into()
    }

    fn run(
        self: Arc<Self>,
        _input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let store = global_store();
            let jobs: Vec<serde_json::Value> = store
                .all()
                .into_iter()
                .filter_map(|j| serde_json::to_value(j).ok())
                .collect();
            let total = jobs.len();
            Ok(CronListToolOutput::Success { jobs, total })
        })
    }
}

// ---------------------------------------------------------------------------
// cron_remove
// ---------------------------------------------------------------------------

/// Remove a previously scheduled cron job by its ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CronRemoveToolInput {
    /// The ID of the cron job to remove (use `cron_list` to find IDs).
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronRemoveToolOutput {
    Success { removed: bool },
    Error { error: String },
}

impl From<CronRemoveToolOutput> for LanguageModelToolResultContent {
    fn from(output: CronRemoveToolOutput) -> Self {
        match output {
            CronRemoveToolOutput::Success { removed } => {
                if removed {
                    "Cron job removed.".into()
                } else {
                    "No cron job found with that ID.".into()
                }
            }
            CronRemoveToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct CronRemoveTool;

impl AgentTool for CronRemoveTool {
    type Input = CronRemoveToolInput;
    type Output = CronRemoveToolOutput;

    const NAME: &'static str = "cron_remove";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Write
    }

    fn initial_title(&self, _input: Result<Self::Input, serde_json::Value>, _cx: &mut App) -> SharedString {
        "Removing cron job…".into()
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        _cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| CronRemoveToolOutput::Error {
                error: format!("Failed to receive input: {e}"),
            })?;
            let store = global_store();
            let removed = store.remove(&input.id);
            Ok(CronRemoveToolOutput::Success { removed })
        })
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

// ---------------------------------------------------------------------------
// cron_pause
// ---------------------------------------------------------------------------

/// Pause a scheduled cron job. The job will not fire again until resumed.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CronPauseToolInput {
    /// The ID of the cron job to pause.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CronPauseToolOutput {
    Success { id: String, paused: bool },
    Error { error: String },
}

impl From<CronPauseToolOutput> for LanguageModelToolResultContent {
    fn from(output: CronPauseToolOutput) -> Self {
        match output {
            CronPauseToolOutput::Success { id, paused } => {
                if paused {
                    format!("Cron job '{id}' paused.").into()
                } else {
                    format!("Cron job '{id}' resumed.").into()
                }
            }
            CronPauseToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct CronPauseTool;

impl AgentTool for CronPauseTool {
    type Input = CronPauseToolInput;
    type Output = CronPauseToolOutput;

    const NAME: &'static str = "cron_pause";

    fn kind() -> acp::ToolKind { acp::ToolKind::Write }
    fn initial_title(&self, _: Result<Self::Input, _>, _: &mut App) -> SharedString {
        "Toggling cron job…".into()
    }
    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _: ToolCallEventStream,
        _: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        Task::ready(async move {
            let input = input.recv().await.map_err(|e| CronPauseToolOutput::Error {
                error: format!("Failed to receive input: {e}"),
            })?;
            let store = global_store();
            let mut paused = false;
            store.update(&input.id, |job| {
                job.paused = !job.paused;
                paused = job.paused;
            });
            if store.all().iter().any(|j| j.id == input.id) {
                Ok(CronPauseToolOutput::Success { id: input.id, paused })
            } else {
                Ok(CronPauseToolOutput::Error { error: format!("No cron job '{}' found", input.id) })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Check for updates"), "check-for-updates");
        assert_eq!(slugify("Run nightly build!!"), "run-nightly-build");
    }
}
