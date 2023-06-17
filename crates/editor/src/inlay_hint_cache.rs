use std::ops::Range;

use crate::{
    editor_settings, scroll::ScrollAnchor, Anchor, Editor, ExcerptId, InlayId, MultiBuffer,
};
use anyhow::Context;
use clock::Global;
use gpui::{ModelHandle, Task, ViewContext};
use language::Buffer;
use log::error;
use project::{InlayHint, InlayHintKind};

use collections::{hash_map, HashMap, HashSet};
use util::post_inc;

#[derive(Debug, Copy, Clone)]
pub enum InlayRefreshReason {
    SettingsChange(editor_settings::InlayHints),
    Scroll(ScrollAnchor),
    VisibleExcerptsChange,
}

#[derive(Debug, Default)]
pub struct InlayHintCache {
    inlay_hints: HashMap<InlayId, InlayHint>,
    hints_in_buffers: HashMap<u64, BufferHints<(Anchor, InlayId)>>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
}

#[derive(Clone, Debug)]
struct BufferHints<I> {
    buffer_version: Global,
    hints_per_excerpt: HashMap<ExcerptId, ExcerptHints<I>>,
}

#[derive(Clone, Debug)]
struct ExcerptHints<I> {
    cached_excerpt_offsets: Vec<Range<usize>>,
    hints: Vec<I>,
}

impl<I> Default for ExcerptHints<I> {
    fn default() -> Self {
        Self {
            cached_excerpt_offsets: Vec::new(),
            hints: Vec::new(),
        }
    }
}

impl<I> BufferHints<I> {
    fn new(buffer_version: Global) -> Self {
        Self {
            buffer_version,
            hints_per_excerpt: HashMap::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<(InlayId, Anchor, InlayHint)>,
}

pub struct InlayHintQuery {
    pub buffer_id: u64,
    pub buffer_version: Global,
    pub excerpt_id: ExcerptId,
    pub excerpt_offset_query_range: Range<usize>,
}

impl InlayHintCache {
    pub fn new(inlay_hint_settings: editor_settings::InlayHints) -> Self {
        Self {
            allowed_hint_kinds: allowed_inlay_hint_types(inlay_hint_settings),
            hints_in_buffers: HashMap::default(),
            inlay_hints: HashMap::default(),
        }
    }

    pub fn apply_settings(
        &mut self,
        inlay_hint_settings: editor_settings::InlayHints,
        currently_visible_ranges: Vec<(ModelHandle<Buffer>, Range<usize>, ExcerptId)>,
        mut currently_shown_inlay_hints: HashMap<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<InlaySplice> {
        let new_allowed_hint_kinds = allowed_inlay_hint_types(inlay_hint_settings);
        if new_allowed_hint_kinds == self.allowed_hint_kinds {
            None
        } else {
            self.allowed_hint_kinds = new_allowed_hint_kinds;
            let mut to_remove = Vec::new();
            let mut to_insert = Vec::new();

            let mut considered_hints =
                HashMap::<u64, HashMap<ExcerptId, HashSet<InlayId>>>::default();
            for (visible_buffer, _, visible_excerpt_id) in currently_visible_ranges {
                let visible_buffer = visible_buffer.read(cx);
                let visible_buffer_id = visible_buffer.remote_id();
                match currently_shown_inlay_hints.entry(visible_buffer_id) {
                    hash_map::Entry::Occupied(mut o) => {
                        let shown_hints_per_excerpt = o.get_mut();
                        for (_, shown_hint_id) in shown_hints_per_excerpt
                            .remove(&visible_excerpt_id)
                            .unwrap_or_default()
                        {
                            considered_hints
                                .entry(visible_buffer_id)
                                .or_default()
                                .entry(visible_excerpt_id)
                                .or_default()
                                .insert(shown_hint_id);
                            match self.inlay_hints.get(&shown_hint_id) {
                                Some(shown_hint) => {
                                    if !self.allowed_hint_kinds.contains(&shown_hint.kind) {
                                        to_remove.push(shown_hint_id);
                                    }
                                }
                                None => to_remove.push(shown_hint_id),
                            }
                        }
                        if shown_hints_per_excerpt.is_empty() {
                            o.remove();
                        }
                    }
                    hash_map::Entry::Vacant(_) => {}
                }
            }

            let reenabled_hints = self
                .hints_in_buffers
                .iter()
                .filter_map(|(cached_buffer_id, cached_hints_per_excerpt)| {
                    let considered_hints_in_excerpts = considered_hints.get(cached_buffer_id)?;
                    let not_considered_cached_hints = cached_hints_per_excerpt
                        .hints_per_excerpt
                        .iter()
                        .filter_map(|(cached_excerpt_id, cached_excerpt_hints)| {
                            let considered_excerpt_hints =
                                considered_hints_in_excerpts.get(&cached_excerpt_id)?;
                            let not_considered_cached_hints = cached_excerpt_hints
                                .hints
                                .iter()
                                .filter(|(_, cached_hint_id)| {
                                    !considered_excerpt_hints.contains(cached_hint_id)
                                })
                                .copied();
                            Some(not_considered_cached_hints)
                        })
                        .flatten();
                    Some(not_considered_cached_hints)
                })
                .flatten()
                .filter_map(|(cached_anchor, cached_inlay_id)| {
                    Some((
                        cached_anchor,
                        cached_inlay_id,
                        self.inlay_hints.get(&cached_inlay_id)?,
                    ))
                })
                .filter(|(_, _, cached_inlay)| self.allowed_hint_kinds.contains(&cached_inlay.kind))
                .map(|(cached_anchor, cached_inlay_id, reenabled_inlay)| {
                    (cached_inlay_id, cached_anchor, reenabled_inlay.clone())
                });
            to_insert.extend(reenabled_hints);

            to_remove.extend(
                currently_shown_inlay_hints
                    .into_iter()
                    .flat_map(|(_, hints_by_excerpt)| hints_by_excerpt)
                    .flat_map(|(_, excerpt_hints)| excerpt_hints)
                    .map(|(_, hint_id)| hint_id),
            );

            Some(InlaySplice {
                to_remove,
                to_insert,
            })
        }
    }

    pub fn clear(&mut self) -> Vec<InlayId> {
        let ids_to_remove = self.inlay_hints.drain().map(|(id, _)| id).collect();
        self.hints_in_buffers.clear();
        ids_to_remove
    }

    pub fn append_hints(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        ranges_to_add: impl Iterator<Item = InlayHintQuery>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        let queries = ranges_to_add.filter_map(|additive_query| {
            let Some(cached_buffer_hints) = self.hints_in_buffers.get(&additive_query.buffer_id)
                else { return Some(vec![additive_query]) };
            if cached_buffer_hints.buffer_version.changed_since(&additive_query.buffer_version) {
                return None
            }
            let Some(excerpt_hints) = cached_buffer_hints.hints_per_excerpt.get(&additive_query.excerpt_id)
                else { return Some(vec![additive_query]) };
            let non_cached_ranges = missing_subranges(&excerpt_hints.cached_excerpt_offsets, &additive_query.excerpt_offset_query_range);
            if non_cached_ranges.is_empty() {
                None
            } else {
                Some(non_cached_ranges.into_iter().map(|non_cached_range| InlayHintQuery {
                    buffer_id: additive_query.buffer_id,
                    buffer_version: additive_query.buffer_version.clone(),
                    excerpt_id: additive_query.excerpt_id,
                    excerpt_offset_query_range: non_cached_range,
                }).collect())
            }
        }).flatten();

        let task_multi_buffer = multi_buffer.clone();
        let fetch_queries_task = fetch_queries(multi_buffer, queries, cx);
        cx.spawn(|editor, mut cx| async move {
            let new_hints = fetch_queries_task.await?;
            editor.update(&mut cx, |editor, cx| {
                let multi_buffer_snapshot = task_multi_buffer.read(cx).snapshot(cx);
                let mut to_insert = Vec::new();
                for (new_buffer_id, new_hints_per_buffer) in new_hints {
                    let cached_buffer_hints = editor
                        .inlay_hint_cache
                        .hints_in_buffers
                        .entry(new_buffer_id)
                        .or_insert_with(|| {
                            BufferHints::new(new_hints_per_buffer.buffer_version.clone())
                        });
                    if cached_buffer_hints
                        .buffer_version
                        .changed_since(&new_hints_per_buffer.buffer_version)
                    {
                        continue;
                    }

                    for (new_excerpt_id, new_excerpt_hints) in
                        new_hints_per_buffer.hints_per_excerpt
                    {
                        let cached_excerpt_hints = cached_buffer_hints
                            .hints_per_excerpt
                            .entry(new_excerpt_id)
                            .or_insert_with(|| ExcerptHints::default());
                        for new_range in new_excerpt_hints.cached_excerpt_offsets {
                            insert_and_merge_ranges(
                                &mut cached_excerpt_hints.cached_excerpt_offsets,
                                &new_range,
                            )
                        }
                        for new_inlay_hint in new_excerpt_hints.hints {
                            let hint_anchor = multi_buffer_snapshot
                                .anchor_in_excerpt(new_excerpt_id, new_inlay_hint.position);
                            let insert_ix =
                                match cached_excerpt_hints.hints.binary_search_by(|probe| {
                                    hint_anchor.cmp(&probe.0, &multi_buffer_snapshot)
                                }) {
                                    Ok(ix) | Err(ix) => ix,
                                };

                            let new_inlay_id = InlayId(post_inc(&mut editor.next_inlay_id));
                            cached_excerpt_hints
                                .hints
                                .insert(insert_ix, (hint_anchor, new_inlay_id));
                            editor
                                .inlay_hint_cache
                                .inlay_hints
                                .insert(new_inlay_id, new_inlay_hint.clone());
                            if editor
                                .inlay_hint_cache
                                .allowed_hint_kinds
                                .contains(&new_inlay_hint.kind)
                            {
                                to_insert.push((new_inlay_id, hint_anchor, new_inlay_hint));
                            }
                        }
                    }
                }

                InlaySplice {
                    to_remove: Vec::new(),
                    to_insert,
                }
            })
        })
    }

    pub fn replace_hints(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        new_ranges: impl Iterator<Item = InlayHintQuery>,
        currently_shown_inlay_hints: HashMap<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        // let inlay_queries_per_buffer = inlay_queries.fold(
        //     HashMap::<u64, BufferInlays<InlayHintQuery>>::default(),
        //     |mut queries, new_query| {
        //         let mut buffer_queries = queries
        //             .entry(new_query.buffer_id)
        //             .or_insert_with(|| BufferInlays::new(new_query.buffer_version.clone()));
        //         assert_eq!(buffer_queries.buffer_version, new_query.buffer_version);
        //         let queries = buffer_queries
        //             .excerpt_inlays
        //             .entry(new_query.excerpt_id)
        //             .or_default();
        //         // let z = multi_buffer_snapshot.anchor_in_excerpt(new_query.excerpt_id, text_anchor);
        //         // .push(new_query);
        //         // match queries
        //         //     .binary_search_by(|probe| inlay.position.cmp(&probe.0, &multi_buffer_snapshot))
        //         // {
        //         //     Ok(ix) | Err(ix) => {
        //         //         excerpt_hints.insert(ix, (inlay.position, inlay.id));
        //         //     }
        //         // }
        //         // queries
        //         todo!("TODO kb")
        //     },
        // );

        todo!("TODO kb")
    }
}

fn allowed_inlay_hint_types(
    inlay_hint_settings: editor_settings::InlayHints,
) -> HashSet<Option<InlayHintKind>> {
    let mut new_allowed_inlay_hint_types = HashSet::default();
    if inlay_hint_settings.show_type_hints {
        new_allowed_inlay_hint_types.insert(Some(InlayHintKind::Type));
    }
    if inlay_hint_settings.show_parameter_hints {
        new_allowed_inlay_hint_types.insert(Some(InlayHintKind::Parameter));
    }
    if inlay_hint_settings.show_other_hints {
        new_allowed_inlay_hint_types.insert(None);
    }
    new_allowed_inlay_hint_types
}

fn missing_subranges(cache: &[Range<usize>], input: &Range<usize>) -> Vec<Range<usize>> {
    let mut missing = Vec::new();

    // Find where the input range would fit in the cache
    let index = match cache.binary_search_by_key(&input.start, |probe| probe.start) {
        Ok(pos) | Err(pos) => pos,
    };

    // Check for a gap from the start of the input range to the first range in the cache
    if index == 0 {
        if input.start < cache[index].start {
            missing.push(input.start..cache[index].start);
        }
    } else {
        let prev_end = cache[index - 1].end;
        if input.start < prev_end {
            missing.push(input.start..prev_end);
        }
    }

    // Iterate through the cache ranges starting from index
    for i in index..cache.len() {
        let start = if i > 0 { cache[i - 1].end } else { input.start };
        let end = cache[i].start;

        if start < end {
            missing.push(start..end);
        }
    }

    // Check for a gap from the last range in the cache to the end of the input range
    if let Some(last_range) = cache.last() {
        if last_range.end < input.end {
            missing.push(last_range.end..input.end);
        }
    } else {
        // If cache is empty, the entire input range is missing
        missing.push(input.start..input.end);
    }

    missing
}

fn insert_and_merge_ranges(cache: &mut Vec<Range<usize>>, new_range: &Range<usize>) {
    if cache.is_empty() {
        cache.push(new_range.clone());
        return;
    }

    // Find the index to insert the new range
    let index = match cache.binary_search_by_key(&new_range.start, |probe| probe.start) {
        Ok(pos) | Err(pos) => pos,
    };

    // Check if the new range overlaps with the previous range in the cache
    if index > 0 && cache[index - 1].end >= new_range.start {
        // Merge with the previous range
        cache[index - 1].end = std::cmp::max(cache[index - 1].end, new_range.end);
    } else {
        // Insert the new range, as it doesn't overlap with the previous range
        cache.insert(index, new_range.clone());
    }

    // Merge overlaps with subsequent ranges
    let mut i = index;
    while i + 1 < cache.len() && cache[i].end >= cache[i + 1].start {
        cache[i].end = std::cmp::max(cache[i].end, cache[i + 1].end);
        cache.remove(i + 1);
        i += 1;
    }
}

fn fetch_queries<'a, 'b>(
    multi_buffer: ModelHandle<MultiBuffer>,
    queries: impl Iterator<Item = InlayHintQuery>,
    cx: &mut ViewContext<'a, 'b, Editor>,
) -> Task<anyhow::Result<HashMap<u64, BufferHints<InlayHint>>>> {
    let mut inlay_fetch_tasks = Vec::new();
    for query in queries {
        let task_multi_buffer = multi_buffer.clone();
        let task = cx.spawn(|editor, mut cx| async move {
            let Some(buffer_handle) = cx.read(|cx| task_multi_buffer.read(cx).buffer(query.buffer_id))
                else { return anyhow::Ok((query, Some(Vec::new()))) };
            let task = editor
                .update(&mut cx, |editor, cx| {
                    editor.project.as_ref().map(|project| {
                        project.update(cx, |project, cx| {
                            project.query_inlay_hints_for_buffer(
                                buffer_handle,
                                query.excerpt_offset_query_range.clone(),
                                cx,
                            )
                        })
                    })
                })
                .context("inlays fecth task spawn")?;
            Ok((
                query,
                match task {
                    Some(task) => task.await.context("inlays for buffer task")?,
                    None => Some(Vec::new()),
                },
            ))
        });

        inlay_fetch_tasks.push(task);
    }

    cx.spawn(|editor, cx| async move {
        let mut inlay_updates: HashMap<u64, BufferHints<InlayHint>> = HashMap::default();
        for task_result in futures::future::join_all(inlay_fetch_tasks).await {
            match task_result {
                Ok((query, Some(response_hints))) => {
                    let Some(buffer_snapshot) = editor.read_with(&cx, |editor, cx| {
                        editor.buffer().read(cx).buffer(query.buffer_id).map(|buffer| buffer.read(cx).snapshot())
                    })? else { continue; };
                    let buffer_hints = inlay_updates
                        .entry(query.buffer_id)
                        .or_insert_with(|| BufferHints::new(query.buffer_version.clone()));
                    if buffer_snapshot.version().changed_since(&buffer_hints.buffer_version) {
                        continue;
                    }
                    let cached_excerpt_hints = buffer_hints
                        .hints_per_excerpt
                        .entry(query.excerpt_id)
                        .or_default();
                    insert_and_merge_ranges(&mut cached_excerpt_hints.cached_excerpt_offsets, &query.excerpt_offset_query_range);
                    let excerpt_hints = &mut cached_excerpt_hints.hints;
                    for inlay in response_hints {
                        match excerpt_hints.binary_search_by(|probe| {
                            inlay.position.cmp(&probe.position, &buffer_snapshot)
                        }) {
                            Ok(ix) | Err(ix) => excerpt_hints.insert(ix, inlay),
                        }
                    }
                }
                Ok((_, None)) => {}
                Err(e) => error!("Failed to update inlays for buffer: {e:#}"),
            }
        }
        Ok(inlay_updates)
    })
}
