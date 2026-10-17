use super::{CursorLayout, Invisible, layout_chunk_renderer_with_width};
use crate::{
    ChunkRenderer, ChunkReplacement,
    display_map::{ChunkRendererId, HighlightedChunk, LongRow, RendererTextSource},
};
use gpui::{
    AnyElement, App, AvailableSpace, BackgroundExecutor, Font, FontRun, HighlightStyle, LineLayout,
    LineWrapper, Pixels, RetainedLineLayout, SharedString, Size, TextRun, TextStyle, TextSystem,
    Window,
};

use std::{
    ops::Range,
    sync::{Arc, OnceLock},
};
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) struct PreparedRowText {
    text: SharedString,
    text_spans: Vec<(Range<usize>, usize)>,
    chunks: Vec<PreparedChunk>,
    paint_runs: OnceLock<Vec<PreparedPaintRun>>,
    diagnostics: Vec<(Range<usize>, lsp::DiagnosticSeverity)>,
    invisibles: Vec<Invisible>,
    oversized_cursor_widths: Vec<(Range<u32>, OnceLock<Pixels>)>,
    drop_executor: Option<BackgroundExecutor>,
    #[cfg(test)]
    cursor_text_input_bytes: AtomicUsize,
    #[cfg(test)]
    paint_style_lookups: AtomicUsize,
}

#[derive(Clone)]
pub(crate) struct PreparedRendererConstraint {
    renderer: ChunkRendererId,
    text: RendererTextSource,
    font: Font,
    font_size: Pixels,
    width: Option<Pixels>,
}

pub(crate) enum LineChunk<'a> {
    Text(HighlightedChunk<'a>),
    Replacement {
        len: usize,
        replacement: ChunkReplacement,
        style: Option<HighlightStyle>,
        diagnostic_underline_severity: Option<lsp::DiagnosticSeverity>,
    },
}

struct PreparedPaintRun {
    columns: Range<usize>,
    style: TextRun,
}

impl PreparedRowText {
    pub(crate) const CURSOR_TEXT_LIMIT: u32 = 1_024;

    pub(crate) fn new<'a>(highlighted_chunks: impl Iterator<Item = HighlightedChunk<'a>>) -> Self {
        let mut text = String::new();
        let mut text_spans: Vec<(Range<usize>, usize)> = Vec::new();
        let mut chunks = Vec::<PreparedChunk>::new();
        let mut invisibles = Vec::new();
        let mut column = 0;
        for chunk in highlighted_chunks {
            let columns = column..column + chunk.text.len();
            column = columns.end;
            let text_offset = text.len();
            if !matches!(&chunk.replacement, Some(ChunkReplacement::Renderer(_))) {
                if let Some((span, _)) = text_spans.last_mut()
                    && span.end == columns.start
                    && !chunks.last().is_some_and(|chunk| {
                        matches!(&chunk.replacement, Some(ChunkReplacement::Renderer(_)))
                    })
                {
                    span.end = columns.end;
                } else {
                    text_spans.push((columns.clone(), text_offset));
                }
                text.push_str(chunk.text);
            }
            if chunk.replacement.is_none() && !chunk.is_inlay {
                if chunk.is_tab {
                    invisibles.push(Invisible::Tab {
                        line_start_offset: columns.start,
                        line_end_offset: columns.end,
                    });
                } else {
                    invisibles.extend(chunk.text.char_indices().filter_map(
                        |(offset, character)| {
                            character.is_whitespace().then_some(Invisible::Whitespace {
                                line_start_offset: columns.start + offset,
                                line_end_offset: columns.start + offset + character.len_utf8(),
                            })
                        },
                    ));
                }
            }
            if let Some(previous) = chunks.last_mut()
                && previous.columns.end == columns.start
                && ((previous.replacement.is_none()
                    && chunk.replacement.is_none()
                    && previous.style == chunk.style
                    && previous.diagnostic_underline_severity
                        == chunk.diagnostic_underline_severity
                    && previous.is_tab == chunk.is_tab
                    && previous.is_inlay == chunk.is_inlay)
                    || matches!((&previous.replacement, &chunk.replacement), (Some(ChunkReplacement::Renderer(left)), Some(ChunkReplacement::Renderer(right))) if left.id == right.id))
            {
                previous.columns.end = columns.end;
                continue;
            }
            chunks.push(PreparedChunk {
                columns,
                text_offset,
                style: chunk.style,
                diagnostic_underline_severity: chunk.diagnostic_underline_severity,
                is_tab: chunk.is_tab,
                is_inlay: chunk.is_inlay,
                replacement: chunk.replacement,
            });
        }
        let oversized_cursor_widths = text_spans
            .iter()
            .flat_map(|(columns, offset)| {
                text[*offset..*offset + columns.len()]
                    .grapheme_indices(true)
                    .filter(|(_, grapheme)| grapheme.len() > Self::CURSOR_TEXT_LIMIT as usize)
                    .map(move |(start, grapheme)| {
                        let start = columns.start + start;
                        (
                            start as u32..(start + grapheme.len()) as u32,
                            OnceLock::new(),
                        )
                    })
            })
            .collect();
        let diagnostics = chunks
            .iter()
            .filter_map(|chunk| {
                chunk
                    .diagnostic_underline_severity
                    .map(|severity| (chunk.columns.clone(), severity))
            })
            .collect();
        Self {
            text: SharedString::from(text),
            text_spans,
            chunks,
            paint_runs: OnceLock::new(),
            diagnostics,
            invisibles,
            oversized_cursor_widths,
            drop_executor: None,
            #[cfg(test)]
            cursor_text_input_bytes: AtomicUsize::new(0),
            #[cfg(test)]
            paint_style_lookups: AtomicUsize::new(0),
        }
    }

    pub(crate) fn prepare_cursor_widths(
        &self,
        row: &LongRow,
        style: &TextStyle,
        wrapper: &LineWrapper,
        text_system: &TextSystem,
    ) {
        self.paint_runs.get_or_init(|| {
            let mut runs = Vec::<PreparedPaintRun>::new();
            for chunk in &self.chunks {
                let style = chunk.style.map_or_else(
                    || style.clone(),
                    |highlight| style.clone().highlight(highlight),
                );
                let style = TextRun {
                    len: chunk.columns.len(),
                    font: style.font(),
                    color: style.color,
                    background_color: style.background_color,
                    underline: style.underline,
                    strikethrough: style.strikethrough,
                };
                if let Some(previous) = runs.last_mut()
                    && previous.columns.end == chunk.columns.start
                    && previous.style.color == style.color
                    && previous.style.background_color == style.background_color
                    && previous.style.underline == style.underline
                    && previous.style.strikethrough == style.strikethrough
                {
                    previous.columns.end = chunk.columns.end;
                    previous.style.len += style.len;
                } else {
                    runs.push(PreparedPaintRun {
                        columns: chunk.columns.clone(),
                        style,
                    });
                }
            }
            runs
        });
        for (columns, width) in &self.oversized_cursor_widths {
            if width.get().is_some() {
                continue;
            }
            let Some(text) = self.cursor_text(columns.clone()) else {
                continue;
            };
            width.get_or_init(|| {
                if text.bytes().all(|byte| byte.is_ascii_whitespace()) {
                    return Pixels::ZERO;
                }
                let font = CursorLayout::text_font(
                    row.font_id_for_column(columns.start, columns),
                    style,
                    text_system,
                );
                let runs = [FontRun {
                    len: text.len(),
                    font_id: text_system.resolve_font(&font),
                }];
                let width = match wrapper.layout_line_for_retention(&text, &runs) {
                    RetainedLineLayout::Dense(_, _, width) => width,
                    RetainedLineLayout::Compact(layout) => layout.width,
                };
                Pixels::from(width)
            });
        }
    }

    pub(crate) fn with_drop_executor(mut self, executor: BackgroundExecutor) -> Self {
        self.drop_executor = Some(executor);
        self
    }

    pub(crate) fn chunks(&self, columns: Range<usize>) -> impl Iterator<Item = LineChunk<'_>> {
        let start = columns.start;
        let end = columns.end;
        let first = self
            .chunks
            .partition_point(|chunk| chunk.columns.end <= start);
        self.chunks[first..]
            .iter()
            .take_while(move |chunk| start < end && chunk.columns.start < end)
            .map(move |chunk| {
                let start = chunk.columns.start.max(start);
                let end = chunk.columns.end.min(end);
                if let Some(replacement) = &chunk.replacement {
                    LineChunk::Replacement {
                        len: end - start,
                        replacement: replacement.clone(),
                        style: chunk.style,
                        diagnostic_underline_severity: chunk.diagnostic_underline_severity,
                    }
                } else {
                    let offset = chunk.text_offset;
                    LineChunk::Text(HighlightedChunk {
                        text: &self.text[offset + start - chunk.columns.start
                            ..offset + end - chunk.columns.start],
                        style: chunk.style,
                        diagnostic_underline_severity: chunk.diagnostic_underline_severity,
                        is_tab: chunk.is_tab,
                        is_inlay: chunk.is_inlay,
                        replacement: None,
                    })
                }
            })
    }

    pub(super) fn styles(
        &self,
        columns: Range<usize>,
        layout: &LineLayout,
    ) -> Option<Vec<TextRun>> {
        let runs = self.paint_runs.get()?;
        let mut output = Vec::new();
        let mut end = columns.start;
        for glyph in layout.runs.iter().flat_map(|run| &run.glyphs) {
            let column = columns.start + glyph.index;
            if column < end {
                continue;
            }
            #[cfg(test)]
            self.paint_style_lookups.fetch_add(1, Ordering::Relaxed);
            let index = runs.partition_point(|run| run.columns.end <= column);
            let Some(run) = runs.get(index) else {
                break;
            };
            let start = run.columns.start.max(columns.start);
            if start > end {
                output.push(TextRun {
                    len: start - end,
                    ..TextRun::default()
                });
            }
            end = run.columns.end.min(columns.end);
            output.push(TextRun {
                len: end - start,
                ..run.style.clone()
            });
        }
        if end < columns.end {
            output.push(TextRun {
                len: columns.end - end,
                ..TextRun::default()
            });
        }
        Some(output)
    }

    pub(super) fn diagnostics(
        &self,
        columns: Range<usize>,
    ) -> impl Iterator<Item = (Range<usize>, lsp::DiagnosticSeverity)> + '_ {
        let start = columns.start;
        let end = columns.end;
        let first = self
            .diagnostics
            .partition_point(|(range, _)| range.end <= start);
        self.diagnostics[first..]
            .iter()
            .take_while(move |(range, _)| range.start < end)
            .map(move |(range, severity)| (range.start.max(start)..range.end.min(end), *severity))
    }

    #[cfg(test)]
    pub(crate) fn prepare_text_slice(&self, columns: Range<usize>) -> SharedString {
        SharedString::new(self.text_slice(columns).expect("prepared text span"))
    }

    pub(super) fn oversized_cursor_width(&self, column: u32) -> Option<Option<Pixels>> {
        let index = self
            .oversized_cursor_widths
            .partition_point(|(columns, _)| columns.end <= column);
        let (columns, width) = self.oversized_cursor_widths.get(index)?;
        columns.contains(&column).then(|| width.get().copied())
    }

    pub(super) fn cursor_text(&self, columns: Range<u32>) -> Option<SharedString> {
        let text = self.text_slice(columns.start as usize..columns.end as usize)?;
        #[cfg(test)]
        self.cursor_text_input_bytes
            .fetch_add(text.len(), Ordering::Relaxed);
        let grapheme = text.graphemes(true).next().unwrap_or_default();
        if let Some(invisible) = grapheme
            .chars()
            .next()
            .filter(|character| crate::display_map::is_invisible(*character))
            && let Some(replacement) = crate::display_map::replacement(invisible)
        {
            Some(SharedString::from(replacement))
        } else if grapheme == "\n" {
            Some(SharedString::new_static(" "))
        } else {
            Some(SharedString::new(grapheme))
        }
    }

    #[cfg(test)]
    pub(super) fn stored_text_len(&self) -> usize {
        self.text.len()
    }

    #[cfg(test)]
    pub(super) fn cursor_text_input_bytes(&self) -> usize {
        self.cursor_text_input_bytes.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(super) fn paint_style_lookups(&self) -> usize {
        self.paint_style_lookups.load(Ordering::Relaxed)
    }

    pub(super) fn invisibles(&self, columns: Range<usize>) -> impl Iterator<Item = Invisible> + '_ {
        let first = self
            .invisibles
            .partition_point(|invisible| invisible.range().end <= columns.start);
        self.invisibles[first..]
            .iter()
            .copied()
            .take_while(move |invisible| {
                columns.start < columns.end && invisible.range().start < columns.end
            })
    }

    fn text_slice(&self, columns: Range<usize>) -> Option<&str> {
        let index = self
            .text_spans
            .partition_point(|(span, _)| span.end <= columns.start);
        let (span, offset) = self.text_spans.get(index)?;
        if !span.contains(&columns.start) {
            return None;
        }
        let start = offset + columns.start - span.start;
        let end = self
            .text
            .floor_char_boundary(offset + columns.end.min(span.end) - span.start);
        self.text.get(start..end)
    }
}

impl Drop for PreparedRowText {
    fn drop(&mut self) {
        if let Some(executor) = &self.drop_executor
            && executor.is_main_thread()
        {
            let retained = (
                std::mem::take(&mut self.text),
                std::mem::take(&mut self.text_spans),
                std::mem::take(&mut self.chunks),
                self.paint_runs.take(),
                std::mem::take(&mut self.diagnostics),
                std::mem::take(&mut self.invisibles),
                std::mem::take(&mut self.oversized_cursor_widths),
            );
            executor
                .spawn(async move {
                    drop(retained);
                })
                .detach();
        }
    }
}

impl PreparedRendererConstraint {
    pub(crate) fn new(
        renderer: ChunkRenderer,
        text: RendererTextSource,
        font: Font,
        font_size: Pixels,
    ) -> Self {
        Self {
            renderer: renderer.id,
            text,
            font,
            font_size,
            width: None,
        }
    }

    pub(crate) fn text(&self) -> &RendererTextSource {
        &self.text
    }

    pub(crate) fn width(&self) -> Option<Pixels> {
        self.width
    }

    pub(crate) fn matches_font(&self, font: &Font, font_size: Pixels) -> bool {
        self.font == *font && self.font_size == font_size
    }

    pub(crate) fn matches(
        &self,
        renderer: &ChunkRenderer,
        text: impl Into<RendererTextSource>,
        font: &Font,
        font_size: Pixels,
    ) -> bool {
        self.renderer == renderer.id
            && self.text.same_source(&text.into())
            && self.matches_font(font, font_size)
    }

    pub(crate) fn measure(&self, text_system: &Arc<TextSystem>) -> Self {
        let text = self.text.materialize();
        let wrapper = text_system.line_wrapper(self.font.clone(), self.font_size);
        let runs = [FontRun {
            len: text.len(),
            font_id: wrapper.font_id(),
        }];
        let width = match wrapper.layout_line_for_retention(&text, &runs) {
            RetainedLineLayout::Dense(_, _, width) => width,
            RetainedLineLayout::Compact(layout) => layout.width,
        };
        Self {
            width: Some(Pixels::from(width)),
            ..self.clone()
        }
    }

    pub(crate) fn layout(
        renderer: &ChunkRenderer,
        constraint: Option<&Self>,
        text: impl Into<RendererTextSource>,
        text_style: &TextStyle,
        text_width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<(AnyElement, Size<Pixels>)> {
        let available_width = if renderer.constrain_width {
            let constraint = constraint.filter(|constraint| {
                constraint.matches(
                    renderer,
                    text,
                    &text_style.font(),
                    text_style.font_size.to_pixels(window.rem_size()),
                )
            })?;
            AvailableSpace::Definite(constraint.width()?)
        } else {
            AvailableSpace::MinContent
        };
        Some(layout_chunk_renderer_with_width(
            renderer,
            available_width,
            text_style,
            text_width,
            window,
            cx,
        ))
    }
}

#[cfg(test)]
impl LineChunk<'_> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Text(chunk) => chunk.text.len(),
            Self::Replacement { len, .. } => *len,
        }
    }
}

impl<'a> From<HighlightedChunk<'a>> for LineChunk<'a> {
    fn from(chunk: HighlightedChunk<'a>) -> Self {
        match chunk {
            HighlightedChunk {
                text,
                replacement: Some(replacement),
                style,
                diagnostic_underline_severity,
                ..
            } => Self::Replacement {
                len: text.len(),
                replacement,
                style,
                diagnostic_underline_severity,
            },
            chunk => Self::Text(chunk),
        }
    }
}

struct PreparedChunk {
    columns: Range<usize>,
    text_offset: usize,
    style: Option<HighlightStyle>,
    diagnostic_underline_severity: Option<lsp::DiagnosticSeverity>,
    is_tab: bool,
    is_inlay: bool,
    replacement: Option<ChunkReplacement>,
}
