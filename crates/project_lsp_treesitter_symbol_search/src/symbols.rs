use std::sync::Arc;

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, Entity, EntityId, SharedString, Task as GpuiTask};
use language::{Anchor, Buffer};
use parking_lot::Mutex;
use project::{Project, Symbol};
use util::ResultExt;

use crate::ProjectSymbolSearchDelegate;
use crate::providers::{DocumentSymbolResult, SearchResult};

// Global cache for symbol providers, keyed by project entity ID
static SYMBOL_CACHES: Mutex<Option<HashMap<EntityId, Arc<SymbolCache>>>> = Mutex::new(None);

struct SymbolCache {
    workspace_symbols: Mutex<Vec<Symbol>>,
    /// Maps file paths to their last known mtime and the symbols found in them.
    outline_cache: Mutex<HashMap<project::ProjectPath, (fs::MTime, Vec<SymbolEntry>)>>,
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
                        workspace_symbols: Mutex::new(Vec::new()),
                        outline_cache: Mutex::new(HashMap::default()),
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

        // Get all file paths from all worktrees
        let worktrees: Vec<_> = project.read(cx).visible_worktrees(cx).collect();

        let mut paths_to_reindex: Vec<(project::ProjectPath, fs::MTime)> = Vec::new();
        let mut total_files = 0;

        {
            let outline_cache = cache.outline_cache.lock();
            for worktree in &worktrees {
                let snapshot = worktree.read(cx).snapshot();
                for entry in snapshot.files(false, 0) {
                    if entry.is_ignored {
                        continue;
                    }
                    total_files += 1;
                    let project_path = project::ProjectPath {
                        worktree_id: snapshot.id(),
                        path: entry.path.clone(),
                    };

                    // Only re-index if the file has changed or isn't in cache
                    let needs_reindex = entry
                        .mtime
                        .map(|mtime| {
                            outline_cache
                                .get(&project_path)
                                .map_or(true, |(cached_mtime, _)| *cached_mtime != mtime)
                        })
                        .unwrap_or(true);

                    if needs_reindex {
                        if let Some(mtime) = entry.mtime {
                            paths_to_reindex.push((project_path, mtime));
                        }
                    }
                }
            }
        }

        log::info!(
            "Search everywhere: {} total files, re-indexing {} changed files",
            total_files,
            paths_to_reindex.len()
        );

        // Workspace symbols (LSP) are fetched every time as they are typically fast
        let workspace_symbols_task = project.update(cx, |project, cx| project.symbols("", cx));

        {
            let mut progress = cache.indexing_progress.lock();
            *progress = (0, paths_to_reindex.len());
        }

        cx.spawn({
            let project = project.clone();
            async move |cx| {
                // Update workspace symbols
                if let Some(symbols) = workspace_symbols_task.await.log_err() {
                    let mut ws_cache = cache.workspace_symbols.lock();
                    *ws_cache = symbols;
                }

                let batch_size = 50;
                let total_to_reindex = paths_to_reindex.len();

                for (batch_idx, batch) in paths_to_reindex.chunks(batch_size).enumerate() {
                    let mut batch_tasks = Vec::new();

                    for (path, mtime) in batch {
                        let path = path.clone();
                        let mtime = *mtime;
                        let project = project.clone();

                        let task = cx.update(|cx| {
                            project.update(cx, |project, cx| {
                                let open_task = project.open_buffer(path.clone(), cx);
                                cx.spawn(async move |_weak_project, cx| {
                                    let buffer = open_task.await?;
                                    let outline_items = cx.update(|cx| {
                                        let buffer_snapshot = buffer.read(cx).snapshot();
                                        buffer_snapshot.outline(None).items
                                    });
                                    anyhow::Ok((path, mtime, buffer, outline_items))
                                })
                            })
                        });

                        batch_tasks.push(task);
                    }

                    for task in batch_tasks {
                        match task.await {
                            Ok((path, mtime, buffer, items)) => {
                                let path_str = path.path.as_unix_str().to_string();
                                let symbols = items
                                    .into_iter()
                                    .map(|item| SymbolEntry::Outline {
                                        name: item.text,
                                        path: path_str.clone(),
                                        buffer: buffer.clone(),
                                        range: item.range,
                                    })
                                    .collect();

                                cache.outline_cache.lock().insert(path, (mtime, symbols));
                            }
                            Err(err) => {
                                if err.to_string().contains("Binary files are not supported") {
                                    continue;
                                }
                                log::error!("Search everywhere indexing error: {err:?}");
                            }
                        }
                    }

                    let processed = ((batch_idx + 1) * batch_size).min(total_to_reindex);
                    {
                        let mut progress = cache.indexing_progress.lock();
                        *progress = (processed, total_to_reindex);
                    }
                    let _ = cx.update(|_| ());
                }

                // Mark indexing as complete
                {
                    let mut indexing = cache.is_indexing.lock();
                    *indexing = false;
                }

                // Notify to refresh UI
                let _ = cx.update(|_cx| {});
            }
        })
        .detach();
    }

    pub fn is_indexing(&self) -> bool {
        *self.cache.is_indexing.lock()
    }

    pub fn indexing_progress_percent(&self) -> usize {
        let (processed, total) = *self.cache.indexing_progress.lock();
        processed
            .saturating_mul(100)
            .checked_div(total)
            .unwrap_or(0)
    }

    pub fn search(
        &self,
        query: &str,
        cx: &mut gpui::Context<picker::Picker<ProjectSymbolSearchDelegate>>,
    ) -> GpuiTask<Vec<(SearchResult, StringMatch)>> {
        if query.is_empty() {
            return GpuiTask::ready(Vec::new());
        }

        let workspace_symbols = self.cache.workspace_symbols.lock().clone();
        let outline_cache = self.cache.outline_cache.lock();
        let mut all_entries = Vec::new();

        for s in workspace_symbols {
            all_entries.push(SymbolEntry::Workspace(s));
        }
        for (_, symbols) in outline_cache.values() {
            all_entries.extend(symbols.iter().cloned());
        }
        drop(outline_cache);

        let query = query.to_string();

        cx.spawn(async move |_, cx| {
            if all_entries.is_empty() {
                return Vec::new();
            }

            let candidates: Vec<StringMatchCandidate> = all_entries
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
                    let entry = all_entries.get(m.candidate_id)?;

                    let result = match entry {
                        SymbolEntry::Workspace(symbol) => {
                            let label = symbol.label.text.clone();
                            let detail = symbol.label.filter_text().to_string();
                            SearchResult {
                                label: SharedString::from(label),
                                detail: Some(SharedString::from(detail)),
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
