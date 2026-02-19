mod extension;
mod known_or_unknown;
mod plan;
mod timestamp;
pub mod websocket_protocol;

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
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OrganizationId(Arc<str>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: Arc<str>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptTermsOfServiceResponse {
    pub user: AuthenticatedUser,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LlmToken(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateLlmTokenResponse {
    pub token: LlmToken,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitAgentThreadFeedbackBody {
    pub organization_id: Option<OrganizationId>,
    pub agent: String,
    pub session_id: String,
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
    pub feedback: String,
}
