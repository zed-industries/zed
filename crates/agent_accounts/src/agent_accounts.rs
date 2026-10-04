//! Local login profiles ("accounts") for external ACP agents, and moving a
//! conversation between agents or accounts.
//!
//! An account is a provider home directory: `CLAUDE_CONFIG_DIR` for Claude
//! Code, `CODEX_HOME` for Codex. Starting an agent with an account means
//! starting its process with that variable set; the default account means
//! leaving the variable alone. Credentials stay where the CLI put them and
//! are never read here.

mod discovery;
pub mod handoff;
pub mod quota;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use discovery::{discover_accounts, discover_accounts_in};

/// The ACP registry id of Claude Code.
pub const CLAUDE_AGENT_ID: &str = "claude-acp";
/// The ACP registry id of Codex.
pub const CODEX_AGENT_ID: &str = "codex-acp";
/// The ACP registry id of Cursor.
pub const CURSOR_AGENT_ID: &str = "cursor";
/// The canonical id for Grok accounts; Grok agents have user-chosen ids.
pub const GROK_AGENT_ID: &str = "grok";

/// Identifies a non-default account: the absolute path of its home directory.
///
/// The path itself is the identity so that a persisted thread can rebuild the
/// agent's environment without looking the account up again.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct AccountId(pub Arc<str>);

impl AccountId {
    pub fn new(home: &Path) -> Self {
        Self(home.to_string_lossy().into())
    }

    pub fn home(&self) -> &Path {
        Path::new(self.0.as_ref())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Picks the CLI's own home explicitly, as opposed to leaving the
    /// choice to the configured default account.
    pub fn system() -> Self {
        Self(SYSTEM_ACCOUNT.into())
    }

    pub fn is_system(&self) -> bool {
        self.as_str() == SYSTEM_ACCOUNT
    }
}

const SYSTEM_ACCOUNT: &str = "system";

impl std::fmt::Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for AccountId {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountProvider {
    Claude,
    Codex,
    Grok,
    Cursor,
}

impl AccountProvider {
    pub const ALL: [Self; 4] = [Self::Claude, Self::Codex, Self::Grok, Self::Cursor];

    /// The provider whose accounts apply to the given agent, if any. Grok
    /// runs as a custom agent with a user-chosen id, so any id naming Grok
    /// counts.
    pub fn for_agent(agent_id: &str) -> Option<Self> {
        match agent_id {
            CLAUDE_AGENT_ID => Some(Self::Claude),
            CODEX_AGENT_ID => Some(Self::Codex),
            CURSOR_AGENT_ID => Some(Self::Cursor),
            id if id.to_lowercase().contains("grok") => Some(Self::Grok),
            _ => None,
        }
    }

    /// The agent id to record for accounts not tied to a running agent,
    /// such as a terminal's. [`Self::for_agent`] maps it back.
    pub fn agent_id(self) -> &'static str {
        match self {
            Self::Claude => CLAUDE_AGENT_ID,
            Self::Codex => CODEX_AGENT_ID,
            Self::Grok => GROK_AGENT_ID,
            Self::Cursor => CURSOR_AGENT_ID,
        }
    }

    /// The environment that points the CLI at an account's home.
    ///
    /// Cursor keeps its login in the macOS Keychain under fixed names, so a
    /// second account needs the file credential store, which lives under
    /// `$HOME/.cursor`: its home is a directory standing in for `HOME`.
    pub fn account_env(self, home: &str) -> Vec<(String, String)> {
        let var = |key: &str, value: String| (key.to_string(), value);
        match self {
            Self::Claude => vec![var("CLAUDE_CONFIG_DIR", home.into())],
            Self::Codex => vec![var("CODEX_HOME", home.into())],
            Self::Grok => vec![var("GROK_HOME", home.into())],
            Self::Cursor => vec![
                var("HOME", home.into()),
                var("CURSOR_CONFIG_DIR", format!("{home}/.cursor")),
                var("AGENT_CLI_CREDENTIAL_STORE", "file".into()),
            ],
        }
    }

    /// The txcript harness name for this provider's sessions.
    pub fn harness(self) -> &'static str {
        match self {
            Self::Claude => "claude_code",
            Self::Codex => "codex",
            Self::Grok => "grok",
            Self::Cursor => "cursor",
        }
    }

    /// Whether a session txcript writes for this provider loads through the
    /// agent's ACP `session/load` with its history. Cursor's ACP server only
    /// reads its own `acp-sessions` store, and even a converted session
    /// placed there replays without reaching the model, so Cursor gets the
    /// transcript as a first message instead.
    pub fn supports_native_handoff(self) -> bool {
        matches!(self, Self::Claude | Self::Codex | Self::Grok)
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Claude => "Claude Code",
            Self::Codex => "Codex",
            Self::Grok => "Grok",
            Self::Cursor => "Cursor",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentAccount {
    pub provider: AccountProvider,
    pub home: PathBuf,
    /// The home as the user would write it (`~/.claude-work`).
    pub home_label: String,
    pub email: Option<String>,
    /// A name the user gave the account in settings.
    pub name: Option<String>,
    /// Whether this is the home the CLI uses when no variable is set.
    pub is_default: bool,
}

impl AgentAccount {
    /// `None` for the default account, which must be launched without the
    /// variable: Claude Code keys its Keychain item on the literal
    /// `CLAUDE_CONFIG_DIR` value, so setting it to the default home would
    /// look like a signed-out profile.
    pub fn id(&self) -> Option<AccountId> {
        (!self.is_default).then(|| AccountId::new(&self.home))
    }

    /// The account as an explicit choice: the CLI's own home is
    /// [`AccountId::system`].
    pub fn selection(&self) -> AccountId {
        self.id().unwrap_or_else(AccountId::system)
    }

    pub fn label(&self) -> String {
        self.name
            .clone()
            .or_else(|| self.email.clone())
            .unwrap_or_else(|| self.home_label.clone())
    }
}

/// An account the user listed in settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfiguredAccount {
    pub provider: AccountProvider,
    pub home: PathBuf,
    pub name: Option<String>,
}

/// Combines discovered accounts with the ones from settings. A configured
/// account with the home of a discovered one names it; the others are added.
pub fn merge_accounts(
    mut accounts: Vec<AgentAccount>,
    configured: &[ConfiguredAccount],
    home_dir: &Path,
) -> Vec<AgentAccount> {
    for entry in configured {
        if let Some(existing) = accounts
            .iter_mut()
            .find(|account| account.provider == entry.provider && account.home == entry.home)
        {
            if entry.name.is_some() {
                existing.name = entry.name.clone();
            }
            continue;
        }
        accounts.push(AgentAccount {
            provider: entry.provider,
            home_label: tilde_label(&entry.home, home_dir),
            home: entry.home.clone(),
            email: None,
            name: entry.name.clone(),
            is_default: false,
        });
    }
    accounts
}

/// A home directory for a new account named `name`, such as
/// `~/.claude-work`, that does not exist yet. Discovery finds these names.
pub fn new_account_home(provider: AccountProvider, name: &str, home_dir: &Path) -> PathBuf {
    let prefix = match provider {
        AccountProvider::Claude => ".claude",
        AccountProvider::Codex => ".codex",
        AccountProvider::Grok => ".grok",
        AccountProvider::Cursor => ".cursor",
    };
    let slug: String = name
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let slug = if slug.is_empty() {
        "account".to_string()
    } else {
        slug
    };
    let base = format!("{prefix}-{slug}");
    let mut candidate = home_dir.join(&base);
    let mut counter = 2;
    while candidate.exists() {
        candidate = home_dir.join(format!("{base}-{counter}"));
        counter += 1;
    }
    candidate
}

/// Creates the directories a new account's CLI expects before its first login.
pub fn prepare_account_home(provider: AccountProvider, home: &Path) -> std::io::Result<()> {
    match provider {
        // Cursor's home stands in for `HOME`; its config lives below it.
        AccountProvider::Cursor => std::fs::create_dir_all(home.join(".cursor")),
        _ => std::fs::create_dir_all(home),
    }
}

/// Expands a leading `~` the way a shell would.
pub fn expand_home(path: &str, home_dir: &Path) -> PathBuf {
    if path == "~" {
        home_dir.to_path_buf()
    } else if let Some(rest) = path.strip_prefix("~/") {
        home_dir.join(rest)
    } else {
        PathBuf::from(path)
    }
}

/// The environment to add to an agent process for the given account.
pub fn account_env(agent_id: &str, account: Option<&AccountId>) -> Vec<(String, String)> {
    match (AccountProvider::for_agent(agent_id), account) {
        (Some(provider), Some(account)) if !account.is_system() => {
            provider.account_env(account.as_str())
        }
        _ => Vec::new(),
    }
}

/// Whether an agent's error says the account ran out of quota or credits,
/// as opposed to a transient rate limit worth retrying.
pub fn is_usage_limit_error(message: &str) -> bool {
    const MARKERS: &[&str] = &[
        "usagelimitexceeded",
        "usage_limit_exceeded",
        "usage limit",
        "out of credits",
        "insufficient credits",
        "credit balance is too low",
        "hit your limit",
        "limit reached",
        "quota exceeded",
        "exceeded your current quota",
        "insufficient_quota",
    ];
    let message = message.to_lowercase();
    MARKERS.iter().any(|marker| message.contains(marker))
}

/// A short label for an account id when its discovery details are unknown.
pub fn fallback_account_label(account: &AccountId, home_dir: &Path) -> String {
    tilde_label(account.home(), home_dir)
}

pub(crate) fn tilde_label(path: &Path, home_dir: &Path) -> String {
    match path.strip_prefix(home_dir) {
        Ok(relative) => format!("~/{}", relative.display()),
        Err(_) => path.display().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn env_only_for_known_agents_and_non_default_accounts() {
        let account = AccountId::from("/Users/me/.claude-work");
        assert_eq!(
            account_env(CLAUDE_AGENT_ID, Some(&account)),
            vec![(
                "CLAUDE_CONFIG_DIR".to_string(),
                "/Users/me/.claude-work".to_string()
            )]
        );
        assert_eq!(
            account_env(CODEX_AGENT_ID, Some(&AccountId::from("/Users/me/.codex-2"))),
            vec![("CODEX_HOME".to_string(), "/Users/me/.codex-2".to_string())]
        );
        assert!(account_env(CLAUDE_AGENT_ID, None).is_empty());
        assert_eq!(
            account_env("Grok Build", Some(&AccountId::from("/Users/me/.grok-2"))),
            vec![("GROK_HOME".to_string(), "/Users/me/.grok-2".to_string())]
        );
        assert_eq!(
            account_env("cursor", Some(&AccountId::from("/Users/me/.cursor-work"))),
            vec![
                ("HOME".to_string(), "/Users/me/.cursor-work".to_string()),
                (
                    "CURSOR_CONFIG_DIR".to_string(),
                    "/Users/me/.cursor-work/.cursor".to_string()
                ),
                ("AGENT_CLI_CREDENTIAL_STORE".to_string(), "file".to_string()),
            ]
        );
        assert!(account_env("gemini", Some(&account)).is_empty());
        assert!(account_env(CLAUDE_AGENT_ID, Some(&AccountId::system())).is_empty());
    }

    #[test]
    fn default_account_has_no_id() {
        let account = AgentAccount {
            provider: AccountProvider::Claude,
            home: PathBuf::from("/Users/me/.claude"),
            home_label: "~/.claude".into(),
            email: None,
            name: None,
            is_default: true,
        };
        assert_eq!(account.id(), None);
        assert_eq!(account.label(), "~/.claude");
    }

    #[test]
    fn configured_accounts_name_or_extend_the_discovered_ones() {
        let home = Path::new("/Users/me");
        let discovered = vec![AgentAccount {
            provider: AccountProvider::Claude,
            home: home.join(".claude-work"),
            home_label: "~/.claude-work".into(),
            email: Some("me@work.dev".into()),
            name: None,
            is_default: false,
        }];
        let configured = vec![
            ConfiguredAccount {
                provider: AccountProvider::Claude,
                home: expand_home("~/.claude-work", home),
                name: Some("Lavoro".into()),
            },
            ConfiguredAccount {
                provider: AccountProvider::Codex,
                home: expand_home("/Volumes/x/codex-home", home),
                name: None,
            },
        ];
        let merged = merge_accounts(discovered, &configured, home);
        let labels: Vec<_> = merged.iter().map(AgentAccount::label).collect();
        assert_eq!(labels, vec!["Lavoro", "/Volumes/x/codex-home"]);
        assert_eq!(merged[1].provider, AccountProvider::Codex);
        assert!(!merged[1].is_default);
    }

    #[test]
    fn new_account_homes_are_named_after_the_account_and_unique() {
        let temp = tempfile::tempdir().unwrap();
        let home = temp.path();
        let first = new_account_home(AccountProvider::Claude, " Lavoro SIAP! ", home);
        assert_eq!(first, home.join(".claude-lavoro-siap"));
        prepare_account_home(AccountProvider::Claude, &first).unwrap();
        assert_eq!(
            new_account_home(AccountProvider::Claude, "lavoro siap", home),
            home.join(".claude-lavoro-siap-2")
        );
        assert_eq!(
            new_account_home(AccountProvider::Codex, "", home),
            home.join(".codex-account")
        );

        let cursor = new_account_home(AccountProvider::Cursor, "work", home);
        prepare_account_home(AccountProvider::Cursor, &cursor).unwrap();
        assert!(cursor.join(".cursor").is_dir());
    }

    #[test]
    fn recognizes_usage_limit_errors() {
        for message in [
            r#"Internal error: {"message": "Your workspace is out of credits.", "codexErrorInfo": "usageLimitExceeded"}"#,
            "Claude AI usage limit reached|1791080690",
            "You've hit your limit · resets 5pm",
            "Your credit balance is too low to access the Anthropic API.",
            "insufficient_quota",
        ] {
            assert!(is_usage_limit_error(message), "{message}");
        }
        for message in [
            "Rate limit exceeded, retrying",
            "Authentication required",
            "overloaded",
        ] {
            assert!(!is_usage_limit_error(message), "{message}");
        }
    }
}
