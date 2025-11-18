use std::{
    collections::hash_map,
    ops::{ControlFlow, Range},
    time::Duration,
};

use clock::Global;
use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{App, Entity, Task};
use language::{
    BufferRow,
    language_settings::{InlayHintKind, InlayHintSettings, language_settings},
};
use lsp::LanguageServerId;
use multi_buffer::{Anchor, ExcerptId, MultiBufferSnapshot};
use project::{
    HoverBlock, HoverBlockKind, InlayHintLabel, InlayHintLabelPartTooltip, InlayHintTooltip,
    InvalidationStrategy, ResolveState,
    lsp_store::{CacheInlayHints, ResolvedHint},
};
use text::{Bias, BufferId};
use ui::{Context, Window};
use util::debug_panic;

use super::{Inlay, InlayId};
use crate::{
    Editor, EditorSnapshot, PointForPosition, ToggleInlayHints, ToggleInlineValues, debounce_value,
    hover_links::{InlayHighlight, TriggerPoint, show_link_definition},
    hover_popover::{self, InlayHover},
    inlays::InlaySplice,
};

pub fn inlay_hint_settings(
    location: Anchor,
    snapshot: &MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) -> InlayHintSettings {
    let file = snapshot.file_at(location);
    let language = snapshot.language_at(location).map(|l| l.name());
    language_settings(language, file, cx).inlay_hints
}

#[derive(Debug)]
pub struct LspInlayHintData {
    enabled: bool,
    modifiers_override: bool,
    enabled_in_settings: bool,
    allowed_hint_kinds: HashSet<Option<InlayHintKind>>,
    invalidate_debounce: Option<Duration>,
    append_debounce: Option<Duration>,
    hint_refresh_tasks: HashMap<BufferId, Vec<Task<()>>>,
    hint_chunk_fetching: HashMap<BufferId, (Global, HashSet<Range<BufferRow>>)>,
    invalidate_hints_for_buffers: HashSet<BufferId>,
    pub added_hints: HashMap<InlayId, Option<InlayHintKind>>,
}

impl LspInlayHintData {
    pub fn new(settings: InlayHintSettings) -> Self {
        Self {
            modifiers_override: false,
            enabled: settings.enabled,
            enabled_in_settings: settings.enabled,
            hint_refresh_tasks: HashMap::default(),
            added_hints: HashMap::default(),
            hint_chunk_fetching: HashMap::default(),
            invalidate_hints_for_buffers: HashSet::default(),
            invalidate_debounce: debounce_value(settings.edit_debounce_ms),
            append_debounce: debounce_value(settings.scroll_debounce_ms),
            allowed_hint_kinds: settings.enabled_inlay_hint_kinds(),
        }
    }

    pub fn modifiers_override(&mut self, new_override: bool) -> Option<bool> {
        if self.modifiers_override == new_override {
            return None;
        }
        self.modifiers_override = new_override;
        if (self.enabled && self.modifiers_override) || (!self.enabled && !self.modifiers_override)
        {
            self.clear();
            Some(false)
        } else {
            Some(true)
        }
    }

    pub fn toggle(&mut self, enabled: bool) -> bool {
        if self.enabled == enabled {
            return false;
        }
        self.enabled = enabled;
        self.modifiers_override = false;
        if !enabled {
            self.clear();
        }
        true
    }

    pub fn clear(&mut self) {
        self.hint_refresh_tasks.clear();
        self.hint_chunk_fetching.clear();
        self.added_hints.clear();
    }

    /// Checks inlay hint settings for enabled hint kinds and general enabled state.
    /// Generates corresponding inlay_map splice updates on settings changes.
    /// Does not update inlay hint cache state on disabling or inlay hint kinds change: only reenabling forces new LSP queries.
    fn update_settings(
        &mut self,
        new_hint_settings: InlayHintSettings,
        visible_hints: Vec<Inlay>,
    ) -> ControlFlow<Option<InlaySplice>, Option<InlaySplice>> {
        let old_enabled = self.enabled;
        // If the setting for inlay hints has changed, update `enabled`. This condition avoids inlay
        // hint visibility changes when other settings change (such as theme).
        //
        // Another option might be to store whether the user has manually toggled inlay hint
        // visibility, and prefer this. This could lead to confusion as it means inlay hint
        // visibility would not change when updating the setting if they were ever toggled.
        if new_hint_settings.enabled != self.enabled_in_settings {
            self.enabled = new_hint_settings.enabled;
            self.enabled_in_settings = new_hint_settings.enabled;
            self.modifiers_override = false;
        };
        self.invalidate_debounce = debounce_value(new_hint_settings.edit_debounce_ms);
        self.append_debounce = debounce_value(new_hint_settings.scroll_debounce_ms);
        let new_allowed_hint_kinds = new_hint_settings.enabled_inlay_hint_kinds();
        match (old_enabled, self.enabled) {
            (false, false) => {
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                ControlFlow::Break(None)
            }
            (true, true) => {
                if new_allowed_hint_kinds == self.allowed_hint_kinds {
                    ControlFlow::Break(None)
                } else {
                    self.allowed_hint_kinds = new_allowed_hint_kinds;
                    ControlFlow::Continue(
                        Some(InlaySplice {
                            to_remove: visible_hints
                                .iter()
                                .filter_map(|inlay| {
                                    let inlay_kind = self.added_hints.get(&inlay.id).copied()?;
                                    if !self.allowed_hint_kinds.contains(&inlay_kind) {
                                        Some(inlay.id)
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            to_insert: Vec::new(),
                        })
                        .filter(|splice| !splice.is_empty()),
                    )
                }
            }
            (true, false) => {
                self.modifiers_override = false;
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                if visible_hints.is_empty() {
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
                self.modifiers_override = false;
                self.allowed_hint_kinds = new_allowed_hint_kinds;
                ControlFlow::Continue(
                    Some(InlaySplice {
                        to_remove: visible_hints
                            .iter()
                            .filter_map(|inlay| {
                                let inlay_kind = self.added_hints.get(&inlay.id).copied()?;
                                if !self.allowed_hint_kinds.contains(&inlay_kind) {
                                    Some(inlay.id)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                        to_insert: Vec::new(),
                    })
                    .filter(|splice| !splice.is_empty()),
                )
            }
        }
    }

    pub(crate) fn remove_inlay_chunk_data<'a>(
        &'a mut self,
        removed_buffer_ids: impl IntoIterator<Item = &'a BufferId> + 'a,
    ) {
        for buffer_id in removed_buffer_ids {
            self.hint_refresh_tasks.remove(buffer_id);
            self.hint_chunk_fetching.remove(buffer_id);
        }
    }
}

#[derive(Debug, Clone)]
pub enum InlayHintRefreshReason {
    ModifiersChanged(bool),
    Toggle(bool),
    SettingsChange(InlayHintSettings),
    NewLinesShown,
    BufferEdited(BufferId),
    RefreshRequested {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    ExcerptsRemoved(Vec<ExcerptId>),
}

impl Editor {
    pub fn supports_inlay_hints(&self, cx: &mut App) -> bool {
        let Some(provider) = self.semantics_provider.as_ref() else {
            return false;
        };

        let mut supports = false;
        self.buffer().update(cx, |this, cx| {
            this.for_each_buffer(|buffer| {
                supports |= provider.supports_inlay_hints(buffer, cx);
            });
        });

        supports
    }

    pub fn toggle_inline_values(
        &mut self,
        _: &ToggleInlineValues,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.inline_value_cache.enabled = !self.inline_value_cache.enabled;

        self.refresh_inline_values(cx);
    }

    pub fn toggle_inlay_hints(
        &mut self,
        _: &ToggleInlayHints,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.refresh_inlay_hints(
            InlayHintRefreshReason::Toggle(!self.inlay_hints_enabled()),
            cx,
        );
    }

    pub fn inlay_hints_enabled(&self) -> bool {
        self.inlay_hints.as_ref().is_some_and(|cache| cache.enabled)
    }

    /// Updates inlay hints for the visible ranges of the singleton buffer(s).
    /// Based on its parameters, either invalidates the previous data, or appends to it.
    pub(crate) fn refresh_inlay_hints(
        &mut self,
        reason: InlayHintRefreshReason,
        cx: &mut Context<Self>,
    ) {
        if self.ignore_lsp_data() || self.inlay_hints.is_none() {
            return;
        }
        let Some(semantics_provider) = self.semantics_provider() else {
            return;
        };
        let Some(invalidate_cache) = self.refresh_editor_data(&reason, cx) else {
            return;
        };

        let debounce = match &reason {
            InlayHintRefreshReason::SettingsChange(_)
            | InlayHintRefreshReason::Toggle(_)
            | InlayHintRefreshReason::ExcerptsRemoved(_)
            | InlayHintRefreshReason::ModifiersChanged(_) => None,
            _may_need_lsp_call => self.inlay_hints.as_ref().and_then(|inlay_hints| {
                if invalidate_cache.should_invalidate() {
                    inlay_hints.invalidate_debounce
                } else {
                    inlay_hints.append_debounce
                }
            }),
        };

        let mut visible_excerpts = self.visible_excerpts(cx);
        let mut invalidate_hints_for_buffers = HashSet::default();
        let ignore_previous_fetches = match reason {
            InlayHintRefreshReason::ModifiersChanged(_)
            | InlayHintRefreshReason::Toggle(_)
            | InlayHintRefreshReason::SettingsChange(_) => true,
            InlayHintRefreshReason::NewLinesShown
            | InlayHintRefreshReason::RefreshRequested { .. }
            | InlayHintRefreshReason::ExcerptsRemoved(_) => false,
            InlayHintRefreshReason::BufferEdited(buffer_id) => {
                let Some(affected_language) = self
                    .buffer()
                    .read(cx)
                    .buffer(buffer_id)
                    .and_then(|buffer| buffer.read(cx).language().cloned())
                else {
                    return;
                };

                invalidate_hints_for_buffers.extend(
                    self.buffer()
                        .read(cx)
                        .all_buffers()
                        .into_iter()
                        .filter_map(|buffer| {
                            let buffer = buffer.read(cx);
                            if buffer.language() == Some(&affected_language) {
                                Some(buffer.remote_id())
                            } else {
                                None
                            }
                        }),
                );

                semantics_provider.invalidate_inlay_hints(&invalidate_hints_for_buffers, cx);
                visible_excerpts.retain(|_, (visible_buffer, _, _)| {
                    visible_buffer.read(cx).language() == Some(&affected_language)
                });
                false
            }
        };

        let multi_buffer = self.buffer().clone();
        let Some(inlay_hints) = self.inlay_hints.as_mut() else {
            return;
        };

        if invalidate_cache.should_invalidate() {
            inlay_hints.clear();
        }
        inlay_hints
            .invalidate_hints_for_buffers
            .extend(invalidate_hints_for_buffers);

        let mut buffers_to_query = HashMap::default();
        for (_, (buffer, buffer_version, visible_range)) in visible_excerpts {
            let buffer_id = buffer.read(cx).remote_id();
            if !self.registered_buffers.contains_key(&buffer_id) {
                continue;
            }

            let buffer_snapshot = buffer.read(cx).snapshot();
            let buffer_anchor_range = buffer_snapshot.anchor_before(visible_range.start)
                ..buffer_snapshot.anchor_after(visible_range.end);

            let visible_excerpts =
                buffers_to_query
                    .entry(buffer_id)
                    .or_insert_with(|| VisibleExcerpts {
                        ranges: Vec::new(),
                        buffer_version: buffer_version.clone(),
                        buffer: buffer.clone(),
                    });
            visible_excerpts.buffer_version = buffer_version;
            visible_excerpts.ranges.push(buffer_anchor_range);
        }

        for (buffer_id, visible_excerpts) in buffers_to_query {
            let Some(buffer) = multi_buffer.read(cx).buffer(buffer_id) else {
                continue;
            };

            let (fetched_for_version, fetched_chunks) = inlay_hints
                .hint_chunk_fetching
                .entry(buffer_id)
                .or_default();
            if visible_excerpts
                .buffer_version
                .changed_since(fetched_for_version)
            {
                *fetched_for_version = visible_excerpts.buffer_version.clone();
                fetched_chunks.clear();
                inlay_hints.hint_refresh_tasks.remove(&buffer_id);
            }

            let known_chunks = if ignore_previous_fetches {
                None
            } else {
                Some((fetched_for_version.clone(), fetched_chunks.clone()))
            };

            let mut applicable_chunks =
                semantics_provider.applicable_inlay_chunks(&buffer, &visible_excerpts.ranges, cx);
            applicable_chunks.retain(|chunk| fetched_chunks.insert(chunk.clone()));
            if applicable_chunks.is_empty() && !ignore_previous_fetches {
                continue;
            }
            inlay_hints
                .hint_refresh_tasks
                .entry(buffer_id)
                .or_default()
                .push(spawn_editor_hints_refresh(
                    buffer_id,
                    invalidate_cache,
                    debounce,
                    visible_excerpts,
                    known_chunks,
                    applicable_chunks,
                    cx,
                ));
        }
    }

    pub fn clear_inlay_hints(&mut self, cx: &mut Context<Self>) {
        let to_remove = self
            .visible_inlay_hints(cx)
            .into_iter()
            .map(|inlay| {
                let inlay_id = inlay.id;
                if let Some(inlay_hints) = &mut self.inlay_hints {
                    inlay_hints.added_hints.remove(&inlay_id);
                }
                inlay_id
            })
            .collect::<Vec<_>>();
        self.splice_inlays(&to_remove, Vec::new(), cx);
    }

    fn refresh_editor_data(
        &mut self,
        reason: &InlayHintRefreshReason,
        cx: &mut Context<'_, Editor>,
    ) -> Option<InvalidationStrategy> {
        let visible_inlay_hints = self.visible_inlay_hints(cx);
        let Some(inlay_hints) = self.inlay_hints.as_mut() else {
            return None;
        };

        let invalidate_cache = match reason {
            InlayHintRefreshReason::ModifiersChanged(enabled) => {
                match inlay_hints.modifiers_override(*enabled) {
                    Some(enabled) => {
                        if enabled {
                            InvalidationStrategy::None
                        } else {
                            self.clear_inlay_hints(cx);
                            return None;
                        }
                    }
                    None => return None,
                }
            }
            InlayHintRefreshReason::Toggle(enabled) => {
                if inlay_hints.toggle(*enabled) {
                    if *enabled {
                        InvalidationStrategy::None
                    } else {
                        self.clear_inlay_hints(cx);
                        return None;
                    }
                } else {
                    return None;
                }
            }
            InlayHintRefreshReason::SettingsChange(new_settings) => {
                match inlay_hints.update_settings(*new_settings, visible_inlay_hints) {
                    ControlFlow::Break(Some(InlaySplice {
                        to_remove,
                        to_insert,
                    })) => {
                        self.splice_inlays(&to_remove, to_insert, cx);
                        return None;
                    }
                    ControlFlow::Break(None) => return None,
                    ControlFlow::Continue(splice) => {
                        if let Some(InlaySplice {
                            to_remove,
                            to_insert,
                        }) = splice
                        {
                            self.splice_inlays(&to_remove, to_insert, cx);
                        }
                        InvalidationStrategy::None
                    }
                }
            }
            InlayHintRefreshReason::ExcerptsRemoved(excerpts_removed) => {
                let to_remove = self
                    .display_map
                    .read(cx)
                    .current_inlays()
                    .filter_map(|inlay| {
                        if excerpts_removed.contains(&inlay.position.excerpt_id) {
                            Some(inlay.id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                self.splice_inlays(&to_remove, Vec::new(), cx);
                return None;
            }
            InlayHintRefreshReason::NewLinesShown => InvalidationStrategy::None,
            InlayHintRefreshReason::BufferEdited(_) => InvalidationStrategy::BufferEdited,
            InlayHintRefreshReason::RefreshRequested {
                server_id,
                request_id,
            } => InvalidationStrategy::RefreshRequested {
                server_id: *server_id,
                request_id: *request_id,
            },
        };

        match &mut self.inlay_hints {
            Some(inlay_hints) => {
                if !inlay_hints.enabled
                    && !matches!(reason, InlayHintRefreshReason::ModifiersChanged(_))
                {
                    return None;
                }
            }
            None => return None,
        }

        Some(invalidate_cache)
    }

    pub(crate) fn visible_inlay_hints(&self, cx: &Context<Editor>) -> Vec<Inlay> {
        self.display_map
            .read(cx)
            .current_inlays()
            .filter(move |inlay| matches!(inlay.id, InlayId::Hint(_)))
            .cloned()
            .collect()
    }

    pub fn update_inlay_link_and_hover_points(
        &mut self,
        snapshot: &EditorSnapshot,
        point_for_position: PointForPosition,
        secondary_held: bool,
        shift_held: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(lsp_store) = self.project().map(|project| project.read(cx).lsp_store()) else {
            return;
        };
        let hovered_offset = if point_for_position.column_overshoot_after_line_end == 0 {
            Some(
                snapshot
                    .display_point_to_inlay_offset(point_for_position.exact_unclipped, Bias::Left),
            )
        } else {
            None
        };
        let mut go_to_definition_updated = false;
        let mut hover_updated = false;
        if let Some(hovered_offset) = hovered_offset {
            let buffer_snapshot = self.buffer().read(cx).snapshot(cx);
            let previous_valid_anchor = buffer_snapshot.anchor_at(
                point_for_position.previous_valid.to_point(snapshot),
                Bias::Left,
            );
            let next_valid_anchor = buffer_snapshot.anchor_at(
                point_for_position.next_valid.to_point(snapshot),
                Bias::Right,
            );
            if let Some(hovered_hint) = self
                .visible_inlay_hints(cx)
                .into_iter()
                .skip_while(|hint| {
                    hint.position
                        .cmp(&previous_valid_anchor, &buffer_snapshot)
                        .is_lt()
                })
                .take_while(|hint| {
                    hint.position
                        .cmp(&next_valid_anchor, &buffer_snapshot)
                        .is_le()
                })
                .max_by_key(|hint| hint.id)
            {
                if let Some(ResolvedHint::Resolved(cached_hint)) =
                    hovered_hint.position.buffer_id.and_then(|buffer_id| {
                        lsp_store.update(cx, |lsp_store, cx| {
                            lsp_store.resolved_hint(buffer_id, hovered_hint.id, cx)
                        })
                    })
                {
                    match cached_hint.resolve_state {
                        ResolveState::Resolved => {
                            let mut extra_shift_left = 0;
                            let mut extra_shift_right = 0;
                            if cached_hint.padding_left {
                                extra_shift_left += 1;
                                extra_shift_right += 1;
                            }
                            if cached_hint.padding_right {
                                extra_shift_right += 1;
                            }
                            match cached_hint.label {
                                InlayHintLabel::String(_) => {
                                    if let Some(tooltip) = cached_hint.tooltip {
                                        hover_popover::hover_at_inlay(
                                            self,
                                            InlayHover {
                                                tooltip: match tooltip {
                                                    InlayHintTooltip::String(text) => HoverBlock {
                                                        text,
                                                        kind: HoverBlockKind::PlainText,
                                                    },
                                                    InlayHintTooltip::MarkupContent(content) => {
                                                        HoverBlock {
                                                            text: content.value,
                                                            kind: content.kind,
                                                        }
                                                    }
                                                },
                                                range: InlayHighlight {
                                                    inlay: hovered_hint.id,
                                                    inlay_position: hovered_hint.position,
                                                    range: extra_shift_left
                                                        ..hovered_hint.text().len()
                                                            + extra_shift_right,
                                                },
                                            },
                                            window,
                                            cx,
                                        );
                                        hover_updated = true;
                                    }
                                }
                                InlayHintLabel::LabelParts(label_parts) => {
                                    let hint_start =
                                        snapshot.anchor_to_inlay_offset(hovered_hint.position);
                                    if let Some((hovered_hint_part, part_range)) =
                                        hover_popover::find_hovered_hint_part(
                                            label_parts,
                                            hint_start,
                                            hovered_offset,
                                        )
                                    {
                                        let highlight_start =
                                            (part_range.start - hint_start).0 + extra_shift_left;
                                        let highlight_end =
                                            (part_range.end - hint_start).0 + extra_shift_right;
                                        let highlight = InlayHighlight {
                                            inlay: hovered_hint.id,
                                            inlay_position: hovered_hint.position,
                                            range: highlight_start..highlight_end,
                                        };
                                        if let Some(tooltip) = hovered_hint_part.tooltip {
                                            hover_popover::hover_at_inlay(
                                                self,
                                                InlayHover {
                                                    tooltip: match tooltip {
                                                        InlayHintLabelPartTooltip::String(text) => {
                                                            HoverBlock {
                                                                text,
                                                                kind: HoverBlockKind::PlainText,
                                                            }
                                                        }
                                                        InlayHintLabelPartTooltip::MarkupContent(
                                                            content,
                                                        ) => HoverBlock {
                                                            text: content.value,
                                                            kind: content.kind,
                                                        },
                                                    },
                                                    range: highlight.clone(),
                                                },
                                                window,
                                                cx,
                                            );
                                            hover_updated = true;
                                        }
                                        if let Some((language_server_id, location)) =
                                            hovered_hint_part.location
                                            && secondary_held
                                            && !self.has_pending_nonempty_selection()
                                        {
                                            go_to_definition_updated = true;
                                            show_link_definition(
                                                shift_held,
                                                self,
                                                TriggerPoint::InlayHint(
                                                    highlight,
                                                    location,
                                                    language_server_id,
                                                ),
                                                snapshot,
                                                window,
                                                cx,
                                            );
                                        }
                                    }
                                }
                            };
                        }
                        ResolveState::CanResolve(_, _) => debug_panic!(
                            "Expected resolved_hint retrieval to return a resolved hint"
                        ),
                        ResolveState::Resolving => {}
                    }
                }
            }
        }

        if !go_to_definition_updated {
            self.hide_hovered_link(cx)
        }
        if !hover_updated {
            hover_popover::hover_at(self, None, window, cx);
        }
    }

    fn inlay_hints_for_buffer(
        &mut self,
        invalidate_cache: InvalidationStrategy,
        buffer_excerpts: VisibleExcerpts,
        known_chunks: Option<(Global, HashSet<Range<BufferRow>>)>,
        cx: &mut Context<Self>,
    ) -> Option<Vec<Task<(Range<BufferRow>, anyhow::Result<CacheInlayHints>)>>> {
        let semantics_provider = self.semantics_provider()?;

        let new_hint_tasks = semantics_provider
            .inlay_hints(
                invalidate_cache,
                buffer_excerpts.buffer,
                buffer_excerpts.ranges,
                known_chunks,
                cx,
            )
            .unwrap_or_default();

        let mut hint_tasks = None;
        for (row_range, new_hints_task) in new_hint_tasks {
            hint_tasks
                .get_or_insert_with(Vec::new)
                .push(cx.spawn(async move |_, _| (row_range, new_hints_task.await)));
        }
        hint_tasks
    }

    fn apply_fetched_hints(
        &mut self,
        buffer_id: BufferId,
        query_version: Global,
        invalidate_cache: InvalidationStrategy,
        new_hints: Vec<(Range<BufferRow>, anyhow::Result<CacheInlayHints>)>,
        cx: &mut Context<Self>,
    ) {
        let visible_inlay_hint_ids = self
            .visible_inlay_hints(cx)
            .iter()
            .filter(|inlay| inlay.position.buffer_id == Some(buffer_id))
            .map(|inlay| inlay.id)
            .collect::<Vec<_>>();
        let Some(inlay_hints) = &mut self.inlay_hints else {
            return;
        };

        let mut hints_to_remove = Vec::new();
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);

        // If we've received hints from the cache, it means `invalidate_cache` had invalidated whatever possible there,
        // and most probably there are no more hints with IDs from `visible_inlay_hint_ids` in the cache.
        // So, if we hover such hints, no resolve will happen.
        //
        // Another issue is in the fact that changing one buffer may lead to other buffers' hints changing, so more cache entries may be removed.
        // Hence, clear all excerpts' hints in the multi buffer: later, the invalidated ones will re-trigger the LSP query, the rest will be restored
        // from the cache.
        if invalidate_cache.should_invalidate() {
            hints_to_remove.extend(visible_inlay_hint_ids);
        }

        let excerpts = self.buffer.read(cx).excerpt_ids();
        let mut inserted_hint_text = HashMap::default();
        let hints_to_insert = new_hints
            .into_iter()
            .filter_map(|(chunk_range, hints_result)| {
                let chunks_fetched = inlay_hints.hint_chunk_fetching.get_mut(&buffer_id);
                match hints_result {
                    Ok(new_hints) => {
                        if new_hints.is_empty() {
                            if let Some((_, chunks_fetched)) = chunks_fetched {
                                chunks_fetched.remove(&chunk_range);
                            }
                        }
                        Some(new_hints)
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to query inlays for buffer row range {chunk_range:?}, {e:#}"
                        );
                        if let Some((for_version, chunks_fetched)) = chunks_fetched {
                            if for_version == &query_version {
                                chunks_fetched.remove(&chunk_range);
                            }
                        }
                        None
                    }
                }
            })
            .flat_map(|new_hints| {
                let mut hints_deduplicated = Vec::new();

                if new_hints.len() > 1 {
                    for (server_id, new_hints) in new_hints {
                        for (new_id, new_hint) in new_hints {
                            let hints_text_for_position = inserted_hint_text
                                .entry(new_hint.position)
                                .or_insert_with(HashMap::default);
                            let insert =
                                match hints_text_for_position.entry(new_hint.text().to_string()) {
                                    hash_map::Entry::Occupied(o) => o.get() == &server_id,
                                    hash_map::Entry::Vacant(v) => {
                                        v.insert(server_id);
                                        true
                                    }
                                };

                            if insert {
                                hints_deduplicated.push((new_id, new_hint));
                            }
                        }
                    }
                } else {
                    hints_deduplicated.extend(new_hints.into_values().flatten());
                }

                hints_deduplicated
            })
            .filter_map(|(hint_id, lsp_hint)| {
                if inlay_hints.allowed_hint_kinds.contains(&lsp_hint.kind)
                    && inlay_hints
                        .added_hints
                        .insert(hint_id, lsp_hint.kind)
                        .is_none()
                {
                    let position = excerpts.iter().find_map(|excerpt_id| {
                        multi_buffer_snapshot.anchor_in_excerpt(*excerpt_id, lsp_hint.position)
                    })?;
                    return Some(Inlay::hint(hint_id, position, &lsp_hint));
                }
                None
            })
            .collect::<Vec<_>>();

        let invalidate_hints_for_buffers =
            std::mem::take(&mut inlay_hints.invalidate_hints_for_buffers);
        if !invalidate_hints_for_buffers.is_empty() {
            hints_to_remove.extend(
                self.visible_inlay_hints(cx)
                    .iter()
                    .filter(|inlay| {
                        inlay.position.buffer_id.is_none_or(|buffer_id| {
                            invalidate_hints_for_buffers.contains(&buffer_id)
                        })
                    })
                    .map(|inlay| inlay.id),
            );
        }

        self.splice_inlays(&hints_to_remove, hints_to_insert, cx);
    }
}

#[derive(Debug)]
struct VisibleExcerpts {
    ranges: Vec<Range<text::Anchor>>,
    buffer_version: Global,
    buffer: Entity<language::Buffer>,
}

fn spawn_editor_hints_refresh(
    buffer_id: BufferId,
    invalidate_cache: InvalidationStrategy,
    debounce: Option<Duration>,
    buffer_excerpts: VisibleExcerpts,
    known_chunks: Option<(Global, HashSet<Range<BufferRow>>)>,
    applicable_chunks: Vec<Range<BufferRow>>,
    cx: &mut Context<'_, Editor>,
) -> Task<()> {
    cx.spawn(async move |editor, cx| {
        if let Some(debounce) = debounce {
            cx.background_executor().timer(debounce).await;
        }

        let query_version = buffer_excerpts.buffer_version.clone();
        let Some(hint_tasks) = editor
            .update(cx, |editor, cx| {
                editor.inlay_hints_for_buffer(invalidate_cache, buffer_excerpts, known_chunks, cx)
            })
            .ok()
        else {
            return;
        };
        let hint_tasks = hint_tasks.unwrap_or_default();
        if hint_tasks.is_empty() {
            editor
                .update(cx, |editor, _| {
                    if let Some((_, hint_chunk_fetching)) = editor
                        .inlay_hints
                        .as_mut()
                        .and_then(|inlay_hints| inlay_hints.hint_chunk_fetching.get_mut(&buffer_id))
                    {
                        for applicable_chunks in &applicable_chunks {
                            hint_chunk_fetching.remove(applicable_chunks);
                        }
                    }
                })
                .ok();
            return;
        }
        let new_hints = join_all(hint_tasks).await;
        editor
            .update(cx, |editor, cx| {
                editor.apply_fetched_hints(
                    buffer_id,
                    query_version,
                    invalidate_cache,
                    new_hints,
                    cx,
                );
            })
            .ok();
    })
}

#[cfg(test)]
pub mod tests {
    use crate::editor_tests::update_test_language_settings;
    use crate::inlays::inlay_hints::InlayHintRefreshReason;
    use crate::scroll::ScrollAmount;
    use crate::{Editor, SelectionEffects};
    use crate::{ExcerptRange, scroll::Autoscroll};
    use collections::HashSet;
    use futures::{StreamExt, future};
    use gpui::{AppContext as _, Context, SemanticVersion, TestAppContext, WindowHandle};
    use itertools::Itertools as _;
    use language::language_settings::InlayHintKind;
    use language::{Capability, FakeLspAdapter};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use languages::rust_lang;
    use lsp::FakeLanguageServer;
    use multi_buffer::MultiBuffer;
    use parking_lot::Mutex;
    use pretty_assertions::assert_eq;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::{AllLanguageSettingsContent, InlayHintSettingsContent, SettingsStore};
    use std::ops::Range;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
    use std::time::Duration;
    use text::{OffsetRangeExt, Point};
    use ui::App;
    use util::path;
    use util::paths::natural_sort;

    #[gpui::test]
    async fn test_basic_cache_update_with_duplicate_hints(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(allowed_hint_kinds.contains(&Some(InlayHintKind::Type))),
                show_parameter_hints: Some(
                    allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        let i = task_lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
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
                },
            );
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([13..13])
                });
                editor.handle_input("some change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get new hints after an edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["3".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get new hints after hint refresh/ request"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_racy_cache_updates(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });
        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        let i = task_lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );
                        Ok(Some(vec![lsp::InlayHint {
                            position: lsp::Position::new(0, i),
                            label: lsp::InlayHintLabel::String(i.to_string()),
                            kind: Some(lsp::InlayHintKind::TYPE),
                            text_edits: None,
                            tooltip: None,
                            padding_left: None,
                            padding_right: None,
                            data: None,
                        }]))
                    }
                },
            );
        })
        .await;
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        // Emulate simultaneous events: both editing, refresh and, slightly after, scroll updates are triggered.
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("foo", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(5));
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(
                    InlayHintRefreshReason::RefreshRequested {
                        server_id: fake_server.server.server_id(),
                        request_id: Some(1),
                    },
                    cx,
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(5));
        editor
            .update(cx, |editor, _window, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::NewLinesShown, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(expected_hints, cached_hint_labels(editor, cx), "Despite multiple simultaneous refreshes, only one inlay hint query should be issued");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_cache_update_on_lsp_completion_tasks(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let task_lsp_request_count = Arc::clone(&lsp_request_count);
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
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
                },
            );
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        let progress_token = 42;
        fake_server
            .request::<lsp::request::WorkDoneProgressCreate>(lsp::WorkDoneProgressCreateParams {
                token: lsp::ProgressToken::Number(progress_token),
            })
            .await
            .into_response()
            .expect("work done progress create request failed");
        cx.executor().run_until_parked();
        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::Number(progress_token),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Begin(
                lsp::WorkDoneProgressBegin::default(),
            )),
        });
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["0".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should not update hints while the work task is running"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        fake_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: lsp::ProgressToken::Number(progress_token),
            value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::End(
                lsp::WorkDoneProgressEnd::default(),
            )),
        });
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "New hints should be queried after the work task is done"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_no_hint_updates_for_unrelated_language_files(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                "other.md": "Test md file with some text",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

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
                Some(tree_sitter_rust::LANGUAGE.into()),
            )));
            let fake_servers = language_registry.register_fake_lsp(
                name,
                FakeLspAdapter {
                    name,
                    capabilities: lsp::ServerCapabilities {
                        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                        ..Default::default()
                    },
                    initializer: Some(Box::new({
                        move |fake_server| {
                            let rs_lsp_request_count = Arc::new(AtomicU32::new(0));
                            let md_lsp_request_count = Arc::new(AtomicU32::new(0));
                            fake_server
                                .set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                                    move |params, _| {
                                        let i = match name {
                                            "Rust" => {
                                                assert_eq!(
                                                    params.text_document.uri,
                                                    lsp::Uri::from_file_path(path!("/a/main.rs"))
                                                        .unwrap(),
                                                );
                                                rs_lsp_request_count.fetch_add(1, Ordering::Release)
                                                    + 1
                                            }
                                            "Markdown" => {
                                                assert_eq!(
                                                    params.text_document.uri,
                                                    lsp::Uri::from_file_path(path!("/a/other.md"))
                                                        .unwrap(),
                                                );
                                                md_lsp_request_count.fetch_add(1, Ordering::Release)
                                                    + 1
                                            }
                                            unexpected => {
                                                panic!("Unexpected language: {unexpected}")
                                            }
                                        };

                                        async move {
                                            let query_start = params.range.start;
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
                                    },
                                );
                        }
                    })),
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
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let rs_editor = cx.add_window(|window, cx| {
            Editor::for_buffer(rs_buffer, Some(project.clone()), window, cx)
        });
        cx.executor().run_until_parked();

        let _rs_fake_server = rs_fake_servers.unwrap().next().await.unwrap();
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        cx.executor().run_until_parked();
        let md_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/other.md"), cx)
            })
            .await
            .unwrap();
        let md_editor =
            cx.add_window(|window, cx| Editor::for_buffer(md_buffer, Some(project), window, cx));
        cx.executor().run_until_parked();

        let _md_fake_server = md_fake_servers.unwrap().next().await.unwrap();
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should have a separate version, repeating Rust editor rules"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        rs_editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([13..13])
                });
                editor.handle_input("some rs change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Rust inlay cache should change after the edit"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should not be affected by Rust editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        md_editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([13..13])
                });
                editor.handle_input("some md change", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        md_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Rust editor should not be affected by Markdown editor changes"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        rs_editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec!["2".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Markdown editor should also change independently"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_setting_changes(cx: &mut gpui::TestAppContext) {
        let allowed_hint_kinds = HashSet::from_iter([None, Some(InlayHintKind::Type)]);
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(allowed_hint_kinds.contains(&Some(InlayHintKind::Type))),
                show_parameter_hints: Some(
                    allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let lsp_request_count = Arc::new(AtomicUsize::new(0));
        let (_, editor, fake_server) = prepare_test_objects(cx, {
            let lsp_request_count = lsp_request_count.clone();
            move |fake_server, file_with_hints| {
                let lsp_request_count = lsp_request_count.clone();
                fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                    move |params, _| {
                        lsp_request_count.fetch_add(1, Ordering::Release);
                        async move {
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(file_with_hints).unwrap(),
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
                                    label: lsp::InlayHintLabel::String(
                                        "parameter hint".to_string(),
                                    ),
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
                    },
                );
            }
        })
        .await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    1,
                    "Should query new hints once"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its first hints when opening the editor"
                );
                assert_eq!(
                    vec!["type hint".to_string(), "other hint".to_string()],
                    visible_hint_labels(editor, cx)
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should load new hints twice"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Cached hints should not change due to allowed hint kinds settings update"
                );
                assert_eq!(
                    vec!["type hint".to_string(), "other hint".to_string()],
                    visible_hint_labels(editor, cx)
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
                vec!["type hint".to_string(), "other hint".to_string()],
            ),
            (
                HashSet::from_iter([None, Some(InlayHintKind::Parameter)]),
                vec!["parameter hint".to_string(), "other hint".to_string()],
            ),
            (
                HashSet::from_iter([Some(InlayHintKind::Type), Some(InlayHintKind::Parameter)]),
                vec!["type hint".to_string(), "parameter hint".to_string()],
            ),
            (
                HashSet::from_iter([
                    None,
                    Some(InlayHintKind::Type),
                    Some(InlayHintKind::Parameter),
                ]),
                vec![
                    "type hint".to_string(),
                    "parameter hint".to_string(),
                    "other hint".to_string(),
                ],
            ),
        ] {
            update_test_language_settings(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                    show_value_hints: Some(true),
                    enabled: Some(true),
                    edit_debounce_ms: Some(0),
                    scroll_debounce_ms: Some(0),
                    show_type_hints: Some(
                        new_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                    ),
                    show_parameter_hints: Some(
                        new_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                    ),
                    show_other_hints: Some(new_allowed_hint_kinds.contains(&None)),
                    show_background: Some(false),
                    toggle_on_modifiers_press: None,
                })
            });
            cx.executor().run_until_parked();
            editor.update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints on allowed hint kinds change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its cached hints unchanged after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    expected_visible_hints,
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints filtered after the settings change for hint kinds {new_allowed_hint_kinds:?}"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    new_allowed_hint_kinds,
                    "Cache should use editor settings to get the allowed hint kinds for hint kinds {new_allowed_hint_kinds:?}"
                );
            }).unwrap();
        }

        let another_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Type)]);
        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(false),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(
                    another_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                ),
                show_parameter_hints: Some(
                    another_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(another_allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints when hints got disabled"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should not clear the cache when hints got disabled"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear visible hints when hints got disabled"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    another_allowed_hint_kinds,
                    "Should update its allowed hint kinds even when hints got disabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not load new hints when they got disabled"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx)
                );
                assert_eq!(Vec::<String>::new(), visible_hint_labels(editor, cx));
            })
            .unwrap();

        let final_allowed_hint_kinds = HashSet::from_iter([Some(InlayHintKind::Parameter)]);
        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(
                    final_allowed_hint_kinds.contains(&Some(InlayHintKind::Type)),
                ),
                show_parameter_hints: Some(
                    final_allowed_hint_kinds.contains(&Some(InlayHintKind::Parameter)),
                ),
                show_other_hints: Some(final_allowed_hint_kinds.contains(&None)),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    2,
                    "Should not query for new hints when they got re-enabled, as the file version did not change"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Should get its cached hints fully repopulated after the hints got re-enabled"
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                    "Should get its visible hints repopulated and filtered after the h"
                );
                assert_eq!(
                    allowed_hint_kinds_for_editor(editor),
                    final_allowed_hint_kinds,
                    "Cache should update editor settings when hints got re-enabled"
                );
            })
            .unwrap();

        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>(())
            .await
            .into_response()
            .expect("inlay refresh request failed");
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    lsp_request_count.load(Ordering::Relaxed),
                    3,
                    "Should query for new hints again"
                );
                assert_eq!(
                    vec![
                        "type hint".to_string(),
                        "parameter hint".to_string(),
                        "other hint".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                );
                assert_eq!(
                    vec!["parameter hint".to_string()],
                    visible_hint_labels(editor, cx),
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_hint_request_cancellation(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let lsp_request_count = Arc::new(AtomicU32::new(0));
        let (_, editor, _) = prepare_test_objects(cx, {
            let lsp_request_count = lsp_request_count.clone();
            move |fake_server, file_with_hints| {
                let lsp_request_count = lsp_request_count.clone();
                fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                    move |params, _| {
                        let lsp_request_count = lsp_request_count.clone();
                        async move {
                            let i = lsp_request_count.fetch_add(1, Ordering::SeqCst) + 1;
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(file_with_hints).unwrap(),
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
                    },
                );
            }
        })
        .await;

        let mut expected_changes = Vec::new();
        for change_after_opening in [
            "initial change #1",
            "initial change #2",
            "initial change #3",
        ] {
            editor
                .update(cx, |editor, window, cx| {
                    editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                        s.select_ranges([13..13])
                    });
                    editor.handle_input(change_after_opening, window, cx);
                })
                .unwrap();
            expected_changes.push(change_after_opening);
        }

        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
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
                    cached_hint_labels(editor, cx),
                    "Should get hints from the last edit landed only"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

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
                    .update(&mut cx, |editor, window, cx| {
                        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                            s.select_ranges([13..13])
                        });
                        editor.handle_input(async_later_change, window, cx);
                    })
                    .unwrap();
            }));
        }
        let _ = future::join_all(edits).await;
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _, cx| {
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
                    cached_hint_labels(editor, cx),
                    "Should get hints from the last edit landed only"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test(iterations = 4)]
    async fn test_large_buffer_inlay_requests_split(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", "let i = 5;\n".repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());

        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));
        let lsp_request_count = Arc::new(AtomicUsize::new(0));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new({
                    let lsp_request_ranges = lsp_request_ranges.clone();
                    let lsp_request_count = lsp_request_count.clone();
                    move |fake_server| {
                        let closure_lsp_request_ranges = Arc::clone(&lsp_request_ranges);
                        let closure_lsp_request_count = Arc::clone(&lsp_request_count);
                        fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                            move |params, _| {
                                let task_lsp_request_ranges =
                                    Arc::clone(&closure_lsp_request_ranges);
                                let task_lsp_request_count = Arc::clone(&closure_lsp_request_count);
                                async move {
                                    assert_eq!(
                                        params.text_document.uri,
                                        lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                    );

                                    task_lsp_request_ranges.lock().push(params.range);
                                    task_lsp_request_count.fetch_add(1, Ordering::Release);
                                    Ok(Some(vec![lsp::InlayHint {
                                        position: params.range.start,
                                        label: lsp::InlayHintLabel::String(
                                            params.range.end.line.to_string(),
                                        ),
                                        kind: None,
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    }]))
                                }
                            },
                        );
                    }
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));
        cx.executor().run_until_parked();
        let _fake_server = fake_servers.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        let ranges = lsp_request_ranges
            .lock()
            .drain(..)
            .sorted_by_key(|r| r.start)
            .collect::<Vec<_>>();
        assert_eq!(
            ranges.len(),
            1,
            "Should query 1 range initially, but got: {ranges:?}"
        );

        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        // Wait for the first hints request to fire off
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        let visible_range_after_scrolls = editor_visible_range(&editor, cx);
        let visible_line_count = editor
            .update(cx, |editor, _window, _| {
                editor.visible_line_count().unwrap()
            })
            .unwrap();
        let selection_in_cached_range = editor
            .update(cx, |editor, _window, cx| {
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
                    first_scroll.end.line, second_scroll.start.line,
                    "Should query 2 adjacent ranges after the scrolls, but got: {ranges:?}"
                );

                let lsp_requests = lsp_request_count.load(Ordering::Acquire);
                assert_eq!(
                    lsp_requests, 3,
                    "Should query hints initially, and after each scroll (2 times)"
                );
                assert_eq!(
                    vec!["50".to_string(), "100".to_string(), "150".to_string()],
                    cached_hint_labels(editor, cx),
                    "Chunks of 50 line width should have been queried each time"
                );
                assert_eq!(
                    vec!["50".to_string(), "100".to_string(), "150".to_string()],
                    visible_hint_labels(editor, cx),
                    "Editor should show only hints that it's scrolled to"
                );

                let mut selection_in_cached_range = visible_range_after_scrolls.end;
                selection_in_cached_range.row -= visible_line_count.ceil() as u32;
                selection_in_cached_range
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::center()),
                    window,
                    cx,
                    |s| s.select_ranges([selection_in_cached_range..selection_in_cached_range]),
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor.update(cx, |_, _, _| {
            let ranges = lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|r| r.start)
                .collect::<Vec<_>>();
            assert!(ranges.is_empty(), "No new ranges or LSP queries should be made after returning to the selection with cached hints");
            assert_eq!(lsp_request_count.load(Ordering::Acquire), 3, "No new requests should be made when selecting within cached chunks");
        }).unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("++++more text++++", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _window, cx| {
            let mut ranges = lsp_request_ranges.lock().drain(..).collect::<Vec<_>>();
            ranges.sort_by_key(|r| r.start);

            assert_eq!(ranges.len(), 2,
                "On edit, should scroll to selection and query a range around it: that range should split into 2 50 rows wide chunks. Instead, got query ranges {ranges:?}");
            let first_chunk = &ranges[0];
            let second_chunk = &ranges[1];
            assert!(first_chunk.end.line == second_chunk.start.line,
                "First chunk {first_chunk:?} should be before second chunk {second_chunk:?}");
            assert!(first_chunk.start.line < selection_in_cached_range.row,
                "Hints should be queried with the selected range after the query range start");

            let lsp_requests = lsp_request_count.load(Ordering::Acquire);
            assert_eq!(lsp_requests, 5, "Two chunks should be re-queried");
            assert_eq!(vec!["100".to_string(), "150".to_string()], cached_hint_labels(editor, cx),
                "Should have (less) hints from the new LSP response after the edit");
            assert_eq!(vec!["100".to_string(), "150".to_string()], visible_hint_labels(editor, cx), "Should show only visible hints (in the center) from the new cached set");
        }).unwrap();
    }

    fn editor_visible_range(
        editor: &WindowHandle<Editor>,
        cx: &mut gpui::TestAppContext,
    ) -> Range<Point> {
        let ranges = editor
            .update(cx, |editor, _window, cx| editor.visible_excerpts(cx))
            .unwrap();
        assert_eq!(
            ranges.len(),
            1,
            "Single buffer should produce a single excerpt with visible range"
        );
        let (_, (excerpt_buffer, _, excerpt_visible_range)) = ranges.into_iter().next().unwrap();
        excerpt_buffer.read_with(cx, |buffer, _| {
            excerpt_visible_range.to_point(&buffer.snapshot())
        })
    }

    #[gpui::test]
    async fn test_multiple_excerpts_large_multibuffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
                path!("/a"),
                json!({
                    "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<Vec<_>>().join("")),
                    "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<Vec<_>>().join("")),
                }),
            )
            .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0)),
                    ExcerptRange::new(Point::new(4, 0)..Point::new(11, 0)),
                    ExcerptRange::new(Point::new(22, 0)..Point::new(33, 0)),
                    ExcerptRange::new(Point::new(44, 0)..Point::new(55, 0)),
                    ExcerptRange::new(Point::new(56, 0)..Point::new(66, 0)),
                    ExcerptRange::new(Point::new(67, 0)..Point::new(77, 0)),
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [
                    ExcerptRange::new(Point::new(0, 1)..Point::new(2, 1)),
                    ExcerptRange::new(Point::new(4, 1)..Point::new(11, 1)),
                    ExcerptRange::new(Point::new(22, 1)..Point::new(33, 1)),
                    ExcerptRange::new(Point::new(44, 1)..Point::new(55, 1)),
                    ExcerptRange::new(Point::new(56, 1)..Point::new(66, 1)),
                    ExcerptRange::new(Point::new(67, 1)..Point::new(77, 1)),
                ],
                cx,
            );
            multibuffer
        });

        cx.executor().run_until_parked();
        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)
        });

        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap()
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
                                        "{hint_text}{E} #{i}",
                                        E = if edited { "(edited)" } else { "" },
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
            .update(cx, |editor, _window, cx| {
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
                    sorted_cached_hint_labels(editor, cx),
                    "When scroll is at the edge of a multibuffer, its visible excerpts only should be queried for inlay hints"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(4, 0)..Point::new(4, 0)]),
                );
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(22, 0)..Point::new(22, 0)]),
                );
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(57, 0)..Point::new(57, 0)]),
                );
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint #0".to_string(),
                    "main hint #1".to_string(),
                    "main hint #2".to_string(),
                    "main hint #3".to_string(),
                    "main hint #4".to_string(),
                    "main hint #5".to_string(),
                ];
                assert_eq!(expected_hints, sorted_cached_hint_labels(editor, cx),
                    "New hints are not shown right after scrolling, we need to wait for the buffer to be registered");
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
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
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After scrolling to the new buffer and waiting for it to be registered, new hints should appear");
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "Editor should show only visible hints",
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(100, 0)..Point::new(100, 0)]),
                );
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
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
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, all hints for all excerpts should be fetched"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "Editor shows only hints for excerpts that were visible when scrolling"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::Next),
                    window,
                    cx,
                    |s| s.select_ranges([Point::new(4, 0)..Point::new(4, 0)]),
                );
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
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
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, further scrolls up should not bring more hints"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                );
            })
            .unwrap();

        // We prepare to change the scrolling on edit, but do not scroll yet
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([Point::new(57, 0)..Point::new(57, 0)])
                });
            })
            .unwrap();
        cx.executor().run_until_parked();
        // Edit triggers the scrolling too
        editor_edited.store(true, Ordering::Release);
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("++++more text++++", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        // Wait again to trigger the inlay hints fetch on scroll
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "main hint(edited) #0".to_string(),
                    "main hint(edited) #1".to_string(),
                    "main hint(edited) #2".to_string(),
                    "main hint(edited) #3".to_string(),
                    "main hint(edited) #4".to_string(),
                    "main hint(edited) #5".to_string(),
                    "other hint(edited) #0".to_string(),
                    "other hint(edited) #1".to_string(),
                    "other hint(edited) #2".to_string(),
                    "other hint(edited) #3".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer edit, editor gets scrolled back to the last selection; \
                all hints should be invalidated and required for all of its visible excerpts"
                );
                assert_eq!(
                    expected_hints,
                    visible_hint_labels(editor, cx),
                    "All excerpts should get their hints"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_editing_in_multi_buffer(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", (0..200).map(|i| format!("let i = {i};\n")).collect::<Vec<_>>().join("")),
                "lib.rs": r#"let a = 1;
let b = 2;
let c = 3;"#
            }),
        )
        .await;

        let lsp_request_ranges = Arc::new(Mutex::new(Vec::new()));

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);

        let closure_ranges_fetched = lsp_request_ranges.clone();
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let closure_ranges_fetched = closure_ranges_fetched.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let closure_ranges_fetched = closure_ranges_fetched.clone();
                            async move {
                                let prefix = if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                                {
                                    closure_ranges_fetched
                                        .lock()
                                        .push(("main.rs", params.range));
                                    "main.rs"
                                } else if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                                {
                                    closure_ranges_fetched.lock().push(("lib.rs", params.range));
                                    "lib.rs"
                                } else {
                                    panic!("Unexpected file path {:?}", params.text_document.uri);
                                };
                                Ok(Some(
                                    (params.range.start.line..params.range.end.line)
                                        .map(|row| lsp::InlayHint {
                                            position: lsp::Position::new(row, 0),
                                            label: lsp::InlayHintLabel::String(format!(
                                                "{prefix} Inlay hint #{row}"
                                            )),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        })
                                        .collect(),
                                ))
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle_1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle_2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/lib.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    // Have first excerpt to spawn over 2 chunks (50 lines each).
                    ExcerptRange::new(Point::new(49, 0)..Point::new(53, 0)),
                    // Have 2nd excerpt to be in the 2nd chunk only.
                    ExcerptRange::new(Point::new(70, 0)..Point::new(73, 0)),
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(4, 0))],
                cx,
            );
            multibuffer
        });

        let editor = cx.add_window(|window, cx| {
            let mut editor =
                Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx);
            editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                s.select_ranges([0..0])
            });
            editor
        });

        let _fake_server = fake_servers.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        assert_eq!(
            vec![
                (
                    "lib.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(2, 10))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(50, 0))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(50, 0), lsp::Position::new(100, 0))
                ),
            ],
            lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|(prefix, r)| (prefix.to_owned(), r.start))
                .collect::<Vec<_>>(),
            "For large buffers, should query chunks that cover both visible excerpt"
        );
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    (0..2)
                        .map(|i| format!("lib.rs Inlay hint #{i}"))
                        .chain((0..100).map(|i| format!("main.rs Inlay hint #{i}")))
                        .collect::<Vec<_>>(),
                    sorted_cached_hint_labels(editor, cx),
                    "Both chunks should provide their inlay hints"
                );
                assert_eq!(
                    vec![
                        "main.rs Inlay hint #49".to_owned(),
                        "main.rs Inlay hint #50".to_owned(),
                        "main.rs Inlay hint #51".to_owned(),
                        "main.rs Inlay hint #52".to_owned(),
                        "main.rs Inlay hint #53".to_owned(),
                        "main.rs Inlay hint #70".to_owned(),
                        "main.rs Inlay hint #71".to_owned(),
                        "main.rs Inlay hint #72".to_owned(),
                        "main.rs Inlay hint #73".to_owned(),
                        "lib.rs Inlay hint #0".to_owned(),
                        "lib.rs Inlay hint #1".to_owned(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Only hints from visible excerpt should be added into the editor"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("a", window, cx);
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(1000));
        cx.executor().run_until_parked();
        assert_eq!(
            vec![
                (
                    "lib.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(2, 10))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(50, 0))
                ),
                (
                    "main.rs",
                    lsp::Range::new(lsp::Position::new(50, 0), lsp::Position::new(100, 0))
                ),
            ],
            lsp_request_ranges
                .lock()
                .drain(..)
                .sorted_by_key(|(prefix, r)| (prefix.to_owned(), r.start))
                .collect::<Vec<_>>(),
            "Same chunks should be re-queried on edit"
        );
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    (0..2)
                        .map(|i| format!("lib.rs Inlay hint #{i}"))
                        .chain((0..100).map(|i| format!("main.rs Inlay hint #{i}")))
                        .collect::<Vec<_>>(),
                    sorted_cached_hint_labels(editor, cx),
                    "Same hints should be re-inserted after the edit"
                );
                assert_eq!(
                    vec![
                        "main.rs Inlay hint #49".to_owned(),
                        "main.rs Inlay hint #50".to_owned(),
                        "main.rs Inlay hint #51".to_owned(),
                        "main.rs Inlay hint #52".to_owned(),
                        "main.rs Inlay hint #53".to_owned(),
                        "main.rs Inlay hint #70".to_owned(),
                        "main.rs Inlay hint #71".to_owned(),
                        "main.rs Inlay hint #72".to_owned(),
                        "main.rs Inlay hint #73".to_owned(),
                        "lib.rs Inlay hint #0".to_owned(),
                        "lib.rs Inlay hint #1".to_owned(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Same hints should be re-inserted into the editor after the edit"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_excerpts_removed(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(false),
                show_parameter_hints: Some(false),
                show_other_hints: Some(false),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|i| format!("let i = {i};\n")).collect::<Vec<_>>().join("")),
                "other.rs": format!("fn main() {{\n{}\n}}", (0..501).map(|j| format!("let j = {j};\n")).collect::<Vec<_>>().join("")),
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/other.rs"), cx)
            })
            .await
            .unwrap();
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let (buffer_1_excerpts, buffer_2_excerpts) = multibuffer.update(cx, |multibuffer, cx| {
            let buffer_1_excerpts = multibuffer.push_excerpts(
                buffer_1.clone(),
                [ExcerptRange::new(Point::new(0, 0)..Point::new(2, 0))],
                cx,
            );
            let buffer_2_excerpts = multibuffer.push_excerpts(
                buffer_2.clone(),
                [ExcerptRange::new(Point::new(0, 1)..Point::new(2, 1))],
                cx,
            );
            (buffer_1_excerpts, buffer_2_excerpts)
        });

        assert!(!buffer_1_excerpts.is_empty());
        assert!(!buffer_2_excerpts.is_empty());

        cx.executor().run_until_parked();
        let editor = cx.add_window(|window, cx| {
            Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)
        });
        let editor_edited = Arc::new(AtomicBool::new(false));
        let fake_server = fake_servers.next().await.unwrap();
        let closure_editor_edited = Arc::clone(&editor_edited);
        fake_server
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_editor_edited = Arc::clone(&closure_editor_edited);
                async move {
                    let hint_text = if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                    {
                        "main hint"
                    } else if params.text_document.uri
                        == lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap()
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
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                        "other hint #0".to_string(),
                        "other hint #1".to_string(),
                        "other hint #2".to_string(),
                        "other hint #3".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "Cache should update for both excerpts despite hints display was disabled; after selecting 2nd buffer, it's now registered with the langserever and should get its hints"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "All hints are disabled and should not be shown despite being present in the cache"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.buffer().update(cx, |multibuffer, cx| {
                    multibuffer.remove_excerpts(buffer_2_excerpts, cx)
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "For the removed excerpt, should clean corresponding cached hints as its buffer was dropped"
                );
                assert!(
                visible_hint_labels(editor, cx).is_empty(),
                "All hints are disabled and should not be shown despite being present in the cache"
            );
            })
            .unwrap();

        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                    ],
                    cached_hint_labels(editor, cx),
                    "Hint display settings change should not change the cache"
                );
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "Settings change should make cached hints visible, but only the visible ones, from the remaining excerpt"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_inside_char_boundary_range_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": format!(r#"fn main() {{\n{}\n}}"#, format!("let i = {};\n", "√".repeat(10)).repeat(500)),
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let lsp_request_count = Arc::new(AtomicU32::new(0));
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let i = lsp_request_count.fetch_add(1, Ordering::Release) + 1;
                            async move {
                                assert_eq!(
                                    params.text_document.uri,
                                    lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                );
                                let query_start = params.range.start;
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
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(expected_hints, cached_hint_labels(editor, cx));
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_toggle_inlay_hints(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(false),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, _fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let lsp_request_count = lsp_request_count.clone();
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );

                        let i = lsp_request_count.fetch_add(1, Ordering::AcqRel) + 1;
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
                },
            );
        })
        .await;

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should display inlays after toggle despite them disabled in settings"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Cache does not change because of toggles in the editor"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear hints after 2nd toggle"
                );
            })
            .unwrap();

        update_test_language_settings(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                let expected_hints = vec!["1".to_string()];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Should not query LSP hints after enabling hints in settings, as file version is the same"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Cache does not change because of toggles in the editor"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Should clear hints after enabling in settings and a 3rd toggle"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _, cx| {
            let expected_hints = vec!["1".to_string()];
            assert_eq!(
                expected_hints,
                cached_hint_labels(editor,cx),
                "Should not query LSP hints after enabling hints in settings and toggling them back on"
            );
            assert_eq!(expected_hints, visible_hint_labels(editor, cx));
        }).unwrap();
    }

    #[gpui::test]
    async fn test_modifiers_change(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let (_, editor, _fake_server) = prepare_test_objects(cx, |fake_server, file_with_hints| {
            let lsp_request_count = Arc::new(AtomicU32::new(0));
            fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                move |params, _| {
                    let lsp_request_count = lsp_request_count.clone();
                    async move {
                        assert_eq!(
                            params.text_document.uri,
                            lsp::Uri::from_file_path(file_with_hints).unwrap(),
                        );

                        let i = lsp_request_count.fetch_add(1, Ordering::AcqRel) + 1;
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
                },
            );
        })
        .await;

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Should display inlays after toggle despite them disabled in settings"
                );
                assert_eq!(vec!["1".to_string()], visible_hint_labels(editor, cx));
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "On modifiers change and hints toggled on, should hide editor inlays"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "When modifiers change is off, no extra requests are sent"
                );
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "When modifiers change is off, hints are back into the editor"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind (2)"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, window, cx| {
                editor.toggle_inlay_hints(&crate::ToggleInlayHints, window, cx)
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "When toggled off, should hide editor inlays"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "Nothing happens with the cache on modifiers change"
                );
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "On modifiers change & hints toggled off, should show editor inlays"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(true), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    vec!["1".to_string()],
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind"
                );
            })
            .unwrap();

        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(
                    vec!["1".to_string()],
                    cached_hint_labels(editor, cx),
                    "When modifiers change is off, no extra requests are sent"
                );
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "When modifiers change is off, editor hints are back into their toggled off state"
                );
            })
            .unwrap();
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(InlayHintRefreshReason::ModifiersChanged(false), cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _, cx| {
                assert_eq!(vec!["1".to_string()], cached_hint_labels(editor, cx));
                assert_eq!(
                    Vec::<String>::new(),
                    visible_hint_labels(editor, cx),
                    "Nothing changes on consequent modifiers change of the same kind (3)"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_inlays_at_the_same_place(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                show_value_hints: Some(true),
                enabled: Some(true),
                edit_debounce_ms: Some(0),
                scroll_debounce_ms: Some(0),
                show_type_hints: Some(true),
                show_parameter_hints: Some(true),
                show_other_hints: Some(true),
                show_background: Some(false),
                toggle_on_modifiers_press: None,
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() {
                    let x = 42;
                    std::thread::scope(|s| {
                        s.spawn(|| {
                            let _x = x;
                        });
                    });
                }",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..Default::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| async move {
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                            );
                            Ok(Some(
                                serde_json::from_value(json!([
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": "move",
                                        "paddingLeft": false,
                                        "paddingRight": false
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": "(",
                                        "paddingLeft": false,
                                        "paddingRight": false
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": [
                                            {
                                                "value": "&x"
                                            }
                                        ],
                                        "paddingLeft": false,
                                        "paddingRight": false,
                                        "data": {
                                            "file_id": 0
                                        }
                                    },
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": ")",
                                        "paddingLeft": false,
                                        "paddingRight": true
                                    },
                                    // not a correct syntax, but checks that same symbols at the same place
                                    // are not deduplicated
                                    {
                                        "position": {
                                            "line": 3,
                                            "character": 16
                                        },
                                        "label": ")",
                                        "paddingLeft": false,
                                        "paddingRight": true
                                    },
                                ]))
                                .unwrap(),
                            ))
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([Point::new(10, 0)..Point::new(10, 0)])
                })
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                let expected_hints = vec![
                    "move".to_string(),
                    "(".to_string(),
                    "&x".to_string(),
                    ") ".to_string(),
                    ") ".to_string(),
                ];
                assert_eq!(
                    expected_hints,
                    cached_hint_labels(editor, cx),
                    "Editor inlay hints should repeat server's order when placed at the same spot"
                );
                assert_eq!(expected_hints, visible_hint_labels(editor, cx));
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_invalidation_and_addition_race(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettingsContent {
                enabled: Some(true),
                ..InlayHintSettingsContent::default()
            })
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": r#"fn main() {
                    let x = 1;
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    ////
                    let x = "2";
                }
"#,
                "lib.rs": r#"fn aaa() {
                    let aa = 22;
                }
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //
                //

                fn bb() {
                    let bb = 33;
                }
"#
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);

        let requests_count = Arc::new(AtomicUsize::new(0));
        let closure_requests_count = requests_count.clone();
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "rust-analyzer",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let requests_count = closure_requests_count.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| {
                            let requests_count = requests_count.clone();
                            async move {
                                requests_count.fetch_add(1, Ordering::Release);
                                if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                                {
                                    Ok(Some(vec![
                                        lsp::InlayHint {
                                            position: lsp::Position::new(1, 9),
                                            label: lsp::InlayHintLabel::String(": i32".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                        lsp::InlayHint {
                                            position: lsp::Position::new(19, 9),
                                            label: lsp::InlayHintLabel::String(": i33".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                    ]))
                                } else if params.text_document.uri
                                    == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                                {
                                    Ok(Some(vec![
                                        lsp::InlayHint {
                                            position: lsp::Position::new(1, 10),
                                            label: lsp::InlayHintLabel::String(": i34".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                        lsp::InlayHint {
                                            position: lsp::Position::new(29, 10),
                                            label: lsp::InlayHintLabel::String(": i35".to_owned()),
                                            kind: Some(lsp::InlayHintKind::TYPE),
                                            text_edits: None,
                                            tooltip: None,
                                            padding_left: None,
                                            padding_right: None,
                                            data: None,
                                        },
                                    ]))
                                } else {
                                    panic!("Unexpected file path {:?}", params.text_document.uri);
                                }
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        // Add another server that does send the same, duplicate hints back
        let mut fake_servers_2 = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "CrabLang-ls",
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                        move |params, _| async move {
                            if params.text_document.uri
                                == lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap()
                            {
                                Ok(Some(vec![
                                    lsp::InlayHint {
                                        position: lsp::Position::new(1, 9),
                                        label: lsp::InlayHintLabel::String(": i32".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                    lsp::InlayHint {
                                        position: lsp::Position::new(19, 9),
                                        label: lsp::InlayHintLabel::String(": i33".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                ]))
                            } else if params.text_document.uri
                                == lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                            {
                                Ok(Some(vec![
                                    lsp::InlayHint {
                                        position: lsp::Position::new(1, 10),
                                        label: lsp::InlayHintLabel::String(": i34".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                    lsp::InlayHint {
                                        position: lsp::Position::new(29, 10),
                                        label: lsp::InlayHintLabel::String(": i35".to_owned()),
                                        kind: Some(lsp::InlayHintKind::TYPE),
                                        text_edits: None,
                                        tooltip: None,
                                        padding_left: None,
                                        padding_right: None,
                                        data: None,
                                    },
                                ]))
                            } else {
                                panic!("Unexpected file path {:?}", params.text_document.uri);
                            }
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        );

        let (buffer_1, _handle_1) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let (buffer_2, _handle_2) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/a/lib.rs"), cx)
            })
            .await
            .unwrap();
        let multi_buffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.push_excerpts(
                buffer_2.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(10, 0)),
                    ExcerptRange::new(Point::new(23, 0)..Point::new(34, 0)),
                ],
                cx,
            );
            multibuffer.push_excerpts(
                buffer_1.clone(),
                [
                    ExcerptRange::new(Point::new(0, 0)..Point::new(10, 0)),
                    ExcerptRange::new(Point::new(13, 0)..Point::new(23, 0)),
                ],
                cx,
            );
            multibuffer
        });

        let editor = cx.add_window(|window, cx| {
            let mut editor =
                Editor::for_multibuffer(multi_buffer, Some(project.clone()), window, cx);
            editor.change_selections(SelectionEffects::default(), window, cx, |s| {
                s.select_ranges([Point::new(3, 3)..Point::new(3, 3)])
            });
            editor
        });

        let fake_server = fake_servers.next().await.unwrap();
        let _fake_server_2 = fake_servers_2.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();

        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
                        ": i32".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                        ": i33".to_string(),
                        ": i34".to_string(),
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i35".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "We receive duplicate hints from 2 servers and cache them all"
                );
                assert_eq!(
                    vec![
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "lib.rs is added before main.rs , so its excerpts should be visible first; hints should be deduplicated per label"
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(Ordering::Acquire),
            2,
            "Should have queried hints once per each file"
        );

        // Scroll all the way down so the 1st buffer is out of sight.
        // The selection is on the 1st buffer still.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Line(88.0), window, cx);
            })
            .unwrap();
        // Emulate a language server refresh request, coming in the background..
        editor
            .update(cx, |editor, _, cx| {
                editor.refresh_inlay_hints(
                    InlayHintRefreshReason::RefreshRequested {
                        server_id: fake_server.server.server_id(),
                        request_id: Some(1),
                    },
                    cx,
                );
            })
            .unwrap();
        // Edit the 1st buffer while scrolled down and not seeing that.
        // The edit will auto scroll to the edit (1st buffer).
        editor
            .update(cx, |editor, window, cx| {
                editor.handle_input("a", window, cx);
            })
            .unwrap();
        // Add more racy additive hint tasks.
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Line(0.2), window, cx);
            })
            .unwrap();

        cx.executor().advance_clock(Duration::from_millis(1000));
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
                        ": i32".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                        ": i33".to_string(),
                        ": i34".to_string(),
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i35".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "No hint changes/duplicates should occur in the cache",
                );
                assert_eq!(
                    vec![
                        ": i34".to_string(),
                        ": i35".to_string(),
                        ": i32".to_string(),
                        ": i33".to_string(),
                    ],
                    visible_hint_labels(editor, cx),
                    "No hint changes/duplicates should occur in the editor excerpts",
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(Ordering::Acquire),
            4,
            "Should have queried hints once more per each file, after editing the file once"
        );
    }

    pub(crate) fn init_test(cx: &mut TestAppContext, f: impl Fn(&mut AllLanguageSettingsContent)) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(SemanticVersion::default(), cx);
            crate::init(cx);
        });

        update_test_language_settings(cx, f);
    }

    async fn prepare_test_objects(
        cx: &mut TestAppContext,
        initialize: impl 'static + Send + Fn(&mut FakeLanguageServer, &'static str) + Send + Sync,
    ) -> (&'static str, WindowHandle<Editor>, FakeLanguageServer) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlays are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/a").as_ref()], cx).await;
        let file_path = path!("/a/main.rs");

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |server| initialize(server, file_path))),
                ..FakeLspAdapter::default()
            },
        );

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/a/main.rs"), cx)
            })
            .await
            .unwrap();
        let editor =
            cx.add_window(|window, cx| Editor::for_buffer(buffer, Some(project), window, cx));

        editor
            .update(cx, |editor, _, cx| {
                assert!(cached_hint_labels(editor, cx).is_empty());
                assert!(visible_hint_labels(editor, cx).is_empty());
            })
            .unwrap();

        cx.executor().run_until_parked();
        let fake_server = fake_servers.next().await.unwrap();
        (file_path, editor, fake_server)
    }

    // Inlay hints in the cache are stored per excerpt as a key, and those keys are guaranteed to be ordered same as in the multi buffer.
    // Ensure a stable order for testing.
    fn sorted_cached_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
        let mut labels = cached_hint_labels(editor, cx);
        labels.sort_by(|a, b| natural_sort(a, b));
        labels
    }

    pub fn cached_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
        let lsp_store = editor.project().unwrap().read(cx).lsp_store();

        let mut all_cached_labels = Vec::new();
        let mut all_fetched_hints = Vec::new();
        for buffer in editor.buffer.read(cx).all_buffers() {
            lsp_store.update(cx, |lsp_store, cx| {
                let hints = lsp_store.latest_lsp_data(&buffer, cx).inlay_hints();
                all_cached_labels.extend(hints.all_cached_hints().into_iter().map(|hint| {
                    let mut label = hint.text().to_string();
                    if hint.padding_left {
                        label.insert(0, ' ');
                    }
                    if hint.padding_right {
                        label.push_str(" ");
                    }
                    label
                }));
                all_fetched_hints.extend(hints.all_fetched_hints());
            });
        }

        all_cached_labels
    }

    pub fn visible_hint_labels(editor: &Editor, cx: &Context<Editor>) -> Vec<String> {
        editor
            .visible_inlay_hints(cx)
            .into_iter()
            .map(|hint| hint.text().to_string())
            .collect()
    }

    fn allowed_hint_kinds_for_editor(editor: &Editor) -> HashSet<Option<InlayHintKind>> {
        editor
            .inlay_hints
            .as_ref()
            .unwrap()
            .allowed_hint_kinds
            .clone()
    }
}
