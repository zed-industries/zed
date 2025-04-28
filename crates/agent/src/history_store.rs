use std::{path::Path, sync::Arc};

use assistant_context_editor::SavedContextMetadata;
use chrono::{DateTime, Utc};
use gpui::{Entity, prelude::*};
use ui::SharedString;

use crate::{
    thread::ThreadId,
    thread_store::{SerializedThreadMetadata, ThreadStore},
};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum RecentEntry {
    Thread(ThreadId, SharedString),
    Context(Arc<Path>, SharedString),
}

impl RecentEntry {
    pub fn title(&self) -> SharedString {
        match self {
            RecentEntry::Thread(_, title) => title.clone(),
            RecentEntry::Context(_, title) => title.clone(),
        }
    }
}

pub struct HistoryStore {
    thread_store: Entity<ThreadStore>,
    context_store: Entity<assistant_context_editor::ContextStore>,
    recently_opened_entries: Vec<RecentEntry>,
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

        let recently_opened_entries = {
            let mut entries = Vec::new();
            for thread in thread_store.update(cx, |this, _cx| this.reverse_chronological_threads())
            {
                entries.push(RecentEntry::Thread(thread.id, thread.summary.clone()));
            }
            for context in context_store.update(cx, |this, _cx| this.contexts()) {
                entries.push(RecentEntry::Context(
                    context.path.into(),
                    context.title.into(),
                ));
            }
            entries
        };

        Self {
            thread_store,
            context_store,
            recently_opened_entries,
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

    pub fn push_recently_opened_entry(&mut self, entry: RecentEntry, _cx: &mut Context<Self>) {
        self.recently_opened_entries.push(entry);
    }

    pub fn recently_opened_entries(
        &self,
        limit: usize,
        _cx: &mut Context<Self>,
    ) -> Vec<RecentEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return Vec::new();
        }

        let start = self.recently_opened_entries.len().saturating_sub(limit);
        let mut entries = self.recently_opened_entries[start..].to_owned();
        entries.reverse();
        entries
    }
}
