//! The session manager's state (S10 U1): the workspace's OMP sessions and
//! the open session's history as OMP pages it. The panel renders it and
//! sends every change through the OMP link.

use cedian_omp::SessionEntry;
use serde_json::{Map, Value};

/// Messages per history page.
pub const HISTORY_PAGE: i64 = 20;

/// One page of OMP's `get_messages_page`, one line per message.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryPage {
    pub lines: Vec<String>,
    pub total: i64,
    /// OMP's cursor for the next page; `None` on the last.
    pub next: Option<String>,
}

impl From<omp_rpc::MessagesPage> for HistoryPage {
    fn from(page: omp_rpc::MessagesPage) -> Self {
        Self {
            lines: page
                .messages
                .iter()
                .map(cedian_omp::history::message_line)
                .collect(),
            total: page.total_messages,
            next: page.next_cursor,
        }
    }
}

/// The open session's history as loaded so far.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct History {
    pub lines: Vec<String>,
    pub total: Option<i64>,
    pub next: Option<String>,
    /// The session's entries, once Timeline was read.
    pub timeline: Vec<String>,
    /// OMP's last assistant text, once read: `Some(None)` when it has none.
    pub last_reply: Option<Option<String>>,
    /// Every message was read at once (`get_messages`).
    pub all: bool,
}

impl History {
    /// Apply a page: the first replaces what is shown, a later one adds.
    pub fn page(&mut self, page: HistoryPage, more: bool) {
        if !more {
            self.lines.clear();
        }
        self.lines.extend(page.lines);
        self.total = Some(page.total);
        self.next = page.next;
    }

    /// Every message at once (`get_messages`): nothing is left to page.
    pub fn all(&mut self, lines: Vec<String>) {
        self.total = Some(lines.len() as i64);
        self.lines = lines;
        self.next = None;
        self.all = true;
    }
}

/// What the session manager shows.
#[derive(Debug, Clone, Default)]
pub struct SessionManager {
    /// The session list is open.
    pub open: bool,
    pub entries: Vec<SessionEntry>,
    /// Why the list could not be read.
    pub list_error: Option<String>,
    /// The current session's name, as OMP last gave it.
    pub title: Option<String>,
    pub history: History,
}

impl SessionManager {
    /// The session with `id` was renamed to `title`.
    pub fn renamed(&mut self, id: &str, title: Option<String>) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.title = title.clone();
        }
        self.title = title;
    }

    /// OMP is now on another session: what was shown of the old one goes.
    pub fn moved(&mut self, title: Option<String>) {
        self.title = title;
        self.history = History::default();
    }
}

/// One line for an entry of OMP's `get_entries`.
pub fn entry_line(entry: &Map<String, Value>) -> String {
    let text = |key: &str| entry.get(key).and_then(Value::as_str);
    let kind = text("type").unwrap_or("entry");
    let detail = match kind {
        "message" => entry
            .get("message")
            .and_then(|m| m.get("role"))
            .and_then(Value::as_str),
        "model_change" => text("model"),
        "thinking_level_change" => text("thinkingLevel"),
        "title_change" => text("title"),
        _ => None,
    };
    match detail {
        Some(detail) => format!("{kind}: {detail}"),
        None => kind.to_string(),
    }
}

/// A session id's last 8 characters: OMP's ids are UUIDv7, whose first
/// characters are the time and repeat between sessions started together.
pub fn short_id(id: &str) -> &str {
    id.get(id.len().saturating_sub(8)..).unwrap_or(id)
}

/// How a session reads in the list.
pub fn entry_label(entry: &SessionEntry) -> String {
    format!(
        "{} · {} · {}",
        entry.title.as_deref().unwrap_or("untitled"),
        short_id(&entry.id),
        entry
            .started
            .get(..16)
            .unwrap_or(&entry.started)
            .replace('T', " ")
    )
}
