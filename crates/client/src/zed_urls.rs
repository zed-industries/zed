//! Contains helper functions for constructing URLs to various Zed-related pages.
//!
//! These URLs will adapt to the configured server URL in order to construct
//! links appropriate for the environment (e.g., by linking to a local copy of
//! zed.dev in development).

use gpui::App;
use settings::Settings;

use crate::ClientSettings;

fn server_url(cx: &App) -> &str {
    &ClientSettings::get_global(cx).server_url
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
        "{server_url}/docs/ai/privacy-and-security",
        server_url = server_url(cx)
    )
}

/// Returns the URL to Zed AI's external agents documentation.
pub fn external_agents_docs(cx: &App) -> String {
    format!(
        "{server_url}/docs/ai/external-agents",
        server_url = server_url(cx)
    )
}

/// Returns the URL to Zed agent servers documentation.
pub fn agent_server_docs(cx: &App) -> String {
    format!(
        "{server_url}/docs/extensions/agent-servers",
        server_url = server_url(cx)
    )
}
