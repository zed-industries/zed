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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecentEntryId {
    Thread(ThreadId),
    Context(Arc<Path>),
}

#[derive(Clone, Debug)]
pub struct RecentEntry {
    pub id: RecentEntryId,
    pub title: SharedString,
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
                entries.push(RecentEntry {
                    id: RecentEntryId::Thread(thread.id),
                    title: thread.summary.clone(),
                });
            }
            for context in context_store.update(cx, |this, _cx| this.contexts()) {
                entries.push(RecentEntry {
                    id: RecentEntryId::Context(context.path.into()),
                    title: context.title.into(),
                });
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
        self.recently_opened_entries
            .retain(|old_entry| old_entry.id != entry.id);
        self.recently_opened_entries.push(entry);
    }

    pub fn recently_opened_entries(
        &self,
        limit: usize,
        filter: impl Fn(&RecentEntry) -> bool,
        _cx: &mut Context<Self>,
    ) -> Vec<RecentEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return Vec::new();
        }

        let mut entries = Vec::with_capacity(limit);
        for entry in self.recently_opened_entries.iter().rev() {
            if filter(entry) {
                entries.push(entry.clone());
                if entries.len() == limit {
                    break;
                }
            }
        }

        entries
    }
}
