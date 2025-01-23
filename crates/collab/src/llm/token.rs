use crate::db::user;
use crate::llm::{DEFAULT_MAX_MONTHLY_SPEND, FREE_TIER_MONTHLY_SPENDING_LIMIT};
use crate::Cents;
use crate::{db::billing_preference, Config};
use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

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
    pub is_staff: bool,
    pub has_llm_closed_beta_feature_flag: bool,
    #[serde(default)]
    pub has_predict_edits_feature_flag: bool,
    pub has_llm_subscription: bool,
    pub max_monthly_spend_in_cents: u32,
    pub custom_llm_monthly_allowance_in_cents: Option<u32>,
    pub plan: rpc::proto::Plan,
}

const LLM_TOKEN_LIFETIME: Duration = Duration::from_secs(60 * 60);

impl LlmTokenClaims {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        user: &user::Model,
        is_staff: bool,
        billing_preferences: Option<billing_preference::Model>,
        has_llm_closed_beta_feature_flag: bool,
        has_predict_edits_feature_flag: bool,
        has_llm_subscription: bool,
        plan: rpc::proto::Plan,
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
            is_staff,
            has_llm_closed_beta_feature_flag,
            has_predict_edits_feature_flag,
            has_llm_subscription,
            max_monthly_spend_in_cents: billing_preferences
                .map_or(DEFAULT_MAX_MONTHLY_SPEND.0, |preferences| {
                    preferences.max_monthly_llm_usage_spending_in_cents as u32
                }),
            custom_llm_monthly_allowance_in_cents: user
                .custom_llm_monthly_allowance_in_cents
                .map(|allowance| allowance as u32),
            plan,
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
