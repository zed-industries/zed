use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use fs::Fs;
use gpui::{AsyncApp, Context, Entity, EventEmitter, WeakEntity};
use language::{Grammar, LanguageRegistry};
use symbol_index::{SymbolIndex, SymbolLocation};
use worktree::{EntryKind, PathChange, UpdatedEntriesSet, WorktreeId};
use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};

pub enum SymbolIndexEvent {
    /// Initial full-worktree scan completed.
    Indexed,
    /// Index content changed (file added/updated/removed).
    UpdatedEntries,
}

pub struct SymbolIndexManager {
    index: SymbolIndex,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    worktree_store: WeakEntity<WorktreeStore>,
    is_indexing: bool,
    indexed_file_count: usize,
    total_file_count: usize,
    /// Paths currently being indexed in the background, to avoid duplicate work.
    pending_index: HashSet<PathBuf>,
    _subscriptions: Vec<gpui::Subscription>,
}

impl EventEmitter<SymbolIndexEvent> for SymbolIndexManager {}

impl SymbolIndexManager {
    pub fn new(
        languages: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        worktree_store: &Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription =
            cx.subscribe(worktree_store, |this, _store, event, cx| {
                this.on_worktree_store_event(event, cx);
            });

        let mut manager = Self {
            index: SymbolIndex::new(),
            languages,
            fs,
            worktree_store: worktree_store.downgrade(),
            is_indexing: false,
            indexed_file_count: 0,
            total_file_count: 0,
            pending_index: HashSet::new(),
            _subscriptions: vec![subscription],
        };

        manager.start_indexing(worktree_store, cx);
        manager
    }

    fn on_worktree_store_event(
        &mut self,
        event: &WorktreeStoreEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeUpdatedEntries(worktree_id, changes) => {
                self.on_updated_entries(*worktree_id, changes, cx);
            }
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                self.remove_worktree(*worktree_id, cx);
            }
            _ => {}
        }
    }

    fn start_indexing(
        &mut self,
        worktree_store: &Entity<WorktreeStore>,
        cx: &mut Context<Self>,
    ) {
        let mut file_list: Vec<(WorktreeId, String, PathBuf)> = Vec::new();

        for worktree_entity in worktree_store.read(cx).visible_worktrees(cx) {
            let worktree_id = worktree_entity.read(cx).id();
            let snapshot = worktree_entity.read(cx).snapshot();
            for entry in snapshot.files(false, 0) {
                if entry.kind != EntryKind::File {
                    continue;
                }
                let abs_path = worktree_entity.read(cx).absolutize(entry.path.as_ref());
                // Pre-filter: skip files with no known language
                if self.languages.language_for_file_path(&abs_path).is_none() {
                    continue;
                }
                let rel_path = entry.path.as_ref().as_unix_str().to_string();
                file_list.push((worktree_id, rel_path, abs_path));
            }
        }

        let total = file_list.len();
        self.total_file_count = total;
        self.indexed_file_count = 0;
        self.is_indexing = true;
        cx.notify();

        let languages = self.languages.clone();
        let fs = self.fs.clone();
        let weak_self = cx.weak_entity();

        cx.spawn(async move |_, cx: &mut AsyncApp| {
            use futures::stream::{self, StreamExt};

            // Group files by extension and load each language once.
            let mut by_extension: HashMap<String, Vec<(WorktreeId, String, PathBuf)>> =
                HashMap::new();
            for file in file_list {
                let ext = file
                    .2
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string();
                by_extension.entry(ext).or_default().push(file);
            }

            let mut grammar_cache: HashMap<String, Option<Arc<Grammar>>> = HashMap::new();
            for (ext, files) in &by_extension {
                if files.is_empty() {
                    continue;
                }
                let sample_path = files[0].2.clone();
                let language = match languages.load_language_for_file_path(&sample_path).await {
                    Ok(language) => language,
                    Err(_) => continue,
                };
                let grammar = match language.grammar() {
                    Some(g) if g.outline_config.is_some() => Some(g.clone()),
                    _ => None,
                };
                grammar_cache.insert(ext.clone(), grammar);
            }

            // Flatten back to a file list with pre-resolved grammars.
            let mut file_list_with_grammar: Vec<(SymbolLocation, PathBuf, Arc<Grammar>)> =
                Vec::new();
            for (ext, files) in by_extension {
                let grammar = match grammar_cache.get(&ext) {
                    Some(Some(g)) => g.clone(),
                    _ => continue,
                };
                for (worktree_id, rel_path, abs_path) in files {
                    let location = SymbolLocation {
                        worktree_id: worktree_id.to_proto(),
                        path: Arc::from(rel_path.as_str()),
                    };
                    file_list_with_grammar.push((location, abs_path, grammar.clone()));
                }
            }

            let background = cx.background_executor().clone();
            let concurrency = background.num_cpus() * 2;
            let mut batch: Vec<(SymbolLocation, Vec<symbol_index::ExtractedSymbol>)> = Vec::new();
            let mut indexed = 0usize;

            let mut results = stream::iter(file_list_with_grammar)
                .map(|(location, abs_path, grammar)| {
                    let fs = fs.clone();
                    let background = background.clone();
                    async move {
                        let extracted = background
                            .spawn(async move {
                                let text = match fs.load(&abs_path).await {
                                    Ok(text) => text,
                                    Err(err) => {
                                        log::warn!(
                                            "symbol_index: failed to read {abs_path:?}: {err}"
                                        );
                                        return Vec::new();
                                    }
                                };
                                symbol_index::extract_symbols(&text, &grammar)
                            })
                            .await;
                        Some((location, extracted))
                    }
                })
                .buffer_unordered(concurrency);

            while let Some(result) = results.next().await {
                indexed += 1;
                if let Some((location, extracted)) = result {
                    batch.push((location, extracted));
                }

                if batch.len() >= 64 {
                    let batch_to_flush = std::mem::take(&mut batch);
                    let result = weak_self.update(cx, |this, cx| {
                        this.index.update_files_batch(batch_to_flush);
                        this.indexed_file_count = indexed;
                        cx.emit(SymbolIndexEvent::UpdatedEntries);
                        cx.notify();
                    });
                    if result.is_err() {
                        return;
                    }
                }
            }

            let result = weak_self.update(cx, |this, cx| {
                if !batch.is_empty() {
                    this.index.update_files_batch(batch);
                }
                this.indexed_file_count = indexed;
                this.is_indexing = false;
                cx.emit(SymbolIndexEvent::Indexed);
                cx.notify();
            });
            if result.is_err() {
                return;
            }
        })
        .detach();
    }

    fn on_updated_entries(
        &mut self,
        worktree_id: WorktreeId,
        changes: &UpdatedEntriesSet,
        cx: &mut Context<Self>,
    ) {
        let worktree_store = match self.worktree_store.upgrade() {
            Some(store) => store,
            None => return,
        };

        let mut to_remove: Vec<SymbolLocation> = Vec::new();
        let mut to_index: Vec<(SymbolLocation, PathBuf)> = Vec::new();

        for (rel_path, _entry_id, change) in changes.iter() {
            let location = SymbolLocation {
                worktree_id: worktree_id.to_proto(),
                path: Arc::from(rel_path.as_ref().as_unix_str()),
            };

            match change {
                PathChange::Removed => {
                    to_remove.push(location);
                }
                PathChange::Added
                | PathChange::Updated
                | PathChange::AddedOrUpdated
                | PathChange::Loaded => {
                    if let Some(worktree) = worktree_store.read(cx).worktree_for_id(worktree_id, cx)
                    {
                        let abs_path = worktree.read(cx).absolutize(rel_path.as_ref());
                        to_index.push((location, abs_path));
                    }
                }
            }
        }

        // Deduplicate: skip files already being indexed.
        to_index.retain(|(_, abs_path)| {
            if self.pending_index.contains(abs_path) {
                false
            } else {
                self.pending_index.insert(abs_path.clone());
                true
            }
        });

        self.index.remove_files_batch(&to_remove);

        if !to_remove.is_empty() || !to_index.is_empty() {
            cx.emit(SymbolIndexEvent::UpdatedEntries);
            cx.notify();
        }

        if to_index.is_empty() {
            return;
        }

        let languages = self.languages.clone();
        let fs = self.fs.clone();
        let weak_self = cx.weak_entity();

        cx.spawn(async move |_, cx: &mut AsyncApp| {
            use futures::stream::{self, StreamExt};

            let background = cx.background_executor().clone();
            let mut batch: Vec<(SymbolLocation, Vec<symbol_index::ExtractedSymbol>)> = Vec::new();

            let mut results = stream::iter(to_index)
                .map(|(location, abs_path)| {
                    let languages = languages.clone();
                    let fs = fs.clone();
                    let background = background.clone();
                    async move {
                        let language =
                            match languages.load_language_for_file_path(&abs_path).await {
                                Ok(language) => language,
                                Err(_) => return (abs_path, None),
                            };
                        let grammar = match language.grammar() {
                            Some(g) if g.outline_config.is_some() => g.clone(),
                            _ => return (abs_path, None),
                        };

                        let abs_path_for_io = abs_path.clone();
                        let extracted = background
                            .spawn(async move {
                                let text = match fs.load(&abs_path_for_io).await {
                                    Ok(text) => text,
                                    Err(err) => {
                                        log::warn!(
                                            "symbol_index: failed to read {abs_path_for_io:?}: {err}"
                                        );
                                        return Vec::new();
                                    }
                                };
                                symbol_index::extract_symbols(&text, &grammar)
                            })
                            .await;
                        (abs_path, Some((location, extracted)))
                    }
                })
                .buffer_unordered(4);

            while let Some((abs_path, result)) = results.next().await {
                if let Some((location, extracted)) = result {
                    batch.push((location, extracted));
                }

                let result = weak_self.update(cx, |this, cx| {
                    this.pending_index.remove(&abs_path);
                    if batch.len() >= 32 {
                        let batch_to_flush = std::mem::take(&mut batch);
                        this.index.update_files_batch(batch_to_flush);
                        cx.emit(SymbolIndexEvent::UpdatedEntries);
                        cx.notify();
                    }
                });
                if result.is_err() {
                    return;
                }
            }

            let _ = weak_self.update(cx, |this, cx| {
                if !batch.is_empty() {
                    this.index.update_files_batch(batch);
                }
                cx.emit(SymbolIndexEvent::UpdatedEntries);
                cx.notify();
            });
        })
        .detach();
    }

    fn remove_worktree(&mut self, worktree_id: WorktreeId, cx: &mut Context<Self>) {
        self.index.remove_worktree(worktree_id.to_proto());
        cx.emit(SymbolIndexEvent::UpdatedEntries);
        cx.notify();
    }

    pub fn snapshot(&mut self) -> symbol_index::IndexSnapshot {
        self.index.snapshot()
    }

    pub fn is_indexing(&self) -> bool {
        self.is_indexing
    }

    /// Returns (indexed, total) during initial scan, or None when idle.
    pub fn progress(&self) -> Option<(usize, usize)> {
        if self.is_indexing {
            Some((self.indexed_file_count, self.total_file_count))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }
}
