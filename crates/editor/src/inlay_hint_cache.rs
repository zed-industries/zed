/// Stores and updates all data received from LSP <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_inlayHint">textDocument/inlayHint</a> requests.
/// Has nothing to do with other inlays, e.g. copilot suggestions — those are stored elsewhere.
/// On every update, cache may query for more inlay hints and update inlays on the screen.
///
/// Inlays stored on screen are in [`crate::display_map::inlay_map`] and this cache is the only way to update any inlay hint data in the visible hints in the inlay map.
/// For determining the update to the `inlay_map`, the cache requires a list of visible inlay hints — all other hints are not relevant and their separate updates are not influencing the cache work.
///
/// Due to the way the data is stored for both visible inlays and the cache, every inlay (and inlay hint) collection is editor-specific, so a single buffer may have multiple sets of inlays of open on different panes.
use std::{
    cmp,
    ops::{ControlFlow, Range},
    sync::Arc,
    time::Duration,
};

use crate::{
    display_map::Inlay, Anchor, Editor, ExcerptId, InlayId, MultiBuffer, MultiBufferSnapshot,
};
use anyhow::Context;
use clock::Global;
use futures::future;
use gpui::{AsyncWindowContext, Model, ModelContext, Task, ViewContext};
use language::{language_settings::InlayHintKind, Buffer, BufferSnapshot};
use parking_lot::RwLock;
use project::{InlayHint, ResolveState};

use collections::{hash_map, HashMap, HashSet};
use language::language_settings::InlayHintSettings;
use smol::lock::Semaphore;
use sum_tree::Bias;
use text::{BufferId, ToOffset, ToPoint};
use util::{post_inc, ResultExt};

pub struct InlayHintCache {
    hints: HashMap<ExcerptId, Arc<RwLock<CachedExcerptHints>>>,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
    version: usize,
    pub(super) enabled: bool,
    update_tasks: HashMap<ExcerptId, TasksForRanges>,
    refresh_task: Option<Task<()>>,
    invalidate_debounce: Option<Duration>,
    append_debounce: Option<Duration>,
    lsp_request_limiter: Arc<Semaphore>,
}

#[derive(Debug)]
struct TasksForRanges {
    tasks: Vec<Task<()>>,
    sorted_ranges: Vec<Range<language::Anchor>>,
}

#[derive(Debug)]
struct CachedExcerptHints {
    version: usize,
    buffer_version: Global,
    buffer_id: BufferId,
    ordered_hints: Vec<InlayId>,
    hints_by_id: HashMap<InlayId, InlayHint>,
}

/// A logic to apply when querying for new inlay hints and deciding what to do with the old entries in the cache in case of conflicts.
#[derive(Debug, Clone, Copy)]
pub(super) enum InvalidationStrategy {
    /// Hints reset is <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_inlayHint_refresh">requested</a> by the LSP server.
    /// Demands to re-query all inlay hints needed and invalidate all cached entries, but does not require instant update with invalidation.
    ///
    /// Despite nothing forbids language server from sending this request on every edit, it is expected to be sent only when certain internal server state update, invisible for the editor otherwise.
    RefreshRequested,
    /// Multibuffer excerpt(s) and/or singleton buffer(s) were edited at least on one place.
    /// Neither editor nor LSP is able to tell which open file hints' are not affected, so all of them have to be invalidated, re-queried and do that fast enough to avoid being slow, but also debounce to avoid loading hints on every fast keystroke sequence.
    BufferEdited,
    /// A new file got opened/new excerpt was added to a multibuffer/a [multi]buffer was scrolled to a new position.
    /// No invalidation should be done at all, all new hints are added to the cache.
    ///
    /// A special case is the settings change: in addition to LSP capabilities, Zed allows omitting certain hint kinds (defined by the corresponding LSP part: type/parameter/other).
    /// This does not lead to cache invalidation, but would require cache usage for determining which hints are not displayed and issuing an update to inlays on the screen.
    None,
}

/// A splice to send into the `inlay_map` for updating the visible inlays on the screen.
/// "Visible" inlays may not be displayed in the buffer right away, but those are ready to be displayed on further buffer scroll, pane item activations, etc. right away without additional LSP queries or settings changes.
/// The data in the cache is never used directly for displaying inlays on the screen, to avoid races with updates from LSP queries and sync overhead.
/// Splice is picked to help avoid extra hint flickering and "jumps" on the screen.
#[derive(Debug, Default)]
pub(super) struct InlaySplice {
    pub to_remove: Vec<InlayId>,
    pub to_insert: Vec<Inlay>,
}

#[derive(Debug)]
struct ExcerptHintsUpdate {
    excerpt_id: ExcerptId,
    remove_from_visible: HashSet<InlayId>,
    remove_from_cache: HashSet<InlayId>,
    add_to_cache: Vec<InlayHint>,
}

#[derive(Debug, Clone, Copy)]
struct ExcerptQuery {
    buffer_id: BufferId,
    excerpt_id: ExcerptId,
    cache_version: usize,
    invalidate: InvalidationStrategy,
    reason: &'static str,
}

impl InvalidationStrategy {
    fn should_invalidate(&self) -> bool {
        matches!(
            self,
            InvalidationStrategy::RefreshRequested | InvalidationStrategy::BufferEdited
        )
    }
}

impl TasksForRanges {
    fn new(query_ranges: QueryRanges, task: Task<()>) -> Self {
        let mut sorted_ranges = Vec::new();
        sorted_ranges.extend(query_ranges.before_visible);
        sorted_ranges.extend(query_ranges.visible);
        sorted_ranges.extend(query_ranges.after_visible);
        Self {
            tasks: vec![task],
            sorted_ranges,
        }
    }

    fn update_cached_tasks(
        &mut self,
        buffer_snapshot: &BufferSnapshot,
        query_ranges: QueryRanges,
        invalidate: InvalidationStrategy,
        spawn_task: impl FnOnce(QueryRanges) -> Task<()>,
    ) {
        let query_ranges = if invalidate.should_invalidate() {
            self.tasks.clear();
            self.sorted_ranges.clear();
            query_ranges
        } else {
            let mut non_cached_query_ranges = query_ranges;
            non_cached_query_ranges.before_visible = non_cached_query_ranges
                .before_visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges.visible = non_cached_query_ranges
                .visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges.after_visible = non_cached_query_ranges
                .after_visible
                .into_iter()
                .flat_map(|query_range| {
                    self.remove_cached_ranges_from_query(buffer_snapshot, query_range)
                })
                .collect();
            non_cached_query_ranges
        };

        if !query_ranges.is_empty() {
            self.tasks.push(spawn_task(query_ranges));
        }
    }

    fn remove_cached_ranges_from_query(
        &mut self,
        buffer_snapshot: &BufferSnapshot,
        query_range: Range<language::Anchor>,
    ) -> Vec<Range<language::Anchor>> {
        let mut ranges_to_query = Vec::new();
        let mut latest_cached_range = None::<&mut Range<language::Anchor>>;
        for cached_range in self
            .sorted_ranges
            .iter_mut()
            .skip_while(|cached_range| {
                cached_range
                    .end
                    .cmp(&query_range.start, buffer_snapshot)
                    .is_lt()
            })
            .take_while(|cached_range| {
                cached_range
                    .start
                    .cmp(&query_range.end, buffer_snapshot)
                    .is_le()
            })
        {
            match latest_cached_range {
                Some(latest_cached_range) => {
                    if latest_cached_range.end.offset.saturating_add(1) < cached_range.start.offset
                    {
                        ranges_to_query.push(latest_cached_range.end..cached_range.start);
                        cached_range.start = latest_cached_range.end;
                    }
                }
                None => {
                    if query_range
                        .start
                        .cmp(&cached_range.start, buffer_snapshot)
                        .is_lt()
                    {
                        ranges_to_query.push(query_range.start..cached_range.start);
                        cached_range.start = query_range.start;
                    }
                }
            }
            latest_cached_range = Some(cached_range);
        }

        match latest_cached_range {
            Some(latest_cached_range) => {
                if latest_cached_range.end.offset.saturating_add(1) < query_range.end.offset {
                    ranges_to_query.push(latest_cached_range.end..query_range.end);
                    latest_cached_range.end = query_range.end;
                }
            }
            None => {
                ranges_to_query.push(query_range.clone());
                self.sorted_ranges.push(query_range);
                self.sorted_ranges
                    .sort_by(|range_a, range_b| range_a.start.cmp(&range_b.start, buffer_snapshot));
            }
        }

        ranges_to_query
    }

    fn invalidate_range(&mut self, buffer: &BufferSnapshot, range: &Range<language::Anchor>) {
        self.sorted_ranges = self
            .sorted_ranges
            .drain(..)
            .filter_map(|mut cached_range| {
                if cached_range.start.cmp(&range.end, buffer).is_gt()
                    || cached_range.end.cmp(&range.start, buffer).is_lt()
                {
                    Some(vec![cached_range])
                } else if cached_range.start.cmp(&range.start, buffer).is_ge()
                    && cached_range.end.cmp(&range.end, buffer).is_le()
                {
                    None
                } else if range.start.cmp(&cached_range.start, buffer).is_ge()
                    && range.end.cmp(&cached_range.end, buffer).is_le()
                {
                    Some(vec![
                        cached_range.start..range.start,
                        range.end..cached_range.end,
                    ])
                } else if cached_range.start.cmp(&range.start, buffer).is_ge() {
                    cached_range.start = range.end;
                    Some(vec![cached_range])
                } else {
                    cached_range.end = range.start;
                    Some(vec![cached_range])
                }
            })
            .flatten()
            .collect();
    }
}

impl InlayHintCache {
    pub(super) fn new(inlay_hint_settings: InlayHintSettings) -> Self {
        Self {
            allowed_hint_kinds: inlay_hint_settings.enabled_inlay_hint_kinds(),
            enabled: inlay_hint_settings.enabled,
            hints: HashMap::default(),
            update_tasks: HashMap::default(),
            refresh_task: None,
            invalidate_debounce: debounce_value(inlay_hint_settings.edit_debounce_ms),
            append_debounce: debounce_value(inlay_hint_settings.scroll_debounce_ms),
            version: 0,
            lsp_request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_LSP_REQUESTS)),
        }
    }

    /// Checks inlay hint settings for enabled hint kinds and general enabled state.
    /// Generates corresponding inlay_map splice updates on settings changes.
    /// Does not update inlay hint cache state on disabling or inlay hint kinds change: only reenabling forces new LSP queries.
    pub(super) fn update_settings(
        &mut self,
        multi_buffer: &Model<MultiBuffer>,
        new_hint_settings: InlayHintSettings,
        visible_hints: Vec<Inlay>,
        cx: &mut ViewContext<Editor>,
    ) -> ControlFlow<Option<InlaySplice>> {
        self.invalidate_debounce = debounce_value(new_hint_settings.edit_debounce_ms);
        self.append_debounce = debounce_value(new_hint_settings.scroll_debounce_ms);
        let new_allowed_hint_kinds = new_hint_settings.enabled_inlay_hint_kinds();
        match (self.enabled, new_hint_settings.enabled) {
            (false, false) => {
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                ControlFlow::Break(None)
            }
            (true, true) => {
                if new_allowed_hint_kinds == self.allowed_hint_kinds {
                    ControlFlow::Break(None)
                } else {
                    let new_splice = self.new_allowed_hint_kinds_splice(
                        multi_buffer,
                        &visible_hints,
                        &new_allowed_hint_kinds,
                        cx,
                    );
                    if new_splice.is_some() {
                        self.version += 1;
                        self.allowed_hint_kinds = new_allowed_hint_kinds;
                    }
                    ControlFlow::Break(new_splice)
                }
            }
            (true, false) => {
                self.enabled = new_hint_settings.enabled;
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                if self.hints.is_empty() {
                    ControlFlow::Break(None)
                } else {
                    self.clear();
                    ControlFlow::Break(Some(InlaySplice {
                        to_remove: visible_hints.iter().map(|inlay| inlay.id).collect(),
                        to_insert: Vec::new(),
                    }))
                }
            }
            (false, true) => {
                self.enabled = new_hint_settings.enabled;
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                ControlFlow::Continue(())
            }
        }
    }

    /// If needed, queries LSP for new inlay hints, using the invalidation strategy given.
    /// To reduce inlay hint jumping, attempts to query a visible range of the editor(s) first,
    /// followed by the delayed queries of the same range above and below the visible one.
    /// This way, concequent refresh invocations are less likely to trigger LSP queries for the invisible ranges.
    pub(super) fn spawn_hint_refresh(
        &mut self,
        reason_description: &'static str,
        excerpts_to_query: HashMap<ExcerptId, (Model<Buffer>, Global, Range<usize>)>,
        invalidate: InvalidationStrategy,
        ignore_debounce: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Option<InlaySplice> {
        if !self.enabled {
            return None;
        }
        let mut invalidated_hints = Vec::new();
        if invalidate.should_invalidate() {
            self.update_tasks
                .retain(|task_excerpt_id, _| excerpts_to_query.contains_key(task_excerpt_id));
            self.hints.retain(|cached_excerpt, cached_hints| {
                let retain = excerpts_to_query.contains_key(cached_excerpt);
                if !retain {
                    invalidated_hints.extend(cached_hints.read().ordered_hints.iter().copied());
                }
                retain
            });
        }
        if excerpts_to_query.is_empty() && invalidated_hints.is_empty() {
            return None;
        }

        let cache_version = self.version + 1;
        let debounce_duration = if ignore_debounce {
            None
        } else if invalidate.should_invalidate() {
            self.invalidate_debounce
        } else {
            self.append_debounce
        };
        self.refresh_task = Some(cx.spawn(|editor, mut cx| async move {
            if let Some(debounce_duration) = debounce_duration {
                cx.background_executor().timer(debounce_duration).await;
            }

            editor
                .update(&mut cx, |editor, cx| {
                    spawn_new_update_tasks(
                        editor,
                        reason_description,
                        excerpts_to_query,
                        invalidate,
                        cache_version,
                        cx,
                    )
                })
                .ok();
        }));

        if invalidated_hints.is_empty() {
            None
        } else {
            Some(InlaySplice {
                to_remove: invalidated_hints,
                to_insert: Vec::new(),
            })
        }
    }

    fn new_allowed_hint_kinds_splice(
        &self,
        multi_buffer: &Model<MultiBuffer>,
        visible_hints: &[Inlay],
        new_kinds: &HashSet<Option<InlayHintKind>>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<InlaySplice> {
        let old_kinds = &self.allowed_hint_kinds;
        if new_kinds == old_kinds {
            return None;
        }

        let mut to_remove = Vec::new();
        let mut to_insert = Vec::new();
        let mut shown_hints_to_remove = visible_hints.iter().fold(
            HashMap::<ExcerptId, Vec<(Anchor, InlayId)>>::default(),
            |mut current_hints, inlay| {
                current_hints
                    .entry(inlay.position.excerpt_id)
                    .or_default()
                    .push((inlay.position, inlay.id));
                current_hints
            },
        );

        let multi_buffer = multi_buffer.read(cx);
        let multi_buffer_snapshot = multi_buffer.snapshot(cx);

        for (excerpt_id, excerpt_cached_hints) in &self.hints {
            let shown_excerpt_hints_to_remove =
                shown_hints_to_remove.entry(*excerpt_id).or_default();
            let excerpt_cached_hints = excerpt_cached_hints.read();
            let mut excerpt_cache = excerpt_cached_hints.ordered_hints.iter().fuse().peekable();
            shown_excerpt_hints_to_remove.retain(|(shown_anchor, shown_hint_id)| {
                let Some(buffer) = shown_anchor
                    .buffer_id
                    .and_then(|buffer_id| multi_buffer.buffer(buffer_id))
                else {
                    return false;
                };
                let buffer_snapshot = buffer.read(cx).snapshot();
                loop {
                    match excerpt_cache.peek() {
                        Some(&cached_hint_id) => {
                            let cached_hint = &excerpt_cached_hints.hints_by_id[cached_hint_id];
                            if cached_hint_id == shown_hint_id {
                                excerpt_cache.next();
                                return !new_kinds.contains(&cached_hint.kind);
                            }

                            match cached_hint
                                .position
                                .cmp(&shown_anchor.text_anchor, &buffer_snapshot)
                            {
                                cmp::Ordering::Less | cmp::Ordering::Equal => {
                                    if !old_kinds.contains(&cached_hint.kind)
                                        && new_kinds.contains(&cached_hint.kind)
                                    {
                                        if let Some(anchor) = multi_buffer_snapshot
                                            .anchor_in_excerpt(*excerpt_id, cached_hint.position)
                                        {
                                            to_insert.push(Inlay::hint(
                                                cached_hint_id.id(),
                                                anchor,
                                                &cached_hint,
                                            ));
                                        }
                                    }
                                    excerpt_cache.next();
                                }
                                cmp::Ordering::Greater => return true,
                            }
                        }
                        None => return true,
                    }
                }
            });

            for cached_hint_id in excerpt_cache {
                let maybe_missed_cached_hint = &excerpt_cached_hints.hints_by_id[cached_hint_id];
                let cached_hint_kind = maybe_missed_cached_hint.kind;
                if !old_kinds.contains(&cached_hint_kind) && new_kinds.contains(&cached_hint_kind) {
                    if let Some(anchor) = multi_buffer_snapshot
                        .anchor_in_excerpt(*excerpt_id, maybe_missed_cached_hint.position)
                    {
                        to_insert.push(Inlay::hint(
                            cached_hint_id.id(),
                            anchor,
                            &maybe_missed_cached_hint,
                        ));
                    }
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

    /// Completely forget of certain excerpts that were removed from the multibuffer.
    pub(super) fn remove_excerpts(
        &mut self,
        excerpts_removed: Vec<ExcerptId>,
    ) -> Option<InlaySplice> {
        let mut to_remove = Vec::new();
        for excerpt_to_remove in excerpts_removed {
            self.update_tasks.remove(&excerpt_to_remove);
            if let Some(cached_hints) = self.hints.remove(&excerpt_to_remove) {
                let cached_hints = cached_hints.read();
                to_remove.extend(cached_hints.ordered_hints.iter().copied());
            }
        }
        if to_remove.is_empty() {
            None
        } else {
            self.version += 1;
            Some(InlaySplice {
                to_remove,
                to_insert: Vec::new(),
            })
        }
    }

    pub(super) fn clear(&mut self) {
        if !self.update_tasks.is_empty() || !self.hints.is_empty() {
            self.version += 1;
        }
        self.update_tasks.clear();
        self.hints.clear();
    }

    pub(super) fn hint_by_id(&self, excerpt_id: ExcerptId, hint_id: InlayId) -> Option<InlayHint> {
        self.hints
            .get(&excerpt_id)?
            .read()
            .hints_by_id
            .get(&hint_id)
            .cloned()
    }

    pub fn hints(&self) -> Vec<InlayHint> {
        let mut hints = Vec::new();
        for excerpt_hints in self.hints.values() {
            let excerpt_hints = excerpt_hints.read();
            hints.extend(
                excerpt_hints
                    .ordered_hints
                    .iter()
                    .map(|id| &excerpt_hints.hints_by_id[id])
                    .cloned(),
            );
        }
        hints
    }

    pub fn version(&self) -> usize {
        self.version
    }

    /// Queries a certain hint from the cache for extra data via the LSP <a href="https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#inlayHint_resolve">resolve</a> request.
    pub(super) fn spawn_hint_resolve(
        &self,
        buffer_id: BufferId,
        excerpt_id: ExcerptId,
        id: InlayId,
        cx: &mut ViewContext<'_, Editor>,
    ) {
        if let Some(excerpt_hints) = self.hints.get(&excerpt_id) {
            let mut guard = excerpt_hints.write();
            if let Some(cached_hint) = guard.hints_by_id.get_mut(&id) {
                if let ResolveState::CanResolve(server_id, _) = &cached_hint.resolve_state {
                    let hint_to_resolve = cached_hint.clone();
                    let server_id = *server_id;
                    cached_hint.resolve_state = ResolveState::Resolving;
                    drop(guard);
                    cx.spawn(|editor, mut cx| async move {
                        let resolved_hint_task = editor.update(&mut cx, |editor, cx| {
                            editor
                                .buffer()
                                .read(cx)
                                .buffer(buffer_id)
                                .and_then(|buffer| {
                                    let project = editor.project.as_ref()?;
                                    Some(project.update(cx, |project, cx| {
                                        project.resolve_inlay_hint(
                                            hint_to_resolve,
                                            buffer,
                                            server_id,
                                            cx,
                                        )
                                    }))
                                })
                        })?;
                        if let Some(resolved_hint_task) = resolved_hint_task {
                            let mut resolved_hint =
                                resolved_hint_task.await.context("hint resolve task")?;
                            editor.update(&mut cx, |editor, _| {
                                if let Some(excerpt_hints) =
                                    editor.inlay_hint_cache.hints.get(&excerpt_id)
                                {
                                    let mut guard = excerpt_hints.write();
                                    if let Some(cached_hint) = guard.hints_by_id.get_mut(&id) {
                                        if cached_hint.resolve_state == ResolveState::Resolving {
                                            resolved_hint.resolve_state = ResolveState::Resolved;
                                            *cached_hint = resolved_hint;
                                        }
                                    }
                                }
                            })?;
                        }

                        anyhow::Ok(())
                    })
                    .detach_and_log_err(cx);
                }
            }
        }
    }
}

fn debounce_value(debounce_ms: u64) -> Option<Duration> {
    if debounce_ms > 0 {
        Some(Duration::from_millis(debounce_ms))
    } else {
        None
    }
}

fn spawn_new_update_tasks(
    editor: &mut Editor,
    reason: &'static str,
    excerpts_to_query: HashMap<ExcerptId, (Model<Buffer>, Global, Range<usize>)>,
    invalidate: InvalidationStrategy,
    update_cache_version: usize,
    cx: &mut ViewContext<'_, Editor>,
) {
    for (excerpt_id, (excerpt_buffer, new_task_buffer_version, excerpt_visible_range)) in
        excerpts_to_query
    {
        if excerpt_visible_range.is_empty() {
            continue;
        }
        let buffer = excerpt_buffer.read(cx);
        let buffer_id = buffer.remote_id();
        let buffer_snapshot = buffer.snapshot();
        if buffer_snapshot
            .version()
            .changed_since(&new_task_buffer_version)
        {
            continue;
        }

        if let Some(cached_excerpt_hints) = editor.inlay_hint_cache.hints.get(&excerpt_id) {
            let cached_excerpt_hints = cached_excerpt_hints.read();
            let cached_buffer_version = &cached_excerpt_hints.buffer_version;
            if cached_excerpt_hints.version > update_cache_version
                || cached_buffer_version.changed_since(&new_task_buffer_version)
            {
                continue;
            }
        };

        let Some(query_ranges) = editor.buffer.update(cx, |multi_buffer, cx| {
            determine_query_ranges(
                multi_buffer,
                excerpt_id,
                &excerpt_buffer,
                excerpt_visible_range,
                cx,
            )
        }) else {
            return;
        };
        let query = ExcerptQuery {
            buffer_id,
            excerpt_id,
            cache_version: update_cache_version,
            invalidate,
            reason,
        };

        let mut new_update_task =
            |query_ranges| new_update_task(query, query_ranges, excerpt_buffer.clone(), cx);

        match editor.inlay_hint_cache.update_tasks.entry(excerpt_id) {
            hash_map::Entry::Occupied(mut o) => {
                o.get_mut().update_cached_tasks(
                    &buffer_snapshot,
                    query_ranges,
                    invalidate,
                    new_update_task,
                );
            }
            hash_map::Entry::Vacant(v) => {
                v.insert(TasksForRanges::new(
                    query_ranges.clone(),
                    new_update_task(query_ranges),
                ));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct QueryRanges {
    before_visible: Vec<Range<language::Anchor>>,
    visible: Vec<Range<language::Anchor>>,
    after_visible: Vec<Range<language::Anchor>>,
}

impl QueryRanges {
    fn is_empty(&self) -> bool {
        self.before_visible.is_empty() && self.visible.is_empty() && self.after_visible.is_empty()
    }
}

fn determine_query_ranges(
    multi_buffer: &mut MultiBuffer,
    excerpt_id: ExcerptId,
    excerpt_buffer: &Model<Buffer>,
    excerpt_visible_range: Range<usize>,
    cx: &mut ModelContext<'_, MultiBuffer>,
) -> Option<QueryRanges> {
    let full_excerpt_range = multi_buffer
        .excerpts_for_buffer(excerpt_buffer, cx)
        .into_iter()
        .find(|(id, _)| id == &excerpt_id)
        .map(|(_, range)| range.context)?;
    let buffer = excerpt_buffer.read(cx);
    let snapshot = buffer.snapshot();
    let excerpt_visible_len = excerpt_visible_range.end - excerpt_visible_range.start;

    let visible_range = if excerpt_visible_range.start == excerpt_visible_range.end {
        return None;
    } else {
        vec![
            buffer.anchor_before(snapshot.clip_offset(excerpt_visible_range.start, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(excerpt_visible_range.end, Bias::Right)),
        ]
    };

    let full_excerpt_range_end_offset = full_excerpt_range.end.to_offset(&snapshot);
    let after_visible_range_start = excerpt_visible_range
        .end
        .saturating_add(1)
        .min(full_excerpt_range_end_offset)
        .min(buffer.len());
    let after_visible_range = if after_visible_range_start == full_excerpt_range_end_offset {
        Vec::new()
    } else {
        let after_range_end_offset = after_visible_range_start
            .saturating_add(excerpt_visible_len)
            .min(full_excerpt_range_end_offset)
            .min(buffer.len());
        vec![
            buffer.anchor_before(snapshot.clip_offset(after_visible_range_start, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(after_range_end_offset, Bias::Right)),
        ]
    };

    let full_excerpt_range_start_offset = full_excerpt_range.start.to_offset(&snapshot);
    let before_visible_range_end = excerpt_visible_range
        .start
        .saturating_sub(1)
        .max(full_excerpt_range_start_offset);
    let before_visible_range = if before_visible_range_end == full_excerpt_range_start_offset {
        Vec::new()
    } else {
        let before_range_start_offset = before_visible_range_end
            .saturating_sub(excerpt_visible_len)
            .max(full_excerpt_range_start_offset);
        vec![
            buffer.anchor_before(snapshot.clip_offset(before_range_start_offset, Bias::Left))
                ..buffer.anchor_after(snapshot.clip_offset(before_visible_range_end, Bias::Right)),
        ]
    };

    Some(QueryRanges {
        before_visible: before_visible_range,
        visible: visible_range,
        after_visible: after_visible_range,
    })
}

const MAX_CONCURRENT_LSP_REQUESTS: usize = 5;
const INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS: u64 = 400;

fn new_update_task(
    query: ExcerptQuery,
    query_ranges: QueryRanges,
    excerpt_buffer: Model<Buffer>,
    cx: &mut ViewContext<'_, Editor>,
) -> Task<()> {
    cx.spawn(move |editor, mut cx| async move {
        let visible_range_update_results = future::join_all(
            query_ranges
                .visible
                .into_iter()
                .filter_map(|visible_range| {
                    let fetch_task = editor
                        .update(&mut cx, |_, cx| {
                            fetch_and_update_hints(
                                excerpt_buffer.clone(),
                                query,
                                visible_range.clone(),
                                query.invalidate.should_invalidate(),
                                cx,
                            )
                        })
                        .log_err()?;
                    Some(async move { (visible_range, fetch_task.await) })
                }),
        )
        .await;

        let hint_delay = cx.background_executor().timer(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS,
        ));

        let query_range_failed =
            |range: &Range<language::Anchor>, e: anyhow::Error, cx: &mut AsyncWindowContext| {
                log::error!("inlay hint update task for range {range:?} failed: {e:#}");
                editor
                    .update(cx, |editor, cx| {
                        if let Some(task_ranges) = editor
                            .inlay_hint_cache
                            .update_tasks
                            .get_mut(&query.excerpt_id)
                        {
                            let buffer_snapshot = excerpt_buffer.read(cx).snapshot();
                            task_ranges.invalidate_range(&buffer_snapshot, &range);
                        }
                    })
                    .ok()
            };

        for (range, result) in visible_range_update_results {
            if let Err(e) = result {
                query_range_failed(&range, e, &mut cx);
            }
        }

        hint_delay.await;
        let invisible_range_update_results = future::join_all(
            query_ranges
                .before_visible
                .into_iter()
                .chain(query_ranges.after_visible.into_iter())
                .filter_map(|invisible_range| {
                    let fetch_task = editor
                        .update(&mut cx, |_, cx| {
                            fetch_and_update_hints(
                                excerpt_buffer.clone(),
                                query,
                                invisible_range.clone(),
                                false, // visible screen request already invalidated the entries
                                cx,
                            )
                        })
                        .log_err()?;
                    Some(async move { (invisible_range, fetch_task.await) })
                }),
        )
        .await;
        for (range, result) in invisible_range_update_results {
            if let Err(e) = result {
                query_range_failed(&range, e, &mut cx);
            }
        }
    })
}

fn fetch_and_update_hints(
    excerpt_buffer: Model<Buffer>,
    query: ExcerptQuery,
    fetch_range: Range<language::Anchor>,
    invalidate: bool,
    cx: &mut ViewContext<Editor>,
) -> Task<anyhow::Result<()>> {
    cx.spawn(|editor, mut cx| async move {
        let buffer_snapshot = excerpt_buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let (lsp_request_limiter, multi_buffer_snapshot) = editor.update(&mut cx, |editor, cx| {
            let multi_buffer_snapshot  = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let lsp_request_limiter = Arc::clone(&editor.inlay_hint_cache.lsp_request_limiter);
            (lsp_request_limiter, multi_buffer_snapshot)
        })?;

        let (lsp_request_guard, got_throttled) = if query.invalidate.should_invalidate() {
            (None, false)
        } else {
            match lsp_request_limiter.try_acquire() {
                Some(guard) => (Some(guard), false),
                None => (Some(lsp_request_limiter.acquire().await), true),
            }
        };
        let fetch_range_to_log =
            fetch_range.start.to_point(&buffer_snapshot)..fetch_range.end.to_point(&buffer_snapshot);
        let inlay_hints_fetch_task = editor
            .update(&mut cx, |editor, cx| {
                if got_throttled {
                    let query_not_around_visible_range = match editor.excerpts_for_inlay_hints_query(None, cx).remove(&query.excerpt_id) {
                        Some((_, _, current_visible_range)) => {
                            let visible_offset_length = current_visible_range.len();
                            let double_visible_range = current_visible_range
                                .start
                                .saturating_sub(visible_offset_length)
                                ..current_visible_range
                                    .end
                                    .saturating_add(visible_offset_length)
                                    .min(buffer_snapshot.len());
                            !double_visible_range
                                .contains(&fetch_range.start.to_offset(&buffer_snapshot))
                                && !double_visible_range
                                    .contains(&fetch_range.end.to_offset(&buffer_snapshot))
                        },
                        None => true,
                    };
                    if query_not_around_visible_range {
                        log::trace!("Fetching inlay hints for range {fetch_range_to_log:?} got throttled and fell off the current visible range, skipping.");
                        if let Some(task_ranges) = editor
                            .inlay_hint_cache
                            .update_tasks
                            .get_mut(&query.excerpt_id)
                        {
                            task_ranges.invalidate_range(&buffer_snapshot, &fetch_range);
                        }
                        return None;
                    }
                }
                editor
                    .buffer()
                    .read(cx)
                    .buffer(query.buffer_id)
                    .and_then(|buffer| {
                        let project = editor.project.as_ref()?;
                        Some(project.update(cx, |project, cx| {
                            project.inlay_hints(buffer, fetch_range.clone(), cx)
                        }))
                    })
            })
            .ok()
            .flatten();

        let cached_excerpt_hints = editor.update(&mut cx, |editor, _| {
            editor
                .inlay_hint_cache
                .hints
                .get(&query.excerpt_id)
                .cloned()
        })?;

        let visible_hints = editor.update(&mut cx, |editor, cx| editor.visible_inlay_hints(cx))?;
        let new_hints = match inlay_hints_fetch_task {
            Some(fetch_task) => {
                log::debug!(
                    "Fetching inlay hints for range {fetch_range_to_log:?}, reason: {query_reason}, invalidate: {invalidate}",
                    query_reason = query.reason,
                );
                log::trace!(
                    "Currently visible hints: {visible_hints:?}, cached hints present: {}",
                    cached_excerpt_hints.is_some(),
                );
                fetch_task.await.context("inlay hint fetch task")?
            }
            None => return Ok(()),
        };
        drop(lsp_request_guard);
        log::debug!(
            "Fetched {} hints for range {fetch_range_to_log:?}",
            new_hints.len()
        );
        log::trace!("Fetched hints: {new_hints:?}");

        let background_task_buffer_snapshot = buffer_snapshot.clone();
        let background_fetch_range = fetch_range.clone();
        let new_update = cx
            .background_executor()
            .spawn(async move {
                calculate_hint_updates(
                    query.excerpt_id,
                    invalidate,
                    background_fetch_range,
                    new_hints,
                    &background_task_buffer_snapshot,
                    cached_excerpt_hints,
                    &visible_hints,
                )
            })
            .await;
        if let Some(new_update) = new_update {
            log::debug!(
                "Applying update for range {fetch_range_to_log:?}: remove from editor: {}, remove from cache: {}, add to cache: {}",
                new_update.remove_from_visible.len(),
                new_update.remove_from_cache.len(),
                new_update.add_to_cache.len()
            );
            log::trace!("New update: {new_update:?}");
            editor
                .update(&mut cx, |editor, cx| {
                    apply_hint_update(
                        editor,
                        new_update,
                        query,
                        invalidate,
                        buffer_snapshot,
                        multi_buffer_snapshot,
                        cx,
                    );
                })
                .ok();
        }
        anyhow::Ok(())
    })
}

fn calculate_hint_updates(
    excerpt_id: ExcerptId,
    invalidate: bool,
    fetch_range: Range<language::Anchor>,
    new_excerpt_hints: Vec<InlayHint>,
    buffer_snapshot: &BufferSnapshot,
    cached_excerpt_hints: Option<Arc<RwLock<CachedExcerptHints>>>,
    visible_hints: &[Inlay],
) -> Option<ExcerptHintsUpdate> {
    let mut add_to_cache = Vec::<InlayHint>::new();
    let mut excerpt_hints_to_persist = HashMap::default();
    for new_hint in new_excerpt_hints {
        if !contains_position(&fetch_range, new_hint.position, buffer_snapshot) {
            continue;
        }
        let missing_from_cache = match &cached_excerpt_hints {
            Some(cached_excerpt_hints) => {
                let cached_excerpt_hints = cached_excerpt_hints.read();
                match cached_excerpt_hints
                    .ordered_hints
                    .binary_search_by(|probe| {
                        cached_excerpt_hints.hints_by_id[probe]
                            .position
                            .cmp(&new_hint.position, buffer_snapshot)
                    }) {
                    Ok(ix) => {
                        let mut missing_from_cache = true;
                        for id in &cached_excerpt_hints.ordered_hints[ix..] {
                            let cached_hint = &cached_excerpt_hints.hints_by_id[id];
                            if new_hint
                                .position
                                .cmp(&cached_hint.position, buffer_snapshot)
                                .is_gt()
                            {
                                break;
                            }
                            if cached_hint == &new_hint {
                                excerpt_hints_to_persist.insert(*id, cached_hint.kind);
                                missing_from_cache = false;
                            }
                        }
                        missing_from_cache
                    }
                    Err(_) => true,
                }
            }
            None => true,
        };
        if missing_from_cache {
            add_to_cache.push(new_hint);
        }
    }

    let mut remove_from_visible = HashSet::default();
    let mut remove_from_cache = HashSet::default();
    if invalidate {
        remove_from_visible.extend(
            visible_hints
                .iter()
                .filter(|hint| hint.position.excerpt_id == excerpt_id)
                .map(|inlay_hint| inlay_hint.id)
                .filter(|hint_id| !excerpt_hints_to_persist.contains_key(hint_id)),
        );

        if let Some(cached_excerpt_hints) = &cached_excerpt_hints {
            let cached_excerpt_hints = cached_excerpt_hints.read();
            remove_from_cache.extend(
                cached_excerpt_hints
                    .ordered_hints
                    .iter()
                    .filter(|cached_inlay_id| {
                        !excerpt_hints_to_persist.contains_key(cached_inlay_id)
                    })
                    .copied(),
            );
            remove_from_visible.extend(remove_from_cache.iter().cloned());
        }
    }

    if remove_from_visible.is_empty() && remove_from_cache.is_empty() && add_to_cache.is_empty() {
        None
    } else {
        Some(ExcerptHintsUpdate {
            excerpt_id,
            remove_from_visible,
            remove_from_cache,
            add_to_cache,
        })
    }
}

fn contains_position(
    range: &Range<language::Anchor>,
    position: language::Anchor,
    buffer_snapshot: &BufferSnapshot,
) -> bool {
    range.start.cmp(&position, buffer_snapshot).is_le()
        && range.end.cmp(&position, buffer_snapshot).is_ge()
}

fn apply_hint_update(
    editor: &mut Editor,
    new_update: ExcerptHintsUpdate,
    query: ExcerptQuery,
    invalidate: bool,
    buffer_snapshot: BufferSnapshot,
    multi_buffer_snapshot: MultiBufferSnapshot,
    cx: &mut ViewContext<'_, Editor>,
) {
    let cached_excerpt_hints = editor
        .inlay_hint_cache
        .hints
        .entry(new_update.excerpt_id)
        .or_insert_with(|| {
            Arc::new(RwLock::new(CachedExcerptHints {
                version: query.cache_version,
                buffer_version: buffer_snapshot.version().clone(),
                buffer_id: query.buffer_id,
                ordered_hints: Vec::new(),
                hints_by_id: HashMap::default(),
            }))
        });
    let mut cached_excerpt_hints = cached_excerpt_hints.write();
    match query.cache_version.cmp(&cached_excerpt_hints.version) {
        cmp::Ordering::Less => return,
        cmp::Ordering::Greater | cmp::Ordering::Equal => {
            cached_excerpt_hints.version = query.cache_version;
        }
    }

    let mut cached_inlays_changed = !new_update.remove_from_cache.is_empty();
    cached_excerpt_hints
        .ordered_hints
        .retain(|hint_id| !new_update.remove_from_cache.contains(hint_id));
    cached_excerpt_hints
        .hints_by_id
        .retain(|hint_id, _| !new_update.remove_from_cache.contains(hint_id));
    let mut splice = InlaySplice::default();
    splice.to_remove.extend(new_update.remove_from_visible);
    for new_hint in new_update.add_to_cache {
        let insert_position = match cached_excerpt_hints
            .ordered_hints
            .binary_search_by(|probe| {
                cached_excerpt_hints.hints_by_id[probe]
                    .position
                    .cmp(&new_hint.position, &buffer_snapshot)
            }) {
            Ok(i) => {
                let mut insert_position = Some(i);
                for id in &cached_excerpt_hints.ordered_hints[i..] {
                    let cached_hint = &cached_excerpt_hints.hints_by_id[id];
                    if new_hint
                        .position
                        .cmp(&cached_hint.position, &buffer_snapshot)
                        .is_gt()
                    {
                        break;
                    }
                    if cached_hint.text() == new_hint.text() {
                        insert_position = None;
                        break;
                    }
                }
                insert_position
            }
            Err(i) => Some(i),
        };

        if let Some(insert_position) = insert_position {
            let new_inlay_id = post_inc(&mut editor.next_inlay_id);
            if editor
                .inlay_hint_cache
                .allowed_hint_kinds
                .contains(&new_hint.kind)
            {
                if let Some(new_hint_position) =
                    multi_buffer_snapshot.anchor_in_excerpt(query.excerpt_id, new_hint.position)
                {
                    splice
                        .to_insert
                        .push(Inlay::hint(new_inlay_id, new_hint_position, &new_hint));
                }
            }
            let new_id = InlayId::Hint(new_inlay_id);
            cached_excerpt_hints.hints_by_id.insert(new_id, new_hint);
            cached_excerpt_hints
                .ordered_hints
                .insert(insert_position, new_id);
            cached_inlays_changed = true;
        }
    }
    cached_excerpt_hints.buffer_version = buffer_snapshot.version().clone();
    drop(cached_excerpt_hints);

    if invalidate {
        let mut outdated_excerpt_caches = HashSet::default();
        for (excerpt_id, excerpt_hints) in &editor.inlay_hint_cache().hints {
            let excerpt_hints = excerpt_hints.read();
            if excerpt_hints.buffer_id == query.buffer_id
                && excerpt_id != &query.excerpt_id
                && buffer_snapshot
                    .version()
                    .changed_since(&excerpt_hints.buffer_version)
            {
                outdated_excerpt_caches.insert(*excerpt_id);
                splice
                    .to_remove
                    .extend(excerpt_hints.ordered_hints.iter().copied());
            }
        }
        cached_inlays_changed |= !outdated_excerpt_caches.is_empty();
        editor
            .inlay_hint_cache
            .hints
            .retain(|excerpt_id, _| !outdated_excerpt_caches.contains(excerpt_id));
    }

    let InlaySplice {
        to_remove,
        to_insert,
    } = splice;
    let displayed_inlays_changed = !to_remove.is_empty() || !to_insert.is_empty();
    if cached_inlays_changed || displayed_inlays_changed {
        editor.inlay_hint_cache.version += 1;
    }
    if displayed_inlays_changed {
        editor.splice_inlays(to_remove, to_insert, cx)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

    use crate::{
        scroll::{scroll_amount::ScrollAmount, Autoscroll},
        ExcerptRange,
    };
    use futures::StreamExt;
    use gpui::{Context, TestAppContext, WindowHandle};
    use itertools::Itertools;
    use language::{
        language_settings::AllLanguageSettingsContent, Capability, FakeLspAdapter, Language,
        LanguageConfig, LanguageMatcher,
    };
    use lsp::FakeLanguageServer;
    use parking_lot::Mutex;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use text::{Point, ToPoint};

    use crate::editor_tests::update_test_language_settings;

    use super::*;

    #[gpui::test]
    async fn test_basic_cache_update_with_duplicate_hints(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                show_parameter_hints: allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                show_other_hints: allowed_hint_kinds.contains(&None),
            })
        });

        let (file_with_hints, editor, fake_server) = prepare_test_objects(cx).await;
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path(file_with_hints).unwrap(),
                    );
                    let current_call_id =
                        Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                    let mut new_hints = Vec::with_capacity(2 * current_call_id as usize);
                    for _ in 0..2 {
                        let mut i = current_call_id;
                        loop {
                            new_hints.push(lsp::InlayHint {
                                position: lsp::Position::new(0, i),
                                label: lsp::InlayHintLabel::String(i.to_string()),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: None,
                                padding_right: None,
                                data: None,
                            });
                            if i == 0 {
                                break;
                            }
                            i -= 1;
                        }
                    }

                    Ok(Some(new_hints))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        let mut edits_made = 1;
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor update the cache version after every cache/view change"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
                editor.handle_input("some change", cx);
                edits_made += 1;
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string(), "1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get new hints after an edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor update the cache version after every cache/view change"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .expect("inlay refresh request failed");
        edits_made += 1;
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string(), "1".to_string(), "2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get new hints after hint refresh/ request"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor update the cache version after every cache/view change"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_cache_update_on_lsp_completion_tasks(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let (file_with_hints, editor, fake_server) = prepare_test_objects(cx).await;
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path(file_with_hints).unwrap(),
                    );
                    let current_call_id =
                        Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                    Ok(Some(vec![lsp::InlayHint {
                        position: lsp::Position::new(0, current_call_id),
                        label: lsp::InlayHintLabel::String(current_call_id.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        let mut edits_made = 1;
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    edits_made,
                    "The editor update the cache version after every cache/view change"
                );
            })
            .unwrap();

        let progress_token = "test_progress_token";
        fake_server
            .request::<lsp::request::WorkDoneProgressCreate>(lsp::WorkDoneProgressCreateParams {
                token: lsp::ProgressToken::String(progress_token.to_string()),
            })
            .await
            .expect("work done progress create request failed");
        cx.executor().run_until_parked();
        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::String(progress_token.to_string()),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Begin(
                lsp::WorkDoneProgressBegin::default(),
            )),
        });
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should not update hints while the work task is running"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    edits_made,
                    "Should not update the cache while the work task is running"
                );
            })
            .unwrap();

        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::String(progress_token.to_string()),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::End(
                lsp::WorkDoneProgressEnd::default(),
            )),
        });
        cx.executor().run_until_parked();

        edits_made += 1;
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "New hints should be queried after the work task is done"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    edits_made,
                    "Cache version should update once after the work task is done"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_no_hint_updates_for_unrelated_language_files(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
                    "/a",
                    json!({
                        "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                        "other.md": "Test md file with some text",
                    }),
                )
                .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let mut rs_fake_servers = None;
        let mut md_fake_servers = None;
        for (name, path_suffix) in [("Rust", "rs"), ("Markdown", "md")] {
            language_registry.add(Arc::new(Language::new(
                LanguageConfig {
                    name: name.into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec![path_suffix.to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_rust::language()),
            )));
            let fake_servers = language_registry.register_fake_lsp_adapter(
                name,
                FakeLspAdapter {
                    name,
                    capabilities: lsp::ServerCapabilities {
                        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );
            match name {
                "Rust" => rs_fake_servers = Some(fake_servers),
                "Markdown" => md_fake_servers = Some(fake_servers),
                _ => unreachable!(),
            }
        }

        let rs_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/a/main.rs", cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        cx.executor().start_waiting();
        let rs_fake_server = rs_fake_servers.unwrap().next().await.unwrap();
        let rs_editor =
            cx.add_window(|cx| Editor::for_buffer(rs_buffer, Some(project.clone()), cx));
        let rs_lsp_request_count = Arc::new(AtomicU32::new(0));
        rs_fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&rs_lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path("/a/main.rs").unwrap(),
                    );
                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                    Ok(Some(vec![lsp::InlayHint {
                        position: lsp::Position::new(0, i),
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    1,
                    "Rust editor update the cache version after every cache/view change"
                );
            })
            .unwrap();

        cx.executor().run_until_parked();
        let md_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/a/other.md", cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        cx.executor().start_waiting();
        let md_fake_server = md_fake_servers.unwrap().next().await.unwrap();
        let md_editor = cx.add_window(|cx| Editor::for_buffer(md_buffer, Some(project), cx));
        let md_lsp_request_count = Arc::new(AtomicU32::new(0));
        md_fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&md_lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path("/a/other.md").unwrap(),
                    );
                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                    Ok(Some(vec![lsp::InlayHint {
                        position: lsp::Position::new(0, i),
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Markdown editor should have a separate version, repeating Rust editor rules"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 1);
            })
            .unwrap();

        rs_editor
            .update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
                editor.handle_input("some rs change", cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Rust inlay cache should change after the edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    2,
                    "Every time hint cache changes, cache version should be incremented"
                );
            })
            .unwrap();
        md_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Markdown editor should not be affected by Rust editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 1);
            })
            .unwrap();

        md_editor
            .update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
                editor.handle_input("some md change", cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Rust editor should not be affected by Markdown editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 2);
            })
            .unwrap();
        rs_editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Markdown editor should also change independently"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 2);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_setting_changes(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                show_parameter_hints: allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                show_other_hints: allowed_hint_kinds.contains(&None),
            })
        });

        let (file_with_hints, editor, fake_server) = prepare_test_objects(cx).await;
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let another_lsp_request_count = Arc::clone(&lsp_request_count);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&another_lsp_request_count);
                async move {
                    Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst);
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path(file_with_hints).unwrap(),
                    );
                    Ok(Some(vec![
                        lsp::InlayHint {
                            position: lsp::Position::new(0, 1),
                            label: lsp::InlayHintLabel::String("type hint".to_string()),
                            kind: Some(lsp::InlayHintKind::TYPE),
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        },
                        lsp::InlayHint {
                            position: lsp::Position::new(0, 2),
                            label: lsp::InlayHintLabel::String("parameter hint".to_string()),
                            kind: Some(lsp::InlayHintKind::PARAMETER),
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        },
                        lsp::InlayHint {
                            position: lsp::Position::new(0, 3),
                            label: lsp::InlayHintLabel::String("other hint".to_string()),
                            kind: None,
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        },
                    ]))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        let mut edits_made = 1;
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    1,
                    "Should query new hints once"
                );
                assert_eq!(
                    vec![
                        "other hint".to_string(),
                        "parameter hint".to_string(),
                        "type hint".to_string(),
                    ],
                    cached_hint_labels(editor),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(
                    vec!["other hint".to_string(), "type hint".to_string()],
                    visible_hint_labels(editor, cx)
                );
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor update the cache version after every cache/view change"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should load new hints twice"
                );
                assert_eq!(
                    vec![
                        "other hint".to_string(),
                        "parameter hint".to_string(),
                        "type hint".to_string(),
                    ],
                    cached_hint_labels(editor),
                    "Cached hints should not change due to allowed hint kinds settings update"
                );
                assert_eq!(
                    vec!["other hint".to_string(), "type hint".to_string()],
                    visible_hint_labels(editor, cx)
                );
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    edits_made,
                    "Should not update cache version due to new loaded hints being the same"
                );
            })
            .unwrap();

        for (new_allowed_hint_kinds, expected_visible_hints) in [
            (HashSet::from_iter([None]), vec!["other hint".to_string()]),
            (
                HashSet::from_iter([Some(InlayHintKind::Type)]),
                vec!["type hint".to_string()],
            ),
            (
                HashSet::from_iter([Some(InlayHintKind::Parameter)]),
                vec!["parameter hint".to_string()],
            ),
            (
                HashSet::from_iter([None, Some(InlayHintKind::Type)]),
                vec!["other hint".to_string(), "type hint".to_string()],
            ),
            (
                HashSet::from_iter([None, Some(InlayHintKind::Parameter)]),
                vec!["other hint".to_string(), "parameter hint".to_string()],
            ),
            (
                HashSet::from_iter([Some(InlayHintKind::Type), Some(InlayHintKind::Parameter)]),
                vec!["parameter hint".to_string(), "type hint".to_string()],
            ),
            (
                HashSet::from_iter([
                    None,
                    Some(InlayHintKind::Type),
                    Some(InlayHintKind::Parameter),
                ]),
                vec![
                    "other hint".to_string(),
                    "parameter hint".to_string(),
                    "type hint".to_string(),
                ],
            ),
        ] {
            edits_made += 1;
            update_test_language_settings(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettings {
                    enabled: true,
                    edit_debounce_ms: 0,
                    scroll_debounce_ms: 0,
                    show_type_hints: new_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                    show_parameter_hints: new_allowed_hint_kinds
                        .contains(&Some(InlayHintKind::Parameter)),
                    show_other_hints: new_allowed_hint_kinds.contains(&None),
                })
            });
            cx.executor().run_until_parked();
            editor.update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints on allowed hint kinds change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    vec![
                        "other hint".to_string(),
                        "parameter hint".to_string(),
                        "type hint".to_string(),
                    ],
                    cached_hint_labels(editor),
                    "Should get its cached hints unchanged after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    expected_visible_hints,
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints filtered after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, new_allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor should update the cache version after every cache/view change for hint kinds {new_allowed_hint_kinds:?} due to visible hints change"
                );
            }).unwrap();
        }

        edits_made += 1;
        let another_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Type)]);
        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: false,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: another_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                show_parameter_hints: another_allowed_hint_kinds
                    .contains(&Some(InlayHintKind::Parameter)),
                show_other_hints: another_allowed_hint_kinds.contains(&None),
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints when hints got disabled"
                );
                assert!(
                    cached_hint_labels(editor).is_empty(),
                    "Should clear the cache when hints got disabled"
                );
                assert!(
                    visible_hint_labels(editor, cx).is_empty(),
                    "Should clear visible hints when hints got disabled"
                );
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, another_allowed_hint_kinds,
                    "Should update its allowed hint kinds even when hints got disabled"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "The editor should update the cache version after hints got disabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
            assert_eq!(
                lsp_request_count.load(Ordering::Relaxed),
                2,
                "Should not load new hints when they got disabled"
            );
            assert!(cached_hint_labels(editor).is_empty());
            assert!(visible_hint_labels(editor, cx).is_empty());
            assert_eq!(
                editor.inlay_hint_cache().version, edits_made,
                "The editor should not update the cache version after /refresh query without updates"
            );
        }).unwrap();

        let final_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Parameter)]);
        edits_made += 1;
        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: final_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                show_parameter_hints: final_allowed_hint_kinds
                    .contains(&Some(InlayHintKind::Parameter)),
                show_other_hints: final_allowed_hint_kinds.contains(&None),
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    3,
                    "Should query for new hints when they got re-enabled"
                );
                assert_eq!(
                    vec![
                        "other hint".to_string(),
                        "parameter hint".to_string(),
                        "type hint".to_string(),
                    ],
                    cached_hint_labels(editor),
                    "Should get its cached hints fully repopulated after the hints got re-enabled"
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints repopulated and filtered after the h"
                );
                let inlay_cache = editor.inlay_hint_cache();
                assert_eq!(
                    inlay_cache.allowed_hint_kinds, final_allowed_hint_kinds,
                    "Cache should update editor settings when hints got re-enabled"
                );
                assert_eq!(
                    inlay_cache.version, edits_made,
                    "Cache should update its version after hints got re-enabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    4,
                    "Should query for new hints again"
                );
                assert_eq!(
                    vec![
                        "other hint".to_string(),
                        "parameter hint".to_string(),
                        "type hint".to_string(),
                    ],
                    cached_hint_labels(editor),
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                );
                assert_eq!(editor.inlay_hint_cache().version, edits_made);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_request_cancellation(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let (file_with_hints, editor, fake_server) = prepare_test_objects(cx).await;
        let fake_server = Arc::new(fake_server);
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let another_lsp_request_count = Arc::clone(&lsp_request_count);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&another_lsp_request_count);
                async move {
                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst) + 1;
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path(file_with_hints).unwrap(),
                    );
                    Ok(Some(vec![lsp::InlayHint {
                        position: lsp::Position::new(0, i),
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;

        let mut expected_changes = Vec::new();
        for change_after_opening in [
            "initial change #1",
            "initial change #2",
            "initial change #3",
        ] {
            editor
                .update(cx, |editor, cx| {
                    editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
                    editor.handle_input(change_after_opening, cx);
                })
                .unwrap();
            expected_changes.push(change_after_opening);
        }

        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
            let current_text = editor.text(cx);
            for change in &expected_changes {
                assert!(
                    current_text.contains(change),
                    "Should apply all changes made"
                );
            }
            assert_eq!(
                lsp_request_count.load(Ordering::Relaxed),
                2,
                "Should query new hints twice: for editor init and for the last edit that interrupted all others"
            );
            let expected_hints = vec!["2".to_string()];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor),
                "Should get hints from the last edit landed only"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            assert_eq!(
                editor.inlay_hint_cache().version, 1,
                "Only one update should be registered in the cache after all cancellations"
            );
        }).unwrap();

        let mut edits = Vec::new();
        for async_later_change in [
            "another change #1",
            "another change #2",
            "another change #3",
        ] {
            expected_changes.push(async_later_change);
            let task_editor = editor;
            edits.push(cx.spawn(|mut cx| async move {
                task_editor
                    .update(&mut cx, |editor, cx| {
                        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
                        editor.handle_input(async_later_change, cx);
                    })
                    .unwrap();
            }));
        }
        let _ = future::join_all(edits).await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, cx| {
                let current_text = editor.text(cx);
                for change in &expected_changes {
                    assert!(
                        current_text.contains(change),
                        "Should apply all changes made"
                    );
                }
                assert_eq!(
                    lsp_request_count.load(Ordering::SeqCst),
                    3,
                    "Should query new hints one more time, for the last edit only"
                );
                let expected_hints = vec!["3".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should get hints from the last edit landed only"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    2,
                    "Should update the cache version once more, for the new change"
                );
            })
            .unwrap();
    }

    #[gpui::test(iterations = 10)]
    async fn test_large_buffer_inlay_requests_split(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/a",
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", "let i = 5;\n".repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(crate::editor_tests::rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/a/main.rs", cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        cx.executor().start_waiting();
        let fake_server = fake_servers.next().await.unwrap();
        let editor = cx.add_window(|cx| Editor::for_buffer(buffer, Some(project), cx));
        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));
        let lsp_request_count = Arc::new(AtomicUsize::new(0));
        let closure_lsp_request_ranges = Arc::clone(&lsp_request_ranges);
        let closure_lsp_request_count = Arc::clone(&lsp_request_count);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_ranges = Arc::clone(&closure_lsp_request_ranges);
                let task_lsp_request_count = Arc::clone(&closure_lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path("/a/main.rs").unwrap(),
                    );

                    task_lsp_request_ranges.lock().push(params.range);
                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::Release) + 1;
                    Ok(Some(vec![lsp::InlayHint {
                        position: params.range.end,
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;

        fn editor_visible_range(
            editor: &WindowHandle<Editor>,
            cx: &mut gpui::TestAppContext,
        ) -> Range<Point> {
            let ranges = editor
                .update(cx, |editor, cx| {
                    editor.excerpts_for_inlay_hints_query(None, cx)
                })
                .unwrap();
            assert_eq!(
                ranges.len(),
                1,
                "Single buffer should produce a single excerpt with visible range"
            );
            let (_, (excerpt_buffer, _, excerpt_visible_range)) =
                ranges.into_iter().next().unwrap();
            excerpt_buffer.update(cx, |buffer, _| {
                let snapshot = buffer.snapshot();
                let start = buffer
                    .anchor_before(excerpt_visible_range.start)
                    .to_point(&snapshot);
                let end = buffer
                    .anchor_after(excerpt_visible_range.end)
                    .to_point(&snapshot);
                start..end
            })
        }

        // in large buffers, requests are made for more than visible range of a buffer.
        // invisible parts are queried later, to avoid excessive requests on quick typing.
        // wait the timeout needed to get all requests.
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        let initial_visible_range = editor_visible_range(&editor, cx);
        let lsp_initial_visible_range = lsp::Range::new(
            lsp::Position::new(
                initial_visible_range.start.row,
                initial_visible_range.start.column,
            ),
            lsp::Position::new(
                initial_visible_range.end.row,
                initial_visible_range.end.column,
            ),
        );
        let expected_initial_query_range_end =
            lsp::Position::new(initial_visible_range.end.row * 2, 2);
        let mut expected_invisible_query_start = lsp_initial_visible_range.end;
        expected_invisible_query_start.character += 1;
        editor.update(cx, |editor, cx| {
            let ranges = lsp_request_ranges.lock().drain(..).collect::<Vec<_>>();
            assert_eq!(ranges.len(), 2,
                "When scroll is at the edge of a big document, its visible part and the same range further should be queried in order, but got: {ranges:?}");
            let visible_query_range = &ranges[0];
            assert_eq!(visible_query_range.start, lsp_initial_visible_range.start);
            assert_eq!(visible_query_range.end, lsp_initial_visible_range.end);
            let invisible_query_range = &ranges[1];

            assert_eq!(invisible_query_range.start, expected_invisible_query_start, "Should initially query visible edge of the document");
            assert_eq!(invisible_query_range.end, expected_initial_query_range_end, "Should initially query visible edge of the document");

            let requests_count = lsp_request_count.load(Ordering::Acquire);
            assert_eq!(requests_count, 2, "Visible + invisible request");
            let expected_hints = vec!["1".to_string(), "2".to_string()];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor),
                "Should have hints from both LSP requests made for a big file"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx), "Should display only hints from the visible range");
            assert_eq!(
                editor.inlay_hint_cache().version, requests_count,
                "LSP queries should've bumped the cache version"
            );
        }).unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        let visible_range_after_scrolls = editor_visible_range(&editor, cx);
        let visible_line_count = editor
            .update(cx, |editor, _| editor.visible_line_count().unwrap())
            .unwrap();
        let selection_in_cached_range = editor
            .update(cx, |editor, cx| {
                let ranges = lsp_request_ranges
                    .lock()
                    .drain(..)
                    .sorted_by_key(|r| r.start)
                    .collect::<Vec<_>>();
                assert_eq!(
                    ranges.len(),
                    2,
                    "Should query 2 ranges after both scrolls, but got: {ranges:?}"
                );
                let first_scroll = &ranges[0];
                let second_scroll = &ranges[1];
                assert_eq!(
                    first_scroll.end, second_scroll.start,
                    "Should query 2 adjacent ranges after the scrolls, but got: {ranges:?}"
                );
                assert_eq!(
                first_scroll.start, expected_initial_query_range_end,
                "First scroll should start the query right after the end of the original scroll",
            );
                assert_eq!(
                second_scroll.end,
                lsp::Position::new(
                    visible_range_after_scrolls.end.row
                        + visible_line_count.ceil() as u32,
                    1,
                ),
                "Second scroll should query one more screen down after the end of the visible range"
            );

                let lsp_requests = lsp_request_count.load(Ordering::Acquire);
                assert_eq!(lsp_requests, 4, "Should query for hints after every scroll");
                let expected_hints = vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should have hints from the new LSP response after the edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    lsp_requests,
                    "Should update the cache for every LSP response with hints added"
                );

                let mut selection_in_cached_range = visible_range_after_scrolls.end;
                selection_in_cached_range.row -= visible_line_count.ceil() as u32;
                selection_in_cached_range
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                    s.select_ranges([selection_in_cached_range..selection_in_cached_range])
                });
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        editor.update(cx, |_, _| {
            let ranges = lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|r| r.start)
                .collect::<Vec<_>>();
            assert!(ranges.is_empty(), "No new ranges or LSP queries should be made after returning to the selection with cached hints");
            assert_eq!(lsp_request_count.load(Ordering::Acquire), 4);
        }).unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.handle_input("++++more text++++", cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
            let mut ranges = lsp_request_ranges.lock().drain(..).collect::<Vec<_>>();
            ranges.sort_by_key(|r| r.start);

            assert_eq!(ranges.len(), 3,
                "On edit, should scroll to selection and query a range around it: visible + same range above and below. Instead, got query ranges {ranges:?}");
            let above_query_range = &ranges[0];
            let visible_query_range = &ranges[1];
            let below_query_range = &ranges[2];
            assert!(above_query_range.end.character < visible_query_range.start.character || above_query_range.end.line + 1 == visible_query_range.start.line,
                "Above range {above_query_range:?} should be before visible range {visible_query_range:?}");
            assert!(visible_query_range.end.character < below_query_range.start.character || visible_query_range.end.line  + 1 == below_query_range.start.line,
                "Visible range {visible_query_range:?} should be before below range {below_query_range:?}");
            assert!(above_query_range.start.line < selection_in_cached_range.row,
                "Hints should be queried with the selected range after the query range start");
            assert!(below_query_range.end.line > selection_in_cached_range.row,
                "Hints should be queried with the selected range before the query range end");
            assert!(above_query_range.start.line <= selection_in_cached_range.row - (visible_line_count * 3.0 / 2.0) as u32,
                "Hints query range should contain one more screen before");
            assert!(below_query_range.end.line >= selection_in_cached_range.row + (visible_line_count * 3.0 / 2.0) as u32,
                "Hints query range should contain one more screen after");

            let lsp_requests = lsp_request_count.load(Ordering::Acquire);
            assert_eq!(lsp_requests, 7, "There should be a visible range and two ranges above and below it queried");
            let expected_hints = vec!["5".to_string(), "6".to_string(), "7".to_string()];
            assert_eq!(expected_hints, cached_hint_labels(editor),
                "Should have hints from the new LSP response after the edit");
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            assert_eq!(editor.inlay_hint_cache().version, lsp_requests, "Should update the cache for every LSP response with hints added");
        }).unwrap();
    }

    #[gpui::test(iterations = 30)]
    async fn test_multiple_excerpts_large_multibuffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
                "/a",
                json!({
                    "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<Vec<_>>().join("")),
                    "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<Vec<_>>().join("")),
                }),
            )
            .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = crate::editor_tests::rust_lang();
        language_registry.add(language);
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees().next().unwrap().read(cx).id()
        });

        let buffer_1 = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, "main.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_2 = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, "other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0, Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 0)..Point::new(2, 0),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(4, 0)..Point::new(11, 0),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(22, 0)..Point::new(33, 0),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(44, 0)..Point::new(55, 0),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(56, 0)..Point::new(66, 0),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(67, 0)..Point::new(77, 0),
                        primary: None,
                    },
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [
                    ExcerptRange {
                        context: Point::new(0, 1)..Point::new(2, 1),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(4, 1)..Point::new(11, 1),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(22, 1)..Point::new(33, 1),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(44, 1)..Point::new(55, 1),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(56, 1)..Point::new(66, 1),
                        primary: None,
                    },
                    ExcerptRange {
                        context: Point::new(67, 1)..Point::new(77, 1),
                        primary: None,
                    },
                ],
                cx,
            );
            multibuffer
        });

        cx.executor().run_until_parked();
        let editor = cx
            .add_window(|cx| Editor::for_multibuffer(multibuffer, Some(project.clone()), true, cx));

        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Url::from_file_path("/a/main.rs").unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Url::from_file_path("/a/other.rs").unwrap()
                    {
                        "other hint"
                    } else {
                        panic!("unexpected uri: {:?}", params.text_document.uri);
                    };

                    // one hint per excerpt
                    let positions = [
                        lsp::Position::new(0, 2),
                        lsp::Position::new(4, 2),
                        lsp::Position::new(22, 2),
                        lsp::Position::new(44, 2),
                        lsp::Position::new(56, 2),
                        lsp::Position::new(67, 2),
                    ];
                    let out_of_range_hint = lsp::InlayHint {
                        position: lsp::Position::new(
                            params.range.start.line + 99,
                            params.range.start.character + 99,
                        ),
                        label: lsp::InlayHintLabel::String(
                            "out of excerpt range, should be ignored".to_string(),
                        ),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    };

                    let edited = task_editor_edited.load(Ordering::Acquire);
                    Ok(Some(
                        std::iter::once(out_of_range_hint)
                            .chain(positions.into_iter().enumerate().map(|(i, position)| {
                                lsp::InlayHint {
                                    position,
                                    label: lsp::InlayHintLabel::String(format!(
                                        "{hint_text}{} #{i}",
                                        if edited { "(edited)" } else { "" },
                                    )),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }
                            }))
                            .collect(),
                    ))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        editor.update(cx, |editor, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "When scroll is at the edge of a multibuffer, its visible excerpts only should be queried for inlay hints"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, expected_hints.len(), "Every visible excerpt hints should bump the version");
            }).unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges([Point::new(4, 0)..Point::new(4, 0)])
                });
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges([Point::new(22, 0)..Point::new(22, 0)])
                });
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges([Point::new(50, 0)..Point::new(50, 0)])
                });
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                ];
                assert_eq!(expected_hints, cached_hint_labels(editor),
                    "With more scrolls of the multibuffer, more hints should be added into the cache and nothing invalidated without edits");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, expected_hints.len(),
                    "Due to every excerpt having one hint, we update cache per new excerpt scrolled");
            }).unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges([Point::new(100, 0)..Point::new(100, 0)])
                });
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        let last_scroll_update_version = editor.update(cx, |editor, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                    "other hint #3".to_string(),
                    "other hint #4".to_string(),
                    "other hint #5".to_string(),
                ];
                assert_eq!(expected_hints, cached_hint_labels(editor),
                    "After multibuffer was scrolled to the end, all hints for all excerpts should be fetched");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, expected_hints.len());
                expected_hints.len()
            }).unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.change_selections(Some(Autoscroll::Next), cx, |s| {
                    s.select_ranges([Point::new(4, 0)..Point::new(4, 0)])
                });
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(
            INVISIBLE_RANGES_HINTS_REQUEST_DELAY_MILLIS + 100,
        ));
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                    "other hint #0".to_string(),
                    "other hint #1".to_string(),
                    "other hint #2".to_string(),
                    "other hint #3".to_string(),
                    "other hint #4".to_string(),
                    "other hint #5".to_string(),
                ];
                assert_eq!(expected_hints, cached_hint_labels(editor),
                    "After multibuffer was scrolled to the end, further scrolls up should not bring more hints");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, last_scroll_update_version, "No updates should happen during scrolling already scrolled buffer");
            }).unwrap();

        editor_edited.store(true, Ordering::Release);
        editor
            .update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.select_ranges([Point::new(57, 0)..Point::new(57, 0)])
                });
                editor.handle_input("++++more text++++", cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
            let expected_hints = vec![
                "main hint #0".to_string(),
                "main hint #1".to_string(),
                "main hint #2".to_string(),
                "main hint #3".to_string(),
                "main hint #4".to_string(),
                "main hint #5".to_string(),
                "other hint(edited) #0".to_string(),
                "other hint(edited) #1".to_string(),
                "other hint(edited) #2".to_string(),
            ];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor),
                "After multibuffer edit, editor gets scrolled back to the last selection; \
    all hints should be invalidated and required for all of its visible excerpts"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));

            let current_cache_version = editor.inlay_hint_cache().version;
            // We expect three new hints for the excerpts from `other.rs`:
            let expected_version = last_scroll_update_version + 3;
            assert_eq!(
                current_cache_version,
                expected_version,
                "We should have updated cache N times == N of new hints arrived (separately from each edited excerpt)"
            );
        }).unwrap();
    }

    #[gpui::test]
    async fn test_excerpts_removed(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: false,
                show_parameter_hints: false,
                show_other_hints: false,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/a",
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<Vec<_>>().join("")),
                "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<Vec<_>>().join("")),
            }),
        )
        .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(crate::editor_tests::rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees().next().unwrap().read(cx).id()
        });

        let buffer_1 = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, "main.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_2 = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, "other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new_model(|_| MultiBuffer::new(0, Capability::ReadWrite));
        let (buffer_1_excerpts, buffer_2_excerpts) = multibuffer.update(cx, |multibuffer, cx| {
            let buffer_1_excerpts = multibuffer.push_excerpts(
                buffer_1.clone(),
                [ExcerptRange {
                    context: Point::new(0, 0)..Point::new(2, 0),
                    primary: None,
                }],
                cx,
            );
            let buffer_2_excerpts = multibuffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange {
                    context: Point::new(0, 1)..Point::new(2, 1),
                    primary: None,
                }],
                cx,
            );
            (buffer_1_excerpts, buffer_2_excerpts)
        });

        assert!(!buffer_1_excerpts.is_empty());
        assert!(!buffer_2_excerpts.is_empty());

        cx.executor().run_until_parked();
        let editor = cx
            .add_window(|cx| Editor::for_multibuffer(multibuffer, Some(project.clone()), true, cx));
        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Url::from_file_path("/a/main.rs").unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Url::from_file_path("/a/other.rs").unwrap()
                    {
                        "other hint"
                    } else {
                        panic!("unexpected uri: {:?}", params.text_document.uri);
                    };

                    let positions = [
                        lsp::Position::new(0, 2),
                        lsp::Position::new(4, 2),
                        lsp::Position::new(22, 2),
                        lsp::Position::new(44, 2),
                        lsp::Position::new(56, 2),
                        lsp::Position::new(67, 2),
                    ];
                    let out_of_range_hint = lsp::InlayHint {
                        position: lsp::Position::new(
                            params.range.start.line + 99,
                            params.range.start.character + 99,
                        ),
                        label: lsp::InlayHintLabel::String(
                            "out of excerpt range, should be ignored".to_string(),
                        ),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    };

                    let edited = task_editor_edited.load(Ordering::Acquire);
                    Ok(Some(
                        std::iter::once(out_of_range_hint)
                            .chain(positions.into_iter().enumerate().map(|(i, position)| {
                                lsp::InlayHint {
                                    position,
                                    label: lsp::InlayHintLabel::String(format!(
                                        "{hint_text}{} #{i}",
                                        if edited { "(edited)" } else { "" },
                                    )),
                                    kind: None,
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }
                            }))
                            .collect(),
                    ))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    vec!["main hint #0".to_string(), "other hint #0".to_string()],
                    cached_hint_labels(editor),
                    "Cache should update for both excerpts despite hints display was disabled"
                );
                assert!(
                visible_hint_labels(editor, cx).is_empty(),
                "All hints are disabled and should not be shown despite being present in the cache"
            );
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    2,
                    "Cache should update once per excerpt query"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.buffer().update(cx, |multibuffer, cx| {
                    multibuffer.remove_excerpts(buffer_2_excerpts, cx)
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert_eq!(
                    vec!["main hint #0".to_string()],
                    cached_hint_labels(editor),
                    "For the removed excerpt, should clean corresponding cached hints"
                );
                assert!(
                visible_hint_labels(editor, cx).is_empty(),
                "All hints are disabled and should not be shown despite being present in the cache"
            );
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    3,
                    "Excerpt removal should trigger a cache update"
                );
            })
            .unwrap();

        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["main hint #0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Hint display settings change should not change the cache"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "Settings change should make cached hints visible"
                );
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    4,
                    "Settings change should trigger a cache update"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_inside_char_boundary_range_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/a",
            json!({
                "main.rs": format!(r#"fn main() {{\n{}\n}}"#, format!("let i = {};\n", "√".repeat(10)).repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(crate::editor_tests::rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/a/main.rs", cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        cx.executor().start_waiting();
        let fake_server = fake_servers.next().await.unwrap();
        let editor = cx.add_window(|cx| Editor::for_buffer(buffer, Some(project), cx));
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let closure_lsp_request_count = Arc::clone(&lsp_request_count);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&closure_lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path("/a/main.rs").unwrap(),
                    );
                    let query_start = params.range.start;
                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::Release) + 1;
                    Ok(Some(vec![lsp::InlayHint {
                        position: query_start,
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                editor.change_selections(None, cx, |s| {
                    s.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(expected_hints, cached_hint_labels(editor));
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 1);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_toggle_inlay_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: false,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let (file_with_hints, editor, fake_server) = prepare_test_objects(cx).await;

        editor
            .update(cx, |editor, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, cx)
            })
            .unwrap();
        cx.executor().start_waiting();
        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let closure_lsp_request_count = Arc::clone(&lsp_request_count);
        fake_server
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_lsp_request_count = Arc::clone(&closure_lsp_request_count);
                async move {
                    assert_eq!(
                        params.text_document.uri,
                        lsp::Url::from_file_path(file_with_hints).unwrap(),
                    );

                    let i = Arc::clone(&task_lsp_request_count).fetch_add(1, Ordering::SeqCst) + 1;
                    Ok(Some(vec![lsp::InlayHint {
                        position: lsp::Position::new(0, i),
                        label: lsp::InlayHintLabel::String(i.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should display inlays after toggle despite them disabled in settings"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    editor.inlay_hint_cache().version,
                    1,
                    "First toggle should be cache's first update"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert!(
                    cached_hint_labels(editor).is_empty(),
                    "Should clear hints after 2nd toggle"
                );
                assert!(visible_hint_labels(editor, cx).is_empty());
                assert_eq!(editor.inlay_hint_cache().version, 2);
            })
            .unwrap();

        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                edit_debounce_ms: 0,
                scroll_debounce_ms: 0,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor),
                    "Should query LSP hints for the 2nd time after enabling hints in settings"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(editor.inlay_hint_cache().version, 3);
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, cx| {
                assert!(
                    cached_hint_labels(editor).is_empty(),
                    "Should clear hints after enabling in settings and a 3rd toggle"
                );
                assert!(visible_hint_labels(editor, cx).is_empty());
                assert_eq!(editor.inlay_hint_cache().version, 4);
            })
            .unwrap();

        editor
            .update(cx, |editor, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor.update(cx, |editor, cx| {
            let expected_hints = vec!["3".to_string()];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor),
                "Should query LSP hints for the 3rd time after enabling hints in settings and toggling them back on"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            assert_eq!(editor.inlay_hint_cache().version, 5);
        }).unwrap();
    }

    pub(crate) fn init_test(cx: &mut TestAppContext, f: impl Fn(&mut AllLanguageSettingsContent)) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            release_channel::init("0.0.0", cx);
            client::init_settings(cx);
            language::init(cx);
            Project::init_settings(cx);
            workspace::init_settings(cx);
            crate::init(cx);
        });

        update_test_language_settings(cx, f);
    }

    async fn prepare_test_objects(
        cx: &mut TestAppContext,
    ) -> (&'static str, WindowHandle<Editor>, FakeLanguageServer) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/a",
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, ["/a".as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(crate::editor_tests::rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp_adapter(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer("/a/main.rs", cx)
            })
            .await
            .unwrap();
        cx.executor().run_until_parked();
        cx.executor().start_waiting();
        let fake_server = fake_servers.next().await.unwrap();
        let editor = cx.add_window(|cx| Editor::for_buffer(buffer, Some(project), cx));

        editor
            .update(cx, |editor, cx| {
                assert!(cached_hint_labels(editor).is_empty());
                assert!(visible_hint_labels(editor, cx).is_empty());
                assert_eq!(editor.inlay_hint_cache().version, 0);
            })
            .unwrap();

        ("/a/main.rs", editor, fake_server)
    }

    pub fn cached_hint_labels(editor: &Editor) -> Vec<String> {
        let mut labels = Vec::new();
        for (_, excerpt_hints) in &editor.inlay_hint_cache().hints {
            let excerpt_hints = excerpt_hints.read();
            for id in &excerpt_hints.ordered_hints {
                labels.push(excerpt_hints.hints_by_id[id].text());
            }
        }

        labels.sort();
        labels
    }

    pub fn visible_hint_labels(editor: &Editor, cx: &ViewContext<'_, Editor>) -> Vec<String> {
        let mut hints = editor
            .visible_inlay_hints(cx)
            .into_iter()
            .map(|hint| hint.text.to_string())
            .collect::<Vec<_>>();
        hints.sort();
        hints
    }
}
