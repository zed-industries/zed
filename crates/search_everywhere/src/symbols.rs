use std::sync::Arc;

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, EntityId, SharedString, Task as GpuiTask};
use language::{Anchor, Buffer, OutlineItem};
use parking_lot::Mutex;
use project::{Project, Symbol};
use util::ResultExt;

use crate::SearchEverywhereDelegate;
use crate::providers::{DocumentSymbolResult, SearchResult, SearchResultCategory};

// Global cache for symbol providers, keyed by project entity ID
static SYMBOL_CACHES: Mutex<Option<HashMap<EntityId, Arc<SymbolCache>>>> = Mutex::new(None);

struct SymbolCache {
    cached_symbols: Mutex<Vec<SymbolEntry>>,
    is_indexing: Mutex<bool>,
    indexing_progress: Mutex<(usize, usize)>,
}

pub struct SymbolProvider {
    project: Entity<Project>,
    cache: Arc<SymbolCache>,
}

#[derive(Clone)]
pub enum SymbolEntry {
    Workspace(Symbol),
    Outline {
        name: String,
        path: String,
        buffer: Entity<Buffer>,
        range: std::ops::Range<Anchor>,
    },
}

impl SymbolProvider {
    pub fn new(project: Entity<Project>) -> Self {
        let project_id = project.entity_id();

        // Get or create the cache for this project
        let cache = {
            let mut caches = SYMBOL_CACHES.lock();
            let caches = caches.get_or_insert_with(HashMap::default);
            caches
                .entry(project_id)
                .or_insert_with(|| {
                    Arc::new(SymbolCache {
                        cached_symbols: Mutex::new(Vec::new()),
                        is_indexing: Mutex::new(false),
                        indexing_progress: Mutex::new((0, 0)),
                    })
                })
                .clone()
        };

        Self { project, cache }
    }

    /// Start indexing all symbols in the project. Call this when the modal opens.
    pub fn start_indexing(&self, cx: &mut App) {
        // Don't start if already indexing
        {
            let mut is_indexing = self.cache.is_indexing.lock();
            if *is_indexing {
                return;
            }
            *is_indexing = true;
        }

        let project = self.project.clone();
        let cache = self.cache.clone();

        // Get all file paths from all worktrees (respects .gitignore)
        let worktrees: Vec<_> = project.read(cx).visible_worktrees(cx).collect();

        let mut paths_to_index: Vec<project::ProjectPath> = Vec::new();
        for worktree in &worktrees {
            let snapshot = worktree.read(cx).snapshot();
            for entry in snapshot.files(false, 0) {
                // Skip ignored files
                if entry.is_ignored {
                    continue;
                }
                paths_to_index.push(project::ProjectPath {
                    worktree_id: snapshot.id(),
                    path: entry.path.clone(),
                });
            }
        }

        log::info!(
            "Search everywhere: starting to index {} files for symbols",
            paths_to_index.len()
        );

        // Also try workspace symbols with empty query to get all symbols (if LSP supports it)
        let workspace_symbols_task = project.update(cx, |project, cx| project.symbols("", cx));

        // Initialize progress
        {
            let mut progress = cache.indexing_progress.lock();
            *progress = (0, paths_to_index.len());
        }

        cx.spawn({
            let project = project.clone();
            async move |cx| {
                let mut all_symbols: Vec<SymbolEntry> = Vec::new();

                // Collect workspace symbols first (fast, if LSP supports it)
                if let Some(symbols) = workspace_symbols_task.await.log_err() {
                    log::info!(
                        "Search everywhere: indexed {} workspace symbols",
                        symbols.len()
                    );
                    for symbol in symbols {
                        all_symbols.push(SymbolEntry::Workspace(symbol));
                    }
                }

                // Now open each file and get outline items (Tree-sitter based, no LSP needed)
                // Process in batches to avoid overwhelming the system
                let batch_size = 50;
                let total_files = paths_to_index.len();

                for (batch_idx, paths_batch) in paths_to_index.chunks(batch_size).enumerate() {
                    let mut batch_tasks: Vec<(
                        String,
                        GpuiTask<anyhow::Result<(Entity<Buffer>, Vec<OutlineItem<Anchor>>)>>,
                    )> = Vec::new();

                    for path in paths_batch {
                        let path_string = path.path.as_unix_str().to_string();
                        let path = path.clone();
                        let project = project.clone();

                        // Open buffer and get outline items
                        let task = cx.update(|cx| {
                            project.update(cx, |project, cx| {
                                let open_task = project.open_buffer(path.clone(), cx);
                                cx.spawn(async move |_weak_project, cx| {
                                    let buffer = open_task.await?;
                                    let outline_items = cx.update(|cx| {
                                        let buffer_snapshot = buffer.read(cx).snapshot();
                                        let outline = buffer_snapshot.outline(None);
                                        outline.items
                                    })?;
                                    anyhow::Ok((buffer, outline_items))
                                })
                            })
                        });

                        if let Ok(task) = task {
                            batch_tasks.push((path_string, task));
                        }
                    }

                    // Wait for batch to complete
                    for (file_path, task) in batch_tasks {
                        match task.await {
                            Ok((buffer, outline_items)) => {
                                if !outline_items.is_empty() {
                                    log::debug!(
                                        "Search everywhere: got {} outline items from {}",
                                        outline_items.len(),
                                        file_path
                                    );
                                }
                                for item in outline_items {
                                    all_symbols.push(SymbolEntry::Outline {
                                        name: item.text.clone(),
                                        path: file_path.clone(),
                                        buffer: buffer.clone(),
                                        range: item.range.clone(),
                                    });
                                }
                            }
                            Err(e) => {
                                log::debug!(
                                    "Search everywhere: failed to get outline for {}: {}",
                                    file_path,
                                    e
                                );
                            }
                        }
                    }

                    // Update progress
                    let processed = ((batch_idx + 1) * batch_size).min(total_files);
                    {
                        let mut progress = cache.indexing_progress.lock();
                        *progress = (processed, total_files);
                    }

                    if processed % 100 == 0 || processed >= total_files {
                        log::info!(
                            "Search everywhere: indexed {}/{} files ({}%), {} symbols so far",
                            processed,
                            total_files,
                            (processed * 100) / total_files.max(1),
                            all_symbols.len()
                        );
                    }

                    // Update cache incrementally so results appear as we index
                    {
                        let mut symbols = cache.cached_symbols.lock();
                        *symbols = all_symbols.clone();
                    }
                }

                log::info!(
                    "Search everywhere: indexing complete, {} total symbols",
                    all_symbols.len()
                );

                // Final cache update
                {
                    let mut symbols = cache.cached_symbols.lock();
                    *symbols = all_symbols;
                }

                // Mark indexing as complete
                {
                    let mut indexing = cache.is_indexing.lock();
                    *indexing = false;
                }

                // Notify to refresh UI
                cx.update(|_cx| {}).ok();
            }
        })
        .detach();
    }

    pub fn is_indexing(&self) -> bool {
        *self.cache.is_indexing.lock()
    }

    pub fn indexing_progress_percent(&self) -> usize {
        let (processed, total) = *self.cache.indexing_progress.lock();
        if total == 0 {
            0
        } else {
            (processed * 100) / total
        }
    }

    pub fn has_cached_symbols(&self) -> bool {
        !self.cache.cached_symbols.lock().is_empty()
    }

    pub fn search(
        &self,
        query: &str,
        cx: &mut gpui::Context<picker::Picker<SearchEverywhereDelegate>>,
    ) -> GpuiTask<Vec<(SearchResult, StringMatch)>> {
        if query.is_empty() {
            return GpuiTask::ready(Vec::new());
        }

        let cached_symbols = self.cache.cached_symbols.lock().clone();
        let query = query.to_string();

        cx.spawn(async move |_, cx| {
            if cached_symbols.is_empty() {
                return Vec::new();
            }

            let candidates: Vec<StringMatchCandidate> = cached_symbols
                .iter()
                .enumerate()
                .map(|(id, entry)| {
                    let name = match entry {
                        SymbolEntry::Workspace(s) => s.label.filter_text().to_string(),
                        SymbolEntry::Outline { name, .. } => name.clone(),
                    };
                    StringMatchCandidate::new(id, &name)
                })
                .collect();

            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            matches
                .into_iter()
                .filter_map(|m| {
                    let entry = cached_symbols.get(m.candidate_id)?;

                    let result = match entry {
                        SymbolEntry::Workspace(symbol) => {
                            let label = symbol.label.text.clone();
                            let detail = symbol.label.filter_text().to_string();
                            SearchResult {
                                label: SharedString::from(label),
                                detail: Some(SharedString::from(detail)),
                                category: SearchResultCategory::Symbol,
                                path: None,
                                action: None,
                                symbol: Some(symbol.clone()),
                                document_symbol: None,
                            }
                        }
                        SymbolEntry::Outline {
                            name,
                            path,
                            buffer,
                            range,
                        } => SearchResult {
                            label: SharedString::from(name.clone()),
                            detail: Some(SharedString::from(path.clone())),
                            category: SearchResultCategory::Symbol,
                            path: None,
                            action: None,
                            symbol: None,
                            document_symbol: Some(DocumentSymbolResult {
                                buffer: buffer.clone(),
                                range: range.clone(),
                            }),
                        },
                    };

                    Some((result, m))
                })
                .collect()
        })
    }
}
