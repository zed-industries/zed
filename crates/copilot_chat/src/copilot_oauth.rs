//! A native GitHub OAuth device-code flow for Copilot.
//!
//! Historically, Zed relied on the Copilot language server to perform the
//! OAuth flow and then read the resulting token out of the language server's
//! on-disk storage. That coupling was fragile. Instead, each Copilot provider
//! (agent and edit predictions) now drives this device-code flow itself and
//! stores the resulting token independently.

use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, bail};
use futures::AsyncReadExt as _;
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::Deserialize;

use crate::CopilotChatConfiguration;

/// The public OAuth application client ID used by GitHub Copilot editor
/// integrations (the same one the Copilot language server uses). Reusing it
/// lets Zed request the same Copilot scopes without registering its own app.
pub const GITHUB_COPILOT_CLIENT_ID: &str = "Iv1.b507a08c87ecfe98";

const DEVICE_CODE_SCOPE: &str = "read:user";
const DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

/// An in-progress device-code authorization. The user must visit
/// [`Self::verification_uri`] and enter [`Self::user_code`] to complete it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceFlow {
    pub user_code: String,
    pub verification_uri: String,
    device_code: String,
    interval: u64,
}

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default = "default_interval")]
    interval: u64,
}

fn default_interval() -> u64 {
    5
}

#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

/// Begins a device-code authorization by requesting a user code from GitHub.
pub async fn request_device_code(
    client: &Arc<dyn HttpClient>,
    configuration: &CopilotChatConfiguration,
) -> Result<DeviceFlow> {
    let body = form_encode(&[
        ("client_id", GITHUB_COPILOT_CLIENT_ID),
        ("scope", DEVICE_CODE_SCOPE),
    ]);

    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(configuration.device_code_url())
        .header("Accept", "application/json")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(AsyncBody::from(body))?;

    let mut response = client.send(request).await?;
    anyhow::ensure!(
        response.status().is_success(),
        "GitHub device-code request failed: {}",
        response.status()
    );

    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;
    let parsed: DeviceCodeResponse = serde_json::from_slice(&body)
        .context("Failed to parse GitHub device-code response")?;

    Ok(DeviceFlow {
        user_code: parsed.user_code,
        verification_uri: parsed.verification_uri,
        device_code: parsed.device_code,
        interval: parsed.interval.max(1),
    })
}

/// Polls GitHub until the user completes the device-code authorization,
/// returning the resulting OAuth (`ghu_…`) token.
pub async fn poll_for_access_token(
    client: &Arc<dyn HttpClient>,
    configuration: &CopilotChatConfiguration,
    device_flow: &DeviceFlow,
    executor: &BackgroundExecutor,
) -> Result<String> {
    let mut interval = device_flow.interval;
    let body = form_encode(&[
        ("client_id", GITHUB_COPILOT_CLIENT_ID),
        ("device_code", device_flow.device_code.as_str()),
        ("grant_type", DEVICE_CODE_GRANT_TYPE),
    ]);

    loop {
        executor.timer(Duration::from_secs(interval)).await;

        let request = HttpRequest::builder()
            .method(Method::POST)
            .uri(configuration.access_token_url())
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(AsyncBody::from(body.clone()))?;

        let mut response = client.send(request).await?;
        let mut response_body = Vec::new();
        response.body_mut().read_to_end(&mut response_body).await?;

        let parsed: AccessTokenResponse = serde_json::from_slice(&response_body)
            .context("Failed to parse GitHub access-token response")?;

        if let Some(token) = parsed.access_token {
            return Ok(token);
        }

        match parsed.error.as_deref() {
            Some("authorization_pending") => continue,
            // GitHub asks us to back off; increase the interval and keep polling.
            Some("slow_down") => interval += 5,
            Some("expired_token") => bail!("The Copilot sign-in code expired. Please try again."),
            Some("access_denied") => bail!("Copilot sign-in was cancelled."),
            Some(other) => bail!("Copilot sign-in failed: {other}"),
            None => bail!("Copilot sign-in failed: unexpected response from GitHub"),
        }
    }
}

fn form_encode(fields: &[(&str, &str)]) -> String {
    fields
        .iter()
        .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
        .collect::<Vec<_>>()
        .join("&")
}

fn url_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}
