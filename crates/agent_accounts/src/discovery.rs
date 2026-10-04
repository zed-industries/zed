//! Finds the Claude Code and Codex homes on this machine.
//!
//! Ported from Superset's profile discovery: candidates are dot-dirs in the
//! home directory (plus `~/.config/*` for Claude), never project trees. A
//! Claude home counts when its own `.claude.json` names an OAuth account; a
//! Codex home counts when it is a `~/.codex*` dir holding an `auth.json`.
//! Only identity files are opened, never credentials.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::{AccountProvider, AgentAccount, tilde_label};

const SCAN_TIME_BUDGET: Duration = Duration::from_millis(1500);
const MAX_STATE_FILE_BYTES: u64 = 50 * 1024 * 1024;

/// Discovers accounts for the current user. Blocking: call it off the main thread.
pub fn discover_accounts() -> Vec<AgentAccount> {
    let home_dir = util::paths::home_dir();
    let ambient_claude = std::env::var_os("CLAUDE_CONFIG_DIR").map(PathBuf::from);
    let ambient_codex = std::env::var_os("CODEX_HOME").map(PathBuf::from);
    let ambient_grok = std::env::var_os("GROK_HOME").map(PathBuf::from);
    discover_accounts_in(
        home_dir,
        ambient_claude.as_deref(),
        ambient_codex.as_deref(),
        ambient_grok.as_deref(),
    )
}

/// Discovers accounts under `home_dir`. The ambient homes are the ones the
/// CLIs use when Zed sets no variable.
pub fn discover_accounts_in(
    home_dir: &Path,
    ambient_claude_dir: Option<&Path>,
    ambient_codex_home: Option<&Path>,
    ambient_grok_home: Option<&Path>,
) -> Vec<AgentAccount> {
    let started = Instant::now();
    let mut accounts = Vec::new();

    let default_claude = ambient_claude_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home_dir.join(".claude"));
    // Only the default `~/.claude` keeps its state file next door.
    let default_claude_state = if ambient_claude_dir.is_some() {
        default_claude.join(".claude.json")
    } else {
        home_dir.join(".claude.json")
    };
    let default_claude_email = read_claude_identity(&default_claude_state).flatten();
    accounts.push(AgentAccount {
        provider: AccountProvider::Claude,
        home_label: tilde_label(&default_claude, home_dir),
        email: default_claude_email.clone(),
        name: None,
        home: default_claude.clone(),
        is_default: true,
    });

    let default_codex = ambient_codex_home
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home_dir.join(".codex"));
    accounts.push(AgentAccount {
        provider: AccountProvider::Codex,
        home_label: tilde_label(&default_codex, home_dir),
        email: read_codex_email(&default_codex),
        name: None,
        home: default_codex.clone(),
        is_default: true,
    });

    let default_grok = ambient_grok_home
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home_dir.join(".grok"));
    accounts.push(AgentAccount {
        provider: AccountProvider::Grok,
        home_label: tilde_label(&default_grok, home_dir),
        email: read_grok_identity(&default_grok).flatten(),
        name: None,
        home: default_grok.clone(),
        is_default: true,
    });
    accounts.push(AgentAccount {
        provider: AccountProvider::Cursor,
        home_label: "~".into(),
        email: read_cursor_email(&home_dir.join(".cursor")),
        name: None,
        home: home_dir.to_path_buf(),
        is_default: true,
    });

    let excluded_claude = [
        default_claude,
        home_dir.join(".claude"),
        home_dir.join(".config").join("claude"),
    ];
    let dot_dirs = subdirectories(home_dir)
        .into_iter()
        .filter(|path| is_dot_dir(path));
    let config_dirs = subdirectories(&home_dir.join(".config"));
    let mut candidates: Vec<PathBuf> = dot_dirs.chain(config_dirs).collect();
    candidates.sort();

    for candidate in candidates {
        if started.elapsed() > SCAN_TIME_BUDGET {
            log::warn!("agent account discovery stopped after its time budget");
            break;
        }
        if candidate == default_codex || excluded_claude.contains(&candidate) {
            continue;
        }

        if let Some(email) = read_claude_identity(&candidate.join(".claude.json")) {
            // Claude Code copies the main profile's identity into a new
            // config dir before anyone signs in there.
            let email = email.filter(|email| Some(email) != default_claude_email.as_ref());
            accounts.push(AgentAccount {
                provider: AccountProvider::Claude,
                home_label: tilde_label(&candidate, home_dir),
                email,
                name: None,
                home: candidate,
                is_default: false,
            });
            continue;
        }

        if candidate.parent() != Some(home_dir) || candidate == default_grok {
            continue;
        }
        let name = candidate
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let found = if name.starts_with(".codex") && candidate.join("auth.json").is_file() {
            Some((AccountProvider::Codex, read_codex_email(&candidate)))
        } else if name.starts_with(".grok") {
            read_grok_identity(&candidate).map(|email| (AccountProvider::Grok, email))
        } else if name.starts_with(".cursor-") && candidate.join(".cursor/auth.json").is_file() {
            Some((
                AccountProvider::Cursor,
                read_cursor_email(&candidate.join(".cursor")),
            ))
        } else {
            None
        };
        if let Some((provider, email)) = found {
            accounts.push(AgentAccount {
                provider,
                home_label: tilde_label(&candidate, home_dir),
                email,
                name: None,
                home: candidate,
                is_default: false,
            });
        }
    }

    accounts
}

/// `Some(email)` when the Grok home holds a login (an `auth.json` entry
/// with a key).
fn read_grok_identity(home: &Path) -> Option<Option<String>> {
    let contents = fs::read(home.join("auth.json")).ok()?;
    let auth: serde_json::Value = serde_json::from_slice(&contents).ok()?;
    let entry = auth.as_object()?.values().find(|entry| {
        entry
            .get("key")
            .and_then(|key| key.as_str())
            .is_some_and(|key| !key.is_empty())
    })?;
    Some(
        entry
            .get("email")
            .and_then(|email| email.as_str())
            .map(str::to_string),
    )
}

/// The email in a Codex login's ID token: the token's claims, not its
/// signature or access token, are read.
fn read_codex_email(home: &Path) -> Option<String> {
    use base64::Engine as _;
    let contents = fs::read(home.join("auth.json")).ok()?;
    let auth: serde_json::Value = serde_json::from_slice(&contents).ok()?;
    let id_token = auth.pointer("/tokens/id_token")?.as_str()?;
    let payload = id_token.split('.').nth(1)?;
    let claims = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload.trim_end_matches('='))
        .ok()?;
    let claims: serde_json::Value = serde_json::from_slice(&claims).ok()?;
    claims
        .get("email")
        .and_then(|email| email.as_str())
        .map(str::to_string)
}

/// The email Cursor's CLI recorded for its login, if any.
fn read_cursor_email(config_dir: &Path) -> Option<String> {
    let contents = fs::read(config_dir.join("cli-config.json")).ok()?;
    let config: serde_json::Value = serde_json::from_slice(&contents).ok()?;
    config
        .pointer("/authInfo/email")
        .and_then(|email| email.as_str())
        .map(str::to_string)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeStateFile {
    oauth_account: Option<ClaudeOAuthAccount>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaudeOAuthAccount {
    email_address: Option<String>,
    account_uuid: Option<String>,
}

/// `Some(email)` when the state file names an OAuth account (the email may
/// itself be missing), `None` when it does not.
fn read_claude_identity(state_path: &Path) -> Option<Option<String>> {
    let metadata = fs::metadata(state_path).ok()?;
    if !metadata.is_file() || metadata.len() > MAX_STATE_FILE_BYTES {
        return None;
    }
    let contents = fs::read(state_path).ok()?;
    let state: ClaudeStateFile = serde_json::from_slice(&contents).ok()?;
    let account = state.oauth_account?;
    if account.account_uuid.is_none() && account.email_address.is_none() {
        return None;
    }
    Some(account.email_address)
}

fn subdirectories(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.path())
        .collect()
}

fn is_dot_dir(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn summary(
        accounts: &[AgentAccount],
        home: &Path,
    ) -> Vec<(AccountProvider, String, Option<String>, bool)> {
        accounts
            .iter()
            .map(|account| {
                (
                    account.provider,
                    tilde_label(&account.home, home),
                    account.email.clone(),
                    account.is_default,
                )
            })
            .collect()
    }

    #[test]
    fn finds_claude_and_codex_homes() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path();
        write(
            &home.join(".claude.json"),
            r#"{"oauthAccount":{"emailAddress":"me@personal.dev","accountUuid":"a"}}"#,
        );
        fs::create_dir_all(home.join(".claude")).unwrap();
        write(
            &home.join(".claude-work/.claude.json"),
            r#"{"oauthAccount":{"emailAddress":"me@work.dev","accountUuid":"b"}}"#,
        );
        write(
            &home.join(".config/claude-uuid-only/.claude.json"),
            r#"{"oauthAccount":{"accountUuid":"c"}}"#,
        );
        // A fork or sandbox home without a login is ignored.
        write(
            &home.join(".claude-sandbox/.claude.json"),
            r#"{"projects":{}}"#,
        );
        // A new profile Claude Code seeded with the main identity.
        write(
            &home.join(".claude-fresh/.claude.json"),
            r#"{"oauthAccount":{"emailAddress":"me@personal.dev","accountUuid":"a"}}"#,
        );
        // {"email":"c2@openai.dev"} as an unsigned JWT payload.
        write(
            &home.join(".codex-2/auth.json"),
            r#"{"tokens":{"id_token":"e30.eyJlbWFpbCI6ImMyQG9wZW5haS5kZXYifQ.sig"}}"#,
        );
        // Only `~/.codex*` dirs are Codex homes.
        write(&home.join(".other/auth.json"), "{}");
        fs::create_dir_all(home.join(".codex-empty")).unwrap();

        write(
            &home.join(".grok-2/auth.json"),
            r#"{"https://accounts.x.ai/sign-in":{"key":"k","email":"g@x.ai"}}"#,
        );
        write(&home.join(".grok-empty/auth.json"), r#"{"x":{"key":""}}"#);
        write(&home.join(".cursor-work/.cursor/auth.json"), "{}");
        write(
            &home.join(".cursor-work/.cursor/cli-config.json"),
            r#"{"authInfo":{"email":"c@work.dev"}}"#,
        );
        fs::create_dir_all(home.join(".cursor-server")).unwrap();

        let accounts = discover_accounts_in(home, None, None, None);
        assert_eq!(
            summary(&accounts, home),
            vec![
                (
                    AccountProvider::Claude,
                    "~/.claude".into(),
                    Some("me@personal.dev".into()),
                    true
                ),
                (AccountProvider::Codex, "~/.codex".into(), None, true),
                (AccountProvider::Grok, "~/.grok".into(), None, true),
                (AccountProvider::Cursor, "~/".into(), None, true),
                (
                    AccountProvider::Claude,
                    "~/.claude-fresh".into(),
                    None,
                    false
                ),
                (
                    AccountProvider::Claude,
                    "~/.claude-work".into(),
                    Some("me@work.dev".into()),
                    false
                ),
                (
                    AccountProvider::Codex,
                    "~/.codex-2".into(),
                    Some("c2@openai.dev".into()),
                    false
                ),
                (
                    AccountProvider::Claude,
                    "~/.config/claude-uuid-only".into(),
                    None,
                    false
                ),
                (
                    AccountProvider::Cursor,
                    "~/.cursor-work".into(),
                    Some("c@work.dev".into()),
                    false
                ),
                (
                    AccountProvider::Grok,
                    "~/.grok-2".into(),
                    Some("g@x.ai".into()),
                    false
                ),
            ]
        );
        assert_eq!(accounts[0].id(), None);
        assert_eq!(
            accounts[5].id().map(|id| id.home().to_path_buf()),
            Some(home.join(".claude-work"))
        );
    }

    #[test]
    fn ambient_homes_become_the_defaults() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path();
        write(
            &home.join(".claude-main/.claude.json"),
            r#"{"oauthAccount":{"emailAddress":"main@dev","accountUuid":"a"}}"#,
        );
        write(&home.join(".codex-main/auth.json"), "{}");

        let accounts = discover_accounts_in(
            home,
            Some(&home.join(".claude-main")),
            Some(&home.join(".codex-main")),
            None,
        );
        assert_eq!(
            summary(&accounts, home),
            vec![
                (
                    AccountProvider::Claude,
                    "~/.claude-main".into(),
                    Some("main@dev".into()),
                    true
                ),
                (AccountProvider::Codex, "~/.codex-main".into(), None, true),
                (AccountProvider::Grok, "~/.grok".into(), None, true),
                (AccountProvider::Cursor, "~/".into(), None, true),
            ]
        );
    }
}
