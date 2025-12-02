use anyhow::Result;
use collections::HashMap;
use fs::MTime;
use futures::{FutureExt, StreamExt as _, channel::mpsc, future};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use language::{Anchor, Buffer, BufferSnapshot, OffsetRangeExt as _, Point, Rope, ToOffset as _};
use project::{LocationLink, Project, ProjectPath};
use smallvec::SmallVec;
use std::{ops::Range, sync::Arc, time::Duration};
use util::{RangeExt as _, ResultExt};

#[cfg(test)]
mod edit_prediction_context_tests;
#[cfg(test)]
mod fake_definition_lsp;

pub struct RelatedExcerptStore {
    project: Entity<Project>,
    related_files: Vec<RelatedFile>,
    cache: HashMap<Identifier, Arc<CacheEntry>>,
    update_tx: mpsc::UnboundedSender<(Entity<Buffer>, Anchor)>,
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
    buffer: WeakEntity<Buffer>,
    anchor_range: Range<Anchor>,
    point_range: Range<Point>,
    text: Rope,
    mtime: MTime,
}

pub struct RelatedFile {
    pub path: ProjectPath,
    pub buffer: WeakEntity<Buffer>,
    pub excerpts: Vec<RelatedExcerpt>,
}

pub struct RelatedExcerpt {
    pub anchor_range: Range<Anchor>,
    pub point_range: Range<Point>,
    pub text: Rope,
}

const DEBOUNCE_DURATION: Duration = Duration::from_millis(100);

impl RelatedExcerptStore {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
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
            project,
            update_tx,
            related_files: Vec::new(),
            cache: Default::default(),
        }
    }

    pub fn cursor_moved(
        &mut self,
        buffer: Entity<Buffer>,
        position: Anchor,
        _: &mut Context<Self>,
    ) {
        self.update_tx.unbounded_send((buffer, position)).ok();
    }

    pub fn related_files(&self) -> &Vec<RelatedFile> {
        &self.related_files
    }

    async fn fetch_excerpts(
        this: WeakEntity<Self>,
        buffer: Entity<Buffer>,
        position: Anchor,
        cx: &mut AsyncApp,
    ) -> Result<()> {
        let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        let identifiers = cx
            .background_spawn(async move { identifiers_for_position(&snapshot, position) })
            .await;

        let async_cx = cx.clone();
        let new_cache = this.update(cx, |this, cx| {
            identifiers
                .into_iter()
                .map(|identifier| {
                    let task = if let Some(entry) = this.cache.get(&identifier) {
                        DefinitionTask::CacheHit(entry.clone())
                    } else {
                        DefinitionTask::CacheMiss(this.project.update(cx, |project, cx| {
                            project.definitions(&buffer, identifier.range.start, cx)
                        }))
                    };

                    let cx = async_cx.clone();
                    let project = this.project.clone();
                    async move {
                        match task {
                            DefinitionTask::CacheHit(cache_entry) => {
                                Some((identifier, cache_entry))
                            }
                            DefinitionTask::CacheMiss(task) => {
                                let locations = task.await.log_err()??;
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
                                    )
                                })
                                .ok()
                            }
                        }
                    }
                })
                .collect::<Vec<_>>()
        })?;

        let new_cache = future::join_all(new_cache)
            .await
            .into_iter()
            .flatten()
            .collect::<HashMap<_, _>>();

        let (related_files, new_cache) = cx
            .background_spawn(async move { (rebuild_related_files(&new_cache), new_cache) })
            .await;

        this.update(cx, |this, cx| {
            this.cache = new_cache;
            this.related_files = related_files;
            cx.notify();
        })?;

        anyhow::Ok(())
    }
}

fn rebuild_related_files(new_entries: &HashMap<Identifier, Arc<CacheEntry>>) -> Vec<RelatedFile> {
    let mut files = Vec::<RelatedFile>::new();
    for entry in new_entries.values() {
        for definition in &entry.definitions {
            let excerpt = RelatedExcerpt {
                anchor_range: definition.anchor_range.clone(),
                point_range: definition.point_range.clone(),
                text: definition.text.clone(),
            };
            if let Some(file) = files
                .iter_mut()
                .find(|existing| existing.path == definition.path)
            {
                file.excerpts.push(excerpt);
            } else {
                files.push(RelatedFile {
                    path: definition.path.clone(),
                    buffer: definition.buffer.clone(),
                    excerpts: vec![excerpt],
                })
            }
        }
    }

    for file in &mut files {
        file.excerpts.sort_by(|a, b| {
            a.point_range
                .start
                .cmp(&b.point_range.start)
                .then(b.point_range.end.cmp(&a.point_range.end))
        });
        file.excerpts.dedup_by(|a, b| {
            if a.point_range.start <= b.point_range.end {
                b.point_range.start = b.point_range.start.min(a.point_range.start);
                b.point_range.end = b.point_range.end.max(a.point_range.end);
                true
            } else {
                false
            }
        });
    }

    files.sort_by_key(|file| file.path.clone());
    files
}

fn process_definition(
    location: LocationLink,
    project: &Entity<Project>,
    cx: &mut App,
) -> Option<CachedDefinition> {
    let buffer = location.target.buffer.read(cx);
    let range = location.target.range;
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
        buffer: location.target.buffer.downgrade(),
        anchor_range: range.clone(),
        point_range: range.to_point(buffer),
        text: buffer.as_rope().slice(range.to_offset(buffer)),
        mtime: file.disk_state().mtime()?,
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
                    range: buffer.anchor_before(node_range.start)
                        ..buffer.anchor_after(node_range.end),
                    name,
                });
                last_range = Some(node_range);
            }

            captures.advance();
        }
    }

    identifiers
}
