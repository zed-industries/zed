use assistant_context_editor::SavedContextMetadata;
use chrono::{DateTime, Utc};
use gpui::{prelude::*, Entity};

use crate::thread_store::{SavedThreadMetadata, ThreadStore};

pub enum HistoryEntry {
    Thread(SavedThreadMetadata),
    Context(SavedContextMetadata),
}

impl HistoryEntry {
    pub fn updated_at(&self) -> DateTime<Utc> {
        match self {
            HistoryEntry::Thread(thread) => thread.updated_at,
            HistoryEntry::Context(context) => context.mtime.to_utc(),
        }
    }
}

pub struct HistoryStore {
    thread_store: Entity<ThreadStore>,
    context_store: Entity<assistant_context_editor::ContextStore>,
}

impl HistoryStore {
    pub fn new(
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context_editor::ContextStore>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            thread_store,
            context_store,
        }
    }

    /// Returns the number of history entries.
    pub fn entry_count(&self, cx: &mut Context<Self>) -> usize {
        self.entries(cx).len()
    }

    pub fn entries(&self, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        let mut history_entries = Vec::new();

        for thread in self.thread_store.update(cx, |this, _cx| this.threads()) {
            history_entries.push(HistoryEntry::Thread(thread));
        }

        for context in self.context_store.update(cx, |this, _cx| this.contexts()) {
            history_entries.push(HistoryEntry::Context(context));
        }

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        history_entries
    }

    pub fn recent_entries(&self, limit: usize, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        self.entries(cx).into_iter().take(limit).collect()
    }
}
