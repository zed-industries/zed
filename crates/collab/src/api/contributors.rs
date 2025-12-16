use std::sync::{Arc, OnceLock};

use axum::{
    Extension, Json, Router,
    extract::{self, Query},
    routing::get,
};
use chrono::{NaiveDateTime, SecondsFormat};
use serde::{Deserialize, Serialize};

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

        Err(anyhow::anyhow!(
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

    if CopilotSweAgentBot::is_copilot_bot(&params) {
        return Ok(Json(CheckIsContributorResponse {
            signed_at: Some(
                CopilotSweAgentBot::created_at()
                    .and_utc()
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            ),
        }));
    }

    if Dependabot::is_dependabot(&params) {
        return Ok(Json(CheckIsContributorResponse {
            signed_at: Some(
                Dependabot::created_at()
                    .and_utc()
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            ),
        }));
    }

    if RenovateBot::is_renovate_bot(&params) {
        return Ok(Json(CheckIsContributorResponse {
            signed_at: Some(
                RenovateBot::created_at()
                    .and_utc()
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            ),
        }));
    }

    if ZedZippyBot::is_zed_zippy_bot(&params) {
        return Ok(Json(CheckIsContributorResponse {
            signed_at: Some(
                ZedZippyBot::created_at()
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

/// The Copilot bot GitHub user (`copilot-swe-agent[bot]`).
///
/// https://api.github.com/users/copilot-swe-agent[bot]
struct CopilotSweAgentBot;

impl CopilotSweAgentBot {
    const LOGIN: &'static str = "copilot-swe-agent[bot]";
    const USER_ID: i32 = 198982749;

    /// Returns the `created_at` timestamp for the Dependabot bot user.
    fn created_at() -> &'static NaiveDateTime {
        static CREATED_AT: OnceLock<NaiveDateTime> = OnceLock::new();
        CREATED_AT.get_or_init(|| {
            chrono::DateTime::parse_from_rfc3339("2025-02-12T20:26:08Z")
                .expect("failed to parse 'created_at' for 'copilot-swe-agent[bot]'")
                .naive_utc()
        })
    }

    /// Returns whether the given contributor selector corresponds to the Copilot bot user.
    fn is_copilot_bot(contributor: &ContributorSelector) -> bool {
        match contributor {
            ContributorSelector::GitHubLogin { github_login } => github_login == Self::LOGIN,
            ContributorSelector::GitHubUserId { github_user_id } => {
                github_user_id == &Self::USER_ID
            }
        }
    }
}

/// The Dependabot bot GitHub user (`dependabot[bot]`).
///
/// https://api.github.com/users/dependabot[bot]
struct Dependabot;

impl Dependabot {
    const LOGIN: &'static str = "dependabot[bot]";
    const USER_ID: i32 = 49699333;

    /// Returns the `created_at` timestamp for the Dependabot bot user.
    fn created_at() -> &'static NaiveDateTime {
        static CREATED_AT: OnceLock<NaiveDateTime> = OnceLock::new();
        CREATED_AT.get_or_init(|| {
            chrono::DateTime::parse_from_rfc3339("2019-04-16T22:34:25Z")
                .expect("failed to parse 'created_at' for 'dependabot[bot]'")
                .naive_utc()
        })
    }

    /// Returns whether the given contributor selector corresponds to the Dependabot bot user.
    fn is_dependabot(contributor: &ContributorSelector) -> bool {
        match contributor {
            ContributorSelector::GitHubLogin { github_login } => github_login == Self::LOGIN,
            ContributorSelector::GitHubUserId { github_user_id } => {
                github_user_id == &Self::USER_ID
            }
        }
    }
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

/// The Zed Zippy bot GitHub user (`zed-zippy[bot]`).
///
/// https://api.github.com/users/zed-zippy[bot]
struct ZedZippyBot;

impl ZedZippyBot {
    const LOGIN: &'static str = "zed-zippy[bot]";
    const USER_ID: i32 = 234243425;

    /// Returns the `created_at` timestamp for the Zed Zippy bot user.
    fn created_at() -> &'static NaiveDateTime {
        static CREATED_AT: OnceLock<NaiveDateTime> = OnceLock::new();
        CREATED_AT.get_or_init(|| {
            chrono::DateTime::parse_from_rfc3339("2025-09-24T17:00:11Z")
                .expect("failed to parse 'created_at' for 'zed-zippy[bot]'")
                .naive_utc()
        })
    }

    /// Returns whether the given contributor selector corresponds to the Zed Zippy bot user.
    fn is_zed_zippy_bot(contributor: &ContributorSelector) -> bool {
        match contributor {
            ContributorSelector::GitHubLogin { github_login } => github_login == Self::LOGIN,
            ContributorSelector::GitHubUserId { github_user_id } => {
                github_user_id == &Self::USER_ID
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct AddContributorBody {
    github_user_id: i32,
    github_login: String,
    github_email: Option<String>,
    github_name: Option<String>,
    github_user_created_at: chrono::DateTime<chrono::Utc>,
}

async fn add_contributor(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(params): extract::Json<AddContributorBody>,
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
