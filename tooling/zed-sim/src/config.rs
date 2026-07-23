//! Runtime configuration for the optional impersonation states.
//!
//! Read from a local JSON file (never committed) plus environment overrides, so
//! no secret ever lands in the repo. Impersonation is offered only when a token,
//! a server URL, and at least one account all resolve.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use serde::Deserialize;

/// The on-disk config file shape. All fields are optional so a partial or
/// missing file degrades gracefully (impersonation simply stays unavailable).
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ConfigFile {
    /// Backend the impersonated session points at (e.g. a preview URL). Zed
    /// derives the cloud API host from this value.
    server_url: Option<String>,
    /// Internal API token. Prefer the `ZED_SIM_IMPERSONATE_TOKEN` env var; this
    /// field is a convenience and the file must stay local (it is gitignored).
    token: Option<String>,
    /// The allow-list of GitHub usernames offered in the UI.
    accounts: Vec<Account>,
}

/// A single impersonatable account.
#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    pub username: String,
    #[serde(default)]
    pub label: Option<String>,
}

/// Fully resolved configuration after applying environment overrides.
pub struct AppConfig {
    pub server_url: Option<String>,
    pub token: Option<String>,
    pub accounts: Vec<Account>,
}

impl AppConfig {
    /// Whether impersonation can actually be performed right now.
    pub fn impersonation_enabled(&self) -> bool {
        self.server_url.is_some() && self.token.is_some() && !self.accounts.is_empty()
    }

    /// Returns the account matching `username`, but only if it is in the
    /// configured allow-list. Guards against arbitrary usernames being posted.
    pub fn find_account(&self, username: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|account| account.username == username)
    }
}

/// Loads config from `path` (if present) and applies environment overrides. A
/// missing file is not an error — impersonation is simply unavailable.
pub fn load(path: &Path) -> Result<AppConfig> {
    let file = if path.exists() {
        let body = std::fs::read_to_string(path)
            .with_context(|| format!("reading config at {}", path.display()))?;
        serde_json::from_str::<ConfigFile>(&body)
            .with_context(|| format!("parsing config at {}", path.display()))?
    } else {
        ConfigFile::default()
    };

    let token = env_override("ZED_SIM_IMPERSONATE_TOKEN").or(file.token);
    let server_url = env_override("ZED_SIM_SERVER_URL").or(file.server_url);

    Ok(AppConfig {
        server_url,
        token,
        accounts: file.accounts,
    })
}

/// The default config path, relative to the current directory (the workspace
/// root when run via `cargo run -p zed-sim`).
pub fn default_path() -> PathBuf {
    PathBuf::from("tooling/zed-sim/zed-sim.config.json")
}

fn env_override(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|value| !value.is_empty())
}
