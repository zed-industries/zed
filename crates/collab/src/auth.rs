use std::sync::Arc;

use super::db::{self, UserId};
use crate::{AppState, Error, Result};
use anyhow::{anyhow, Context};
use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use rand::thread_rng;
use scrypt::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

pub async fn validate_header<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
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
    let mut credentials_valid = false;
    for password_hash in state.db.get_access_token_hashes(user_id).await? {
        if verify_access_token(&access_token, &password_hash)? {
            credentials_valid = true;
            break;
        }
    }

    if credentials_valid {
        let user = state
            .db
            .get_user_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("user {} not found", user_id))?;
        req.extensions_mut().insert(user);
        Ok::<_, Error>(next.run(req).await)
    } else {
        Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "invalid credentials".to_string(),
        ))
    }
}

const MAX_ACCESS_TOKENS_TO_STORE: usize = 8;

pub async fn create_access_token(db: &dyn db::Db, user_id: UserId) -> Result<String> {
    let access_token = rpc::auth::random_token();
    let access_token_hash =
        hash_access_token(&access_token).context("failed to hash access token")?;
    db.create_access_token_hash(user_id, &access_token_hash, MAX_ACCESS_TOKENS_TO_STORE)
        .await?;
    Ok(access_token)
}

fn hash_access_token(token: &str) -> Result<String> {
    // Avoid slow hashing in debug mode.
    let params = if cfg!(debug_assertions) {
        scrypt::Params::new(1, 1, 1).unwrap()
    } else {
        scrypt::Params::recommended()
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

pub fn encrypt_access_token(access_token: &str, public_key: String) -> Result<String> {
    let native_app_public_key =
        rpc::auth::PublicKey::try_from(public_key).context("failed to parse app public key")?;
    let encrypted_access_token = native_app_public_key
        .encrypt_string(&access_token)
        .context("failed to encrypt access token with public key")?;
    Ok(encrypted_access_token)
}

pub fn verify_access_token(token: &str, hash: &str) -> Result<bool> {
    let hash = PasswordHash::new(hash).map_err(anyhow::Error::new)?;
    Ok(Scrypt.verify_password(token.as_bytes(), &hash).is_ok())
}
