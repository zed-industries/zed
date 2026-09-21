use std::{hash::Hasher, ops::Range, sync::Arc};

use collections::{FxHasher, HashMap};
use gpui::{Font, LineLayout, Pixels, SharedString, WindowTextSystem};
use language::LanguageAwareStyling;
use parking_lot::Mutex;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    EditorStyle,
    display_map::{
        ChunkRendererId, ChunkReplacement, DisplayPoint, DisplayRow, DisplaySnapshot,
        HighlightedChunk,
    },
    scroll::ScrollPixelOffset,
};

const CHUNK_LEN: u32 = 2_048;
const REUSE_MARGIN: u32 = 64;

#[derive(Clone)]
pub struct RulerShaper {
    pub text_system: Arc<WindowTextSystem>,
    pub style: EditorStyle,
    pub font_size: Pixels,
}

impl RulerShaper {
    pub fn layout_columns(
        &self,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        columns: Range<u32>,
    ) -> Arc<LineLayout> {
        let text = row_chunks(snapshot, row, columns, &self.style)
            .map(|chunk| chunk.text)
            .collect::<String>();
        self.layout(&text)
    }

    fn layout(&self, text: &str) -> Arc<LineLayout> {
        let run = self.style.text.to_run(text.len());
        self.text_system
            .layout_line(text, self.font_size, &[run], None)
    }

    fn matches(&self, ruler: &RowRuler) -> bool {
        ruler.font_size == self.font_size && ruler.font == self.style.text.font()
    }
}

pub struct RowRulerCache {
    version: usize,
    masked: bool,
    rulers: Mutex<HashMap<u32, Arc<RowRuler>>>,
    previous: Mutex<HashMap<u32, Arc<RowRuler>>>,
    renderer_widths: Mutex<HashMap<ChunkRendererId, Pixels>>,
}

impl RowRulerCache {
    pub fn new(version: usize, masked: bool, previous: Option<&RowRulerCache>) -> Self {
        let (previous_rulers, renderer_widths) = previous
            .map(|previous| {
                let rulers = previous.rulers.lock();
                let rulers = if rulers.is_empty() {
                    previous.previous.lock().clone()
                } else {
                    rulers.clone()
                };
                (rulers, previous.renderer_widths.lock().clone())
            })
            .unwrap_or_default();
        Self {
            version,
            masked,
            rulers: Mutex::new(HashMap::default()),
            previous: Mutex::new(previous_rulers),
            renderer_widths: Mutex::new(renderer_widths),
        }
    }

    pub fn matches(&self, version: usize, masked: bool) -> bool {
        self.version == version && self.masked == masked
    }

    pub fn update_renderer_widths(
        &self,
        widths: impl IntoIterator<Item = (ChunkRendererId, Pixels)>,
    ) -> bool {
        let mut renderer_widths = self.renderer_widths.lock();
        let mut changed = false;
        for (id, width) in widths {
            if renderer_widths.insert(id, width) != Some(width) {
                changed = true;
            }
        }
        if changed {
            let mut rulers = self.rulers.lock();
            if !rulers.is_empty() {
                *self.previous.lock() = std::mem::take(&mut *rulers);
            }
        }
        changed
    }

    pub fn get_or_build(
        &self,
        wrap_row: u32,
        shaper: &RulerShaper,
        build: impl FnOnce(Option<&RowRuler>, &HashMap<ChunkRendererId, Pixels>) -> RowRuler,
    ) -> Arc<RowRuler> {
        if let Some(ruler) = self.rulers.lock().get(&wrap_row)
            && shaper.matches(ruler)
        {
            return ruler.clone();
        }
        let previous = self
            .previous
            .lock()
            .get(&wrap_row)
            .filter(|previous| shaper.matches(previous))
            .cloned();
        let ruler = Arc::new(build(previous.as_deref(), &self.renderer_widths.lock()));
        self.rulers.lock().insert(wrap_row, ruler.clone());
        ruler
    }
}

#[derive(Clone, Debug)]
struct RulerChunk {
    len: u32,
    width: Pixels,
    hash: u64,
    fixed: bool,
}

#[derive(Debug)]
pub struct RowRuler {
    font: Font,
    font_size: Pixels,
    chunks: Vec<RulerChunk>,
    starts: Vec<u32>,
    xs: Vec<ScrollPixelOffset>,
}

impl RowRuler {
    pub fn new(
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
        previous: Option<&RowRuler>,
        renderer_widths: &HashMap<ChunkRendererId, Pixels>,
    ) -> Self {
        let row_len = snapshot.line_len(row);
        let mut text = String::with_capacity(row_len as usize);
        let mut fixed_spans = Vec::<(Range<u32>, Pixels)>::new();
        let mut replacement_widths = HashMap::<SharedString, Pixels>::default();
        for chunk in row_chunks(snapshot, row, 0..row_len, &shaper.style) {
            let start = text.len() as u32;
            text.push_str(chunk.text);
            let Some(replacement) = chunk.replacement else {
                continue;
            };
            let width = match replacement {
                ChunkReplacement::Str(replacement) => *replacement_widths
                    .entry(replacement.clone())
                    .or_insert_with(|| shaper.layout(&replacement).width),
                ChunkReplacement::Renderer(renderer) => renderer
                    .measured_width
                    .or_else(|| renderer_widths.get(&renderer.id).copied())
                    .unwrap_or_else(|| shaper.layout(chunk.text).width),
            };
            fixed_spans.push((start..text.len() as u32, width));
        }

        let boundaries = GraphemeBoundaries::new(&text);
        let (prefix, suffix) = previous.map_or((Vec::new(), Vec::new()), |previous| {
            previous.reusable_chunks(&text, &fixed_spans, &boundaries)
        });
        let middle_start = prefix.iter().map(|chunk| chunk.len).sum::<u32>();
        let middle_end = text.len() as u32 - suffix.iter().map(|chunk| chunk.len).sum::<u32>();
        let mut chunks = prefix;
        chunks.extend(chunk_row_text(
            &text,
            middle_start..middle_end,
            &fixed_spans,
            &boundaries,
            shaper,
        ));
        chunks.extend(suffix);

        let mut starts = Vec::with_capacity(chunks.len() + 1);
        let mut xs = Vec::with_capacity(chunks.len() + 1);
        starts.push(0);
        xs.push(0.);
        for chunk in &chunks {
            starts.push(starts.last().copied().unwrap_or(0) + chunk.len);
            xs.push(xs.last().copied().unwrap_or(0.) + ScrollPixelOffset::from(chunk.width));
        }
        Self {
            font: shaper.style.text.font(),
            font_size: shaper.font_size,
            chunks,
            starts,
            xs,
        }
    }

    pub fn len(&self) -> u32 {
        self.starts.last().copied().unwrap_or(0)
    }

    pub fn width(&self) -> ScrollPixelOffset {
        self.xs.last().copied().unwrap_or(0.)
    }

    pub fn x_for_column(
        &self,
        column: u32,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
    ) -> ScrollPixelOffset {
        if column >= self.len() {
            return self.width();
        }
        let ix = self
            .starts
            .partition_point(|start| *start <= column)
            .saturating_sub(1);
        let start = self.starts[ix];
        let x = self.xs[ix];
        if self.chunks[ix].fixed || column == start {
            return x;
        }
        let layout = self.layout_chunk(ix, snapshot, row, shaper);
        x + ScrollPixelOffset::from(layout.x_for_index((column - start) as usize))
    }

    pub fn column_for_x(
        &self,
        x: ScrollPixelOffset,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
    ) -> u32 {
        if x <= 0. || self.chunks.is_empty() {
            return 0;
        }
        if x >= self.width() {
            return self.len();
        }
        let ix = self
            .xs
            .partition_point(|chunk_x| *chunk_x <= x)
            .saturating_sub(1)
            .min(self.chunks.len() - 1);
        let start = self.starts[ix];
        let chunk_x = self.xs[ix];
        let chunk = &self.chunks[ix];
        if chunk.fixed {
            return if x - chunk_x > ScrollPixelOffset::from(chunk.width) / 2. {
                start + chunk.len
            } else {
                start
            };
        }
        let layout = self.layout_chunk(ix, snapshot, row, shaper);
        start + layout.closest_index_for_x(Pixels::from(x - chunk_x)) as u32
    }

    pub fn columns_for_x_range(&self, x: Range<ScrollPixelOffset>) -> Range<u32> {
        if self.chunks.is_empty() {
            return 0..0;
        }
        let last = self.chunks.len();
        let first = self
            .xs
            .partition_point(|chunk_x| *chunk_x <= x.start)
            .saturating_sub(1)
            .min(last - 1);
        let end = self
            .xs
            .partition_point(|chunk_x| *chunk_x < x.end)
            .clamp(first + 1, last);
        self.starts[first]..self.starts[end]
    }

    pub fn width_of_chunks(&self, columns: Range<u32>) -> ScrollPixelOffset {
        let first = self
            .starts
            .partition_point(|start| *start <= columns.start)
            .saturating_sub(1);
        let end = self
            .starts
            .partition_point(|start| *start < columns.end)
            .clamp(first, self.chunks.len());
        self.xs[end] - self.xs[first]
    }

    pub fn chunk_ranges(&self, columns: Range<u32>) -> impl Iterator<Item = Range<u32>> + '_ {
        let first = self
            .starts
            .partition_point(|start| *start <= columns.start)
            .saturating_sub(1);
        let end = self
            .starts
            .partition_point(|start| *start < columns.end)
            .min(self.chunks.len());
        (first..end).map(|ix| self.starts[ix]..self.starts[ix + 1])
    }

    fn layout_chunk(
        &self,
        ix: usize,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
    ) -> Arc<LineLayout> {
        shaper.layout_columns(snapshot, row, self.starts[ix]..self.starts[ix + 1])
    }

    fn reusable_chunks(
        &self,
        text: &str,
        fixed_spans: &[(Range<u32>, Pixels)],
        boundaries: &GraphemeBoundaries,
    ) -> (Vec<RulerChunk>, Vec<RulerChunk>) {
        let new_len = text.len() as u32;
        let mut prefix_count = 0;
        let mut position = 0;
        for chunk in &self.chunks {
            let end = position + chunk.len;
            if end > new_len
                || chunk_hash(text, position..end, fixed_spans, boundaries) != Some(chunk.hash)
            {
                break;
            }
            position = end;
            prefix_count += 1;
        }
        if prefix_count == self.chunks.len() && position == new_len {
            return (self.chunks.clone(), Vec::new());
        }
        let mismatch_start = position;
        while prefix_count > 0 && self.starts[prefix_count] + REUSE_MARGIN > mismatch_start {
            prefix_count -= 1;
        }

        let shift = i64::from(new_len) - i64::from(self.len());
        let mut suffix_count = 0;
        let mut mismatch_end = new_len;
        for ix in (prefix_count..self.chunks.len()).rev() {
            let old_start = i64::from(self.starts[ix]) + shift;
            let old_end = i64::from(self.starts[ix + 1]) + shift;
            if old_start < i64::from(mismatch_start) {
                break;
            }
            let range = old_start as u32..old_end as u32;
            if chunk_hash(text, range.clone(), fixed_spans, boundaries)
                != Some(self.chunks[ix].hash)
            {
                break;
            }
            mismatch_end = range.start;
            suffix_count += 1;
        }
        let mut suffix_start = self.chunks.len() - suffix_count;
        while suffix_start < self.chunks.len()
            && (i64::from(self.starts[suffix_start]) + shift)
                < i64::from(mismatch_end.max(mismatch_start)) + i64::from(REUSE_MARGIN)
        {
            suffix_start += 1;
        }
        (
            self.chunks[..prefix_count].to_vec(),
            self.chunks[suffix_start..].to_vec(),
        )
    }
}

fn chunk_row_text(
    text: &str,
    range: Range<u32>,
    fixed_spans: &[(Range<u32>, Pixels)],
    boundaries: &GraphemeBoundaries,
    shaper: &RulerShaper,
) -> Vec<RulerChunk> {
    let mut chunks = Vec::new();
    let mut position = range.start;
    let mut spans = fixed_spans
        .iter()
        .skip_while(|(span, _)| span.end <= range.start)
        .peekable();
    while position < range.end {
        if let Some((span, width)) = spans.peek()
            && span.start <= position
        {
            let end = span.end.min(range.end);
            chunks.push(RulerChunk {
                len: end - position,
                width: *width,
                hash: hash_bytes(
                    &text.as_bytes()[position as usize..end as usize],
                    Some(*width),
                ),
                fixed: true,
            });
            position = end;
            spans.next();
            continue;
        }
        let stretch_end = spans
            .peek()
            .map_or(range.end, |(span, _)| span.start.min(range.end));
        let mut end = stretch_end.min(position + CHUNK_LEN);
        if end < stretch_end {
            end = boundaries.at_or_after(end).min(stretch_end);
        }
        let chunk_text = &text[position as usize..end as usize];
        chunks.push(RulerChunk {
            len: end - position,
            width: shaper.layout(chunk_text).width,
            hash: hash_bytes(chunk_text.as_bytes(), None),
            fixed: false,
        });
        position = end;
    }
    chunks
}

fn chunk_hash(
    text: &str,
    range: Range<u32>,
    fixed_spans: &[(Range<u32>, Pixels)],
    boundaries: &GraphemeBoundaries,
) -> Option<u64> {
    if !boundaries.is_boundary(range.start) || !boundaries.is_boundary(range.end) {
        return None;
    }
    let first_overlapping = fixed_spans.partition_point(|(span, _)| span.end <= range.start);
    let overlapping = fixed_spans[first_overlapping..]
        .iter()
        .take_while(|(span, _)| span.start < range.end)
        .collect::<Vec<_>>();
    let fixed_width = match overlapping.as_slice() {
        [] => None,
        [(span, width)] if *span == range => Some(*width),
        _ => return None,
    };
    Some(hash_bytes(
        &text.as_bytes()[range.start as usize..range.end as usize],
        fixed_width,
    ))
}

struct GraphemeBoundaries {
    len: u32,
    bits: Option<Vec<u64>>,
}

impl GraphemeBoundaries {
    fn new(text: &str) -> Self {
        let len = text.len() as u32;
        if text.is_ascii() {
            return Self { len, bits: None };
        }
        let mut bits = vec![0u64; text.len() / 64 + 1];
        for (offset, _) in text.grapheme_indices(true) {
            bits[offset / 64] |= 1 << (offset % 64);
        }
        bits[text.len() / 64] |= 1 << (text.len() % 64);
        Self {
            len,
            bits: Some(bits),
        }
    }

    fn is_boundary(&self, offset: u32) -> bool {
        if offset > self.len {
            return false;
        }
        match &self.bits {
            None => true,
            Some(bits) => {
                let offset = offset as usize;
                bits[offset / 64] & (1 << (offset % 64)) != 0
            }
        }
    }

    fn at_or_after(&self, offset: u32) -> u32 {
        let Some(bits) = &self.bits else {
            return offset.min(self.len);
        };
        let offset = offset.min(self.len) as usize;
        let mut word_ix = offset / 64;
        let mut word = bits[word_ix] & (u64::MAX << (offset % 64));
        while word == 0 {
            word_ix += 1;
            word = bits[word_ix];
        }
        (word_ix * 64 + word.trailing_zeros() as usize) as u32
    }
}

fn hash_bytes(bytes: &[u8], fixed_width: Option<Pixels>) -> u64 {
    let mut hasher = FxHasher::default();
    hasher.write(bytes);
    match fixed_width {
        Some(width) => {
            hasher.write_u8(1);
            hasher.write_u32(f32::from(width).to_bits());
        }
        None => hasher.write_u8(0),
    }
    hasher.finish()
}

fn row_chunks<'a>(
    snapshot: &'a DisplaySnapshot,
    row: DisplayRow,
    columns: Range<u32>,
    style: &'a EditorStyle,
) -> impl Iterator<Item = HighlightedChunk<'a>> + 'a {
    snapshot.highlighted_chunks_in_range(
        DisplayPoint::new(row, columns.start)..DisplayPoint::new(row, columns.end),
        LanguageAwareStyling {
            tree_sitter: false,
            diagnostics: false,
        },
        style,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MAX_LINE_LEN,
        display_map::{HorizontalViewport, RowLayout},
        test::editor_test_context::EditorTestContext,
    };
    use gpui::{TestAppContext, px};
    use language::Point;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_ruler_chunks_match_full_row_layout(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!(
            "{}\u{1}{}e\u{301}{}",
            "e\u{301}".repeat(CHUNK_LEN as usize / 3 + 5),
            "🙂".repeat(MAX_LINE_LEN),
            "x".repeat(3_000)
        );
        cx.set_state(&format!("ˇ{text}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper();
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        let full_row = details.shape_row_text(row_chunks(
            &snapshot,
            DisplayRow(0),
            0..text.len() as u32,
            &shaper.style,
        ));

        assert_eq!(ruler.len(), text.len() as u32);
        assert!(ruler.chunks.len() > 3);
        assert_eq!(ruler.chunks.iter().filter(|chunk| chunk.fixed).count(), 1);
        let boundaries = GraphemeBoundaries::new(&text);
        for boundary in &ruler.starts {
            assert!(
                boundaries.is_boundary(*boundary),
                "chunk boundary {boundary} splits a grapheme"
            );
        }
        let tolerance = ScrollPixelOffset::from(full_row.width) * 1e-4;
        assert!((ruler.width() - ScrollPixelOffset::from(full_row.width)).abs() < tolerance);
        for column in (0..=text.len())
            .step_by(97)
            .filter(|column| text.is_char_boundary(*column))
        {
            let x = ruler.x_for_column(column as u32, &snapshot, DisplayRow(0), &shaper);
            assert!(
                (x - ScrollPixelOffset::from(full_row.x_for_index(column))).abs() < tolerance,
                "column {column} is at {x} instead of its full-row position"
            );
            assert_eq!(
                ruler.column_for_x(x, &snapshot, DisplayRow(0), &shaper),
                column as u32
            );
        }
    }

    #[gpui::test]
    async fn test_ruler_reuses_unchanged_chunks_across_edits(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = "漢字".repeat(MAX_LINE_LEN * 4);
        cx.set_state(&format!("ˇ{text}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper();
        let before = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert!(before.chunks.len() >= 8);

        let edit_column = text.len() as u32 / 2;
        cx.update_editor(|editor, _, cx| {
            editor.edit(
                [(Point::new(0, edit_column)..Point::new(0, edit_column), "ab")],
                cx,
            );
        });
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let reused = RowRuler::new(
            &snapshot,
            DisplayRow(0),
            &shaper,
            Some(&before),
            &HashMap::default(),
        );
        let fresh = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        let edited_text = snapshot.text();
        assert_eq!(reused.len(), text.len() as u32 + 2);
        let tolerance = fresh.width() * 1e-6;
        assert!((reused.width() - fresh.width()).abs() < tolerance);
        for column in (0..=edited_text.len())
            .step_by(101)
            .filter(|column| edited_text.is_char_boundary(*column))
        {
            let reused_x = reused.x_for_column(column as u32, &snapshot, DisplayRow(0), &shaper);
            let fresh_x = fresh.x_for_column(column as u32, &snapshot, DisplayRow(0), &shaper);
            assert!(
                (reused_x - fresh_x).abs() < tolerance,
                "column {column}: {reused_x} != {fresh_x}"
            );
        }

        let edit_chunk = before.starts.partition_point(|start| *start <= edit_column) - 1;
        let kept_prefix = &before.chunks[..edit_chunk.saturating_sub(1)];
        let kept_suffix = &before.chunks[edit_chunk + 2..];
        assert!(!kept_prefix.is_empty() && !kept_suffix.is_empty());
        for (kept, rebuilt) in kept_prefix.iter().zip(&reused.chunks) {
            assert_eq!(kept.hash, rebuilt.hash);
        }
        for (kept, rebuilt) in kept_suffix.iter().rev().zip(reused.chunks.iter().rev()) {
            assert_eq!(kept.hash, rebuilt.hash);
        }
        let rebuilt_count = reused.chunks.len() - kept_prefix.len() - kept_suffix.len();
        assert!(rebuilt_count <= 4, "{rebuilt_count} chunks were reshaped");
    }

    #[gpui::test]
    async fn test_ruler_reuse_keeps_grapheme_boundaries(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let flags = "🇺🇸".repeat(2_048);
        cx.set_state(&format!("ˇ{flags}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper();
        let before = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert!(before.chunks.len() > 3);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 0)..Point::new(0, 0), "🇨")], cx);
        });
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let edited_text = snapshot.text();
        let reused = RowRuler::new(
            &snapshot,
            DisplayRow(0),
            &shaper,
            Some(&before),
            &HashMap::default(),
        );
        let fresh = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        let boundaries = GraphemeBoundaries::new(&edited_text);
        for ruler in [&reused, &fresh] {
            assert_eq!(ruler.len(), edited_text.len() as u32);
            for boundary in &ruler.starts {
                assert!(
                    boundaries.is_boundary(*boundary),
                    "chunk boundary {boundary} splits a flag"
                );
            }
        }
        assert!((reused.width() - fresh.width()).abs() < fresh.width() * 1e-6);
    }

    #[gpui::test]
    async fn test_ruler_cache_survives_frames_and_follows_edits(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = "漢字".repeat(MAX_LINE_LEN);
        cx.set_state(&format!("ˇ{text}"));
        let ruler_for = |cx: &mut EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                editor.set_visible_column_count(100.);
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                let cell = details.grid_cell();
                assert!(snapshot.is_long_unwrapped_row(DisplayRow(0)));
                assert!(!snapshot.is_windowed_row(DisplayRow(0), cell));
                let layout = snapshot.layout_row(DisplayRow(0), &details);
                let RowLayout::Windowed { geometry, .. } = &layout else {
                    panic!("ruled rows must be windowed");
                };
                let ruled = snapshot.ruled_row(DisplayRow(0), details.ruler_shaper());
                (
                    ruled.ruler.clone(),
                    geometry.width(px(0.)),
                    ruled.columns_for_viewport(
                        &HorizontalViewport {
                            scroll_columns: 500.,
                            visible_columns: 100.,
                            text_align: gpui::TextAlign::Left,
                            content_width: px(0.),
                        },
                        cell,
                    ),
                )
            })
        };

        let (first, width, columns) = ruler_for(&mut cx);
        let (second, _, _) = ruler_for(&mut cx);
        assert!(Arc::ptr_eq(&first, &second));
        assert!(columns.start < columns.end && columns.end <= text.len() as u32);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 0)..Point::new(0, 0), "漢")], cx);
        });
        let (third, edited_width, _) = ruler_for(&mut cx);
        assert!(!Arc::ptr_eq(&first, &third));
        assert_eq!(third.len(), text.len() as u32 + 3);
        assert!(edited_width > width);
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            crate::init(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }
}
