use crate::{Anchor, DisplayPoint, Editor, EditorSnapshot, SelectPhase};
use gpui::{Task, ViewContext};
use language::{Bias, ToOffset};
use project::LocationLink;
use std::ops::Range;
use util::TryFutureExt;

#[derive(Debug, Default)]
pub struct LinkGoToDefinitionState {
    pub last_trigger_point: Option<TriggerPoint>,
    pub symbol_range: Option<Range<Anchor>>,
    pub kind: Option<LinkDefinitionKind>,
    pub definitions: Vec<LocationLink>,
    pub task: Option<Task<Option<()>>>,
}

pub enum GoToDefinitionTrigger {
    Text(DisplayPoint),
    InlayHint(Anchor, LocationLink),
    None,
}

#[derive(Debug, Clone)]
pub enum TriggerPoint {
    Text(Anchor),
    InlayHint(Anchor, LocationLink),
}

impl TriggerPoint {
    fn anchor(&self) -> &Anchor {
        match self {
            TriggerPoint::Text(anchor) => anchor,
            TriggerPoint::InlayHint(anchor, _) => anchor,
        }
    }

    pub fn definition_kind(&self, shift: bool) -> LinkDefinitionKind {
        match self {
            TriggerPoint::Text(_) => {
                if shift {
                    LinkDefinitionKind::Type
                } else {
                    LinkDefinitionKind::Symbol
                }
            }
            TriggerPoint::InlayHint(_, link) => LinkDefinitionKind::Type,
        }
    }
}

pub fn update_go_to_definition_link(
    editor: &mut Editor,
    origin: GoToDefinitionTrigger,
    cmd_held: bool,
    shift_held: bool,
    cx: &mut ViewContext<Editor>,
) {
    let pending_nonempty_selection = editor.has_pending_nonempty_selection();

    // Store new mouse point as an anchor
    let snapshot = editor.snapshot(cx);
    let trigger_point = match origin {
        GoToDefinitionTrigger::Text(p) => {
            Some(TriggerPoint::Text(snapshot.buffer_snapshot.anchor_before(
                p.to_offset(&snapshot.display_snapshot, Bias::Left),
            )))
        }
        GoToDefinitionTrigger::InlayHint(p, target) => Some(TriggerPoint::InlayHint(p, target)),
        GoToDefinitionTrigger::None => None,
    };

    // If the new point is the same as the previously stored one, return early
    if let (Some(a), Some(b)) = (
        &trigger_point,
        &editor.link_go_to_definition_state.last_trigger_point,
    ) {
        if a.anchor()
            .cmp(b.anchor(), &snapshot.buffer_snapshot)
            .is_eq()
        {
            return;
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

    let trigger_anchor = trigger_point.anchor().clone();
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
        let point_after_start = symbol_range
            .start
            .cmp(&trigger_anchor, &snapshot.buffer_snapshot)
            .is_le();

        let point_before_end = symbol_range
            .end
            .cmp(&trigger_anchor, &snapshot.buffer_snapshot)
            .is_ge();

        let point_within_range = point_after_start && point_before_end;
        if point_within_range && same_kind {
            return;
        }
    }

    let task = cx.spawn(|this, mut cx| {
        async move {
            let result = match trigger_point {
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

                                    start..end
                                })
                            }),
                            definition_result,
                        )
                    })
                }
                TriggerPoint::InlayHint(trigger_source, trigger_target) => {
                    // TODO kb range is wrong, should be in inlay coordinates
                    Some((Some(trigger_source..trigger_source), vec![trigger_target]))
                }
            };

            this.update(&mut cx, |this, cx| {
                // Clear any existing highlights
                this.clear_text_highlights::<LinkGoToDefinitionState>(cx);
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
                            let target = &definition.target;
                            if target.buffer == buffer {
                                let range = &target.range;
                                // Expand range by one character as lsp definition ranges include positions adjacent
                                // but not contained by the symbol range
                                let start = buffer_snapshot.clip_offset(
                                    range.start.to_offset(&buffer_snapshot).saturating_sub(1),
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
                        });

                    if any_definition_does_not_contain_current_location {
                        // If no symbol range returned from language server, use the surrounding word.
                        let highlight_range = symbol_range.unwrap_or_else(|| {
                            let snapshot = &snapshot.buffer_snapshot;
                            let (offset_range, _) = snapshot.surrounding_word(trigger_anchor);

                            snapshot.anchor_before(offset_range.start)
                                ..snapshot.anchor_after(offset_range.end)
                        });

                        // Highlight symbol using theme link definition highlight style
                        let style = theme::current(cx).editor.link_definition;
                        this.highlight_text::<LinkGoToDefinitionState>(
                            vec![highlight_range],
                            style,
                            cx,
                        );
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

    editor.clear_text_highlights::<LinkGoToDefinitionState>(cx);
}

pub fn go_to_fetched_definition(
    editor: &mut Editor,
    point: DisplayPoint,
    split: bool,
    cx: &mut ViewContext<Editor>,
) {
    go_to_fetched_definition_of_kind(LinkDefinitionKind::Symbol, editor, point, split, cx);
}

pub fn go_to_fetched_type_definition(
    editor: &mut Editor,
    point: DisplayPoint,
    split: bool,
    cx: &mut ViewContext<Editor>,
) {
    go_to_fetched_definition_of_kind(LinkDefinitionKind::Type, editor, point, split, cx);
}

fn go_to_fetched_definition_of_kind(
    kind: LinkDefinitionKind,
    editor: &mut Editor,
    point: DisplayPoint,
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
                position: point,
                add: false,
                click_count: 1,
            },
            cx,
        );

        match kind {
            LinkDefinitionKind::Symbol => editor.go_to_definition(&Default::default(), cx),
            LinkDefinitionKind::Type => editor.go_to_type_definition(&Default::default(), cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{editor_tests::init_test, test::editor_lsp_test_context::EditorLspTestContext};
    use futures::StreamExt;
    use gpui::{
        platform::{self, Modifiers, ModifiersChangedEvent},
        View,
    };
    use indoc::indoc;
    use lsp::request::{GotoDefinition, GotoTypeDefinition};

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
                GoToDefinitionTrigger::Text(hover_point),
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
            go_to_fetched_type_definition(editor, hover_point, false, cx);
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
                GoToDefinitionTrigger::Text(hover_point),
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
                GoToDefinitionTrigger::Text(hover_point),
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
                GoToDefinitionTrigger::Text(hover_point),
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
                GoToDefinitionTrigger::Text(hover_point),
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
                GoToDefinitionTrigger::Text(hover_point),
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
                GoToDefinitionTrigger::Text(hover_point),
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
            go_to_fetched_definition(editor, hover_point, false, cx);
        });
        // Assert selection moved to to definition
        cx.lsp
            .handle_request::<GotoDefinition, _, _>(move |_, _| async move {
                // Empty definition response to make sure we aren't hitting the lsp and using
                // the cached location instead
                Ok(Some(lsp::GotoDefinitionResponse::Link(vec![])))
            });
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
            go_to_fetched_definition(editor, hover_point, false, cx);
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
                GoToDefinitionTrigger::Text(hover_point),
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
}
