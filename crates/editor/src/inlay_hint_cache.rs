use std::cmp;

use crate::{
    display_map::Inlay, editor_settings, Anchor, Editor, ExcerptId, InlayId, MultiBuffer,
    MultiBufferSnapshot,
};
use anyhow::Context;
use clock::Global;
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use gpui::{ModelHandle, Task, ViewContext};
use log::error;
use project::{InlayHint, InlayHintKind};

use collections::{hash_map, HashMap, HashSet};

#[derive(Debug)]
pub struct InlayHintCache {
    inlay_hints: HashMap<InlayId, InlayHint>,
    hints_in_buffers: HashMap<u64, BufferHints<(Anchor, InlayId)>>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
    hint_updates_tx: smol::channel::Sender<HintsUpdate>,
}

#[derive(Debug)]
struct CacheSnapshot {
    inlay_hints: HashMap<InlayId, InlayHint>,
    hints_in_buffers: HashMap<u64, BufferHints<(Anchor, InlayId)>>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
}

#[derive(Clone, Debug)]
struct BufferHints<H> {
    buffer_version: Global,
    hints_per_excerpt: HashMap<ExcerptId, Vec<H>>,
}

#[derive(Debug)]
pub struct InlayHintQuery {
    pub buffer_id: u64,
    pub buffer_version: Global,
    pub excerpt_id: ExcerptId,
}

impl<H> BufferHints<H> {
    fn new(buffer_version: Global) -> Self {
        Self {
            buffer_version,
            hints_per_excerpt: HashMap::default(),
        }
    }
}

impl InlayHintCache {
    pub fn new(
        inlay_hint_settings: editor_settings::InlayHints,
        cx: &mut ViewContext<Editor>,
    ) -> Self {
        let (hint_updates_tx, hint_updates_rx) = smol::channel::unbounded();
        let (update_results_tx, update_results_rx) = smol::channel::unbounded();
        spawn_hints_update_loop(hint_updates_rx, update_results_tx, cx);
        cx.spawn(|editor, mut cx| async move {
            while let Ok(update_result) = update_results_rx.recv().await {
                let editor_absent = editor
                    .update(&mut cx, |editor, cx| {
                        let multi_buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
                        let inlay_hint_cache = &mut editor.inlay_hint_cache;
                        if let Some(new_allowed_hint_kinds) = update_result.new_allowed_hint_kinds {
                            inlay_hint_cache.allowed_hint_kinds = new_allowed_hint_kinds;
                        }

                        inlay_hint_cache.hints_in_buffers.retain(|_, buffer_hints| {
                            buffer_hints.hints_per_excerpt.retain(|_, excerpt_hints| {
                                excerpt_hints.retain(|(_, hint_id)| {
                                    !update_result.remove_from_cache.contains(hint_id)
                                });
                                !excerpt_hints.is_empty()
                            });
                            !buffer_hints.hints_per_excerpt.is_empty()
                        });
                        inlay_hint_cache.inlay_hints.retain(|hint_id, _| {
                            !update_result.remove_from_cache.contains(hint_id)
                        });

                        for (new_buffer_id, new_buffer_inlays) in update_result.add_to_cache {
                            let cached_buffer_hints = inlay_hint_cache
                                .hints_in_buffers
                                .entry(new_buffer_id)
                                .or_insert_with(|| {
                                    BufferHints::new(new_buffer_inlays.buffer_version.clone())
                                });
                            if cached_buffer_hints
                                .buffer_version
                                .changed_since(&new_buffer_inlays.buffer_version)
                            {
                                continue;
                            }
                            for (excerpt_id, new_excerpt_inlays) in
                                new_buffer_inlays.hints_per_excerpt
                            {
                                let cached_excerpt_hints = cached_buffer_hints
                                    .hints_per_excerpt
                                    .entry(excerpt_id)
                                    .or_default();
                                for (new_hint_position, new_hint, new_inlay_id) in
                                    new_excerpt_inlays
                                {
                                    if let hash_map::Entry::Vacant(v) =
                                        inlay_hint_cache.inlay_hints.entry(new_inlay_id)
                                    {
                                        v.insert(new_hint);
                                        match cached_excerpt_hints.binary_search_by(|probe| {
                                            new_hint_position.cmp(&probe.0, &multi_buffer_snapshot)
                                        }) {
                                            Ok(ix) | Err(ix) => cached_excerpt_hints
                                                .insert(ix, (new_hint_position, new_inlay_id)),
                                        }
                                    }
                                }
                            }
                        }

                        let InlaySplice {
                            to_remove,
                            to_insert,
                        } = update_result.splice;
                        editor.splice_inlay_hints(to_remove, to_insert, cx)
                    })
                    .is_err();
                if editor_absent {
                    return;
                }
            }
        })
        .detach();
        Self {
            allowed_hint_kinds: allowed_hint_types(inlay_hint_settings),
            hints_in_buffers: HashMap::default(),
            inlay_hints: HashMap::default(),
            hint_updates_tx,
        }
    }

    pub fn spawn_settings_update(
        &mut self,
        multi_buffer_snapshot: MultiBufferSnapshot,
        inlay_hint_settings: editor_settings::InlayHints,
        current_inlays: Vec<Inlay>,
    ) {
        if !inlay_hint_settings.enabled {
            self.allowed_hint_kinds = allowed_hint_types(inlay_hint_settings);
            if self.inlay_hints.is_empty() {
                return;
            } else {
                self.hint_updates_tx
                    .send_blocking(HintsUpdate {
                        multi_buffer_snapshot,
                        cache: self.snapshot(),
                        visible_inlays: current_inlays,
                        kind: HintsUpdateKind::Clean,
                    })
                    .ok();
                return;
            }
        }

        let new_allowed_hint_kinds = allowed_hint_types(inlay_hint_settings);
        if new_allowed_hint_kinds == self.allowed_hint_kinds {
            return;
        }

        self.hint_updates_tx
            .send_blocking(HintsUpdate {
                multi_buffer_snapshot,
                cache: self.snapshot(),
                visible_inlays: current_inlays,
                kind: HintsUpdateKind::AllowedHintKindsChanged {
                    new: new_allowed_hint_kinds,
                },
            })
            .ok();
    }

    pub fn spawn_hints_update(
        &mut self,
        multi_buffer_snapshot: MultiBufferSnapshot,
        queries: Vec<InlayHintQuery>,
        current_inlays: Vec<Inlay>,
        conflicts_invalidate_cache: bool,
    ) {
        let conflicts_with_cache = conflicts_invalidate_cache
            && queries.iter().any(|update_query| {
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

        let queries_per_buffer = queries
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
            .fold(
                HashMap::<u64, (Global, Vec<ExcerptId>)>::default(),
                |mut queries_per_buffer, new_query| {
                    let (current_verison, excerpts_to_query) =
                        queries_per_buffer.entry(new_query.buffer_id).or_default();

                    if new_query.buffer_version.changed_since(current_verison) {
                        *current_verison = new_query.buffer_version;
                        *excerpts_to_query = vec![new_query.excerpt_id];
                    } else if !current_verison.changed_since(&new_query.buffer_version) {
                        excerpts_to_query.push(new_query.excerpt_id);
                    }

                    queries_per_buffer
                },
            );

        for (queried_buffer, (buffer_version, excerpts)) in queries_per_buffer {
            self.hint_updates_tx
                .send_blocking(HintsUpdate {
                    multi_buffer_snapshot: multi_buffer_snapshot.clone(),
                    visible_inlays: current_inlays.clone(),
                    cache: self.snapshot(),
                    kind: HintsUpdateKind::BufferUpdate {
                        invalidate_cache: conflicts_with_cache,
                        buffer_id: queried_buffer,
                        buffer_version,
                        excerpts,
                    },
                })
                .ok();
        }
    }

    // TODO kb could be big and cloned per symbol input.
    // Instead, use `Box`/`Arc`/`Rc`?
    fn snapshot(&self) -> CacheSnapshot {
        CacheSnapshot {
            inlay_hints: self.inlay_hints.clone(),
            hints_in_buffers: self.hints_in_buffers.clone(),
            allowed_hint_kinds: self.allowed_hint_kinds.clone(),
        }
    }
}

#[derive(Debug, Default)]
struct InlaySplice {
    to_remove: Vec<InlayId>,
    to_insert: Vec<(InlayId, Anchor, InlayHint)>,
}

struct HintsUpdate {
    multi_buffer_snapshot: MultiBufferSnapshot,
    visible_inlays: Vec<Inlay>,
    cache: CacheSnapshot,
    kind: HintsUpdateKind,
}

enum HintsUpdateKind {
    Clean,
    AllowedHintKindsChanged {
        new: HashSet<Option<InlayHintKind>>,
    },
    BufferUpdate {
        buffer_id: u64,
        buffer_version: Global,
        excerpts: Vec<ExcerptId>,
        invalidate_cache: bool,
    },
}

struct UpdateResult {
    splice: InlaySplice,
    new_allowed_hint_kinds: Option<HashSet<Option<InlayHintKind>>>,
    remove_from_cache: HashSet<InlayId>,
    add_to_cache: HashMap<u64, BufferHints<(Anchor, InlayHint, InlayId)>>,
}

impl HintsUpdate {
    fn merge(&mut self, mut other: Self) -> Result<(), Self> {
        match (&mut self.kind, &mut other.kind) {
            (HintsUpdateKind::Clean, HintsUpdateKind::Clean) => return Ok(()),
            (
                HintsUpdateKind::AllowedHintKindsChanged { .. },
                HintsUpdateKind::AllowedHintKindsChanged { .. },
            ) => {
                *self = other;
                return Ok(());
            }
            (
                HintsUpdateKind::BufferUpdate {
                    buffer_id: old_buffer_id,
                    buffer_version: old_buffer_version,
                    excerpts: old_excerpts,
                    ..
                },
                HintsUpdateKind::BufferUpdate {
                    buffer_id: new_buffer_id,
                    buffer_version: new_buffer_version,
                    excerpts: new_excerpts,
                    invalidate_cache: new_invalidate_cache,
                },
            ) => {
                if old_buffer_id == new_buffer_id {
                    if new_buffer_version.changed_since(old_buffer_version) {
                        *self = other;
                        return Ok(());
                    } else if old_buffer_version.changed_since(new_buffer_version) {
                        return Ok(());
                    } else if *new_invalidate_cache {
                        *self = other;
                        return Ok(());
                    } else {
                        let old_inlays = self
                            .visible_inlays
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect::<Vec<_>>();
                        let new_inlays = other
                            .visible_inlays
                            .iter()
                            .map(|inlay| inlay.id)
                            .collect::<Vec<_>>();
                        if old_inlays == new_inlays {
                            old_excerpts.extend(new_excerpts.drain(..));
                            old_excerpts.dedup();
                            return Ok(());
                        }
                    }
                }
            }
            _ => {}
        }

        Err(other)
    }

    async fn run(self, result_sender: smol::channel::Sender<UpdateResult>) {
        match self.kind {
            HintsUpdateKind::Clean => {
                if !self.cache.inlay_hints.is_empty() || !self.visible_inlays.is_empty() {
                    result_sender
                        .send(UpdateResult {
                            splice: InlaySplice {
                                to_remove: self
                                    .visible_inlays
                                    .iter()
                                    .map(|inlay| inlay.id)
                                    .collect(),
                                to_insert: Vec::new(),
                            },
                            new_allowed_hint_kinds: None,
                            remove_from_cache: self.cache.inlay_hints.keys().copied().collect(),
                            add_to_cache: HashMap::default(),
                        })
                        .await
                        .ok();
                }
            }
            HintsUpdateKind::AllowedHintKindsChanged { new } => {
                if let Some(splice) = new_allowed_hint_kinds_splice(
                    &self.multi_buffer_snapshot,
                    self.visible_inlays,
                    &self.cache,
                    &new,
                ) {
                    result_sender
                        .send(UpdateResult {
                            splice,
                            new_allowed_hint_kinds: Some(new),
                            remove_from_cache: HashSet::default(),
                            add_to_cache: HashMap::default(),
                        })
                        .await
                        .ok();
                }
            }
            HintsUpdateKind::BufferUpdate {
                buffer_id,
                buffer_version,
                excerpts,
                invalidate_cache,
            } => {
                let mut tasks = excerpts
                    .into_iter()
                    .map(|excerpt_id| async move {
                        //
                        todo!("TODO kb")
                    })
                    .collect::<FuturesUnordered<_>>();
                while let Some(update) = tasks.next().await {
                    todo!("TODO kb")
                }
            }
        }
    }
}

fn spawn_hints_update_loop(
    hint_updates_rx: smol::channel::Receiver<HintsUpdate>,
    update_results_tx: smol::channel::Sender<UpdateResult>,
    cx: &mut ViewContext<'_, '_, Editor>,
) {
    cx.background()
        .spawn(async move {
            let mut update = None::<HintsUpdate>;
            let mut next_update = None::<HintsUpdate>;
            loop {
                if update.is_none() {
                    match hint_updates_rx.recv().await {
                        Ok(first_task) => update = Some(first_task),
                        Err(smol::channel::RecvError) => return,
                    }
                }

                let mut updates_limit = 10;
                'update_merge: loop {
                    match hint_updates_rx.try_recv() {
                        Ok(new_update) => {
                            match update.as_mut() {
                                Some(update) => match update.merge(new_update) {
                                    Ok(()) => {}
                                    Err(new_update) => {
                                        next_update = Some(new_update);
                                        break 'update_merge;
                                    }
                                },
                                None => update = Some(new_update),
                            };

                            if updates_limit == 0 {
                                break 'update_merge;
                            }
                            updates_limit -= 1;
                        }
                        Err(smol::channel::TryRecvError::Empty) => break 'update_merge,
                        Err(smol::channel::TryRecvError::Closed) => return,
                    }
                }

                if let Some(update) = update.take() {
                    let (run_tx, run_rx) = smol::channel::unbounded();
                    let mut update_handle = std::pin::pin!(update.run(run_tx).fuse());
                    loop {
                        futures::select_biased! {
                            update_result = run_rx.recv().fuse() => {
                                match update_result {
                                    Ok(update_result) => {
                                        if let Err(_) = update_results_tx.send(update_result).await {
                                            return
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                            _ = &mut update_handle => {
                                while let Ok(update_result) = run_rx.try_recv() {
                                    if let Err(_) = update_results_tx.send(update_result).await {
                                        return
                                    }
                                }
                                break
                            },
                        }
                    }
                }
                update = next_update.take();
            }
        })
        .detach()
}

fn new_allowed_hint_kinds_splice(
    multi_buffer_snapshot: &MultiBufferSnapshot,
    current_inlays: Vec<Inlay>,
    hints_cache: &CacheSnapshot,
    new_kinds: &HashSet<Option<InlayHintKind>>,
) -> Option<InlaySplice> {
    let old_kinds = &hints_cache.allowed_hint_kinds;
    if old_kinds == new_kinds {
        return None;
    }

    let mut to_remove = Vec::new();
    let mut to_insert = Vec::new();
    let mut shown_hints_to_remove = group_inlays(current_inlays);

    for (buffer_id, cached_buffer_hints) in &hints_cache.hints_in_buffers {
        let shown_buffer_hints_to_remove = shown_hints_to_remove.entry(*buffer_id).or_default();
        for (excerpt_id, cached_excerpt_hints) in &cached_buffer_hints.hints_per_excerpt {
            let shown_excerpt_hints_to_remove =
                shown_buffer_hints_to_remove.entry(*excerpt_id).or_default();
            let mut cached_hints = cached_excerpt_hints.iter().fuse().peekable();
            shown_excerpt_hints_to_remove.retain(|(shown_anchor, shown_hint_id)| {
                loop {
                    match cached_hints.peek() {
                        Some((cached_anchor, cached_hint_id)) => {
                            if cached_hint_id == shown_hint_id {
                                return !new_kinds.contains(
                                    &hints_cache.inlay_hints.get(&cached_hint_id).unwrap().kind,
                                );
                            }

                            match cached_anchor.cmp(shown_anchor, &multi_buffer_snapshot) {
                                cmp::Ordering::Less | cmp::Ordering::Equal => {
                                    let maybe_missed_cached_hint =
                                        hints_cache.inlay_hints.get(&cached_hint_id).unwrap();
                                    let cached_hint_kind = maybe_missed_cached_hint.kind;
                                    if !old_kinds.contains(&cached_hint_kind)
                                        && new_kinds.contains(&cached_hint_kind)
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

                match hints_cache.inlay_hints.get(&shown_hint_id) {
                    Some(shown_hint) => !new_kinds.contains(&shown_hint.kind),
                    None => true,
                }
            });

            for (cached_anchor, cached_hint_id) in cached_hints {
                let maybe_missed_cached_hint =
                    hints_cache.inlay_hints.get(&cached_hint_id).unwrap();
                let cached_hint_kind = maybe_missed_cached_hint.kind;
                if !old_kinds.contains(&cached_hint_kind) && new_kinds.contains(&cached_hint_kind) {
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
    Some(InlaySplice {
        to_remove,
        to_insert,
    })
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

// TODO kb wrong, query and update the editor separately
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

fn group_inlays(inlays: Vec<Inlay>) -> HashMap<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>> {
    inlays.into_iter().fold(
        HashMap::<u64, HashMap<ExcerptId, Vec<(Anchor, InlayId)>>>::default(),
        |mut current_hints, inlay| {
            if let Some(buffer_id) = inlay.position.buffer_id {
                current_hints
                    .entry(buffer_id)
                    .or_default()
                    .entry(inlay.position.excerpt_id)
                    .or_default()
                    .push((inlay.position, inlay.id));
            }
            current_hints
        },
    )
}

// async fn update_hints(
//     multi_buffer: ModelHandle<MultiBuffer>,
//     queries: Vec<InlayHintQuery>,
//     current_inlays: Vec<Inlay>,
//     invalidate_cache: bool,
//     cx: &mut ViewContext<'_, '_, Editor>,
// ) -> Option<InlaySplice> {
//     let fetch_queries_task = fetch_queries(multi_buffer, queries.into_iter(), cx);
//     let new_hints = fetch_queries_task.await.context("inlay hints fetch")?;

//     let mut to_remove = Vec::new();
//     let mut to_insert = Vec::new();
//     let mut cache_hints_to_persist: HashMap<u64, (Global, HashMap<ExcerptId, HashSet<InlayId>>)> =
//         HashMap::default();

//     editor.update(&mut cx, |editor, cx| {
//         let multi_buffer_snapshot = task_multi_buffer.read(cx).snapshot(cx);
//         for (new_buffer_id, new_hints_per_buffer) in new_hints {
//             let cached_buffer_hints = editor
//                 .inlay_hint_cache
//                 .hints_in_buffers
//                 .entry(new_buffer_id)
//                 .or_insert_with(|| {
//                     BufferHints::new(new_hints_per_buffer.buffer_version.clone())
//                 });

//             let buffer_cache_hints_to_persist =
//                 cache_hints_to_persist.entry(new_buffer_id).or_insert_with(|| (new_hints_per_buffer.buffer_version.clone(), HashMap::default()));
//             if cached_buffer_hints
//                 .buffer_version
//                 .changed_since(&new_hints_per_buffer.buffer_version)
//             {
//                 buffer_cache_hints_to_persist.0 = new_hints_per_buffer.buffer_version;
//                 buffer_cache_hints_to_persist.1.extend(
//                     cached_buffer_hints.hints_per_excerpt.iter().map(
//                         |(excerpt_id, excerpt_hints)| {
//                             (
//                                 *excerpt_id,
//                                 excerpt_hints.iter().map(|(_, id)| *id).collect(),
//                             )
//                         },
//                     ),
//                 );
//                 continue;
//             }

//             let shown_buffer_hints = currently_shown_hints.get(&new_buffer_id);
//             for (new_excerpt_id, new_hints_per_excerpt) in
//                 new_hints_per_buffer.hints_per_excerpt
//             {
//                 let excerpt_cache_hints_to_persist = buffer_cache_hints_to_persist.1
//                     .entry(new_excerpt_id)
//                     .or_default();
//                 let cached_excerpt_hints = cached_buffer_hints
//                     .hints_per_excerpt
//                     .entry(new_excerpt_id)
//                     .or_default();
//                 let empty_shown_excerpt_hints = Vec::new();
//                 let shown_excerpt_hints = shown_buffer_hints.and_then(|hints| hints.get(&new_excerpt_id)).unwrap_or(&empty_shown_excerpt_hints);
//                 for new_hint in new_hints_per_excerpt {
//                     let new_hint_anchor = multi_buffer_snapshot
//                         .anchor_in_excerpt(new_excerpt_id, new_hint.position);
//                     let cache_insert_ix = match cached_excerpt_hints.binary_search_by(|probe| {
//                         new_hint_anchor.cmp(&probe.0, &multi_buffer_snapshot)
//                     }) {
//                         Ok(ix) => {
//                             let (_, cached_inlay_id) = cached_excerpt_hints[ix];
//                             let cache_hit = editor
//                                 .inlay_hint_cache
//                                 .inlay_hints
//                                 .get(&cached_inlay_id)
//                                 .filter(|cached_hint| cached_hint == &&new_hint)
//                                 .is_some();
//                             if cache_hit {
//                                 excerpt_cache_hints_to_persist
//                                     .insert(cached_inlay_id);
//                                 None
//                             } else {
//                                 Some(ix)
//                             }
//                         }
//                         Err(ix) => Some(ix),
//                     };

//                     let shown_inlay_id = match shown_excerpt_hints.binary_search_by(|probe| {
//                         probe.0.cmp(&new_hint_anchor, &multi_buffer_snapshot)
//                     }) {
//                         Ok(ix) => {{
//                             let (_, shown_inlay_id) = shown_excerpt_hints[ix];
//                             let shown_hint_found =  editor.inlay_hint_cache.inlay_hints.get(&shown_inlay_id)
//                                 .filter(|cached_hint| cached_hint == &&new_hint).is_some();
//                             if shown_hint_found {
//                                 Some(shown_inlay_id)
//                             } else {
//                                 None
//                             }
//                         }},
//                         Err(_) => None,
//                     };

//                     if let Some(insert_ix) = cache_insert_ix {
//                         let hint_id = match shown_inlay_id {
//                             Some(shown_inlay_id) => shown_inlay_id,
//                             None => {
//                                 let new_hint_id = InlayId(post_inc(&mut editor.next_inlay_id));
//                                 if editor.inlay_hint_cache.allowed_hint_kinds.contains(&new_hint.kind)
//                                 {
//                                     to_insert.push((new_hint_id, new_hint_anchor, new_hint.clone()));
//                                 }
//                                 new_hint_id
//                             }
//                         };
//                         excerpt_cache_hints_to_persist.insert(hint_id);
//                         cached_excerpt_hints.insert(insert_ix, (new_hint_anchor, hint_id));
//                         editor
//                             .inlay_hint_cache
//                             .inlay_hints
//                             .insert(hint_id, new_hint);
//                     }
//                 }
//             }
//         }

//         if conflicts_with_cache {
//             for (shown_buffer_id, mut shown_hints_to_clean) in currently_shown_hints {
//                 match cache_hints_to_persist.get(&shown_buffer_id) {
//                     Some(cached_buffer_hints) => {
//                         for (persisted_id, cached_hints) in &cached_buffer_hints.1 {
//                             shown_hints_to_clean.entry(*persisted_id).or_default()
//                                 .retain(|(_, shown_id)| !cached_hints.contains(shown_id));
//                         }
//                     },
//                     None => {},
//                 }
//                 to_remove.extend(shown_hints_to_clean.into_iter()
//                     .flat_map(|(_, excerpt_hints)| excerpt_hints.into_iter().map(|(_, hint_id)| hint_id)));
//             }

//             editor.inlay_hint_cache.hints_in_buffers.retain(|buffer_id, buffer_hints| {
//                 let Some(mut buffer_hints_to_persist) = cache_hints_to_persist.remove(buffer_id) else { return false; };
//                 buffer_hints.buffer_version = buffer_hints_to_persist.0;
//                 buffer_hints.hints_per_excerpt.retain(|excerpt_id, excerpt_hints| {
//                     let Some(excerpt_hints_to_persist) = buffer_hints_to_persist.1.remove(&excerpt_id) else { return false; };
//                     excerpt_hints.retain(|(_, hint_id)| {
//                         let retain = excerpt_hints_to_persist.contains(hint_id);
//                         if !retain {
//                             editor
//                                 .inlay_hint_cache
//                                 .inlay_hints
//                                 .remove(hint_id);
//                         }
//                         retain
//                     });
//                     !excerpt_hints.is_empty()
//                 });
//                 !buffer_hints.hints_per_excerpt.is_empty()
//             });
//         }

//         Some(InlaySplice {
//             to_remove,
//             to_insert,
//         })
//     })
// }
