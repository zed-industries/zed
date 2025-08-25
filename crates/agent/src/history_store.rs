use crate::{
    ThreadId,
    thread_store::{SerializedThreadMetadata, ThreadStore},
};
use anyhow::{Context as _, Result};
use assistant_context::SavedContextMetadata;
use chrono::{DateTime, Utc};
use gpui::{App, AsyncApp, Entity, SharedString, Task, prelude::*};
use itertools::Itertools;
use paths::contexts_dir;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, path::Path, sync::Arc, time::Duration};
use util::ResultExt as _;

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

    pub fn id(&self) -> HistoryEntryId {
        match self {
            HistoryEntry::Thread(thread) => HistoryEntryId::Thread(thread.id.clone()),
            HistoryEntry::Context(context) => HistoryEntryId::Context(context.path.clone()),
        }
    }

    pub fn title(&self) -> &SharedString {
        match self {
            HistoryEntry::Thread(thread) => &thread.summary,
            HistoryEntry::Context(context) => &context.title,
        }
    }
}

/// Generic identifier for a history entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HistoryEntryId {
    Thread(ThreadId),
    Context(Arc<Path>),
}

#[derive(Serialize, Deserialize)]
enum SerializedRecentOpen {
    Thread(String),
    ContextName(String),
    /// Old format which stores the full path
    Context(String),
}

pub struct HistoryStore {
    thread_store: Entity<ThreadStore>,
    context_store: Entity<assistant_context::ContextStore>,
    recently_opened_entries: VecDeque<HistoryEntryId>,
    _subscriptions: Vec<gpui::Subscription>,
    _save_recently_opened_entries_task: Task<()>,
}

impl HistoryStore {
    pub fn new(
        thread_store: Entity<ThreadStore>,
        context_store: Entity<assistant_context::ContextStore>,
        initial_recent_entries: impl IntoIterator<Item = HistoryEntryId>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread_store, |_, _, cx| cx.notify()),
            cx.observe(&context_store, |_, _, cx| cx.notify()),
        ];

        cx.spawn(async move |this, cx| {
            let entries = Self::load_recently_opened_entries(cx).await.log_err()?;
            this.update(cx, |this, _| {
                this.recently_opened_entries
                    .extend(
                        entries.into_iter().take(
                            MAX_RECENTLY_OPENED_ENTRIES
                                .saturating_sub(this.recently_opened_entries.len()),
                        ),
                    );
            })
            .ok()
        })
        .detach();

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

        history_entries.extend(
            self.thread_store
                .read(cx)
                .reverse_chronological_threads()
                .cloned()
                .map(HistoryEntry::Thread),
        );
        history_entries.extend(
            self.context_store
                .read(cx)
                .unordered_contexts()
                .cloned()
                .map(HistoryEntry::Context),
        );

        history_entries.sort_unstable_by_key(|entry| std::cmp::Reverse(entry.updated_at()));
        history_entries
    }

    pub fn recent_entries(&self, limit: usize, cx: &mut Context<Self>) -> Vec<HistoryEntry> {
        self.entries(cx).into_iter().take(limit).collect()
    }

    pub fn recently_opened_entries(&self, cx: &App) -> Vec<HistoryEntry> {
        #[cfg(debug_assertions)]
        if std::env::var("ZED_SIMULATE_NO_THREAD_HISTORY").is_ok() {
            return Vec::new();
        }

        let thread_entries = self
            .thread_store
            .read(cx)
            .reverse_chronological_threads()
            .flat_map(|thread| {
                self.recently_opened_entries
                    .iter()
                    .enumerate()
                    .flat_map(|(index, entry)| match entry {
                        HistoryEntryId::Thread(id) if &thread.id == id => {
                            Some((index, HistoryEntry::Thread(thread.clone())))
                        }
                        _ => None,
                    })
            });

        let context_entries =
            self.context_store
                .read(cx)
                .unordered_contexts()
                .flat_map(|context| {
                    self.recently_opened_entries
                        .iter()
                        .enumerate()
                        .flat_map(|(index, entry)| match entry {
                            HistoryEntryId::Context(path) if &context.path == path => {
                                Some((index, HistoryEntry::Context(context.clone())))
                            }
                            _ => None,
                        })
                });

        thread_entries
            .chain(context_entries)
            // optimization to halt iteration early
            .take(self.recently_opened_entries.len())
            .sorted_unstable_by_key(|(index, _)| *index)
            .map(|(_, entry)| entry)
            .collect()
    }

    fn save_recently_opened_entries(&mut self, cx: &mut Context<Self>) {
        let serialized_entries = self
            .recently_opened_entries
            .iter()
            .filter_map(|entry| match entry {
                HistoryEntryId::Context(path) => path.file_name().map(|file| {
                    SerializedRecentOpen::ContextName(file.to_string_lossy().to_string())
                }),
                HistoryEntryId::Thread(id) => Some(SerializedRecentOpen::Thread(id.to_string())),
            })
            .collect::<Vec<_>>();

        self._save_recently_opened_entries_task = cx.spawn(async move |_, cx| {
            cx.background_executor()
                .timer(SAVE_RECENTLY_OPENED_ENTRIES_DEBOUNCE)
                .await;
            cx.background_spawn(async move {
                let path = paths::data_dir().join(NAVIGATION_HISTORY_PATH);
                let content = serde_json::to_string(&serialized_entries)?;
                std::fs::write(path, content)?;
                anyhow::Ok(())
            })
            .await
            .log_err();
        });
    }

    fn load_recently_opened_entries(cx: &AsyncApp) -> Task<Result<Vec<HistoryEntryId>>> {
        cx.background_spawn(async move {
            let path = paths::data_dir().join(NAVIGATION_HISTORY_PATH);
            let contents = match smol::fs::read_to_string(path).await {
                Ok(it) => it,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    return Ok(Vec::new());
                }
                Err(e) => {
                    return Err(e)
                        .context("deserializing persisted agent panel navigation history");
                }
            };
            let entries = serde_json::from_str::<Vec<SerializedRecentOpen>>(&contents)
                .context("deserializing persisted agent panel navigation history")?
                .into_iter()
                .take(MAX_RECENTLY_OPENED_ENTRIES)
                .flat_map(|entry| match entry {
                    SerializedRecentOpen::Thread(id) => {
                        Some(HistoryEntryId::Thread(id.as_str().into()))
                    }
                    SerializedRecentOpen::ContextName(file_name) => Some(HistoryEntryId::Context(
                        contexts_dir().join(file_name).into(),
                    )),
                    SerializedRecentOpen::Context(path) => {
                        Path::new(&path).file_name().map(|file_name| {
                            HistoryEntryId::Context(contexts_dir().join(file_name).into())
                        })
                    }
                })
                .collect::<Vec<_>>();
            Ok(entries)
        })
    }

    pub fn push_recently_opened_entry(&mut self, entry: HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != &entry);
        self.recently_opened_entries.push_front(entry);
        self.recently_opened_entries
            .truncate(MAX_RECENTLY_OPENED_ENTRIES);
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_thread(&mut self, id: ThreadId, cx: &mut Context<Self>) {
        self.recently_opened_entries.retain(
            |entry| !matches!(entry, HistoryEntryId::Thread(thread_id) if thread_id == &id),
        );
        self.save_recently_opened_entries(cx);
    }

    pub fn replace_recently_opened_text_thread(
        &mut self,
        old_path: &Path,
        new_path: &Arc<Path>,
        cx: &mut Context<Self>,
    ) {
        for entry in &mut self.recently_opened_entries {
            match entry {
                HistoryEntryId::Context(path) if path.as_ref() == old_path => {
                    *entry = HistoryEntryId::Context(new_path.clone());
                    break;
                }
                _ => {}
            }
        }
        self.save_recently_opened_entries(cx);
    }

    pub fn remove_recently_opened_entry(&mut self, entry: &HistoryEntryId, cx: &mut Context<Self>) {
        self.recently_opened_entries
            .retain(|old_entry| old_entry != entry);
        self.save_recently_opened_entries(cx);
    }
}
