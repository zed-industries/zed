use crate::{
    display_map::DisplaySnapshot,
    element::PointForPosition,
    hover_popover::{self, InlayHover},
    Anchor, DisplayPoint, Editor, EditorSnapshot, InlayId, SelectPhase,
};
use gpui::{Task, ViewContext};
use language::{Bias, ToOffset};
use lsp::LanguageServerId;
use project::{
    HoverBlock, HoverBlockKind, InlayHintLabelPartTooltip, InlayHintTooltip, LocationLink,
    ResolveState,
};
use std::ops::Range;
use util::TryFutureExt;

#[derive(Debug, Default)]
pub struct LinkGoToDefinitionState {
    pub last_trigger_point: Option<TriggerPoint>,
    pub symbol_range: Option<RangeInEditor>,
    pub kind: Option<LinkDefinitionKind>,
    pub definitions: Vec<GoToDefinitionLink>,
    pub task: Option<Task<Option<()>>>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RangeInEditor {
    Text(Range<Anchor>),
    Inlay(InlayHighlight),
}

impl RangeInEditor {
    pub fn as_text_range(&self) -> Option<Range<Anchor>> {
        match self {
            Self::Text(range) => Some(range.clone()),
            Self::Inlay(_) => None,
        }
    }

    fn point_within_range(&self, trigger_point: &TriggerPoint, snapshot: &EditorSnapshot) -> bool {
        match (self, trigger_point) {
            (Self::Text(range), TriggerPoint::Text(point)) => {
                let point_after_start = range.start.cmp(point, &snapshot.buffer_snapshot).is_le();
                point_after_start && range.end.cmp(point, &snapshot.buffer_snapshot).is_ge()
            }
            (Self::Inlay(highlight), TriggerPoint::InlayHint(point, _, _)) => {
                highlight.inlay == point.inlay
                    && highlight.range.contains(&point.range.start)
                    && highlight.range.contains(&point.range.end)
            }
            (Self::Inlay(_), TriggerPoint::Text(_))
            | (Self::Text(_), TriggerPoint::InlayHint(_, _, _)) => false,
        }
    }
}

#[derive(Debug)]
pub enum GoToDefinitionTrigger {
    Text(DisplayPoint),
    InlayHint(InlayHighlight, lsp::Location, LanguageServerId),
}

#[derive(Debug, Clone)]
pub enum GoToDefinitionLink {
    Text(LocationLink),
    InlayHint(lsp::Location, LanguageServerId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlayHighlight {
    pub inlay: InlayId,
    pub inlay_position: Anchor,
    pub range: Range<usize>,
}

#[derive(Debug, Clone)]
pub enum TriggerPoint {
    Text(Anchor),
    InlayHint(InlayHighlight, lsp::Location, LanguageServerId),
}

impl TriggerPoint {
    pub fn definition_kind(&self, shift: bool) -> LinkDefinitionKind {
        match self {
            TriggerPoint::Text(_) => {
                if shift {
                    LinkDefinitionKind::Type
                } else {
                    LinkDefinitionKind::Symbol
                }
            }
            TriggerPoint::InlayHint(_, _, _) => LinkDefinitionKind::Type,
        }
    }

    fn anchor(&self) -> &Anchor {
        match self {
            TriggerPoint::Text(anchor) => anchor,
            TriggerPoint::InlayHint(inlay_range, _, _) => &inlay_range.inlay_position,
        }
    }
}

pub fn update_go_to_definition_link(
    editor: &mut Editor,
    origin: Option<GoToDefinitionTrigger>,
    cmd_held: bool,
    shift_held: bool,
    cx: &mut ViewContext<Editor>,
) {
    let pending_nonempty_selection = editor.has_pending_nonempty_selection();

    // Store new mouse point as an anchor
    let snapshot = editor.snapshot(cx);
    let trigger_point = match origin {
        Some(GoToDefinitionTrigger::Text(p)) => {
            Some(TriggerPoint::Text(snapshot.buffer_snapshot.anchor_before(
                p.to_offset(&snapshot.display_snapshot, Bias::Left),
            )))
        }
        Some(GoToDefinitionTrigger::InlayHint(p, lsp_location, language_server_id)) => {
            Some(TriggerPoint::InlayHint(p, lsp_location, language_server_id))
        }
        None => None,
    };

    // If the new point is the same as the previously stored one, return early
    if let (Some(a), Some(b)) = (
        &trigger_point,
        &editor.link_go_to_definition_state.last_trigger_point,
    ) {
        match (a, b) {
            (TriggerPoint::Text(anchor_a), TriggerPoint::Text(anchor_b)) => {
                if anchor_a.cmp(anchor_b, &snapshot.buffer_snapshot).is_eq() {
                    return;
                }
            }
            (TriggerPoint::InlayHint(range_a, _, _), TriggerPoint::InlayHint(range_b, _, _)) => {
                if range_a == range_b {
                    return;
                }
            }
            _ => {}
        }
    }

    editor.link_go_to_definition_state.last_trigger_point = trigger_point.clone();

    if pending_nonempty_selection {
        hide_link_definition(editor, cx);
        return;
    }

    if cmd_held {
        if let Some(trigger_point) = trigger_point {
            let kind = trigger_point.definition_kind(shift_held);
            show_link_definition(kind, editor, trigger_point, snapshot, cx);
            return;
        }
    }

    hide_link_definition(editor, cx);
}

pub fn update_inlay_link_and_hover_points(
    snapshot: &DisplaySnapshot,
    point_for_position: PointForPosition,
    editor: &mut Editor,
    cmd_held: bool,
    shift_held: bool,
    cx: &mut ViewContext<'_, '_, Editor>,
) {
    let hovered_offset = if point_for_position.column_overshoot_after_line_end == 0 {
        Some(snapshot.display_point_to_inlay_offset(point_for_position.exact_unclipped, Bias::Left))
    } else {
        None
    };
    let mut go_to_definition_updated = false;
    let mut hover_updated = false;
    if let Some(hovered_offset) = hovered_offset {
        let buffer_snapshot = editor.buffer().read(cx).snapshot(cx);
        let previous_valid_anchor = buffer_snapshot.anchor_at(
            point_for_position.previous_valid.to_point(snapshot),
            Bias::Left,
        );
        let next_valid_anchor = buffer_snapshot.anchor_at(
            point_for_position.next_valid.to_point(snapshot),
            Bias::Right,
        );
        if let Some(hovered_hint) = editor
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
            let inlay_hint_cache = editor.inlay_hint_cache();
            let excerpt_id = previous_valid_anchor.excerpt_id;
            if let Some(cached_hint) = inlay_hint_cache.hint_by_id(excerpt_id, hovered_hint.id) {
                match cached_hint.resolve_state {
                    ResolveState::CanResolve(_, _) => {
                        if let Some(buffer_id) = previous_valid_anchor.buffer_id {
                            inlay_hint_cache.spawn_hint_resolve(
                                buffer_id,
                                excerpt_id,
                                hovered_hint.id,
                                cx,
                            );
                        }
                    }
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
                            project::InlayHintLabel::String(_) => {
                                if let Some(tooltip) = cached_hint.tooltip {
                                    hover_popover::hover_at_inlay(
                                        editor,
                                        InlayHover {
                                            excerpt: excerpt_id,
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
                                                    ..hovered_hint.text.len() + extra_shift_right,
                                            },
                                        },
                                        cx,
                                    );
                                    hover_updated = true;
                                }
                            }
                            project::InlayHintLabel::LabelParts(label_parts) => {
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
                                            editor,
                                            InlayHover {
                                                excerpt: excerpt_id,
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
                                            cx,
                                        );
                                        hover_updated = true;
                                    }
                                    if let Some((language_server_id, location)) =
                                        hovered_hint_part.location
                                    {
                                        go_to_definition_updated = true;
                                        update_go_to_definition_link(
                                            editor,
                                            Some(GoToDefinitionTrigger::InlayHint(
                                                highlight,
                                                location,
                                                language_server_id,
                                            )),
                                            cmd_held,
                                            shift_held,
                                            cx,
                                        );
                                    }
                                }
                            }
                        };
                    }
                    ResolveState::Resolving => {}
                }
            }
        }
    }

    if !go_to_definition_updated {
        update_go_to_definition_link(editor, None, cmd_held, shift_held, cx);
    }
    if !hover_updated {
        hover_popover::hover_at(editor, None, cx);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkDefinitionKind {
    Symbol,
    Type,
}

pub fn show_link_definition(
    definition_kind: LinkDefinitionKind,
    editor: &mut Editor,
    trigger_point: TriggerPoint,
    snapshot: EditorSnapshot,
    cx: &mut ViewContext<Editor>,
) {
    let same_kind = editor.link_go_to_definition_state.kind == Some(definition_kind);
    if !same_kind {
        hide_link_definition(editor, cx);
    }

    if editor.pending_rename.is_some() {
        return;
    }

    let trigger_anchor = trigger_point.anchor();
    let (buffer, buffer_position) = if let Some(output) = editor
        .buffer
        .read(cx)
        .text_anchor_for_position(trigger_anchor.clone(), cx)
    {
        output
    } else {
        return;
    };

    let excerpt_id = if let Some((excerpt_id, _, _)) = editor
        .buffer()
        .read(cx)
        .excerpt_containing(trigger_anchor.clone(), cx)
    {
        excerpt_id
    } else {
        return;
    };

    let project = if let Some(project) = editor.project.clone() {
        project
    } else {
        return;
    };

    // Don't request again if the location is within the symbol region of a previous request with the same kind
    if let Some(symbol_range) = &editor.link_go_to_definition_state.symbol_range {
        if same_kind && symbol_range.point_within_range(&trigger_point, &snapshot) {
            return;
        }
    }

    let task = cx.spawn(|this, mut cx| {
        async move {
            let result = match &trigger_point {
                TriggerPoint::Text(_) => {
                    // query the LSP for definition info
                    cx.update(|cx| {
                        project.update(cx, |project, cx| match definition_kind {
                            LinkDefinitionKind::Symbol => {
                                project.definition(&buffer, buffer_position, cx)
                            }

                            LinkDefinitionKind::Type => {
                                project.type_definition(&buffer, buffer_position, cx)
                            }
                        })
                    })
                    .await
                    .ok()
                    .map(|definition_result| {
                        (
                            definition_result.iter().find_map(|link| {
                                link.origin.as_ref().map(|origin| {
                                    let start = snapshot
                                        .buffer_snapshot
                                        .anchor_in_excerpt(excerpt_id.clone(), origin.range.start);
                                    let end = snapshot
                                        .buffer_snapshot
                                        .anchor_in_excerpt(excerpt_id.clone(), origin.range.end);
                                    RangeInEditor::Text(start..end)
                                })
                            }),
                            definition_result
                                .into_iter()
                                .map(GoToDefinitionLink::Text)
                                .collect(),
                        )
                    })
                }
                TriggerPoint::InlayHint(highlight, lsp_location, server_id) => Some((
                    Some(RangeInEditor::Inlay(highlight.clone())),
                    vec![GoToDefinitionLink::InlayHint(
                        lsp_location.clone(),
                        *server_id,
                    )],
                )),
            };

            this.update(&mut cx, |this, cx| {
                // Clear any existing highlights
                this.clear_highlights::<LinkGoToDefinitionState>(cx);
                this.link_go_to_definition_state.kind = Some(definition_kind);
                this.link_go_to_definition_state.symbol_range = result
                    .as_ref()
                    .and_then(|(symbol_range, _)| symbol_range.clone());

                if let Some((symbol_range, definitions)) = result {
                    this.link_go_to_definition_state.definitions = definitions.clone();

                    let buffer_snapshot = buffer.read(cx).snapshot();

                    // Only show highlight if there exists a definition to jump to that doesn't contain
                    // the current location.
                    let any_definition_does_not_contain_current_location =
                        definitions.iter().any(|definition| {
                            match &definition {
                                GoToDefinitionLink::Text(link) => {
                                    if link.target.buffer == buffer {
                                        let range = &link.target.range;
                                        // Expand range by one character as lsp definition ranges include positions adjacent
                                        // but not contained by the symbol range
                                        let start = buffer_snapshot.clip_offset(
                                            range
                                                .start
                                                .to_offset(&buffer_snapshot)
                                                .saturating_sub(1),
                                            Bias::Left,
                                        );
                                        let end = buffer_snapshot.clip_offset(
                                            range.end.to_offset(&buffer_snapshot) + 1,
                                            Bias::Right,
                                        );
                                        let offset = buffer_position.to_offset(&buffer_snapshot);
                                        !(start <= offset && end >= offset)
                                    } else {
                                        true
                                    }
                                }
                                GoToDefinitionLink::InlayHint(_, _) => true,
                            }
                        });

                    if any_definition_does_not_contain_current_location {
                        // Highlight symbol using theme link definition highlight style
                        let style = theme::current(cx).editor.link_definition;
                        let highlight_range =
                            symbol_range.unwrap_or_else(|| match &trigger_point {
                                TriggerPoint::Text(trigger_anchor) => {
                                    let snapshot = &snapshot.buffer_snapshot;
                                    // If no symbol range returned from language server, use the surrounding word.
                                    let (offset_range, _) =
                                        snapshot.surrounding_word(*trigger_anchor);
                                    RangeInEditor::Text(
                                        snapshot.anchor_before(offset_range.start)
                                            ..snapshot.anchor_after(offset_range.end),
                                    )
                                }
                                TriggerPoint::InlayHint(highlight, _, _) => {
                                    RangeInEditor::Inlay(highlight.clone())
                                }
                            });

                        match highlight_range {
                            RangeInEditor::Text(text_range) => this
                                .highlight_text::<LinkGoToDefinitionState>(
                                    vec![text_range],
                                    style,
                                    cx,
                                ),
                            RangeInEditor::Inlay(highlight) => this
                                .highlight_inlays::<LinkGoToDefinitionState>(
                                    vec![highlight],
                                    style,
                                    cx,
                                ),
                        }
                    } else {
                        hide_link_definition(this, cx);
                    }
                }
            })?;

            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    });

    editor.link_go_to_definition_state.task = Some(task);
}

pub fn hide_link_definition(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    if editor.link_go_to_definition_state.symbol_range.is_some()
        || !editor.link_go_to_definition_state.definitions.is_empty()
    {
        editor.link_go_to_definition_state.symbol_range.take();
        editor.link_go_to_definition_state.definitions.clear();
        cx.notify();
    }

    editor.link_go_to_definition_state.task = None;

    editor.clear_highlights::<LinkGoToDefinitionState>(cx);
}

pub fn go_to_fetched_definition(
    editor: &mut Editor,
    point: PointForPosition,
    split: bool,
    cx: &mut ViewContext<Editor>,
) {
    go_to_fetched_definition_of_kind(LinkDefinitionKind::Symbol, editor, point, split, cx);
}

pub fn go_to_fetched_type_definition(
    editor: &mut Editor,
    point: PointForPosition,
    split: bool,
    cx: &mut ViewContext<Editor>,
) {
    go_to_fetched_definition_of_kind(LinkDefinitionKind::Type, editor, point, split, cx);
}

fn go_to_fetched_definition_of_kind(
    kind: LinkDefinitionKind,
    editor: &mut Editor,
    point: PointForPosition,
    split: bool,
    cx: &mut ViewContext<Editor>,
) {
    let cached_definitions = editor.link_go_to_definition_state.definitions.clone();
    hide_link_definition(editor, cx);
    let cached_definitions_kind = editor.link_go_to_definition_state.kind;

    let is_correct_kind = cached_definitions_kind == Some(kind);
    if !cached_definitions.is_empty() && is_correct_kind {
        if !editor.focused {
            cx.focus_self();
        }

        editor.navigate_to_definitions(cached_definitions, split, cx);
    } else {
        editor.select(
            SelectPhase::Begin {
                position: point.next_valid,
                add: false,
                click_count: 1,
            },
            cx,
        );

        if point.as_valid().is_some() {
            match kind {
                LinkDefinitionKind::Symbol => editor.go_to_definition(&Default::default(), cx),
                LinkDefinitionKind::Type => editor.go_to_type_definition(&Default::default(), cx),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::ToDisplayPoint,
        editor_tests::init_test,
        inlay_hint_cache::tests::{cached_hint_labels, visible_hint_labels},
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use futures::StreamExt;
    use gpui::{
        platform::{self, Modifiers, ModifiersChangedEvent},
        View,
    };
    use indoc::indoc;
    use language::language_settings::InlayHintSettings;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};
    use util::assert_set_eq;

    #[gpui::test]
    async fn test_link_go_to_type_definition(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                type_definition_provider: Some(lsp::TypeDefinitionProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            struct A;
            let vˇariable = A;
        "});

        // Basic hold cmd+shift, expect highlight in region if response contains type definition
        let hover_point = cx.display_point(indoc! {"
            struct A;
            let vˇariable = A;
        "});
        let symbol_range = cx.lsp_range(indoc! {"
            struct A;
            let «variable» = A;
        "});
        let target_range = cx.lsp_range(indoc! {"
            struct «A»;
            let variable = A;
        "});

        let mut requests =
            cx.handle_request::<GotoTypeDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoTypeDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: Some(symbol_range),
                        target_uri: url.clone(),
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        // Press cmd+shift to trigger highlight
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                true,
                cx,
            );
        });
        requests.next().await;
        cx.foreground().run_until_parked();
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            struct A;
            let «variable» = A;
        "});

        // Unpress shift causes highlight to go away (normal goto-definition is not valid here)
        cx.update_editor(|editor, cx| {
            editor.modifiers_changed(
                &platform::ModifiersChangedEvent {
                    modifiers: Modifiers {
                        cmd: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                cx,
            );
        });
        // Assert no link highlights
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            struct A;
            let variable = A;
        "});

        // Cmd+shift click without existing definition requests and jumps
        let hover_point = cx.display_point(indoc! {"
            struct A;
            let vˇariable = A;
        "});
        let target_range = cx.lsp_range(indoc! {"
            struct «A»;
            let variable = A;
        "});

        let mut requests =
            cx.handle_request::<GotoTypeDefinition, _, _>(move |url, _, _| async move {
                Ok(Some(lsp::GotoTypeDefinitionResponse::Link(vec![
                    lsp::LocationLink {
                        origin_selection_range: None,
                        target_uri: url,
                        target_range,
                        target_selection_range: target_range,
                    },
                ])))
            });

        cx.update_editor(|editor, cx| {
            go_to_fetched_type_definition(editor, PointForPosition::valid(hover_point), false, cx);
        });
        requests.next().await;
        cx.foreground().run_until_parked();

        cx.assert_editor_state(indoc! {"
            struct «Aˇ»;
            let variable = A;
        "});
    }

    #[gpui::test]
    async fn test_link_go_to_definition(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            fn ˇtest() { do_work(); }
            fn do_work() { test(); }
        "});

        // Basic hold cmd, expect highlight in region if response contains definition
        let hover_point = cx.display_point(indoc! {"
            fn test() { do_wˇork(); }
            fn do_work() { test(); }
        "});
        let symbol_range = cx.lsp_range(indoc! {"
            fn test() { «do_work»(); }
            fn do_work() { test(); }
        "});
        let target_range = cx.lsp_range(indoc! {"
            fn test() { do_work(); }
            fn «do_work»() { test(); }
        "});

        let mut requests = cx.handle_request::<GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range: Some(symbol_range),
                    target_uri: url.clone(),
                    target_range,
                    target_selection_range: target_range,
                },
            ])))
        });

        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        requests.next().await;
        cx.foreground().run_until_parked();
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { «do_work»(); }
            fn do_work() { test(); }
        "});

        // Unpress cmd causes highlight to go away
        cx.update_editor(|editor, cx| {
            editor.modifiers_changed(&Default::default(), cx);
        });

        // Assert no link highlights
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});

        // Response without source range still highlights word
        cx.update_editor(|editor, _| editor.link_go_to_definition_state.last_trigger_point = None);
        let mut requests = cx.handle_request::<GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    // No origin range
                    origin_selection_range: None,
                    target_uri: url.clone(),
                    target_range,
                    target_selection_range: target_range,
                },
            ])))
        });
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        requests.next().await;
        cx.foreground().run_until_parked();

        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { «do_work»(); }
            fn do_work() { test(); }
        "});

        // Moving mouse to location with no response dismisses highlight
        let hover_point = cx.display_point(indoc! {"
            fˇn test() { do_work(); }
            fn do_work() { test(); }
        "});
        let mut requests = cx
            .lsp
            .handle_request::<GotoDefinition, _, _>(move |_, _| async move {
                // No definitions returned
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        requests.next().await;
        cx.foreground().run_until_parked();

        // Assert no link highlights
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});

        // Move mouse without cmd and then pressing cmd triggers highlight
        let hover_point = cx.display_point(indoc! {"
            fn test() { do_work(); }
            fn do_work() { teˇst(); }
        "});
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                false,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();

        // Assert no link highlights
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});

        let symbol_range = cx.lsp_range(indoc! {"
            fn test() { do_work(); }
            fn do_work() { «test»(); }
        "});
        let target_range = cx.lsp_range(indoc! {"
            fn «test»() { do_work(); }
            fn do_work() { test(); }
        "});

        let mut requests = cx.handle_request::<GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range: Some(symbol_range),
                    target_uri: url,
                    target_range,
                    target_selection_range: target_range,
                },
            ])))
        });
        cx.update_editor(|editor, cx| {
            editor.modifiers_changed(
                &ModifiersChangedEvent {
                    modifiers: Modifiers {
                        cmd: true,
                        ..Default::default()
                    },
                },
                cx,
            );
        });
        requests.next().await;
        cx.foreground().run_until_parked();

        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { «test»(); }
        "});

        // Deactivating the window dismisses the highlight
        cx.update_workspace(|workspace, cx| {
            workspace.on_window_activation_changed(false, cx);
        });
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});

        // Moving the mouse restores the highlights.
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { «test»(); }
        "});

        // Moving again within the same symbol range doesn't re-request
        let hover_point = cx.display_point(indoc! {"
            fn test() { do_work(); }
            fn do_work() { tesˇt(); }
        "});
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { «test»(); }
        "});

        // Cmd click with existing definition doesn't re-request and dismisses highlight
        cx.update_editor(|editor, cx| {
            go_to_fetched_definition(editor, PointForPosition::valid(hover_point), false, cx);
        });
        // Assert selection moved to to definition
        cx.lsp
            .handle_request::<GotoDefinition, _, _>(move |_, _| async move {
                // Empty definition response to make sure we aren't hitting the lsp and using
                // the cached location instead
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
        cx.foreground().run_until_parked();
        cx.assert_editor_state(indoc! {"
            fn «testˇ»() { do_work(); }
            fn do_work() { test(); }
        "});

        // Assert no link highlights after jump
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});

        // Cmd click without existing definition requests and jumps
        let hover_point = cx.display_point(indoc! {"
            fn test() { do_wˇork(); }
            fn do_work() { test(); }
        "});
        let target_range = cx.lsp_range(indoc! {"
            fn test() { do_work(); }
            fn «do_work»() { test(); }
        "});

        let mut requests = cx.handle_request::<GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range: None,
                    target_uri: url,
                    target_range,
                    target_selection_range: target_range,
                },
            ])))
        });
        cx.update_editor(|editor, cx| {
            go_to_fetched_definition(editor, PointForPosition::valid(hover_point), false, cx);
        });
        requests.next().await;
        cx.foreground().run_until_parked();
        cx.assert_editor_state(indoc! {"
            fn test() { do_work(); }
            fn «do_workˇ»() { test(); }
        "});

        // 1. We have a pending selection, mouse point is over a symbol that we have a response for, hitting cmd and nothing happens
        // 2. Selection is completed, hovering
        let hover_point = cx.display_point(indoc! {"
            fn test() { do_wˇork(); }
            fn do_work() { test(); }
        "});
        let target_range = cx.lsp_range(indoc! {"
            fn test() { do_work(); }
            fn «do_work»() { test(); }
        "});
        let mut requests = cx.handle_request::<GotoDefinition, _, _>(move |url, _, _| async move {
            Ok(Some(lsp::GotoDefinitionResponse::Link(vec![
                lsp::LocationLink {
                    origin_selection_range: None,
                    target_uri: url,
                    target_range,
                    target_selection_range: target_range,
                },
            ])))
        });

        // create a pending selection
        let selection_range = cx.ranges(indoc! {"
            fn «test() { do_w»ork(); }
            fn do_work() { test(); }
        "})[0]
            .clone();
        cx.update_editor(|editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let anchor_range = snapshot.anchor_before(selection_range.start)
                ..snapshot.anchor_after(selection_range.end);
            editor.change_selections(Some(crate::Autoscroll::fit()), cx, |s| {
                s.set_pending_anchor_range(anchor_range, crate::SelectMode::Character)
            });
        });
        cx.update_editor(|editor, cx| {
            update_go_to_definition_link(
                editor,
                Some(GoToDefinitionTrigger::Text(hover_point)),
                true,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();
        assert!(requests.try_next().is_err());
        cx.assert_editor_text_highlights::<LinkGoToDefinitionState>(indoc! {"
            fn test() { do_work(); }
            fn do_work() { test(); }
        "});
        cx.foreground().run_until_parked();
    }

    #[gpui::test]
    async fn test_link_go_to_inlay(cx: &mut gpui::TestAppContext) {
        init_test(cx, |settings| {
            settings.defaults.inlay_hints = Some(InlayHintSettings {
                enabled: true,
                show_type_hints: true,
                show_parameter_hints: true,
                show_other_hints: true,
            })
        });

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            cx,
        )
        .await;
        cx.set_state(indoc! {"
            struct TestStruct;

            fn main() {
                let variableˇ = TestStruct;
            }
        "});
        let hint_start_offset = cx.ranges(indoc! {"
            struct TestStruct;

            fn main() {
                let variableˇ = TestStruct;
            }
        "})[0]
            .start;
        let hint_position = cx.to_lsp(hint_start_offset);
        let target_range = cx.lsp_range(indoc! {"
            struct «TestStruct»;

            fn main() {
                let variable = TestStruct;
            }
        "});

        let expected_uri = cx.buffer_lsp_url.clone();
        let hint_label = ": TestStruct";
        cx.lsp
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let expected_uri = expected_uri.clone();
                async move {
                    assert_eq!(params.text_document.uri, expected_uri);
                    Ok(Some(vec![lsp::InlayHint {
                        position: hint_position,
                        label: lsp::InlayHintLabel::LabelParts(vec![lsp::InlayHintLabelPart {
                            value: hint_label.to_string(),
                            location: Some(lsp::Location {
                                uri: params.text_document.uri,
                                range: target_range,
                            }),
                            ..Default::default()
                        }]),
                        kind: Some(lsp::InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: None,
                        padding_left: Some(false),
                        padding_right: Some(false),
                        data: None,
                    }]))
                }
            })
            .next()
            .await;
        cx.foreground().run_until_parked();
        cx.update_editor(|editor, cx| {
            let expected_layers = vec![hint_label.to_string()];
            assert_eq!(expected_layers, cached_hint_labels(editor));
            assert_eq!(expected_layers, visible_hint_labels(editor, cx));
        });

        let inlay_range = cx
            .ranges(indoc! {"
            struct TestStruct;

            fn main() {
                let variable« »= TestStruct;
            }
        "})
            .get(0)
            .cloned()
            .unwrap();
        let hint_hover_position = cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let previous_valid = inlay_range.start.to_display_point(&snapshot);
            let next_valid = inlay_range.end.to_display_point(&snapshot);
            assert_eq!(previous_valid.row(), next_valid.row());
            assert!(previous_valid.column() < next_valid.column());
            let exact_unclipped = DisplayPoint::new(
                previous_valid.row(),
                previous_valid.column() + (hint_label.len() / 2) as u32,
            );
            PointForPosition {
                previous_valid,
                next_valid,
                exact_unclipped,
                column_overshoot_after_line_end: 0,
            }
        });
        // Press cmd to trigger highlight
        cx.update_editor(|editor, cx| {
            update_inlay_link_and_hover_points(
                &editor.snapshot(cx),
                hint_hover_position,
                editor,
                true,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();
        cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let actual_highlights = snapshot
                .inlay_highlights::<LinkGoToDefinitionState>()
                .into_iter()
                .flat_map(|highlights| highlights.values().map(|(_, highlight)| highlight))
                .collect::<Vec<_>>();

            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let expected_highlight = InlayHighlight {
                inlay: InlayId::Hint(0),
                inlay_position: buffer_snapshot.anchor_at(inlay_range.start, Bias::Right),
                range: 0..hint_label.len(),
            };
            assert_set_eq!(actual_highlights, vec![&expected_highlight]);
        });

        // Unpress cmd causes highlight to go away
        cx.update_editor(|editor, cx| {
            editor.modifiers_changed(
                &platform::ModifiersChangedEvent {
                    modifiers: Modifiers {
                        cmd: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                cx,
            );
        });
        // Assert no link highlights
        cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let actual_ranges = snapshot
                .text_highlight_ranges::<LinkGoToDefinitionState>()
                .map(|ranges| ranges.as_ref().clone().1)
                .unwrap_or_default();

            assert!(actual_ranges.is_empty(), "When no cmd is pressed, should have no hint label selected, but got: {actual_ranges:?}");
        });

        // Cmd+click without existing definition requests and jumps
        cx.update_editor(|editor, cx| {
            editor.modifiers_changed(
                &platform::ModifiersChangedEvent {
                    modifiers: Modifiers {
                        cmd: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                cx,
            );
            update_inlay_link_and_hover_points(
                &editor.snapshot(cx),
                hint_hover_position,
                editor,
                true,
                false,
                cx,
            );
        });
        cx.foreground().run_until_parked();
        cx.update_editor(|editor, cx| {
            go_to_fetched_type_definition(editor, hint_hover_position, false, cx);
        });
        cx.foreground().run_until_parked();
        cx.assert_editor_state(indoc! {"
            struct «TestStructˇ»;

            fn main() {
                let variable = TestStruct;
            }
        "});
    }
}
