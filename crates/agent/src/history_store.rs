use std::{collections::VecDeque, path::Path};

use anyhow::{Context as _, anyhow};
use assistant_context_editor::{AssistantContext, SavedContextMetadata};
use chrono::{DateTime, Utc};
use futures::future::{TryFutureExt as _, join_all};
use gpui::{Entity, Task, prelude::*};
use serde::{Deserialize, Serialize};
use smol::future::FutureExt;
use std::time::Duration;
use ui::{App, SharedString};
use util::ResultExt as _;

use crate::{
    Thread,
    thread::ThreadId,
    thread_store::{SerializedThreadMetadata, ThreadStore},
};

const MAX_RECENTLY_OPENED_ENTRIES: usize = 6;
const NAVIGATION_HISTORY_PATH: &str = "agent-navigation-history.json";
const SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE: Duration = Duration::from_millis(50);

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
pub(crate) enum RecentEntry {
    Thread(Entity<Thread>),
    Context(Entity<AssistantContext>),
}

impl RecentEntry {
    pub(crate) fn summary(&self, cx: &App) -> SharedString {
        match self {
            RecentEntry::Thread(thread) => thread.read(cx).summary_or_default(),
            RecentEntry::Context(context) => context.read(cx).summary_or_default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum SerializedRecentEntry {
    Thread(String),
    Context(String),
}

pub struct HistoryStore {
    thread_store: Entity<ThreadStore>,
    context_store: Entity<assistant_context_editor::ContextStore>,
    recently_opened_entries: VecDeque<RecentEntry>,
    _subscriptions: Vec<gpui::Subscription>,
    _save_recently_opened_entries_task: Task<()>,
}

impl HistoryStore {
    pub fn new(
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context_editor::ContextStore>,
        initial_recent_entries: impl IntoIterator<Item = RecentEntry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread_store, |_, _, cx| cx.notify()),
            cx.observe(&context_store, |_, _, cx| cx.notify()),
        ];

        cx.spawn({
            let thread_store = thread_store.downgrade();
            let context_store = context_store.downgrade();
            async move |this, cx| {
                let path = paths::data_dir().join(NAVIGATION_HISTORY_PATH);
                let contents = cx
                    .background_spawn(async move { std::fs::read_to_string(path) })
                    .await
                    .context("reading persisted agent panel navigation history")?;
                let entries = serde_json::from_str::<Vec<SerializedRecentEntry>>(dbg!(&contents))
                    .context("deserializing persisted agent panel navigation history")?
                    .into_iter()
                    .take(MAX_RECENTLY_OPENED_ENTRIES)
                    .map(|serialized| match serialized {
                        SerializedRecentEntry::Thread(id) => thread_store
                            .update(cx, |thread_store, cx| {
                                thread_store
                                    .open_thread(&ThreadId::from(id.as_str()), cx)
                                    .map_ok(RecentEntry::Thread)
                                    .boxed()
                            })
                            .unwrap_or_else(|_| async { Err(anyhow!("no thread store")) }.boxed()),
                        SerializedRecentEntry::Context(id) => context_store
                            .update(cx, |context_store, cx| {
                                context_store
                                    .open_local_context(Path::new(&id).into(), cx)
                                    .map_ok(RecentEntry::Context)
                                    .boxed()
                            })
                            .unwrap_or_else(|_| async { Err(anyhow!("no context store")) }.boxed()),
                    });
                let entries = join_all(entries)
                    .await
                    .into_iter()
                    .filter_map(|result| result.log_err())
                    .collect::<VecDeque<_>>();

                this.update(cx, |this, _| {
                    this.recently_opened_entries.extend(entries);
                    this.recently_opened_entries
                        .truncate(MAX_RECENTLY_OPENED_ENTRIES);
                })
                .ok();

                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);

        Self {
            thread_store,
            context_store,
            recently_opened_entries: initial_recent_entries.into_iter().collect(),
            _subscriptions: subscriptions,
            _save_recently_opened_entries_task: Task::ready(()),
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

    fn save_recently_opened_entries(&mut self, cx: &mut Context<Self>) {
        let serialized_entries = self
            .recently_opened_entries
            .iter()
            .filter_map(|entry| match entry {
                RecentEntry::Context(context) => Some(SerializedRecentEntry::Context(
                    context.read(cx).path()?.to_str()?.to_owned(),
                )),
                RecentEntry::Thread(thread) => Some(SerializedRecentEntry::Thread(
                    thread.read(cx).id().to_string(),
                )),
            })
            .collect::<Vec<_>>();

        self._save_recently_opened_entries_task = cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE)
                .await;
            cx.background_spawn(async move {
                let path = paths::data_dir().join(NAVIGATION_HISTORY_PATH);
                let content = serde_json::to_string(&serialized_entries)?;
                dbg!(&content);
                std::fs::write(path, content)?;
                anyhow::Ok(())
            })
            .await
            .log_err();
        });
    }

    pub fn push_recently_opened_entry(&mut self, entry: RecentEntry, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != &entry);
        self.recently_opened_entries.push_front(entry);
        self.recently_opened_entries
            .truncate(MAX_RECENTLY_OPENED_ENTRIES);
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_entry(&mut self, entry: &RecentEntry, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != entry);
        self.save_recently_opened_entries(cx);
    }

    pub fn recently_opened_entries(&self, _cx: &mut Context<Self>) -> VecDeque<RecentEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return VecDeque::new();
        }

        self.recently_opened_entries.clone()
    }
}
