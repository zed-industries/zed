//! Contains helper functions for constructing URLs to various Zed-related pages.
//!
//! These URLs will adapt to the configured server URL in order to construct
//! links appropriate for the environment (e.g., by linking to a local copy of
//! zed.dev in development).

use gpui::App;
use release_channel::ReleaseChannel;
use settings::Settings;

use crate::ClientSettings;

fn server_url(cx: &App) -> &str {
    &ClientSettings::get_global(cx).server_url
}

fn docs_url(cx: &App) -> String {
    let server_url = server_url(cx);
    match ReleaseChannel::try_global(cx).unwrap_or_default() {
        ReleaseChannel::Stable => {
            format!("{server_url}/docs")
        }
        ReleaseChannel::Preview => {
            format!("{server_url}/docs/preview")
        }
        ReleaseChannel::Dev | ReleaseChannel::Nightly => {
            format!("{server_url}/docs/nightly")
        }
    }
}

/// Returns the URL to the account page on zed.dev.
pub fn account_url(cx: &App) -> String {
    format!("{server_url}/account", server_url = server_url(cx))
}

/// Returns the URL to the start trial page on zed.dev.
pub fn start_trial_url(cx: &App) -> String {
    format!(
        "{server_url}/account/start-trial",
        server_url = server_url(cx)
    )
}

/// Returns the URL to the upgrade page on zed.dev.
pub fn upgrade_to_zed_pro_url(cx: &App) -> String {
    format!("{server_url}/account/upgrade", server_url = server_url(cx))
}

/// Returns the URL to Zed's terms of service.
pub fn terms_of_service(cx: &App) -> String {
    format!("{server_url}/terms-of-service", server_url = server_url(cx))
}

/// Returns the URL to Zed AI's privacy and security docs.
pub fn ai_privacy_and_security(cx: &App) -> String {
    format!(
        "{docs_url}/ai/privacy-and-security",
        docs_url = docs_url(cx)
    )
}

/// Returns the URL to Zed's edit prediction documentation.
pub fn edit_prediction_docs(cx: &App) -> String {
    format!("{docs_url}/ai/edit-prediction", docs_url = docs_url(cx))
}

pub fn skills_docs(cx: &App) -> String {
    format!("{docs_url}/ai/skills", docs_url = docs_url(cx))
}

/// Returns the URL to Zed's Agent sandboxing documentation.
///
/// Pass `section` to deep-link to a specific section anchor on the page (for
/// example, `Some("installing-bubblewrap")`); pass `None` to link to the top of
/// the page.
///
/// Unlike the account/app links above, this targets `zed.dev/docs` (via
/// [`release_channel::docs_url`]) rather than the configured `server_url`: the
/// docs are a static site hosted on `zed.dev`, so pointing at a local dev
/// `server_url` would 404.
pub fn sandboxing_docs(section: Option<&str>, cx: &App) -> String {
    let base = release_channel::docs_url("ai/sandboxing", cx);
    match section {
        Some(section) => format!("{base}#{section}"),
        None => base,
    }
}
pub fn llm_provider_docs(cx: &App) -> String {
    format!("{docs_url}/ai/llm-providers", docs_url = docs_url(cx))
}

/// Returns the URL to Zed's ACP registry blog post.
pub fn acp_registry_blog(cx: &App) -> String {
    format!(
        "{server_url}/blog/acp-registry",
        server_url = server_url(cx)
    )
}

pub fn shared_agent_thread_url(session_id: &str) -> String {
    format!("zed://agent/shared/{}", session_id)
}
