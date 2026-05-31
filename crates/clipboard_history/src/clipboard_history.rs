use collections::VecDeque;
use gpui::{Global, SharedString};
use parking_lot::Mutex;
use std::time::Duration;
use workspace::WorkspaceDb;

const MAX_CLIPBOARD_HISTORY: usize = 300;
const CLIPBOARD_POLL_INTERVAL: Duration = Duration::from_millis(500);

static CLIPBOARD_ENTRIES: Mutex<VecDeque<ClipboardEntry>> = Mutex::new(VecDeque::new());
static LAST_CLIPBOARD_TEXT: Mutex<Option<String>> = Mutex::new(None);

#[derive(Clone, Debug)]
pub struct ClipboardEntry {
    pub text: String,
    pub timestamp: std::time::SystemTime,
}

impl ClipboardEntry {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timestamp: std::time::SystemTime::now(),
        }
    }

    pub fn preview(&self) -> SharedString {
        let text = self.text.trim();
        let max_len = 500;

        // Replace newlines with ⏎ symbol
        let text_with_newline_symbols = text.replace('\n', "⏎");

        if text_with_newline_symbols.len() <= max_len {
            text_with_newline_symbols.into()
        } else {
            // Use character-based indexing to avoid breaking multi-byte UTF-8 characters
            let preview = text_with_newline_symbols
                .chars()
                .take(max_len)
                .collect::<String>();
            format!("{}…", preview).into()
        }
    }
}

pub struct ClipboardHistory {}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl Global for ClipboardHistory {}

impl ClipboardHistory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_entry(text: String, cx: &gpui::App) {
        // Don't add empty entries or very short entries (<=3 chars)
        if text.is_empty() || text.len() <= 3 {
            return;
        }

        let mut entries = CLIPBOARD_ENTRIES.lock();

        // Remove any existing occurrence of this text to avoid duplicates
        entries.retain(|entry| entry.text != text);

        // Add the new entry at the front
        let entry = ClipboardEntry::new(text.clone());
        let timestamp = format_timestamp(&entry.timestamp);
        entries.push_front(entry);

        // Trim to max size if needed
        if entries.len() > MAX_CLIPBOARD_HISTORY {
            entries.pop_back();
        }

        drop(entries);

        // Save to database asynchronously
        let db = WorkspaceDb::global(cx);
        smol::spawn(async move {
            if let Err(e) = db.save_clipboard_entry(&text, &timestamp).await {
                log::error!("Failed to save clipboard entry to database: {:?}", e);
            }
        })
        .detach();
    }

    pub fn entries() -> Vec<ClipboardEntry> {
        CLIPBOARD_ENTRIES.lock().iter().cloned().collect()
    }
}

/// Helper function to track clipboard text in history
pub fn track_clipboard(text: &str, cx: &gpui::App) {
    *LAST_CLIPBOARD_TEXT.lock() = Some(text.to_string());
    ClipboardHistory::add_entry(text.to_string(), cx);
}

/// Format a SystemTime as an SQLite-compatible timestamp string
fn format_timestamp(time: &std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();

    // Convert to SQLite datetime format: YYYY-MM-DD HH:MM:SS
    let datetime = chrono::DateTime::from_timestamp(secs as i64, 0)
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Parse an SQLite timestamp string back to SystemTime
fn parse_timestamp(timestamp: &str) -> std::time::SystemTime {
    use std::time::UNIX_EPOCH;

    if let Ok(datetime) = chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S") {
        let secs = datetime.and_utc().timestamp();
        UNIX_EPOCH + std::time::Duration::from_secs(secs as u64)
    } else {
        std::time::SystemTime::now()
    }
}

pub fn init(cx: &mut gpui::App) {
    cx.set_global(ClipboardHistory::new());

    // Load clipboard history from database on startup
    let db = WorkspaceDb::global(cx);
    cx.spawn(async move |cx: &mut gpui::AsyncApp| {
        // Clean up duplicates in database first
        match db.delete_duplicate_clipboard_entries().await {
            Ok(()) => {
                log::info!("Removed duplicate clipboard entries from database");
            }
            Err(e) => {
                log::error!("Failed to delete duplicate clipboard entries: {:?}", e);
            }
        }

        // Load clipboard history from database
        match db.get_clipboard_entries(MAX_CLIPBOARD_HISTORY).await {
            Ok(db_entries) => {
                let mut entries = CLIPBOARD_ENTRIES.lock();
                entries.clear();
                for (text, timestamp) in db_entries {
                    let entry = ClipboardEntry {
                        text,
                        timestamp: parse_timestamp(&timestamp),
                    };
                    entries.push_back(entry);
                }
            }
            Err(e) => {
                log::error!("Failed to load clipboard history from database: {:?}", e);
            }
        }

        // Poll the system clipboard for changes from other applications
        loop {
            cx.background_executor()
                .timer(CLIPBOARD_POLL_INTERVAL)
                .await;

            let clipboard_text =
                cx.update(|cx| cx.read_from_clipboard().and_then(|item| item.text()));

            if let Some(text) = clipboard_text {
                let mut last = LAST_CLIPBOARD_TEXT.lock();
                let is_new = last.as_ref() != Some(&text);
                if is_new {
                    *last = Some(text.clone());
                    drop(last);
                    // We need a reference to the db for saving; get it through the app context
                    cx.update(|cx| {
                        ClipboardHistory::add_entry(text, cx);
                    });
                }
            }
        }
    })
    .detach();

    // Initialize last known clipboard text from current clipboard content
    if let Some(item) = cx.read_from_clipboard() {
        if let Some(text) = item.text() {
            *LAST_CLIPBOARD_TEXT.lock() = Some(text);
        }
    }
}
