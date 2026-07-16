//! Persistent cross-session memory for the Zed agent.
//!
//! Memory stores structured facts that survive beyond a single thread
//! session — user preferences, project conventions, environment quirks,
//! and lessons learned. This is the foundation for self-learning: skills
//! are created from repeated patterns, and the curator consolidates
//! overlapping facts over time.
//!
//! ## Storage format
//!
//! Facts are stored in `~/.zed/memory.jsonl` — one JSON object per line.
//! Each fact has a `key` (unique slug), `value` (the content), optional
//! `category` for grouping, optional `tags` for search, and an embedded
//! `created_at` / `updated_at` timestamp.
//!
//! ## TODO (Phase 2)
//!
//! - SQLite-backed store for better query performance at scale
//! - Curator background task that observes thread outcomes and auto-creates
//!   skill entries from repeating patterns
//! - Cross-device sync via file watch

use anyhow::{Context as _, Result};
use collections::HashMap;
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use util::paths::home_dir;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// A single memory fact that persists across sessions.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MemoryFact {
    /// Unique slug for this fact, e.g. `"user-prefers-tabs"`.
    pub key: String,
    /// The fact content — free-form text the model reads directly.
    pub value: String,
    /// Optional grouping category, e.g. `"preference"`, `"convention"`.
    #[serde(default)]
    pub category: Option<String>,
    /// Tags for search/filter, e.g. `["zed", "keybindings"]`.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Unix-millis timestamp of first creation.
    pub created_at: u64,
    /// Unix-millis timestamp of last update.
    pub updated_at: u64,
}

/// The shape returned to the model from `memory_search`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySearchResult {
    pub facts: Vec<MemoryFact>,
    pub total: usize,
}

// ---------------------------------------------------------------------------
// Store trait
// ---------------------------------------------------------------------------

/// Persistence layer for memory facts.
pub trait MemoryStore: Send + Sync {
    /// Write (insert or update) a fact identified by its `key`.
    fn write(&self, key: String, value: String, category: Option<String>, tags: Vec<String>);

    /// Retrieve a fact by its exact key.
    fn get(&self, key: &str) -> Option<MemoryFact>;

    /// Search facts whose key, value, category, or tags match `query`
    /// (case-insensitive substring match).
    fn search(&self, query: &str) -> Vec<MemoryFact>;

    /// Delete a fact by key.
    fn delete(&self, key: &str) -> bool;

    /// List all facts, optionally filtered by category.
    fn all(&self, category: Option<&str>) -> Vec<MemoryFact>;
}

// ---------------------------------------------------------------------------
// JSONL file store
// ---------------------------------------------------------------------------

/// A simple JSONL-backed memory store.
///
/// Every mutation appends to the file (no compaction yet — see the
/// `TODO` above). The file is re-read on every search so multiple
/// agent instances share the fact base even without an in-memory cache.
pub struct JsonFileMemoryStore {
    path: PathBuf,
    /// In-memory index: key → fact. Kept in sync with the file.
    index: Mutex<HashMap<String, MemoryFact>>,
}

impl JsonFileMemoryStore {
    /// Create or open the store at `~/.zed/memory.jsonl`.
    pub fn global() -> Arc<Self> {
        let path = home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".zed")
            .join("memory.jsonl");

        let store = Self {
            path,
            index: Mutex::new(HashMap::new()),
        };
        store.reload();
        Arc::new(store)
    }

    /// Create a store at a custom path (for tests).
    pub fn new(path: PathBuf) -> Self {
        let store = Self {
            path,
            index: Mutex::new(HashMap::new()),
        };
        store.reload();
        store
    }

    /// Reload the in-memory index from disk.
    fn reload(&self) {
        let content = std::fs::read_to_string(&self.path).unwrap_or_default();
        let mut index = HashMap::new();
        for line in content.lines() {
            if let Ok(fact) = serde_json::from_str::<MemoryFact>(line) {
                index.insert(fact.key.clone(), fact);
            }
        }
        *self.index.lock() = index;
    }

    fn now_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64
    }

    fn append_fact(&self, fact: &MemoryFact) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let line = serde_json::to_string(fact).unwrap_or_default();
        // Use std::io::Write for atomic-ish append
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            use std::io::Write;
            let _ = writeln!(f, "{line}");
        }
    }

    /// Rewrite the entire file from the in-memory index.
    /// Called after a delete to remove the line from the file.
    fn rewrite_all(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let index = self.index.lock();
        let mut lines = Vec::new();
        for fact in index.values() {
            if let Ok(line) = serde_json::to_string(fact) {
                lines.push(line);
            }
        }
        let content = lines.join("\n");
        let _ = std::fs::write(&self.path, content);
    }
}

impl MemoryStore for JsonFileMemoryStore {
    fn write(
        &self,
        key: String,
        value: String,
        category: Option<String>,
        tags: Vec<String>,
    ) {
        let now = Self::now_millis();
        let mut index = self.index.lock();

        let fact = if let Some(existing) = index.get_mut(&key) {
            existing.value = value;
            existing.category = category;
            existing.tags = tags;
            existing.updated_at = now;
            existing.clone()
        } else {
            let fact = MemoryFact {
                key: key.clone(),
                value,
                category,
                tags,
                created_at: now,
                updated_at: now,
            };
            index.insert(key, fact.clone());
            fact
        };
        // Drop the lock before file I/O
        drop(index);
        self.append_fact(&fact);
    }

    fn get(&self, key: &str) -> Option<MemoryFact> {
        self.index.lock().get(key).cloned()
    }

    fn search(&self, query: &str) -> Vec<MemoryFact> {
        let q = query.to_lowercase();
        let index = self.index.lock();
        let mut results: Vec<&MemoryFact> = index
            .values()
            .filter(|f| {
                f.key.to_lowercase().contains(&q)
                    || f.value.to_lowercase().contains(&q)
                    || f.category
                        .as_ref()
                        .is_some_and(|c| c.to_lowercase().contains(&q))
                    || f.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect();
        // Most recently updated first
        results.sort_by_key(|f| std::cmp::Reverse(f.updated_at));
        results.into_iter().cloned().collect()
    }

    fn delete(&self, key: &str) -> bool {
        let mut index = self.index.lock();
        if index.remove(key).is_some() {
            drop(index);
            self.rewrite_all();
            true
        } else {
            false
        }
    }

    fn all(&self, category: Option<&str>) -> Vec<MemoryFact> {
        let index = self.index.lock();
        let mut facts: Vec<&MemoryFact> = match category {
            Some(cat) => index
                .values()
                .filter(|f| f.category.as_deref() == Some(cat))
                .collect(),
            None => index.values().collect(),
        };
        facts.sort_by_key(|f| std::cmp::Reverse(f.updated_at));
        facts.into_iter().cloned().collect()
    }
}

// ---------------------------------------------------------------------------
// Global singleton
// ---------------------------------------------------------------------------

use std::sync::LazyLock;

static GLOBAL_MEMORY_STORE: LazyLock<Arc<JsonFileMemoryStore>> =
    LazyLock::new(JsonFileMemoryStore::global);

/// Access the global memory store singleton.
pub fn global_store() -> Arc<JsonFileMemoryStore> {
    GLOBAL_MEMORY_STORE.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_write_and_get() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileMemoryStore::new(dir.path().join("test.jsonl"));

        store.write("prefers-dark".into(), "true".into(), Some("preference".into()), vec!["theme".into()]);

        let fact = store.get("prefers-dark").unwrap();
        assert_eq!(fact.value, "true");
        assert_eq!(fact.category.unwrap(), "preference");
    }

    #[test]
    fn test_search() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileMemoryStore::new(dir.path().join("test.jsonl"));

        store.write("lang-rust".into(), "Uses Rust for backend".into(), None, vec!["language".into()]);
        store.write("lang-py".into(), "Uses Python for scripts".into(), None, vec!["language".into()]);

        let results = store.search("rust");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "lang-rust");
    }

    #[test]
    fn test_delete() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileMemoryStore::new(dir.path().join("test.jsonl"));

        store.write("test-key".into(), "test-value".into(), None, vec![]);
        assert!(store.get("test-key").is_some());

        store.delete("test-key");
        assert!(store.get("test-key").is_none());
    }

    #[test]
    fn test_persistence_across_reload() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("persist.jsonl");

        // Write in one instance
        {
            let store = JsonFileMemoryStore::new(path.clone());
            store.write("stored".into(), "persisted".into(), None, vec![]);
        }

        // Read in a new instance (same file)
        let store = JsonFileMemoryStore::new(path);
        let fact = store.get("stored").unwrap();
        assert_eq!(fact.value, "persisted");
    }
}
