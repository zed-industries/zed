use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use fs::Fs;
use futures::stream::{self, StreamExt};
use gpui::{AsyncApp, BackgroundExecutor, Context, Entity, EventEmitter, WeakEntity};
use language::{Grammar, LanguageId, LanguageName, LanguageRegistry};
use symbol_index::{ExtractedSymbol, SymbolIndex, SymbolLocation};
use text::Rope;
use worktree::{EntryKind, PathChange, UpdatedEntriesSet, WorktreeId};

use crate::worktree_store::{WorktreeStore, WorktreeStoreEvent};

/// Number of files to accumulate before publishing their symbols in one
/// batch.
const PUBLISH_BATCH_SIZE: usize = 64;

pub enum SymbolIndexEvent {
    /// Initial full-worktree scan completed.
    Indexed,
    /// Index content changed (file added/updated/removed).
    UpdatedEntries,
}

/// A file whose symbols are being extracted in the background, together with
/// the revision of the change that the read was scheduled for.
#[derive(Clone)]
struct FileToIndex {
    location: SymbolLocation,
    abs_path: PathBuf,
    /// Handle for language detection, carrying the file's worktree and path
    /// so settings can be resolved for the file's location.
    file: Arc<dyn language::File>,
    revision: u64,
}

pub struct SymbolIndexManager {
    index: SymbolIndex,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    worktree_store: WeakEntity<WorktreeStore>,
    is_indexing: bool,
    indexed_file_count: usize,
    total_file_count: usize,
    /// Latest revision of every indexed file, bumped on every observed
    /// change. The result of a read is only published when the file's
    /// revision is still the one the read was scheduled for.
    file_revisions: HashMap<SymbolLocation, u64>,
    /// Files with a read/extract task in flight.
    indexing: HashSet<SymbolLocation>,
    /// Files that changed while a task was in flight; their completions are
    /// stale and the files need to be read again.
    dirty: HashSet<SymbolLocation>,
    /// Results of up-to-date completions, waiting to be published in batch.
    /// Coalesced by file at publication time: only the entry for a file's
    /// latest revision is ever inserted into the index.
    pending_symbols: Vec<(SymbolLocation, u64, LanguageName, Vec<ExtractedSymbol>)>,
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
        let subscription = cx.subscribe(worktree_store, |this, _store, event, cx| {
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
            file_revisions: HashMap::new(),
            indexing: HashSet::new(),
            dirty: HashSet::new(),
            pending_symbols: Vec::new(),
            _subscriptions: vec![subscription],
        };

        manager.start_indexing(worktree_store, cx);
        manager
    }

    fn on_worktree_store_event(&mut self, event: &WorktreeStoreEvent, cx: &mut Context<Self>) {
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

    fn start_indexing(&mut self, worktree_store: &Entity<WorktreeStore>, cx: &mut Context<Self>) {
        let mut files = Vec::new();

        for worktree_entity in worktree_store.read(cx).visible_worktrees(cx) {
            let worktree = worktree_entity.read(cx);
            let worktree_id = worktree.id();
            let snapshot = worktree.snapshot();
            for entry in snapshot.files(false, 0) {
                // Skip FIFOs like project search does, as reading them would
                // block.
                if entry.is_fifo || entry.kind != EntryKind::File {
                    continue;
                }
                let abs_path = worktree.absolutize(entry.path.as_ref());
                let location = SymbolLocation {
                    worktree_id: worktree_id.to_proto(),
                    path: Arc::from(entry.path.as_ref().as_unix_str().to_string().as_str()),
                };
                files.push(FileToIndex {
                    location,
                    abs_path,
                    file: worktree::File::for_entry(entry.clone(), worktree_entity.clone()),
                    revision: 0,
                });
            }
        }

        self.total_file_count = files.len();
        self.indexed_file_count = 0;
        self.is_indexing = true;
        cx.notify();

        // Register every file, so that changes observed while the scan runs
        // invalidate the scan's results for those files.
        for file in &files {
            self.file_revisions
                .entry(file.location.clone())
                .or_insert(0);
            self.indexing.insert(file.location.clone());
        }

        let languages = self.languages.clone();
        let fs = self.fs.clone();
        let weak_self = cx.weak_entity();

        cx.spawn(async move |_, cx: &mut AsyncApp| {
            let (files_with_grammar, unavailable) =
                resolve_grammars(languages, fs.clone(), files, cx).await;
            if !unavailable.is_empty()
                && weak_self
                    .update(cx, |this, cx| {
                        this.indexed_file_count += unavailable.len();
                        for location in unavailable {
                            this.on_file_unavailable(&location, cx);
                        }
                    })
                    .is_err()
            {
                return;
            }

            let background = cx.background_executor().clone();
            let concurrency = background.num_cpus() * 2;
            let mut batch: Vec<(FileToIndex, LanguageName, Vec<ExtractedSymbol>)> = Vec::new();
            let mut indexed = 0usize;

            let mut results = stream::iter(files_with_grammar)
                .map(|(file, grammar, language_name)| {
                    read_file_symbols(fs.clone(), background.clone(), file, grammar, language_name)
                })
                .buffer_unordered(concurrency);

            while let Some(result) = results.next().await {
                indexed += 1;
                batch.push(result);

                if batch.len() >= PUBLISH_BATCH_SIZE {
                    let batch_to_flush = std::mem::take(&mut batch);
                    let result = weak_self.update(cx, |this, cx| {
                        for (file, language_name, extracted) in batch_to_flush {
                            this.on_file_indexed(file, language_name, extracted, cx);
                        }
                        this.indexed_file_count = indexed;
                    });
                    if result.is_err() {
                        return;
                    }
                }
            }

            let result = weak_self.update(cx, |this, cx| {
                for (file, language_name, extracted) in batch {
                    this.on_file_indexed(file, language_name, extracted, cx);
                }
                this.indexed_file_count = indexed;
                this.finish_scan(cx);
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
        let mut to_index: Vec<(SymbolLocation, PathBuf, Arc<dyn language::File>)> = Vec::new();

        for (rel_path, _entry_id, change) in changes.iter() {
            let location = SymbolLocation {
                worktree_id: worktree_id.to_proto(),
                path: Arc::from(rel_path.as_ref().as_unix_str().to_string().as_str()),
            };

            match change {
                PathChange::Removed => {
                    to_remove.push(location);
                }
                PathChange::Added
                | PathChange::Updated
                | PathChange::AddedOrUpdated
                | PathChange::Loaded => {
                    let Some(worktree_entity) =
                        worktree_store.read(cx).worktree_for_id(worktree_id, cx)
                    else {
                        continue;
                    };
                    let worktree = worktree_entity.read(cx);
                    let snapshot = worktree.snapshot();
                    let Some(entry) = snapshot.entry_for_path(rel_path.as_ref()) else {
                        continue;
                    };
                    // Apply the same eligibility rules as the initial scan,
                    // which lists entries without ignored files: skip FIFOs
                    // (reading them would block) and non-files, and treat a
                    // file that has become ignored as removed, so its indexed
                    // symbols do not survive the change.
                    if entry.is_fifo
                        || entry.kind != EntryKind::File
                        || (entry.is_ignored && !entry.is_always_included)
                    {
                        to_remove.push(location);
                        continue;
                    }
                    let abs_path = worktree.absolutize(rel_path.as_ref());
                    to_index.push((
                        location,
                        abs_path,
                        worktree::File::for_entry(entry.clone(), worktree_entity.clone()),
                    ));
                }
            }
        }

        // Removals take effect immediately, and dropping the revision entry
        // makes any in-flight read for the removed file discard its result on
        // completion.
        for location in &to_remove {
            self.file_revisions.remove(location);
        }
        if !to_remove.is_empty() {
            self.index.remove_files_batch(to_remove.iter().cloned());
        }

        let mut changed = !to_remove.is_empty();

        // Register the new revisions. Files with a task in flight are marked
        // dirty; their in-flight results are discarded on completion and the
        // files are read again at the latest revision. Bumping the revision
        // also invalidates any unpublished result for the file, which is
        // older than the change that just arrived.
        let mut files_to_spawn = Vec::new();
        for (location, abs_path, file) in to_index {
            let revision = self
                .file_revisions
                .entry(location.clone())
                .and_modify(|revision| *revision += 1)
                .or_insert(1);
            self.pending_symbols
                .retain(|(pending, _, _, _)| *pending != location);
            changed = true;
            if self.indexing.contains(&location) {
                self.dirty.insert(location);
            } else {
                files_to_spawn.push((location, abs_path, file, *revision));
            }
        }

        if changed {
            cx.emit(SymbolIndexEvent::UpdatedEntries);
            cx.notify();
        }

        if files_to_spawn.is_empty() {
            return;
        }

        self.spawn_index_tasks(files_to_spawn, cx);
    }

    /// Publish the result of a completed background read. Stale completions
    /// are discarded: a file that changed again while it was being read is
    /// read again at its latest revision, and a file that was removed in the
    /// meantime is dropped entirely.
    fn on_file_indexed(
        &mut self,
        file: FileToIndex,
        language_name: LanguageName,
        extracted: Vec<ExtractedSymbol>,
        cx: &mut Context<Self>,
    ) {
        self.indexing.remove(&file.location);

        if self.dirty.remove(&file.location) {
            // The file changed while it was being read; the result is stale.
            // Read it again unless it was removed in the meantime.
            let latest_revision = self.file_revisions.get(&file.location).copied();
            if let Some(revision) = latest_revision {
                self.spawn_index_tasks(
                    std::iter::once((file.location, file.abs_path, file.file, revision)),
                    cx,
                );
            }
        } else if self.file_revisions.get(&file.location) == Some(&file.revision) {
            // Still up to date: publish. Any older unpublished result for this
            // file was already dropped when the revision was bumped.
            self.pending_symbols
                .push((file.location, file.revision, language_name, extracted));
        }
        // else: the file was removed while it was being read; drop the result.

        self.flush_pending_symbols(cx);
    }

    /// Give up on a file whose language could not be resolved, without
    /// publishing anything for it.
    fn on_file_unavailable(&mut self, location: &SymbolLocation, cx: &mut Context<Self>) {
        self.indexing.remove(location);
        self.dirty.remove(location);
        self.flush_pending_symbols(cx);
    }

    /// Publish pending symbols once enough of them accumulated, or once no
    /// more reads are in flight.
    fn flush_pending_symbols(&mut self, cx: &mut Context<Self>) {
        if self.pending_symbols.is_empty() {
            return;
        }
        if !self.indexing.is_empty() && self.pending_symbols.len() < PUBLISH_BATCH_SIZE {
            return;
        }
        let batch = self.take_publishable_pending();
        if batch.is_empty() {
            return;
        }
        self.index.update_files_batch(batch);
        cx.emit(SymbolIndexEvent::UpdatedEntries);
        cx.notify();
    }

    fn finish_scan(&mut self, cx: &mut Context<Self>) {
        self.is_indexing = false;
        let batch = self.take_publishable_pending();
        if !batch.is_empty() {
            self.index.update_files_batch(batch);
        }
        cx.emit(SymbolIndexEvent::Indexed);
        cx.notify();
    }

    /// Takes the pending entries that may be published: entries whose file
    /// has since been removed or changed again are dropped, and when several
    /// entries for one file remain, only the newest is kept, so a batch can
    /// never insert two revisions of the same file.
    fn take_publishable_pending(
        &mut self,
    ) -> Vec<(SymbolLocation, LanguageName, Vec<ExtractedSymbol>)> {
        publishable_pending(
            std::mem::take(&mut self.pending_symbols),
            &self.file_revisions,
        )
    }

    fn spawn_index_tasks(
        &mut self,
        files: impl IntoIterator<Item = (SymbolLocation, PathBuf, Arc<dyn language::File>, u64)>,
        cx: &mut Context<Self>,
    ) {
        let mut files_to_index = Vec::new();
        for (location, abs_path, file, revision) in files {
            files_to_index.push(FileToIndex {
                location,
                abs_path,
                file,
                revision,
            });
        }

        for file in &files_to_index {
            self.indexing.insert(file.location.clone());
        }

        let languages = self.languages.clone();
        let fs = self.fs.clone();
        let weak_self = cx.weak_entity();

        cx.spawn(async move |_, cx: &mut AsyncApp| {
            let (files_with_grammar, unavailable) =
                resolve_grammars(languages, fs.clone(), files_to_index, cx).await;
            if !unavailable.is_empty()
                && weak_self
                    .update(cx, |this, cx| {
                        for location in unavailable {
                            this.on_file_unavailable(&location, cx);
                        }
                    })
                    .is_err()
            {
                return;
            }

            let background = cx.background_executor().clone();

            let mut results = stream::iter(files_with_grammar)
                .map(|(file, grammar, language_name)| {
                    read_file_symbols(fs.clone(), background.clone(), file, grammar, language_name)
                })
                .buffer_unordered(4);

            while let Some(result) = results.next().await {
                if weak_self
                    .update(cx, |this, cx| {
                        let (file, language_name, extracted) = result;
                        this.on_file_indexed(file, language_name, extracted, cx);
                    })
                    .is_err()
                {
                    return;
                }
            }
        })
        .detach();
    }

    fn remove_worktree(&mut self, worktree_id: WorktreeId, cx: &mut Context<Self>) {
        let worktree_id = worktree_id.to_proto();
        self.index.remove_worktree(worktree_id);
        // Invalidate any in-flight reads for this worktree.
        self.file_revisions
            .retain(|location, _| location.worktree_id != worktree_id);
        self.indexing
            .retain(|location| location.worktree_id != worktree_id);
        self.dirty
            .retain(|location| location.worktree_id != worktree_id);
        // Unpublished results for this worktree must not re-insert its
        // symbols after the removal above.
        self.pending_symbols
            .retain(|(location, _, _, _)| location.worktree_id != worktree_id);
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

/// Filters completed, unpublished extraction results down to the entries
/// that may be published: entries whose file has since been removed or
/// changed again (`revisions` no longer matches) are dropped, and when
/// several entries for one file remain, only the newest is kept, so a batch
/// can never insert two revisions of the same file.
fn publishable_pending(
    pending: Vec<(SymbolLocation, u64, LanguageName, Vec<ExtractedSymbol>)>,
    revisions: &HashMap<SymbolLocation, u64>,
) -> Vec<(SymbolLocation, LanguageName, Vec<ExtractedSymbol>)> {
    let mut latest: HashMap<SymbolLocation, usize> = HashMap::new();
    for (ix, (location, revision, _, _)) in pending.iter().enumerate() {
        if revisions.get(location) == Some(revision) {
            // A later entry for the same file supersedes an earlier one.
            latest.insert(location.clone(), ix);
        }
    }
    pending
        .into_iter()
        .enumerate()
        .filter(|(ix, (location, revision, _, _))| {
            revisions.get(location) == Some(revision) && latest.get(location) == Some(ix)
        })
        .map(|(_, (location, _, language_name, extracted))| (location, language_name, extracted))
        .collect()
}

/// Resolve each file's language before grouping, so files that share an
/// extension but resolve to different languages (e.g. `deno.lock` and
/// `pixi.lock`) are indexed with their own language's grammar. Language
/// detection goes through `language_for_file`, like buffer loading, so
/// user-configured `file_types` and content-based detection (shebangs)
/// resolve too; content is only probed when the path alone does not match.
/// Returns the files that could not be matched with a grammar.
async fn resolve_grammars(
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    files: Vec<FileToIndex>,
    cx: &AsyncApp,
) -> (
    Vec<(FileToIndex, Arc<Grammar>, LanguageName)>,
    Vec<SymbolLocation>,
) {
    let mut resolved = Vec::with_capacity(files.len());
    let mut unavailable = Vec::new();
    let mut grammars: HashMap<LanguageId, Option<(Arc<Grammar>, LanguageName)>> = HashMap::new();

    for file in files {
        let language_id = match cx.update(|cx| languages.language_for_file(&file.file, None, cx)) {
            Some(language_id) => Some(language_id),
            None => {
                // The path alone did not match any language; retry with
                // the file's content so extensionless scripts resolve the
                // same way they do when opened as a buffer.
                let content = fs
                    .load(&file.abs_path)
                    .await
                    .ok()
                    .map(|text| Rope::from(text));
                cx.update(|cx| languages.language_for_file(&file.file, content.as_ref(), cx))
            }
        };
        let Some(language_id) = language_id else {
            unavailable.push(file.location);
            continue;
        };
        let grammar = match grammars.get(&language_id) {
            Some(grammar) => grammar.clone(),
            None => {
                let language = languages
                    .load_language(language_id)
                    .await
                    .ok()
                    .and_then(|language| language.ok());
                let grammar = language.and_then(|language| {
                    let language_name = language.name();
                    language
                        .grammar()
                        .filter(|grammar| grammar.outline_config.is_some())
                        .cloned()
                        .map(|grammar| (grammar, language_name))
                });
                grammars.insert(language_id, grammar.clone());
                grammar
            }
        };
        match grammar {
            Some((grammar, language_name)) => resolved.push((file, grammar, language_name)),
            None => unavailable.push(file.location),
        }
    }

    (resolved, unavailable)
}

/// Read a file and extract its symbols on a background thread.
fn read_file_symbols(
    fs: Arc<dyn Fs>,
    background: BackgroundExecutor,
    file: FileToIndex,
    grammar: Arc<Grammar>,
    language_name: LanguageName,
) -> impl Future<Output = (FileToIndex, LanguageName, Vec<ExtractedSymbol>)> {
    async move {
        let abs_path = file.abs_path.clone();
        let extracted = background
            .spawn(async move {
                let text = match fs.load(&abs_path).await {
                    Ok(text) => text,
                    Err(err) => {
                        log::warn!("symbol_index: failed to read {abs_path:?}: {err}");
                        return Vec::new();
                    }
                };
                symbol_index::extract_symbols(&text, &grammar)
            })
            .await;
        (file, language_name, extracted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Project;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use serde_json::json;
    use std::sync::atomic::AtomicBool;
    use util::path;

    fn extracted_symbol(name: &str) -> Vec<ExtractedSymbol> {
        vec![ExtractedSymbol {
            name: name.to_string(),
            display_text: format!("fn {name}()"),
            name_range: 3..(3 + name.len() as u32),
            kind: language::SymbolKind::Function,
            row: 0,
            column: 3,
        }]
    }

    fn location(path: &str) -> SymbolLocation {
        SymbolLocation {
            worktree_id: 1,
            path: path.into(),
        }
    }

    #[test]
    fn test_publishable_pending_coalesces_by_file() {
        let a = location("a.rs");
        let b = location("b.rs");
        let rust = LanguageName::new("Rust");

        // Two revisions of `a.rs` and one of `b.rs` accumulated while reads
        // were in flight. `a.rs`'s latest revision is 2, so only the second
        // entry may be published.
        let pending = vec![
            (a.clone(), 1, rust.clone(), extracted_symbol("a_v1")),
            (b.clone(), 1, rust.clone(), extracted_symbol("b_v1")),
            (a.clone(), 2, rust, extracted_symbol("a_v2")),
        ];
        let revisions = HashMap::from([(a.clone(), 2), (b.clone(), 1)]);
        let batch = publishable_pending(pending, &revisions);

        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].0, b);
        assert_eq!(&*batch[0].2[0].name, "b_v1");
        assert_eq!(batch[1].0, a);
        assert_eq!(&*batch[1].2[0].name, "a_v2");
    }

    #[test]
    fn test_publishable_pending_drops_stale_and_removed_entries() {
        let a = location("a.rs");
        let rust = LanguageName::new("Rust");

        // `a.rs` changed again after this entry completed (revision 3 is in
        // flight), and `b.rs` was removed (no revision entry anymore).
        let b = location("b.rs");
        let pending = vec![
            (a.clone(), 2, rust.clone(), extracted_symbol("a")),
            (b, 1, rust, extracted_symbol("b")),
        ];
        let revisions = HashMap::from([(a, 3)]);
        let batch = publishable_pending(pending, &revisions);
        assert!(batch.is_empty());
    }

    async fn index_symbols(
        project: &gpui::Entity<Project>,
        query: &str,
        cx: &mut TestAppContext,
    ) -> Vec<String> {
        let snapshot = project.update(cx, |project, cx| {
            project
                .symbol_index(cx)
                .update(cx, |manager, _| manager.snapshot())
        });
        let results = snapshot
            .search(
                query,
                10,
                Arc::new(AtomicBool::new(false)),
                cx.executor(),
                None,
            )
            .await;
        results
            .into_iter()
            .map(|result| result.symbol.name.to_string())
            .collect()
    }

    #[gpui::test]
    async fn test_symbol_index_skips_ignored_files(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".gitignore": "gen.rs\n",
                "gen.rs": "fn generated_function() {}\n",
                "lib.rs": "fn kept_function() {}\n",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(language::rust_lang());
        // Force the initial scan.
        project.update(cx, |project, cx| {
            project.symbol_index(cx);
        });
        cx.run_until_parked();

        let names = index_symbols(&project, "function", cx).await;
        assert_eq!(names, ["kept_function".to_string()]);

        // Saving an ignored file must not add its symbols to the index.
        fs.save(
            path!("/dir/gen.rs").as_ref(),
            &"fn generated_function() {}\nfn another_generated() {}\n".into(),
            Default::default(),
        )
        .await
        .unwrap();
        cx.run_until_parked();

        let names = index_symbols(&project, "generated", cx).await;
        assert!(names.is_empty());
        let names = index_symbols(&project, "function", cx).await;
        assert_eq!(names, ["kept_function".to_string()]);
    }
}
