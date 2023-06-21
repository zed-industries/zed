use std::cmp;

use crate::{
    display_map::Inlay, editor_settings, Anchor, Editor, ExcerptId, InlayId, MultiBufferSnapshot,
};
use anyhow::Context;
use gpui::{Task, ViewContext};
use log::error;
use project::{InlayHint, InlayHintKind};

use collections::{hash_map, HashMap, HashSet};
use util::post_inc;

pub struct InlayHintCache {
    snapshot: Box<CacheSnapshot>,
    update_tasks: HashMap<ExcerptId, InlayHintUpdateTask>,
}

struct InlayHintUpdateTask {
    version: usize,
    _task: Task<()>,
}

#[derive(Debug, Clone)]
struct CacheSnapshot {
    hints: HashMap<ExcerptId, ExcerptCachedHints>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
    version: usize,
}

#[derive(Debug, Clone)]
struct ExcerptCachedHints {
    version: usize,
    hints: Vec<(Anchor, InlayId, InlayHint)>,
}

#[derive(Clone)]
pub struct HintsUpdateState {
    multi_buffer_snapshot: MultiBufferSnapshot,
    visible_inlays: Vec<Inlay>,
    cache: Box<CacheSnapshot>,
}

#[derive(Debug, Default)]
pub struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<(Anchor, InlayId, InlayHint)>,
}

#[derive(Debug)]
struct ExcerptHintsUpdate {
    excerpt_id: ExcerptId,
    cache_version: usize,
    remove_from_visible: Vec<InlayId>,
    remove_from_cache: HashSet<InlayId>,
    add_to_cache: Vec<(Anchor, InlayHint)>,
}

impl InlayHintCache {
    pub fn new(inlay_hint_settings: editor_settings::InlayHints) -> Self {
        Self {
            snapshot: Box::new(CacheSnapshot {
                allowed_hint_kinds: allowed_hint_types(inlay_hint_settings),
                hints: HashMap::default(),
                version: 0,
            }),
            update_tasks: HashMap::default(),
        }
    }

    pub fn update_settings(
        &mut self,
        inlay_hint_settings: editor_settings::InlayHints,
        update_state: HintsUpdateState,
    ) -> Option<InlaySplice> {
        let new_allowed_hint_kinds = allowed_hint_types(inlay_hint_settings);
        if !inlay_hint_settings.enabled {
            if self.snapshot.hints.is_empty() {
                self.snapshot.allowed_hint_kinds = new_allowed_hint_kinds;
            } else {
                self.clear();
                self.snapshot.allowed_hint_kinds = new_allowed_hint_kinds;
                return Some(InlaySplice {
                    to_remove: update_state
                        .visible_inlays
                        .iter()
                        .map(|inlay| inlay.id)
                        .collect(),
                    to_insert: Vec::new(),
                });
            }

            return None;
        }

        if new_allowed_hint_kinds == self.snapshot.allowed_hint_kinds {
            return None;
        }

        let new_splice = new_allowed_hint_kinds_splice(update_state, &new_allowed_hint_kinds);
        if new_splice.is_some() {
            self.snapshot.version += 1;
            self.update_tasks.clear();
            self.snapshot.allowed_hint_kinds = new_allowed_hint_kinds;
        }
        new_splice
    }

    pub fn spawn_hints_update(&self, invalidate_cache: bool, cx: &mut ViewContext<Editor>) {
        cx.spawn(|editor, mut cx| async move {
            editor
                .update(&mut cx, |editor, cx| {
                    let mut excerpts_to_query = editor
                        .excerpt_visible_offsets(cx)
                        .into_iter()
                        .map(|(buffer, _, excerpt_id)| (excerpt_id, buffer.read(cx).remote_id()))
                        .collect::<HashMap<_, _>>();

                    let update_state = get_update_state(editor, cx);
                    let update_tasks = &mut editor.inlay_hint_cache.update_tasks;
                    if invalidate_cache {
                        update_tasks.retain(|task_excerpt_id, _| {
                            excerpts_to_query.contains_key(task_excerpt_id)
                        });
                    }

                    let cache_version = editor.inlay_hint_cache.snapshot.version;
                    excerpts_to_query.retain(|visible_excerpt_id, _| {
                        match update_tasks.entry(*visible_excerpt_id) {
                            hash_map::Entry::Occupied(o) => {
                                match o.get().version.cmp(&cache_version) {
                                    cmp::Ordering::Less => true,
                                    cmp::Ordering::Equal => invalidate_cache,
                                    cmp::Ordering::Greater => false,
                                }
                            }
                            hash_map::Entry::Vacant(_) => true,
                        }
                    });

                    for (excerpt_id, buffer_id) in excerpts_to_query {
                        update_tasks.insert(
                            excerpt_id,
                            new_update_task(
                                buffer_id,
                                excerpt_id,
                                cache_version,
                                update_state.clone(),
                                invalidate_cache,
                                cx,
                            ),
                        );
                    }
                })
                .ok();
        })
        .detach();
    }

    fn snapshot(&self) -> Box<CacheSnapshot> {
        self.snapshot.clone()
    }

    fn clear(&mut self) {
        self.snapshot.version += 1;
        self.update_tasks.clear();
        self.snapshot.hints.clear();
        self.snapshot.allowed_hint_kinds.clear();
    }
}

fn new_update_task(
    buffer_id: u64,
    excerpt_id: ExcerptId,
    cache_version: usize,
    state: HintsUpdateState,
    invalidate_cache: bool,
    cx: &mut ViewContext<'_, '_, Editor>,
) -> InlayHintUpdateTask {
    let hints_fetch_task = hints_fetch_task(buffer_id, excerpt_id, cx);
    let task_multi_buffer_snapshot = state.multi_buffer_snapshot.clone();

    InlayHintUpdateTask {
        version: cache_version,
        _task: cx.spawn(|editor, mut cx| async move {
            match hints_fetch_task.await {
                Ok(Some(new_hints)) => {
                    if let Some(new_update) = cx
                        .background()
                        .spawn(async move {
                            new_excerpt_hints_update_result(
                                state,
                                excerpt_id,
                                new_hints,
                                invalidate_cache,
                            )
                        })
                        .await
                    {
                        editor
                            .update(&mut cx, |editor, cx| {
                                let cached_excerpt_hints = editor
                                    .inlay_hint_cache
                                    .snapshot
                                    .hints
                                    .entry(new_update.excerpt_id)
                                    .or_insert_with(|| ExcerptCachedHints {
                                        version: new_update.cache_version,
                                        hints: Vec::new(),
                                    });
                                match new_update.cache_version.cmp(&cached_excerpt_hints.version) {
                                    cmp::Ordering::Less => return,
                                    cmp::Ordering::Greater | cmp::Ordering::Equal => {
                                        cached_excerpt_hints.version = new_update.cache_version;
                                    }
                                }

                                editor.inlay_hint_cache.snapshot.version += 1;
                                let mut splice = InlaySplice {
                                    to_remove: new_update.remove_from_visible,
                                    to_insert: Vec::new(),
                                };

                                for (new_hint_position, new_hint) in new_update.add_to_cache {
                                    let new_inlay_id = InlayId(post_inc(&mut editor.next_inlay_id));
                                    if editor
                                        .inlay_hint_cache
                                        .snapshot
                                        .allowed_hint_kinds
                                        .contains(&new_hint.kind)
                                    {
                                        splice.to_insert.push((
                                            new_hint_position,
                                            new_inlay_id,
                                            new_hint.clone(),
                                        ));
                                    }

                                    match cached_excerpt_hints.hints.binary_search_by(|probe| {
                                        probe.0.cmp(&new_hint_position, &task_multi_buffer_snapshot)
                                    }) {
                                        Ok(ix) | Err(ix) => cached_excerpt_hints.hints.insert(
                                            ix,
                                            (new_hint_position, new_inlay_id, new_hint),
                                        ),
                                    }
                                }
                                editor.inlay_hint_cache.snapshot.hints.retain(
                                    |_, excerpt_hints| {
                                        excerpt_hints.hints.retain(|(_, hint_id, _)| {
                                            !new_update.remove_from_cache.contains(hint_id)
                                        });
                                        !excerpt_hints.hints.is_empty()
                                    },
                                );

                                let InlaySplice {
                                    to_remove,
                                    to_insert,
                                } = splice;
                                if !to_remove.is_empty() || !to_insert.is_empty() {
                                    editor.splice_inlay_hints(to_remove, to_insert, cx)
                                }
                            })
                            .ok();
                    }
                }
                Ok(None) => {}
                Err(e) => error!(
                    "Failed to fecth hints for excerpt {excerpt_id:?} in buffer {buffer_id} : {e}"
                ),
            }
        }),
    }
}

pub fn get_update_state(editor: &Editor, cx: &ViewContext<'_, '_, Editor>) -> HintsUpdateState {
    HintsUpdateState {
        visible_inlays: visible_inlay_hints(editor, cx).cloned().collect(),
        cache: editor.inlay_hint_cache.snapshot(),
        multi_buffer_snapshot: editor.buffer().read(cx).snapshot(cx),
    }
}

fn new_allowed_hint_kinds_splice(
    state: HintsUpdateState,
    new_kinds: &HashSet<Option<InlayHintKind>>,
) -> Option<InlaySplice> {
    let old_kinds = &state.cache.allowed_hint_kinds;
    if new_kinds == old_kinds {
        return None;
    }

    let mut to_remove = Vec::new();
    let mut to_insert = Vec::new();
    let mut shown_hints_to_remove = state.visible_inlays.iter().fold(
        HashMap::<ExcerptId, Vec<(Anchor, InlayId)>>::default(),
        |mut current_hints, inlay| {
            current_hints
                .entry(inlay.position.excerpt_id)
                .or_default()
                .push((inlay.position, inlay.id));
            current_hints
        },
    );

    for (excerpt_id, excerpt_cached_hints) in &state.cache.hints {
        let shown_excerpt_hints_to_remove = shown_hints_to_remove.entry(*excerpt_id).or_default();
        let mut excerpt_cached_hints = excerpt_cached_hints.hints.iter().fuse().peekable();
        shown_excerpt_hints_to_remove.retain(|(shown_anchor, shown_hint_id)| loop {
            match excerpt_cached_hints.peek() {
                Some((cached_anchor, cached_hint_id, cached_hint)) => {
                    if cached_hint_id == shown_hint_id {
                        excerpt_cached_hints.next();
                        return !new_kinds.contains(&cached_hint.kind);
                    }

                    match cached_anchor.cmp(shown_anchor, &state.multi_buffer_snapshot) {
                        cmp::Ordering::Less | cmp::Ordering::Equal => {
                            if !old_kinds.contains(&cached_hint.kind)
                                && new_kinds.contains(&cached_hint.kind)
                            {
                                to_insert.push((
                                    *cached_anchor,
                                    *cached_hint_id,
                                    cached_hint.clone(),
                                ));
                            }
                            excerpt_cached_hints.next();
                        }
                        cmp::Ordering::Greater => return true,
                    }
                }
                None => return true,
            }
        });

        for (cached_anchor, cached_hint_id, maybe_missed_cached_hint) in excerpt_cached_hints {
            let cached_hint_kind = maybe_missed_cached_hint.kind;
            if !old_kinds.contains(&cached_hint_kind) && new_kinds.contains(&cached_hint_kind) {
                to_insert.push((
                    *cached_anchor,
                    *cached_hint_id,
                    maybe_missed_cached_hint.clone(),
                ));
            }
        }
    }

    to_remove.extend(
        shown_hints_to_remove
            .into_values()
            .flatten()
            .map(|(_, hint_id)| hint_id),
    );
    if to_remove.is_empty() && to_insert.is_empty() {
        None
    } else {
        Some(InlaySplice {
            to_remove,
            to_insert,
        })
    }
}

fn new_excerpt_hints_update_result(
    state: HintsUpdateState,
    excerpt_id: ExcerptId,
    new_excerpt_hints: Vec<InlayHint>,
    invalidate_cache: bool,
) -> Option<ExcerptHintsUpdate> {
    let mut add_to_cache: Vec<(Anchor, InlayHint)> = Vec::new();
    let shown_excerpt_hints = state
        .visible_inlays
        .iter()
        .filter(|hint| hint.position.excerpt_id == excerpt_id)
        .collect::<Vec<_>>();
    let empty = Vec::new();
    let cached_excerpt_hints = state
        .cache
        .hints
        .get(&excerpt_id)
        .map(|buffer_excerpts| &buffer_excerpts.hints)
        .unwrap_or(&empty);

    let mut excerpt_hints_to_persist = HashSet::default();
    for new_hint in new_excerpt_hints {
        // TODO kb this somehow spoils anchors and make them equal for different text anchors.
        let new_hint_anchor = state
            .multi_buffer_snapshot
            .anchor_in_excerpt(excerpt_id, new_hint.position);
        // TODO kb use merge sort or something else better
        let should_add_to_cache = match cached_excerpt_hints
            .binary_search_by(|probe| probe.0.cmp(&new_hint_anchor, &state.multi_buffer_snapshot))
        {
            Ok(ix) => {
                let (_, cached_inlay_id, cached_hint) = &cached_excerpt_hints[ix];
                if cached_hint == &new_hint {
                    excerpt_hints_to_persist.insert(*cached_inlay_id);
                    false
                } else {
                    true
                }
            }
            Err(_) => true,
        };

        let shown_inlay_id = match shown_excerpt_hints.binary_search_by(|probe| {
            probe
                .position
                .cmp(&new_hint_anchor, &state.multi_buffer_snapshot)
        }) {
            Ok(ix) => {
                let shown_hint = &shown_excerpt_hints[ix];
                state
                    .cache
                    .hints
                    .get(&excerpt_id)
                    .and_then(|excerpt_hints| {
                        excerpt_hints
                            .hints
                            .iter()
                            .find_map(|(_, cached_id, cached_hint)| {
                                if cached_id == &shown_hint.id && cached_hint == &new_hint {
                                    Some(cached_id)
                                } else {
                                    None
                                }
                            })
                    })
            }
            Err(_) => None,
        };

        if should_add_to_cache {
            if shown_inlay_id.is_none() {
                add_to_cache.push((new_hint_anchor, new_hint.clone()));
            }
        }
    }

    let mut remove_from_visible = Vec::new();
    let mut remove_from_cache = HashSet::default();
    if invalidate_cache {
        remove_from_visible.extend(
            shown_excerpt_hints
                .iter()
                .map(|inlay_hint| inlay_hint.id)
                .filter(|hint_id| !excerpt_hints_to_persist.contains(hint_id)),
        );
        remove_from_cache.extend(
            state
                .cache
                .hints
                .values()
                .flat_map(|excerpt_hints| excerpt_hints.hints.iter().map(|(_, id, _)| id))
                .filter(|cached_inlay_id| !excerpt_hints_to_persist.contains(cached_inlay_id)),
        );
    }

    if remove_from_visible.is_empty() && remove_from_cache.is_empty() && add_to_cache.is_empty() {
        None
    } else {
        Some(ExcerptHintsUpdate {
            cache_version: state.cache.version,
            excerpt_id,
            remove_from_visible,
            remove_from_cache,
            add_to_cache,
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

fn hints_fetch_task(
    buffer_id: u64,
    excerpt_id: ExcerptId,
    cx: &mut ViewContext<'_, '_, Editor>,
) -> Task<anyhow::Result<Option<Vec<InlayHint>>>> {
    cx.spawn(|editor, mut cx| async move {
        let Ok(task) = editor
            .update(&mut cx, |editor, cx| {
                Some({
                    let multi_buffer = editor.buffer().read(cx);
                    let buffer_handle = multi_buffer.buffer(buffer_id)?;
                    let (_, excerpt_range) = multi_buffer
                        .excerpts_for_buffer(&buffer_handle, cx)
                        .into_iter()
                        .find(|(id, _)| id == &excerpt_id)?;
                    editor.project.as_ref()?.update(cx, |project, cx| {
                        project.inlay_hints(
                            buffer_handle,
                            excerpt_range.context,
                            cx,
                        )
                    })
                })
            }) else {
                return Ok(None);
            };
        Ok(match task {
            Some(task) => task.await.context("inlays for buffer task")?,
            None => Some(Vec::new()),
        })
    })
}

fn visible_inlay_hints<'a, 'b: 'a, 'c, 'd: 'a>(
    editor: &'a Editor,
    cx: &'b ViewContext<'c, 'd, Editor>,
) -> impl Iterator<Item = &'b Inlay> + 'a {
    editor
        .display_map
        .read(cx)
        .current_inlays()
        .filter(|inlay| Some(inlay.id) != editor.copilot_state.suggestion.as_ref().map(|h| h.id))
}
