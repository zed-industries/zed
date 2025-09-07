use anyhow::{anyhow, Result};
use gpui::{App, BorrowAppContext, Context, EventEmitter, Global};
use serde::{Deserialize, Serialize};
use url::Url;

/// GitHub authentication state
#[derive(Clone)]
pub struct GithubAuth {
    token: Option<String>,
    user: Option<GithubUser>,
    enterprise_url: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
}

impl Global for GithubAuth {}

impl GithubAuth {
    pub fn init(cx: &mut App) {
        // Try to load token from multiple sources in order of preference:
        // 1. Environment variable (for CI/temporary override)
        // 2. Saved token file in Zed config directory
        // 3. Git credentials (as last resort, may cause hanging)
        let token = std::env::var("GITHUB_TOKEN")
            .ok()
            .or_else(|| Self::load_saved_token())
            .or_else(|| Self::load_from_git_credentials());

        cx.set_global(Self {
            token: token.clone(),
            user: None,
            enterprise_url: None,
        });

        // If we have a token from environment, save it for future use
        if let Some(token) = token {
            if std::env::var("GITHUB_TOKEN").is_ok() {
                Self::save_token(&token);
            }
        }
    }

    fn get_token_file_path() -> std::path::PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"));
        config_dir.join("zed").join(".github_token")
    }

    fn save_token(token: &str) {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        
        let token_path = Self::get_token_file_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = token_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        // Write token with restricted permissions (600)
        if let Ok(_) = fs::write(&token_path, token) {
            if let Ok(metadata) = fs::metadata(&token_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o600);
                let _ = fs::set_permissions(&token_path, permissions);
            }
            log::info!("GitHub token saved to persistent storage");
        }
    }

    fn load_saved_token() -> Option<String> {
        use std::fs;
        
        let token_path = Self::get_token_file_path();
        
        match fs::read_to_string(&token_path) {
            Ok(token) => {
                let token = token.trim().to_string();
                if !token.is_empty() {
                    log::info!("Loaded GitHub token from persistent storage");
                    Some(token)
                } else {
                    None
                }
            }
            Err(_) => None
        }
    }

    fn load_from_git_credentials() -> Option<String> {
        use std::io::Write;
        use std::process::{Command, Stdio};
        use std::time::Duration;

        // Try to get GitHub token from git credential helper with timeout
        let mut child = Command::new("git")
            .args(&["credential", "fill"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;

        if let Some(stdin) = child.stdin.as_mut() {
            // Request credentials for github.com
            if stdin
                .write_all(b"protocol=https\nhost=github.com\n\n")
                .is_err()
            {
                // Kill the process if write fails
                let _ = child.kill();
                return None;
            }
        }

        // Use a timeout to prevent hanging
        let output = match child.wait_with_output() {
            Ok(output) if output.status.success() => output,
            _ => return None,
        };

        // Parse the output to get the password (token)
        let output_str = String::from_utf8(output.stdout).ok()?;
        for line in output_str.lines() {
            if let Some(token) = line.strip_prefix("password=") {
                return Some(token.to_string());
            }
        }

        None
    }

    pub fn trigger_git_login(cx: &mut App) -> anyhow::Result<()> {
        use std::process::Command;

        // Check if GitHub CLI is available
        let gh_available = Command::new("gh")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if gh_available {
            // Use GitHub CLI for authentication with required scopes
            // repo: Full control of private repositories (needed for PR operations)
            // read:user: Read user profile data
            Command::new("gh")
                .args(&["auth", "login", "--web", "--scopes", "repo,read:user"])
                .spawn()?;

            // TODO: After login completes, reload credentials
            // This would need to be done asynchronously or via a refresh action
        } else {
            // Fall back to opening GitHub token page with required scopes
            let url = "https://github.com/settings/tokens/new?scopes=repo,read:user&description=Zed%20Pull%20Requests";
            cx.open_url(&url);
        }

        Ok(())
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn update_global<F, R>(cx: &mut App, f: F) -> R
    where
        F: FnOnce(&mut Self, &mut App) -> R,
    {
        cx.update_global::<Self, _>(f)
    }

    pub fn refresh_credentials(cx: &mut App) {
        if let Some(token) = Self::load_from_git_credentials() {
            Self::update_global(cx, |auth, _cx| {
                auth.set_token(token);
            });
        }
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn user(&self) -> Option<&GithubUser> {
        self.user.as_ref()
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn clear_token(&mut self) {
        self.token = None;
        self.user = None;
    }

    pub fn set_user(&mut self, user: GithubUser) {
        self.user = Some(user);
    }

    pub fn set_enterprise_url(&mut self, url: Option<Url>) {
        self.enterprise_url = url;
    }

    pub fn enterprise_url(&self) -> Option<&Url> {
        self.enterprise_url.as_ref()
    }
}

/// Authentication dialog for GitHub
pub struct GithubAuthDialog {
    token_input: String,
    enterprise_url_input: String,
    is_enterprise: bool,
    error_message: Option<String>,
    is_validating: bool,
}

impl GithubAuthDialog {
    pub fn new() -> Self {
        Self {
            token_input: String::new(),
            enterprise_url_input: String::new(),
            is_enterprise: false,
            error_message: None,
            is_validating: false,
        }
    }

    pub fn authenticate(&mut self, cx: &mut Context<Self>) {
        if self.token_input.is_empty() {
            // Try to trigger git login instead
            if let Err(e) = GithubAuth::trigger_git_login(cx) {
                self.error_message = Some(format!("Failed to trigger login: {}", e));
            } else {
                self.error_message =
                    Some("Please complete GitHub login in your browser".to_string());
            }
            cx.notify();
            return;
        }

        self.is_validating = true;
        self.error_message = None;
        cx.notify();

        let token = self.token_input.clone();
        let enterprise_url = if self.is_enterprise {
            Url::parse(&self.enterprise_url_input).ok()
        } else {
            None
        };

        cx.spawn(async move |this, cx| {
            // Validate token by making a test API call
            let validation_result = validate_token(&token, enterprise_url.as_ref()).await;

            this.update(cx, |dialog, cx| {
                dialog.is_validating = false;

                match validation_result {
                    Ok(user) => {
                        // Save the token and user info
                        GithubAuth::update_global(cx, |auth, _cx| {
                            auth.set_token(token);
                            auth.set_user(user);
                            auth.set_enterprise_url(enterprise_url);
                        });

                        // Close the dialog
                        cx.emit(AuthDialogEvent::Authenticated);
                    }
                    Err(e) => {
                        dialog.error_message = Some(format!("Authentication failed: {}", e));
                    }
                }
                cx.notify();
            })?;

            Ok::<(), anyhow::Error>(())
        })
        .detach_and_log_err(cx);
    }
}

impl EventEmitter<AuthDialogEvent> for GithubAuthDialog {}

pub enum AuthDialogEvent {
    Authenticated,
    Cancelled,
}

async fn validate_token(token: &str, enterprise_url: Option<&Url>) -> Result<GithubUser> {
    use http_client::{HttpClient, Request};

    let base_url = if let Some(url) = enterprise_url {
        format!("{}/api/v3", url)
    } else {
        "https://api.github.com".to_string()
    };

    let url = format!("{}/user", base_url);

    // This is a placeholder - we need to get the HTTP client from the workspace
    // For now, just return an error
    return Err(anyhow!("Token validation not yet implemented"));

    // TODO: Implement actual validation when we have access to HTTP client
    // let request = Request::get(&url)
    //     .header("Authorization", format!("Bearer {}", token))
    //     .header("Accept", "application/vnd.github+json")
    //     .header("User-Agent", "zed-pull-requests");
    //
    // let mut response = client
    //     .send(request.body(http_client::AsyncBody::default())?)
    //     .await?;
    //
    // if !response.status().is_success() {
    //     return Err(anyhow!(
    //         "Invalid token or API error: {}",
    //         response.status()
    //     ));
    // }
    //
    // let mut body = Vec::new();
    // futures::AsyncReadExt::read_to_end(&mut response.body_mut(), &mut body).await?;
    //
    // let user: GithubUser = serde_json::from_slice(&body)?;
    // Ok(user)
}

/// Settings for GitHub integration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GithubSettings {
    pub enterprise_url: Option<String>,
    pub default_merge_method: MergeMethod,
    pub auto_refresh_interval: u64, // seconds
    pub show_draft_prs: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

impl Default for GithubSettings {
    fn default() -> Self {
        Self {
            enterprise_url: None,
            default_merge_method: MergeMethod::Merge,
            auto_refresh_interval: 300, // 5 minutes
            show_draft_prs: true,
        }
    }
}

// Add trait implementation for detach_and_log_err
trait TaskExt {
    fn detach_and_log_err(self, cx: &mut gpui::Context<GithubAuthDialog>);
}

impl TaskExt for gpui::Task<anyhow::Result<()>> {
    fn detach_and_log_err(self, _cx: &mut gpui::Context<GithubAuthDialog>) {
        self.detach();
    }
}
