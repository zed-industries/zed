//! Per-thread draft prompt persistence and display label rendering.
//!
//! Drafts are persisted in the thread metadata store with `session_id: None`,
//! but their unsent prompt text is kept separately here so we don't have to
//! plumb draft-prompt storage through the native agent's thread database.
//!
//! The display-label helpers ([`display_label_for_draft`] and friends) live
//! alongside the storage so the sidebar's preview rendering can't drift from
//! the format we persist.

use agent_client_protocol::schema as acp;
use anyhow::Context as _;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Entity, Task};
use ui::SharedString;
use util::ResultExt as _;
use workspace::Workspace;

use crate::AgentPanel;
use crate::thread_metadata_store::ThreadId;

const NAMESPACE: &str = "agent_draft_prompts";

/// Maximum length (in characters) of a draft label rendered in the sidebar.
const MAX_LABEL_CHARS: usize = 250;

pub fn read(thread_id: ThreadId, cx: &App) -> Option<Vec<acp::ContentBlock>> {
    let kvp = KeyValueStore::global(cx);
    let raw = kvp
        .scoped(NAMESPACE)
        .read(&thread_id_key(thread_id))
        .log_err()
        .flatten()?;
    serde_json::from_str(&raw).log_err()
}

pub fn write(
    thread_id: ThreadId,
    prompt: &[acp::ContentBlock],
    cx: &App,
) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = thread_id_key(thread_id);
    let payload = match serde_json::to_string(prompt).context("serializing draft prompt") {
        Ok(payload) => payload,
        Err(err) => return Task::ready(Err(err)),
    };
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).write(key, payload).await })
}

pub fn delete(thread_id: ThreadId, cx: &App) -> Task<anyhow::Result<()>> {
    let kvp = KeyValueStore::global(cx);
    let key = thread_id_key(thread_id);
    cx.background_spawn(async move { kvp.scoped(NAMESPACE).delete(key).await })
}

fn thread_id_key(thread_id: ThreadId) -> String {
    thread_id.to_key_string()
}

/// Rewrites `[@Something](scheme://...)` mention links as `@Something` so the
/// sidebar's draft-title preview doesn't show raw markdown link syntax.
pub fn clean_mention_links(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut remaining = input;

    while let Some(start) = remaining.find("[@") {
        result.push_str(&remaining[..start]);
        let after_bracket = &remaining[start + 1..];
        if let Some(close_bracket) = after_bracket.find("](") {
            let mention = &after_bracket[..close_bracket];
            let after_link_start = &after_bracket[close_bracket + 2..];
            if let Some(close_paren) = after_link_start.find(')') {
                result.push_str(mention);
                remaining = &after_link_start[close_paren + 1..];
                continue;
            }
        }
        result.push_str("[@");
        remaining = &remaining[start + 2..];
    }
    result.push_str(remaining);
    result
}

/// Collapses whitespace and truncates raw editor text for display as a draft
/// label in the sidebar.
pub fn truncate_draft_label(raw: &str) -> Option<SharedString> {
    let first_line = raw.lines().next().unwrap_or("");
    let cleaned = clean_mention_links(first_line);
    let mut text: String = cleaned.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        return None;
    }
    if let Some((truncate_at, _)) = text.char_indices().nth(MAX_LABEL_CHARS) {
        text.truncate(truncate_at);
    }
    Some(text.into())
}

/// Renders a draft thread's display label for sidebar rows and similar
/// preview UI.
///
/// Prefers the live message editor's text (when the thread's
/// `ConversationView` is loaded in the workspace's `AgentPanel`), and
/// otherwise falls back to the persisted draft prompt in the kvp store so
/// drafts restored from disk — but not yet opened — still show a meaningful
/// title instead of the generic default.
pub fn display_label_for_draft(
    workspace: Option<&Entity<Workspace>>,
    thread_id: ThreadId,
    cx: &App,
) -> Option<SharedString> {
    let in_memory = workspace
        .and_then(|ws| ws.read(cx).panel::<AgentPanel>(cx))
        .and_then(|panel| panel.read(cx).editor_text_if_in_memory(thread_id, cx));
    match in_memory {
        Some(Some(raw)) => return truncate_draft_label(&raw),
        Some(None) => return None,
        None => {}
    }

    let blocks = read(thread_id, cx)?;
    let raw = blocks
        .iter()
        .filter_map(|block| match block {
            acp::ContentBlock::Text(text) => Some(text.text.as_str()),
            acp::ContentBlock::ResourceLink(link) => Some(link.uri.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");
    truncate_draft_label(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_mention_links() {
        // Simple mention link.
        assert_eq!(
            clean_mention_links("check [@Button.tsx](file:///path/to/Button.tsx)"),
            "check @Button.tsx"
        );

        // Multiple mention links on one line.
        assert_eq!(
            clean_mention_links("look at [@foo.rs](file:///foo.rs) and [@bar.rs](file:///bar.rs)"),
            "look at @foo.rs and @bar.rs"
        );

        // Plain text without mentions is preserved.
        assert_eq!(
            clean_mention_links("plain text with no mentions"),
            "plain text with no mentions"
        );

        // Broken syntax (no closing bracket) is left alone.
        assert_eq!(
            clean_mention_links("broken [@mention without closing"),
            "broken [@mention without closing"
        );

        // Non-`@` markdown links are not touched.
        assert_eq!(
            clean_mention_links("see [docs](https://example.com)"),
            "see [docs](https://example.com)"
        );

        // Empty input.
        assert_eq!(clean_mention_links(""), "");
    }
}
