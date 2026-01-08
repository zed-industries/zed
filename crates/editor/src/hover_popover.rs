use crate::{
    ActiveDiagnostic, Anchor, AnchorRangeExt, DisplayPoint, DisplayRow, Editor, EditorSettings,
    EditorSnapshot, GlobalDiagnosticRenderer, Hover,
    display_map::{InlayOffset, ToDisplayPoint, is_invisible},
    hover_links::{InlayHighlight, RangeInEditor},
    movement::TextLayoutDetails,
    scroll::ScrollAmount,
};
use anyhow::Context as _;
use gpui::{
    AnyElement, AsyncWindowContext, Context, Entity, Focusable as _, FontWeight, Hsla,
    InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels, ScrollHandle, Size,
    StatefulInteractiveElement, StyleRefinement, Styled, Subscription, Task, TextStyleRefinement,
    Window, div, px,
};
use itertools::Itertools;
use language::{DiagnosticEntry, Language, LanguageRegistry};
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use multi_buffer::{MultiBufferOffset, ToOffset, ToPoint};
use project::{HoverBlock, HoverBlockKind, InlayHintLabelPart};
use settings::Settings;
use std::{borrow::Cow, cell::RefCell};
use std::{ops::Range, sync::Arc, time::Duration};
use std::{path::PathBuf, rc::Rc};
use theme::ThemeSettings;
use ui::{CopyButton, Scrollbars, WithScrollbar, prelude::*, theme_is_transparent};
use url::Url;
use util::TryFutureExt;
use workspace::{OpenOptions, OpenVisible, Workspace};

pub const MIN_POPOVER_CHARACTER_WIDTH: f32 = 20.;
pub const MIN_POPOVER_LINE_HEIGHT: f32 = 4.;
pub const POPOVER_RIGHT_OFFSET: Pixels = px(8.0);
pub const HOVER_POPOVER_GAP: Pixels = px(10.);

/// Bindable action which uses the most recent selection head to trigger a hover
pub fn hover(editor: &mut Editor, _: &Hover, window: &mut Window, cx: &mut Context<Editor>) {
    let head = editor.selections.newest_anchor().head();
    show_hover(editor, head, true, window, cx);
}

/// The internal hover action dispatches between `show_hover` or `hide_hover`
/// depending on whether a point to hover over is provided.
pub fn hover_at(
    editor: &mut Editor,
    anchor: Option<Anchor>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if EditorSettings::get_global(cx).hover_popover_enabled {
        if show_keyboard_hover(editor, window, cx) {
            return;
        }
        if let Some(anchor) = anchor {
            show_hover(editor, anchor, false, window, cx);
        } else {
            hide_hover(editor, cx);
        }
    }
}

pub fn show_keyboard_hover(
    editor: &mut Editor,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> bool {
    if let Some(anchor) = editor.hover_state.info_popovers.iter().find_map(|p| {
        if *p.keyboard_grace.borrow() {
            p.anchor
        } else {
            None
        }
    }) {
        show_hover(editor, anchor, false, window, cx);
        return true;
    }

    if let Some(anchor) = editor
        .hover_state
        .diagnostic_popover
        .as_ref()
        .and_then(|d| {
            if *d.keyboard_grace.borrow() {
                Some(d.anchor)
            } else {
                None
            }
        })
    {
        show_hover(editor, anchor, false, window, cx);
        return true;
    }

    false
}

pub struct InlayHover {
    pub(crate) range: InlayHighlight,
    pub tooltip: HoverBlock,
}

pub fn find_hovered_hint_part(
    label_parts: Vec<InlayHintLabelPart>,
    hint_start: InlayOffset,
    hovered_offset: InlayOffset,
) -> Option<(InlayHintLabelPart, Range<InlayOffset>)> {
    if hovered_offset >= hint_start {
        let mut offset_in_hint = hovered_offset - hint_start;
        let mut part_start = hint_start;
        for part in label_parts {
            let part_len = part.value.len();
            if offset_in_hint >= part_len {
                offset_in_hint -= part_len;
                part_start.0 += part_len;
            } else {
                let part_end = InlayOffset(part_start.0 + part_len);
                return Some((part, part_start..part_end));
            }
        }
    }
    None
}

pub fn hover_at_inlay(
    editor: &mut Editor,
    inlay_hover: InlayHover,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    if EditorSettings::get_global(cx).hover_popover_enabled {
        if editor.pending_rename.is_some() {
            return;
        }

        let Some(project) = editor.project.clone() else {
            return;
        };

        if editor
            .hover_state
            .info_popovers
            .iter()
            .any(|InfoPopover { symbol_range, .. }| {
                if let RangeInEditor::Inlay(range) = symbol_range
                    && range == &inlay_hover.range
                {
                    // Hover triggered from same location as last time. Don't show again.
                    return true;
                }
                false
            })
        {
            return;
        }

        let hover_popover_delay = EditorSettings::get_global(cx).hover_popover_delay.0;

        let task = cx.spawn_in(window, async move |this, cx| {
            async move {
                cx.background_executor()
                    .timer(Duration::from_millis(hover_popover_delay))
                    .await;
                this.update(cx, |this, _| {
                    this.hover_state.diagnostic_popover = None;
                })?;

                let language_registry = project.read_with(cx, |p, _| p.languages().clone());
                let blocks = vec![inlay_hover.tooltip];
                let parsed_content =
                    parse_blocks(&blocks, Some(&language_registry), None, cx).await;

                let scroll_handle = ScrollHandle::new();

                let subscription = this
                    .update(cx, |_, cx| {
                        parsed_content.as_ref().map(|parsed_content| {
                            cx.observe(parsed_content, |_, _, cx| cx.notify())
                        })
                    })
                    .ok()
                    .flatten();

                let hover_popover = InfoPopover {
                    symbol_range: RangeInEditor::Inlay(inlay_hover.range.clone()),
                    parsed_content,
                    scroll_handle,
                    keyboard_grace: Rc::new(RefCell::new(false)),
                    anchor: None,
                    _subscription: subscription,
                };

                this.update(cx, |this, cx| {
                    // TODO: no background highlights happen for inlays currently
                    this.hover_state.info_popovers = vec![hover_popover];
                    cx.notify();
                })?;

                anyhow::Ok(())
            }
            .log_err()
            .await
        });

        editor.hover_state.info_task = Some(task);
    }
}

/// Hides the type information popup.
/// Triggered by the `Hover` action when the cursor is not over a symbol or when the
/// selections changed.
pub fn hide_hover(editor: &mut Editor, cx: &mut Context<Editor>) -> bool {
    let info_popovers = editor.hover_state.info_popovers.drain(..);
    let diagnostics_popover = editor.hover_state.diagnostic_popover.take();
    let did_hide = info_popovers.count() > 0 || diagnostics_popover.is_some();

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
    anchor: Anchor,
    ignore_timeout: bool,
    window: &mut Window,
    cx: &mut Context<Editor>,
) -> Option<()> {
    if editor.pending_rename.is_some() {
        return None;
    }

    let snapshot = editor.snapshot(window, cx);

    let (buffer, buffer_position) = editor
        .buffer
        .read(cx)
        .text_anchor_for_position(anchor, cx)?;

    let (excerpt_id, _, _) = editor.buffer().read(cx).excerpt_containing(anchor, cx)?;

    let language_registry = editor
        .project()
        .map(|project| project.read(cx).languages().clone());
    let provider = editor.semantics_provider.clone()?;

    if !ignore_timeout {
        if same_info_hover(editor, &snapshot, anchor)
            || same_diagnostic_hover(editor, &snapshot, anchor)
            || editor.hover_state.diagnostic_popover.is_some()
        {
            // Hover triggered from same location as last time. Don't show again.
            return None;
        } else {
            hide_hover(editor, cx);
        }
    }

    // Don't request again if the location is the same as the previous request
    if let Some(triggered_from) = &editor.hover_state.triggered_from
        && triggered_from
            .cmp(&anchor, &snapshot.buffer_snapshot())
            .is_eq()
    {
        return None;
    }

    let hover_popover_delay = EditorSettings::get_global(cx).hover_popover_delay.0;
    let all_diagnostics_active = editor.active_diagnostics == ActiveDiagnostic::All;
    let active_group_id = if let ActiveDiagnostic::Group(group) = &editor.active_diagnostics {
        Some(group.group_id)
    } else {
        None
    };

    let renderer = GlobalDiagnosticRenderer::global(cx);
    let task = cx.spawn_in(window, async move |this, cx| {
        async move {
            // If we need to delay, delay a set amount initially before making the lsp request
            let delay = if ignore_timeout {
                None
            } else {
                let lsp_request_early = hover_popover_delay / 2;
                cx.background_executor()
                    .timer(Duration::from_millis(
                        hover_popover_delay - lsp_request_early,
                    ))
                    .await;

                // Construct delay task to wait for later
                let total_delay = Some(
                    cx.background_executor()
                        .timer(Duration::from_millis(lsp_request_early)),
                );
                total_delay
            };

            let hover_request = cx.update(|_, cx| provider.hover(&buffer, buffer_position, cx))?;

            if let Some(delay) = delay {
                delay.await;
            }
            let offset = anchor.to_offset(&snapshot.buffer_snapshot());
            let local_diagnostic = if all_diagnostics_active {
                None
            } else {
                snapshot
                    .buffer_snapshot()
                    .diagnostics_with_buffer_ids_in_range::<MultiBufferOffset>(offset..offset)
                    .filter(|(_, diagnostic)| {
                        Some(diagnostic.diagnostic.group_id) != active_group_id
                    })
                    // Find the entry with the most specific range
                    .min_by_key(|(_, entry)| entry.range.end - entry.range.start)
            };

            let diagnostic_popover = if let Some((buffer_id, local_diagnostic)) = local_diagnostic {
                let group = snapshot
                    .buffer_snapshot()
                    .diagnostic_group(buffer_id, local_diagnostic.diagnostic.group_id)
                    .collect::<Vec<_>>();
                let point_range = local_diagnostic
                    .range
                    .start
                    .to_point(&snapshot.buffer_snapshot())
                    ..local_diagnostic
                        .range
                        .end
                        .to_point(&snapshot.buffer_snapshot());
                let markdown = cx.update(|_, cx| {
                    renderer
                        .as_ref()
                        .and_then(|renderer| {
                            renderer.render_hover(
                                group,
                                point_range,
                                buffer_id,
                                language_registry.clone(),
                                cx,
                            )
                        })
                        .context("no rendered diagnostic")
                })??;

                let (background_color, border_color) = cx.update(|_, cx| {
                    let status_colors = cx.theme().status();
                    match local_diagnostic.diagnostic.severity {
                        DiagnosticSeverity::ERROR => {
                            (status_colors.error_background, status_colors.error_border)
                        }
                        DiagnosticSeverity::WARNING => (
                            status_colors.warning_background,
                            status_colors.warning_border,
                        ),
                        DiagnosticSeverity::INFORMATION => {
                            (status_colors.info_background, status_colors.info_border)
                        }
                        DiagnosticSeverity::HINT => {
                            (status_colors.hint_background, status_colors.hint_border)
                        }
                        _ => (
                            status_colors.ignored_background,
                            status_colors.ignored_border,
                        ),
                    }
                })?;

                let subscription =
                    this.update(cx, |_, cx| cx.observe(&markdown, |_, _, cx| cx.notify()))?;

                let local_diagnostic = DiagnosticEntry {
                    diagnostic: local_diagnostic.diagnostic.to_owned(),
                    range: snapshot
                        .buffer_snapshot()
                        .anchor_before(local_diagnostic.range.start)
                        ..snapshot
                            .buffer_snapshot()
                            .anchor_after(local_diagnostic.range.end),
                };

                let scroll_handle = ScrollHandle::new();

                Some(DiagnosticPopover {
                    local_diagnostic,
                    markdown,
                    border_color,
                    scroll_handle,
                    background_color,
                    keyboard_grace: Rc::new(RefCell::new(ignore_timeout)),
                    anchor,
                    _subscription: subscription,
                })
            } else {
                None
            };

            this.update(cx, |this, _| {
                this.hover_state.diagnostic_popover = diagnostic_popover;
            })?;

            let invisible_char = if let Some(invisible) = snapshot
                .buffer_snapshot()
                .chars_at(anchor)
                .next()
                .filter(|&c| is_invisible(c))
            {
                let after = snapshot.buffer_snapshot().anchor_after(
                    anchor.to_offset(&snapshot.buffer_snapshot()) + invisible.len_utf8(),
                );
                Some((invisible, anchor..after))
            } else if let Some(invisible) = snapshot
                .buffer_snapshot()
                .reversed_chars_at(anchor)
                .next()
                .filter(|&c| is_invisible(c))
            {
                let before = snapshot.buffer_snapshot().anchor_before(
                    anchor.to_offset(&snapshot.buffer_snapshot()) - invisible.len_utf8(),
                );

                Some((invisible, before..anchor))
            } else {
                None
            };

            let hovers_response = if let Some(hover_request) = hover_request {
                hover_request.await.unwrap_or_default()
            } else {
                Vec::new()
            };
            let snapshot = this.update_in(cx, |this, window, cx| this.snapshot(window, cx))?;
            let mut hover_highlights = Vec::with_capacity(hovers_response.len());
            let mut info_popovers = Vec::with_capacity(
                hovers_response.len() + if invisible_char.is_some() { 1 } else { 0 },
            );

            if let Some((invisible, range)) = invisible_char {
                let blocks = vec![HoverBlock {
                    text: format!("Unicode character U+{:02X}", invisible as u32),
                    kind: HoverBlockKind::PlainText,
                }];
                let parsed_content =
                    parse_blocks(&blocks, language_registry.as_ref(), None, cx).await;
                let scroll_handle = ScrollHandle::new();
                let subscription = this
                    .update(cx, |_, cx| {
                        parsed_content.as_ref().map(|parsed_content| {
                            cx.observe(parsed_content, |_, _, cx| cx.notify())
                        })
                    })
                    .ok()
                    .flatten();
                info_popovers.push(InfoPopover {
                    symbol_range: RangeInEditor::Text(range),
                    parsed_content,
                    scroll_handle,
                    keyboard_grace: Rc::new(RefCell::new(ignore_timeout)),
                    anchor: Some(anchor),
                    _subscription: subscription,
                })
            }

            for hover_result in hovers_response {
                // Create symbol range of anchors for highlighting and filtering of future requests.
                let range = hover_result
                    .range
                    .and_then(|range| {
                        let range = snapshot
                            .buffer_snapshot()
                            .anchor_range_in_excerpt(excerpt_id, range)?;
                        Some(range)
                    })
                    .or_else(|| {
                        let snapshot = &snapshot.buffer_snapshot();
                        let range = snapshot.syntax_ancestor(anchor..anchor)?.1;
                        Some(snapshot.anchor_before(range.start)..snapshot.anchor_after(range.end))
                    })
                    .unwrap_or_else(|| anchor..anchor);

                let blocks = hover_result.contents;
                let language = hover_result.language;
                let parsed_content =
                    parse_blocks(&blocks, language_registry.as_ref(), language, cx).await;
                let scroll_handle = ScrollHandle::new();
                hover_highlights.push(range.clone());
                let subscription = this
                    .update(cx, |_, cx| {
                        parsed_content.as_ref().map(|parsed_content| {
                            cx.observe(parsed_content, |_, _, cx| cx.notify())
                        })
                    })
                    .ok()
                    .flatten();
                info_popovers.push(InfoPopover {
                    symbol_range: RangeInEditor::Text(range),
                    parsed_content,
                    scroll_handle,
                    keyboard_grace: Rc::new(RefCell::new(ignore_timeout)),
                    anchor: Some(anchor),
                    _subscription: subscription,
                });
            }

            this.update_in(cx, |editor, window, cx| {
                if hover_highlights.is_empty() {
                    editor.clear_background_highlights::<HoverState>(cx);
                } else {
                    // Highlight the selected symbol using a background highlight
                    editor.highlight_background::<HoverState>(
                        &hover_highlights,
                        |_, theme| theme.colors().element_hover, // todo update theme
                        cx,
                    );
                }

                editor.hover_state.info_popovers = info_popovers;
                cx.notify();
                window.refresh();
            })?;

            anyhow::Ok(())
        }
        .log_err()
        .await
    });

    editor.hover_state.info_task = Some(task);
    None
}

fn same_info_hover(editor: &Editor, snapshot: &EditorSnapshot, anchor: Anchor) -> bool {
    editor
        .hover_state
        .info_popovers
        .iter()
        .any(|InfoPopover { symbol_range, .. }| {
            symbol_range
                .as_text_range()
                .map(|range| {
                    let hover_range = range.to_offset(&snapshot.buffer_snapshot());
                    let offset = anchor.to_offset(&snapshot.buffer_snapshot());
                    // LSP returns a hover result for the end index of ranges that should be hovered, so we need to
                    // use an inclusive range here to check if we should dismiss the popover
                    (hover_range.start..=hover_range.end).contains(&offset)
                })
                .unwrap_or(false)
        })
}

fn same_diagnostic_hover(editor: &Editor, snapshot: &EditorSnapshot, anchor: Anchor) -> bool {
    editor
        .hover_state
        .diagnostic_popover
        .as_ref()
        .map(|diagnostic| {
            let hover_range = diagnostic
                .local_diagnostic
                .range
                .to_offset(&snapshot.buffer_snapshot());
            let offset = anchor.to_offset(&snapshot.buffer_snapshot());

            // Here we do basically the same as in `same_info_hover`, see comment there for an explanation
            (hover_range.start..=hover_range.end).contains(&offset)
        })
        .unwrap_or(false)
}

async fn parse_blocks(
    blocks: &[HoverBlock],
    language_registry: Option<&Arc<LanguageRegistry>>,
    language: Option<Arc<Language>>,
    cx: &mut AsyncWindowContext,
) -> Option<Entity<Markdown>> {
    let combined_text = blocks
        .iter()
        .map(|block| match &block.kind {
            project::HoverBlockKind::PlainText | project::HoverBlockKind::Markdown => {
                Cow::Borrowed(block.text.trim())
            }
            project::HoverBlockKind::Code { language } => {
                Cow::Owned(format!("```{}\n{}\n```", language, block.text.trim()))
            }
        })
        .join("\n\n");

    cx.new_window_entity(|_window, cx| {
        Markdown::new(
            combined_text.into(),
            language_registry.cloned(),
            language.map(|language| language.name()),
            cx,
        )
    })
    .ok()
}

pub fn hover_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let settings = ThemeSettings::get_global(cx);
    let ui_font_family = settings.ui_font.family.clone();
    let ui_font_features = settings.ui_font.features.clone();
    let ui_font_fallbacks = settings.ui_font.fallbacks.clone();
    let buffer_font_family = settings.buffer_font.family.clone();
    let buffer_font_features = settings.buffer_font.features.clone();
    let buffer_font_fallbacks = settings.buffer_font.fallbacks.clone();

    let mut base_text_style = window.text_style();
    base_text_style.refine(&TextStyleRefinement {
        font_family: Some(ui_font_family),
        font_features: Some(ui_font_features),
        font_fallbacks: ui_font_fallbacks,
        color: Some(cx.theme().colors().editor_foreground),
        ..Default::default()
    });
    MarkdownStyle {
        base_text_style,
        code_block: StyleRefinement::default()
            .my(rems(1.))
            .font_buffer(cx)
            .font_features(buffer_font_features.clone()),
        inline_code: TextStyleRefinement {
            background_color: Some(cx.theme().colors().background),
            font_family: Some(buffer_font_family),
            font_features: Some(buffer_font_features),
            font_fallbacks: buffer_font_fallbacks,
            ..Default::default()
        },
        rule_color: cx.theme().colors().border,
        block_quote_border_color: Color::Muted.color(cx),
        block_quote: TextStyleRefinement {
            color: Some(Color::Muted.color(cx)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            underline: Some(gpui::UnderlineStyle {
                thickness: px(1.),
                color: Some(cx.theme().colors().editor_foreground),
                wavy: false,
            }),
            ..Default::default()
        },
        syntax: cx.theme().syntax().clone(),
        selection_background_color: cx.theme().colors().element_selection_background,
        heading: StyleRefinement::default()
            .font_weight(FontWeight::BOLD)
            .text_base()
            .mt(rems(1.))
            .mb_0(),
        table_columns_min_size: true,
        ..Default::default()
    }
}

pub fn diagnostics_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let settings = ThemeSettings::get_global(cx);
    let ui_font_family = settings.ui_font.family.clone();
    let ui_font_fallbacks = settings.ui_font.fallbacks.clone();
    let ui_font_features = settings.ui_font.features.clone();
    let buffer_font_family = settings.buffer_font.family.clone();
    let buffer_font_features = settings.buffer_font.features.clone();
    let buffer_font_fallbacks = settings.buffer_font.fallbacks.clone();

    let mut base_text_style = window.text_style();
    base_text_style.refine(&TextStyleRefinement {
        font_family: Some(ui_font_family),
        font_features: Some(ui_font_features),
        font_fallbacks: ui_font_fallbacks,
        color: Some(cx.theme().colors().editor_foreground),
        ..Default::default()
    });
    MarkdownStyle {
        base_text_style,
        code_block: StyleRefinement::default().my(rems(1.)).font_buffer(cx),
        inline_code: TextStyleRefinement {
            background_color: Some(cx.theme().colors().editor_background.opacity(0.5)),
            font_family: Some(buffer_font_family),
            font_features: Some(buffer_font_features),
            font_fallbacks: buffer_font_fallbacks,
            ..Default::default()
        },
        rule_color: cx.theme().colors().border,
        block_quote_border_color: Color::Muted.color(cx),
        block_quote: TextStyleRefinement {
            color: Some(Color::Muted.color(cx)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            color: Some(cx.theme().colors().editor_foreground),
            underline: Some(gpui::UnderlineStyle {
                thickness: px(1.),
                color: Some(cx.theme().colors().editor_foreground),
                wavy: false,
            }),
            ..Default::default()
        },
        syntax: cx.theme().syntax().clone(),
        selection_background_color: cx.theme().colors().element_selection_background,
        height_is_multiple_of_line_height: true,
        heading: StyleRefinement::default()
            .font_weight(FontWeight::BOLD)
            .text_base()
            .mb_0(),
        table_columns_min_size: true,
        ..Default::default()
    }
}

pub fn open_markdown_url(link: SharedString, window: &mut Window, cx: &mut App) {
    if let Ok(uri) = Url::parse(&link)
        && uri.scheme() == "file"
        && let Some(workspace) = window.root::<Workspace>().flatten()
    {
        workspace.update(cx, |workspace, cx| {
            let task = workspace.open_abs_path(
                PathBuf::from(uri.path()),
                OpenOptions {
                    visible: Some(OpenVisible::None),
                    ..Default::default()
                },
                window,
                cx,
            );

            cx.spawn_in(window, async move |_, cx| {
                let item = task.await?;
                // Ruby LSP uses URLs with #L1,1-4,4
                // we'll just take the first number and assume it's a line number
                let Some(fragment) = uri.fragment() else {
                    return anyhow::Ok(());
                };
                let mut accum = 0u32;
                for c in fragment.chars() {
                    if c >= '0' && c <= '9' && accum < u32::MAX / 2 {
                        accum *= 10;
                        accum += c as u32 - '0' as u32;
                    } else if accum > 0 {
                        break;
                    }
                }
                if accum == 0 {
                    return Ok(());
                }
                let Some(editor) = cx.update(|_, cx| item.act_as::<Editor>(cx))? else {
                    return Ok(());
                };
                editor.update_in(cx, |editor, window, cx| {
                    editor.change_selections(Default::default(), window, cx, |selections| {
                        selections.select_ranges([
                            text::Point::new(accum - 1, 0)..text::Point::new(accum - 1, 0)
                        ]);
                    });
                })
            })
            .detach_and_log_err(cx);
        });
        return;
    }
    cx.open_url(&link);
}

#[derive(Default)]
pub struct HoverState {
    pub info_popovers: Vec<InfoPopover>,
    pub diagnostic_popover: Option<DiagnosticPopover>,
    pub triggered_from: Option<Anchor>,
    pub info_task: Option<Task<Option<()>>>,
}

impl HoverState {
    pub fn visible(&self) -> bool {
        !self.info_popovers.is_empty() || self.diagnostic_popover.is_some()
    }

    pub(crate) fn render(
        &mut self,
        snapshot: &EditorSnapshot,
        visible_rows: Range<DisplayRow>,
        max_size: Size<Pixels>,
        text_layout_details: &TextLayoutDetails,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Option<(DisplayPoint, Vec<AnyElement>)> {
        if !self.visible() {
            return None;
        }
        // If there is a diagnostic, position the popovers based on that.
        // Otherwise use the start of the hover range
        let anchor = self
            .diagnostic_popover
            .as_ref()
            .map(|diagnostic_popover| &diagnostic_popover.local_diagnostic.range.start)
            .or_else(|| {
                self.info_popovers.iter().find_map(|info_popover| {
                    match &info_popover.symbol_range {
                        RangeInEditor::Text(range) => Some(&range.start),
                        RangeInEditor::Inlay(_) => None,
                    }
                })
            })
            .or_else(|| {
                self.info_popovers.iter().find_map(|info_popover| {
                    match &info_popover.symbol_range {
                        RangeInEditor::Text(_) => None,
                        RangeInEditor::Inlay(range) => Some(&range.inlay_position),
                    }
                })
            })?;
        let mut point = anchor.to_display_point(&snapshot.display_snapshot);
        // Clamp the point within the visible rows in case the popup source spans multiple lines
        if visible_rows.end <= point.row() {
            point = crate::movement::up_by_rows(
                &snapshot.display_snapshot,
                point,
                1 + (point.row() - visible_rows.end).0,
                text::SelectionGoal::None,
                true,
                text_layout_details,
            )
            .0;
        } else if point.row() < visible_rows.start {
            point = crate::movement::down_by_rows(
                &snapshot.display_snapshot,
                point,
                (visible_rows.start - point.row()).0,
                text::SelectionGoal::None,
                true,
                text_layout_details,
            )
            .0;
        }

        if !visible_rows.contains(&point.row()) {
            log::error!("Hover popover point out of bounds after moving");
            return None;
        }

        let mut elements = Vec::new();

        if let Some(diagnostic_popover) = self.diagnostic_popover.as_ref() {
            elements.push(diagnostic_popover.render(max_size, window, cx));
        }
        for info_popover in &mut self.info_popovers {
            elements.push(info_popover.render(max_size, window, cx));
        }

        Some((point, elements))
    }

    pub fn focused(&self, window: &mut Window, cx: &mut Context<Editor>) -> bool {
        let mut hover_popover_is_focused = false;
        for info_popover in &self.info_popovers {
            if let Some(markdown_view) = &info_popover.parsed_content
                && markdown_view.focus_handle(cx).is_focused(window)
            {
                hover_popover_is_focused = true;
            }
        }
        if let Some(diagnostic_popover) = &self.diagnostic_popover
            && diagnostic_popover
                .markdown
                .focus_handle(cx)
                .is_focused(window)
        {
            hover_popover_is_focused = true;
        }
        hover_popover_is_focused
    }
}

pub struct InfoPopover {
    pub symbol_range: RangeInEditor,
    pub parsed_content: Option<Entity<Markdown>>,
    pub scroll_handle: ScrollHandle,
    pub keyboard_grace: Rc<RefCell<bool>>,
    pub anchor: Option<Anchor>,
    _subscription: Option<Subscription>,
}

impl InfoPopover {
    pub(crate) fn render(
        &mut self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        let keyboard_grace = Rc::clone(&self.keyboard_grace);
        div()
            .id("info_popover")
            .occlude()
            .elevation_2(cx)
            // Prevent a mouse down/move on the popover from being propagated to the editor,
            // because that would dismiss the popover.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                let mut keyboard_grace = keyboard_grace.borrow_mut();
                *keyboard_grace = false;
                cx.stop_propagation();
            })
            .when_some(self.parsed_content.clone(), |this, markdown| {
                this.child(
                    div()
                        .id("info-md-container")
                        .overflow_y_scroll()
                        .max_w(max_size.width)
                        .max_h(max_size.height)
                        .track_scroll(&self.scroll_handle)
                        .child(
                            MarkdownElement::new(markdown, hover_markdown_style(window, cx))
                                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                    copy_button: false,
                                    copy_button_on_hover: false,
                                    border: false,
                                })
                                .on_url_click(open_markdown_url)
                                .p_2(),
                        ),
                )
                .custom_scrollbars(
                    Scrollbars::for_settings::<EditorSettings>()
                        .tracked_scroll_handle(&self.scroll_handle),
                    window,
                    cx,
                )
            })
            .into_any_element()
    }

    pub fn scroll(&self, amount: ScrollAmount, window: &mut Window, cx: &mut Context<Editor>) {
        let mut current = self.scroll_handle.offset();
        current.y -= amount.pixels(
            window.line_height(),
            self.scroll_handle.bounds().size.height - px(16.),
        ) / 2.0;
        cx.notify();
        self.scroll_handle.set_offset(current);
    }
}

pub struct DiagnosticPopover {
    pub(crate) local_diagnostic: DiagnosticEntry<Anchor>,
    markdown: Entity<Markdown>,
    border_color: Hsla,
    background_color: Hsla,
    pub keyboard_grace: Rc<RefCell<bool>>,
    pub anchor: Anchor,
    _subscription: Subscription,
    pub scroll_handle: ScrollHandle,
}

impl DiagnosticPopover {
    pub fn render(
        &self,
        max_size: Size<Pixels>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        let keyboard_grace = Rc::clone(&self.keyboard_grace);
        let this = cx.entity().downgrade();
        div()
            .id("diagnostic")
            .occlude()
            .elevation_2_borderless(cx)
            // Don't draw the background color if the theme
            // allows transparent surfaces.
            .when(theme_is_transparent(cx), |this| {
                this.bg(gpui::transparent_black())
            })
            // Prevent a mouse move on the popover from being propagated to the editor,
            // because that would dismiss the popover.
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            // Prevent a mouse down on the popover from being propagated to the editor,
            // because that would move the cursor.
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                let mut keyboard_grace = keyboard_grace.borrow_mut();
                *keyboard_grace = false;
                cx.stop_propagation();
            })
            .child(
                div()
                    .py_1()
                    .px_2()
                    .bg(self.background_color)
                    .border_1()
                    .border_color(self.border_color)
                    .rounded_lg()
                    .child(
                        h_flex()
                            .id("diagnostic-content-container")
                            .gap_1()
                            .items_start()
                            .max_w(max_size.width)
                            .max_h(max_size.height)
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .child(
                                MarkdownElement::new(
                                    self.markdown.clone(),
                                    diagnostics_markdown_style(window, cx),
                                )
                                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                    copy_button: false,
                                    copy_button_on_hover: false,
                                    border: false,
                                })
                                .on_url_click(
                                    move |link, window, cx| {
                                        if let Some(renderer) = GlobalDiagnosticRenderer::global(cx)
                                        {
                                            this.update(cx, |this, cx| {
                                                renderer.as_ref().open_link(this, link, window, cx);
                                            })
                                            .ok();
                                        }
                                    },
                                ),
                            )
                            .child({
                                let message = self.local_diagnostic.diagnostic.message.clone();
                                CopyButton::new(message).tooltip_label("Copy Diagnostic")
                            }),
                    )
                    .custom_scrollbars(
                        Scrollbars::for_settings::<EditorSettings>()
                            .tracked_scroll_handle(&self.scroll_handle),
                        window,
                        cx,
                    ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        PointForPosition,
        actions::ConfirmCompletion,
        editor_tests::{handle_completion_request, init_test},
        inlays::inlay_hints::tests::{cached_hint_labels, visible_hint_labels},
        test::editor_lsp_test_context::EditorLspTestContext,
    };
    use collections::BTreeSet;
    use gpui::App;
    use indoc::indoc;
    use markdown::parser::MarkdownEvent;
    use project::InlayId;
    use settings::InlayHintSettingsContent;
    use smol::stream::StreamExt;
    use std::sync::atomic;
    use std::sync::atomic::AtomicUsize;
    use text::Bias;

    fn get_hover_popover_delay(cx: &gpui::TestAppContext) -> u64 {
        cx.read(|cx: &App| -> u64 { EditorSettings::get_global(cx).hover_popover_delay.0 })
    }

    impl InfoPopover {
        fn get_rendered_text(&self, cx: &gpui::App) -> String {
            let mut rendered_text = String::new();
            if let Some(parsed_content) = self.parsed_content.clone() {
                let markdown = parsed_content.read(cx);
                let text = markdown.parsed_markdown().source().to_string();
                let data = markdown.parsed_markdown().events();
                let slice = data;

                for (range, event) in slice.iter() {
                    match event {
                        MarkdownEvent::SubstitutedText(parsed) => {
                            rendered_text.push_str(parsed.as_str())
                        }
                        MarkdownEvent::Text | MarkdownEvent::Code => {
                            rendered_text.push_str(&text[range.clone()])
                        }
                        _ => {}
                    }
                }
            }
            rendered_text
        }
    }

    #[gpui::test]
    async fn test_mouse_hover_info_popover_with_autocomplete_popover(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx, |_| {});

        let mut cx = EditorLspTestContext::new_rust(
            lsp::ServerCapabilities {
                hover_provider: Some(lsp::HoverProviderCapability::Simple(true)),
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
        )
        .await;
        let counter = Arc::new(AtomicUsize::new(0));
        // Basic hover delays and then pops without moving the mouse
        cx.set_state(indoc! {"
                oneˇ
                two
                three
                fn test() { println!(); }
            "});

        //prompt autocompletion menu
        cx.simulate_keystroke(".");
        handle_completion_request(
            indoc! {"
                        one.|<>
                        two
                        three
                    "},
            vec!["first_completion", "second_completion"],
            true,
            counter.clone(),
            &mut cx,
        )
        .await;
        cx.condition(|editor, _| editor.context_menu_visible()) // wait until completion menu is visible
            .await;
        assert_eq!(counter.load(atomic::Ordering::Acquire), 1); // 1 completion request

        let hover_point = cx.display_point(indoc! {"
                one.
                two
                three
                fn test() { printˇln!(); }
            "});
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let anchor = snapshot
                .buffer_snapshot()
                .anchor_before(hover_point.to_offset(&snapshot, Bias::Left));
            hover_at(editor, Some(anchor), window, cx)
        });
        assert!(!cx.editor(|editor, _window, _cx| editor.hover_state.visible()));

        // After delay, hover should be visible.
        let symbol_range = cx.lsp_range(indoc! {"
                one.
                two
                three
                fn test() { «println!»(); }
            "});
        let mut requests =
            cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: "some basic docs".to_string(),
                    }),
                    range: Some(symbol_range),
                }))
            });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        requests.next().await;

        cx.editor(|editor, _window, cx| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);
            assert_eq!(rendered_text, "some basic docs".to_string())
        });

        // check that the completion menu is still visible and that there still has only been 1 completion request
        cx.editor(|editor, _, _| assert!(editor.context_menu_visible()));
        assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

        //apply a completion and check it was successfully applied
        let _apply_additional_edits = cx.update_editor(|editor, window, cx| {
            editor.context_menu_next(&Default::default(), window, cx);
            editor
                .confirm_completion(&ConfirmCompletion::default(), window, cx)
                .unwrap()
        });
        cx.assert_editor_state(indoc! {"
            one.second_completionˇ
            two
            three
            fn test() { println!(); }
        "});

        // check that the completion menu is no longer visible and that there still has only been 1 completion request
        cx.editor(|editor, _, _| assert!(!editor.context_menu_visible()));
        assert_eq!(counter.load(atomic::Ordering::Acquire), 1);

        //verify the information popover is still visible and unchanged
        cx.editor(|editor, _, cx| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);

            assert_eq!(rendered_text, "some basic docs".to_string())
        });

        // Mouse moved with no hover response dismisses
        let hover_point = cx.display_point(indoc! {"
                one.second_completionˇ
                two
                three
                fn teˇst() { println!(); }
            "});
        let mut request = cx
            .lsp
            .set_request_handler::<lsp::request::HoverRequest, _, _>(
                |_, _| async move { Ok(None) },
            );
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let anchor = snapshot
                .buffer_snapshot()
                .anchor_before(hover_point.to_offset(&snapshot, Bias::Left));
            hover_at(editor, Some(anchor), window, cx)
        });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        request.next().await;

        // verify that the information popover is no longer visible
        cx.editor(|editor, _, _| {
            assert!(!editor.hover_state.visible());
        });
    }

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

        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let anchor = snapshot
                .buffer_snapshot()
                .anchor_before(hover_point.to_offset(&snapshot, Bias::Left));
            hover_at(editor, Some(anchor), window, cx)
        });
        assert!(!cx.editor(|editor, _window, _cx| editor.hover_state.visible()));

        // After delay, hover should be visible.
        let symbol_range = cx.lsp_range(indoc! {"
            fn test() { «println!»(); }
        "});
        let mut requests =
            cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: "some basic docs".to_string(),
                    }),
                    range: Some(symbol_range),
                }))
            });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        requests.next().await;

        cx.editor(|editor, _, cx| {
            assert!(editor.hover_state.visible());
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);

            assert_eq!(rendered_text, "some basic docs".to_string())
        });

        // Mouse moved with no hover response dismisses
        let hover_point = cx.display_point(indoc! {"
            fn teˇst() { println!(); }
        "});
        let mut request = cx
            .lsp
            .set_request_handler::<lsp::request::HoverRequest, _, _>(
                |_, _| async move { Ok(None) },
            );
        cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let anchor = snapshot
                .buffer_snapshot()
                .anchor_before(hover_point.to_offset(&snapshot, Bias::Left));
            hover_at(editor, Some(anchor), window, cx)
        });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        request.next().await;
        cx.editor(|editor, _, _| {
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
        cx.update_editor(|editor, window, cx| hover(editor, &Hover, window, cx));
        let symbol_range = cx.lsp_range(indoc! {"
            «fn» test() { println!(); }
        "});

        cx.editor(|editor, _window, _cx| {
            assert!(!editor.hover_state.visible());

            assert_eq!(
                editor.hover_state.info_popovers.len(),
                0,
                "Expected no hovers but got but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
        });

        let mut requests =
            cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
                Ok(Some(lsp::Hover {
                    contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                        kind: lsp::MarkupKind::Markdown,
                        value: "some other basic docs".to_string(),
                    }),
                    range: Some(symbol_range),
                }))
            });

        requests.next().await;
        cx.dispatch_action(Hover);

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );

            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);

            assert_eq!(rendered_text, "some other basic docs".to_string())
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
        cx.update_editor(|editor, window, cx| hover(editor, &Hover, window, cx));
        let symbol_range = cx.lsp_range(indoc! {"
            «fn» test() { println!(); }
        "});
        cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| async move {
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
        cx.dispatch_action(Hover);

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);

            assert_eq!(
                rendered_text,
                "regular text for hover to show".to_string(),
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
        cx.update_editor(|editor, window, cx| hover(editor, &Hover, window, cx));
        let symbol_range = cx.lsp_range(indoc! {"
            «fn» test() { println!(); }
        "});

        let code_str = "\nlet hovered_point: Vector2F // size = 8, align = 0x4\n";
        let markdown_string = format!("\n```rust\n{code_str}```");

        let closure_markdown_string = markdown_string.clone();
        cx.set_request_handler::<lsp::request::HoverRequest, _, _>(move |_, _, _| {
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

        cx.dispatch_action(Hover);

        cx.condition(|editor, _| editor.hover_state.visible()).await;
        cx.editor(|editor, _, cx| {
            assert_eq!(
                editor.hover_state.info_popovers.len(),
                1,
                "Expected exactly one hover but got: {:?}",
                editor.hover_state.info_popovers.len()
            );
            let rendered_text = editor
                .hover_state
                .info_popovers
                .first()
                .unwrap()
                .get_rendered_text(cx);

            assert_eq!(
                rendered_text, code_str,
                "Should not have extra line breaks at end of rendered hover"
            );
        });
    }

    #[gpui::test]
    // https://github.com/zed-industries/zed/issues/15498
    async fn test_info_hover_with_hrs(cx: &mut gpui::TestAppContext) {
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
            fn fuˇnc(abc def: i32) -> u32 {
            }
        "});

        cx.lsp
            .set_request_handler::<lsp::request::HoverRequest, _, _>({
                |_, _| async move {
                    Ok(Some(lsp::Hover {
                        contents: lsp::HoverContents::Markup(lsp::MarkupContent {
                            kind: lsp::MarkupKind::Markdown,
                            value: indoc!(
                                r#"
                    ### function `errands_data_read`

                    ---
                    → `char *`
                    Function to read a file into a string

                    ---
                    ```cpp
                    static char *errands_data_read()
                    ```
                    "#
                            )
                            .to_string(),
                        }),
                        range: None,
                    }))
                }
            });
        cx.update_editor(|editor, window, cx| hover(editor, &Default::default(), window, cx));
        cx.run_until_parked();

        cx.update_editor(|editor, _, cx| {
            let popover = editor.hover_state.info_popovers.first().unwrap();
            let content = popover.get_rendered_text(cx);

            assert!(content.contains("Function to read a file"));
        });
    }

    #[gpui::test]
    async fn test_hover_inlay_label_parts(cx: &mut gpui::TestAppContext) {
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
        let hint_position = cx.to_lsp(MultiBufferOffset(hint_start_offset));
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
            .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
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
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, _, cx| {
            let expected_layers = vec![entire_hint_label.to_string()];
            assert_eq!(expected_layers, cached_hint_labels(editor, cx));
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
            .first()
            .cloned()
            .unwrap();
        let new_type_hint_part_hover_position = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let previous_valid = MultiBufferOffset(inlay_range.start).to_display_point(&snapshot);
            let next_valid = MultiBufferOffset(inlay_range.end).to_display_point(&snapshot);
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
        cx.update_editor(|editor, window, cx| {
            editor.update_inlay_link_and_hover_points(
                &editor.snapshot(window, cx),
                new_type_hint_part_hover_position,
                true,
                false,
                window,
                cx,
            );
        });

        let resolve_closure_uri = uri.clone();
        cx.lsp
            .set_request_handler::<lsp::request::InlayHintResolveRequest, _, _>(
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
        cx.background_executor.run_until_parked();

        cx.update_editor(|editor, window, cx| {
            editor.update_inlay_link_and_hover_points(
                &editor.snapshot(window, cx),
                new_type_hint_part_hover_position,
                true,
                false,
                window,
                cx,
            );
        });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, _, cx| {
            let hover_state = &editor.hover_state;
            assert!(
                hover_state.diagnostic_popover.is_none() && hover_state.info_popovers.len() == 1
            );
            let popover = hover_state.info_popovers.first().unwrap();
            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            assert_eq!(
                popover.symbol_range,
                RangeInEditor::Inlay(InlayHighlight {
                    inlay: InlayId::Hint(0),
                    inlay_position: buffer_snapshot
                        .anchor_after(MultiBufferOffset(inlay_range.start)),
                    range: ": ".len()..": ".len() + new_type_label.len(),
                }),
                "Popover range should match the new type label part"
            );
            assert_eq!(
                popover.get_rendered_text(cx),
                format!("A tooltip for {new_type_label}"),
            );
        });

        let struct_hint_part_hover_position = cx.update_editor(|editor, window, cx| {
            let snapshot = editor.snapshot(window, cx);
            let previous_valid = MultiBufferOffset(inlay_range.start).to_display_point(&snapshot);
            let next_valid = MultiBufferOffset(inlay_range.end).to_display_point(&snapshot);
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
        cx.update_editor(|editor, window, cx| {
            editor.update_inlay_link_and_hover_points(
                &editor.snapshot(window, cx),
                struct_hint_part_hover_position,
                true,
                false,
                window,
                cx,
            );
        });
        cx.background_executor
            .advance_clock(Duration::from_millis(get_hover_popover_delay(&cx) + 100));
        cx.background_executor.run_until_parked();
        cx.update_editor(|editor, _, cx| {
            let hover_state = &editor.hover_state;
            assert!(
                hover_state.diagnostic_popover.is_none() && hover_state.info_popovers.len() == 1
            );
            let popover = hover_state.info_popovers.first().unwrap();
            let buffer_snapshot = editor.buffer().update(cx, |buffer, cx| buffer.snapshot(cx));
            assert_eq!(
                popover.symbol_range,
                RangeInEditor::Inlay(InlayHighlight {
                    inlay: InlayId::Hint(0),
                    inlay_position: buffer_snapshot
                        .anchor_after(MultiBufferOffset(inlay_range.start)),
                    range: ": ".len() + new_type_label.len() + "<".len()
                        ..": ".len() + new_type_label.len() + "<".len() + struct_label.len(),
                }),
                "Popover range should match the struct label part"
            );
            assert_eq!(
                popover.get_rendered_text(cx),
                format!("A tooltip for {struct_label}"),
                "Rendered markdown element should remove backticks from text"
            );
        });
    }

    #[test]
    fn test_find_hovered_hint_part_with_multibyte_characters() {
        use crate::display_map::InlayOffset;
        use multi_buffer::MultiBufferOffset;
        use project::InlayHintLabelPart;

        // Test with multi-byte UTF-8 character "→" (3 bytes, 1 character)
        let label = "→ app/Livewire/UserProfile.php";
        let label_parts = vec![InlayHintLabelPart {
            value: label.to_string(),
            tooltip: None,
            location: None,
        }];

        let hint_start = InlayOffset(MultiBufferOffset(100));

        // Verify the label has more bytes than characters (due to "→")
        assert_eq!(label.len(), 32); // bytes
        assert_eq!(label.chars().count(), 30); // characters

        // Test hovering at the last byte (should find the part)
        let last_byte_offset = InlayOffset(MultiBufferOffset(100 + label.len() - 1));
        let result = find_hovered_hint_part(label_parts.clone(), hint_start, last_byte_offset);
        assert!(
            result.is_some(),
            "Should find part when hovering at last byte"
        );
        let (part, range) = result.unwrap();
        assert_eq!(part.value, label);
        assert_eq!(range.start, hint_start);
        assert_eq!(range.end, InlayOffset(MultiBufferOffset(100 + label.len())));

        // Test hovering at the first byte of "→" (byte 0)
        let first_byte_offset = InlayOffset(MultiBufferOffset(100));
        let result = find_hovered_hint_part(label_parts.clone(), hint_start, first_byte_offset);
        assert!(
            result.is_some(),
            "Should find part when hovering at first byte"
        );

        // Test hovering in the middle of "→" (byte 1, still part of the arrow character)
        let mid_arrow_offset = InlayOffset(MultiBufferOffset(101));
        let result = find_hovered_hint_part(label_parts, hint_start, mid_arrow_offset);
        assert!(
            result.is_some(),
            "Should find part when hovering in middle of multi-byte char"
        );

        // Test with multiple parts containing multi-byte characters
        // Part ranges are [start, end) - start inclusive, end exclusive
        // "→ " occupies bytes [0, 4), "path" occupies bytes [4, 8)
        let parts = vec![
            InlayHintLabelPart {
                value: "→ ".to_string(), // 4 bytes (3 + 1)
                tooltip: None,
                location: None,
            },
            InlayHintLabelPart {
                value: "path".to_string(), // 4 bytes
                tooltip: None,
                location: None,
            },
        ];

        // Hover at byte 3 (last byte of "→ ", the space character)
        let arrow_last_byte = InlayOffset(MultiBufferOffset(100 + 3));
        let result = find_hovered_hint_part(parts.clone(), hint_start, arrow_last_byte);
        assert!(result.is_some(), "Should find first part at its last byte");
        let (part, range) = result.unwrap();
        assert_eq!(part.value, "→ ");
        assert_eq!(
            range,
            InlayOffset(MultiBufferOffset(100))..InlayOffset(MultiBufferOffset(104))
        );

        // Hover at byte 4 (first byte of "path", at the boundary)
        let path_start_offset = InlayOffset(MultiBufferOffset(100 + 4));
        let result = find_hovered_hint_part(parts.clone(), hint_start, path_start_offset);
        assert!(result.is_some(), "Should find second part at boundary");
        let (part, _) = result.unwrap();
        assert_eq!(part.value, "path");

        // Hover at byte 7 (last byte of "path")
        let path_end_offset = InlayOffset(MultiBufferOffset(100 + 7));
        let result = find_hovered_hint_part(parts, hint_start, path_end_offset);
        assert!(result.is_some(), "Should find second part at last byte");
        let (part, range) = result.unwrap();
        assert_eq!(part.value, "path");
        assert_eq!(
            range,
            InlayOffset(MultiBufferOffset(104))..InlayOffset(MultiBufferOffset(108))
        );
    }
}
