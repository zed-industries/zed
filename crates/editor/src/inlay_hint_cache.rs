use std::cmp;

use crate::{editor_settings, Anchor, Editor, ExcerptId, InlayId, MultiBuffer};
use anyhow::Context;
use clock::Global;
use gpui::{ModelHandle, Task, ViewContext};
use log::error;
use project::{InlayHint, InlayHintKind};

use collections::{HashMap, HashSet};
use util::post_inc;

#[derive(Debug, Copy, Clone)]
pub enum InlayRefreshReason {
    SettingsChange(editor_settings::InlayHints),
    VisibleExcerptsChange,
}

#[derive(Debug, Default)]
pub struct InlayHintCache {
    inlay_hints: HashMap<InlayId, InlayHint>,
    hints_in_buffers: HashMap<u64, BufferHints<(Anchor, InlayId)>>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
}

#[derive(Clone, Debug)]
struct BufferHints<H> {
    buffer_version: Global,
    hints_per_excerpt: HashMap<ExcerptId, Vec<H>>,
}

impl<H> BufferHints<H> {
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

#[derive(Debug)]
pub struct InlayHintQuery {
    pub buffer_id: u64,
    pub buffer_version: Global,
    pub excerpt_id: ExcerptId,
}

impl InlayHintCache {
    pub fn new(inlay_hint_settings: editor_settings::InlayHints) -> Self {
        Self {
            allowed_hint_kinds: allowed_hint_types(inlay_hint_settings),
            hints_in_buffers: HashMap::default(),
            inlay_hints: HashMap::default(),
        }
    }

    pub fn apply_settings(
        &mut self,
        multi_buffer: &ModelHandle<MultiBuffer>,
        inlay_hint_settings: editor_settings::InlayHints,
        currently_shown_hints: HashMap<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<InlaySplice> {
        if !inlay_hint_settings.enabled {
            self.allowed_hint_kinds = allowed_hint_types(inlay_hint_settings);
            if self.inlay_hints.is_empty() {
                return None;
            } else {
                let to_remove = self.inlay_hints.keys().copied().collect();
                self.inlay_hints.clear();
                self.hints_in_buffers.clear();
                return Some(InlaySplice {
                    to_remove,
                    to_insert: Vec::new(),
                });
            }
        }

        let new_allowed_hint_kinds = allowed_hint_types(inlay_hint_settings);
        if new_allowed_hint_kinds == self.allowed_hint_kinds {
            None
        } else {
            let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
            let mut to_remove = Vec::new();
            let mut to_insert = Vec::new();
            let mut shown_hints_to_remove = currently_shown_hints;

            // TODO kb move into a background task
            for (buffer_id, cached_buffer_hints) in &self.hints_in_buffers {
                let shown_buffer_hints_to_remove =
                    shown_hints_to_remove.entry(*buffer_id).or_default();
                for (excerpt_id, cached_excerpt_hints) in &cached_buffer_hints.hints_per_excerpt {
                    let shown_excerpt_hints_to_remove =
                        shown_buffer_hints_to_remove.entry(*excerpt_id).or_default();
                    let mut cached_hints = cached_excerpt_hints.iter().fuse().peekable();
                    shown_excerpt_hints_to_remove.retain(|(shown_anchor, shown_hint_id)| {
                        loop {
                            match cached_hints.peek() {
                                Some((cached_anchor, cached_hint_id)) => {
                                    if cached_hint_id == shown_hint_id {
                                        return !new_allowed_hint_kinds.contains(
                                            &self.inlay_hints.get(&cached_hint_id).unwrap().kind,
                                        );
                                    }

                                    match cached_anchor.cmp(shown_anchor, &multi_buffer_snapshot) {
                                        cmp::Ordering::Less | cmp::Ordering::Equal => {
                                            let maybe_missed_cached_hint =
                                                self.inlay_hints.get(&cached_hint_id).unwrap();
                                            let cached_hint_kind = maybe_missed_cached_hint.kind;
                                            if !self.allowed_hint_kinds.contains(&cached_hint_kind)
                                                && new_allowed_hint_kinds
                                                    .contains(&cached_hint_kind)
                                            {
                                                to_insert.push((
                                                    *cached_hint_id,
                                                    *cached_anchor,
                                                    maybe_missed_cached_hint.clone(),
                                                ));
                                            }
                                            cached_hints.next();
                                        }
                                        cmp::Ordering::Greater => break,
                                    }
                                }
                                None => return true,
                            }
                        }

                        match self.inlay_hints.get(&shown_hint_id) {
                            Some(shown_hint) => !new_allowed_hint_kinds.contains(&shown_hint.kind),
                            None => true,
                        }
                    });

                    for (cached_anchor, cached_hint_id) in cached_hints {
                        let maybe_missed_cached_hint =
                            self.inlay_hints.get(&cached_hint_id).unwrap();
                        let cached_hint_kind = maybe_missed_cached_hint.kind;
                        if !self.allowed_hint_kinds.contains(&cached_hint_kind)
                            && new_allowed_hint_kinds.contains(&cached_hint_kind)
                        {
                            to_insert.push((
                                *cached_hint_id,
                                *cached_anchor,
                                maybe_missed_cached_hint.clone(),
                            ));
                        }
                    }
                }
            }

            to_remove.extend(
                shown_hints_to_remove
                    .into_iter()
                    .flat_map(|(_, hints_by_excerpt)| hints_by_excerpt)
                    .flat_map(|(_, excerpt_hints)| excerpt_hints)
                    .map(|(_, hint_id)| hint_id),
            );
            self.allowed_hint_kinds = new_allowed_hint_kinds;
            Some(InlaySplice {
                to_remove,
                to_insert,
            })
        }
    }

    pub fn update_hints(
        &mut self,
        multi_buffer: ModelHandle<MultiBuffer>,
        queries: Vec<InlayHintQuery>,
        currently_shown_hints: HashMap<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<anyhow::Result<InlaySplice>> {
        let conflicts_with_cache = queries.iter().any(|update_query| {
            let Some(cached_buffer_hints) = self.hints_in_buffers.get(&update_query.buffer_id)
                else { return false };
            if cached_buffer_hints
                .buffer_version
                .changed_since(&update_query.buffer_version)
            {
                false
            } else if update_query
                .buffer_version
                .changed_since(&cached_buffer_hints.buffer_version)
            {
                true
            } else {
                cached_buffer_hints
                    .hints_per_excerpt
                    .contains_key(&update_query.excerpt_id)
            }
        });

        // TODO kb remember queries that run and do not query for these ranges if the buffer version was not changed
        let queries = queries
            .into_iter()
            .filter_map(|query| {
                let Some(cached_buffer_hints) = self.hints_in_buffers.get(&query.buffer_id)
                    else { return Some(query) };
                if cached_buffer_hints
                    .buffer_version
                    .changed_since(&query.buffer_version)
                {
                    return None;
                }
                if conflicts_with_cache
                    || !cached_buffer_hints
                        .hints_per_excerpt
                        .contains_key(&query.excerpt_id)
                {
                    Some(query)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let task_multi_buffer = multi_buffer.clone();
        let fetch_queries_task = fetch_queries(multi_buffer, queries.into_iter(), cx);
        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        let mut cache_hints_to_persist: HashMap<
            u64,
            (Global, HashMap<ExcerptId, HashSet<InlayId>>),
        > = HashMap::default();
        cx.spawn(|editor, mut cx| async move {
            let new_hints = fetch_queries_task.await.context("inlay hints fetch")?;
            editor.update(&mut cx, |editor, cx| {
                let multi_buffer_snapshot = task_multi_buffer.read(cx).snapshot(cx);
                for (new_buffer_id, new_hints_per_buffer) in new_hints {
                    let cached_buffer_hints = editor
                        .inlay_hint_cache
                        .hints_in_buffers
                        .entry(new_buffer_id)
                        .or_insert_with(|| {
                            BufferHints::new(new_hints_per_buffer.buffer_version.clone())
                        });

                    let buffer_cache_hints_to_persist =
                        cache_hints_to_persist.entry(new_buffer_id).or_insert_with(|| (new_hints_per_buffer.buffer_version.clone(), HashMap::default()));
                    if cached_buffer_hints
                        .buffer_version
                        .changed_since(&new_hints_per_buffer.buffer_version)
                    {
                        buffer_cache_hints_to_persist.0 = new_hints_per_buffer.buffer_version;
                        buffer_cache_hints_to_persist.1.extend(
                            cached_buffer_hints.hints_per_excerpt.iter().map(
                                |(excerpt_id, excerpt_hints)| {
                                    (
                                        *excerpt_id,
                                        excerpt_hints.iter().map(|(_, id)| *id).collect(),
                                    )
                                },
                            ),
                        );
                        continue;
                    }

                    let shown_buffer_hints = currently_shown_hints.get(&new_buffer_id);
                    for (new_excerpt_id, new_hints_per_excerpt) in
                        new_hints_per_buffer.hints_per_excerpt
                    {
                        let excerpt_cache_hints_to_persist = buffer_cache_hints_to_persist.1
                            .entry(new_excerpt_id)
                            .or_default();
                        let cached_excerpt_hints = cached_buffer_hints
                            .hints_per_excerpt
                            .entry(new_excerpt_id)
                            .or_default();
                        let empty_shown_excerpt_hints = Vec::new();
                        let shown_excerpt_hints = shown_buffer_hints.and_then(|hints| hints.get(&new_excerpt_id)).unwrap_or(&empty_shown_excerpt_hints);
                        for new_hint in new_hints_per_excerpt {
                            let new_hint_anchor = multi_buffer_snapshot
                                .anchor_in_excerpt(new_excerpt_id, new_hint.position);
                            let cache_insert_ix = match cached_excerpt_hints.binary_search_by(|probe| {
                                new_hint_anchor.cmp(&probe.0, &multi_buffer_snapshot)
                            }) {
                                Ok(ix) => {
                                    let (_, cached_inlay_id) = cached_excerpt_hints[ix];
                                    let cache_hit = editor
                                        .inlay_hint_cache
                                        .inlay_hints
                                        .get(&cached_inlay_id)
                                        .filter(|cached_hint| cached_hint == &&new_hint)
                                        .is_some();
                                    if cache_hit {
                                        excerpt_cache_hints_to_persist
                                            .insert(cached_inlay_id);
                                        None
                                    } else {
                                        Some(ix)
                                    }
                                }
                                Err(ix) => Some(ix),
                            };

                            let shown_inlay_id = match shown_excerpt_hints.binary_search_by(|probe| {
                                probe.0.cmp(&new_hint_anchor, &multi_buffer_snapshot)
                            }) {
                                Ok(ix) => {{
                                    let (_, shown_inlay_id) = shown_excerpt_hints[ix];
                                    let shown_hint_found =  editor.inlay_hint_cache.inlay_hints.get(&shown_inlay_id)
                                        .filter(|cached_hint| cached_hint == &&new_hint).is_some();
                                    if shown_hint_found {
                                        Some(shown_inlay_id)
                                    } else {
                                        None
                                    }
                                }},
                                Err(_) => None,
                            };

                            if let Some(insert_ix) = cache_insert_ix {
                                let hint_id = match shown_inlay_id {
                                    Some(shown_inlay_id) => shown_inlay_id,
                                    None => {
                                        let new_hint_id = InlayId(post_inc(&mut editor.next_inlay_id));
                                        if editor.inlay_hint_cache.allowed_hint_kinds.contains(&new_hint.kind)
                                        {
                                            to_insert.push((new_hint_id, new_hint_anchor, new_hint.clone()));
                                        }
                                        new_hint_id
                                    }
                                };
                                excerpt_cache_hints_to_persist.insert(hint_id);
                                cached_excerpt_hints.insert(insert_ix, (new_hint_anchor, hint_id));
                                editor
                                    .inlay_hint_cache
                                    .inlay_hints
                                    .insert(hint_id, new_hint);
                            }
                        }
                    }
                }

                if conflicts_with_cache {
                    for (shown_buffer_id, mut shown_hints_to_clean) in currently_shown_hints {
                        match cache_hints_to_persist.get(&shown_buffer_id) {
                            Some(cached_buffer_hints) => {
                                for (persisted_id, cached_hints) in &cached_buffer_hints.1 {
                                    shown_hints_to_clean.entry(*persisted_id).or_default()
                                        .retain(|(_, shown_id)| !cached_hints.contains(shown_id));
                                }
                            },
                            None => {},
                        }
                        to_remove.extend(shown_hints_to_clean.into_iter()
                            .flat_map(|(_, excerpt_hints)| excerpt_hints.into_iter().map(|(_, hint_id)| hint_id)));
                    }

                    editor.inlay_hint_cache.hints_in_buffers.retain(|buffer_id, buffer_hints| {
                        let Some(mut buffer_hints_to_persist) = cache_hints_to_persist.remove(buffer_id) else { return false; };
                        buffer_hints.buffer_version = buffer_hints_to_persist.0;
                        buffer_hints.hints_per_excerpt.retain(|excerpt_id, excerpt_hints| {
                            let Some(excerpt_hints_to_persist) = buffer_hints_to_persist.1.remove(&excerpt_id) else { return false; };
                            excerpt_hints.retain(|(_, hint_id)| {
                                let retain = excerpt_hints_to_persist.contains(hint_id);
                                if !retain {
                                    editor
                                        .inlay_hint_cache
                                        .inlay_hints
                                        .remove(hint_id);
                                }
                                retain
                            });
                            !excerpt_hints.is_empty()
                        });
                        !buffer_hints.hints_per_excerpt.is_empty()
                    });
                }

                InlaySplice {
                    to_remove,
                    to_insert,
                }
            })
        })
    }
}

fn allowed_hint_types(
    inlay_hint_settings: editor_settings::InlayHints,
) -> HashSet<Option<InlayHintKind>> {
    let mut new_allowed_hint_types = HashSet::default();
    if inlay_hint_settings.show_type_hints {
        new_allowed_hint_types.insert(Some(InlayHintKind::Type));
    }
    if inlay_hint_settings.show_parameter_hints {
        new_allowed_hint_types.insert(Some(InlayHintKind::Parameter));
    }
    if inlay_hint_settings.show_other_hints {
        new_allowed_hint_types.insert(None);
    }
    new_allowed_hint_types
}

fn fetch_queries(
    multi_buffer: ModelHandle<MultiBuffer>,
    queries: impl Iterator<Item = InlayHintQuery>,
    cx: &mut ViewContext<'_, '_, Editor>,
) -> Task<anyhow::Result<HashMap<u64, BufferHints<InlayHint>>>> {
    let mut inlay_fetch_tasks = Vec::new();
    for query in queries {
        let task_multi_buffer = multi_buffer.clone();
        let task = cx.spawn(|editor, mut cx| async move {
            let Some(buffer_handle) = cx.read(|cx| task_multi_buffer.read(cx).buffer(query.buffer_id))
                else { return anyhow::Ok((query, Some(Vec::new()))) };
            let task = editor
                .update(&mut cx, |editor, cx| {
                    if let Some((_, excerpt_range)) = task_multi_buffer.read(cx)
                        .excerpts_for_buffer(&buffer_handle, cx)
                        .into_iter()
                        .find(|(excerpt_id, _)| excerpt_id == &query.excerpt_id)
                    {
                        editor.project.as_ref().map(|project| {
                            project.update(cx, |project, cx| {
                                project.query_inlay_hints_for_buffer(
                                    buffer_handle,
                                    excerpt_range.context,
                                    cx,
                                )
                            })
                        })
                    } else {
                        None
                    }
                })
                .context("inlays fetch task spawn")?;
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
                    for inlay in response_hints {
                        match cached_excerpt_hints.binary_search_by(|probe| {
                            inlay.position.cmp(&probe.position, &buffer_snapshot)
                        }) {
                            Ok(ix) | Err(ix) => cached_excerpt_hints.insert(ix, inlay),
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
