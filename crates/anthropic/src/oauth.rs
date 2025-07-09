use anyhow::{Context as _, Result, anyhow};
use base64::Engine;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

const CLIENT_ID: &str = "9d1c250a-e61b-44d9-88ed-5944d1962f5e";
const AUTHORIZATION_URL: &str = "https://claude.ai/oauth/authorize";
const TOKEN_URL: &str = "https://console.anthropic.com/v1/oauth/token";
const REDIRECT_URI: &str = "https://console.anthropic.com/oauth/code/callback";
const SCOPE: &str = "org:create_api_key user:profile user:inference";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
    pub token_type: String,
}

impl OAuthTokens {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now >= self.expires_at
    }
}

#[derive(Debug, Clone)]
pub struct PkceChallenge {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    token_type: String,
}

#[derive(Clone)]
pub struct AnthropicOAuth {
    http_client: Arc<dyn HttpClient>,
}

impl AnthropicOAuth {
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self { http_client }
    }

    pub fn generate_pkce_challenge() -> Result<PkceChallenge> {
        let mut rng = rand::thread_rng();
        let code_verifier: String = (0..128)
            .map(|_| {
                let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
                chars[rng.gen_range(0..chars.len())] as char
            })
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let hash = hasher.finalize();
        let code_challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash);

        let state = code_verifier.clone();

        Ok(PkceChallenge {
            code_verifier,
            code_challenge,
            state,
        })
    }

    pub fn get_authorization_url(&self, pkce_challenge: &PkceChallenge) -> Result<String> {
        let mut url = Url::parse(AUTHORIZATION_URL)?;

        let params = [
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("code", "true"),
            ("scope", SCOPE),
            ("code_challenge", &pkce_challenge.code_challenge),
            ("code_challenge_method", "S256"),
            ("redirect_uri", REDIRECT_URI),
            ("state", &pkce_challenge.state),
        ];

        for (key, value) in &params {
            url.query_pairs_mut().append_pair(key, value);
        }

        Ok(url.to_string())
    }

    pub async fn exchange_code_for_tokens(
        &self,
        code_input: &str,
        expected_state: &str,
        pkce_verifier: &str,
    ) -> Result<OAuthTokens> {
        let splits: Vec<&str> = code_input.split('#').collect();
        let actual_code = splits[0];
        let actual_state = if splits.len() > 1 { splits[1] } else { "" };

        // Validate state parameter to prevent CSRF attacks
        if actual_state != expected_state && !actual_state.is_empty() {
            return Err(anyhow!(
                "Invalid state parameter: expected {}, got {}",
                expected_state,
                actual_state
            ));
        }

        let request_body = serde_json::json!({
            "code": actual_code,
            "state": actual_state,
            "grant_type": "authorization_code",
            "client_id": CLIENT_ID,
            "redirect_uri": REDIRECT_URI,
            "code_verifier": pkce_verifier,
        });

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(TOKEN_URL)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(request_body.to_string()))
            .context("Failed to build token exchange request")?;

        let mut response = self
            .http_client
            .send(request)
            .await
            .context("Failed to send token exchange request")?;

        let mut body = String::new();
        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
            .await
            .context("Failed to read token response")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Token exchange failed with status {}: {}",
                response.status(),
                body
            ));
        }

        let token_response: TokenResponse =
            serde_json::from_str(&body).context("Failed to parse token response")?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + token_response.expires_in;

        Ok(OAuthTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            token_type: token_response.token_type,
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthTokens> {
        let request_body = HashMap::from([
            ("grant_type", "refresh_token"),
            ("client_id", CLIENT_ID),
            ("refresh_token", refresh_token),
        ]);

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(TOKEN_URL)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(serde_json::to_string(&request_body)?))
            .context("Failed to build token refresh request")?;

        let mut response = self
            .http_client
            .send(request)
            .await
            .context("Failed to send token refresh request")?;

        let mut body = String::new();
        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
            .await
            .context("Failed to read refresh response")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Token refresh failed with status {}: {}",
                response.status(),
                body
            ));
        }

        let token_response: TokenResponse =
            serde_json::from_str(&body).context("Failed to parse refresh response")?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + token_response.expires_in;

        Ok(OAuthTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            token_type: token_response.token_type,
        })
    }
}
