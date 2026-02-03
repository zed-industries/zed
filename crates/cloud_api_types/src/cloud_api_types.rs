mod known_or_unknown;
mod plan;
mod timestamp;
pub mod websocket_protocol;

use serde::{Deserialize, Serialize};

pub use crate::known_or_unknown::*;
pub use crate::plan::*;
pub use crate::timestamp::Timestamp;

pub const ZED_SYSTEM_ID_HEADER_NAME: &str = "x-zed-system-id";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAuthenticatedUserResponse {
    pub user: AuthenticatedUser,
    pub feature_flags: Vec<String>,
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
