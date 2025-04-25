use assistant_context_editor::SavedContextMetadata;
use chrono::{DateTime, Utc};
use gpui::{Entity, prelude::*};

use crate::thread_store::{SerializedThreadMetadata, ThreadStore};

#[derive(Debug)]
pub enum HistoryEntry {
    Thread(SerializedThreadMetadata),
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
    _subscriptions: Vec<gpui::Subscription>,
}

impl HistoryStore {
    pub fn new(
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context_editor::ContextStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread_store, |_, _, cx| cx.notify()),
            cx.observe(&context_store, |_, _, cx| cx.notify()),
        ];

        Self {
            thread_store,
            context_store,
            _subscriptions: subscriptions,
        }
    }

    pub fn entries(&self, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        let mut history_entries = Vec::new();

        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return history_entries;
        }

        for thread in self
            .thread_store
            .update(cx, |this, _cx| this.reverse_chronological_threads())
        {
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
