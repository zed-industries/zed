use anyhow::{Result, anyhow};
use base64::Engine as _;
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use rand::RngCore as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub const CODEX_API_URL: &str = "https://chatgpt.com/backend-api";
pub const CODEX_OAUTH_CREDENTIALS_KEY: &str = "openai-codex-oauth";

// These must match the OpenAI Codex CLI OAuth app's registered redirect URI.
pub const CODEX_CLI_OAUTH_REDIRECT_URI: &str = "http://localhost:1455/auth/callback";

const CODEX_OAUTH_CALLBACK_PATH: &str = "/auth/callback";
const CODEX_CLI_CLIENT_ID: &str = "app_EMoamEEZ73f0CkXaXp7hrann";
const AUTHORIZE_URL: &str = "https://auth.openai.com/oauth/authorize";
const TOKEN_URL: &str = "https://auth.openai.com/oauth/token";
const OAUTH_SCOPE: &str = "openid profile email offline_access";
const JWT_CLAIM_PATH: &str = "https://api.openai.com/auth";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodexOAuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at_ms: u64,
    pub account_id: String,
}

#[derive(Debug)]
pub struct CodexAuthorizationFlow {
    pub verifier: String,
    pub state: String,
    pub url: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

pub fn create_codex_authorization_flow() -> Result<CodexAuthorizationFlow> {
    let verifier = random_urlsafe(32);
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(Sha256::digest(verifier.as_bytes()));
    let state = random_urlsafe(16);
    let mut url = url::Url::parse(AUTHORIZE_URL)?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", CODEX_CLI_CLIENT_ID)
        .append_pair("redirect_uri", CODEX_CLI_OAUTH_REDIRECT_URI)
        .append_pair("scope", OAUTH_SCOPE)
        .append_pair("code_challenge", &challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", &state)
        .append_pair("id_token_add_organizations", "true")
        .append_pair("codex_cli_simplified_flow", "true")
        .append_pair("originator", "zed");
    Ok(CodexAuthorizationFlow {
        verifier,
        state,
        url: url.to_string(),
    })
}

pub async fn exchange_codex_authorization_code(
    http_client: &dyn HttpClient,
    code: &str,
    verifier: &str,
) -> Result<CodexOAuthSession> {
    token_request(
        http_client,
        &[
            ("grant_type", "authorization_code"),
            ("client_id", CODEX_CLI_CLIENT_ID),
            ("code", code),
            ("code_verifier", verifier),
            ("redirect_uri", CODEX_CLI_OAUTH_REDIRECT_URI),
        ],
    )
    .await
}

pub async fn refresh_codex_session(
    http_client: &dyn HttpClient,
    refresh_token: &str,
) -> Result<CodexOAuthSession> {
    token_request(
        http_client,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", CODEX_CLI_CLIENT_ID),
            ("refresh_token", refresh_token),
        ],
    )
    .await
}

pub fn parse_codex_authorization_callback_path(path: &str, expected_state: &str) -> Result<String> {
    let url = url::Url::parse(&format!("http://localhost{path}"))?;
    let callback_state = url
        .query_pairs()
        .find_map(|(key, value)| (key == "state").then_some(value.into_owned()));
    let code = url
        .query_pairs()
        .find_map(|(key, value)| (key == "code").then_some(value.into_owned()));

    match (
        url.path() == CODEX_OAUTH_CALLBACK_PATH,
        callback_state.as_deref() == Some(expected_state),
        code,
    ) {
        (true, true, Some(code)) => Ok(code),
        (false, _, _) => Err(anyhow!("OAuth callback route did not match /auth/callback")),
        (true, false, _) => Err(anyhow!(
            "OAuth callback state did not match the login request"
        )),
        (true, true, None) => Err(anyhow!(
            "OAuth callback did not include an authorization code"
        )),
    }
}

async fn token_request(
    http_client: &dyn HttpClient,
    params: &[(&str, &str)],
) -> Result<CodexOAuthSession> {
    let body = url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter().copied())
        .finish();
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(TOKEN_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = http_client.send(request).await?;
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "OpenAI Codex OAuth token request failed: {} {}",
            response.status(),
            body
        ));
    }

    let token: TokenResponse = serde_json::from_str(&body)?;
    let account_id = extract_codex_account_id(&token.access_token)?;
    Ok(CodexOAuthSession {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        expires_at_ms: now_ms() + token.expires_in * 1000,
        account_id,
    })
}

fn extract_codex_account_id(access_token: &str) -> Result<String> {
    let payload = access_token
        .split('.')
        .nth(1)
        .ok_or_else(|| anyhow!("OpenAI Codex access token is not a JWT"))?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(payload))?;
    let json: serde_json::Value = serde_json::from_slice(&decoded)?;
    json.get(JWT_CLAIM_PATH)
        .and_then(|auth| auth.get("chatgpt_account_id"))
        .and_then(|account_id| account_id.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow!("OpenAI Codex access token did not include a ChatGPT account ID"))
}

fn random_urlsafe(bytes_len: usize) -> String {
    let mut bytes = vec![0; bytes_len];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub fn now_ms() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_chatgpt_account_id_from_access_token() -> Result<()> {
        let payload = serde_json::json!({
            JWT_CLAIM_PATH: {
                "chatgpt_account_id": "account-id"
            }
        });
        let token = format!(
            "header.{}.signature",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload.to_string())
        );

        assert_eq!(extract_codex_account_id(&token)?, "account-id");
        Ok(())
    }

    #[test]
    fn builds_codex_authorization_flow() -> Result<()> {
        let flow = create_codex_authorization_flow()?;
        let url = url::Url::parse(&flow.url)?;
        let query = url
            .query_pairs()
            .collect::<std::collections::HashMap<_, _>>();

        assert_eq!(url.as_str().split('?').next(), Some(AUTHORIZE_URL));
        assert_eq!(
            query.get("client_id").map(|value| value.as_ref()),
            Some(CODEX_CLI_CLIENT_ID)
        );
        assert_eq!(
            query.get("redirect_uri").map(|value| value.as_ref()),
            Some(CODEX_CLI_OAUTH_REDIRECT_URI)
        );
        assert_eq!(
            query
                .get("code_challenge_method")
                .map(|value| value.as_ref()),
            Some("S256")
        );
        assert_eq!(
            query.get("state").map(|value| value.as_ref()),
            Some(flow.state.as_str())
        );
        assert!(!flow.verifier.is_empty());
        Ok(())
    }

    #[test]
    fn parses_codex_authorization_callback_path() -> Result<()> {
        assert_eq!(
            parse_codex_authorization_callback_path(
                "/auth/callback?code=authorization-code&state=expected-state",
                "expected-state",
            )?,
            "authorization-code"
        );

        assert!(
            parse_codex_authorization_callback_path(
                "/auth/callback?code=authorization-code&state=other-state",
                "expected-state",
            )
            .is_err()
        );
        Ok(())
    }
}
