//! Per-thread account and hand-off records.
//!
//! A thread run with a non-default account must be reopened with the same
//! account, because its session lives in that account's store. A thread
//! created by "Continue with…" remembers where it came from. Both are kept
//! here, keyed by [`ThreadId`], instead of in the thread metadata table.

use agent_accounts::AccountId;
use anyhow::Context as _;
use collections::HashMap;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Global, Task, TaskExt as _};
use project::AgentId;
use serde::{Deserialize, Serialize};
use util::ResultExt as _;

use crate::Agent;
use crate::thread_metadata_store::ThreadId;

const NAMESPACE: &str = "agent_thread_accounts";

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadAccountInfo {
    /// The account the thread's agent runs with; `None` is the default one.
    #[serde(default)]
    pub account: Option<AccountId>,
    /// Set on threads created by continuing another thread.
    #[serde(default)]
    pub handoff_from: Option<HandoffSource>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandoffSource {
    pub thread_id: String,
    pub agent_id: AgentId,
    #[serde(default)]
    pub account: Option<AccountId>,
}

impl HandoffSource {
    pub fn source_thread_id(&self) -> Option<ThreadId> {
        ThreadId::from_key_string(&self.thread_id)
    }
}

/// What was last written in this session, so that reads right after a write
/// don't race the background persistence.
#[derive(Default)]
struct RecentWrites(HashMap<ThreadId, ThreadAccountInfo>);

impl Global for RecentWrites {}

pub fn read(thread_id: ThreadId, cx: &App) -> Option<ThreadAccountInfo> {
    if let Some(info) = cx
        .try_global::<RecentWrites>()
        .and_then(|recent| recent.0.get(&thread_id))
    {
        return Some(info.clone());
    }
    let raw = KeyValueStore::global(cx)
        .scoped(NAMESPACE)
        .read(&thread_id.to_key_string())
        .log_err()
        .flatten()?;
    serde_json::from_str(&raw).log_err()
}

pub fn write(
    thread_id: ThreadId,
    info: &ThreadAccountInfo,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    cx.default_global::<RecentWrites>()
        .0
        .insert(thread_id, info.clone());
    let kvp = KeyValueStore::global(cx);
    let key = thread_id.to_key_string();
    let payload = match serde_json::to_string(info).context("serializing thread account") {
        Ok(payload) => payload,
        Err(err) => return Task::ready(Err(err)),
    };
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).write(key, payload).await })
}

/// Records the account a thread runs with, keeping any hand-off record.
pub fn record_account(thread_id: ThreadId, account: Option<AccountId>, cx: &mut App) {
    let existing = read(thread_id, cx);
    if existing.as_ref().map(|info| &info.account) == Some(&account)
        || (existing.is_none() && account.is_none())
    {
        return;
    }
    let info = ThreadAccountInfo {
        account,
        ..existing.unwrap_or_default()
    };
    write(thread_id, &info, cx).detach_and_log_err(cx);
}

/// The agent to reopen a persisted thread with: the stored account wins over
/// an agent that does not name one.
pub fn agent_for_thread(agent: Agent, thread_id: ThreadId, cx: &App) -> Agent {
    match agent {
        Agent::Custom { id, account: None } => {
            let account = read(thread_id, cx).and_then(|info| info.account);
            Agent::Custom { id, account }
        }
        agent => agent,
    }
}

const TERMINAL_NAMESPACE: &str = "agent_terminal_accounts";

/// The account a terminal thread was opened with: its shell gets the
/// account's home variable, so the agent CLIs started in it use that account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminalAccount {
    pub agent_id: AgentId,
    pub account: AccountId,
}

impl TerminalAccount {
    pub fn env(&self) -> HashMap<String, String> {
        agent_accounts::account_env(self.agent_id.as_ref(), Some(&self.account))
            .into_iter()
            .collect()
    }
}

#[derive(Default)]
struct RecentTerminalWrites(HashMap<String, TerminalAccount>);

impl Global for RecentTerminalWrites {}

pub fn terminal_account(terminal_key: &str, cx: &App) -> Option<TerminalAccount> {
    if let Some(account) = cx
        .try_global::<RecentTerminalWrites>()
        .and_then(|recent| recent.0.get(terminal_key))
    {
        return Some(account.clone());
    }
    let raw = KeyValueStore::global(cx)
        .scoped(TERMINAL_NAMESPACE)
        .read(terminal_key)
        .log_err()
        .flatten()?;
    serde_json::from_str(&raw).log_err()
}

pub fn write_terminal_account(terminal_key: String, account: TerminalAccount, cx: &mut App) {
    let payload = match serde_json::to_string(&account).context("serializing terminal account") {
        Ok(payload) => payload,
        Err(err) => {
            log::error!("{err:#}");
            return;
        }
    };
    cx.default_global::<RecentTerminalWrites>()
        .0
        .insert(terminal_key.clone(), account);
    let kvp = KeyValueStore::global(cx);
    cx.background_spawn(async move {
        kvp.scoped(TERMINAL_NAMESPACE)
            .write(terminal_key, payload)
            .await
    })
    .detach_and_log_err(cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    fn init(cx: &mut TestAppContext) {
        cx.update(|cx| cx.set_global(db::AppDatabase::test_new()));
    }

    /// Drops the in-memory copies so reads must come from the database.
    fn forget_recent_writes(cx: &mut TestAppContext) {
        cx.run_until_parked();
        cx.update(|cx| {
            if cx.has_global::<RecentWrites>() {
                cx.remove_global::<RecentWrites>();
            }
            if cx.has_global::<RecentTerminalWrites>() {
                cx.remove_global::<RecentTerminalWrites>();
            }
        });
    }

    #[gpui::test]
    async fn persisted_thread_reopens_with_its_account(cx: &mut TestAppContext) {
        init(cx);
        let thread_id = ThreadId::new();
        let account = AccountId::from("/Users/me/.claude-work");
        cx.update(|cx| record_account(thread_id, Some(account.clone()), cx));
        forget_recent_writes(cx);

        cx.update(|cx| {
            assert_eq!(
                read(thread_id, cx).and_then(|info| info.account),
                Some(account.clone())
            );
            assert_eq!(
                agent_for_thread(Agent::from(AgentId::new("claude-acp")), thread_id, cx),
                Agent::Custom {
                    id: AgentId::new("claude-acp"),
                    account: Some(account.clone()),
                }
            );
            // An explicitly chosen account is kept.
            let other = AccountId::from("/Users/me/.claude-home");
            assert_eq!(
                agent_for_thread(
                    Agent::with_account(AgentId::new("claude-acp"), Some(other.clone())),
                    thread_id,
                    cx
                ),
                Agent::with_account(AgentId::new("claude-acp"), Some(other)),
            );
            // Threads without a record keep the default account.
            assert_eq!(
                agent_for_thread(Agent::from(AgentId::new("codex-acp")), ThreadId::new(), cx),
                Agent::from(AgentId::new("codex-acp")),
            );
        });
    }

    #[gpui::test]
    async fn recording_the_account_keeps_the_handoff_source(cx: &mut TestAppContext) {
        init(cx);
        let thread_id = ThreadId::new();
        let source = HandoffSource {
            thread_id: ThreadId::new().to_key_string(),
            agent_id: AgentId::new("claude-acp"),
            account: None,
        };
        let account = AccountId::from("/Users/me/.codex-2");
        cx.update(|cx| {
            write(
                thread_id,
                &ThreadAccountInfo {
                    account: Some(account.clone()),
                    handoff_from: Some(source.clone()),
                },
                cx,
            )
            .detach();
            // Opening the thread records the same account again: a no-op.
            record_account(thread_id, Some(account.clone()), cx);
        });
        forget_recent_writes(cx);

        cx.update(|cx| {
            assert_eq!(
                read(thread_id, cx),
                Some(ThreadAccountInfo {
                    account: Some(account),
                    handoff_from: Some(source),
                })
            );
        });
    }

    #[gpui::test]
    async fn terminal_account_sets_the_home_variable(cx: &mut TestAppContext) {
        init(cx);
        cx.update(|cx| {
            write_terminal_account(
                "terminal-1".into(),
                TerminalAccount {
                    agent_id: AgentId::new("codex-acp"),
                    account: AccountId::from("/Users/me/.codex-2"),
                },
                cx,
            )
        });
        forget_recent_writes(cx);

        cx.update(|cx| {
            let account = terminal_account("terminal-1", cx).expect("persisted");
            assert_eq!(
                account.env(),
                HashMap::from_iter([("CODEX_HOME".to_string(), "/Users/me/.codex-2".to_string())])
            );
            assert_eq!(terminal_account("terminal-2", cx), None);
        });
    }
}
