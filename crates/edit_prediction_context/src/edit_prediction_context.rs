use crate::assemble_excerpts::assemble_excerpt_ranges;
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, StreamExt as _, channel::mpsc, future};
use gpui::{App, AppContext, AsyncApp, Context, Entity, EntityId, EventEmitter, Task, WeakEntity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, Point, ToOffset as _};
use project::{LocationLink, Project, ProjectPath};
use smallvec::SmallVec;
use std::{
    collections::hash_map,
    ops::Range,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use util::paths::PathStyle;
use util::rel_path::RelPath;
use util::{RangeExt as _, ResultExt};

mod assemble_excerpts;
#[cfg(test)]
mod edit_prediction_context_tests;
#[cfg(test)]
mod fake_definition_lsp;

pub use zeta_prompt::{RelatedExcerpt, RelatedFile};

const IDENTIFIER_LINE_COUNT: u32 = 3;

pub struct RelatedExcerptStore {
    project: WeakEntity<Project>,
    related_buffers: Vec<RelatedBuffer>,
    cache: HashMap<Identifier, Arc<CacheEntry>>,
    update_tx: mpsc::UnboundedSender<(Entity<Buffer>, Anchor)>,
    identifier_line_count: u32,
}

struct RelatedBuffer {
    buffer: Entity<Buffer>,
    path: Arc<Path>,
    anchor_ranges: Vec<Range<Anchor>>,
    cached_file: Option<CachedRelatedFile>,
}

struct CachedRelatedFile {
    excerpts: Vec<RelatedExcerpt>,
    buffer_version: clock::Global,
}

pub enum RelatedExcerptStoreEvent {
    StartedRefresh,
    FinishedRefresh {
        cache_hit_count: usize,
        cache_miss_count: usize,
        mean_definition_latency: Duration,
        max_definition_latency: Duration,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Identifier {
    pub name: String,
    pub range: Range<Anchor>,
}

enum DefinitionTask {
    CacheHit(Arc<CacheEntry>),
    CacheMiss(Task<Result<Option<Vec<LocationLink>>>>),
}

#[derive(Debug)]
struct CacheEntry {
    definitions: SmallVec<[CachedDefinition; 1]>,
}

#[derive(Clone, Debug)]
struct CachedDefinition {
    path: ProjectPath,
    buffer: Entity<Buffer>,
    anchor_range: Range<Anchor>,
}

const DEBOUNCE_DURATION: Duration = Duration::from_millis(100);

impl EventEmitter<RelatedExcerptStoreEvent> for RelatedExcerptStore {}

impl RelatedExcerptStore {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        let (update_tx, mut update_rx) = mpsc::unbounded::<(Entity<Buffer>, Anchor)>();
        cx.spawn(async move |this, cx| {
            let executor = cx.background_executor().clone();
            while let Some((mut buffer, mut position)) = update_rx.next().await {
                let mut timer = executor.timer(DEBOUNCE_DURATION).fuse();
                loop {
                    futures::select_biased! {
                        next = update_rx.next() => {
                            if let Some((new_buffer, new_position)) = next {
                                buffer = new_buffer;
                                position = new_position;
                                timer = executor.timer(DEBOUNCE_DURATION).fuse();
                            } else {
                                return anyhow::Ok(());
                            }
                        }
                        _ = timer => break,
                    }
                }

                Self::fetch_excerpts(this.clone(), buffer, position, cx).await?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        RelatedExcerptStore {
            project: project.downgrade(),
            update_tx,
            related_buffers: Vec::new(),
            cache: Default::default(),
            identifier_line_count: IDENTIFIER_LINE_COUNT,
        }
    }

    pub fn set_identifier_line_count(&mut self, count: u32) {
        self.identifier_line_count = count;
    }

    pub fn refresh(&mut self, buffer: Entity<Buffer>, position: Anchor, _: &mut Context<Self>) {
        self.update_tx.unbounded_send((buffer, position)).ok();
    }

    pub fn related_files(&mut self, cx: &App) -> Vec<RelatedFile> {
        self.related_buffers
            .iter_mut()
            .map(|related| related.related_file(cx))
            .collect()
    }

    pub fn related_files_with_buffers(&mut self, cx: &App) -> Vec<(RelatedFile, Entity<Buffer>)> {
        self.related_buffers
            .iter_mut()
            .map(|related| (related.related_file(cx), related.buffer.clone()))
            .collect::<Vec<_>>()
    }

    pub fn set_related_files(&mut self, files: Vec<RelatedFile>, cx: &App) {
        self.related_buffers = files
            .into_iter()
            .filter_map(|file| {
                let project = self.project.upgrade()?;
                let project = project.read(cx);
                let worktree = project.worktrees(cx).find(|wt| {
                    let root_name = wt.read(cx).root_name().as_unix_str();
                    file.path
                        .components()
                        .next()
                        .is_some_and(|c| c.as_os_str() == root_name)
                })?;
                let worktree = worktree.read(cx);
                let relative_path = file
                    .path
                    .strip_prefix(worktree.root_name().as_unix_str())
                    .ok()?;
                let relative_path = RelPath::new(relative_path, PathStyle::Posix).ok()?;
                let project_path = ProjectPath {
                    worktree_id: worktree.id(),
                    path: relative_path.into_owned().into(),
                };
                let buffer = project.get_open_buffer(&project_path, cx)?;
                let snapshot = buffer.read(cx).snapshot();
                let anchor_ranges = file
                    .excerpts
                    .iter()
                    .map(|excerpt| {
                        let start = snapshot.anchor_before(Point::new(excerpt.row_range.start, 0));
                        let end_row = excerpt.row_range.end;
                        let end_col = snapshot.line_len(end_row);
                        let end = snapshot.anchor_after(Point::new(end_row, end_col));
                        start..end
                    })
                    .collect();
                Some(RelatedBuffer {
                    buffer,
                    path: file.path.clone(),
                    anchor_ranges,
                    cached_file: None,
                })
            })
            .collect();
    }

    async fn fetch_excerpts(
        this: WeakEntity<Self>,
        buffer: Entity<Buffer>,
        position: Anchor,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (project, snapshot, identifier_line_count) = this.read_with(cx, |this, cx| {
            (
                this.project.upgrade(),
                buffer.read(cx).snapshot(),
                this.identifier_line_count,
            )
        })?;
        let Some(project) = project else {
            return Ok(());
        };

        let file = snapshot.file().cloned();
        if let Some(file) = &file {
            log::debug!("retrieving_context buffer:{}", file.path().as_unix_str());
        }

        this.update(cx, |_, cx| {
            cx.emit(RelatedExcerptStoreEvent::StartedRefresh);
        })?;

        let identifiers = cx
            .background_spawn(async move {
                identifiers_for_position(&snapshot, position, identifier_line_count)
            })
            .await;

        let async_cx = cx.clone();
        let start_time = Instant::now();
        let futures = this.update(cx, |this, cx| {
            identifiers
                .into_iter()
                .filter_map(|identifier| {
                    let task = if let Some(entry) = this.cache.get(&identifier) {
                        DefinitionTask::CacheHit(entry.clone())
                    } else {
                        DefinitionTask::CacheMiss(
                            this.project
                                .update(cx, |project, cx| {
                                    project.definitions(&buffer, identifier.range.start, cx)
                                })
                                .ok()?,
                        )
                    };

                    let cx = async_cx.clone();
                    let project = project.clone();
                    Some(async move {
                        match task {
                            DefinitionTask::CacheHit(cache_entry) => {
                                Some((identifier, cache_entry, None))
                            }
                            DefinitionTask::CacheMiss(task) => {
                                let locations = task.await.log_err()??;
                                let duration = start_time.elapsed();
                                Some(cx.update(|cx| {
                                    (
                                        identifier,
                                        Arc::new(CacheEntry {
                                            definitions: locations
                                                .into_iter()
                                                .filter_map(|location| {
                                                    process_definition(location, &project, cx)
                                                })
                                                .collect(),
                                        }),
                                        Some(duration),
                                    )
                                }))
                            }
                        }
                    })
                })
                .collect::<Vec<_>>()
        })?;

        let mut cache_hit_count = 0;
        let mut cache_miss_count = 0;
        let mut mean_definition_latency = Duration::ZERO;
        let mut max_definition_latency = Duration::ZERO;
        let mut new_cache = HashMap::default();
        new_cache.reserve(futures.len());
        for (identifier, entry, duration) in future::join_all(futures).await.into_iter().flatten() {
            new_cache.insert(identifier, entry);
            if let Some(duration) = duration {
                cache_miss_count += 1;
                mean_definition_latency += duration;
                max_definition_latency = max_definition_latency.max(duration);
            } else {
                cache_hit_count += 1;
            }
        }
        mean_definition_latency /= cache_miss_count.max(1) as u32;

        let (new_cache, related_buffers) = rebuild_related_files(&project, new_cache, cx).await?;

        if let Some(file) = &file {
            log::debug!(
                "finished retrieving context buffer:{}, latency:{:?}",
                file.path().as_unix_str(),
                start_time.elapsed()
            );
        }

        this.update(cx, |this, cx| {
            this.cache = new_cache;
            this.related_buffers = related_buffers;
            cx.emit(RelatedExcerptStoreEvent::FinishedRefresh {
                cache_hit_count,
                cache_miss_count,
                mean_definition_latency,
                max_definition_latency,
            });
        })?;

        anyhow::Ok(())
    }
}

async fn rebuild_related_files(
    project: &Entity<Project>,
    mut new_entries: HashMap<Identifier, Arc<CacheEntry>>,
    cx: &mut AsyncApp,
) -> Result<(HashMap<Identifier, Arc<CacheEntry>>, Vec<RelatedBuffer>)> {
    let mut snapshots = HashMap::default();
    let mut worktree_root_names = HashMap::default();
    for entry in new_entries.values() {
        for definition in &entry.definitions {
            if let hash_map::Entry::Vacant(e) = snapshots.entry(definition.buffer.entity_id()) {
                definition
                    .buffer
                    .read_with(cx, |buffer, _| buffer.parsing_idle())
                    .await;
                e.insert(
                    definition
                        .buffer
                        .read_with(cx, |buffer, _| buffer.snapshot()),
                );
            }
            let worktree_id = definition.path.worktree_id;
            if let hash_map::Entry::Vacant(e) =
                worktree_root_names.entry(definition.path.worktree_id)
            {
                project.read_with(cx, |project, cx| {
                    if let Some(worktree) = project.worktree_for_id(worktree_id, cx) {
                        e.insert(worktree.read(cx).root_name().as_unix_str().to_string());
                    }
                });
            }
        }
    }

    Ok(cx
        .background_spawn(async move {
            let mut ranges_by_buffer =
                HashMap::<EntityId, (Entity<Buffer>, Vec<Range<Point>>)>::default();
            let mut paths_by_buffer = HashMap::default();
            for entry in new_entries.values_mut() {
                for definition in &entry.definitions {
                    let Some(snapshot) = snapshots.get(&definition.buffer.entity_id()) else {
                        continue;
                    };
                    paths_by_buffer.insert(definition.buffer.entity_id(), definition.path.clone());

                    ranges_by_buffer
                        .entry(definition.buffer.entity_id())
                        .or_insert_with(|| (definition.buffer.clone(), Vec::new()))
                        .1
                        .push(definition.anchor_range.to_point(snapshot));
                }
            }

            let mut related_buffers: Vec<RelatedBuffer> = ranges_by_buffer
                .into_iter()
                .filter_map(|(entity_id, (buffer, ranges))| {
                    let snapshot = snapshots.get(&entity_id)?;
                    let project_path = paths_by_buffer.get(&entity_id)?;
                    let row_ranges = assemble_excerpt_ranges(snapshot, ranges);
                    let root_name = worktree_root_names.get(&project_path.worktree_id)?;

                    let path: Arc<Path> = Path::new(&format!(
                        "{}/{}",
                        root_name,
                        project_path.path.as_unix_str()
                    ))
                    .into();

                    let anchor_ranges = row_ranges
                        .into_iter()
                        .map(|row_range| {
                            let start = snapshot.anchor_before(Point::new(row_range.start, 0));
                            let end_col = snapshot.line_len(row_range.end);
                            let end = snapshot.anchor_after(Point::new(row_range.end, end_col));
                            start..end
                        })
                        .collect();

                    let mut related_buffer = RelatedBuffer {
                        buffer,
                        path,
                        anchor_ranges,
                        cached_file: None,
                    };
                    related_buffer.fill_cache(snapshot);
                    Some(related_buffer)
                })
                .collect();

            related_buffers.sort_by_key(|related| related.path.clone());

            (new_entries, related_buffers)
        })
        .await)
}

impl RelatedBuffer {
    fn related_file(&mut self, cx: &App) -> RelatedFile {
        let buffer = self.buffer.read(cx);
        let path = self.path.clone();
        let cached = if let Some(cached) = &self.cached_file
            && buffer.version() == cached.buffer_version
        {
            cached
        } else {
            self.fill_cache(buffer)
        };
        let related_file = RelatedFile {
            path,
            excerpts: cached.excerpts.clone(),
            max_row: buffer.max_point().row,
        };
        return related_file;
    }

    fn fill_cache(&mut self, buffer: &text::BufferSnapshot) -> &CachedRelatedFile {
        let excerpts = self
            .anchor_ranges
            .iter()
            .map(|range| {
                let start = range.start.to_point(buffer);
                let end = range.end.to_point(buffer);
                RelatedExcerpt {
                    row_range: start.row..end.row,
                    text: buffer.text_for_range(start..end).collect::<String>().into(),
                }
            })
            .collect::<Vec<_>>();
        self.cached_file = Some(CachedRelatedFile {
            excerpts: excerpts,
            buffer_version: buffer.version().clone(),
        });
        self.cached_file.as_ref().unwrap()
    }
}

use language::ToPoint as _;

const MAX_TARGET_LEN: usize = 128;

fn process_definition(
    location: LocationLink,
    project: &Entity<Project>,
    cx: &mut App,
) -> Option<CachedDefinition> {
    let buffer = location.target.buffer.read(cx);
    let anchor_range = location.target.range;
    let file = buffer.file()?;
    let worktree = project.read(cx).worktree_for_id(file.worktree_id(cx), cx)?;
    if worktree.read(cx).is_single_file() {
        return None;
    }

    // If the target range is large, it likely means we requested the definition of an entire module.
    // For individual definitions, the target range should be small as it only covers the symbol.
    let buffer = location.target.buffer.read(cx);
    let target_len = anchor_range.to_offset(&buffer).len();
    if target_len > MAX_TARGET_LEN {
        return None;
    }

    Some(CachedDefinition {
        path: ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        },
        buffer: location.target.buffer,
        anchor_range,
    })
}

/// Gets all of the identifiers that are present in the given line, and its containing
/// outline items.
fn identifiers_for_position(
    buffer: &BufferSnapshot,
    position: Anchor,
    identifier_line_count: u32,
) -> Vec<Identifier> {
    let offset = position.to_offset(buffer);
    let point = buffer.offset_to_point(offset);

    // Search for identifiers on lines adjacent to the cursor.
    let start = Point::new(point.row.saturating_sub(identifier_line_count), 0);
    let end = Point::new(point.row + identifier_line_count + 1, 0).min(buffer.max_point());
    let line_range = start..end;
    let mut ranges = vec![line_range.to_offset(&buffer)];

    // Search for identifiers mentioned in headers/signatures of containing outline items.
    let outline_items = buffer.outline_items_as_offsets_containing(offset..offset, false, None);
    for item in outline_items {
        if let Some(body_range) = item.body_range(&buffer) {
            ranges.push(item.range.start..body_range.start.to_offset(&buffer));
        } else {
            ranges.push(item.range.clone());
        }
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    ranges.dedup_by(|a, b| {
        if a.start <= b.end {
            b.start = b.start.min(a.start);
            b.end = b.end.max(a.end);
            true
        } else {
            false
        }
    });

    let mut identifiers = Vec::new();
    let outer_range =
        ranges.first().map_or(0, |r| r.start)..ranges.last().map_or(buffer.len(), |r| r.end);

    let mut captures = buffer
        .syntax
        .captures(outer_range.clone(), &buffer.text, |grammar| {
            grammar
                .highlights_config
                .as_ref()
                .map(|config| &config.query)
        });

    for range in ranges {
        captures.set_byte_range(range.start..outer_range.end);

        let mut last_range = None;
        while let Some(capture) = captures.peek() {
            let node_range = capture.node.byte_range();
            if node_range.start > range.end {
                break;
            }
            let config = captures.grammars()[capture.grammar_index]
                .highlights_config
                .as_ref();

            if let Some(config) = config
                && config.identifier_capture_indices.contains(&capture.index)
                && range.contains_inclusive(&node_range)
                && Some(&node_range) != last_range.as_ref()
            {
                let name = buffer.text_for_range(node_range.clone()).collect();
                identifiers.push(Identifier {
                    range: buffer.anchor_after(node_range.start)
                        ..buffer.anchor_before(node_range.end),
                    name,
                });
                last_range = Some(node_range);
            }

            captures.advance();
        }
    }

    identifiers
}
