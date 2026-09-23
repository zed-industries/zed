use std::{
    hash::{Hash, Hasher},
    num::NonZeroU32,
    ops::Range,
    sync::Arc,
};

use collections::{FxHasher, HashMap};
use gpui::{
    Font, FontStyle, FontWeight, HighlightStyle, Hsla, LineLayout, Pixels, SharedString,
    StrikethroughStyle, TextRun, UnderlineStyle, WindowTextSystem,
};
use language::LanguageAwareStyling;
use multi_buffer::Anchor;
use parking_lot::Mutex;
use text::Bias;
use unicode_bidi::BidiClass;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use crate::{
    EditorStyle,
    display_map::{
        ChunkRendererId, ChunkReplacement, DisplayPoint, DisplayRow, DisplaySnapshot,
        HighlightedChunk,
    },
    scroll::ScrollPixelOffset,
};
use project::{InlayId, project_settings::DiagnosticSeverity};

const CHUNK_LEN: u32 = 2_048;
const MAX_CHUNK_LEN: u32 = 3 * CHUNK_LEN;
const FORCED_BOUNDARY_SLACK: u32 = 1_024;
const SHAPING_CONTEXT: u32 = 64;
const MAX_SHAPED_LEN: u32 = 10_240;
const BIDI_CHUNK_UNITS: u32 = MAX_SHAPED_LEN - FORCED_BOUNDARY_SLACK - 4;
const _: () = assert!(
    CHUNK_LEN + MAX_CHUNK_LEN + FORCED_BOUNDARY_SLACK + 4 + 2 * SHAPING_CONTEXT <= MAX_SHAPED_LEN
);

#[derive(Clone)]
pub struct RulerShaper {
    pub text_system: Arc<WindowTextSystem>,
    pub style: EditorStyle,
    pub font_size: Pixels,
    pub language_aware: LanguageAwareStyling,
}

impl RulerShaper {
    pub fn layout_columns(
        &self,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        columns: Range<u32>,
    ) -> Arc<LineLayout> {
        let mut text = String::new();
        let mut runs = Vec::new();
        for chunk in self.chunks(snapshot, row, columns) {
            text.push_str(chunk.text);
            runs.push(self.run(chunk.style, chunk.text.len()));
        }
        self.layout(&text, &runs)
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        snapshot: &'a DisplaySnapshot,
        row: DisplayRow,
        columns: Range<u32>,
    ) -> impl Iterator<Item = HighlightedChunk<'a>> + 'a {
        snapshot.highlighted_chunks_in_range(
            DisplayPoint::new(row, columns.start)..DisplayPoint::new(row, columns.end),
            self.language_aware,
            &self.style,
        )
    }

    fn run(&self, style: Option<HighlightStyle>, len: usize) -> TextRun {
        match style {
            Some(style) => self.style.text.clone().highlight(style).to_run(len),
            None => self.style.text.to_run(len),
        }
    }

    fn layout(&self, text: &str, runs: &[TextRun]) -> Arc<LineLayout> {
        self.text_system
            .layout_line(text, self.font_size, runs, None)
    }

    fn renderer_metrics_key(&self) -> u64 {
        renderer_metrics_key(&self.style.text.font(), self.font_size)
    }

    fn metrics_fingerprint(&self) -> u64 {
        let mut hasher = FxHasher::default();
        self.style.text.font().hash(&mut hasher);
        hasher.write_u32(f32::from(self.font_size).to_bits());
        hasher.write_usize(Arc::as_ptr(&self.style.syntax) as usize);
        hash_font_overrides(&mut hasher, Some(self.style.inlay_hints_style));
        hash_font_overrides(
            &mut hasher,
            Some(self.style.edit_prediction_styles.insertion),
        );
        hash_font_overrides(
            &mut hasher,
            Some(self.style.edit_prediction_styles.whitespace),
        );
        hasher.write_u8(self.language_aware.tree_sitter as u8);
        hasher.write_u8(self.language_aware.diagnostics as u8);
        hasher.write_u8(self.style.show_underlines as u8);
        hasher.finish()
    }

    fn matches(&self, ruler: &RowRuler) -> bool {
        ruler.metrics_fingerprint == self.metrics_fingerprint()
    }
}

pub fn renderer_metrics_key(font: &Font, font_size: Pixels) -> u64 {
    let mut hasher = FxHasher::default();
    font.hash(&mut hasher);
    hasher.write_u32(f32::from(font_size).to_bits());
    hasher.finish()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct FontOverrides {
    weight: Option<FontWeight>,
    style: Option<FontStyle>,
}

impl FontOverrides {
    fn from_style(style: Option<HighlightStyle>) -> Self {
        Self {
            weight: style.and_then(|style| style.font_weight),
            style: style.and_then(|style| style.font_style),
        }
    }
}

fn hash_font_overrides(hasher: &mut FxHasher, style: Option<HighlightStyle>) {
    FontOverrides::from_style(style).hash(hasher);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Decorations {
    color: Hsla,
    underline: Option<UnderlineStyle>,
    strikethrough: Option<StrikethroughStyle>,
}

impl Decorations {
    fn from_run(run: &TextRun) -> Self {
        Self {
            color: run.color,
            underline: run.underline,
            strikethrough: run.strikethrough,
        }
    }
}

#[derive(Clone, Debug)]
struct StyleSegment {
    end: u32,
    overrides: FontOverrides,
    decorations: Decorations,
    style: Option<HighlightStyle>,
}

pub type RulerCacheVersion = (usize, usize, usize, DiagnosticSeverity, NonZeroU32);

pub struct RowRulerCache {
    version: RulerCacheVersion,
    masked: bool,
    rulers: Mutex<HashMap<u32, Arc<RowRuler>>>,
    previous: Mutex<HashMap<u32, Arc<RowRuler>>>,
    renderer_widths: Mutex<(u64, HashMap<ChunkRendererId, Pixels>)>,
}

impl RowRulerCache {
    pub fn new(
        version: RulerCacheVersion,
        masked: bool,
        previous: Option<&RowRulerCache>,
        mut retain: impl FnMut(&RowRuler) -> Option<u32>,
    ) -> Self {
        let (previous_rulers, retained, renderer_widths) = previous
            .map(|previous| {
                let rulers = previous.rulers.lock();
                let retained = rulers
                    .values()
                    .filter_map(|ruler| Some((retain(ruler)?, ruler.clone())))
                    .collect::<HashMap<_, _>>();
                let rulers = if rulers.is_empty() {
                    previous.previous.lock().clone()
                } else {
                    rulers.clone()
                };
                (rulers, retained, previous.renderer_widths.lock().clone())
            })
            .unwrap_or_default();
        Self {
            version,
            masked,
            rulers: Mutex::new(retained),
            previous: Mutex::new(previous_rulers),
            renderer_widths: Mutex::new(renderer_widths),
        }
    }

    pub fn matches(&self, version: RulerCacheVersion, masked: bool) -> bool {
        self.version == version && self.masked == masked
    }

    pub fn version(&self) -> (RulerCacheVersion, bool) {
        (self.version, self.masked)
    }

    #[cfg(test)]
    pub fn cached_rulers(&self) -> Vec<(u32, Arc<RowRuler>)> {
        let mut rulers = self
            .rulers
            .lock()
            .iter()
            .map(|(row, ruler)| (*row, ruler.clone()))
            .collect::<Vec<_>>();
        rulers.sort_by_key(|(row, _)| *row);
        rulers
    }

    pub fn update_renderer_widths(
        &self,
        widths: impl IntoIterator<Item = (ChunkRendererId, Pixels)>,
        metrics_key: u64,
    ) -> bool {
        let mut renderer_widths = self.renderer_widths.lock();
        let mut changed = false;
        if renderer_widths.0 != metrics_key {
            changed = !renderer_widths.1.is_empty();
            *renderer_widths = (metrics_key, HashMap::default());
        }
        let renderer_widths = &mut renderer_widths.1;
        for (id, width) in widths {
            if matches!(id, ChunkRendererId::Inlay(_))
                && renderer_widths.insert(id, width) != Some(width)
            {
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

    pub fn remove_renderer_widths(&self, inlays: &[InlayId]) {
        let mut renderer_widths = self.renderer_widths.lock();
        for inlay in inlays {
            renderer_widths.1.remove(&ChunkRendererId::Inlay(*inlay));
        }
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
        let renderer_widths = self.renderer_widths.lock();
        let no_widths = HashMap::default();
        let widths = if renderer_widths.0 == shaper.renderer_metrics_key() {
            &renderer_widths.1
        } else {
            &no_widths
        };
        let ruler = Arc::new(build(previous.as_deref(), widths));
        drop(renderer_widths);
        self.rulers.lock().insert(wrap_row, ruler.clone());
        ruler
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderPiece {
    pub chunk: Range<u32>,
    pub context: Range<u32>,
    pub width: Pixels,
    pub x: ScrollPixelOffset,
}

#[derive(Clone, Debug)]
struct RulerChunk {
    len: u32,
    width: Pixels,
    hash: u64,
    fixed: bool,
    context_before: u32,
    context_after: u32,
}

#[derive(Debug)]
pub struct RowRuler {
    metrics_fingerprint: u64,
    row_range: Range<Anchor>,
    retainable: bool,
    rtl: bool,
    chunks: Vec<RulerChunk>,
    starts: Vec<u32>,
    xs: Vec<ScrollPixelOffset>,
}

#[derive(Clone, Copy)]
struct BidiParagraph {
    rtl: bool,
    first_strong: u32,
}

struct RowText {
    text: String,
    fixed_spans: Vec<(Range<u32>, Pixels)>,
    segments: Vec<StyleSegment>,
    forced_boundaries: Vec<u32>,
    bidi: Option<BidiParagraph>,
}

impl RowText {
    fn read(
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
        renderer_widths: &HashMap<ChunkRendererId, Pixels>,
    ) -> Self {
        let row_len = snapshot.line_len(row);
        let mut text = String::with_capacity(row_len as usize);
        let mut fixed_spans = Vec::<(Range<u32>, Pixels)>::new();
        let mut segments = Vec::<StyleSegment>::new();
        let mut replacement_widths = HashMap::<(SharedString, FontOverrides), Pixels>::default();
        for chunk in shaper.chunks(snapshot, row, 0..row_len) {
            let start = text.len() as u32;
            text.push_str(chunk.text);
            let end = text.len() as u32;
            let overrides = FontOverrides::from_style(chunk.style);
            match segments.last_mut() {
                Some(segment) if segment.style == chunk.style => segment.end = end,
                last => {
                    let decorations = Decorations::from_run(&shaper.run(chunk.style, 0));
                    match last {
                        Some(segment)
                            if segment.overrides == overrides
                                && segment.decorations == decorations =>
                        {
                            segment.end = end
                        }
                        _ => segments.push(StyleSegment {
                            end,
                            overrides,
                            decorations,
                            style: chunk.style,
                        }),
                    }
                }
            }
            let Some(replacement) = chunk.replacement else {
                continue;
            };
            let width = match replacement {
                ChunkReplacement::Str(replacement) => *replacement_widths
                    .entry((replacement.clone(), overrides))
                    .or_insert_with(|| {
                        let run = shaper.run(chunk.style, replacement.len());
                        shaper.layout(&replacement, &[run]).width
                    }),
                ChunkReplacement::Renderer(renderer) => renderer
                    .measured_width
                    .or_else(|| renderer_widths.get(&renderer.id).copied())
                    .unwrap_or_else(|| {
                        let run = shaper.run(chunk.style, chunk.text.len());
                        shaper.layout(chunk.text, &[run]).width
                    }),
            };
            fixed_spans.push((start..end, width));
        }
        let bidi = text
            .bytes()
            .any(|byte| matches!(byte, 0xD6..=0xDF | 0xE0 | 0xE2 | 0xEF | 0xF0))
            && text.chars().any(is_bidi_char);
        let bidi = bidi.then(|| {
            let first_strong = text
                .char_indices()
                .find_map(|(offset, char)| strong_direction(char).map(|rtl| (offset, rtl)));
            BidiParagraph {
                rtl: first_strong.is_some_and(|(_, rtl)| rtl),
                first_strong: first_strong.map_or(text.len(), |(offset, _)| offset) as u32,
            }
        });
        Self {
            text,
            fixed_spans,
            segments,
            forced_boundaries: Vec::new(),
            bidi,
        }
    }

    fn max_chunk_end(&self, start: u32) -> u32 {
        let len = self.text.len() as u32;
        if self.bidi.is_none() {
            return start.saturating_add(MAX_CHUNK_LEN).min(len);
        }
        let mut units = 0;
        for (offset, char) in self.text[start as usize..].char_indices() {
            units += char.len_utf16() as u32;
            if units > BIDI_CHUNK_UNITS {
                return start + offset as u32;
            }
        }
        len
    }

    fn target_chunk_end(&self, start: u32, limit: u32) -> u32 {
        if self.bidi.is_some() {
            self.max_chunk_end(start).min(limit)
        } else {
            start.saturating_add(CHUNK_LEN).min(limit)
        }
    }

    fn exceeds_max_chunk_len(&self) -> bool {
        self.max_chunk_end(0) < self.text.len() as u32
    }

    fn forced_boundaries(&self, boundaries: &GraphemeBoundaries, shaper: &RulerShaper) -> Vec<u32> {
        let len = self.text.len() as u32;
        let mut forced = Vec::new();
        let mut previous = 0;
        while previous < len {
            let window_end = self.max_chunk_end(previous);
            if window_end == len {
                break;
            }
            if let Some(safe) = self.last_shaping_boundary_in(previous, window_end, boundaries) {
                previous = safe;
                continue;
            }
            let slack_end = window_end.saturating_add(FORCED_BOUNDARY_SLACK).min(len);
            let boundary = self
                .forced_boundary_in(previous, window_end, slack_end, boundaries, shaper)
                .unwrap_or_else(|| {
                    let mut offset = slack_end as usize;
                    while !self.text.is_char_boundary(offset) {
                        offset += 1;
                    }
                    offset as u32
                });
            if boundary >= len {
                break;
            }
            forced.push(boundary);
            previous = boundary;
        }
        forced
    }

    fn forced_boundary_in(
        &self,
        previous: u32,
        start: u32,
        end: u32,
        boundaries: &GraphemeBoundaries,
        shaper: &RulerShaper,
    ) -> Option<u32> {
        if self.bidi.is_some() {
            return boundaries.next_matching(start, end, |_| true);
        }
        let mut context_start = start.saturating_sub(SHAPING_CONTEXT).max(previous) as usize;
        while !self.text.is_char_boundary(context_start) {
            context_start += 1;
        }
        let mut layout_end = end as usize;
        while !self.text.is_char_boundary(layout_end) {
            layout_end -= 1;
        }
        let layout = self.layout_range(context_start as u32..layout_end as u32, shaper);
        boundaries
            .next_matching(start, end, |offset| {
                offset < end && layout.is_glyph_boundary((offset - context_start as u32) as usize)
            })
            .or_else(|| boundaries.next_matching(start, end, |_| true))
    }

    fn last_shaping_boundary_in(
        &self,
        after: u32,
        end: u32,
        boundaries: &GraphemeBoundaries,
    ) -> Option<u32> {
        let bytes = self.text.as_bytes();
        let end = end.min(bytes.len() as u32);
        let fixed_edge = self.last_fixed_span_edge_in(after, end);
        let floor = fixed_edge.unwrap_or(after);
        let mut position = end;
        while position > floor {
            let offset = position as usize;
            let beside_isolated_byte = bytes[offset - 1..]
                .iter()
                .take(2)
                .any(|byte| may_be_shaping_isolated(*byte));
            if beside_isolated_byte
                && self.is_shaping_boundary_between_graphemes(position)
                && boundaries.is_boundary(position)
            {
                return Some(position);
            }
            position -= 1;
        }
        fixed_edge
    }

    fn last_fixed_span_edge_in(&self, after: u32, end: u32) -> Option<u32> {
        let first_after_end = self
            .fixed_spans
            .partition_point(|(span, _)| span.start <= end);
        self.fixed_spans[..first_after_end]
            .iter()
            .rev()
            .take_while(|(span, _)| span.end > after)
            .flat_map(|(span, _)| [span.end, span.start])
            .find(|edge| after < *edge && *edge <= end)
    }

    fn is_forced_boundary(&self, offset: u32) -> bool {
        self.forced_boundaries.binary_search(&offset).is_ok()
    }

    fn next_forced_boundary_after(&self, offset: u32) -> u32 {
        let ix = self
            .forced_boundaries
            .partition_point(|forced| *forced <= offset);
        self.forced_boundaries
            .get(ix)
            .copied()
            .unwrap_or(self.text.len() as u32)
    }

    fn last_forced_boundary_at_or_before(&self, offset: u32) -> u32 {
        let ix = self
            .forced_boundaries
            .partition_point(|forced| *forced <= offset);
        ix.checked_sub(1).map_or(0, |ix| self.forced_boundaries[ix])
    }

    fn is_shaping_boundary(&self, offset: u32, boundaries: &GraphemeBoundaries) -> bool {
        if offset == 0 || offset == self.text.len() as u32 || self.is_forced_boundary(offset) {
            return true;
        }
        boundaries.is_boundary(offset) && self.is_shaping_boundary_between_graphemes(offset)
    }

    fn is_shaping_boundary_between_graphemes(&self, offset: u32) -> bool {
        let offset = offset as usize;
        if !self.text.is_char_boundary(offset) {
            return false;
        }
        let span_edge = self
            .fixed_spans
            .partition_point(|(span, _)| span.end < offset as u32);
        if self.fixed_spans[span_edge..]
            .iter()
            .take_while(|(span, _)| span.start <= offset as u32)
            .any(|(span, _)| span.start == offset as u32 || span.end == offset as u32)
        {
            return true;
        }
        let (Some(before), Some(after)) = (
            self.text[..offset].chars().next_back(),
            self.text[offset..].chars().next(),
        ) else {
            return false;
        };
        match self.bidi {
            None => is_shaping_isolated(before) || is_shaping_isolated(after),
            Some(paragraph) => {
                if offset as u32 <= paragraph.first_strong || before.is_whitespace() {
                    return false;
                }
                if after.is_whitespace() {
                    self.text[offset..]
                        .chars()
                        .find(|char| !char.is_whitespace())
                        .is_some_and(|char| strong_direction(char) == Some(paragraph.rtl))
                } else {
                    (is_shaping_isolated(before) || is_shaping_isolated(after))
                        && strong_direction(before) == Some(paragraph.rtl)
                        && strong_direction(after) == Some(paragraph.rtl)
                }
            }
        }
    }

    fn context_of(&self, range: Range<u32>) -> Range<u32> {
        if self.bidi.is_some() {
            return range;
        }
        let stretch_start = self
            .fixed_spans
            .partition_point(|(span, _)| span.end <= range.start);
        let stretch = if stretch_start == 0 {
            0
        } else {
            self.fixed_spans[stretch_start - 1].0.end
        }
            ..self
                .fixed_spans
                .get(stretch_start)
                .map_or(self.text.len() as u32, |(span, _)| span.start);
        let mut start = range
            .start
            .saturating_sub(SHAPING_CONTEXT)
            .max(stretch.start)
            .max(self.last_forced_boundary_at_or_before(range.start))
            as usize;
        while !self.text.is_char_boundary(start) {
            start += 1;
        }
        let mut end = range
            .end
            .saturating_add(SHAPING_CONTEXT)
            .min(stretch.end)
            .min(self.next_forced_boundary_after(range.end.saturating_sub(1)))
            as usize;
        while !self.text.is_char_boundary(end) {
            end += 1;
        }
        start as u32..end as u32
    }

    fn segments_in(&self, range: Range<u32>) -> impl Iterator<Item = (Range<u32>, &StyleSegment)> {
        let first = self
            .segments
            .partition_point(|segment| segment.end <= range.start);
        let mut segments = self.segments[first..].iter();
        let mut start = if first == 0 {
            0
        } else {
            self.segments[first - 1].end
        };
        std::iter::from_fn(move || {
            if start >= range.end {
                return None;
            }
            let segment = segments.next()?;
            let segment_range = start.max(range.start)..segment.end.min(range.end);
            start = segment.end;
            Some((segment_range, segment))
        })
    }

    fn hash_chunk(&self, range: Range<u32>, fixed_width: Option<Pixels>) -> u64 {
        let mut hasher = FxHasher::default();
        let context = match fixed_width {
            Some(width) => {
                hasher.write_u8(1);
                hasher.write_u32(f32::from(width).to_bits());
                range
            }
            None => {
                let context = self.context_of(range.clone());
                hasher.write_u8(0);
                hasher.write_u32(range.start - context.start);
                hasher.write_u32(context.end - range.end);
                context
            }
        };
        hasher.write(&self.text.as_bytes()[context.start as usize..context.end as usize]);
        for (segment_range, segment) in self.segments_in(context) {
            hasher.write_u32(segment_range.end - segment_range.start);
            segment.overrides.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn layout_range(&self, range: Range<u32>, shaper: &RulerShaper) -> Arc<LineLayout> {
        let runs = self
            .segments_in(range.clone())
            .map(|(segment_range, segment)| {
                shaper.run(
                    segment.style,
                    (segment_range.end - segment_range.start) as usize,
                )
            })
            .collect::<Vec<_>>();
        shaper.layout(&self.text[range.start as usize..range.end as usize], &runs)
    }

    fn layout_chunk(&self, range: Range<u32>, shaper: &RulerShaper) -> Pixels {
        let context = self.context_of(range.clone());
        let layout = self.layout_range(context.clone(), shaper);
        let start_x = if context.start < range.start {
            layout.x_for_index((range.start - context.start) as usize)
        } else {
            Pixels::ZERO
        };
        let end_x = if range.end < context.end {
            layout.x_for_index((range.end - context.start) as usize)
        } else {
            layout.width
        };
        end_x - start_x
    }
}

fn may_be_shaping_isolated(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte >= 0x80
}

fn is_shaping_isolated(char: char) -> bool {
    char.is_whitespace()
        || matches!(
            u32::from(char),
            0x3000..=0x30FF
                | 0x3400..=0x4DBF
                | 0x4E00..=0x9FFF
                | 0xAC00..=0xD7AF
                | 0xF900..=0xFAFF
                | 0xFF00..=0xFFEF
                | 0x20000..=0x3134F
        )
}

fn strong_direction(char: char) -> Option<bool> {
    match unicode_bidi::bidi_class(char) {
        BidiClass::L => Some(false),
        BidiClass::R | BidiClass::AL => Some(true),
        _ => None,
    }
}

fn is_bidi_char(char: char) -> bool {
    matches!(
        u32::from(char),
        0x0590..=0x08FF
            | 0x200F
            | 0x202B
            | 0x202E
            | 0x2067
            | 0xFB1D..=0xFDFF
            | 0xFE70..=0xFEFF
            | 0x10800..=0x10FFF
            | 0x1E800..=0x1EFFF
    )
}

impl RowRuler {
    pub fn new(
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
        previous: Option<&RowRuler>,
        renderer_widths: &HashMap<ChunkRendererId, Pixels>,
    ) -> Self {
        let mut row_text = RowText::read(snapshot, row, shaper, renderer_widths);
        if row_text.exceeds_max_chunk_len() {
            let forced =
                row_text.forced_boundaries(&GraphemeBoundaries::new(&row_text.text), shaper);
            row_text.forced_boundaries = forced;
        }
        let boundaries = GraphemeBoundaries::new(&row_text.text);
        let (prefix, suffix) = previous.map_or((Vec::new(), Vec::new()), |previous| {
            previous.reusable_chunks(&row_text, &boundaries)
        });
        let middle_start = prefix.iter().map(|chunk| chunk.len).sum::<u32>();
        let middle_end =
            row_text.text.len() as u32 - suffix.iter().map(|chunk| chunk.len).sum::<u32>();
        let mut chunks = prefix;
        chunks.extend(chunk_row_text(
            &row_text,
            middle_start..middle_end,
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
        let buffer = snapshot.buffer_snapshot();
        let row_start_point =
            snapshot.display_point_to_point(DisplayPoint::new(row, 0), Bias::Left);
        let row_end_point = snapshot
            .display_point_to_point(DisplayPoint::new(row, snapshot.line_len(row)), Bias::Right);
        let row_start = buffer.point_to_offset(row_start_point);
        let row_end = buffer.point_to_offset(row_end_point);
        let retainable = row_start < row_end;
        Self {
            metrics_fingerprint: shaper.metrics_fingerprint(),
            row_range: buffer.anchor_after(row_start)..buffer.anchor_before(row_end),
            retainable,
            rtl: row_text.bidi.is_some_and(|paragraph| paragraph.rtl),
            chunks,
            starts,
            xs,
        }
    }

    pub fn row_range(&self) -> Option<&Range<Anchor>> {
        self.retainable.then_some(&self.row_range)
    }

    pub fn len(&self) -> u32 {
        self.starts.last().copied().unwrap_or(0)
    }

    pub fn width(&self) -> ScrollPixelOffset {
        self.xs.last().copied().unwrap_or(0.)
    }

    pub fn is_rtl(&self) -> bool {
        self.rtl
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
        let x = self.visual_x(ix);
        if self.chunks[ix].fixed || (column == start && !self.rtl) {
            return x;
        }
        let context = self.chunk_context(ix);
        let layout = shaper.layout_columns(snapshot, row, context.clone());
        let origin = chunk_origin(&layout, &context, start);
        x + ScrollPixelOffset::from(layout.x_for_index((column - context.start) as usize) - origin)
    }

    pub fn column_for_x(
        &self,
        x: ScrollPixelOffset,
        snapshot: &DisplaySnapshot,
        row: DisplayRow,
        shaper: &RulerShaper,
    ) -> u32 {
        if self.chunks.is_empty() {
            return 0;
        }
        if !self.rtl {
            if x <= 0. {
                return 0;
            }
            if x >= self.width() {
                return self.len();
            }
        }
        let x = x.clamp(0., self.width());
        let ix = self.chunk_ix_for_x(x);
        let start = self.starts[ix];
        let chunk_x = self.visual_x(ix);
        let chunk = &self.chunks[ix];
        if chunk.fixed {
            return if x - chunk_x > ScrollPixelOffset::from(chunk.width) / 2. {
                start + chunk.len
            } else {
                start
            };
        }
        let context = self.chunk_context(ix);
        let layout = shaper.layout_columns(snapshot, row, context.clone());
        let origin = chunk_origin(&layout, &context, start);
        let column =
            context.start + layout.closest_index_for_x(Pixels::from(x - chunk_x) + origin) as u32;
        column.clamp(start, start + chunk.len)
    }

    pub fn columns_for_x_range(&self, x: Range<ScrollPixelOffset>) -> Range<u32> {
        if self.chunks.is_empty() {
            return 0..0;
        }
        let x = if self.rtl {
            self.width() - x.end..self.width() - x.start
        } else {
            x
        };
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

    pub fn x_range_for_columns(&self, columns: Range<u32>) -> Range<ScrollPixelOffset> {
        let (first, end) = self.chunk_ixs_for_columns(columns);
        if self.rtl {
            self.width() - self.xs[end]..self.width() - self.xs[first]
        } else {
            self.xs[first]..self.xs[end]
        }
    }

    pub fn render_pieces(&self, columns: Range<u32>) -> impl Iterator<Item = RenderPiece> + '_ {
        let (first, end) = self.chunk_ixs_for_columns(columns);
        (first..end).map(|ix| RenderPiece {
            chunk: self.starts[ix]..self.starts[ix + 1],
            context: self.chunk_context(ix),
            width: self.chunks[ix].width,
            x: self.visual_x(ix),
        })
    }

    fn chunk_ixs_for_columns(&self, columns: Range<u32>) -> (usize, usize) {
        let first = self
            .starts
            .partition_point(|start| *start <= columns.start)
            .saturating_sub(1)
            .min(self.chunks.len());
        let end = self
            .starts
            .partition_point(|start| *start < columns.end)
            .clamp(first, self.chunks.len());
        (first, end)
    }

    fn visual_x(&self, ix: usize) -> ScrollPixelOffset {
        if self.rtl {
            self.width() - self.xs[ix + 1]
        } else {
            self.xs[ix]
        }
    }

    fn chunk_ix_for_x(&self, x: ScrollPixelOffset) -> usize {
        let last = self.chunks.len() - 1;
        if self.rtl {
            let mirrored = self.width() - x;
            self.xs
                .partition_point(|chunk_x| *chunk_x < mirrored)
                .saturating_sub(1)
                .min(last)
        } else {
            self.xs
                .partition_point(|chunk_x| *chunk_x <= x)
                .saturating_sub(1)
                .min(last)
        }
    }

    fn chunk_context(&self, ix: usize) -> Range<u32> {
        let chunk = &self.chunks[ix];
        self.starts[ix] - chunk.context_before..self.starts[ix + 1] + chunk.context_after
    }

    fn reusable_chunks(
        &self,
        row_text: &RowText,
        boundaries: &GraphemeBoundaries,
    ) -> (Vec<RulerChunk>, Vec<RulerChunk>) {
        let new_len = row_text.text.len() as u32;
        let mut prefix_count = 0;
        let mut position = 0;
        for chunk in &self.chunks {
            let end = position + chunk.len;
            if end > new_len || chunk_hash(row_text, position..end, boundaries) != Some(chunk.hash)
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

        let shift = i64::from(new_len) - i64::from(self.len());
        let mut suffix_count = 0;
        for ix in (prefix_count..self.chunks.len()).rev() {
            let old_start = i64::from(self.starts[ix]) + shift;
            let old_end = i64::from(self.starts[ix + 1]) + shift;
            if old_start < i64::from(mismatch_start) {
                break;
            }
            let range = old_start as u32..old_end as u32;
            if chunk_hash(row_text, range, boundaries) != Some(self.chunks[ix].hash) {
                break;
            }
            suffix_count += 1;
        }
        (
            self.chunks[..prefix_count].to_vec(),
            self.chunks[self.chunks.len() - suffix_count..].to_vec(),
        )
    }
}

fn chunk_origin(layout: &LineLayout, context: &Range<u32>, start: u32) -> Pixels {
    if context.start < start {
        layout.x_for_index((start - context.start) as usize)
    } else {
        Pixels::ZERO
    }
}

fn chunk_row_text(
    row_text: &RowText,
    range: Range<u32>,
    boundaries: &GraphemeBoundaries,
    shaper: &RulerShaper,
) -> Vec<RulerChunk> {
    let mut chunks = Vec::new();
    let mut position = range.start;
    let mut spans = row_text
        .fixed_spans
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
                hash: row_text.hash_chunk(position..end, Some(*width)),
                fixed: true,
                context_before: 0,
                context_after: 0,
            });
            position = end;
            spans.next();
            continue;
        }
        let stretch_end = spans
            .peek()
            .map_or(range.end, |(span, _)| span.start.min(range.end));
        let limit = stretch_end.min(row_text.next_forced_boundary_after(position));
        let target = row_text.target_chunk_end(position, limit);
        let end = if target >= limit {
            limit
        } else if row_text.bidi.is_some() {
            row_text
                .last_shaping_boundary_in(position, target, boundaries)
                .unwrap_or(limit)
        } else {
            boundaries
                .next_matching(target, limit, |offset| {
                    offset == limit || row_text.is_shaping_boundary_between_graphemes(offset)
                })
                .unwrap_or(limit)
        };
        let context = row_text.context_of(position..end);
        chunks.push(RulerChunk {
            len: end - position,
            width: row_text.layout_chunk(position..end, shaper),
            hash: row_text.hash_chunk(position..end, None),
            fixed: false,
            context_before: position - context.start,
            context_after: context.end - end,
        });
        position = end;
    }
    chunks
}

fn chunk_hash(
    row_text: &RowText,
    range: Range<u32>,
    boundaries: &GraphemeBoundaries,
) -> Option<u64> {
    if !row_text.is_shaping_boundary(range.start, boundaries)
        || !row_text.is_shaping_boundary(range.end, boundaries)
    {
        return None;
    }
    let fixed_spans = &row_text.fixed_spans;
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
    Some(row_text.hash_chunk(range, fixed_width))
}

const REGIONAL_INDICATOR_PREFIX: &[u8] = b"\xF0\x9F\x87";

enum GraphemeBoundaries<'a> {
    Ascii { len: u32 },
    Segmented { text: &'a str },
    Indexed { len: u32, bits: Vec<u64> },
}

impl<'a> GraphemeBoundaries<'a> {
    fn new(text: &'a str) -> Self {
        let len = text.len() as u32;
        if text.is_ascii() {
            return Self::Ascii { len };
        }
        let bytes = text.as_bytes();
        let has_regional_indicators = bytes.contains(&REGIONAL_INDICATOR_PREFIX[0])
            && bytes
                .windows(REGIONAL_INDICATOR_PREFIX.len())
                .any(|window| window == REGIONAL_INDICATOR_PREFIX);
        if !has_regional_indicators {
            return Self::Segmented { text };
        }
        let mut bits = vec![0u64; text.len() / 64 + 1];
        for (offset, _) in text.grapheme_indices(true) {
            bits[offset / 64] |= 1 << (offset % 64);
        }
        bits[text.len() / 64] |= 1 << (text.len() % 64);
        Self::Indexed { len, bits }
    }

    fn len(&self) -> u32 {
        match self {
            Self::Ascii { len } | Self::Indexed { len, .. } => *len,
            Self::Segmented { text } => text.len() as u32,
        }
    }

    fn is_boundary(&self, offset: u32) -> bool {
        if offset > self.len() {
            return false;
        }
        match self {
            Self::Ascii { .. } => true,
            Self::Segmented { text } => {
                let offset = offset as usize;
                text.is_char_boundary(offset)
                    && GraphemeCursor::new(offset, text.len(), true).is_boundary(text, 0)
                        == Ok(true)
            }
            Self::Indexed { bits, .. } => {
                let offset = offset as usize;
                bits[offset / 64] & (1 << (offset % 64)) != 0
            }
        }
    }

    #[cfg(test)]
    fn at_or_after(&self, offset: u32, limit: u32) -> Option<u32> {
        self.next_matching(offset, limit, |_| true)
    }

    fn next_matching(
        &self,
        offset: u32,
        limit: u32,
        mut accept: impl FnMut(u32) -> bool,
    ) -> Option<u32> {
        let limit = limit.min(self.len());
        let offset = offset.min(limit);
        match self {
            Self::Ascii { .. } => (offset..=limit).find(|offset| accept(*offset)),
            Self::Segmented { text } => {
                let mut boundary = offset as usize;
                while !text.is_char_boundary(boundary) {
                    boundary += 1;
                }
                let mut limit = limit as usize;
                while !text.is_char_boundary(limit) {
                    limit -= 1;
                }
                if boundary > limit {
                    return None;
                }
                let searchable = &text[..limit];
                let mut cursor = GraphemeCursor::new(boundary, text.len(), true);
                if boundary == limit || cursor.is_boundary(searchable, 0) == Ok(true) {
                    if accept(boundary as u32) {
                        return Some(boundary as u32);
                    }
                    if boundary == limit {
                        return None;
                    }
                }
                loop {
                    match cursor.next_boundary(searchable, 0) {
                        Ok(Some(boundary)) if accept(boundary as u32) => {
                            return Some(boundary as u32);
                        }
                        Ok(Some(_)) => {}
                        Ok(None) => return accept(limit as u32).then_some(limit as u32),
                        Err(_) => return None,
                    }
                }
            }
            Self::Indexed { bits, .. } => {
                let mut offset = offset as usize;
                let limit = limit as usize;
                loop {
                    let mut word_ix = offset / 64;
                    let mut word = bits[word_ix] & (u64::MAX << (offset % 64));
                    while word == 0 && (word_ix + 1) * 64 <= limit {
                        word_ix += 1;
                        word = bits[word_ix];
                    }
                    if word == 0 {
                        return None;
                    }
                    let boundary = word_ix * 64 + word.trailing_zeros() as usize;
                    if boundary > limit {
                        return None;
                    }
                    if accept(boundary as u32) {
                        return Some(boundary as u32);
                    }
                    offset = boundary + 1;
                }
            }
        }
    }
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
    use multi_buffer::MultiBufferOffset;
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
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        let full_row =
            details.shape_row_text(shaper.chunks(&snapshot, DisplayRow(0), 0..text.len() as u32));

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
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
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
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
        let before = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(before.chunks.len(), 3);

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
    async fn test_ruler_measures_and_tracks_font_changing_highlights(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = "漢字".repeat(MAX_LINE_LEN * 2);
        cx.set_state(&format!("ˇ{text}"));
        let ruler_for = |cx: &mut EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                snapshot
                    .ruled_row(
                        DisplayRow(0),
                        details.ruler_shaper(&snapshot, DisplayRow(0)),
                    )
                    .ruler
            })
        };
        let plain = ruler_for(&mut cx);

        let bold_range = 3 * 3_000..3 * 3_010;
        cx.update_editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            editor.highlight_text_key(
                crate::display_map::HighlightKey::BufferSearchHighlights,
                vec![
                    buffer.anchor_before(MultiBufferOffset(bold_range.start))
                        ..buffer.anchor_after(MultiBufferOffset(bold_range.end)),
                ],
                HighlightStyle {
                    font_weight: Some(gpui::FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
                false,
                cx,
            );
        });
        let bold = ruler_for(&mut cx);
        assert!(!Arc::ptr_eq(&plain, &bold));
        assert_eq!(plain.starts, bold.starts);
        let affected = |ruler: &RowRuler, range: &Range<usize>| {
            (0..ruler.chunks.len())
                .filter(|ix| {
                    let context = ruler.chunk_context(*ix);
                    (context.start as usize) < range.end && range.start < context.end as usize
                })
                .collect::<Vec<_>>()
        };
        let bold_chunks = affected(&plain, &bold_range);
        assert!(!bold_chunks.is_empty() && bold_chunks.len() <= 2);
        for (ix, (before, after)) in plain.chunks.iter().zip(&bold.chunks).enumerate() {
            assert_eq!(
                before.hash == after.hash,
                !bold_chunks.contains(&ix),
                "chunk {ix} must change exactly when its shaped runs change"
            );
        }

        let unchanged = ruler_for(&mut cx);
        assert!(Arc::ptr_eq(&bold, &unchanged));

        let colored_range = 3 * 1_000..3 * 1_004;
        cx.update_editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            editor.highlight_text_key(
                crate::display_map::HighlightKey::DocumentHighlightRead,
                vec![
                    buffer.anchor_before(MultiBufferOffset(colored_range.start))
                        ..buffer.anchor_after(MultiBufferOffset(colored_range.end)),
                ],
                HighlightStyle {
                    color: Some(gpui::red()),
                    ..HighlightStyle::default()
                },
                false,
                cx,
            );
        });
        let colored = ruler_for(&mut cx);
        assert_eq!(bold.starts, colored.starts);
        let colored_chunks = affected(&bold, &colored_range);
        assert!(!colored_chunks.is_empty());
        for (ix, (before, after)) in bold.chunks.iter().zip(&colored.chunks).enumerate() {
            assert_eq!(
                before.hash == after.hash,
                !colored_chunks.contains(&ix),
                "chunk {ix} must change exactly when its decoration boundaries change"
            );
        }

        cx.update_editor(|editor, _, cx| {
            editor.clear_highlights(crate::display_map::HighlightKey::MatchingBracket, cx);
        });
        let same = ruler_for(&mut cx);
        assert!(Arc::ptr_eq(&colored, &same));
    }

    #[gpui::test]
    async fn test_rows_chunk_only_beside_isolated_chars_or_at_hard_breaks(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!(
            "{} {},{}",
            "x".repeat(5_000),
            "y".repeat(3_000),
            "z".repeat(3_000)
        );
        cx.set_state(&format!("ˇ{text}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(ruler.starts, vec![0, 5_000, 11_002]);
        assert!(
            ruler
                .chunks
                .iter()
                .all(|chunk| chunk.context_before <= SHAPING_CONTEXT)
        );
        assert_eq!(ruler.chunks[1].context_before, SHAPING_CONTEXT);

        let ligatures = format!("\u{754c}aa{}", "!=".repeat(1_050));
        cx.set_state(&format!("ˇ{ligatures}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(ruler.starts, vec![0, ligatures.len() as u32]);

        let punctuation = "!=".repeat(10_000);
        cx.set_state(&format!("ˇ{punctuation}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(
            ruler.starts,
            vec![
                0,
                MAX_CHUNK_LEN,
                2 * MAX_CHUNK_LEN,
                3 * MAX_CHUNK_LEN,
                punctuation.len() as u32
            ]
        );
        for piece in ruler.render_pieces(0..punctuation.len() as u32) {
            assert_eq!(piece.context, piece.chunk);
        }

        let arabic = format!(
            "{} {} {}",
            "\u{628}".repeat(3_000),
            "\u{644}".repeat(1_000),
            "\u{62f}".repeat(3_000)
        );
        cx.set_state(&format!("ˇ{arabic}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let bidi = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(bidi.starts, vec![0, arabic.len() as u32]);
        assert!(bidi.is_rtl());

        let hebrew = format!(
            "{} {} {}",
            "\u{5d0}".repeat(3_000),
            "\u{5d1}".repeat(4_000),
            "\u{5d2}".repeat(3_000)
        );
        cx.set_state(&format!("ˇ{hebrew}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let bidi = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(bidi.starts, vec![0, 14_001, hebrew.len() as u32]);
        assert!(
            bidi.chunks
                .iter()
                .all(|chunk| chunk.context_before == 0 && chunk.context_after == 0)
        );
        assert!(
            bidi.chunks
                .iter()
                .all(|chunk| chunk.len / 2 <= BIDI_CHUNK_UNITS)
        );
        let width = bidi.width();
        let first_width = ScrollPixelOffset::from(bidi.chunks[0].width);
        let pieces = bidi
            .render_pieces(0..hebrew.len() as u32)
            .collect::<Vec<_>>();
        assert_eq!(
            pieces.iter().map(|piece| piece.x).collect::<Vec<_>>(),
            vec![width - first_width, 0.]
        );
        assert_eq!(
            bidi.x_range_for_columns(0..14_001),
            width - first_width..width
        );
        assert_eq!(
            bidi.x_range_for_columns(14_001..hebrew.len() as u32),
            0.0..width - first_width
        );
        assert_eq!(
            bidi.columns_for_x_range(0.0..1.0),
            14_001..hebrew.len() as u32
        );
        assert_eq!(bidi.columns_for_x_range(width - 1.0..width), 0..14_001);
        let x_at = |column: u32| bidi.x_for_column(column, &snapshot, DisplayRow(0), &shaper);
        assert_eq!(x_at(0), width - first_width);
        assert_eq!(x_at(14_001), 0.);
        assert!(x_at(14_001) < x_at(14_004) && x_at(14_004) < x_at(0));
        for column in [0, 2, 6_000, 14_001, 14_004, hebrew.len() as u32 - 2] {
            assert_eq!(
                bidi.column_for_x(x_at(column), &snapshot, DisplayRow(0), &shaper),
                column
            );
        }

        let embedded = format!(
            "abc {} def {}",
            "\u{5d0}".repeat(5_000),
            "\u{5d1}".repeat(5_000)
        );
        cx.set_state(&format!("ˇ{embedded}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let ltr = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert!(!ltr.is_rtl());
        assert_eq!(ltr.starts, vec![0, 10_004, embedded.len() as u32]);
        assert_eq!(ltr.x_range_for_columns(0..10_004), 0.0..ltr.xs[1]);

        let numbers_first = format!(
            "123 {} {}",
            "\u{5d0}".repeat(9_000),
            "\u{5d1}".repeat(1_000)
        );
        cx.set_state(&format!("ˇ{numbers_first}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let bidi = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert!(bidi.is_rtl());
        assert_eq!(bidi.starts, vec![0, 18_004, numbers_first.len() as u32]);
    }

    #[gpui::test]
    async fn test_forced_boundaries_are_hard_shaping_breaks(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let text = format!("a{}", "f".repeat(20_000));
        cx.set_state(&format!("ˇ{text}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(
            ruler.starts,
            vec![
                0,
                MAX_CHUNK_LEN,
                2 * MAX_CHUNK_LEN,
                3 * MAX_CHUNK_LEN,
                text.len() as u32
            ]
        );
        for (ix, chunk) in ruler.chunks.iter().enumerate() {
            assert_eq!(chunk.context_before, 0, "chunk {ix}");
            assert_eq!(chunk.context_after, 0, "chunk {ix}");
        }
        let pieces = ruler
            .render_pieces(0..text.len() as u32)
            .collect::<Vec<_>>();
        assert_eq!(pieces.len(), 4);
        for (piece, next) in pieces.iter().zip(pieces.iter().skip(1)) {
            assert_eq!(piece.context.end, next.context.start);
            assert_eq!(piece.context, piece.chunk);
        }

        let cluster = format!("a{}", "\u{301}".repeat(500_000));
        cx.set_state(&format!("ˇ{cluster}"));
        let snapshot =
            cx.update_editor(|editor, window, cx| editor.snapshot(window, cx).display_snapshot);
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert!(
            ruler.chunks.len()
                >= (cluster.len() as u32 / (MAX_CHUNK_LEN + FORCED_BOUNDARY_SLACK + 4)) as usize
        );
        for chunk in &ruler.chunks {
            assert!(chunk.len + chunk.context_before + chunk.context_after <= MAX_SHAPED_LEN);
        }
    }

    #[gpui::test]
    async fn test_rulers_survive_edits_in_other_rows(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let long = "漢字".repeat(MAX_LINE_LEN);
        cx.set_state(&format!("short\nˇ{long}\nother"));
        let ruler_for = |cx: &mut EditorTestContext, row: u32| {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                snapshot
                    .ruled_row(
                        DisplayRow(row),
                        details.ruler_shaper(&snapshot, DisplayRow(row)),
                    )
                    .ruler
            })
        };
        let first = ruler_for(&mut cx, 1);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(2, 0)..Point::new(2, 0), "x")], cx);
        });
        assert!(Arc::ptr_eq(&first, &ruler_for(&mut cx, 1)));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 0)..Point::new(0, 0), "above\n")], cx);
        });
        assert!(Arc::ptr_eq(&first, &ruler_for(&mut cx, 2)));

        cx.update_editor(|editor, _, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let start = buffer.point_to_offset(Point::new(3, 0));
            editor.highlight_text_key(
                crate::display_map::HighlightKey::DocumentHighlightRead,
                vec![
                    buffer.anchor_before(start)
                        ..buffer.anchor_after(MultiBufferOffset(start.0 + 1)),
                ],
                HighlightStyle {
                    font_weight: Some(gpui::FontWeight::BOLD),
                    ..HighlightStyle::default()
                },
                false,
                cx,
            );
        });
        assert!(Arc::ptr_eq(&first, &ruler_for(&mut cx, 2)));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(2, 3)..Point::new(2, 3), "漢")], cx);
        });
        let edited = ruler_for(&mut cx, 2);
        assert!(!Arc::ptr_eq(&first, &edited));
        assert_eq!(edited.len(), long.len() as u32 + 3);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(2, 0)..Point::new(2, 0), "\n")], cx);
        });
        assert!(Arc::ptr_eq(&edited, &ruler_for(&mut cx, 3)));
    }

    #[gpui::test]
    async fn test_rulers_follow_syntax_changes_outside_the_edited_range(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let language = Arc::new(
            language::Language::new(
                language::LanguageConfig {
                    name: "Rust".into(),
                    ..Default::default()
                },
                Some(tree_sitter_rust::LANGUAGE.into()),
            )
            .with_highlights_query("(block_comment) @comment")
            .unwrap(),
        );
        language.set_theme(&theme::SyntaxTheme::new_test([("comment", gpui::red())]));
        let text = format!("/*\n{}\n*/\nfn main() {{}}", "x".repeat(4_096));
        cx.set_state(&format!("ˇ{text}"));
        cx.update_buffer(|buffer, cx| buffer.set_language(Some(language), cx));
        let parsed = |cx: &mut EditorTestContext| {
            cx.run_until_parked();
            assert!(!cx.buffer(|buffer, _| buffer.is_parsing()));
        };
        parsed(&mut cx);
        let ruler_for = |cx: &mut EditorTestContext| {
            cx.update_editor(|editor, window, cx| {
                let snapshot = editor.snapshot(window, cx).display_snapshot;
                let details = editor.text_layout_details(window, cx);
                snapshot
                    .ruled_row(
                        DisplayRow(1),
                        details.ruler_shaper(&snapshot, DisplayRow(1)),
                    )
                    .ruler
            })
        };
        let commented = ruler_for(&mut cx);

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(3, 3)..Point::new(3, 3), "x")], cx);
        });
        parsed(&mut cx);
        assert!(Arc::ptr_eq(&commented, &ruler_for(&mut cx)));

        cx.update_editor(|editor, _, cx| {
            editor.edit([(Point::new(0, 0)..Point::new(0, 2), "")], cx);
        });
        parsed(&mut cx);
        assert!(!Arc::ptr_eq(&commented, &ruler_for(&mut cx)));
    }

    #[test]
    fn test_grapheme_boundary_search_is_bounded_by_the_limit() {
        let text = format!("a{}b", "\u{301}".repeat(100));
        let cluster_end = text.len() as u32 - 1;
        let boundaries = GraphemeBoundaries::new(&text);
        assert_eq!(boundaries.at_or_after(1, 50), None);
        assert_eq!(boundaries.at_or_after(1, cluster_end), None);
        assert_eq!(
            boundaries.at_or_after(1, text.len() as u32),
            Some(cluster_end)
        );
        assert_eq!(
            boundaries.at_or_after(cluster_end, text.len() as u32),
            Some(cluster_end)
        );
        assert!(boundaries.is_boundary(cluster_end));
        assert!(!boundaries.is_boundary(3));

        let flags = "🇺🇸".repeat(10);
        let boundaries = GraphemeBoundaries::new(&flags);
        assert_eq!(boundaries.at_or_after(1, 7), None);
        assert_eq!(boundaries.at_or_after(1, 8), Some(8));
        assert_eq!(boundaries.at_or_after(9, flags.len() as u32), Some(16));
    }

    #[gpui::test]
    async fn test_ruler_bounds_chunks_across_oversized_grapheme_clusters(cx: &mut TestAppContext) {
        init_test(cx);
        let mut cx = EditorTestContext::new(cx).await;
        let cluster = format!("a{}", "\u{301}".repeat(20_000));
        let text = format!("{}{cluster}{}", "x".repeat(3_000), "y".repeat(3_000));
        cx.set_state(&format!("ˇ{text}"));
        let (snapshot, details) = cx.update_editor(|editor, window, cx| {
            (
                editor.snapshot(window, cx).display_snapshot,
                editor.text_layout_details(window, cx),
            )
        });
        let shaper = details.ruler_shaper(&snapshot, DisplayRow(0));
        let ruler = RowRuler::new(&snapshot, DisplayRow(0), &shaper, None, &HashMap::default());
        assert_eq!(ruler.len(), text.len() as u32);
        let boundaries = GraphemeBoundaries::new(&text);
        let cluster_range = 3_000..3_000 + cluster.len() as u32;
        for (ix, boundary) in ruler.starts.iter().enumerate() {
            let in_cluster = cluster_range.start < *boundary && *boundary < cluster_range.end;
            assert!(
                boundaries.is_boundary(*boundary) || in_cluster,
                "chunk boundary {boundary} splits a grapheme cluster"
            );
            assert!(text.is_char_boundary(*boundary as usize));
            if in_cluster {
                assert_eq!(ruler.chunks[ix - 1].context_after, 0);
                assert_eq!(ruler.chunks[ix].context_before, 0);
            }
        }
        for (ix, chunk) in ruler.chunks.iter().enumerate() {
            assert!(
                chunk.len + chunk.context_before + chunk.context_after <= MAX_SHAPED_LEN,
                "chunk {ix} would shape {} bytes",
                chunk.len + chunk.context_before + chunk.context_after
            );
            let columns = ruler.starts[ix]..ruler.starts[ix + 1];
            let standalone =
                details.shape_row_text(shaper.chunks(&snapshot, DisplayRow(0), columns));
            assert!(
                (chunk.width - standalone.width).abs() < standalone.width * 1e-4 + px(0.5),
                "chunk {ix}: {:?} vs {:?}",
                chunk.width,
                standalone.width
            );
        }
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
                let ruled = snapshot.ruled_row(
                    DisplayRow(0),
                    details.ruler_shaper(&snapshot, DisplayRow(0)),
                );
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
