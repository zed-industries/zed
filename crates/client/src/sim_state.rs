//! Staff-only state injection for Zed Sim (see `tooling/zed-sim/INJECTION_PLAN.md`).
//!
//! This module is compiled only under the `staff-sim` feature. It produces a
//! synthetic `GetAuthenticatedUserResponse` for a chosen state, which the caller
//! feeds through `UserStore::update_authenticated_user` — the same path the real
//! server response uses — so the editor renders as that state with no backend.
//!
//! Everything here is cosmetic: it sets what the client believes and displays,
//! not real server enforcement.

use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{Duration, Utc};
use cloud_api_client::{
    AuthenticatedUser, GetAuthenticatedUserResponse, KnownOrUnknown, Plan, PlanInfo,
    SubscriptionPeriod, Timestamp,
};
use cloud_llm_client::{CurrentUsage, UsageData, UsageLimit};
use gpui::{AsyncApp, Entity};

use crate::{Client, Status, UserStore};

/// The injectable signed-in states. Business member/admin are deferred until
/// the personal-plan states are proven end to end (they need organizations
/// populated in the response).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimAuthState {
    /// Signed in on the Free plan, never trialed.
    FreeSignedIn,
    /// Signed in on Pro.
    Pro,
    /// Mid-trial.
    TrialActive,
    /// Trial has ended: Free plan with a past trial start, which drives the
    /// real end-of-trial upsell.
    TrialExpired,
}

/// Applies the state requested via `ZED_SIM_STATE` (if any) to the running
/// client and user store, with no network access:
///
/// - marks the client `Authenticated` **without connecting to collab**, and
/// - feeds a synthesized authenticated-user response through the normal path.
///
/// Returns `Ok(true)` when a state was applied. A missing/unknown
/// `ZED_SIM_STATE` is a no-op (`Ok(false)`).
pub async fn apply_from_env(
    client: &Arc<Client>,
    user_store: &Entity<UserStore>,
    cx: &mut AsyncApp,
) -> anyhow::Result<bool> {
    let Some(state) = SimAuthState::from_env() else {
        return Ok(false);
    };
    let response = synthesize_response(state);

    // Signed-in for the UI, but deliberately NOT connected: no websocket to the
    // collab server, no outbound session. Purely local state.
    client.set_status(Status::Authenticated, cx);
    user_store.update(cx, |user_store, cx| {
        user_store.apply_sim_state(response, cx);
    });

    log::info!("zed-sim: applied simulated state {state:?}");
    Ok(true)
}

impl SimAuthState {
    /// Reads the requested state from the `ZED_SIM_STATE` environment variable.
    /// Returns `None` when unset or unrecognized.
    pub fn from_env() -> Option<Self> {
        Self::from_id(std::env::var("ZED_SIM_STATE").ok()?.trim())
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "free" => Some(Self::FreeSignedIn),
            "pro" => Some(Self::Pro),
            "trial" => Some(Self::TrialActive),
            "trial_expired" => Some(Self::TrialExpired),
            _ => None,
        }
    }
}

/// Builds a synthetic authenticated-user response for the given state.
pub fn synthesize_response(state: SimAuthState) -> GetAuthenticatedUserResponse {
    GetAuthenticatedUserResponse {
        user: sim_user(),
        feature_flags: Vec::new(),
        organizations: Vec::new(),
        default_organization_id: None,
        plans_by_organization: BTreeMap::new(),
        configuration_by_organization: BTreeMap::new(),
        plan: plan_info(state),
    }
}

fn sim_user() -> AuthenticatedUser {
    AuthenticatedUser {
        id: 9_999_999,
        metrics_id: "zed-sim".to_string(),
        avatar_url: String::new(),
        github_login: "zed-sim-user".to_string(),
        name: Some("Zed Sim User".to_string()),
        is_staff: false,
        accepted_tos_at: Some(Timestamp(Utc::now())),
        has_connected_to_collab_once: false,
    }
}

fn plan_info(state: SimAuthState) -> PlanInfo {
    let comfortable_usage = CurrentUsage {
        edit_predictions: UsageData {
            used: 0,
            limit: UsageLimit::Unlimited,
        },
    };

    let (plan, trial_started_at, subscription_period) = match state {
        SimAuthState::FreeSignedIn => (Plan::ZedFree, None, None),
        SimAuthState::Pro => (Plan::ZedPro, None, None),
        SimAuthState::TrialActive => {
            let now = Utc::now();
            (
                Plan::ZedProTrial,
                Some(Timestamp(now - Duration::days(3))),
                Some(SubscriptionPeriod {
                    started_at: Timestamp(now - Duration::days(3)),
                    ended_at: Timestamp(now + Duration::days(11)),
                }),
            )
        }
        // Trial expired: Free plan with a past trial start. This is the exact
        // shape the end-of-trial upsell keys off of.
        SimAuthState::TrialExpired => (
            Plan::ZedFree,
            Some(Timestamp(Utc::now() - Duration::days(30))),
            None,
        ),
    };

    PlanInfo {
        plan: KnownOrUnknown::Known(plan),
        subscription_period,
        usage: comfortable_usage,
        trial_started_at,
        is_account_too_young: false,
        has_overdue_invoices: false,
    }
}
