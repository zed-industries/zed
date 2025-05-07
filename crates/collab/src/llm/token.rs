use crate::Cents;
use crate::db::billing_subscription::SubscriptionKind;
use crate::db::{billing_subscription, user};
use crate::llm::{
    AGENT_EXTENDED_TRIAL_FEATURE_FLAG, DEFAULT_MAX_MONTHLY_SPEND, FREE_TIER_MONTHLY_SPENDING_LIMIT,
};
use crate::{Config, db::billing_preference};
use anyhow::{Result, anyhow};
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;
use zed_llm_client::Plan;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmTokenClaims {
    pub iat: u64,
    pub exp: u64,
    pub jti: String,
    pub user_id: u64,
    pub system_id: Option<String>,
    pub metrics_id: Uuid,
    pub github_user_login: String,
    pub account_created_at: NaiveDateTime,
    pub is_staff: bool,
    pub has_llm_closed_beta_feature_flag: bool,
    pub bypass_account_age_check: bool,
    pub has_llm_subscription: bool,
    #[serde(default)]
    pub use_llm_request_queue: bool,
    pub max_monthly_spend_in_cents: u32,
    pub custom_llm_monthly_allowance_in_cents: Option<u32>,
    #[serde(default)]
    pub use_new_billing: bool,
    pub plan: Plan,
    #[serde(default)]
    pub has_extended_trial: bool,
    #[serde(default)]
    pub subscription_period: Option<(NaiveDateTime, NaiveDateTime)>,
    #[serde(default)]
    pub enable_model_request_overages: bool,
    #[serde(default)]
    pub model_request_overages_spend_limit_in_cents: u32,
    #[serde(default)]
    pub can_use_web_search_tool: bool,
}

const LLM_TOKEN_LIFETIME: Duration = Duration::from_secs(60 * 60);

impl LlmTokenClaims {
    pub fn create(
        user: &user::Model,
        is_staff: bool,
        billing_preferences: Option<billing_preference::Model>,
        feature_flags: &Vec<String>,
        has_legacy_llm_subscription: bool,
        subscription: Option<billing_subscription::Model>,
        system_id: Option<String>,
        config: &Config,
    ) -> Result<String> {
        let secret = config
            .llm_api_secret
            .as_ref()
            .ok_or_else(|| anyhow!("no LLM API secret"))?;

        let now = Utc::now();
        let claims = Self {
            iat: now.timestamp() as u64,
            exp: (now + LLM_TOKEN_LIFETIME).timestamp() as u64,
            jti: uuid::Uuid::new_v4().to_string(),
            user_id: user.id.to_proto(),
            system_id,
            metrics_id: user.metrics_id,
            github_user_login: user.github_login.clone(),
            account_created_at: user.account_created_at(),
            is_staff,
            has_llm_closed_beta_feature_flag: feature_flags
                .iter()
                .any(|flag| flag == "llm-closed-beta"),
            bypass_account_age_check: feature_flags
                .iter()
                .any(|flag| flag == "bypass-account-age-check"),
            can_use_web_search_tool: feature_flags.iter().any(|flag| flag == "assistant2"),
            has_llm_subscription: has_legacy_llm_subscription,
            max_monthly_spend_in_cents: billing_preferences
                .as_ref()
                .map_or(DEFAULT_MAX_MONTHLY_SPEND.0, |preferences| {
                    preferences.max_monthly_llm_usage_spending_in_cents as u32
                }),
            custom_llm_monthly_allowance_in_cents: user
                .custom_llm_monthly_allowance_in_cents
                .map(|allowance| allowance as u32),
            use_new_billing: feature_flags.iter().any(|flag| flag == "new-billing"),
            use_llm_request_queue: feature_flags.iter().any(|flag| flag == "llm-request-queue"),
            plan: if is_staff {
                Plan::ZedPro
            } else {
                subscription
                    .as_ref()
                    .and_then(|subscription| subscription.kind)
                    .map_or(Plan::Free, |kind| match kind {
                        SubscriptionKind::ZedFree => Plan::Free,
                        SubscriptionKind::ZedPro => Plan::ZedPro,
                        SubscriptionKind::ZedProTrial => Plan::ZedProTrial,
                    })
            },
            has_extended_trial: feature_flags
                .iter()
                .any(|flag| flag == AGENT_EXTENDED_TRIAL_FEATURE_FLAG),
            subscription_period: billing_subscription::Model::current_period(
                subscription,
                is_staff,
            )
            .map(|(start, end)| (start.naive_utc(), end.naive_utc())),
            enable_model_request_overages: billing_preferences
                .as_ref()
                .map_or(false, |preferences| {
                    preferences.model_request_overages_enabled
                }),
            model_request_overages_spend_limit_in_cents: billing_preferences
                .as_ref()
                .map_or(0, |preferences| {
                    preferences.model_request_overages_spend_limit_in_cents as u32
                }),
        };

        Ok(jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?)
    }

    pub fn validate(token: &str, config: &Config) -> Result<LlmTokenClaims, ValidateLlmTokenError> {
        let secret = config
            .llm_api_secret
            .as_ref()
            .ok_or_else(|| anyhow!("no LLM API secret"))?;

        match jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => Ok(token.claims),
            Err(e) => {
                if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                    Err(ValidateLlmTokenError::Expired)
                } else {
                    Err(ValidateLlmTokenError::JwtError(e))
                }
            }
        }
    }

    pub fn free_tier_monthly_spending_limit(&self) -> Cents {
        self.custom_llm_monthly_allowance_in_cents
            .map(Cents)
            .unwrap_or(FREE_TIER_MONTHLY_SPENDING_LIMIT)
    }
}

#[derive(Error, Debug)]
pub enum ValidateLlmTokenError {
    #[error("access token is expired")]
    Expired,
    #[error("access token validation error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}
