use std::sync::{Arc, OnceLock};

use anyhow::anyhow;
use axum::{
    Extension, Json, Router,
    extract::{self, Query},
    routing::get,
};
use chrono::{NaiveDateTime, SecondsFormat};
use serde::{Deserialize, Serialize};

use crate::api::AuthenticatedUserParams;
use crate::db::ContributorSelector;
use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new()
        .route("/contributors", get(get_contributors).post(add_contributor))
        .route("/contributor", get(check_is_contributor))
}

async fn get_contributors(Extension(app): Extension<Arc<AppState>>) -> Result<Json<Vec<String>>> {
    Ok(Json(app.db.get_contributors().await?))
}

#[derive(Debug, Deserialize)]
struct CheckIsContributorParams {
    github_user_id: Option<i32>,
    github_login: Option<String>,
}

impl CheckIsContributorParams {
    fn into_contributor_selector(self) -> Result<ContributorSelector> {
        if let Some(github_user_id) = self.github_user_id {
            return Ok(ContributorSelector::GitHubUserId { github_user_id });
        }

        if let Some(github_login) = self.github_login {
            return Ok(ContributorSelector::GitHubLogin { github_login });
        }

        Err(anyhow!(
            "must be one of `github_user_id` or `github_login`."
        ))?
    }
}

#[derive(Debug, Serialize)]
struct CheckIsContributorResponse {
    signed_at: Option<String>,
}

async fn check_is_contributor(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<CheckIsContributorParams>,
) -> Result<Json<CheckIsContributorResponse>> {
    let params = params.into_contributor_selector()?;

    if RenovateBot::is_renovate_bot(&params) {
        return Ok(Json(CheckIsContributorResponse {
            signed_at: Some(
                RenovateBot::created_at()
                    .and_utc()
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            ),
        }));
    }

    Ok(Json(CheckIsContributorResponse {
        signed_at: app
            .db
            .get_contributor_sign_timestamp(&params)
            .await?
            .map(|ts| ts.and_utc().to_rfc3339_opts(SecondsFormat::Millis, true)),
    }))
}

/// The Renovate bot GitHub user (`renovate[bot]`).
///
/// https://api.github.com/users/renovate[bot]
struct RenovateBot;

impl RenovateBot {
    const LOGIN: &'static str = "renovate[bot]";
    const USER_ID: i32 = 29139614;

    /// Returns the `created_at` timestamp for the Renovate bot user.
    fn created_at() -> &'static NaiveDateTime {
        static CREATED_AT: OnceLock<NaiveDateTime> = OnceLock::new();
        CREATED_AT.get_or_init(|| {
            chrono::DateTime::parse_from_rfc3339("2017-06-02T07:04:12Z")
                .expect("failed to parse 'created_at' for 'renovate[bot]'")
                .naive_utc()
        })
    }

    /// Returns whether the given contributor selector corresponds to the Renovate bot user.
    fn is_renovate_bot(contributor: &ContributorSelector) -> bool {
        match contributor {
            ContributorSelector::GitHubLogin { github_login } => github_login == Self::LOGIN,
            ContributorSelector::GitHubUserId { github_user_id } => {
                github_user_id == &Self::USER_ID
            }
        }
    }
}

async fn add_contributor(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(params): extract::Json<AuthenticatedUserParams>,
) -> Result<()> {
    let initial_channel_id = app.config.auto_join_channel_id;
    app.db
        .add_contributor(
            &params.github_login,
            params.github_user_id,
            params.github_email.as_deref(),
            params.github_name.as_deref(),
            params.github_user_created_at,
            initial_channel_id,
        )
        .await
}
