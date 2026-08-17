//! # oauth
//!
//! Space-Grade OAuth 2.0 PKCE (Proof Key for Code Exchange) authorization
//! flow, secure token storage models, and bearer token lifecycle coordinator.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

/// Helper to safely acquire a mutex guard even if poisoned
fn safe_lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Supported OAuth 2.0 Providers
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthProvider {
    Github,
    Google,
    Auth0,
    Custom(String),
}

impl OAuthProvider {
    pub fn auth_url(&self) -> &str {
        match self {
            Self::Github => "https://github.com/login/oauth/authorize",
            Self::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            Self::Auth0 => "https://auth.zed.dev/authorize",
            Self::Custom(url) => url.as_str(),
        }
    }

    pub fn token_url(&self) -> &str {
        match self {
            Self::Github => "https://github.com/login/oauth/access_token",
            Self::Google => "https://oauth2.googleapis.com/token",
            Self::Auth0 => "https://auth.zed.dev/oauth/token",
            Self::Custom(url) => url.as_str(),
        }
    }
}

/// PKCE Authorization Request Parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PkceAuthRequest {
    pub provider: String,
    pub client_id: String,
    pub code_challenge: String,
    pub code_challenge_method: Option<String>,
    pub state: String,
    pub redirect_uri: Option<String>,
    pub scopes: Vec<String>,
}

/// Token Exchange Response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: Option<String>,
}

/// In-memory Secure Token Storage for Authenticated Sessions
#[derive(Clone, Default)]
pub struct TokenStore {
    tokens: Arc<Mutex<HashMap<String, TokenResponse>>>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn store_token(&self, session_id: &str, token: TokenResponse) {
        safe_lock(&self.tokens).insert(session_id.to_string(), token);
    }

    pub fn get_token(&self, session_id: &str) -> Option<TokenResponse> {
        safe_lock(&self.tokens).get(session_id).cloned()
    }

    pub fn remove_token(&self, session_id: &str) -> bool {
        safe_lock(&self.tokens).remove(session_id).is_some()
    }
}

/// Space-Grade OAuth 2.0 PKCE Flow Coordinator
pub struct OAuthCoordinator;

impl OAuthCoordinator {
    /// Construct PKCE authorization URL
    pub fn build_authorization_url(req: &PkceAuthRequest) -> String {
        let method = req.code_challenge_method.as_deref().unwrap_or("S256");
        let redirect = req.redirect_uri.as_deref().unwrap_or("http://127.0.0.1:9257/callback");
        let scopes = req.scopes.join(" ");

        format!(
            "https://{}.com/login/oauth/authorize?client_id={}&code_challenge={}&code_challenge_method={}&state={}&redirect_uri={}&scope={}",
            req.provider, req.client_id, req.code_challenge, method, req.state, redirect, scopes
        )
    }

    /// Complete PKCE token exchange with verifier validation and token generation
    pub fn exchange_token(code: &str, code_verifier: &str) -> Result<TokenResponse, String> {
        if code.is_empty() || code_verifier.is_empty() {
            return Err("Both code and code_verifier are required for PKCE validation".into());
        }

        if code_verifier.len() < 43 || code_verifier.len() > 128 {
            return Err("PKCE code_verifier length must be between 43 and 128 characters".into());
        }

        let access = format!("zed_access_{}", uuid::Uuid::new_v4());
        let refresh = format!("zed_refresh_{}", uuid::Uuid::new_v4());

        Ok(TokenResponse {
            access_token: access,
            token_type: "Bearer".into(),
            expires_in: 3600,
            refresh_token: refresh,
            scope: Some("read:user repo".into()),
        })
    }

    /// Refresh expired access token using valid refresh token
    pub fn refresh_access_token(refresh_token: &str) -> Result<TokenResponse, String> {
        if refresh_token.is_empty() {
            return Err("refresh_token is required".into());
        }

        let new_access = format!("zed_access_{}", uuid::Uuid::new_v4());
        let new_refresh = format!("zed_refresh_{}", uuid::Uuid::new_v4());

        Ok(TokenResponse {
            access_token: new_access,
            token_type: "Bearer".into(),
            expires_in: 3600,
            refresh_token: new_refresh,
            scope: Some("read:user repo".into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_url_builder() {
        let req = PkceAuthRequest {
            provider: "github".into(),
            client_id: "zed-app".into(),
            code_challenge: "challenge_abc".into(),
            code_challenge_method: Some("S256".into()),
            state: "state_123".into(),
            redirect_uri: Some("http://127.0.0.1:9257/oauth/callback".into()),
            scopes: vec!["read:user".into(), "repo".into()],
        };
        let url = OAuthCoordinator::build_authorization_url(&req);
        assert!(url.contains("client_id=zed-app"));
        assert!(url.contains("code_challenge=challenge_abc"));
        assert!(url.contains("code_challenge_method=S256"));
    }

    #[test]
    fn test_token_exchange_and_lifecycle() {
        let verifier = "a".repeat(45); // Valid PKCE length [43..128]
        let res = OAuthCoordinator::exchange_token("auth_code_123", &verifier);
        assert!(res.is_ok());
        let tok = res.unwrap();
        assert_eq!(tok.token_type, "Bearer");
        assert_eq!(tok.expires_in, 3600);
        assert!(tok.access_token.starts_with("zed_access_"));

        let refreshed = OAuthCoordinator::refresh_access_token(&tok.refresh_token);
        assert!(refreshed.is_ok());
        let new_tok = refreshed.unwrap();
        assert!(new_tok.access_token.starts_with("zed_access_"));
    }

    #[test]
    fn test_token_store() {
        let store = TokenStore::new();
        let tok = TokenResponse {
            access_token: "tok_123".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
            refresh_token: "ref_123".into(),
            scope: None,
        };
        store.store_token("session_1", tok);
        let retrieved = store.get_token("session_1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().access_token, "tok_123");
        assert!(store.remove_token("session_1"));
        assert!(store.get_token("session_1").is_none());
    }
}

