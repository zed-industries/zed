//! Integration tests for diagnosing GitHub Copilot API authentication.
//!
//! These tests hit the real Copilot API and require a valid OAuth token.
//! They are all `#[ignore]`d so they won't run in CI.
//!
//! To run all:
//!   GH_COPILOT_TOKEN=<token> cargo test -p copilot_chat -- --ignored test_copilot --nocapture
//!
//! You can grab the token from ~/.config/github-copilot/apps.json (the `oauth_token` field).

use std::sync::Arc;

use futures::AsyncReadExt;
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use serde::Deserialize;

use crate::{
    COPILOT_OAUTH_ENV_VAR, ChatMessage, ChatMessageContent, ModelSchema, Request,
    copilot_request_headers,
};

const GITHUBCOPILOT_ENDPOINT: &str = "https://api.githubcopilot.com";

fn create_test_client() -> Arc<dyn HttpClient> {
    Arc::new(
        reqwest_client::ReqwestClient::user_agent("Zed/test (integration-test)")
            .expect("failed to create HTTP client"),
    )
}

fn get_test_oauth_token() -> String {
    std::env::var(COPILOT_OAUTH_ENV_VAR).expect("Set GH_COPILOT_TOKEN env var to run this test")
}

#[derive(Deserialize)]
struct TokenExchangeResponse {
    token: String,
    endpoints: TokenExchangeEndpoints,
}

#[derive(Deserialize)]
struct TokenExchangeEndpoints {
    api: String,
}

/// Calls the Copilot token exchange endpoint to get:
/// - The copilot-internal token (for comparison/baseline testing)
/// - The discovered API endpoint URL (e.g. `https://api.business.githubcopilot.com`)
async fn do_token_exchange(oauth_token: &str, client: &Arc<dyn HttpClient>) -> (String, String) {
    let token_url = "https://api.github.com/copilot_internal/v2/token";

    let request = HttpRequest::builder()
        .method(Method::GET)
        .uri(token_url)
        .header("Authorization", format!("token {}", oauth_token))
        .header("Accept", "application/json")
        .body(AsyncBody::empty())
        .expect("failed to build token request");

    let (status, body) = send_and_read(client, request).await;
    assert!(
        status.is_success(),
        "Token exchange failed: {status}\n{body}"
    );

    let parsed: TokenExchangeResponse =
        serde_json::from_str(&body).expect("failed to parse token response");
    (parsed.token, parsed.endpoints.api)
}

async fn send_and_read(
    client: &Arc<dyn HttpClient>,
    request: http_client::Request<AsyncBody>,
) -> (http_client::StatusCode, String) {
    let mut response = client.send(request).await.expect("failed to send request");
    let status = response.status();
    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .expect("failed to read response body");
    let body_str = std::str::from_utf8(&body).expect("response body is not utf8");
    (status, body_str.to_string())
}

fn make_simple_completion_request() -> Request {
    Request {
        intent: false,
        n: 1,
        stream: false,
        temperature: 0.0,
        model: "gpt-4o-mini".to_string(),
        messages: vec![ChatMessage::User {
            content: ChatMessageContent::Plain("Say exactly: hello".to_string()),
        }],
        tools: vec![],
        tool_choice: None,
    }
}

/// Sends a models request against the given base endpoint using the given token.
/// Returns (status, parsed_model_count_or_none).
async fn try_models(
    client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
) -> (http_client::StatusCode, String) {
    let models_url = format!("{base_url}/models");
    let request = copilot_request_headers(
        HttpRequest::builder()
            .method(Method::GET)
            .uri(models_url.as_str()),
        token,
        None,
    )
    .header("x-github-api-version", "2025-05-01")
    .body(AsyncBody::empty())
    .expect("failed to build request");

    send_and_read(client, request).await
}

/// Sends a completions request against the given base endpoint using the given token.
/// Optionally adds a Copilot-Integration-Id header.
async fn try_completion(
    client: &Arc<dyn HttpClient>,
    base_url: &str,
    token: &str,
    integration_id: Option<&str>,
) -> (http_client::StatusCode, String) {
    let completions_url = format!("{base_url}/chat/completions");
    let json = serde_json::to_string(&make_simple_completion_request()).expect("serialize");

    let mut builder = copilot_request_headers(
        HttpRequest::builder()
            .method(Method::POST)
            .uri(completions_url.as_str()),
        token,
        Some(true),
    );
    if let Some(id) = integration_id {
        builder = builder.header("Copilot-Integration-Id", id);
    }

    let request = builder
        .body(AsyncBody::from(json))
        .expect("failed to build request");

    send_and_read(client, request).await
}

/// Integration test: models listing with OAuth token at both endpoints.
///
/// Run with:
///   GH_COPILOT_TOKEN=<token> cargo test -p copilot_chat -- --ignored test_copilot_models --nocapture
#[ignore]
#[test]
fn test_copilot_models() {
    let oauth_token = get_test_oauth_token();
    let client = create_test_client();

    smol::block_on(async {
        println!("=== Copilot Models Test ===\n");

        let (_copilot_token, discovered_endpoint) = do_token_exchange(&oauth_token, &client).await;
        println!("Main Endpoint endpoint:   {GITHUBCOPILOT_ENDPOINT}");
        println!("Discovered endpoint:  {discovered_endpoint}\n");

        for (label, base_url) in [
            ("Main Endpoint", GITHUBCOPILOT_ENDPOINT),
            ("discovered", discovered_endpoint.as_str()),
        ] {
            println!("--- Models @ {label} endpoint ({base_url}) ---");
            let (status, body_str) = try_models(&client, base_url, &oauth_token).await;
            println!("Status: {status}");

            if status.is_success() {
                let models =
                    serde_json::from_str::<ModelSchema>(&body_str).expect("failed to parse models");
                println!("Parsed {} models", models.data.len());
                for model in &models.data {
                    println!("  - {} ({})", model.name, model.id);
                }
            } else {
                println!("Body: {body_str}");
            }
            println!();
        }
    });
}

/// Integration test: chat completions with OAuth token at both endpoints.
///
/// Run with:
///   GH_COPILOT_TOKEN=<token> cargo test -p copilot_chat -- --ignored test_copilot_completion --nocapture
#[ignore]
#[test]
fn test_copilot_completion() {
    let oauth_token = get_test_oauth_token();
    let client = create_test_client();

    smol::block_on(async {
        println!("=== Copilot Completion Test ===\n");

        let (_copilot_token, discovered_endpoint) = do_token_exchange(&oauth_token, &client).await;
        println!("Main Endpoint endpoint:   {GITHUBCOPILOT_ENDPOINT}");
        println!("Discovered endpoint:  {discovered_endpoint}\n");

        let mut any_succeeded = false;
        for (label, base_url) in [
            ("Main Endpoint", GITHUBCOPILOT_ENDPOINT),
            ("discovered", discovered_endpoint.as_str()),
        ] {
            println!("--- Completion (OAuth) @ {label} endpoint ({base_url}) ---");
            let (status, body_str) = try_completion(&client, base_url, &oauth_token, None).await;
            println!("Status: {status}");
            println!("Body: {body_str}\n");
            any_succeeded |= status.is_success();
        }

        assert!(
            any_succeeded,
            "Completion with OAuth token failed at both endpoints"
        );
    });
}

/// Diagnostic test that tries every combination of auth approach and endpoint
/// for completions, and prints a summary table.
///
/// Run with:
///   GH_COPILOT_TOKEN=<token> cargo test -p copilot_chat -- --ignored test_copilot_completion_auth_diagnostic --nocapture
#[ignore]
#[test]
fn test_copilot_completion_auth_diagnostic() {
    let oauth_token = get_test_oauth_token();
    let client = create_test_client();

    smol::block_on(async {
        println!("=== Copilot Completion Auth Diagnostic ===\n");

        let (copilot_token, discovered_endpoint) = do_token_exchange(&oauth_token, &client).await;
        println!("Main Endpoint endpoint:   {GITHUBCOPILOT_ENDPOINT}");
        println!("Discovered endpoint:  {discovered_endpoint}\n");

        struct Probe {
            label: &'static str,
            endpoint_label: &'static str,
            status: http_client::StatusCode,
        }

        let mut probes: Vec<Probe> = Vec::new();

        let endpoints: &[(&str, &str)] = &[
            ("Main Endpoint", GITHUBCOPILOT_ENDPOINT),
            ("discovered", discovered_endpoint.as_str()),
        ];

        let auth_approaches: &[(&str, &str, Option<&str>)] = &[
            ("OAuth token", oauth_token.as_str(), None),
            (
                "Copilot token (no integration-id)",
                copilot_token.as_str(),
                None,
            ),
            (
                "Copilot token + integration-id",
                copilot_token.as_str(),
                Some("vscode-chat"),
            ),
        ];

        for &(endpoint_label, base_url) in endpoints {
            for &(auth_label, token, integration_id) in auth_approaches {
                let full_label = format!("{auth_label} @ {endpoint_label}");
                println!("--- {full_label} ---");

                let (status, body) = try_completion(&client, base_url, token, integration_id).await;
                println!("Status: {status}");
                println!("Body: {body}\n");

                probes.push(Probe {
                    label: auth_label,
                    endpoint_label,
                    status,
                });
            }
        }

        // Summary table
        println!("=== Summary ===");
        println!("{:<45} {:<12} {}", "Approach", "Endpoint", "Status");
        println!("{}", "-".repeat(75));
        let mut any_succeeded = false;
        for probe in &probes {
            println!(
                "{:<45} {:<12} {}",
                probe.label, probe.endpoint_label, probe.status
            );
            any_succeeded |= probe.status.is_success();
        }
        println!();

        assert!(
            any_succeeded,
            "No auth/endpoint combination succeeded for completions"
        );
    });
}
