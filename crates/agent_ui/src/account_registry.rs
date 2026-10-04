//! The provider accounts found on this machine, for the account pickers.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use agent_accounts::quota::{AccountQuota, QuotaStatus};
use agent_accounts::{AccountId, AccountProvider, AgentAccount, ConfiguredAccount};
use collections::HashMap;
use gpui::{App, AppContext as _, Global};
use settings::{RegisterSetting, Settings};

/// The `agent_accounts` settings.
#[derive(Clone, Debug, RegisterSetting)]
pub struct AgentAccountsSettings {
    pub discover: bool,
    pub accounts: Vec<ConfiguredAccount>,
    pub auto_switch: bool,
    pub auto_switch_threshold_percent: f32,
}

impl Settings for AgentAccountsSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = content.agent_accounts.clone().unwrap_or_default();
        let home_dir = util::paths::home_dir();
        let accounts = content
            .accounts
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                let Some(provider) = AccountProvider::for_agent(&entry.agent) else {
                    log::warn!(
                        "agent_accounts: agent `{}` has no account support",
                        entry.agent
                    );
                    return None;
                };
                Some(ConfiguredAccount {
                    provider,
                    home: agent_accounts::expand_home(&entry.home, home_dir),
                    name: entry.name,
                })
            })
            .collect();
        let auto_switch = content.auto_switch.unwrap_or_default();
        Self {
            discover: content.discover.unwrap_or(true),
            accounts,
            auto_switch: auto_switch.enabled.unwrap_or(false),
            auto_switch_threshold_percent: auto_switch.threshold_percent.unwrap_or(95.0),
        }
    }
}

/// How long a discovery result is reused before menus trigger a new scan.
const REFRESH_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Default)]
pub struct AccountRegistry {
    accounts: Arc<Vec<AgentAccount>>,
    refreshed_at: Option<Instant>,
    refreshing: bool,
}

impl Global for AccountRegistry {}

impl AccountRegistry {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default());
        Self::refresh(cx);
        let mut previous = AgentAccountsSettings::get_global(cx).clone();
        cx.observe_global::<settings::SettingsStore>(move |cx| {
            let current = AgentAccountsSettings::get_global(cx);
            if current.discover != previous.discover || current.accounts != previous.accounts {
                previous = current.clone();
                Self::refresh(cx);
            }
        })
        .detach();
    }

    /// Rescans in the background unless a recent result exists.
    pub fn refresh_if_stale(cx: &mut App) {
        let Some(registry) = cx.try_global::<Self>() else {
            return;
        };
        let stale = registry
            .refreshed_at
            .is_none_or(|at| at.elapsed() > REFRESH_INTERVAL);
        if stale && !registry.refreshing {
            Self::refresh(cx);
        }
    }

    fn refresh(cx: &mut App) {
        cx.global_mut::<Self>().refreshing = true;
        let settings = AgentAccountsSettings::get_global(cx).clone();
        let scan = cx.background_spawn(async move {
            let discovered = if settings.discover {
                agent_accounts::discover_accounts()
            } else {
                Vec::new()
            };
            agent_accounts::merge_accounts(discovered, &settings.accounts, util::paths::home_dir())
        });
        cx.spawn(async move |cx| {
            let accounts = scan.await;
            cx.update_global::<Self, _>(|registry, _| {
                registry.accounts = Arc::new(accounts);
                registry.refreshed_at = Some(Instant::now());
                registry.refreshing = false;
            });
        })
        .detach();
    }

    /// The accounts usable with an agent, default first. Empty for agents
    /// without account support.
    pub fn accounts_for_agent(agent_id: &str, cx: &App) -> Vec<AgentAccount> {
        let Some(provider) = AccountProvider::for_agent(agent_id) else {
            return Vec::new();
        };
        let Some(registry) = cx.try_global::<Self>() else {
            return Vec::new();
        };
        let mut accounts: Vec<_> = registry
            .accounts
            .iter()
            .filter(|account| account.provider == provider)
            .cloned()
            .collect();
        accounts.sort_by_key(|account| !account.is_default);
        accounts
    }

    /// A short label for an account of the given agent.
    pub fn label(agent_id: &str, account: Option<&AccountId>, cx: &App) -> String {
        let accounts = Self::accounts_for_agent(agent_id, cx);
        let found = accounts
            .iter()
            .find(|candidate| candidate.id().as_ref() == account);
        match (found, account) {
            (Some(found), _) => found.label(),
            (None, Some(account)) => {
                agent_accounts::fallback_account_label(account, util::paths::home_dir())
            }
            (None, None) => "default".to_string(),
        }
    }
}

/// How long a quota reading is reused. Anthropic rejects clients that poll
/// its usage endpoint more often than about every five minutes.
const QUOTA_TTL: Duration = Duration::from_secs(5 * 60);

struct QuotaEntry {
    quota: Option<AccountQuota>,
    fetched_at: Option<Instant>,
    fetching: bool,
}

/// Cached quota readings, keyed by provider and account home.
#[derive(Default)]
pub struct QuotaRegistry {
    entries: HashMap<(AccountProvider, PathBuf), QuotaEntry>,
}

impl Global for QuotaRegistry {}

impl QuotaRegistry {
    /// The last reading for an account, without fetching.
    pub fn quota(account: &AgentAccount, cx: &App) -> Option<AccountQuota> {
        cx.try_global::<Self>()?
            .entries
            .get(&(account.provider, account.home.clone()))?
            .quota
            .clone()
    }

    /// Fetches the accounts' quota in the background where the cached
    /// reading is missing or older than five minutes.
    pub fn refresh_if_stale(accounts: &[AgentAccount], cx: &mut App) {
        for account in accounts {
            let key = (account.provider, account.home.clone());
            let registry = cx.default_global::<Self>();
            let entry = registry.entries.entry(key.clone()).or_insert(QuotaEntry {
                quota: None,
                fetched_at: None,
                fetching: false,
            });
            let stale = entry.fetched_at.is_none_or(|at| at.elapsed() > QUOTA_TTL);
            if entry.fetching || !stale {
                continue;
            }
            entry.fetching = true;
            let account = account.clone();
            let http = cx.http_client();
            let fetch = cx.background_spawn(async move {
                agent_accounts::quota::fetch_quota(&account, http).await
            });
            cx.spawn(async move |cx| {
                let quota = fetch.await;
                cx.update_global::<Self, _>(|registry, _| {
                    if let Some(entry) = registry.entries.get_mut(&key) {
                        entry.quota = Some(quota);
                        entry.fetched_at = Some(Instant::now());
                        entry.fetching = false;
                    }
                });
            })
            .detach();
        }
    }

    /// Records that the agent reported the account out of quota, until the
    /// next reading replaces it.
    pub fn mark_exhausted(agent_id: &str, account: Option<&AccountId>, cx: &mut App) {
        let Some(found) = AccountRegistry::accounts_for_agent(agent_id, cx)
            .into_iter()
            .find(|candidate| candidate.id().as_ref() == account)
        else {
            return;
        };
        let quota = AccountQuota {
            status: QuotaStatus::Ok,
            windows: vec![agent_accounts::quota::QuotaWindow {
                id: "reported".into(),
                label: "Limit".into(),
                used_percent: 100,
                resets_at: None,
            }],
            plan: None,
        };
        cx.default_global::<Self>().entries.insert(
            (found.provider, found.home),
            QuotaEntry {
                quota: Some(quota),
                fetched_at: Some(Instant::now()),
                fetching: false,
            },
        );
    }

    /// Another account of the agent to move to after it ran out: the one with
    /// the most quota left, else any account not known to be exhausted.
    pub fn any_alternative(
        agent_id: &str,
        current: Option<&AccountId>,
        cx: &App,
    ) -> Option<AgentAccount> {
        Self::best_alternative(agent_id, current, cx).or_else(|| {
            AccountRegistry::accounts_for_agent(agent_id, cx)
                .into_iter()
                .filter(|account| account.id().as_ref() != current)
                .find(|account| !Self::is_exhausted(account, cx))
        })
    }

    /// Whether the account's last reading crossed the auto-switch threshold.
    pub fn is_exhausted(account: &AgentAccount, cx: &App) -> bool {
        let threshold = AgentAccountsSettings::get_global(cx).auto_switch_threshold_percent;
        Self::quota(account, cx).is_some_and(|quota| quota.is_exhausted(threshold))
    }

    /// The other account of the same agent with the most quota left, among
    /// those with a fresh, usable reading below the threshold.
    pub fn best_alternative(
        agent_id: &str,
        current: Option<&AccountId>,
        cx: &App,
    ) -> Option<AgentAccount> {
        let threshold = AgentAccountsSettings::get_global(cx).auto_switch_threshold_percent;
        AccountRegistry::accounts_for_agent(agent_id, cx)
            .into_iter()
            .filter(|account| account.id().as_ref() != current)
            .filter_map(|account| {
                let quota = Self::quota(&account, cx)?;
                let usable = quota.status == QuotaStatus::Ok && !quota.is_exhausted(threshold);
                usable.then(|| (quota.max_used_percent().unwrap_or(0), account))
            })
            .min_by_key(|(used, _)| *used)
            .map(|(_, account)| account)
    }
}

/// Shows an account with its quota, e.g. "me@work.dev — Session (5h) 42%".
pub fn account_label_with_quota(account: &AgentAccount, cx: &App) -> String {
    let mut label = account.label();
    if account.is_default {
        label.push_str(" (default)");
    }
    if let Some(summary) = QuotaRegistry::quota(account, cx).and_then(|quota| quota.summary()) {
        label.push_str(" — ");
        label.push_str(&summary);
    }
    label
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_accounts::quota::QuotaWindow;
    use gpui::{BorrowAppContext as _, TestAppContext};
    use settings::SettingsStore;

    fn account(home: &str, is_default: bool) -> AgentAccount {
        AgentAccount {
            provider: AccountProvider::Claude,
            home: PathBuf::from(home),
            home_label: home.into(),
            email: None,
            name: None,
            is_default,
        }
    }

    fn quota(used: u32) -> AccountQuota {
        AccountQuota {
            status: QuotaStatus::Ok,
            windows: vec![QuotaWindow {
                id: "five_hour".into(),
                label: "Session (5h)".into(),
                used_percent: used,
                resets_at: None,
            }],
            plan: None,
        }
    }

    fn init(cx: &mut TestAppContext, settings_json: &str) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            AgentAccountsSettings::register(cx);
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.set_user_settings(settings_json, cx).unwrap();
            });
        });
    }

    fn set_accounts(cx: &mut TestAppContext, entries: Vec<(AgentAccount, Option<AccountQuota>)>) {
        cx.update(|cx| {
            let mut quotas = QuotaRegistry::default();
            for (account, quota) in &entries {
                quotas.entries.insert(
                    (account.provider, account.home.clone()),
                    QuotaEntry {
                        quota: quota.clone(),
                        fetched_at: Some(Instant::now()),
                        fetching: false,
                    },
                );
            }
            cx.set_global(quotas);
            cx.set_global(AccountRegistry {
                accounts: Arc::new(entries.into_iter().map(|(account, _)| account).collect()),
                refreshed_at: Some(Instant::now()),
                refreshing: false,
            });
        });
    }

    #[gpui::test]
    fn picks_the_account_with_most_quota_left(cx: &mut TestAppContext) {
        init(
            cx,
            r#"{"agent_accounts": {"auto_switch": {"enabled": true, "threshold_percent": 90}}}"#,
        );
        let default = account("/h/.claude", true);
        let work = account("/h/.claude-work", false);
        let home = account("/h/.claude-home", false);
        let spent = account("/h/.claude-spent", false);
        set_accounts(
            cx,
            vec![
                (default.clone(), Some(quota(95))),
                (work.clone(), Some(quota(60))),
                (home.clone(), Some(quota(20))),
                (spent, Some(quota(91))),
            ],
        );
        cx.update(|cx| {
            assert!(QuotaRegistry::is_exhausted(&default, cx));
            assert!(!QuotaRegistry::is_exhausted(&work, cx));
            let best = QuotaRegistry::best_alternative("claude-acp", None, cx).unwrap();
            assert_eq!(best.home, home.home);
            // The current account is never its own alternative.
            let best =
                QuotaRegistry::best_alternative("claude-acp", home.id().as_ref(), cx).unwrap();
            assert_eq!(best.home, work.home);
            assert_eq!(
                account_label_with_quota(&default, cx),
                "/h/.claude (default) — Session (5h) 95%"
            );
        });
    }

    #[gpui::test]
    fn reported_limits_exclude_the_account(cx: &mut TestAppContext) {
        init(cx, "{}");
        let default = account("/h/.claude", true);
        let unknown = account("/h/.claude-unknown", false);
        let spent = account("/h/.claude-spent", false);
        set_accounts(
            cx,
            vec![
                (default.clone(), None),
                (spent.clone(), None),
                (unknown.clone(), None),
            ],
        );
        cx.update(|cx| {
            QuotaRegistry::mark_exhausted("claude-acp", None, cx);
            QuotaRegistry::mark_exhausted("claude-acp", spent.id().as_ref(), cx);
            assert!(QuotaRegistry::is_exhausted(&default, cx));
            let alternative = QuotaRegistry::any_alternative("claude-acp", None, cx).unwrap();
            assert_eq!(alternative.home, unknown.home);
        });
    }

    #[gpui::test]
    fn accounts_without_a_reading_are_not_alternatives(cx: &mut TestAppContext) {
        init(cx, "{}");
        let default = account("/h/.claude", true);
        let unknown = account("/h/.claude-unknown", false);
        set_accounts(cx, vec![(default, Some(quota(99))), (unknown, None)]);
        cx.update(|cx| {
            assert!(QuotaRegistry::best_alternative("claude-acp", None, cx).is_none());
        });
    }

    #[gpui::test]
    fn reads_configured_accounts(cx: &mut TestAppContext) {
        init(
            cx,
            r#"{"agent_accounts": {
                "discover": false,
                "accounts": [
                    {"agent": "claude-acp", "home": "~/.claude-lavoro", "name": "Lavoro"},
                    {"agent": "Grok Build", "home": "/opt/grok-2"},
                    {"agent": "gemini", "home": "/x"}
                ]
            }}"#,
        );
        cx.update(|cx| {
            let settings = AgentAccountsSettings::get_global(cx);
            assert!(!settings.discover);
            assert!(!settings.auto_switch);
            assert_eq!(settings.auto_switch_threshold_percent, 95.0);
            assert_eq!(
                settings.accounts,
                vec![
                    ConfiguredAccount {
                        provider: AccountProvider::Claude,
                        home: util::paths::home_dir().join(".claude-lavoro"),
                        name: Some("Lavoro".into()),
                    },
                    ConfiguredAccount {
                        provider: AccountProvider::Grok,
                        home: PathBuf::from("/opt/grok-2"),
                        name: None,
                    },
                ]
            );
        });
    }
}
