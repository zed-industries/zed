use super::super::*;

// implements https://docs.github.com/en/rest/copilot/copilot-user-management
// as of API Version 2022-11-28
//
// We have chosen to not map out the enums as the copilot API is still fresh,
// this means GitHub may add additional enums in the future and they would
// require more maintenance than just providing a String.
//
// For a list of available enums, refer to the "response schema" in the link above.
//
// missing:
// - billing/seats misses the assigning_team field
//
// OAuth app tokens and personal access tokens (classic) need either the manage_billing:copilot, read:org, or read:enterprise scopes to use this endpoint.
// Some of these permissions, as of writing, are only available to GitHub Enterprise customers and further limited to Enterprise Administrators.

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotBilling {
    pub seat_breakdown: CopilotSeatBreakdown,
    pub seat_management_setting: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ide_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_code_suggestions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSeatBreakdown {
    pub total: u32,
    pub added_this_cycle: u32,
    pub pending_invitation: u32,
    pub pending_cancellation: u32,
    pub active_this_cycle: u32,
    pub inactive_this_cycle: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotBillingSeats {
    pub total_seats: u32,
    pub seats: Vec<CopilotSeat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CopilotSeat {
    pub created_at: DateTime<Utc>,
    pub pending_cancellation_date: Option<String>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_activity_editor: Option<String>,
    pub plan_type: Option<String>,
    pub assignee: Option<SimpleUser>, // null if user has deleted their account within this billing cycle.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SeatsCreated {
    /// The total number of seats created for the specified user(s).
    pub seats_created: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SeatsCancelled {
    /// The total number of seats set to "pending cancellation" for members of the specified team(s).
    pub seats_cancelled: u32,
}
