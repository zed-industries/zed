use std::{
    collections::hash_map,
    ops::{ControlFlow, Range},
    time::Duration,
};

use clock::Global;
use collections::{HashMap, HashSet};
use futures::future::join_all;
use gpui::{App, Entity, Pixels, Task};
use itertools::Itertools;
use language::{
    BufferRow,
    language_settings::{InlayHintKind, InlayHintSettings},
};
use lsp::LanguageServerId;
use multi_buffer::{Anchor, MultiBufferSnapshot};
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
    display_map::{DisplayMap, InlayOffset},
    hover_links::{InlayHighlight, TriggerPoint, show_link_definition},
    hover_popover::{self, InlayHover},
    inlays::InlaySplice,
};

pub fn inlay_hint_settings(
    location: Anchor,
    snapshot: &MultiBufferSnapshot,
    cx: &mut Context<Editor>,
) -> InlayHintSettings {
    snapshot.language_settings_at(location, cx).inlay_hints
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

    /// Like `clear`, but only wipes tracking state for the given buffer IDs.
    /// Hints belonging to other buffers are left intact so they are neither
    /// re-fetched nor duplicated on the next `NewLinesShown`.
    pub fn clear_for_buffers(
        &mut self,
        buffer_ids: &HashSet<BufferId>,
        current_hints: impl IntoIterator<Item = Inlay>,
        snapshot: &MultiBufferSnapshot,
    ) {
        for buffer_id in buffer_ids {
            self.hint_refresh_tasks.remove(buffer_id);
            self.hint_chunk_fetching.remove(buffer_id);
        }
        for hint in current_hints {
            if let Some((text_anchor, _)) = snapshot.anchor_to_buffer_anchor(hint.position) {
                if buffer_ids.contains(&text_anchor.buffer_id) {
                    self.added_hints.remove(&hint.id);
                }
            }
        }
    }

    /// Checks inlay hint settings for enabled hint kinds and general enabled state.
    /// Generates corresponding inlay_map splice updates on settings changes.
    /// Does not update inlay hint cache state on disabling or inlay hint kinds change: only reenabling forces new LSP queries.
    fn update_settings(
        &mut self,
        new_hint_settings: InlayHintSettings,
        visible_hints: impl IntoIterator<Item = Inlay>,
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
                                .into_iter()
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
                let mut visible_hints = visible_hints.into_iter().peekable();
                if visible_hints.peek().is_none() {
                    ControlFlow::Break(None)
                } else {
                    self.clear();
                    ControlFlow::Break(Some(InlaySplice {
                        to_remove: visible_hints.map(|inlay| inlay.id).collect(),
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
                            .into_iter()
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
    ServerRemoved,
    RefreshRequested {
        server_id: LanguageServerId,
        request_id: Option<usize>,
    },
    BuffersRemoved(Vec<BufferId>),
}

impl Editor {
    pub fn supports_inlay_hints(&self, cx: &mut App) -> bool {
        let Some(provider) = self.semantics_provider.as_ref() else {
            return false;
        };

        let mut supports = false;
        self.buffer().update(cx, |this, cx| {
            this.for_each_buffer(&mut |buffer| {
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
        if !self.lsp_data_enabled() || self.inlay_hints.is_none() {
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
            | InlayHintRefreshReason::BuffersRemoved(_)
            | InlayHintRefreshReason::ModifiersChanged(_) => None,
            _may_need_lsp_call => self.inlay_hints.as_ref().and_then(|inlay_hints| {
                if invalidate_cache.should_invalidate() {
                    inlay_hints.invalidate_debounce
                } else {
                    inlay_hints.append_debounce
                }
            }),
        };

        let mut visible_excerpts = self.visible_buffer_ranges(cx);
        visible_excerpts.retain(|(snapshot, _, _)| self.is_lsp_relevant(snapshot.file(), cx));

        let mut invalidate_hints_for_buffers = HashSet::default();
        let ignore_previous_fetches = match reason {
            InlayHintRefreshReason::ModifiersChanged(_)
            | InlayHintRefreshReason::Toggle(_)
            | InlayHintRefreshReason::SettingsChange(_)
            | InlayHintRefreshReason::ServerRemoved => true,
            InlayHintRefreshReason::NewLinesShown
            | InlayHintRefreshReason::RefreshRequested { .. }
            | InlayHintRefreshReason::BuffersRemoved(_) => false,
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
                visible_excerpts.retain(|(buffer_snapshot, _, _)| {
                    buffer_snapshot.language() == Some(&affected_language)
                });
                false
            }
        };

        let multi_buffer = self.buffer().clone();

        let Some(inlay_hints) = self.inlay_hints.as_mut() else {
            return;
        };

        if invalidate_cache.should_invalidate() {
            if invalidate_hints_for_buffers.is_empty() {
                inlay_hints.clear();
            } else {
                inlay_hints.clear_for_buffers(
                    &invalidate_hints_for_buffers,
                    Self::visible_inlay_hints(self.display_map.read(cx)),
                    &multi_buffer.read(cx).snapshot(cx),
                );
            }
        }
        inlay_hints
            .invalidate_hints_for_buffers
            .extend(invalidate_hints_for_buffers);

        let mut buffers_to_query = HashMap::default();
        for (buffer_snapshot, visible_range, _) in visible_excerpts {
            let buffer_id = buffer_snapshot.remote_id();

            if !self.registered_buffers.contains_key(&buffer_id) {
                continue;
            }

            let Some(buffer) = multi_buffer.read(cx).buffer(buffer_id) else {
                continue;
            };

            let buffer_version = buffer_snapshot.version().clone();
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
        let to_remove = Self::visible_inlay_hints(self.display_map.read(cx))
            .map(|inlay| inlay.id)
            .collect::<Vec<_>>();
        self.splice_inlays(&to_remove, Vec::new(), cx);
    }

    fn refresh_editor_data(
        &mut self,
        reason: &InlayHintRefreshReason,
        cx: &mut Context<'_, Editor>,
    ) -> Option<InvalidationStrategy> {
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
                let visible_inlay_hints =
                    Self::visible_inlay_hints(self.display_map.read(cx)).collect::<Vec<_>>();
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
            InlayHintRefreshReason::BuffersRemoved(buffers_removed) => {
                let to_remove = self
                    .display_map
                    .read(cx)
                    .current_inlays()
                    .filter_map(|inlay| {
                        let anchor = inlay.position.raw_text_anchor()?;
                        if buffers_removed.contains(&anchor.buffer_id) {
                            Some(inlay.id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                self.splice_inlays(&to_remove, Vec::new(), cx);
                return None;
            }
            InlayHintRefreshReason::ServerRemoved => InvalidationStrategy::BufferEdited,
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

    pub(super) fn visible_inlay_hints(
        display_map: &DisplayMap,
    ) -> impl Iterator<Item = Inlay> + use<'_> {
        display_map
            .current_inlays()
            .filter(move |inlay| matches!(inlay.id, InlayId::Hint(_)))
            .cloned()
    }

    pub(crate) fn allowed_hint_kinds_for_editor(editor: &Editor) -> HashSet<Option<InlayHintKind>> {
        editor
            .inlay_hints
            .as_ref()
            .unwrap()
            .allowed_hint_kinds
            .clone()
    }

    pub fn update_inlay_link_and_hover_points(
        &mut self,
        snapshot: &EditorSnapshot,
        point_for_position: PointForPosition,
        mouse_position: Option<gpui::Point<Pixels>>,
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
            if let Some(hovered_hint) = Self::visible_inlay_hints(self.display_map.read(cx))
                .filter(|hint| snapshot.can_resolve(&hint.position))
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
                if let Some(ResolvedHint::Resolved(cached_hint)) = buffer_snapshot
                    .anchor_to_buffer_anchor(hovered_hint.position)
                    .and_then(|(anchor, _)| {
                        lsp_store.update(cx, |lsp_store, cx| {
                            lsp_store.resolved_hint(anchor.buffer_id, hovered_hint.id, cx)
                        })
                    })
                {
                    match cached_hint.resolve_state {
                        ResolveState::Resolved => {
                            let original_text = cached_hint.text();
                            let actual_left_padding =
                                if cached_hint.padding_left && !original_text.starts_with(" ") {
                                    1
                                } else {
                                    0
                                };
                            let actual_right_padding =
                                if cached_hint.padding_right && !original_text.ends_with(" ") {
                                    1
                                } else {
                                    0
                                };
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
                                                    range: actual_left_padding
                                                        ..hovered_hint.text().len()
                                                            - actual_right_padding,
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
                                    let content_start =
                                        InlayOffset(hint_start.0 + actual_left_padding);
                                    if let Some((hovered_hint_part, part_range)) =
                                        hover_popover::find_hovered_hint_part(
                                            label_parts,
                                            content_start,
                                            hovered_offset,
                                        )
                                    {
                                        let highlight_start = part_range.start - hint_start;
                                        let highlight_end = part_range.end - hint_start;
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
            hover_popover::hover_at(self, None, mouse_position, window, cx);
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
        let multi_buffer_snapshot = self.buffer.read(cx).snapshot(cx);
        let visible_inlay_hint_ids = Self::visible_inlay_hints(self.display_map.read(cx))
            .filter(|inlay| {
                multi_buffer_snapshot
                    .anchor_to_buffer_anchor(inlay.position)
                    .map(|(anchor, _)| anchor.buffer_id)
                    == Some(buffer_id)
            })
            .map(|inlay| inlay.id)
            .collect::<Vec<_>>();
        let Some(inlay_hints) = &mut self.inlay_hints else {
            return;
        };
        let Some(buffer_snapshot) = self
            .buffer
            .read(cx)
            .buffer(buffer_id)
            .map(|buffer| buffer.read(cx).snapshot())
        else {
            return;
        };

        let mut hints_to_remove = Vec::new();

        // If we've received hints from the cache, it means `invalidate_cache` had invalidated whatever possible there,
        // and most probably there are no more hints with IDs from `visible_inlay_hint_ids` in the cache.
        // So, if we hover such hints, no resolve will happen.
        //
        // Another issue is in the fact that changing one buffer may lead to other buffers' hints changing, so more cache entries may be removed.
        // Hence, clear all excerpts' hints in the multi buffer: later, the invalidated ones will re-trigger the LSP query, the rest will be restored
        // from the cache.
        if invalidate_cache.should_invalidate() {
            hints_to_remove.extend(visible_inlay_hint_ids);

            // When invalidating, this task removes ALL visible hints for the buffer
            // but only adds back hints for its own chunk ranges. Chunks fetched by
            // other concurrent tasks (e.g., a scroll task that completed before this
            // edit task) would have their hints removed but remain marked as "already
            // fetched" in hint_chunk_fetching, preventing re-fetch on the next
            // NewLinesShown. Fix: retain only chunks that this task has results for.
            let task_chunk_ranges: HashSet<&Range<BufferRow>> =
                new_hints.iter().map(|(range, _)| range).collect();
            if let Some((_, fetched_chunks)) = inlay_hints.hint_chunk_fetching.get_mut(&buffer_id) {
                fetched_chunks.retain(|chunk| task_chunk_ranges.contains(chunk));
            }
        }

        let mut inserted_hint_text = HashMap::default();
        let new_hints = new_hints
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
            .filter(|(hint_id, lsp_hint)| {
                inlay_hints.allowed_hint_kinds.contains(&lsp_hint.kind)
                    && inlay_hints
                        .added_hints
                        .insert(*hint_id, lsp_hint.kind)
                        .is_none()
            })
            .sorted_by(|(_, a), (_, b)| a.position.cmp(&b.position, &buffer_snapshot))
            .collect::<Vec<_>>();

        let hints_to_insert = multi_buffer_snapshot
            .text_anchors_to_visible_anchors(
                new_hints.iter().map(|(_, lsp_hint)| lsp_hint.position),
            )
            .into_iter()
            .zip(&new_hints)
            .filter_map(|(position, (hint_id, hint))| Some(Inlay::hint(*hint_id, position?, &hint)))
            .collect();
        let invalidate_hints_for_buffers =
            std::mem::take(&mut inlay_hints.invalidate_hints_for_buffers);
        if !invalidate_hints_for_buffers.is_empty() {
            hints_to_remove.extend(
                Self::visible_inlay_hints(self.display_map.read(cx))
                    .filter(|inlay| {
                        multi_buffer_snapshot
                            .anchor_to_buffer_anchor(inlay.position)
                            .is_none_or(|(anchor, _)| {
                                invalidate_hints_for_buffers.contains(&anchor.buffer_id)
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
