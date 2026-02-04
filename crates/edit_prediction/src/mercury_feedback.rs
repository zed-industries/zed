use gpui::http_client::{self, AsyncBody, HttpClient, Method};
use semver::Version;
use std::sync::Arc;

const FEEDBACK_API_URL: &str = "https://api-feedback.inceptionlabs.ai/feedback";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MercuryUserAction {
    Accept,
    Reject,
    Ignore,
}

impl MercuryUserAction {
    fn as_str(&self) -> &'static str {
        match self {
            MercuryUserAction::Accept => "accept",
            MercuryUserAction::Reject => "reject",
            MercuryUserAction::Ignore => "ignore",
        }
    }
}

/// Sends feedback to the Mercury API.
/// This function spawns a background task and returns immediately.
/// The request_id must start with "cmpl-" (as returned by the Inception API).
pub fn send_mercury_feedback(
    request_id: String,
    action: MercuryUserAction,
    app_version: Version,
    http_client: Arc<dyn HttpClient>,
) {
    if !request_id.starts_with("cmpl-") {
        log::warn!(
            "Mercury feedback: invalid request_id '{}' - must start with 'cmpl-'",
            request_id
        );
        return;
    }

    std::thread::spawn(move || {
        if let Err(e) = send_feedback_blocking(&request_id, action, &app_version, &http_client) {
            log::error!("Failed to send Mercury feedback: {}", e);
        }
    });
}

fn send_feedback_blocking(
    request_id: &str,
    action: MercuryUserAction,
    app_version: &Version,
    http_client: &Arc<dyn HttpClient>,
) -> anyhow::Result<()> {
    let body = serde_json::json!({
        "request_id": request_id,
        "provider_name": "zed",
        "user_action": action.as_str(),
        "provider_version": app_version.to_string()
    });

    let request = http_client::Request::builder()
        .uri(FEEDBACK_API_URL)
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(AsyncBody::from(serde_json::to_vec(&body)?))?;

    futures::executor::block_on(async {
        let response = http_client.send(request).await?;
        if !response.status().is_success() {
            anyhow::bail!("Feedback API returned status: {}", response.status());
        }
        anyhow::Ok(())
    })?;

    log::debug!(
        "Mercury feedback sent: request_id={}, action={:?}",
        request_id,
        action
    );

    Ok(())
}
