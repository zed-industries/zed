use crate::{
    element::PointForPosition,
    hover_popover::{self, InlayHover},
    Anchor, Editor, EditorSnapshot, GoToDefinition, GoToTypeDefinition, InlayId, SelectPhase,
};
use gpui::{px, AsyncWindowContext, Model, Modifiers, Task, ViewContext};
use language::{Bias, ToOffset};
use linkify::{LinkFinder, LinkKind};
use lsp::LanguageServerId;
use project::{
    HoverBlock, HoverBlockKind, InlayHintLabelPartTooltip, InlayHintTooltip, LocationLink,
    ResolveState,
};
use std::ops::Range;
use theme::ActiveTheme as _;
use util::TryFutureExt;

#[derive(Debug)]
pub struct HoveredLinkState {
    pub last_trigger_point: TriggerPoint,
    pub preferred_kind: LinkDefinitionKind,
    pub symbol_range: Option<RangeInEditor>,
    pub links: Vec<HoverLink>,
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

#[derive(Debug, Clone)]
pub enum HoverLink {
    Url(String),
    Text(LocationLink),
    InlayHint(lsp::Location, LanguageServerId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InlayHighlight {
    pub inlay: InlayId,
    pub inlay_position: Anchor,
    pub range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerPoint {
    Text(Anchor),
    InlayHint(InlayHighlight, lsp::Location, LanguageServerId),
}

impl TriggerPoint {
    fn anchor(&self) -> &Anchor {
        match self {
            TriggerPoint::Text(anchor) => anchor,
            TriggerPoint::InlayHint(inlay_range, _, _) => &inlay_range.inlay_position,
        }
    }
}

impl Editor {
    pub(crate) fn update_hovered_link(
        &mut self,
        point_for_position: PointForPosition,
        snapshot: &EditorSnapshot,
        modifiers: Modifiers,
        cx: &mut ViewContext<Self>,
    ) {
        if !modifiers.command || self.has_pending_selection() {
            self.hide_hovered_link(cx);
            return;
        }

        match point_for_position.as_valid() {
            Some(point) => {
                let trigger_point = TriggerPoint::Text(
                    snapshot
                        .buffer_snapshot
                        .anchor_before(point.to_offset(&snapshot.display_snapshot, Bias::Left)),
                );

                show_link_definition(modifiers.shift, self, trigger_point, snapshot, cx);
            }
            None => {
                update_inlay_link_and_hover_points(
                    &snapshot,
                    point_for_position,
                    self,
                    modifiers.command,
                    modifiers.shift,
                    cx,
                );
            }
        }
    }

    pub(crate) fn hide_hovered_link(&mut self, cx: &mut ViewContext<Self>) {
        self.hovered_link_state.take();
        self.clear_highlights::<HoveredLinkState>(cx);
    }

    pub(crate) fn handle_click_hovered_link(
        &mut self,
        point: PointForPosition,
        modifiers: Modifiers,
        cx: &mut ViewContext<Editor>,
    ) {
        if let Some(hovered_link_state) = self.hovered_link_state.take() {
            self.hide_hovered_link(cx);
            if !hovered_link_state.links.is_empty() {
                if !self.focus_handle.is_focused(cx) {
                    cx.focus(&self.focus_handle);
                }

                self.navigate_to_hover_links(None, hovered_link_state.links, modifiers.alt, cx);
                return;
            }
        }

        // We don't have the correct kind of link cached, set the selection on
        // click and immediately trigger GoToDefinition.
        self.select(
            SelectPhase::Begin {
                position: point.next_valid,
                add: false,
                click_count: 1,
            },
            cx,
        );

        if point.as_valid().is_some() {
            if modifiers.shift {
                self.go_to_type_definition(&GoToTypeDefinition, cx)
            } else {
                self.go_to_definition(&GoToDefinition, cx)
            }
        }
    }
}

pub fn update_inlay_link_and_hover_points(
    snapshot: &EditorSnapshot,
    point_for_position: PointForPosition,
    editor: &mut Editor,
    cmd_held: bool,
    shift_held: bool,
    cx: &mut ViewContext<'_, Editor>,
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
                                        if cmd_held && !editor.has_pending_nonempty_selection() {
                                            go_to_definition_updated = true;
                                            show_link_definition(
                                                shift_held,
                                                editor,
                                                TriggerPoint::InlayHint(
                                                    highlight,
                                                    location,
                                                    language_server_id,
                                                ),
                                                snapshot,
                                                cx,
                                            );
                                        }
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
        editor.hide_hovered_link(cx)
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
    shift_held: bool,
    editor: &mut Editor,
    trigger_point: TriggerPoint,
    snapshot: &EditorSnapshot,
    cx: &mut ViewContext<Editor>,
) {
    let preferred_kind = match trigger_point {
        TriggerPoint::Text(_) if !shift_held => LinkDefinitionKind::Symbol,
        _ => LinkDefinitionKind::Type,
    };

    let (mut hovered_link_state, is_cached) =
        if let Some(existing) = editor.hovered_link_state.take() {
            (existing, true)
        } else {
            (
                HoveredLinkState {
                    last_trigger_point: trigger_point.clone(),
                    symbol_range: None,
                    preferred_kind,
                    links: vec![],
                    task: None,
                },
                false,
            )
        };

    if editor.pending_rename.is_some() {
        return;
    }

    let trigger_anchor = trigger_point.anchor();
    let Some((buffer, buffer_position)) = editor
        .buffer
        .read(cx)
        .text_anchor_for_position(trigger_anchor.clone(), cx)
    else {
        return;
    };

    let Some((excerpt_id, _, _)) = editor
        .buffer()
        .read(cx)
        .excerpt_containing(trigger_anchor.clone(), cx)
    else {
        return;
    };

    let same_kind = hovered_link_state.preferred_kind == preferred_kind
        || hovered_link_state
            .links
            .first()
            .is_some_and(|d| matches!(d, HoverLink::Url(_)));

    if same_kind {
        if is_cached && (&hovered_link_state.last_trigger_point == &trigger_point)
            || hovered_link_state
                .symbol_range
                .as_ref()
                .is_some_and(|symbol_range| {
                    symbol_range.point_within_range(&trigger_point, &snapshot)
                })
        {
            editor.hovered_link_state = Some(hovered_link_state);
            return;
        }
    } else {
        editor.hide_hovered_link(cx)
    }
    let project = editor.project.clone();

    let snapshot = snapshot.buffer_snapshot.clone();
    hovered_link_state.task = Some(cx.spawn(|this, mut cx| {
        async move {
            let result = match &trigger_point {
                TriggerPoint::Text(_) => {
                    if let Some((url_range, url)) = find_url(&buffer, buffer_position, cx.clone()) {
                        this.update(&mut cx, |_, _| {
                            let start =
                                snapshot.anchor_in_excerpt(excerpt_id.clone(), url_range.start);
                            let end = snapshot.anchor_in_excerpt(excerpt_id.clone(), url_range.end);
                            (
                                Some(RangeInEditor::Text(start..end)),
                                vec![HoverLink::Url(url)],
                            )
                        })
                        .ok()
                    } else if let Some(project) = project {
                        // query the LSP for definition info
                        project
                            .update(&mut cx, |project, cx| match preferred_kind {
                                LinkDefinitionKind::Symbol => {
                                    project.definition(&buffer, buffer_position, cx)
                                }

                                LinkDefinitionKind::Type => {
                                    project.type_definition(&buffer, buffer_position, cx)
                                }
                            })?
                            .await
                            .ok()
                            .map(|definition_result| {
                                (
                                    definition_result.iter().find_map(|link| {
                                        link.origin.as_ref().map(|origin| {
                                            let start = snapshot.anchor_in_excerpt(
                                                excerpt_id.clone(),
                                                origin.range.start,
                                            );
                                            let end = snapshot.anchor_in_excerpt(
                                                excerpt_id.clone(),
                                                origin.range.end,
                                            );
                                            RangeInEditor::Text(start..end)
                                        })
                                    }),
                                    definition_result.into_iter().map(HoverLink::Text).collect(),
                                )
                            })
                    } else {
                        None
                    }
                }
                TriggerPoint::InlayHint(highlight, lsp_location, server_id) => Some((
                    Some(RangeInEditor::Inlay(highlight.clone())),
                    vec![HoverLink::InlayHint(lsp_location.clone(), *server_id)],
                )),
            };

            this.update(&mut cx, |this, cx| {
                // Clear any existing highlights
                this.clear_highlights::<HoveredLinkState>(cx);
                let Some(hovered_link_state) = this.hovered_link_state.as_mut() else {
                    return;
                };
                hovered_link_state.preferred_kind = preferred_kind;
                hovered_link_state.symbol_range = result
                    .as_ref()
                    .and_then(|(symbol_range, _)| symbol_range.clone());

                if let Some((symbol_range, definitions)) = result {
                    hovered_link_state.links = definitions.clone();

                    let buffer_snapshot = buffer.read(cx).snapshot();

                    // Only show highlight if there exists a definition to jump to that doesn't contain
                    // the current location.
                    let any_definition_does_not_contain_current_location =
                        definitions.iter().any(|definition| {
                            match &definition {
                                HoverLink::Text(link) => {
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
                                HoverLink::InlayHint(_, _) => true,
                                HoverLink::Url(_) => true,
                            }
                        });

                    if any_definition_does_not_contain_current_location {
                        let style = gpui::HighlightStyle {
                            underline: Some(gpui::UnderlineStyle {
                                thickness: px(1.),
                                ..Default::default()
                            }),
                            color: Some(cx.theme().colors().link_text_hover),
                            ..Default::default()
                        };
                        let highlight_range =
                            symbol_range.unwrap_or_else(|| match &trigger_point {
                                TriggerPoint::Text(trigger_anchor) => {
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
                            RangeInEditor::Text(text_range) => {
                                this.highlight_text::<HoveredLinkState>(vec![text_range], style, cx)
                            }
                            RangeInEditor::Inlay(highlight) => this
                                .highlight_inlays::<HoveredLinkState>(vec![highlight], style, cx),
                        }
                    } else {
                        this.hide_hovered_link(cx);
                    }
                }
            })?;

            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    }));

    editor.hovered_link_state = Some(hovered_link_state);
}

pub(crate) fn find_url(
    buffer: &Model<language::Buffer>,
    position: text::Anchor,
    mut cx: AsyncWindowContext,
) -> Option<(Range<text::Anchor>, String)> {
    const LIMIT: usize = 2048;

    let Ok(snapshot) = buffer.update(&mut cx, |buffer, _| buffer.snapshot()) else {
        return None;
    };

    let offset = position.to_offset(&snapshot);
    let mut token_start = offset;
    let mut token_end = offset;
    let mut found_start = false;
    let mut found_end = false;

    for ch in snapshot.reversed_chars_at(offset).take(LIMIT) {
        if ch.is_whitespace() {
            found_start = true;
            break;
        }
        token_start -= ch.len_utf8();
    }
    if !found_start {
        return None;
    }

    for ch in snapshot
        .chars_at(offset)
        .take(LIMIT - (offset - token_start))
    {
        if ch.is_whitespace() {
            found_end = true;
            break;
        }
        token_end += ch.len_utf8();
    }
    if !found_end {
        return None;
    }

    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    let input = snapshot
        .text_for_range(token_start..token_end)
        .collect::<String>();

    let relative_offset = offset - token_start;
    for link in finder.links(&input) {
        if link.start() <= relative_offset && link.end() >= relative_offset {
            let range = snapshot.anchor_before(token_start + link.start())
                ..snapshot.anchor_after(token_start + link.end());
            return Some((range, link.as_str().to_string()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        display_map::ToDisplayPoint,
        editor_tests::init_test,
        inlay_hint_cache::tests::{cached_hint_labels, visible_hint_labels},
        test::editor_lsp_test_context::EditorLspTestContext,
        DisplayPoint,
    };
    use futures::StreamExt;
    use gpui::Modifiers;
    use indoc::indoc;
    use language::language_settings::InlayHintSettings;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};
    use util::assert_set_eq;
    use workspace::item::Item;

    #[gpui::test]
    async fn test_hover_type_links(cx: &mut gpui::TestAppContext) {
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
        let screen_coord = cx.editor(|editor, cx| editor.pixel_position_of_cursor(cx));

        // Basic hold cmd+shift, expect highlight in region if response contains type definition
        let symbol_range = cx.lsp_range(indoc! {"
            struct A;
            let «variable» = A;
        "});
        let target_range = cx.lsp_range(indoc! {"
            struct «A»;
            let variable = A;
        "});

        cx.run_until_parked();

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

        cx.cx
            .cx
            .simulate_mouse_move(screen_coord.unwrap(), Modifiers::command_shift());

        requests.next().await;
        cx.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            struct A;
            let «variable» = A;
        "});

        cx.simulate_modifiers_change(Modifiers::command());
        cx.run_until_parked();
        // Assert no link highlights
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            struct A;
            let variable = A;
        "});

        cx.cx
            .cx
            .simulate_click(screen_coord.unwrap(), Modifiers::command_shift());

        cx.assert_editor_state(indoc! {"
            struct «Aˇ»;
            let variable = A;
        "});
    }

    #[gpui::test]
    async fn test_hover_links(cx: &mut gpui::TestAppContext) {
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
        let hover_point = cx.pixel_position(indoc! {"
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

        cx.simulate_mouse_move(hover_point, Modifiers::command());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "});

        // Unpress cmd causes highlight to go away
        cx.simulate_modifiers_change(Modifiers::none());
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
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

        cx.simulate_mouse_move(hover_point, Modifiers::command());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { «do_work»(); }
                fn do_work() { test(); }
            "});

        // Moving mouse to location with no response dismisses highlight
        let hover_point = cx.pixel_position(indoc! {"
                fˇn test() { do_work(); }
                fn do_work() { test(); }
            "});
        let mut requests = cx
            .lsp
            .handle_request::<GotoDefinition, _, _>(move |_, _| async move {
                // No definitions returned
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
        cx.simulate_mouse_move(hover_point, Modifiers::command());

        requests.next().await;
        cx.background_executor.run_until_parked();

        // Assert no link highlights
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

        // // Move mouse without cmd and then pressing cmd triggers highlight
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { teˇst(); }
            "});
        cx.simulate_mouse_move(hover_point, Modifiers::none());

        // Assert no link highlights
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
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

        cx.simulate_modifiers_change(Modifiers::command());

        requests.next().await;
        cx.background_executor.run_until_parked();

        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

        cx.deactivate_window();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

        cx.simulate_mouse_move(hover_point, Modifiers::command());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

        // Moving again within the same symbol range doesn't re-request
        let hover_point = cx.pixel_position(indoc! {"
                fn test() { do_work(); }
                fn do_work() { tesˇt(); }
            "});
        cx.simulate_mouse_move(hover_point, Modifiers::command());
        cx.background_executor.run_until_parked();
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { «test»(); }
            "});

        // Cmd click with existing definition doesn't re-request and dismisses highlight
        cx.simulate_click(hover_point, Modifiers::command());
        cx.lsp
            .handle_request::<GotoDefinition, _, _>(move |_, _| async move {
                // Empty definition response to make sure we aren't hitting the lsp and using
                // the cached location instead
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                fn «testˇ»() { do_work(); }
                fn do_work() { test(); }
            "});

        // Assert no link highlights after jump
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});

        // Cmd click without existing definition requests and jumps
        let hover_point = cx.pixel_position(indoc! {"
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
        cx.simulate_click(hover_point, Modifiers::command());
        requests.next().await;
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                fn test() { do_work(); }
                fn «do_workˇ»() { test(); }
            "});

        // 1. We have a pending selection, mouse point is over a symbol that we have a response for, hitting cmd and nothing happens
        // 2. Selection is completed, hovering
        let hover_point = cx.pixel_position(indoc! {"
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
        cx.simulate_mouse_move(hover_point, Modifiers::command());
        cx.background_executor.run_until_parked();
        assert!(requests.try_next().is_err());
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
                fn test() { do_work(); }
                fn do_work() { test(); }
            "});
        cx.background_executor.run_until_parked();
    }

    #[gpui::test]
    async fn test_inlay_hover_links(cx: &mut gpui::TestAppContext) {
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
        cx.background_executor.run_until_parked();
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
        let midpoint = cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let previous_valid = inlay_range.start.to_display_point(&snapshot);
            let next_valid = inlay_range.end.to_display_point(&snapshot);
            assert_eq!(previous_valid.row(), next_valid.row());
            assert!(previous_valid.column() < next_valid.column());
            DisplayPoint::new(
                previous_valid.row(),
                previous_valid.column() + (hint_label.len() / 2) as u32,
            )
        });
        // Press cmd to trigger highlight
        let hover_point = cx.pixel_position_for(midpoint);
        cx.simulate_mouse_move(hover_point, Modifiers::command());
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let actual_highlights = snapshot
                .inlay_highlights::<HoveredLinkState>()
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

        cx.simulate_mouse_move(hover_point, Modifiers::none());
        // Assert no link highlights
        cx.update_editor(|editor, cx| {
                let snapshot = editor.snapshot(cx);
                let actual_ranges = snapshot
                    .text_highlight_ranges::<HoveredLinkState>()
                    .map(|ranges| ranges.as_ref().clone().1)
                    .unwrap_or_default();

                assert!(actual_ranges.is_empty(), "When no cmd is pressed, should have no hint label selected, but got: {actual_ranges:?}");
            });

        cx.simulate_modifiers_change(Modifiers::command());
        cx.background_executor.run_until_parked();
        cx.simulate_click(hover_point, Modifiers::command());
        cx.background_executor.run_until_parked();
        cx.assert_editor_state(indoc! {"
                struct «TestStructˇ»;

                fn main() {
                    let variable = TestStruct;
                }
            "});
    }

    #[gpui::test]
    async fn test_urls(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            Let's test a [complex](https://zed.dev/channel/had-(oops)) caseˇ.
        "});

        let screen_coord = cx.pixel_position(indoc! {"
            Let's test a [complex](https://zed.dev/channel/had-(ˇoops)) case.
            "});

        cx.simulate_mouse_move(screen_coord, Modifiers::command());
        cx.assert_editor_text_highlights::<HoveredLinkState>(indoc! {"
            Let's test a [complex](«https://zed.dev/channel/had-(oops)ˇ») case.
        "});

        cx.simulate_click(screen_coord, Modifiers::command());
        assert_eq!(
            cx.opened_url(),
            Some("https://zed.dev/channel/had-(oops)".into())
        );
    }
}
