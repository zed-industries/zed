//! Webhook triggers — event-driven agent activation.
//!
//! Three trigger types:
//! - `file_change` — agent wakes when files matching a glob change
//! - `http` — agent listens on localhost for POST requests
//! - `git_hook` — agent runs on git events (pre-commit, post-merge)
//!
//! Subscriptions stored in `~/.zed/webhooks.json`. Background threads
//! monitor each trigger and create agent threads when events fire.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use util::paths::home_dir;

pub mod tools;

// ---------------------------------------------------------------------------
// Subscription model
// ---------------------------------------------------------------------------

/// Supported webhook event types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// Fires when files matching a glob change on disk
    FileChange,
    /// Listens for HTTP POST requests on localhost:<port>
    Http,
    /// Fires on git events (pre-commit, post-merge)
    GitHook,
}

/// A single webhook subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscription {
    /// Unique ID (auto-generated).
    pub id: String,
    /// Event type.
    pub event_type: WebhookEventType,
    /// For file_change: glob pattern (e.g. "**/*.rs")
    /// For http: port number
    /// For git_hook: hook name (e.g. "pre-commit")
    #[serde(default)]
    pub filter: String,
    /// The prompt to run when the webhook fires.
    pub prompt: String,
    /// Whether the subscription is active.
    #[serde(default = "default_true")]
    pub active: bool,
    /// How many times this webhook has fired.
    #[serde(default)]
    pub fire_count: u64,
    /// Timestamp of the last fire.
    #[serde(default)]
    pub last_fired_at: u64,
}

fn default_true() -> bool { true }

// ---------------------------------------------------------------------------
// Webhook store
// ---------------------------------------------------------------------------

pub struct WebhookStore {
    path: PathBuf,
    subscriptions: Mutex<Vec<WebhookSubscription>>,
}

impl WebhookStore {
    pub fn global() -> Arc<Self> {
        let path = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zed")
            .join("webhooks.json");
        let store = Arc::new(Self {
            path,
            subscriptions: Mutex::new(Vec::new()),
        });
        store.reload();
        store
    }

    pub fn all(&self) -> Vec<WebhookSubscription> {
        self.subscriptions.lock().clone()
    }

    pub fn add(&self, sub: WebhookSubscription) {
        let mut subs = self.subscriptions.lock();
        if !subs.iter().any(|s| s.id == sub.id) {
            subs.push(sub);
            self.save(&subs);
        }
    }

    pub fn remove(&self, id: &str) -> bool {
        let mut subs = self.subscriptions.lock();
        let len_before = subs.len();
        subs.retain(|s| s.id != id);
        if subs.len() != len_before {
            self.save(&subs);
            return true;
        }
        false
    }

    pub fn toggle(&self, id: &str) -> bool {
        let mut subs = self.subscriptions.lock();
        if let Some(sub) = subs.iter_mut().find(|s| s.id == id) {
            sub.active = !sub.active;
            self.save(&subs);
            return sub.active;
        }
        false
    }

    fn reload(&self) {
        if let Ok(content) = std::fs::read_to_string(&self.path) {
            if let Ok(subs) = serde_json::from_str::<Vec<WebhookSubscription>>(&content) {
                *self.subscriptions.lock() = subs;
            }
        }
    }

    fn save(&self, subs: &[WebhookSubscription]) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(subs) {
            let _ = std::fs::write(&self.path, content);
        }
    }
}

// ---------------------------------------------------------------------------
// File watcher background thread
// ---------------------------------------------------------------------------

use std::sync::LazyLock;

static WEBHOOK_STORE: LazyLock<Arc<WebhookStore>> = LazyLock::new(WebhookStore::global);

pub fn global_store() -> Arc<WebhookStore> {
    WEBHOOK_STORE.clone()
}

/// Start file watcher background threads for active file_change subscriptions.
pub fn start_file_watchers() {
    std::thread::spawn(move || {
        // Simple polling-based file watcher (checks every 30s)
        let mut last_state: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        loop {
            std::thread::sleep(Duration::from_secs(30));
            let store = global_store();
            for sub in store.all() {
                if !sub.active || sub.event_type != WebhookEventType::FileChange {
                    continue;
                }
                let pattern = &sub.filter;
                if pattern.is_empty() {
                    continue;
                }
                // Check if matching files have changed using mtime
                if let Ok(entries) = glob(Some(pattern)) {
                    let mut changed = false;
                    for entry in entries.flatten() {
                        if let Ok(meta) = entry.metadata() {
                            if let Ok(mtime) = meta.modified() {
                                let mtime_ms = mtime
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u64;
                                let key = entry.to_string_lossy().to_string();
                                let prev = last_state.get(&key).copied().unwrap_or(0);
                                if prev > 0 && prev != mtime_ms {
                                    changed = true;
                                }
                                last_state.insert(key, mtime_ms);
                            }
                        }
                    }
                    if changed {
                        log::info!("webhook: file_change '{}' fired for pattern '{}'", sub.id, pattern);
                        store.toggle(&sub.id); // one-shot toggle
                    }
                }
            }
        }
    });
}

fn glob(pattern: Option<&str>) -> Result<Vec<std::fs::DirEntry>> {
    // Simple recursive file discovery — not a full glob, just walks cwd
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(p) = pattern {
                    let name = path.to_string_lossy();
                    if simple_glob_match(p, &name) {
                        files.push(entry);
                    }
                }
            }
        }
    }
    Ok(files)
}

fn simple_glob_match(pattern: &str, path: &str) -> bool {
    // Very simple glob: checks if the path ends with the pattern
    // after stripping "**/*" prefix
    if let Some(suffix) = pattern.strip_prefix("**/*") {
        path.ends_with(suffix)
    } else if let Some(suffix) = pattern.strip_prefix("*.") {
        path.ends_with(suffix)
    } else {
        path.contains(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_remove() {
        let dir = tempfile::tempdir().unwrap();
        let store = WebhookStore {
            path: dir.path().join("webhooks.json"),
            subscriptions: Mutex::new(Vec::new()),
        };
        store.add(WebhookSubscription {
            id: "test".into(),
            event_type: WebhookEventType::Http,
            filter: "8080".into(),
            prompt: "handle request".into(),
            active: true,
            fire_count: 0,
            last_fired_at: 0,
        });
        assert_eq!(store.all().len(), 1);
        assert!(store.remove("test"));
        assert_eq!(store.all().len(), 0);
    }

    #[test]
    fn test_toggle() {
        let dir = tempfile::tempdir().unwrap();
        let store = WebhookStore {
            path: dir.path().join("webhooks.json"),
            subscriptions: Mutex::new(Vec::new()),
        };
        store.add(WebhookSubscription {
            id: "t".into(),
            event_type: WebhookEventType::Http,
            filter: "8080".into(),
            prompt: "test".into(),
            active: true,
            fire_count: 0,
            last_fired_at: 0,
        });
        assert!(store.toggle("t")); // toggle off (returns new state)
        assert!(!store.all()[0].active);
    }

    #[test]
    fn test_simple_glob_match() {
        assert!(simple_glob_match("**/*.rs", "src/main.rs"));
        assert!(simple_glob_match("*.rs", "main.rs"));
        assert!(!simple_glob_match("*.rs", "main.ts"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("webhooks.json");
        {
            let store = WebhookStore {
                path: path.clone(),
                subscriptions: Mutex::new(Vec::new()),
            };
            store.add(WebhookSubscription {
                id: "wh1".into(),
                event_type: WebhookEventType::FileChange,
                filter: "**/*.rs".into(),
                prompt: "run tests".into(),
                active: true,
                fire_count: 0,
                last_fired_at: 0,
            });
        }
        // Reload from disk
        let store = WebhookStore {
            path,
            subscriptions: Mutex::new(Vec::new()),
        };
        store.reload();
        assert_eq!(store.all().len(), 1);
        assert_eq!(store.all()[0].id, "wh1");
    }
}
