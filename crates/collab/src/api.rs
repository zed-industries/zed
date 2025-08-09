pub mod billing;
pub mod contributors;
pub mod events;
pub mod extensions;
pub mod ips_file;
pub mod slack;

use crate::db::Database;
use crate::{
    AppState, Error, Result, auth,
    db::{User, UserId},
    rpc,
};
use ::rpc::proto;
use anyhow::Context as _;
use axum::extract;
use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{Path, Query},
    headers::Header,
    http::{self, HeaderName, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::response::ErasedJson;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use tower::ServiceBuilder;

pub use extensions::fetch_extensions_from_blob_store_periodically;

pub struct CloudflareIpCountryHeader(String);

impl Header for CloudflareIpCountryHeader {
    fn name() -> &'static HeaderName {
        static CLOUDFLARE_IP_COUNTRY_HEADER: OnceLock<HeaderName> = OnceLock::new();
        CLOUDFLARE_IP_COUNTRY_HEADER.get_or_init(|| HeaderName::from_static("cf-ipcountry"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let country_code = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        Ok(Self(country_code.to_string()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

impl std::fmt::Display for CloudflareIpCountryHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SystemIdHeader(String);

impl Header for SystemIdHeader {
    fn name() -> &'static HeaderName {
        static SYSTEM_ID_HEADER: OnceLock<HeaderName> = OnceLock::new();
        SYSTEM_ID_HEADER.get_or_init(|| HeaderName::from_static("x-zed-system-id"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let system_id = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        Ok(Self(system_id.to_string()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

impl std::fmt::Display for SystemIdHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn routes(rpc_server: Arc<rpc::Server>) -> Router<(), Body> {
    Router::new()
        .route("/users/look_up", get(look_up_user))
        .route("/users/:id/access_tokens", post(create_access_token))
        .route("/users/:id/refresh_llm_tokens", post(refresh_llm_tokens))
        .route("/users/:id/update_plan", post(update_plan))
        .route("/rpc_server_snapshot", get(get_rpc_server_snapshot))
        .merge(contributors::router())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(rpc_server))
                .layer(middleware::from_fn(validate_api_token)),
        )
}

pub async fn validate_api_token<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("token ")
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    if token != state.config.api_token {
        Err(Error::http(
            StatusCode::UNAUTHORIZED,
            "invalid authorization token".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

#[derive(Debug, Deserialize)]
struct LookUpUserParams {
    identifier: String,
}

#[derive(Debug, Serialize)]
struct LookUpUserResponse {
    user: Option<User>,
}

async fn look_up_user(
    Query(params): Query<LookUpUserParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<LookUpUserResponse>> {
    let user = resolve_identifier_to_user(&app.db, &params.identifier).await?;
    let user = if let Some(user) = user {
        match user {
            UserOrId::User(user) => Some(user),
            UserOrId::Id(id) => app.db.get_user_by_id(id).await?,
        }
    } else {
        None
    };

    Ok(Json(LookUpUserResponse { user }))
}

enum UserOrId {
    User(User),
    Id(UserId),
}

async fn resolve_identifier_to_user(
    db: &Arc<Database>,
    identifier: &str,
) -> Result<Option<UserOrId>> {
    if let Some(identifier) = identifier.parse::<i32>().ok() {
        let user = db.get_user_by_id(UserId(identifier)).await?;

        return Ok(user.map(UserOrId::User));
    }

    if identifier.starts_with("cus_") {
        let billing_customer = db
            .get_billing_customer_by_stripe_customer_id(&identifier)
            .await?;

        return Ok(billing_customer.map(|billing_customer| UserOrId::Id(billing_customer.user_id)));
    }

    if identifier.starts_with("sub_") {
        let billing_subscription = db
            .get_billing_subscription_by_stripe_subscription_id(&identifier)
            .await?;

        if let Some(billing_subscription) = billing_subscription {
            let billing_customer = db
                .get_billing_customer_by_id(billing_subscription.billing_customer_id)
                .await?;

            return Ok(
                billing_customer.map(|billing_customer| UserOrId::Id(billing_customer.user_id))
            );
        } else {
            return Ok(None);
        }
    }

    if identifier.contains('@') {
        let user = db.get_user_by_email(identifier).await?;

        return Ok(user.map(UserOrId::User));
    }

    if let Some(user) = db.get_user_by_github_login(identifier).await? {
        return Ok(Some(UserOrId::User(user)));
    }

    Ok(None)
}

#[derive(Deserialize, Debug)]
struct CreateUserParams {
    github_user_id: i32,
    github_login: String,
    email_address: String,
    email_confirmation_code: Option<String>,
    #[serde(default)]
    admin: bool,
    #[serde(default)]
    invite_count: i32,
}

async fn get_rpc_server_snapshot(
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<ErasedJson> {
    Ok(ErasedJson::pretty(rpc_server.snapshot().await))
}

#[derive(Deserialize)]
struct CreateAccessTokenQueryParams {
    public_key: String,
    impersonate: Option<String>,
}

#[derive(Serialize)]
struct CreateAccessTokenResponse {
    user_id: UserId,
    encrypted_access_token: String,
}

async fn create_access_token(
    Path(user_id): Path<UserId>,
    Query(params): Query<CreateAccessTokenQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<CreateAccessTokenResponse>> {
    let user = app
        .db
        .get_user_by_id(user_id)
        .await?
        .context("user not found")?;

    let mut impersonated_user_id = None;
    if let Some(impersonate) = params.impersonate {
        if user.admin {
            if let Some(impersonated_user) = app.db.get_user_by_github_login(&impersonate).await? {
                impersonated_user_id = Some(impersonated_user.id);
            } else {
                return Err(Error::http(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("user {impersonate} does not exist"),
                ));
            }
        } else {
            return Err(Error::http(
                StatusCode::UNAUTHORIZED,
                "you do not have permission to impersonate other users".to_string(),
            ));
        }
    }

    let access_token =
        auth::create_access_token(app.db.as_ref(), user_id, impersonated_user_id).await?;
    let encrypted_access_token =
        auth::encrypt_access_token(&access_token, params.public_key.clone())?;

    Ok(Json(CreateAccessTokenResponse {
        user_id: impersonated_user_id.unwrap_or(user_id),
        encrypted_access_token,
    }))
}

#[derive(Serialize)]
struct RefreshLlmTokensResponse {}

async fn refresh_llm_tokens(
    Path(user_id): Path<UserId>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<Json<RefreshLlmTokensResponse>> {
    rpc_server.refresh_llm_tokens_for_user(user_id).await;

    Ok(Json(RefreshLlmTokensResponse {}))
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdatePlanBody {
    pub plan: cloud_llm_client::Plan,
    pub subscription_period: SubscriptionPeriod,
    pub usage: cloud_llm_client::CurrentUsage,
    pub trial_started_at: Option<DateTime<Utc>>,
    pub is_usage_based_billing_enabled: bool,
    pub is_account_too_young: bool,
    pub has_overdue_invoices: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
struct SubscriptionPeriod {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

#[derive(Serialize)]
struct UpdatePlanResponse {}

async fn update_plan(
    Path(user_id): Path<UserId>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
    extract::Json(body): extract::Json<UpdatePlanBody>,
) -> Result<Json<UpdatePlanResponse>> {
    let plan = match body.plan {
        cloud_llm_client::Plan::ZedFree => proto::Plan::Free,
        cloud_llm_client::Plan::ZedPro => proto::Plan::ZedPro,
        cloud_llm_client::Plan::ZedProTrial => proto::Plan::ZedProTrial,
    };

    let update_user_plan = proto::UpdateUserPlan {
        plan: plan.into(),
        trial_started_at: body
            .trial_started_at
            .map(|trial_started_at| trial_started_at.timestamp() as u64),
        is_usage_based_billing_enabled: Some(body.is_usage_based_billing_enabled),
        usage: Some(proto::SubscriptionUsage {
            model_requests_usage_amount: body.usage.model_requests.used,
            model_requests_usage_limit: Some(usage_limit_to_proto(body.usage.model_requests.limit)),
            edit_predictions_usage_amount: body.usage.edit_predictions.used,
            edit_predictions_usage_limit: Some(usage_limit_to_proto(
                body.usage.edit_predictions.limit,
            )),
        }),
        subscription_period: Some(proto::SubscriptionPeriod {
            started_at: body.subscription_period.started_at.timestamp() as u64,
            ended_at: body.subscription_period.ended_at.timestamp() as u64,
        }),
        account_too_young: Some(body.is_account_too_young),
        has_overdue_invoices: Some(body.has_overdue_invoices),
    };

    rpc_server
        .update_plan_for_user(user_id, update_user_plan)
        .await?;

    Ok(Json(UpdatePlanResponse {}))
}

fn usage_limit_to_proto(limit: cloud_llm_client::UsageLimit) -> proto::UsageLimit {
    proto::UsageLimit {
        variant: Some(match limit {
            cloud_llm_client::UsageLimit::Limited(limit) => {
                proto::usage_limit::Variant::Limited(proto::usage_limit::Limited {
                    limit: limit as u32,
                })
            }
            cloud_llm_client::UsageLimit::Unlimited => {
                proto::usage_limit::Variant::Unlimited(proto::usage_limit::Unlimited {})
            }
        }),
    }
}
