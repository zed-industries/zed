use crate::{
    display_map::{InlayOffset, ToDisplayPoint},
    link_go_to_definition::{InlayHighlight, RangeInEditor},
    Anchor, AnchorRangeExt, DisplayPoint, Editor, EditorSettings, EditorSnapshot, EditorStyle,
    ExcerptId, RangeToAnchorExt,
};
use futures::FutureExt;
use gpui::{
    actions,
    elements::{Flex, MouseEventHandler, Padding, ParentElement, Text},
    fonts::{HighlightStyle, Underline, Weight},
    platform::{CursorStyle, MouseButton},
    AnyElement, AppContext, CursorRegion, Element, ModelHandle, MouseRegion, Task, ViewContext,
};
use language::{Bias, DiagnosticEntry, DiagnosticSeverity, Language, LanguageRegistry};
use project::{HoverBlock, HoverBlockKind, InlayHintLabelPart, Project};
use std::{ops::Range, sync::Arc, time::Duration};
use util::TryFutureExt;

pub const HOVER_DELAY_MILLIS: u64 = 350;
pub const HOVER_REQUEST_DELAY_MILLIS: u64 = 200;

pub const MIN_POPOVER_CHARACTER_WIDTH: f32 = 20.;
pub const MIN_POPOVER_LINE_HEIGHT: f32 = 4.;
pub const HOVER_POPOVER_GAP: f32 = 10.;

actions!(editor, [Hover]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(hover);
}

/// Bindable action which uses the most recent selection head to trigger a hover
pub fn hover(editor: &mut Editor, _: &Hover, cx: &mut ViewContext<Editor>) {
    let head = editor.selections.newest_display(cx).head();
    show_hover(editor, head, true, cx);
}

/// The internal hover action dispatches between `show_hover` or `hide_hover`
/// depending on whether a point to hover over is provided.
pub fn hover_at(editor: &mut Editor, point: Option<DisplayPoint>, cx: &mut ViewContext<Editor>) {
    if settings::get::<EditorSettings>(cx).hover_popover_enabled {
        if let Some(point) = point {
            show_hover(editor, point, false, cx);
        } else {
            hide_hover(editor, cx);
        }
    }
}

pub struct InlayHover {
    pub excerpt: ExcerptId,
    pub range: InlayHighlight,
    pub tooltip: HoverBlock,
}

pub fn find_hovered_hint_part(
    label_parts: Vec<InlayHintLabelPart>,
    hint_start: InlayOffset,
    hovered_offset: InlayOffset,
) -> Option<(InlayHintLabelPart, Range<InlayOffset>)> {
    if hovered_offset >= hint_start {
        let mut hovered_character = (hovered_offset - hint_start).0;
        let mut part_start = hint_start;
        for part in label_parts {
            let part_len = part.value.chars().count();
            if hovered_character > part_len {
                hovered_character -= part_len;
                part_start.0 += part_len;
            } else {
                let part_end = InlayOffset(part_start.0 + part_len);
                return Some((part, part_start..part_end));
            }
        }
    }
    None
}

pub fn hover_at_inlay(editor: &mut Editor, inlay_hover: InlayHover, cx: &mut ViewContext<Editor>) {
    if settings::get::<EditorSettings>(cx).hover_popover_enabled {
        if editor.pending_rename.is_some() {
            return;
        }

        let Some(project) = editor.project.clone() else {
            return;
        };

        if let Some(InfoPopover { symbol_range, .. }) = &editor.hover_state.info_popover {
            if let RangeInEditor::Inlay(range) = symbol_range {
                if range == &inlay_hover.range {
                    // Hover triggered from same location as last time. Don't show again.
                    return;
                }
            }
            hide_hover(editor, cx);
        }

        let task = cx.spawn(|this, mut cx| {
            async move {
                cx.background()
                    .timer(Duration::from_millis(HOVER_DELAY_MILLIS))
                    .await;
                this.update(&mut cx, |this, _| {
                    this.hover_state.diagnostic_popover = None;
                })?;

                let hover_popover = InfoPopover {
                    project: project.clone(),
                    symbol_range: RangeInEditor::Inlay(inlay_hover.range.clone()),
                    blocks: vec![inlay_hover.tooltip],
                    language: None,
                    rendered_content: None,
                };

                this.update(&mut cx, |this, cx| {
                    // Highlight the selected symbol using a background highlight
                    this.highlight_inlay_background::<HoverState>(
                        vec![inlay_hover.range],
                        |theme| theme.editor.hover_popover.highlight,
                        cx,
                    );
                    this.hover_state.info_popover = Some(hover_popover);
                    cx.notify();
                })?;

                anyhow::Ok(())
            }
            .log_err()
        });

        editor.hover_state.info_task = Some(task);
    }
}

/// Hides the type information popup.
/// Triggered by the `Hover` action when the cursor is not over a symbol or when the
/// selections changed.
pub fn hide_hover(editor: &mut Editor, cx: &mut ViewContext<Editor>) -> bool {
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
                .as_text_range()
                .map(|range| {
                    range
                        .to_offset(&snapshot.buffer_snapshot)
                        .contains(&multibuffer_offset)
                })
                .unwrap_or(false)
            {
                // Hover triggered from same location as last time. Don't show again.
                return;
            } else {
                hide_hover(editor, cx);
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

    let task = cx.spawn(|this, mut cx| {
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

            this.update(&mut cx, |this, _| {
                this.hover_state.diagnostic_popover =
                    local_diagnostic.map(|local_diagnostic| DiagnosticPopover {
                        local_diagnostic,
                        primary_diagnostic,
                    });
            })?;

            // Construct new hover popover from hover request
            let hover_popover = hover_request.await.ok().flatten().and_then(|hover_result| {
                if hover_result.is_empty() {
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
                    symbol_range: RangeInEditor::Text(range),
                    blocks: hover_result.contents,
                    language: hover_result.language,
                    rendered_content: None,
                })
            });

            this.update(&mut cx, |this, cx| {
                if let Some(symbol_range) = hover_popover
                    .as_ref()
                    .and_then(|hover_popover| hover_popover.symbol_range.as_text_range())
                {
                    // Highlight the selected symbol using a background highlight
                    this.highlight_background::<HoverState>(
                        vec![symbol_range],
                        |theme| theme.editor.hover_popover.highlight,
                        cx,
                    );
                } else {
                    this.clear_background_highlights::<HoverState>(cx);
                }

                this.hover_state.info_popover = hover_popover;
                cx.notify();
            })?;

            Ok::<_, anyhow::Error>(())
        }
        .log_err()
    });

    editor.hover_state.info_task = Some(task);
}

fn render_blocks(
    theme_id: usize,
    blocks: &[HoverBlock],
    language_registry: &Arc<LanguageRegistry>,
    language: Option<&Arc<Language>>,
    style: &EditorStyle,
) -> RenderedInfo {
    let mut text = String::new();
    let mut highlights = Vec::new();
    let mut region_ranges = Vec::new();
    let mut regions = Vec::new();

    for block in blocks {
        match &block.kind {
            HoverBlockKind::PlainText => {
                new_paragraph(&mut text, &mut Vec::new());
                text.push_str(&block.text);
            }
            HoverBlockKind::Markdown => {
                use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

                let mut bold_depth = 0;
                let mut italic_depth = 0;
                let mut link_url = None;
                let mut current_language = None;
                let mut list_stack = Vec::new();

                for event in Parser::new_ext(&block.text, Options::all()) {
                    let prev_len = text.len();
                    match event {
                        Event::Text(t) => {
                            if let Some(language) = &current_language {
                                render_code(
                                    &mut text,
                                    &mut highlights,
                                    t.as_ref(),
                                    language,
                                    style,
                                );
                            } else {
                                text.push_str(t.as_ref());

                                let mut style = HighlightStyle::default();
                                if bold_depth > 0 {
                                    style.weight = Some(Weight::BOLD);
                                }
                                if italic_depth > 0 {
                                    style.italic = Some(true);
                                }
                                if let Some(link_url) = link_url.clone() {
                                    region_ranges.push(prev_len..text.len());
                                    regions.push(RenderedRegion {
                                        link_url: Some(link_url),
                                        code: false,
                                    });
                                    style.underline = Some(Underline {
                                        thickness: 1.0.into(),
                                        ..Default::default()
                                    });
                                }

                                if style != HighlightStyle::default() {
                                    let mut new_highlight = true;
                                    if let Some((last_range, last_style)) = highlights.last_mut() {
                                        if last_range.end == prev_len && last_style == &style {
                                            last_range.end = text.len();
                                            new_highlight = false;
                                        }
                                    }
                                    if new_highlight {
                                        highlights.push((prev_len..text.len(), style));
                                    }
                                }
                            }
                        }
                        Event::Code(t) => {
                            text.push_str(t.as_ref());
                            region_ranges.push(prev_len..text.len());
                            if link_url.is_some() {
                                highlights.push((
                                    prev_len..text.len(),
                                    HighlightStyle {
                                        underline: Some(Underline {
                                            thickness: 1.0.into(),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    },
                                ));
                            }
                            regions.push(RenderedRegion {
                                code: true,
                                link_url: link_url.clone(),
                            });
                        }
                        Event::Start(tag) => match tag {
                            Tag::Paragraph => new_paragraph(&mut text, &mut list_stack),
                            Tag::Heading(_, _, _) => {
                                new_paragraph(&mut text, &mut list_stack);
                                bold_depth += 1;
                            }
                            Tag::CodeBlock(kind) => {
                                new_paragraph(&mut text, &mut list_stack);
                                current_language = if let CodeBlockKind::Fenced(language) = kind {
                                    language_registry
                                        .language_for_name(language.as_ref())
                                        .now_or_never()
                                        .and_then(Result::ok)
                                } else {
                                    language.cloned()
                                }
                            }
                            Tag::Emphasis => italic_depth += 1,
                            Tag::Strong => bold_depth += 1,
                            Tag::Link(_, url, _) => link_url = Some(url.to_string()),
                            Tag::List(number) => {
                                list_stack.push((number, false));
                            }
                            Tag::Item => {
                                let len = list_stack.len();
                                if let Some((list_number, has_content)) = list_stack.last_mut() {
                                    *has_content = false;
                                    if !text.is_empty() && !text.ends_with('\n') {
                                        text.push('\n');
                                    }
                                    for _ in 0..len - 1 {
                                        text.push_str("  ");
                                    }
                                    if let Some(number) = list_number {
                                        text.push_str(&format!("{}. ", number));
                                        *number += 1;
                                        *has_content = false;
                                    } else {
                                        text.push_str("- ");
                                    }
                                }
                            }
                            _ => {}
                        },
                        Event::End(tag) => match tag {
                            Tag::Heading(_, _, _) => bold_depth -= 1,
                            Tag::CodeBlock(_) => current_language = None,
                            Tag::Emphasis => italic_depth -= 1,
                            Tag::Strong => bold_depth -= 1,
                            Tag::Link(_, _, _) => link_url = None,
                            Tag::List(_) => drop(list_stack.pop()),
                            _ => {}
                        },
                        Event::HardBreak => text.push('\n'),
                        Event::SoftBreak => text.push(' '),
                        _ => {}
                    }
                }
            }
            HoverBlockKind::Code { language } => {
                if let Some(language) = language_registry
                    .language_for_name(language)
                    .now_or_never()
                    .and_then(Result::ok)
                {
                    render_code(&mut text, &mut highlights, &block.text, &language, style);
                } else {
                    text.push_str(&block.text);
                }
            }
        }
    }

    RenderedInfo {
        theme_id,
        text: text.trim().to_string(),
        highlights,
        region_ranges,
        regions,
    }
}

fn render_code(
    text: &mut String,
    highlights: &mut Vec<(Range<usize>, HighlightStyle)>,
    content: &str,
    language: &Arc<Language>,
    style: &EditorStyle,
) {
    let prev_len = text.len();
    text.push_str(content);
    for (range, highlight_id) in language.highlight_text(&content.into(), 0..content.len()) {
        if let Some(style) = highlight_id.style(&style.syntax) {
            highlights.push((prev_len + range.start..prev_len + range.end, style));
        }
    }
}

fn new_paragraph(text: &mut String, list_stack: &mut Vec<(Option<u64>, bool)>) {
    let mut is_subsequent_paragraph_of_list = false;
    if let Some((_, has_content)) = list_stack.last_mut() {
        if *has_content {
            is_subsequent_paragraph_of_list = true;
        } else {
            *has_content = true;
            return;
        }
    }

    if !text.is_empty() {
        if !text.ends_with('\n') {
            text.push('\n');
        }
        text.push('\n');
    }
    for _ in 0..list_stack.len().saturating_sub(1) {
        text.push_str("  ");
    }
    if is_subsequent_paragraph_of_list {
        text.push_str("  ");
    }
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
        &mut self,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        visible_rows: Range<u32>,
        cx: &mut ViewContext<Editor>,
    ) -> Option<(DisplayPoint, Vec<AnyElement<Editor>>)> {
        // If there is a diagnostic, position the popovers based on that.
        // Otherwise use the start of the hover range
        let anchor = self
            .diagnostic_popover
            .as_ref()
            .map(|diagnostic_popover| &diagnostic_popover.local_diagnostic.range.start)
            .or_else(|| {
                self.info_popover
                    .as_ref()
                    .map(|info_popover| match &info_popover.symbol_range {
                        RangeInEditor::Text(range) => &range.start,
                        RangeInEditor::Inlay(range) => &range.inlay_position,
                    })
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
        if let Some(info_popover) = self.info_popover.as_mut() {
            elements.push(info_popover.render(style, cx));
        }

        Some((point, elements))
    }
}

#[derive(Debug, Clone)]
pub struct InfoPopover {
    pub project: ModelHandle<Project>,
    symbol_range: RangeInEditor,
    pub blocks: Vec<HoverBlock>,
    language: Option<Arc<Language>>,
    rendered_content: Option<RenderedInfo>,
}

#[derive(Debug, Clone)]
struct RenderedInfo {
    theme_id: usize,
    text: String,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
    region_ranges: Vec<Range<usize>>,
    regions: Vec<RenderedRegion>,
}

#[derive(Debug, Clone)]
struct RenderedRegion {
    code: bool,
    link_url: Option<String>,
}

impl InfoPopover {
    pub fn render(
        &mut self,
        style: &EditorStyle,
        cx: &mut ViewContext<Editor>,
    ) -> AnyElement<Editor> {
        if let Some(rendered) = &self.rendered_content {
            if rendered.theme_id != style.theme_id {
                self.rendered_content = None;
            }
        }

        let rendered_content = self.rendered_content.get_or_insert_with(|| {
            render_blocks(
                style.theme_id,
                &self.blocks,
                self.project.read(cx).languages(),
                self.language.as_ref(),
                style,
            )
        });

        MouseEventHandler::new::<InfoPopover, _>(0, cx, |_, cx| {
            let mut region_id = 0;
            let view_id = cx.view_id();

            let code_span_background_color = style.document_highlight_read_background;
            let regions = rendered_content.regions.clone();
            Flex::column()
                .scrollable::<HoverBlock>(1, None, cx)
                .with_child(
                    Text::new(rendered_content.text.clone(), style.text.clone())
                        .with_highlights(rendered_content.highlights.clone())
                        .with_custom_runs(
                            rendered_content.region_ranges.clone(),
                            move |ix, bounds, cx| {
                                region_id += 1;
                                let region = regions[ix].clone();
                                if let Some(url) = region.link_url {
                                    cx.scene().push_cursor_region(CursorRegion {
                                        bounds,
                                        style: CursorStyle::PointingHand,
                                    });
                                    cx.scene().push_mouse_region(
                                        MouseRegion::new::<Self>(view_id, region_id, bounds)
                                            .on_click::<Editor, _>(
                                                MouseButton::Left,
                                                move |_, _, cx| cx.platform().open_url(&url),
                                            ),
                                    );
                                }
                                if region.code {
                                    cx.scene().push_quad(gpui::Quad {
                                        bounds,
                                        background: Some(code_span_background_color),
                                        border: Default::default(),
                                        corner_radii: (2.0).into(),
                                    });
                                }
                            },
                        )
                        .with_soft_wrap(true),
                )
                .contained()
                .with_style(style.hover_popover.container)
        })
        .on_move(|_, _, _| {}) // Consume move events so they don't reach regions underneath.
        .with_cursor_style(CursorStyle::Arrow)
        .with_padding(Padding {
            bottom: HOVER_POPOVER_GAP,
            top: HOVER_POPOVER_GAP,
            ..Default::default()
        })
        .into_any()
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticPopover {
    local_diagnostic: DiagnosticEntry<Anchor>,
    primary_diagnostic: Option<DiagnosticEntry<Anchor>>,
}

impl DiagnosticPopover {
    pub fn render(&self, style: &EditorStyle, cx: &mut ViewContext<Editor>) -> AnyElement<Editor> {
        enum PrimaryDiagnostic {}

        let mut text_style = style.hover_popover.prose.clone();
        text_style.font_size = style.text.font_size;
        let diagnostic_source_style = style.hover_popover.diagnostic_source_highlight.clone();

        let text = match &self.local_diagnostic.diagnostic.source {
            Some(source) => Text::new(
                format!("{source}: {}", self.local_diagnostic.diagnostic.message),
                text_style,
            )
            .with_highlights(vec![(0..source.len(), diagnostic_source_style)]),

            None => Text::new(self.local_diagnostic.diagnostic.message.clone(), text_style),
        };

        let container_style = match self.local_diagnostic.diagnostic.severity {
            DiagnosticSeverity::HINT => style.hover_popover.info_container,
            DiagnosticSeverity::INFORMATION => style.hover_popover.info_container,
            DiagnosticSeverity::WARNING => style.hover_popover.warning_container,
            DiagnosticSeverity::ERROR => style.hover_popover.error_container,
            _ => style.hover_popover.container,
        };

        let tooltip_style = theme::current(cx).tooltip.clone();

        MouseEventHandler::new::<DiagnosticPopover, _>(0, cx, |_, _| {
            text.with_soft_wrap(true)
                .contained()
                .with_style(container_style)
        })
        .with_padding(Padding {
            top: HOVER_POPOVER_GAP,
            bottom: HOVER_POPOVER_GAP,
            ..Default::default()
        })
        .on_move(|_, _, _| {}) // Consume move events so they don't reach regions underneath.
        .on_click(MouseButton::Left, |_, this, cx| {
            this.go_to_diagnostic(&Default::default(), cx)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<PrimaryDiagnostic>(
            0,
            "Go To Diagnostic".to_string(),
            Some(Box::new(crate::GoToDiagnostic)),
            tooltip_style,
            cx,
        )
        .into_any()
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
    use super::*;
    use crate::{
        editor_tests::init_test,
        element::PointForPosition,
        inlay_hint_cache::tests::{cached_hint_labels, visible_hint_labels},
        link_go_to_definition::update_inlay_link_and_hover_points,
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use collections::BTreeSet;
    use gpui::fonts::Weight;
    use indoc::indoc;
    use language::{language_settings::InlayHintSettings, Diagnostic, DiagnosticSet};
    use lsp::LanguageServerId;
    use project::{HoverBlock, HoverBlockKind};
    use smol::stream::StreamExt;
    use unindent::Unindent;
    use util::test::marked_text_ranges;

    #[gpui::test]
    async fn test_mouse_hover_info_popover(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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

        cx.update_editor(|editor, cx| hover_at(editor, Some(hover_point), cx));
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
                        value: "some basic docs".to_string(),
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
                editor.hover_state.info_popover.clone().unwrap().blocks,
                vec![HoverBlock {
                    text: "some basic docs".to_string(),
                    kind: HoverBlockKind::Markdown,
                },]
            )
        });

        // Mouse moved with no hover response dismisses
        let hover_point = cx.display_point(indoc! {"
            fn teˇst() { println!(); }
        "});
        let mut request = cx
            .lsp
            .handle_request::<lsp::request::HoverRequest, _, _>(|_, _| async move { Ok(None) });
        cx.update_editor(|editor, cx| hover_at(editor, Some(hover_point), cx));
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));
        request.next().await;
        cx.editor(|editor, _| {
            assert!(!editor.hover_state.visible());
        });
    }

    #[gpui::test]
    async fn test_keyboard_hover_info_popover(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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
                    value: "some other basic docs".to_string(),
                }),
                range: Some(symbol_range),
            }))
        })
        .next()
        .await;

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _| {
            assert_eq!(
                editor.hover_state.info_popover.clone().unwrap().blocks,
                vec![HoverBlock {
                    text: "some other basic docs".to_string(),
                    kind: HoverBlockKind::Markdown,
                }]
            )
        });
    }

    #[gpui::test]
    async fn test_empty_hovers_filtered(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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
                contents: lsp::HoverContents::Array(vec![
                    lsp::MarkedString::String("regular text for hover to show".to_string()),
                    lsp::MarkedString::String("".to_string()),
                    lsp::MarkedString::LanguageString(lsp::LanguageString {
                        language: "Rust".to_string(),
                        value: "".to_string(),
                    }),
                ]),
                range: Some(symbol_range),
            }))
        })
        .next()
        .await;

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _| {
            assert_eq!(
                editor.hover_state.info_popover.clone().unwrap().blocks,
                vec![HoverBlock {
                    text: "regular text for hover to show".to_string(),
                    kind: HoverBlockKind::Markdown,
                }],
                "No empty string hovers should be shown"
            );
        });
    }

    #[gpui::test]
    async fn test_line_ends_trimmed(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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

        let code_str = "\nlet hovered_point: Vector2F // size = 8, align = 0x4\n";
        let markdown_string = format!("\n```rust\n{code_str}```");

        let closure_markdown_string = markdown_string.clone();
        cx.handle_request::<lsp::request::HoverRequest, _, _>(move |_, _, _| {
            let future_markdown_string = closure_markdown_string.clone();
            async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: future_markdown_string,
                    }),
                    range: Some(symbol_range),
                }))
            }
        })
        .next()
        .await;

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, cx| {
            let blocks = editor.hover_state.info_popover.clone().unwrap().blocks;
            assert_eq!(
                blocks,
                vec![HoverBlock {
                    text: markdown_string,
                    kind: HoverBlockKind::Markdown,
                }],
            );

            let style = editor.style(cx);
            let rendered = render_blocks(0, &blocks, &Default::default(), None, &style);
            assert_eq!(
                rendered.text,
                code_str.trim(),
                "Should not have extra line breaks at end of rendered hover"
            );
        });
    }

    #[gpui::test]
    async fn test_hover_diagnostic_and_info_popovers(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

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
            buffer.update_diagnostics(LanguageServerId(0), set, cx);
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
                    value: "some new docs".to_string(),
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

    #[gpui::test]
    fn test_render_blocks(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        cx.add_window(|cx| {
            let editor = Editor::single_line(None, cx);
            let style = editor.style(cx);

            struct Row {
                blocks: Vec<HoverBlock>,
                expected_marked_text: String,
                expected_styles: Vec<HighlightStyle>,
            }

            let rows = &[
                // Strong emphasis
                Row {
                    blocks: vec![HoverBlock {
                        text: "one **two** three".to_string(),
                        kind: HoverBlockKind::Markdown,
                    }],
                    expected_marked_text: "one «two» three".to_string(),
                    expected_styles: vec![HighlightStyle {
                        weight: Some(Weight::BOLD),
                        ..Default::default()
                    }],
                },
                // Links
                Row {
                    blocks: vec![HoverBlock {
                        text: "one [two](the-url) three".to_string(),
                        kind: HoverBlockKind::Markdown,
                    }],
                    expected_marked_text: "one «two» three".to_string(),
                    expected_styles: vec![HighlightStyle {
                        underline: Some(Underline {
                            thickness: 1.0.into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                },
                // Lists
                Row {
                    blocks: vec![HoverBlock {
                        text: "
                            lists:
                            * one
                                - a
                                - b
                            * two
                                - [c](the-url)
                                - d"
                        .unindent(),
                        kind: HoverBlockKind::Markdown,
                    }],
                    expected_marked_text: "
                        lists:
                        - one
                          - a
                          - b
                        - two
                          - «c»
                          - d"
                    .unindent(),
                    expected_styles: vec![HighlightStyle {
                        underline: Some(Underline {
                            thickness: 1.0.into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                },
                // Multi-paragraph list items
                Row {
                    blocks: vec![HoverBlock {
                        text: "
                            * one two
                              three

                            * four five
                                * six seven
                                  eight

                                  nine
                                * ten
                            * six"
                            .unindent(),
                        kind: HoverBlockKind::Markdown,
                    }],
                    expected_marked_text: "
                        - one two three
                        - four five
                          - six seven eight

                            nine
                          - ten
                        - six"
                        .unindent(),
                    expected_styles: vec![HighlightStyle {
                        underline: Some(Underline {
                            thickness: 1.0.into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                },
            ];

            for Row {
                blocks,
                expected_marked_text,
                expected_styles,
            } in &rows[0..]
            {
                let rendered = render_blocks(0, &blocks, &Default::default(), None, &style);

                let (expected_text, ranges) = marked_text_ranges(expected_marked_text, false);
                let expected_highlights = ranges
                    .into_iter()
                    .zip(expected_styles.iter().cloned())
                    .collect::<Vec<_>>();
                assert_eq!(
                    rendered.text, expected_text,
                    "wrong text for input {blocks:?}"
                );
                assert_eq!(
                    rendered.highlights, expected_highlights,
                    "wrong highlights for input {blocks:?}"
                );
            }

            editor
        });
    }

    #[gpui::test]
    async fn test_hover_inlay_label_parts(cx: &mut gpui::TestAppContext) {
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
                inlay_hint_provider: Some(lsp::OneOf::Right(
                    lsp::InlayHintServerCapabilities::Options(lsp::InlayHintOptions {
                        resolve_provider: Some(true),
                        ..Default::default()
                    }),
                )),
                ..Default::default()
            },
            cx,
        )
        .await;

        cx.set_state(indoc! {"
            struct TestStruct;

            // ==================

            struct TestNewType<T>(T);

            fn main() {
                let variableˇ = TestNewType(TestStruct);
            }
        "});

        let hint_start_offset = cx.ranges(indoc! {"
            struct TestStruct;

            // ==================

            struct TestNewType<T>(T);

            fn main() {
                let variableˇ = TestNewType(TestStruct);
            }
        "})[0]
            .start;
        let hint_position = cx.to_lsp(hint_start_offset);
        let new_type_target_range = cx.lsp_range(indoc! {"
            struct TestStruct;

            // ==================

            struct «TestNewType»<T>(T);

            fn main() {
                let variable = TestNewType(TestStruct);
            }
        "});
        let struct_target_range = cx.lsp_range(indoc! {"
            struct «TestStruct»;

            // ==================

            struct TestNewType<T>(T);

            fn main() {
                let variable = TestNewType(TestStruct);
            }
        "});

        let uri = cx.buffer_lsp_url.clone();
        let new_type_label = "TestNewType";
        let struct_label = "TestStruct";
        let entire_hint_label = ": TestNewType<TestStruct>";
        let closure_uri = uri.clone();
        cx.lsp
            .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
                let task_uri = closure_uri.clone();
                async move {
                    assert_eq!(params.text_document.uri, task_uri);
                    Ok(Some(vec![lsp::InlayHint {
                        position: hint_position,
                        label: lsp::InlayHintLabel::LabelParts(vec![lsp::InlayHintLabelPart {
                            value: entire_hint_label.to_string(),
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
            let expected_layers = vec![entire_hint_label.to_string()];
            assert_eq!(expected_layers, cached_hint_labels(editor));
            assert_eq!(expected_layers, visible_hint_labels(editor, cx));
        });

        let inlay_range = cx
            .ranges(indoc! {"
                struct TestStruct;

                // ==================

                struct TestNewType<T>(T);

                fn main() {
                    let variable« »= TestNewType(TestStruct);
                }
        "})
            .get(0)
            .cloned()
            .unwrap();
        let new_type_hint_part_hover_position = cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let previous_valid = inlay_range.start.to_display_point(&snapshot);
            let next_valid = inlay_range.end.to_display_point(&snapshot);
            assert_eq!(previous_valid.row(), next_valid.row());
            assert!(previous_valid.column() < next_valid.column());
            let exact_unclipped = DisplayPoint::new(
                previous_valid.row(),
                previous_valid.column()
                    + (entire_hint_label.find(new_type_label).unwrap() + new_type_label.len() / 2)
                        as u32,
            );
            PointForPosition {
                previous_valid,
                next_valid,
                exact_unclipped,
                column_overshoot_after_line_end: 0,
            }
        });
        cx.update_editor(|editor, cx| {
            update_inlay_link_and_hover_points(
                &editor.snapshot(cx),
                new_type_hint_part_hover_position,
                editor,
                true,
                false,
                cx,
            );
        });

        let resolve_closure_uri = uri.clone();
        cx.lsp
            .handle_request::<lsp::request::InlayHintResolveRequest, _, _>(
                move |mut hint_to_resolve, _| {
                    let mut resolved_hint_positions = BTreeSet::new();
                    let task_uri = resolve_closure_uri.clone();
                    async move {
                        let inserted = resolved_hint_positions.insert(hint_to_resolve.position);
                        assert!(inserted, "Hint {hint_to_resolve:?} was resolved twice");

                        // `: TestNewType<TestStruct>`
                        hint_to_resolve.label = lsp::InlayHintLabel::LabelParts(vec![
                            lsp::InlayHintLabelPart {
                                value: ": ".to_string(),
                                ..Default::default()
                            },
                            lsp::InlayHintLabelPart {
                                value: new_type_label.to_string(),
                                location: Some(lsp::Location {
                                    uri: task_uri.clone(),
                                    range: new_type_target_range,
                                }),
                                tooltip: Some(lsp::InlayHintLabelPartTooltip::String(format!(
                                    "A tooltip for `{new_type_label}`"
                                ))),
                                ..Default::default()
                            },
                            lsp::InlayHintLabelPart {
                                value: "<".to_string(),
                                ..Default::default()
                            },
                            lsp::InlayHintLabelPart {
                                value: struct_label.to_string(),
                                location: Some(lsp::Location {
                                    uri: task_uri,
                                    range: struct_target_range,
                                }),
                                tooltip: Some(lsp::InlayHintLabelPartTooltip::MarkupContent(
                                    lsp::MarkupContent {
                                        kind: lsp::MarkupKind::Markdown,
                                        value: format!("A tooltip for `{struct_label}`"),
                                    },
                                )),
                                ..Default::default()
                            },
                            lsp::InlayHintLabelPart {
                                value: ">".to_string(),
                                ..Default::default()
                            },
                        ]);

                        Ok(hint_to_resolve)
                    }
                },
            )
            .next()
            .await;
        cx.foreground().run_until_parked();

        cx.update_editor(|editor, cx| {
            update_inlay_link_and_hover_points(
                &editor.snapshot(cx),
                new_type_hint_part_hover_position,
                editor,
                true,
                false,
                cx,
            );
        });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));
        cx.foreground().run_until_parked();
        cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let hover_state = &editor.hover_state;
            assert!(hover_state.diagnostic_popover.is_none() && hover_state.info_popover.is_some());
            let popover = hover_state.info_popover.as_ref().unwrap();
            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let entire_inlay_start = snapshot.display_point_to_inlay_offset(
                inlay_range.start.to_display_point(&snapshot),
                Bias::Left,
            );

            let expected_new_type_label_start = InlayOffset(entire_inlay_start.0 + ": ".len());
            // TODO kb
            // assert_eq!(
            //     popover.symbol_range,
            //     RangeInEditor::Inlay(InlayRange {
            //         inlay_position: buffer_snapshot.anchor_at(inlay_range.start, Bias::Right),
            //         highlight_start: expected_new_type_label_start,
            //         highlight_end: InlayOffset(
            //             expected_new_type_label_start.0 + new_type_label.len()
            //         ),
            //     }),
            //     "Popover range should match the new type label part"
            // );
            assert_eq!(
                popover
                    .rendered_content
                    .as_ref()
                    .expect("should have label text for new type hint")
                    .text,
                format!("A tooltip for `{new_type_label}`"),
                "Rendered text should not anyhow alter backticks"
            );
        });

        let struct_hint_part_hover_position = cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let previous_valid = inlay_range.start.to_display_point(&snapshot);
            let next_valid = inlay_range.end.to_display_point(&snapshot);
            assert_eq!(previous_valid.row(), next_valid.row());
            assert!(previous_valid.column() < next_valid.column());
            let exact_unclipped = DisplayPoint::new(
                previous_valid.row(),
                previous_valid.column()
                    + (entire_hint_label.find(struct_label).unwrap() + struct_label.len() / 2)
                        as u32,
            );
            PointForPosition {
                previous_valid,
                next_valid,
                exact_unclipped,
                column_overshoot_after_line_end: 0,
            }
        });
        cx.update_editor(|editor, cx| {
            update_inlay_link_and_hover_points(
                &editor.snapshot(cx),
                struct_hint_part_hover_position,
                editor,
                true,
                false,
                cx,
            );
        });
        cx.foreground()
            .advance_clock(Duration::from_millis(HOVER_DELAY_MILLIS + 100));
        cx.foreground().run_until_parked();
        cx.update_editor(|editor, cx| {
            let snapshot = editor.snapshot(cx);
            let hover_state = &editor.hover_state;
            assert!(hover_state.diagnostic_popover.is_none() && hover_state.info_popover.is_some());
            let popover = hover_state.info_popover.as_ref().unwrap();
            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            let entire_inlay_start = snapshot.display_point_to_inlay_offset(
                inlay_range.start.to_display_point(&snapshot),
                Bias::Left,
            );
            let expected_struct_label_start =
                InlayOffset(entire_inlay_start.0 + ": ".len() + new_type_label.len() + "<".len());
            // TODO kb
            // assert_eq!(
            //     popover.symbol_range,
            //     RangeInEditor::Inlay(InlayRange {
            //         inlay_position: buffer_snapshot.anchor_at(inlay_range.start, Bias::Right),
            //         highlight_start: expected_struct_label_start,
            //         highlight_end: InlayOffset(expected_struct_label_start.0 + struct_label.len()),
            //     }),
            //     "Popover range should match the struct label part"
            // );
            assert_eq!(
                popover
                    .rendered_content
                    .as_ref()
                    .expect("should have label text for struct hint")
                    .text,
                format!("A tooltip for {struct_label}"),
                "Rendered markdown element should remove backticks from text"
            );
        });
    }
}
