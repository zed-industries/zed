use crate::models::workflows::HeadCommit;

use super::*;
use crate::models::pulls::PullRequest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRunOutput {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub annotations_count: u64,
    pub annotations_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckRun {
    pub id: CheckRunId,
    pub node_id: String,
    pub details_url: Option<String>,
    pub head_sha: String,
    pub url: String,
    pub html_url: Option<String>,
    pub conclusion: Option<String>,
    pub output: CheckRunOutput,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListCheckRuns {
    pub total_count: u64,
    pub check_runs: Vec<CheckRun>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckSuite {
    pub id: CheckSuiteId,
    pub node_id: String,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub url: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    app: Option<App>,
    pub repository: Repository,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub head_commit: HeadCommit,
    latest_check_runs_count: i64,
    check_runs_url: String,
    rerequestable: Option<bool>,
    runs_rerequestable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListCheckSuites {
    pub total_count: u32,
    pub check_suites: Vec<CheckSuite>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckSuitePreferences {
    pub preferences: CheckSuiteUpdatePreferences,
    pub repository: Repository,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CheckSuiteUpdatePreferences {
    pub auto_trigger_checks: Vec<AutoTriggerCheck>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoTriggerCheck {
    /// Enables or disables automatic creation of CheckSuite events upon pushes to the repository.
    pub app_id: AppId,
    pub setting: bool,
}
