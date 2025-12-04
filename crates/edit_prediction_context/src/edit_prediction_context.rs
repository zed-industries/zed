use crate::assemble_excerpts::assemble_excerpts;
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt, StreamExt as _, channel::mpsc, future};
use gpui::{App, AppContext, AsyncApp, Context, Entity, EventEmitter, Task, WeakEntity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, Point, Rope, ToOffset as _};
use project::{LocationLink, Project, ProjectPath};
use serde::{Serialize, Serializer};
use smallvec::SmallVec;
use std::{
    collections::hash_map,
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};
use util::{RangeExt as _, ResultExt};

mod assemble_excerpts;
#[cfg(test)]
mod edit_prediction_context_tests;
mod excerpt;
#[cfg(test)]
mod fake_definition_lsp;

pub use cloud_llm_client::predict_edits_v3::Line;
pub use excerpt::{EditPredictionExcerpt, EditPredictionExcerptOptions, EditPredictionExcerptText};

pub struct RelatedExcerptStore {
    project: WeakEntity<Project>,
    related_files: Vec<RelatedFile>,
    cache: HashMap<Identifier, Arc<CacheEntry>>,
    update_tx: mpsc::UnboundedSender<(Entity<Buffer>, Anchor)>,
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

#[derive(Clone, Debug, Serialize)]
pub struct RelatedFile {
    #[serde(serialize_with = "serialize_project_path")]
    pub path: ProjectPath,
    #[serde(skip)]
    pub buffer: WeakEntity<Buffer>,
    pub excerpts: Vec<RelatedExcerpt>,
    pub max_row: u32,
}

impl RelatedFile {
    pub fn merge_excerpts(&mut self) {
        self.excerpts.sort_unstable_by(|a, b| {
            a.point_range
                .start
                .cmp(&b.point_range.start)
                .then(b.point_range.end.cmp(&a.point_range.end))
        });

        let mut index = 1;
        while index < self.excerpts.len() {
            if self.excerpts[index - 1]
                .point_range
                .end
                .cmp(&self.excerpts[index].point_range.start)
                .is_ge()
            {
                let removed = self.excerpts.remove(index);
                if removed
                    .point_range
                    .end
                    .cmp(&self.excerpts[index - 1].point_range.end)
                    .is_gt()
                {
                    self.excerpts[index - 1].point_range.end = removed.point_range.end;
                    self.excerpts[index - 1].anchor_range.end = removed.anchor_range.end;
                }
            } else {
                index += 1;
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RelatedExcerpt {
    #[serde(skip)]
    pub anchor_range: Range<Anchor>,
    #[serde(serialize_with = "serialize_point_range")]
    pub point_range: Range<Point>,
    #[serde(serialize_with = "serialize_rope")]
    pub text: Rope,
}

fn serialize_project_path<S: Serializer>(
    project_path: &ProjectPath,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    project_path.path.serialize(serializer)
}

fn serialize_rope<S: Serializer>(rope: &Rope, serializer: S) -> Result<S::Ok, S::Error> {
    rope.to_string().serialize(serializer)
}

fn serialize_point_range<S: Serializer>(
    range: &Range<Point>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    [
        [range.start.row, range.start.column],
        [range.end.row, range.end.column],
    ]
    .serialize(serializer)
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
            related_files: Vec::new(),
            cache: Default::default(),
        }
    }

    pub fn refresh(&mut self, buffer: Entity<Buffer>, position: Anchor, _: &mut Context<Self>) {
        self.update_tx.unbounded_send((buffer, position)).ok();
    }

    pub fn related_files(&self) -> &[RelatedFile] {
        &self.related_files
    }

    async fn fetch_excerpts(
        this: WeakEntity<Self>,
        buffer: Entity<Buffer>,
        position: Anchor,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let (project, snapshot) = this.read_with(cx, |this, cx| {
            (this.project.upgrade(), buffer.read(cx).snapshot())
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
            .background_spawn(async move { identifiers_for_position(&snapshot, position) })
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
                                cx.update(|cx| {
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
                                })
                                .ok()
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

        let (new_cache, related_files) = rebuild_related_files(new_cache, cx).await?;

        if let Some(file) = &file {
            log::debug!(
                "finished retrieving context buffer:{}, latency:{:?}",
                file.path().as_unix_str(),
                start_time.elapsed()
            );
        }

        this.update(cx, |this, cx| {
            this.cache = new_cache;
            this.related_files = related_files;
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
    new_entries: HashMap<Identifier, Arc<CacheEntry>>,
    cx: &mut AsyncApp,
) -> Result<(HashMap<Identifier, Arc<CacheEntry>>, Vec<RelatedFile>)> {
    let mut snapshots = HashMap::default();
    for entry in new_entries.values() {
        for definition in &entry.definitions {
            if let hash_map::Entry::Vacant(e) = snapshots.entry(definition.buffer.entity_id()) {
                definition
                    .buffer
                    .read_with(cx, |buffer, _| buffer.parsing_idle())?
                    .await;
                e.insert(
                    definition
                        .buffer
                        .read_with(cx, |buffer, _| buffer.snapshot())?,
                );
            }
        }
    }

    Ok(cx
        .background_spawn(async move {
            let mut files = Vec::<RelatedFile>::new();
            let mut ranges_by_buffer = HashMap::<_, Vec<Range<Point>>>::default();
            let mut paths_by_buffer = HashMap::default();
            for entry in new_entries.values() {
                for definition in &entry.definitions {
                    let Some(snapshot) = snapshots.get(&definition.buffer.entity_id()) else {
                        continue;
                    };
                    paths_by_buffer.insert(definition.buffer.entity_id(), definition.path.clone());
                    ranges_by_buffer
                        .entry(definition.buffer.clone())
                        .or_default()
                        .push(definition.anchor_range.to_point(snapshot));
                }
            }

            for (buffer, ranges) in ranges_by_buffer {
                let Some(snapshot) = snapshots.get(&buffer.entity_id()) else {
                    continue;
                };
                let Some(project_path) = paths_by_buffer.get(&buffer.entity_id()) else {
                    continue;
                };
                let excerpts = assemble_excerpts(snapshot, ranges);
                files.push(RelatedFile {
                    path: project_path.clone(),
                    buffer: buffer.downgrade(),
                    excerpts,
                    max_row: snapshot.max_point().row,
                });
            }

            files.sort_by_key(|file| file.path.clone());
            (new_entries, files)
        })
        .await)
}

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
fn identifiers_for_position(buffer: &BufferSnapshot, position: Anchor) -> Vec<Identifier> {
    let offset = position.to_offset(buffer);
    let point = buffer.offset_to_point(offset);

    let line_range = Point::new(point.row, 0)..Point::new(point.row + 1, 0).min(buffer.max_point());
    let mut ranges = vec![line_range.to_offset(&buffer)];

    // Include the range of the outline item itself, but not its body.
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
