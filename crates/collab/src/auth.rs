use crate::{
    db::{self, AccessTokenId, Database, UserId},
    AppState, Error, Result,
};
use anyhow::{anyhow, Context};
use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use prometheus::{exponential_buckets, register_histogram, Histogram};
use rand::thread_rng;
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::{sync::Arc, time::Instant};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Impersonator(pub Option<db::User>);

/// Validates the authorization header. This has two mechanisms, one for the ADMIN_TOKEN
/// and one for the access tokens that we issue.
pub async fn validate_header<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::Http(
                StatusCode::UNAUTHORIZED,
                "missing authorization header".to_string(),
            )
        })?
        .split_whitespace();

    let user_id = UserId(auth_header.next().unwrap_or("").parse().map_err(|_| {
        Error::Http(
            StatusCode::BAD_REQUEST,
            "missing user id in authorization header".to_string(),
        )
    })?);

    let access_token = auth_header.next().ok_or_else(|| {
        Error::Http(
            StatusCode::BAD_REQUEST,
            "missing access token in authorization header".to_string(),
        )
    })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    // In development, allow impersonation using the admin API token.
    // Don't allow this in production because we can't tell who is doing
    // the impersonating.
    let validate_result = if let (Some(admin_token), true) = (
        access_token.strip_prefix("ADMIN_TOKEN:"),
        state.config.is_development(),
    ) {
        Ok(VerifyAccessTokenResult {
            is_valid: state.config.api_token == admin_token,
            impersonator_id: None,
        })
    } else {
        verify_access_token(&access_token, user_id, &state.db).await
    };

    if let Ok(validate_result) = validate_result {
        if validate_result.is_valid {
            let user = state
                .db
                .get_user_by_id(user_id)
                .await?
                .ok_or_else(|| anyhow!("user {} not found", user_id))?;

            let impersonator = if let Some(impersonator_id) = validate_result.impersonator_id {
                let impersonator = state
                    .db
                    .get_user_by_id(impersonator_id)
                    .await?
                    .ok_or_else(|| anyhow!("user {} not found", impersonator_id))?;
                Some(impersonator)
            } else {
                None
            };
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(Impersonator(impersonator));
            return Ok::<_, Error>(next.run(req).await);
        }
    }

    Err(Error::Http(
        StatusCode::UNAUTHORIZED,
        "invalid credentials".to_string(),
    ))
}

const MAX_ACCESS_TOKENS_TO_STORE: usize = 8;

#[derive(Serialize, Deserialize)]
struct AccessTokenJson {
    version: usize,
    id: AccessTokenId,
    token: String,
}

/// Creates a new access token to identify the given user. before returning it, you should
/// encrypt it with the user's public key.
pub async fn create_access_token(
    db: &db::Database,
    user_id: UserId,
    impersonated_user_id: Option<UserId>,
) -> Result<String> {
    const VERSION: usize = 1;
    let access_token = rpc::auth::random_token();
    let access_token_hash =
        hash_access_token(&access_token).context("failed to hash access token")?;
    let id = db
        .create_access_token(
            user_id,
            impersonated_user_id,
            &access_token_hash,
            MAX_ACCESS_TOKENS_TO_STORE,
        )
        .await?;
    Ok(serde_json::to_string(&AccessTokenJson {
        version: VERSION,
        id,
        token: access_token,
    })?)
}

fn hash_access_token(token: &str) -> Result<String> {
    // Avoid slow hashing in debug mode.
    let params = if cfg!(debug_assertions) {
        scrypt::Params::new(1, 1, 1).unwrap()
    } else {
        scrypt::Params::new(14, 8, 1).unwrap()
    };

    Ok(Scrypt
        .hash_password(
            token.as_bytes(),
            None,
            params,
            &SaltString::generate(thread_rng()),
        )
        .map_err(anyhow::Error::new)?
        .to_string())
}

/// Encrypts the given access token with the given public key to avoid leaking it on the way
/// to the client.
pub fn encrypt_access_token(access_token: &str, public_key: String) -> Result<String> {
    let native_app_public_key =
        rpc::auth::PublicKey::try_from(public_key).context("failed to parse app public key")?;
    let encrypted_access_token = native_app_public_key
        .encrypt_string(access_token)
        .context("failed to encrypt access token with public key")?;
    Ok(encrypted_access_token)
}

pub struct VerifyAccessTokenResult {
    pub is_valid: bool,
    pub impersonator_id: Option<UserId>,
}

/// Checks that the given access token is valid for the given user.
pub async fn verify_access_token(
    token: &str,
    user_id: UserId,
    db: &Arc<Database>,
) -> Result<VerifyAccessTokenResult> {
    static METRIC_ACCESS_TOKEN_HASHING_TIME: OnceLock<Histogram> = OnceLock::new();
    let metric_access_token_hashing_time = METRIC_ACCESS_TOKEN_HASHING_TIME.get_or_init(|| {
        register_histogram!(
            "access_token_hashing_time",
            "time spent hashing access tokens",
            exponential_buckets(10.0, 2.0, 10).unwrap(),
        )
        .unwrap()
    });

    let token: AccessTokenJson = serde_json::from_str(&token)?;

    let db_token = db.get_access_token(token.id).await?;
    let token_user_id = db_token.impersonated_user_id.unwrap_or(db_token.user_id);
    if token_user_id != user_id {
        return Err(anyhow!("no such access token"))?;
    }

    let db_hash = PasswordHash::new(&db_token.hash).map_err(anyhow::Error::new)?;
    let t0 = Instant::now();
    let is_valid = Scrypt
        .verify_password(token.token.as_bytes(), &db_hash)
        .is_ok();
    let duration = t0.elapsed();
    log::info!("hashed access token in {:?}", duration);
    metric_access_token_hashing_time.observe(duration.as_millis() as f64);
    Ok(VerifyAccessTokenResult {
        is_valid,
        impersonator_id: if db_token.impersonated_user_id.is_some() {
            Some(db_token.user_id)
        } else {
            None
        },
    })
}
