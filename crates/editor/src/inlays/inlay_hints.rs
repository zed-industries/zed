use std::{
    ops::{ControlFlow, Range},
    time::Duration,
};

use collections::{HashMap, HashSet};
use gpui::{App, Task};
use language::{
    BufferRow,
    language_settings::{InlayHintKind, InlayHintSettings, language_settings},
};
use lsp::LanguageServerId;
use multi_buffer::{Anchor, ExcerptId, MultiBufferSnapshot};
use project::{
    HoverBlock, HoverBlockKind, InlayHintLabel, InlayHintLabelPartTooltip, InlayHintTooltip,
    InvalidationStrategy, ResolveState,
    lsp_store::{ResolvedHint, RowChunkCachedHints},
};
use text::{Bias, BufferId, OffsetRangeExt as _};
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
    inlays_for_version: Option<clock::Global>,
    hint_tasks: HashMap<BufferId, HashMap<Range<BufferRow>, Task<()>>>,
    hint_chunks_received: HashSet<Range<BufferRow>>,
    pub hint_kinds: HashMap<InlayId, Option<InlayHintKind>>,
}

impl LspInlayHintData {
    pub fn new(settings: InlayHintSettings) -> Self {
        Self {
            modifiers_override: false,
            enabled: settings.enabled,
            enabled_in_settings: settings.enabled,
            inlays_for_version: None,
            hint_tasks: HashMap::default(),
            hint_kinds: HashMap::default(),
            hint_chunks_received: HashSet::default(),
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
        self.inlays_for_version = None;
        self.hint_tasks.clear();
        self.hint_chunks_received.clear();
        self.hint_kinds.clear();
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
                                    let inlay_kind = self.hint_kinds.get(&inlay.id).copied()?;
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
                                let inlay_kind = self.hint_kinds.get(&inlay.id).copied()?;
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
}

#[derive(Debug, Clone)]
pub enum InlayHintRefreshReason {
    ModifiersChanged(bool),
    Toggle(bool),
    SettingsChange(InlayHintSettings),
    NewLinesShown,
    BufferEdited,
    RefreshRequested(LanguageServerId),
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

    /// TODO kb docs
    pub(crate) fn refresh_inlay_hints(
        &mut self,
        reason: InlayHintRefreshReason,
        cx: &mut Context<Self>,
    ) {
        // TODO kb when hints are turned off in the editor and toggled on again, nothing happens
        // TODO kb multi buffer is eager to hide the hints which is wrong
        if !self.mode.is_full() || self.inlay_hints.is_none() {
            return;
        }
        let Some(semantics_provider) = self.semantics_provider.clone() else {
            return;
        };

        let invalidate_cache = {
            let visible_inlay_hints = self.visible_inlay_hints(cx);
            let Some(inlay_hints) = self.inlay_hints.as_mut() else {
                return;
            };

            let invalidate_cache = match reason {
                InlayHintRefreshReason::ModifiersChanged(enabled) => {
                    match inlay_hints.modifiers_override(enabled) {
                        Some(enabled) => {
                            if enabled {
                                InvalidationStrategy::RefreshRequested(None)
                            } else {
                                self.splice_inlays(
                                    &visible_inlay_hints
                                        .iter()
                                        .map(|inlay| inlay.id)
                                        .collect::<Vec<InlayId>>(),
                                    Vec::new(),
                                    cx,
                                );
                                return;
                            }
                        }
                        None => return,
                    }
                }
                InlayHintRefreshReason::Toggle(enabled) => {
                    if inlay_hints.toggle(enabled) {
                        if enabled {
                            InvalidationStrategy::None
                        } else {
                            self.splice_inlays(
                                &visible_inlay_hints
                                    .iter()
                                    .map(|inlay| inlay.id)
                                    .collect::<Vec<InlayId>>(),
                                Vec::new(),
                                cx,
                            );
                            return;
                        }
                    } else {
                        return;
                    }
                }
                InlayHintRefreshReason::SettingsChange(new_settings) => {
                    match inlay_hints.update_settings(new_settings, visible_inlay_hints) {
                        ControlFlow::Break(Some(InlaySplice {
                            to_remove,
                            to_insert,
                        })) => {
                            self.splice_inlays(&to_remove, to_insert, cx);
                            return;
                        }
                        ControlFlow::Break(None) => return,
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
                    self.display_map.update(cx, |display_map, _| {
                        display_map.remove_inlays_for_excerpts(&excerpts_removed)
                    });
                    return;
                }
                InlayHintRefreshReason::NewLinesShown => InvalidationStrategy::None,
                InlayHintRefreshReason::BufferEdited => InvalidationStrategy::BufferEdited,
                InlayHintRefreshReason::RefreshRequested(server_id) => {
                    InvalidationStrategy::RefreshRequested(Some(server_id))
                }
            };
            invalidate_cache
        };

        match &self.inlay_hints {
            Some(inlay_hints) => {
                if !inlay_hints.enabled {
                    return;
                }
            }
            None => return,
        }
        let ignore_debounce = matches!(
            reason,
            InlayHintRefreshReason::SettingsChange(_)
                | InlayHintRefreshReason::Toggle(_)
                | InlayHintRefreshReason::ExcerptsRemoved(_)
                | InlayHintRefreshReason::ModifiersChanged(_)
        );
        let debounce = if ignore_debounce {
            None
        } else {
            self.inlay_hints.as_ref().and_then(|inlay_hints| {
                if invalidate_cache.should_invalidate() {
                    inlay_hints.invalidate_debounce
                } else {
                    inlay_hints.append_debounce
                }
            })
        };

        // TODO kb this will result for multiple requests for the same chunks, batch ranges per buffer id instead.
        for (excerpt_id, (buffer, buffer_version, range)) in self.visible_excerpts(cx) {
            let Some(inlay_hints) = self.inlay_hints.as_mut() else {
                return;
            };
            let buffer_id = buffer.read(cx).remote_id();
            let buffer_snapshot = buffer.read(cx).snapshot();
            let buffer_anchor_range =
                buffer_snapshot.anchor_before(range.start)..buffer_snapshot.anchor_after(range.end);
            let buffer_point_range = buffer_anchor_range.to_point(&buffer_snapshot);

            let hints_range = buffer_point_range.start.row..buffer_point_range.end.row;
            // TODO kb check if the range is covered already by the fetched chunks, for non-invalidate cases
            // TODO kb we can have new allowed hint kids applied, with old spliced away and duplicates in the new inlay hints
            // TODO kb also, avoid splices that remove and re-add same inlays
            let a = &inlay_hints.hint_chunks_received;

            let semantics_provider = semantics_provider.clone();
            inlay_hints.hint_tasks.entry(buffer_id).or_default().insert(
                // TODO kb this is a range of the visible excerpt
                // does not look appropriate?
                hints_range.clone(),
                cx.spawn(async move |editor, cx| {
                    if let Some(debounce) = debounce {
                        cx.background_executor().timer(debounce).await;
                    }
                    let Ok(Some(new_hints)) = cx.update(|cx| {
                        semantics_provider.inlay_hints(
                            invalidate_cache,
                            buffer,
                            buffer_anchor_range.clone(),
                            cx,
                        )
                    }) else {
                        return;
                    };
                    let new_hints = new_hints.await;
                    editor
                        .update(cx, |editor, cx| {
                            let visible_inlay_hint_ids = editor
                                .visible_inlay_hints(cx)
                                .iter()
                                .map(|inlay| inlay.id)
                                .collect::<Vec<_>>();
                            let multi_buffer_snapshot = editor.buffer.read(cx).snapshot(cx);
                            let Some(buffer_snapshot) =
                                multi_buffer_snapshot.buffer_for_excerpt(excerpt_id)
                            else {
                                return;
                            };

                            let mut update_data = None;
                            let should_invalidate = invalidate_cache.should_invalidate();
                            if let Some(inlay_hints) = editor.inlay_hints.as_mut() {
                                let inlay_tasks =
                                    inlay_hints.hint_tasks.entry(buffer_id).or_default();
                                match new_hints {
                                    Ok(new_hints) => {
                                        let mut hints_to_remove = Vec::new();
                                        match &inlay_hints.inlays_for_version {
                                            Some(inlays_for_version) => {
                                                if !inlays_for_version
                                                    .changed_since(&buffer_version)
                                                {
                                                    if should_invalidate
                                                        || buffer_version
                                                            .changed_since(inlays_for_version)
                                                    {
                                                        inlay_tasks.clear();
                                                        inlay_hints.hint_chunks_received.clear();
                                                        inlay_hints.hint_kinds.clear();
                                                        hints_to_remove
                                                            .extend(visible_inlay_hint_ids);
                                                    }
                                                }
                                            }
                                            None => {}
                                        }

                                        let hints_to_insert = new_hints
                                            .into_iter()
                                            .flat_map(
                                                |(chunk, RowChunkCachedHints { hints, .. })| {
                                                    inlay_hints
                                                        .hint_chunks_received
                                                        .insert(chunk.clone());
                                                    hints
                                                        .into_values()
                                                        .flatten()
                                                        .filter(|(new_hint_id, _)| {
                                                            !inlay_hints
                                                                .hint_kinds
                                                                .contains_key(new_hint_id)
                                                        })
                                                        .filter(|(_, new_hint)| {
                                                            inlay_hints
                                                                .allowed_hint_kinds
                                                                .contains(&new_hint.kind)
                                                        })
                                                        .collect::<Vec<_>>()
                                                },
                                            )
                                            .collect::<Vec<_>>()
                                            .into_iter()
                                            .filter_map(|(hint_id, lsp_hint)| {
                                                if lsp_hint
                                                    .position
                                                    .cmp(
                                                        &buffer_anchor_range.start,
                                                        buffer_snapshot,
                                                    )
                                                    .is_ge()
                                                    && lsp_hint
                                                        .position
                                                        .cmp(
                                                            &buffer_anchor_range.end,
                                                            buffer_snapshot,
                                                        )
                                                        .is_le()
                                                {
                                                    let position = multi_buffer_snapshot
                                                        .anchor_in_excerpt(
                                                            excerpt_id,
                                                            lsp_hint.position,
                                                        )?;
                                                    inlay_hints
                                                        .hint_kinds
                                                        .insert(hint_id, lsp_hint.kind);
                                                    return Some(Inlay::hint(
                                                        hint_id, position, &lsp_hint,
                                                    ));
                                                }
                                                None
                                            })
                                            .collect();
                                        // TODO kb hints to remove and to insert may be almost the same, causing unnecessary flickering
                                        update_data = Some((hints_to_remove, hints_to_insert));
                                        inlay_hints.inlays_for_version = Some(buffer_version);
                                    }
                                    // TODO kb who should log and clean up the errored state? Could we do that with `lsp_store_cx.spawn`?
                                    Err(_) => {}
                                }

                                inlay_tasks.remove(&hints_range);
                            }

                            if let Some((hints_to_remove, hints_to_insert)) = update_data {
                                editor.splice_inlays(&hints_to_remove, hints_to_insert, cx);
                            }
                        })
                        .ok();
                }),
            );
        }
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
}

#[cfg(test)]
pub mod tests {
    use crate::editor_tests::update_test_language_settings;
    use crate::scroll::ScrollAmount;
    use crate::{Editor, SelectionEffects};
    use crate::{ExcerptRange, scroll::Autoscroll, test::editor_lsp_test_context::rust_lang};
    use collections::HashSet;
    use futures::{StreamExt, future};
    use gpui::{AppContext as _, Context, SemanticVersion, TestAppContext, WindowHandle};
    use itertools::Itertools as _;
    use language::language_settings::InlayHintKind;
    use language::{Capability, FakeLspAdapter};
    use language::{Language, LanguageConfig, LanguageMatcher};
    use lsp::FakeLanguageServer;
    use multi_buffer::MultiBuffer;
    use parking_lot::Mutex;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::{AllLanguageSettingsContent, InlayHintSettingsContent, SettingsStore};
    use std::ops::Range;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
    use text::{Point, ToPoint as _};
    use ui::App;
    use util::path;

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

        let progress_token = "test_progress_token";
        fake_server
            .request::<lsp::request::WorkDoneProgressCreate>(lsp::WorkDoneProgressCreateParams {
                token: lsp::ProgressToken::String(progress_token.to_string()),
            })
            .await
            .into_response()
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
            token: lsp::ProgressToken::String(progress_token.to_string()),
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

        // TODO kb we seem to run inlay fetch both on editor registration and on editor creation?
        // TODO kb also, what's up with the color task inside lsp_data, for zero color capabilities?
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
                    ..Default::default()
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
                                        position: params.range.end,
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
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, window, cx| {
                editor.scroll_screen(&ScrollAmount::Page(1.0), window, cx);
            })
            .unwrap();
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
                    first_scroll.end.line + 1,
                    second_scroll.start.line,
                    "Should query 2 adjacent ranges after the scrolls, but got: {ranges:?}"
                );

                let lsp_requests = lsp_request_count.load(Ordering::Acquire);
                assert_eq!(
                    lsp_requests, 3,
                    "Should query hints initially, and after each scroll (2 times)"
                );
                assert_eq!(
                    vec!["49".to_string(), "100".to_string(), "150".to_string()],
                    cached_hint_labels(editor, cx),
                    "Chunks of 50 line width should have been queried each time"
                );
                assert_eq!(
                    vec!["49".to_string(), "100".to_string()],
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
        cx.executor().run_until_parked();
        editor.update(cx, |editor, _window, cx| {
            let mut ranges = lsp_request_ranges.lock().drain(..).collect::<Vec<_>>();
            ranges.sort_by_key(|r| r.start);

            assert_eq!(ranges.len(), 2,
                "On edit, should scroll to selection and query a range around it: that range should split into 2 50 rows wide chunks. Instead, got query ranges {ranges:?}");
            let first_chunk = &ranges[0];
            let second_chunk = &ranges[1];
            assert!(first_chunk.end.line + 1 == second_chunk.start.line,
                "First chunk {first_chunk:?} should be before second chunk {second_chunk:?}");
            assert!(first_chunk.start.line < selection_in_cached_range.row,
                "Hints should be queried with the selected range after the query range start");

            let lsp_requests = lsp_request_count.load(Ordering::Acquire);
            assert_eq!(lsp_requests, 5, "Two chunks should be re-queried");
            assert_eq!(vec!["100".to_string(), "150".to_string()], cached_hint_labels(editor, cx),
                "Should have (less) hints from the new LSP response after the edit");
            assert_eq!(vec!["100".to_string()], visible_hint_labels(editor, cx), "Should show only visible hints (in the center) from the new cached set");
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
                    |s| s.select_ranges([Point::new(50, 0)..Point::new(50, 0)]),
                );
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
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
                    ], sorted_cached_hint_labels(editor, cx),
                    "With more scrolls of the multibuffer, more hints should be added into the cache and nothing invalidated without edits");
                assert_eq!(
                    vec![
                        "main hint #0".to_string(),
                        "main hint #1".to_string(),
                        "main hint #2".to_string(),
                        "main hint #3".to_string(),
                        "main hint #4".to_string(),
                        "main hint #5".to_string(),
                        "other hint #0".to_string(),
                        "other hint #1".to_string(),
                        "other hint #2".to_string(),
                    ],
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
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
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
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, all hints for all excerpts should be fetched"
                );
                assert_eq!(
                    vec![
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
                    ],
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
                assert_eq!(
                    vec![
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
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer was scrolled to the end, further scrolls up should not bring more hints"
                );
                assert_eq!(
                    vec![
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
                    ],
                    visible_hint_labels(editor, cx),
                );
            })
            .unwrap();

        editor_edited.store(true, Ordering::Release);
        editor
            .update(cx, |editor, window, cx| {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([Point::new(57, 0)..Point::new(57, 0)])
                });
                editor.handle_input("++++more text++++", window, cx);
            })
            .unwrap();
        cx.executor().run_until_parked();
        editor
            .update(cx, |editor, _window, cx| {
                assert_eq!(
                    vec![
                        "main hint(edited) #0".to_string(),
                        "main hint(edited) #1".to_string(),
                        "main hint(edited) #2".to_string(),
                        "main hint(edited) #3".to_string(),
                        "other hint(edited) #0".to_string(),
                        "other hint(edited) #1".to_string(),
                        "other hint(edited) #2".to_string(),
                        "other hint(edited) #3".to_string(),
                    ],
                    sorted_cached_hint_labels(editor, cx),
                    "After multibuffer edit, editor gets scrolled back to the last selection; \
                all hints should be invalidated and required for all of its visible excerpts"
                );
                // TODO kb is this right?
                assert_eq!(
                    vec!["other hint(edited) #0".to_string()],
                    visible_hint_labels(editor, cx),
                    "Only the visible hints should be shown after editing"
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
                    "Cache should update for both excerpts despite hints display was disabled; cache should not include hints out of ranges (request one and row chunk one)"
                );
                let visible_hints = visible_hint_labels(editor, cx);
                assert!(
                    visible_hints.is_empty(),
                    "All hints are disabled and should not be shown despite being present in the cache, but got: {visible_hints:?}"
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
                    vec!["main hint #0".to_string()],
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
                    ..Default::default()
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
                ..Default::default()
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

                        let i = lsp_request_count.fetch_add(1, Ordering::SeqCst) + 1;
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

    pub(crate) fn init_test(cx: &mut TestAppContext, f: impl Fn(&mut AllLanguageSettingsContent)) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(SemanticVersion::default(), cx);
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
        labels.sort();
        labels
    }

    pub fn cached_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
        let lsp_store = editor.project().unwrap().read(cx).lsp_store();

        let mut all_cached_labels = Vec::new();
        let mut all_fetched_hints = Vec::new();
        for buffer in editor.buffer.read(cx).all_buffers() {
            lsp_store.update(cx, |lsp_store, cx| {
                let hints = &lsp_store.latest_lsp_data(&buffer, cx).inlay_hints;
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
