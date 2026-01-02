use std::sync::Arc;

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, BackgroundExecutor, Context, Entity, EntityId, Subscription, Task};
use language::{Anchor, Buffer, BufferEvent};
use parking_lot::Mutex;

use crate::{Project, ProjectPath, worktree_store::WorktreeStoreEvent};

/// An entry in the symbol index, representing either an LSP workspace symbol
/// or a Tree-sitter outline item.
#[derive(Clone, Debug)]
pub struct OutlineSymbol {
    pub name: String,
    pub path: ProjectPath,
    pub buffer: Entity<Buffer>,
    pub range: std::ops::Range<Anchor>,
}

/// Index of symbols extracted from Tree-sitter outlines.
/// This provides symbol search for languages without LSP support.
pub struct SymbolIndex {
    /// Symbols indexed by file path
    symbols_by_path: HashMap<ProjectPath, Vec<OutlineSymbol>>,
    /// All symbols flattened for searching
    all_symbols: Vec<OutlineSymbol>,
    /// Whether indexing is in progress
    is_indexing: bool,
    /// Progress: (processed, total)
    indexing_progress: (usize, usize),
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self {
            symbols_by_path: HashMap::default(),
            all_symbols: Vec::new(),
            is_indexing: false,
            indexing_progress: (0, 0),
        }
    }

    pub fn is_indexing(&self) -> bool {
        self.is_indexing
    }

    pub fn indexing_progress(&self) -> (usize, usize) {
        self.indexing_progress
    }

    pub fn indexing_progress_percent(&self) -> usize {
        let (processed, total) = self.indexing_progress;
        if total == 0 {
            0
        } else {
            (processed * 100) / total
        }
    }

    pub fn symbols(&self) -> &[OutlineSymbol] {
        &self.all_symbols
    }

    pub fn is_empty(&self) -> bool {
        self.all_symbols.is_empty()
    }

    /// Update symbols for a specific file path
    pub fn update_file(&mut self, path: ProjectPath, symbols: Vec<OutlineSymbol>) {
        if symbols.is_empty() {
            self.symbols_by_path.remove(&path);
        } else {
            self.symbols_by_path.insert(path, symbols);
        }
        self.rebuild_all_symbols();
    }

    /// Remove symbols for a specific file path
    pub fn remove_file(&mut self, path: &ProjectPath) {
        if self.symbols_by_path.remove(path).is_some() {
            self.rebuild_all_symbols();
        }
    }

    /// Clear all symbols
    pub fn clear(&mut self) {
        self.symbols_by_path.clear();
        self.all_symbols.clear();
    }

    fn rebuild_all_symbols(&mut self) {
        self.all_symbols = self
            .symbols_by_path
            .values()
            .flat_map(|symbols| symbols.iter().cloned())
            .collect();
    }

    fn set_indexing(&mut self, is_indexing: bool) {
        self.is_indexing = is_indexing;
    }

    fn set_progress(&mut self, processed: usize, total: usize) {
        self.indexing_progress = (processed, total);
    }
}

/// Global cache of symbol indices, keyed by project entity ID.
/// This allows symbol indices to persist across modal open/close cycles.
static SYMBOL_INDICES: Mutex<Option<HashMap<EntityId, Arc<Mutex<SymbolIndex>>>>> = Mutex::new(None);

/// Get or create the symbol index for a project
pub fn get_symbol_index(project: &Entity<Project>) -> Arc<Mutex<SymbolIndex>> {
    let project_id = project.entity_id();
    let mut indices = SYMBOL_INDICES.lock();
    let indices = indices.get_or_insert_with(HashMap::default);
    indices
        .entry(project_id)
        .or_insert_with(|| Arc::new(Mutex::new(SymbolIndex::new())))
        .clone()
}

/// Start indexing all symbols in the project.
/// This extracts outline items from all files using Tree-sitter.
pub fn start_indexing(project: Entity<Project>, cx: &mut App) {
    let index = get_symbol_index(&project);

    // Don't start if already indexing
    {
        let mut index_guard = index.lock();
        if index_guard.is_indexing() {
            return;
        }
        index_guard.set_indexing(true);
    }

    // Collect all file paths to index
    let worktrees: Vec<_> = project.read(cx).visible_worktrees(cx).collect();
    let mut paths_to_index: Vec<ProjectPath> = Vec::new();

    for worktree in &worktrees {
        let snapshot = worktree.read(cx).snapshot();
        for entry in snapshot.files(false, 0) {
            if entry.is_ignored {
                continue;
            }
            paths_to_index.push(ProjectPath {
                worktree_id: snapshot.id(),
                path: entry.path.clone(),
            });
        }
    }

    log::info!(
        "Symbol index: starting to index {} files",
        paths_to_index.len()
    );

    // Initialize progress
    {
        let mut index_guard = index.lock();
        index_guard.set_progress(0, paths_to_index.len());
    }

    cx.spawn({
        async move |cx| {
            let batch_size = 50;
            let total_files = paths_to_index.len();

            for (batch_idx, paths_batch) in paths_to_index.chunks(batch_size).enumerate() {
                let mut batch_tasks: Vec<(ProjectPath, Task<anyhow::Result<Vec<OutlineSymbol>>>)> =
                    Vec::new();

                for path in paths_batch {
                    let path = path.clone();
                    let project = project.clone();

                    let task = cx.update(|cx| {
                        project.update(cx, |project, cx| {
                            let open_task = project.open_buffer(path.clone(), cx);
                            let path_for_task = path.clone();
                            cx.spawn(async move |_project, cx| {
                                let buffer = open_task.await?;
                                let symbols = cx.update(|cx| {
                                    extract_outline_symbols(&buffer, &path_for_task, cx)
                                })?;
                                anyhow::Ok(symbols)
                            })
                        })
                    });

                    if let Ok(task) = task {
                        batch_tasks.push((path, task));
                    }
                }

                // Process batch results
                for (path, task) in batch_tasks {
                    match task.await {
                        Ok(symbols) => {
                            if !symbols.is_empty() {
                                log::debug!(
                                    "Symbol index: got {} symbols from {}",
                                    symbols.len(),
                                    path.path.as_unix_str()
                                );
                                let mut index_guard = index.lock();
                                index_guard.update_file(path, symbols);
                            }
                        }
                        Err(e) => {
                            log::debug!(
                                "Symbol index: failed to index {}: {}",
                                path.path.as_unix_str(),
                                e
                            );
                        }
                    }
                }

                // Update progress
                let processed = ((batch_idx + 1) * batch_size).min(total_files);
                {
                    let mut index_guard = index.lock();
                    index_guard.set_progress(processed, total_files);
                }

                if processed % 100 == 0 || processed >= total_files {
                    let symbol_count = index.lock().symbols().len();
                    log::info!(
                        "Symbol index: indexed {}/{} files ({}%), {} symbols",
                        processed,
                        total_files,
                        (processed * 100) / total_files.max(1),
                        symbol_count
                    );
                }
            }

            log::info!(
                "Symbol index: complete, {} total symbols",
                index.lock().symbols().len()
            );

            // Mark indexing as complete
            {
                let mut index_guard = index.lock();
                index_guard.set_indexing(false);
            }
        }
    })
    .detach();
}

/// Extract outline symbols from a buffer using Tree-sitter
fn extract_outline_symbols(
    buffer: &Entity<Buffer>,
    path: &ProjectPath,
    cx: &App,
) -> Vec<OutlineSymbol> {
    let buffer_ref = buffer.read(cx);
    let snapshot = buffer_ref.snapshot();
    let outline = snapshot.outline(None);

    outline
        .items
        .into_iter()
        .map(|item| OutlineSymbol {
            name: item.text,
            path: path.clone(),
            buffer: buffer.clone(),
            range: item.range,
        })
        .collect()
}

/// Subscribe to buffer and worktree events for incremental index updates.
/// Returns subscriptions that should be stored to keep them active.
pub fn subscribe_for_updates(
    project: &Entity<Project>,
    cx: &mut Context<Project>,
) -> Vec<Subscription> {
    let mut subscriptions = Vec::new();
    let index = get_symbol_index(project);

    // Subscribe to worktree events for file creation/deletion
    let worktree_store = project.read(cx).worktree_store.clone();
    let project_weak = cx.entity().downgrade();

    subscriptions.push(cx.subscribe(&worktree_store, move |_project, _store, event, cx| {
        match event {
            WorktreeStoreEvent::WorktreeAdded(_worktree) => {
                // Re-index when a worktree is added
                if let Some(project) = project_weak.upgrade() {
                    start_indexing(project, cx);
                }
            }
            WorktreeStoreEvent::WorktreeRemoved(_, worktree_id) => {
                // Remove symbols for the removed worktree
                let mut index_guard = index.lock();
                let paths_to_remove: Vec<_> = index_guard
                    .symbols_by_path
                    .keys()
                    .filter(|p| p.worktree_id == *worktree_id)
                    .cloned()
                    .collect();
                for path in paths_to_remove {
                    index_guard.remove_file(&path);
                }
            }
            _ => {}
        }
    }));

    subscriptions
}

/// Subscribe to a buffer for reparsing events to update its symbols.
pub fn subscribe_to_buffer(
    project: &Entity<Project>,
    buffer: &Entity<Buffer>,
    path: ProjectPath,
    cx: &mut Context<Project>,
) -> Subscription {
    let index = get_symbol_index(project);
    let buffer_clone = buffer.clone();

    cx.subscribe(buffer, move |_project, _buffer, event, cx| {
        if matches!(event, BufferEvent::Reparsed) {
            let symbols = extract_outline_symbols(&buffer_clone, &path, cx);
            let mut index_guard = index.lock();
            index_guard.update_file(path.clone(), symbols);
        }
    })
}

/// Search result from the symbol index
pub struct SymbolSearchResult {
    pub symbol: OutlineSymbol,
    pub string_match: StringMatch,
}

/// Search the symbol index for symbols matching the query.
/// Returns a task that performs fuzzy matching on a background thread.
pub fn search(
    project: &Entity<Project>,
    query: &str,
    max_results: usize,
    executor: BackgroundExecutor,
) -> Task<Vec<SymbolSearchResult>> {
    if query.is_empty() {
        return Task::ready(Vec::new());
    }

    let index = get_symbol_index(project);
    let symbols = index.lock().all_symbols.clone();
    let query = query.to_string();

    executor.clone().spawn(async move {
        if symbols.is_empty() {
            return Vec::new();
        }

        let candidates: Vec<StringMatchCandidate> = symbols
            .iter()
            .enumerate()
            .map(|(id, symbol)| StringMatchCandidate::new(id, &symbol.name))
            .collect();

        let matches = fuzzy::match_strings(
            &candidates,
            &query,
            false, // smart_case
            true,  // penalize_length
            max_results,
            &Default::default(),
            executor.clone(),
        )
        .await;

        matches
            .into_iter()
            .filter_map(|m| {
                let symbol = symbols.get(m.candidate_id)?.clone();
                Some(SymbolSearchResult {
                    symbol,
                    string_match: m,
                })
            })
            .collect()
    })
}
