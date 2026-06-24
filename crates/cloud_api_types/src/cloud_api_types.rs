mod extension;
pub mod internal_api;
mod known_or_unknown;
mod plan;
mod timestamp;
pub mod websocket_protocol;

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub use crate::extension::*;
pub use crate::known_or_unknown::*;
pub use crate::plan::*;
pub use crate::timestamp::Timestamp;

pub const ZED_SYSTEM_ID_HEADER_NAME: &str = "x-zed-system-id";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAuthenticatedUserResponse {
    pub user: AuthenticatedUser,
    pub feature_flags: Vec<String>,
    #[serde(default)]
    pub organizations: Vec<Organization>,
    #[serde(default)]
    pub default_organization_id: Option<OrganizationId>,
    #[serde(default)]
    pub plans_by_organization: BTreeMap<OrganizationId, KnownOrUnknown<Plan, String>>,
    #[serde(default)]
    pub configuration_by_organization: BTreeMap<OrganizationId, OrganizationConfiguration>,
    pub plan: PlanInfo,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub metrics_id: String,
    pub avatar_url: String,
    pub github_login: String,
    pub name: Option<String>,
    pub is_staff: bool,
    pub accepted_tos_at: Option<Timestamp>,
    pub has_connected_to_collab_once: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct OrganizationId(pub Arc<str>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: Arc<str>,
    pub is_personal: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationConfiguration {
    pub is_zed_model_provider_enabled: bool,
    pub is_agent_thread_feedback_enabled: bool,
    pub is_collaboration_enabled: bool,
    pub edit_prediction: OrganizationEditPredictionConfiguration,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationEditPredictionConfiguration {
    pub is_enabled: bool,
    pub is_feedback_enabled: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptTermsOfServiceResponse {
    pub user: AuthenticatedUser,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LlmToken(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateLlmTokenBody {
    pub organization_id: OrganizationId,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateLlmTokenResponse {
    pub token: LlmToken,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateSystemSettingsBody {
    pub selected_organization_id: Option<OrganizationId>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub selected_organization_id: Option<OrganizationId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitAgentThreadFeedbackBody {
    pub organization_id: Option<OrganizationId>,
    pub agent: String,
    pub session_id: String,
    pub parent_session_id: Option<String>,
    pub rating: String,
    pub thread: serde_json::Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitAgentThreadFeedbackCommentsBody {
    pub organization_id: Option<OrganizationId>,
    pub agent: String,
    pub session_id: String,
    pub comments: String,
    pub thread: serde_json::Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitEditPredictionFeedbackBody {
    pub organization_id: Option<OrganizationId>,
    pub request_id: String,
    pub rating: String,
    pub inputs: serde_json::Value,
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<String>,
    pub feedback: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SettledEditPrediction {
    pub request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_editable_region: Option<String>,
    pub ts_error_count_before_prediction: usize,
    pub ts_error_count_after_prediction: usize,
    pub can_collect_data: bool,
    pub is_in_open_source_repo: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_data: Option<SettledEditPredictionSampleData>,
    #[serde(flatten)]
    pub kept_chars: EditPredictionSettledKeptChars,
    pub example: Option<serde_json::Value>,
    pub model_version: Option<String>,
    #[serde(rename = "e2e_latency")]
    pub e2e_latency_ms: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SettledEditPredictionSampleData {
    pub repository_url: Option<String>,
    pub revision: Option<String>,
    /// Note: this is only the uncommitted diff for files in `edit_history`
    /// This is done to avoid excessive memory usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncommitted_diff: Option<String>,
    pub editable_path: Arc<Path>,
    pub editable_offset_range: Range<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buffer_diagnostics: Vec<zeta_prompt::ActiveBufferDiagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub future_edit_history_events: Vec<Arc<zeta_prompt::Event>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub navigation_history: Vec<EditPredictionRecentFile>,
    pub edit_events_before_quiescence: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_edit_cursor_offset: Option<usize>,
}

pub const MAX_EDIT_PREDICTION_SETTLED_PER_REQUEST: usize = 32;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitEditPredictionSettledBatchBody {
    pub predictions: Vec<SettledEditPrediction>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitEditPredictionSettledResponse {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EditPredictionRecentFile {
    pub path: Arc<Path>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor_position: Option<usize>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EditPredictionSettledKeptChars {
    #[serde(rename = "edit_bytes_candidate_new")]
    pub candidate_new: usize,
    #[serde(rename = "edit_bytes_reference_new")]
    pub reference_new: usize,
    #[serde(rename = "edit_bytes_candidate_deleted")]
    pub candidate_deleted: usize,
    #[serde(rename = "edit_bytes_reference_deleted")]
    pub reference_deleted: usize,
    #[serde(rename = "edit_bytes_kept")]
    pub kept: usize,
    #[serde(rename = "edit_bytes_correctly_deleted")]
    pub correctly_deleted: usize,
    #[serde(rename = "edit_bytes_discarded")]
    pub discarded: usize,
    #[serde(rename = "edit_bytes_context")]
    pub context: usize,
    #[serde(rename = "edit_bytes_kept_rate")]
    pub kept_rate: f64,
    #[serde(rename = "edit_bytes_recall_rate")]
    pub recall_rate: f64,
}
