use std::{
    collections::hash_map,
    ops::{ControlFlow, Range},
    sync::Arc,
    time::Duration,
};

use anyhow::Context as _;
use clock::Global;
use collections::{HashMap, HashSet, IndexMap};
use futures::future;
use gpui::{AppContext as _, AsyncApp, Entity, Task};
use language::{language_settings::SemanticTokensSettings, Buffer, BufferSnapshot};
use multi_buffer::{ExcerptId, MultiBufferSnapshot};
use parking_lot::RwLock;
use project::SemanticToken;
use smol::lock::Semaphore;
use text::{AnchorRangeExt, BufferId, ToOffset, ToPoint as _};
use ui::{ActiveTheme, Context};
use util::{post_inc, ResultExt};

use crate::{
    display_map::Token,
    tasks_for_ranges::{
        contains_position, determine_query_ranges, InvalidationStrategy, QueryRanges,
        TasksForRanges,
    },
    Editor,
};

const MAX_CONCURRENT_LSP_REQUESTS: usize = 5;
const INVISIBLE_RANGES_TOKENS_REQUEST_DELAY_MILLIS: u64 = 400;

pub struct SemanticTokensCache {
    pub(crate) enabled: bool,
    tokens: HashMap<ExcerptId, Arc<RwLock<CachedExcerptTokens>>>,
    enabled_in_settings: bool,
    update_tasks: HashMap<ExcerptId, TasksForRanges>,
    refresh_task: Task<()>,
    invalidate_debounce: Option<Duration>,
    append_debounce: Option<Duration>,
    lsp_request_limiter: Arc<Semaphore>,
    version: usize,
}

#[derive(Debug, Default)]
pub(super) struct TokenSplice {
    pub to_remove: Vec<usize>,
    pub to_insert: Vec<Token>,
}

#[derive(Debug)]
struct CachedExcerptTokens {
    version: usize,
    buffer_version: Global,
    buffer_id: BufferId,
    ordered_tokens: Vec<usize>,
    tokens_by_id: IndexMap<usize, SemanticToken>,
}

#[derive(Debug, Clone, Copy)]
struct ExcerptQuery {
    buffer_id: BufferId,
    excerpt_id: ExcerptId,
    cache_version: usize,
    invalidate: InvalidationStrategy,
    reason: &'static str,
}

#[derive(Debug)]
struct ExcerptTokensUpdate {
    excerpt_id: ExcerptId,
    remove_from_visible: HashSet<usize>,
    remove_from_cache: HashSet<usize>,
    add_to_cache: Vec<SemanticToken>,
}

impl SemanticTokensCache {
    pub(super) fn new(semantic_tokens_settings: SemanticTokensSettings) -> Self {
        Self {
            enabled: semantic_tokens_settings.enabled,
            version: 0,
            enabled_in_settings: semantic_tokens_settings.enabled,
            tokens: Default::default(),
            refresh_task: Task::ready(()),
            update_tasks: Default::default(),
            invalidate_debounce: debounce_value(semantic_tokens_settings.edit_debounce_ms),
            append_debounce: debounce_value(semantic_tokens_settings.scroll_debounce_ms),
            lsp_request_limiter: Arc::new(Semaphore::new(MAX_CONCURRENT_LSP_REQUESTS)),
        }
    }

    /// If needed, queries LSP for new semantic tokens, using the invalidation strategy given.
    /// To reduce semantic tokens jumping, attempts to query a visible range of the editor(s) first,
    /// followed by the delayed queries of the same range above and below the visible one.
    /// This way, subsequent refresh invocations are less likely to trigger LSP queries for the invisible ranges.
    pub(super) fn spawn_token_refresh(
        &mut self,
        reason_description: &'static str,
        excerpts_to_query: HashMap<ExcerptId, (Entity<Buffer>, Global, Range<usize>)>,
        invalidate: InvalidationStrategy,
        ignore_debounce: bool,
        cx: &mut Context<Editor>,
    ) -> Option<TokenSplice> {
        if !self.enabled {
            return None;
        }
        let mut invalidated_tokens = Vec::new();
        if invalidate.should_invalidate() {
            self.update_tasks
                .retain(|task_excerpt_id, _| excerpts_to_query.contains_key(task_excerpt_id));
            self.tokens.retain(|cached_excerpt, cached_tokens| {
                let retain = excerpts_to_query.contains_key(cached_excerpt);
                if !retain {
                    invalidated_tokens.extend(cached_tokens.read().ordered_tokens.iter().copied());
                }
                retain
            });
        }
        if excerpts_to_query.is_empty() && invalidated_tokens.is_empty() {
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
        self.refresh_task = cx.spawn(async move |editor, cx| {
            if let Some(debounce_duration) = debounce_duration {
                cx.background_executor().timer(debounce_duration).await;
            }

            editor
                .update(cx, |editor, cx| {
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
        });

        if invalidated_tokens.is_empty() {
            None
        } else {
            Some(TokenSplice {
                to_remove: invalidated_tokens,
                to_insert: Vec::new(),
            })
        }
    }

    /// Checks semantic token settings for enabled hint kinds and general enabled state.
    /// Generates corresponding inlay_map splice updates on settings changes.
    /// Does not update inlay hint cache state on disabling or inlay hint kinds change: only reenabling forces new LSP queries.
    pub(super) fn update_settings(
        &mut self,
        new_token_settings: SemanticTokensSettings,
        visible_tokens: Vec<Token>,
    ) -> ControlFlow<Option<TokenSplice>> {
        let old_enabled = self.enabled;
        // If the setting for semantic tokens has changed, update `enabled`. This condition avoids inlay
        // hint visibility changes when other settings change (such as theme).
        //
        // Another option might be to store whether the user has manually toggled semantic tokens
        // visibility, and prefer this. This could lead to confusion as it means semantic tokens
        // visibility would not change when updating the setting if they were ever toggled.
        if new_token_settings.enabled != self.enabled_in_settings {
            self.enabled = new_token_settings.enabled;
            self.enabled_in_settings = new_token_settings.enabled;
        };
        self.invalidate_debounce = debounce_value(new_token_settings.edit_debounce_ms);
        self.append_debounce = debounce_value(new_token_settings.scroll_debounce_ms);
        match (old_enabled, self.enabled) {
            (false, false) => ControlFlow::Break(None),
            (true, true) => ControlFlow::Break(None),
            (true, false) => {
                if self.tokens.is_empty() {
                    ControlFlow::Break(None)
                } else {
                    self.clear();
                    ControlFlow::Break(Some(TokenSplice {
                        to_remove: visible_tokens.iter().map(|inlay| inlay.id).collect(),
                        to_insert: Vec::new(),
                    }))
                }
            }
            (false, true) => ControlFlow::Continue(()),
        }
    }

    pub(super) fn toggle(&mut self, enabled: bool) -> bool {
        if self.enabled == enabled {
            return false;
        }
        self.enabled = enabled;
        if !enabled {
            self.clear();
        }
        true
    }

    /// Completely forget of certain excerpts that were removed from the multibuffer.
    pub(super) fn remove_excerpts(
        &mut self,
        excerpts_removed: Vec<ExcerptId>,
    ) -> Option<TokenSplice> {
        let mut to_remove = Vec::new();
        for excerpt_to_remove in excerpts_removed {
            self.update_tasks.remove(&excerpt_to_remove);
            if let Some(cached_tokens) = self.tokens.remove(&excerpt_to_remove) {
                let cached_tokens = cached_tokens.read();
                to_remove.extend(cached_tokens.ordered_tokens.iter().copied());
            }
        }
        if to_remove.is_empty() {
            None
        } else {
            self.version += 1;
            Some(TokenSplice {
                to_remove,
                to_insert: Vec::new(),
            })
        }
    }

    pub(super) fn clear(&mut self) {
        if !self.update_tasks.is_empty() || !self.tokens.is_empty() {
            self.version += 1;
        }
        self.update_tasks.clear();
        self.refresh_task = Task::ready(());
        self.tokens.clear();
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
    excerpts_to_query: HashMap<ExcerptId, (Entity<Buffer>, Global, Range<usize>)>,
    invalidate: InvalidationStrategy,
    update_cache_version: usize,
    cx: &mut Context<Editor>,
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

        if let Some(cached_excerpt_tokens) = editor.semantic_tokens_cache.tokens.get(&excerpt_id) {
            let cached_excerpt_tokens = cached_excerpt_tokens.read();
            let cached_buffer_version = &cached_excerpt_tokens.buffer_version;
            if cached_excerpt_tokens.version > update_cache_version
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

        match editor.semantic_tokens_cache.update_tasks.entry(excerpt_id) {
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

fn new_update_task(
    query: ExcerptQuery,
    query_ranges: QueryRanges,
    excerpt_buffer: Entity<Buffer>,
    cx: &mut Context<Editor>,
) -> Task<()> {
    cx.spawn(async move |editor, cx| {
        let visible_range_update_results = future::join_all(
            query_ranges
                .visible
                .into_iter()
                .filter_map(|visible_range| {
                    let fetch_task = editor
                        .update(cx, |_, cx| {
                            fetch_and_update_tokens(
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

        let token_delay = cx.background_executor().timer(Duration::from_millis(
            INVISIBLE_RANGES_TOKENS_REQUEST_DELAY_MILLIS,
        ));

        let query_range_failed =
            |range: &Range<language::Anchor>, e: anyhow::Error, cx: &mut AsyncApp| {
                log::error!("semantic tokens update task for range failed: {e:#?}");
                editor
                    .update(cx, |editor, cx| {
                        if let Some(task_ranges) = editor
                            .semantic_tokens_cache
                            .update_tasks
                            .get_mut(&query.excerpt_id)
                        {
                            let buffer_snapshot = excerpt_buffer.read(cx).snapshot();
                            task_ranges.invalidate_range(&buffer_snapshot, range);
                        }
                    })
                    .ok()
            };

        for (range, result) in visible_range_update_results {
            if let Err(e) = result {
                query_range_failed(&range, e, cx);
            }
        }

        token_delay.await;
        let invisible_range_update_results = future::join_all(
            query_ranges
                .before_visible
                .into_iter()
                .chain(query_ranges.after_visible.into_iter())
                .filter_map(|invisible_range| {
                    let fetch_task = editor
                        .update(cx, |_, cx| {
                            fetch_and_update_tokens(
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
                query_range_failed(&range, e, cx);
            }
        }
    })
}

fn fetch_and_update_tokens(
    excerpt_buffer: Entity<Buffer>,
    query: ExcerptQuery,
    fetch_range: Range<language::Anchor>,
    invalidate: bool,
    cx: &mut Context<Editor>,
) -> Task<anyhow::Result<()>> {
    cx.spawn(async move |editor, cx|{
        let buffer_snapshot = excerpt_buffer.update(cx, |buffer, _| buffer.snapshot())?;
        let (lsp_request_limiter, multi_buffer_snapshot) =
            editor.update(cx, |editor, cx| {
                let multi_buffer_snapshot =
                    editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
                let lsp_request_limiter = Arc::clone(&editor.semantic_tokens_cache.lsp_request_limiter);
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
        let fetch_range_to_log = fetch_range.start.to_point(&buffer_snapshot)
            ..fetch_range.end.to_point(&buffer_snapshot);
        let semantic_tokens_fetch_task = editor
            .update(cx, |editor, cx| {
                if got_throttled {
                    let query_not_around_visible_range = match editor
                        .excerpts_for_query(None, cx)
                        .remove(&query.excerpt_id)
                    {
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
                        }
                        None => true,
                    };
                    if query_not_around_visible_range {
                        log::trace!("Fetching semantic tokens for range {fetch_range_to_log:?} got throttled and fell off the current visible range, skipping.");
                        if let Some(task_ranges) = editor
                            .semantic_tokens_cache
                            .update_tasks
                            .get_mut(&query.excerpt_id)
                        {
                            task_ranges.invalidate_range(&buffer_snapshot, &fetch_range);
                        }
                        return None;
                    }
                }

                let buffer = editor.buffer().read(cx).buffer(query.buffer_id)?;

                // TODO: request range
                editor
                    .semantics_provider
                    .as_ref()?
                    .semantic_tokens(buffer, cx)
            })
            .ok()
            .flatten();

        let cached_excerpt_tokens = editor.update(cx, |editor, _| {
            editor
                .semantic_tokens_cache
                .tokens
                .get(&query.excerpt_id)
                .cloned()
        })?;

        let visible_tokens = editor.update(cx, |editor, cx| editor.visible_semantic_tokens(cx))?;
        let new_tokens = match semantic_tokens_fetch_task {
            Some(fetch_task) => {
                log::debug!(
                    "Fetching semantic tokens for range {fetch_range_to_log:?}, reason: {query_reason}, invalidate: {invalidate}",
                    query_reason = query.reason,
                );
                log::trace!(
                    "Currently visible semantic tokens: {visible_tokens:?}, cached semantic tokens present: {}",
                    cached_excerpt_tokens.is_some(),
                );
                fetch_task.await.context("semantic tokens fetch task")?
            }
            None => return Ok(()),
        };
        drop(lsp_request_guard);
        log::debug!(
            "Fetched {} semantic tokens for range {fetch_range_to_log:?}",
            new_tokens.len()
        );
        log::trace!("Fetched semantic tokens: {new_tokens:?}");

        let background_task_buffer_snapshot = buffer_snapshot.clone();
        let background_fetch_range = fetch_range.clone();
        let new_update = cx.background_spawn(async move {
            calculate_token_updates(
                query.excerpt_id,
                invalidate,
                background_fetch_range,
                new_tokens,
                &background_task_buffer_snapshot,
                cached_excerpt_tokens,
                &visible_tokens,
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
                .update(cx, |editor,  cx| {
                    apply_token_update(
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

fn calculate_token_updates(
    excerpt_id: ExcerptId,
    invalidate: bool,
    fetch_range: Range<language::Anchor>,
    new_excerpt_tokens: Vec<SemanticToken>,
    buffer_snapshot: &BufferSnapshot,
    cached_excerpt_tokens: Option<Arc<RwLock<CachedExcerptTokens>>>,
    visible_tokens: &[Token],
) -> Option<ExcerptTokensUpdate> {
    let mut add_to_cache = Vec::<SemanticToken>::new();
    for new_token in new_excerpt_tokens {
        if !contains_position(&fetch_range, new_token.range.start, buffer_snapshot) {
            continue;
        }
        if !contains_position(&fetch_range, new_token.range.end, buffer_snapshot) {
            continue;
        }
        let missing_from_cache = match &cached_excerpt_tokens {
            Some(cached_excerpt_tokens) => {
                let cached_excerpt_tokens = cached_excerpt_tokens.read();
                match cached_excerpt_tokens
                    .ordered_tokens
                    .binary_search_by(|probe| {
                        cached_excerpt_tokens.tokens_by_id[probe]
                            .range
                            .cmp(&new_token.range, buffer_snapshot)
                    }) {
                    Ok(ix) => {
                        let mut missing_from_cache = true;
                        for id in &cached_excerpt_tokens.ordered_tokens[ix..] {
                            let cached_token = &cached_excerpt_tokens.tokens_by_id[id];
                            if new_token
                                .range
                                .cmp(&cached_token.range, buffer_snapshot)
                                .is_gt()
                            {
                                break;
                            }
                            if cached_token == &new_token {
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
            add_to_cache.push(new_token);
        }
    }

    let mut remove_from_visible = HashSet::default();
    let mut remove_from_cache = HashSet::default();
    if invalidate {
        remove_from_visible.extend(
            visible_tokens
                .iter()
                .filter(|token| token.range.start.excerpt_id == excerpt_id)
                .map(|token| token.id),
        );

        if let Some(cached_excerpt_tokens) = &cached_excerpt_tokens {
            let cached_excerpt_tokens = cached_excerpt_tokens.read();
            remove_from_cache.extend(cached_excerpt_tokens.ordered_tokens.iter().copied());
            remove_from_visible.extend(remove_from_cache.iter().cloned());
        }
    }

    if remove_from_visible.is_empty() && remove_from_cache.is_empty() && add_to_cache.is_empty() {
        None
    } else {
        Some(ExcerptTokensUpdate {
            excerpt_id,
            remove_from_visible,
            remove_from_cache,
            add_to_cache,
        })
    }
}

fn apply_token_update(
    editor: &mut Editor,
    new_update: ExcerptTokensUpdate,
    query: ExcerptQuery,
    invalidate: bool,
    buffer_snapshot: BufferSnapshot,
    multi_buffer_snapshot: MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) {
    let cached_excerpt_tokens = editor
        .semantic_tokens_cache
        .tokens
        .entry(new_update.excerpt_id)
        .or_insert_with(|| {
            Arc::new(RwLock::new(CachedExcerptTokens {
                version: query.cache_version,
                buffer_version: buffer_snapshot.version().clone(),
                buffer_id: query.buffer_id,
                ordered_tokens: Vec::new(),
                tokens_by_id: IndexMap::default(),
            }))
        });
    let mut cached_excerpt_tokens = cached_excerpt_tokens.write();
    match query.cache_version.cmp(&cached_excerpt_tokens.version) {
        std::cmp::Ordering::Less => return,
        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
            cached_excerpt_tokens.version = query.cache_version;
        }
    }

    let mut cached_tokens_changed = !new_update.remove_from_cache.is_empty();
    cached_excerpt_tokens
        .ordered_tokens
        .retain(|token_id| !new_update.remove_from_cache.contains(token_id));
    cached_excerpt_tokens
        .tokens_by_id
        .retain(|token_id, _| !new_update.remove_from_cache.contains(token_id));
    let mut splice = TokenSplice::default();
    splice.to_remove.extend(new_update.remove_from_visible);
    for new_token in new_update.add_to_cache {
        let Some(mut style) = cx.theme().tokens().get(new_token.r#type.as_str()) else {
            continue;
        };
        for r#mod in new_token.modifiers.iter() {
            let Some(r#mod) = cx.theme().modifiers().get(r#mod.as_str()) else {
                continue;
            };
            style.highlight(r#mod);
        }
        let insert_position = match cached_excerpt_tokens
            .ordered_tokens
            .binary_search_by(|probe| {
                cached_excerpt_tokens.tokens_by_id[probe]
                    .range
                    .cmp(&new_token.range, &buffer_snapshot)
            }) {
            Ok(i) => {
                // When a token is added to the same position where existing ones are present,
                // do not deduplicate it: we split token queries into non-overlapping ranges
                // and each token batch returned by the server should already contain unique tokens.
                i + cached_excerpt_tokens.ordered_tokens[i..].len() + 1
            }
            Err(i) => i,
        };

        let new_token_id = post_inc(&mut editor.next_semantic_id);
        if let (Some(new_start), Some(new_end)) = (
            multi_buffer_snapshot.anchor_in_excerpt(query.excerpt_id, new_token.range.start),
            multi_buffer_snapshot.anchor_in_excerpt(query.excerpt_id, new_token.range.end),
        ) {
            splice
                .to_insert
                .push(Token::new(new_token_id, new_start..new_end, style));
        }
        cached_excerpt_tokens
            .tokens_by_id
            .insert(new_token_id, new_token);
        if cached_excerpt_tokens.ordered_tokens.len() <= insert_position {
            cached_excerpt_tokens.ordered_tokens.push(new_token_id);
        } else {
            cached_excerpt_tokens
                .ordered_tokens
                .insert(insert_position, new_token_id);
        }

        cached_tokens_changed = true;
    }
    cached_excerpt_tokens.buffer_version = buffer_snapshot.version().clone();
    drop(cached_excerpt_tokens);

    if invalidate {
        let mut outdated_excerpt_caches = HashSet::default();
        for (excerpt_id, excerpt_tokens) in &editor.semantic_tokens_cache.tokens {
            let excerpt_tokens = excerpt_tokens.read();
            if excerpt_tokens.buffer_id == query.buffer_id
                && excerpt_id != &query.excerpt_id
                && buffer_snapshot
                    .version()
                    .changed_since(&excerpt_tokens.buffer_version)
            {
                outdated_excerpt_caches.insert(*excerpt_id);
                splice
                    .to_remove
                    .extend(excerpt_tokens.ordered_tokens.iter().cloned());
            }
        }
        cached_tokens_changed |= !outdated_excerpt_caches.is_empty();
        editor
            .semantic_tokens_cache
            .tokens
            .retain(|excerpt_id, _| !outdated_excerpt_caches.contains(excerpt_id));
    }

    let TokenSplice {
        to_remove,
        to_insert,
    } = splice;
    let displayed_tokens_changed = !to_remove.is_empty() || !to_insert.is_empty();
    if cached_tokens_changed || displayed_tokens_changed {
        editor.semantic_tokens_cache.version += 1;
    }
    if displayed_tokens_changed {
        editor.splice_tokens(&to_remove, to_insert, cx)
    }
}
