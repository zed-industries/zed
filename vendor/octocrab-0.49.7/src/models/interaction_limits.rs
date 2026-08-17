use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub struct InteractionLimit {
    pub limit: InteractionLimitType,
    pub origin: String,
    pub expires_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InteractionLimitType {
    ExistingUsers,
    ContributorsOnly,
    CollaboratorsOnly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InteractionLimitExpiry {
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
    SixMonths,
}
