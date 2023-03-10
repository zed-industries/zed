use futures::FutureExt;
use gpui::{
    actions,
    elements::{Flex, MouseEventHandler, Padding, Text},
    impl_internal_actions,
    platform::CursorStyle,
    Axis, Element, ElementBox, ModelHandle, MouseButton, MutableAppContext, RenderContext, Task,
    ViewContext,
};
use language::{Bias, DiagnosticEntry, DiagnosticSeverity};
use project::{HoverBlock, Project};
use settings::Settings;
use std::{ops::Range, time::Duration};
use util::TryFutureExt;

use crate::{
    display_map::ToDisplayPoint, Anchor, AnchorRangeExt, DisplayPoint, Editor, EditorSnapshot,
    EditorStyle, GoToDiagnostic, RangeToAnchorExt,
};

pub const HOVER_DELAY_MILLIS: u64 = 350;
pub const HOVER_REQUEST_DELAY_MILLIS: u64 = 200;

pub const MIN_POPOVER_CHARACTER_WIDTH: f32 = 20.;
pub const MIN_POPOVER_LINE_HEIGHT: f32 = 4.;
pub const HOVER_POPOVER_GAP: f32 = 10.;

#[derive(Clone, PartialEq)]
pub struct HoverAt {
    pub point: Option<DisplayPoint>,
}

#[derive(Copy, Clone, PartialEq)]
pub struct HideHover;

actions!(editor, [Hover]);
impl_internal_actions!(editor, [HoverAt, HideHover]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(hover);
    cx.add_action(hover_at);
    cx.add_action(hide_hover);
}

/// Bindable action which uses the most recent selection head to trigger a hover
pub fn hover(editor: &mut Editor, _: &Hover, cx: &mut ViewContext<Editor>) {
    let head = editor.selections.newest_display(cx).head();
    show_hover(editor, head, true, cx);
}

/// The internal hover action dispatches between `show_hover` or `hide_hover`
/// depending on whether a point to hover over is provided.
pub fn hover_at(editor: &mut Editor, action: &HoverAt, cx: &mut ViewContext<Editor>) {
    if cx.global::<Settings>().hover_popover_enabled {
        if let Some(point) = action.point {
            show_hover(editor, point, false, cx);
        } else {
            hide_hover(editor, &HideHover, cx);
        }
    }
}

/// Hides the type information popup.
/// Triggered by the `Hover` action when the cursor is not over a symbol or when the
/// selections changed.
pub fn hide_hover(editor: &mut Editor, _: &HideHover, cx: &mut ViewContext<Editor>) -> bool {
    let did_hide = editor.hover_state.info_popover.take().is_some()
        | editor.hover_state.diagnostic_popover.take().is_some();

    editor.hover_state.info_task = None;
    editor.hover_state.triggered_from = None;

    editor.clear_background_highlights::<HoverState>(cx);

    if did_hide {
        cx.notify();
    }

    did_hide
}

/// Queries the LSP and shows type info and documentation
/// about the symbol the mouse is currently hovering over.
/// Triggered by the `Hover` action when the cursor may be over a symbol.
fn show_hover(
    editor: &mut Editor,
    point: DisplayPoint,
    ignore_timeout: bool,
    cx: &mut ViewContext<Editor>,
) {
    if editor.pending_rename.is_some() {
        return;
    }

    let snapshot = editor.snapshot(cx);
    let multibuffer_offset = point.to_offset(&snapshot.display_snapshot, Bias::Left);

    let (buffer, buffer_position) = if let Some(output) = editor
        .buffer
        .read(cx)
        .text_anchor_for_position(multibuffer_offset, cx)
    {
        output
    } else {
        return;
    };

    let excerpt_id = if let Some((excerpt_id, _, _)) = editor
        .buffer()
        .read(cx)
        .excerpt_containing(multibuffer_offset, cx)
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

    if !ignore_timeout {
        if let Some(InfoPopover { symbol_range, .. }) = &editor.hover_state.info_popover {
            if symbol_range
                .to_offset(&snapshot.buffer_snapshot)
                .contains(&multibuffer_offset)
            {
                // Hover triggered from same location as last time. Don't show again.
                return;
            } else {
                hide_hover(editor, &HideHover, cx);
            }
        }
    }

    // Get input anchor
    let anchor = snapshot
        .buffer_snapshot
        .anchor_at(multibuffer_offset, Bias::Left);

    // Don't request again if the location is the same as the previous request
    if let Some(triggered_from) = &editor.hover_state.triggered_from {
        if triggered_from
            .cmp(&anchor, &snapshot.buffer_snapshot)
            .is_eq()
        {
            return;
        }
    }

    let task = cx.spawn_weak(|this, mut cx| {
        async move {
            // If we need to delay, delay a set amount initially before making the lsp request
            let delay = if !ignore_timeout {
                // Construct delay task to wait for later
                let total_delay = Some(
                    cx.background()
                        .timer(Duration::from_millis(HOVER_DELAY_MILLIS)),
                );

                cx.background()
                    .timer(Duration::from_millis(HOVER_REQUEST_DELAY_MILLIS))
                    .await;
                total_delay
            } else {
                None
            };

            // query the LSP for hover info
            let hover_request = cx.update(|cx| {
                project.update(cx, |project, cx| {
                    project.hover(&buffer, buffer_position, cx)
                })
            });

            if let Some(delay) = delay {
                delay.await;
            }

            // If there's a diagnostic, assign it on the hover state and notify
            let local_diagnostic = snapshot
                .buffer_snapshot
                .diagnostics_in_range::<_, usize>(multibuffer_offset..multibuffer_offset, false)
                // Find the entry with the most specific range
                .min_by_key(|entry| entry.range.end - entry.range.start)
                .map(|entry| DiagnosticEntry {
                    diagnostic: entry.diagnostic,
                    range: entry.range.to_anchors(&snapshot.buffer_snapshot),
                });

            // Pull the primary diagnostic out so we can jump to it if the popover is clicked
            let primary_diagnostic = local_diagnostic.as_ref().and_then(|local_diagnostic| {
                snapshot
                    .buffer_snapshot
                    .diagnostic_group::<usize>(local_diagnostic.diagnostic.group_id)
                    .find(|diagnostic| diagnostic.diagnostic.is_primary)
                    .map(|entry| DiagnosticEntry {
                        diagnostic: entry.diagnostic,
                        range: entry.range.to_anchors(&snapshot.buffer_snapshot),
                    })
            });

            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, _| {
                    this.hover_state.diagnostic_popover =
                        local_diagnostic.map(|local_diagnostic| DiagnosticPopover {
                            local_diagnostic,
                            primary_diagnostic,
                        });
                });
            }

            // Construct new hover popover from hover request
            let hover_popover = hover_request.await.ok().flatten().and_then(|hover_result| {
                if hover_result.contents.is_empty() {
                    return None;
                }

                // Create symbol range of anchors for highlighting and filtering
                // of future requests.
                let range = if let Some(range) = hover_result.range {
                    let start = snapshot
                        .buffer_snapshot
                        .anchor_in_excerpt(excerpt_id.clone(), range.start);
                    let end = snapshot
                        .buffer_snapshot
                        .anchor_in_excerpt(excerpt_id.clone(), range.end);

                    start..end
                } else {
                    anchor..anchor
                };

                Some(InfoPopover {
                    project: project.clone(),
                    symbol_range: range,
                    contents: hover_result.contents,
                })
            });

            if let Some(this) = this.upgrade(&cx) {
                this.update(&mut cx, |this, cx| {
                    if let Some(hover_popover) = hover_popover.as_ref() {
                        // Highlight the selected symbol using a background highlight
                        this.highlight_background::<HoverState>(
                            vec![hover_popover.symbol_range.clone()],
                            |theme| theme.editor.hover_popover.highlight,
                            cx,
                        );
                    } else {
                        this.clear_background_highlights::<HoverState>(cx);
                    }

                    this.hover_state.info_popover = hover_popover;
                    cx.notify();
                });
            }
            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    });

    editor.hover_state.info_task = Some(task);
}

#[derive(Default)]
pub struct HoverState {
    pub info_popover: Option<InfoPopover>,
    pub diagnostic_popover: Option<DiagnosticPopover>,
    pub triggered_from: Option<Anchor>,
    pub info_task: Option<Task<Option<()>>>,
}

impl HoverState {
    pub fn visible(&self) -> bool {
        self.info_popover.is_some() || self.diagnostic_popover.is_some()
    }

    pub fn render(
        &self,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        visible_rows: Range<u32>,
        cx: &mut RenderContext<Editor>,
    ) -> Option<(DisplayPoint, Vec<ElementBox>)> {
        // If there is a diagnostic, position the popovers based on that.
        // Otherwise use the start of the hover range
        let anchor = self
            .diagnostic_popover
            .as_ref()
            .map(|diagnostic_popover| &diagnostic_popover.local_diagnostic.range.start)
            .or_else(|| {
                self.info_popover
                    .as_ref()
                    .map(|info_popover| &info_popover.symbol_range.start)
            })?;
        let point = anchor.to_display_point(&snapshot.display_snapshot);

        // Don't render if the relevant point isn't on screen
        if !self.visible() || !visible_rows.contains(&point.row()) {
            return None;
        }

        let mut elements = Vec::new();

        if let Some(diagnostic_popover) = self.diagnostic_popover.as_ref() {
            elements.push(diagnostic_popover.render(style, cx));
        }
        if let Some(info_popover) = self.info_popover.as_ref() {
            elements.push(info_popover.render(style, cx));
        }

        Some((point, elements))
    }
}

#[derive(Debug, Clone)]
pub struct InfoPopover {
    pub project: ModelHandle<Project>,
    pub symbol_range: Range<Anchor>,
    pub contents: Vec<HoverBlock>,
}

impl InfoPopover {
    pub fn render(&self, style: &EditorStyle, cx: &mut RenderContext<Editor>) -> ElementBox {
        MouseEventHandler::<InfoPopover>::new(0, cx, |_, cx| {
            let mut flex = Flex::new(Axis::Vertical).scrollable::<HoverBlock, _>(1, None, cx);
            flex.extend(self.contents.iter().map(|content| {
                let languages = self.project.read(cx).languages();
                if let Some(language) = content.language.clone().and_then(|language| {
                    languages.language_for_name(&language).now_or_never()?.ok()
                }) {
                    let runs = language
                        .highlight_text(&content.text.as_str().into(), 0..content.text.len());

                    Text::new(content.text.clone(), style.text.clone())
                        .with_soft_wrap(true)
                        .with_highlights(
                            runs.iter()
                                .filter_map(|(range, id)| {
                                    id.style(style.theme.syntax.as_ref())
                                        .map(|style| (range.clone(), style))
                                })
                                .collect(),
                        )
                        .boxed()
                } else {
                    let mut text_style = style.hover_popover.prose.clone();
                    text_style.font_size = style.text.font_size;

                    Text::new(content.text.clone(), text_style)
                        .with_soft_wrap(true)
                        .contained()
                        .with_style(style.hover_popover.block_style)
                        .boxed()
                }
            }));
            flex.contained()
                .with_style(style.hover_popover.container)
                .boxed()
        })
        .on_move(|_, _| {}) // Consume move events so they don't reach regions underneath.
        .with_cursor_style(CursorStyle::Arrow)
        .with_padding(Padding {
            bottom: HOVER_POPOVER_GAP,
            top: HOVER_POPOVER_GAP,
            ..Default::default()
        })
        .boxed()
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticPopover {
    local_diagnostic: DiagnosticEntry<Anchor>,
    primary_diagnostic: Option<DiagnosticEntry<Anchor>>,
}

impl DiagnosticPopover {
    pub fn render(&self, style: &EditorStyle, cx: &mut RenderContext<Editor>) -> ElementBox {
        enum PrimaryDiagnostic {}

        let mut text_style = style.hover_popover.prose.clone();
        text_style.font_size = style.text.font_size;

        let container_style = match self.local_diagnostic.diagnostic.severity {
            DiagnosticSeverity::HINT => style.hover_popover.info_container,
            DiagnosticSeverity::INFORMATION => style.hover_popover.info_container,
            DiagnosticSeverity::WARNING => style.hover_popover.warning_container,
            DiagnosticSeverity::ERROR => style.hover_popover.error_container,
            _ => style.hover_popover.container,
        };

        let tooltip_style = cx.global::<Settings>().theme.tooltip.clone();

        MouseEventHandler::<DiagnosticPopover>::new(0, cx, |_, _| {
            Text::new(self.local_diagnostic.diagnostic.message.clone(), text_style)
                .with_soft_wrap(true)
                .contained()
                .with_style(container_style)
                .boxed()
        })
        .with_padding(Padding {
            top: HOVER_POPOVER_GAP,
            bottom: HOVER_POPOVER_GAP,
            ..Default::default()
        })
        .on_move(|_, _| {}) // Consume move events so they don't reach regions underneath.
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(GoToDiagnostic)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<PrimaryDiagnostic, _>(
            0,
            "Go To Diagnostic".to_string(),
            Some(Box::new(crate::GoToDiagnostic)),
            tooltip_style,
            cx,
        )
        .boxed()
    }

    pub fn activation_info(&self) -> (usize, Anchor) {
        let entry = self
            .primary_diagnostic
            .as_ref()
            .unwrap_or(&self.local_diagnostic);

        (entry.diagnostic.group_id, entry.range.start.clone())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use language::{Diagnostic, DiagnosticSet};
    use project::HoverBlock;
    use smol::stream::StreamExt;

    use crate::test::editor_lsp_test_context::EditorLspTestContext;

    use super::*;

    #[gpui::test]
    async fn test_mouse_hover_info_popover(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        // Basic hover delays and then pops without moving the mouse
        cx.set_state(indoc! {"
            fn ˇtest() { println!(); }
        "});
        let hover_point = cx.display_point(indoc! {"
            fn test() { printˇln!(); }
        "});

        cx.update_editor(|editor, cx| {
            hover_at(
                editor,
                &HoverAt {
                    point: Some(hover_point),
                },
                cx,
            )
        });
        assert!(!cx.editor(|editor, _| editor.hover_state.visible()));

        // After delay, hover should be visible.
        let symbol_range = cx.lsp_range(indoc! {"
            fn test() { «println!»(); }
        "});
        let mut requests =
            cx.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: indoc! {"
                            # Some basic docs
                            Some test documentation"}
                        .to_string(),
                    }),
                    range: Some(symbol_range),
                }))
            });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));
        requests.next().await;

        cx.editor(|editor, _| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.info_popover.clone().unwrap().contents,
                vec![
                    HoverBlock {
                        text: "Some basic docs".to_string(),
                        language: None
                    },
                    HoverBlock {
                        text: "Some test documentation".to_string(),
                        language: None
                    }
                ]
            )
        });

        // Mouse moved with no hover response dismisses
        let hover_point = cx.display_point(indoc! {"
            fn teˇst() { println!(); }
        "});
        let mut request = cx
            .lsp
            .handle_request::<lsp::request::HoverRequest, _, _>(|_, _| async move { Ok(None) });
        cx.update_editor(|editor, cx| {
            hover_at(
                editor,
                &HoverAt {
                    point: Some(hover_point),
                },
                cx,
            )
        });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));
        request.next().await;
        cx.editor(|editor, _| {
            assert!(!editor.hover_state.visible());
        });
    }

    #[gpui::test]
    async fn test_keyboard_hover_info_popover(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        // Hover with keyboard has no delay
        cx.set_state(indoc! {"
            fˇn test() { println!(); }
        "});
        cx.update_editor(|editor, cx| hover(editor, &Hover, cx));
        let symbol_range = cx.lsp_range(indoc! {"
            «fn» test() { println!(); }
        "});
        cx.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
            Ok(Some(lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: indoc! {"
                        # Some other basic docs
                        Some other test documentation"}
                    .to_string(),
                }),
                range: Some(symbol_range),
            }))
        })
        .next()
        .await;

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _| {
            assert_eq!(
                editor.hover_state.info_popover.clone().unwrap().contents,
                vec![
                    HoverBlock {
                        text: "Some other basic docs".to_string(),
                        language: None
                    },
                    HoverBlock {
                        text: "Some other test documentation".to_string(),
                        language: None
                    }
                ]
            )
        });
    }

    #[gpui::test]
    async fn test_hover_diagnostic_and_info_popovers(cx: &mut gpui::TestAppContext) {
        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            cx,
        )
        .await;

        // Hover with just diagnostic, pops DiagnosticPopover immediately and then
        // info popover once request completes
        cx.set_state(indoc! {"
            fn teˇst() { println!(); }
        "});

        // Send diagnostic to client
        let range = cx.text_anchor_range(indoc! {"
            fn «test»() { println!(); }
        "});
        cx.update_buffer(|buffer, cx| {
            let snapshot = buffer.text_snapshot();
            let set = DiagnosticSet::from_sorted_entries(
                vec![DiagnosticEntry {
                    range,
                    diagnostic: Diagnostic {
                        message: "A test diagnostic message.".to_string(),
                        ..Default::default()
                    },
                }],
                &snapshot,
            );
            buffer.update_diagnostics(set, cx);
        });

        // Hover pops diagnostic immediately
        cx.update_editor(|editor, cx| hover(editor, &Hover, cx));
        cx.foreground().run_until_parked();

        cx.editor(|Editor { hover_state, .. }, _| {
            assert!(hover_state.diagnostic_popover.is_some() && hover_state.info_popover.is_none())
        });

        // Info Popover shows after request responded to
        let range = cx.lsp_range(indoc! {"
            fn «test»() { println!(); }
        "});
        cx.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
            Ok(Some(lsp::Hover {
                contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                    kind: lsp::MarkupKind::Markdown,
                    value: indoc! {"
                        # Some other basic docs
                        Some other test documentation"}
                    .to_string(),
                }),
                range: Some(range),
            }))
        });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));

        cx.foreground().run_until_parked();
        cx.editor(|Editor { hover_state, .. }, _| {
            hover_state.diagnostic_popover.is_some() && hover_state.info_task.is_some()
        });
    }
}
