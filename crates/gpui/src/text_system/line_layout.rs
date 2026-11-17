use crate::{FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString, Size, point, px};
use collections::FxHashMap;
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    cell::RefCell,
    hash::{Hash, Hasher},
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use super::LineWrapper;

/// A laid out and styled line of text
#[derive(Default, Debug)]
pub struct LineLayout {
    /// The font size for this line
    pub font_size: Pixels,
    /// The width of the line
    pub width: Pixels,
    /// The ascent of the line
    pub ascent: Pixels,
    /// The descent of the line
    pub descent: Pixels,
    /// The shaped runs that make up this line
    pub runs: Vec<ShapedRun>,
    /// The length of the line in utf-8 bytes
    pub len: usize,
}

/// A run of text that has been shaped .
#[derive(Debug, Clone)]
pub struct ShapedRun {
    /// The font id for this run
    pub font_id: FontId,
    /// The glyphs that make up this run
    pub glyphs: Vec<ShapedGlyph>,
}

/// A single glyph, ready to paint.
#[derive(Clone, Debug)]
pub struct ShapedGlyph {
    /// The ID for this glyph, as determined by the text system.
    pub id: GlyphId,

    /// The position of this glyph in its containing line.
    pub position: Point<Pixels>,

    /// The index of this glyph in the original text.
    pub index: usize,

    /// Whether this glyph is an emoji
    pub is_emoji: bool,
}

impl LineLayout {
    /// The index for the character at the given x coordinate
    pub fn index_for_x(&self, x: Pixels) -> Option<usize> {
        if x >= self.width {
            None
        } else {
            for run in self.runs.iter().rev() {
                for glyph in run.glyphs.iter().rev() {
                    if glyph.position.x <= x {
                        return Some(glyph.index);
                    }
                }
            }
            Some(0)
        }
    }

    /// closest_index_for_x returns the character boundary closest to the given x coordinate
    /// (e.g. to handle aligning up/down arrow keys)
    pub fn closest_index_for_x(&self, x: Pixels) -> usize {
        let mut prev_index = 0;
        let mut prev_x = px(0.);

        for run in self.runs.iter() {
            for glyph in run.glyphs.iter() {
                if glyph.position.x >= x {
                    if glyph.position.x - x < x - prev_x {
                        return glyph.index;
                    } else {
                        return prev_index;
                    }
                }
                prev_index = glyph.index;
                prev_x = glyph.position.x;
            }
        }

        if self.len == 1 {
            if x > self.width / 2. {
                return 1;
            } else {
                return 0;
            }
        }

        self.len
    }

    /// The x position of the character at the given index
    pub fn x_for_index(&self, index: usize) -> Pixels {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return glyph.position.x;
                }
            }
        }
        self.width
    }

    /// The corresponding Font at the given index
    pub fn font_id_for_index(&self, index: usize) -> Option<FontId> {
        for run in &self.runs {
            for glyph in &run.glyphs {
                if glyph.index >= index {
                    return Some(run.font_id);
                }
            }
        }
        None
    }

    /// Split this layout at a byte index, returning `(prefix, suffix)`.
    ///
    /// - `prefix` contains glyphs for bytes `[0, byte_index)` with original positions.
    ///   Its width equals the x-advance up to the split point.
    /// - `suffix` contains glyphs for bytes `[byte_index, len)` with positions
    ///   shifted left so the first glyph starts at x=0, and byte indices rebased to 0.
    /// - `font_size`, `ascent`, and `descent` are copied to both halves.
    pub fn split_at(&self, byte_index: usize) -> (LineLayout, LineLayout) {
        let x_offset = self.x_for_index(byte_index);

        // Partition glyph runs. A single run may contribute glyphs to both halves.
        let mut left_runs = Vec::new();
        let mut right_runs = Vec::new();

        for run in &self.runs {
            let split_pos = run.glyphs.partition_point(|g| g.index < byte_index);

            if split_pos > 0 {
                left_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: run.glyphs[..split_pos].to_vec(),
                });
            }

            if split_pos < run.glyphs.len() {
                let right_glyphs = run.glyphs[split_pos..]
                    .iter()
                    .map(|g| ShapedGlyph {
                        id: g.id,
                        position: point(g.position.x - x_offset, g.position.y),
                        index: g.index - byte_index,
                        is_emoji: g.is_emoji,
                    })
                    .collect();
                right_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: right_glyphs,
                });
            }
        }

        let left = LineLayout {
            font_size: self.font_size,
            width: x_offset,
            ascent: self.ascent,
            descent: self.descent,
            runs: left_runs,
            len: byte_index,
        };

        let right = LineLayout {
            font_size: self.font_size,
            width: self.width - x_offset,
            ascent: self.ascent,
            descent: self.descent,
            runs: right_runs,
            len: self.len - byte_index,
        };

        (left, right)
    }

    /// This line wrapped to `wrap_width`: laid out again with its rows end to
    /// end, and where each row after the first begins. `None` when it fits.
    ///
    /// Rows are broken in the order the text is read, and each keeps the left
    /// to right order its glyphs have in the whole line, packed together. That
    /// is the order the row would be given on its own: the Unicode
    /// bidirectional algorithm orders a line by reversing contiguous runs of it
    /// (UAX #9, L2), and a contiguous part of a line keeps the order the whole
    /// gave it. Broken in the order the glyphs are drawn, a right-to-left
    /// paragraph began with its last words.
    fn wrap(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
        scratch: &mut WrapScratch,
    ) -> Option<(LineLayout, SmallVec<[WrapBoundary; 1]>)> {
        let WrapScratch {
            drawn,
            clusters,
            break_indices,
        } = scratch;

        // Every glyph left to right, with how far it moves the pen.
        drawn.clear();
        drawn.extend(self.runs.iter().enumerate().flat_map(|(run_ix, run)| {
            run.glyphs
                .iter()
                .enumerate()
                .map(move |(glyph_ix, glyph)| DrawnGlyph {
                    run_ix,
                    glyph_ix,
                    index: glyph.index,
                    x: glyph.position.x,
                    advance: Pixels::ZERO,
                })
        }));
        drawn.sort_by_key(|glyph| glyph.x);
        for ix in 0..drawn.len() {
            let next_x = drawn.get(ix + 1).map_or(self.width, |next| next.x);
            drawn[ix].advance = next_x - drawn[ix].x;
        }

        // The width of each cluster, in the order the text is read. Shaping can
        // combine characters across a break opportunity, so only a cluster's
        // start is a place to break.
        clusters.clear();
        clusters.extend(drawn.iter().map(|glyph| (glyph.index, glyph.advance)));
        clusters.sort_by_key(|(index, _)| *index);
        clusters.dedup_by(|(index, advance), (previous_index, previous_advance)| {
            let same = index == previous_index;
            if same {
                *previous_advance += *advance;
            }
            same
        });
        break_indices.clear();
        break_indices.extend(LineWrapper::break_indices(text).filter(|index| {
            clusters
                .binary_search_by_key(index, |(start, _)| *start)
                .is_ok()
        }));

        let mut row_starts = SmallVec::<[usize; 1]>::new();
        let mut row_start_x = px(0.);
        let mut candidate = None;
        let mut x = px(0.);
        for &(index, advance) in clusters.iter() {
            let row_start = row_starts.last().copied().unwrap_or(0);
            x += advance;
            if text.as_bytes()[index] == b'\n' {
                continue;
            }
            if index > row_start && break_indices.binary_search(&index).is_ok() {
                candidate = Some((index, x - advance));
            }
            if x - row_start_x > wrap_width && index > row_start {
                // When used line_clamp, we should limit the number of lines.
                if max_lines.is_some_and(|max_lines| row_starts.len() + 1 >= max_lines) {
                    break;
                }
                let (start, start_x) = candidate.take().unwrap_or((index, x - advance));
                row_starts.push(start);
                row_start_x = start_x;
            }
        }
        if row_starts.is_empty() {
            return None;
        }

        let mut wrapped = LineLayout {
            font_size: self.font_size,
            width: self.width,
            ascent: self.ascent,
            descent: self.descent,
            runs: Vec::new(),
            len: self.len,
        };
        let mut boundaries = SmallVec::new();
        let mut x = px(0.);
        let mut row_start = 0;
        for row_end in row_starts.iter().copied().chain([self.len]) {
            let mut row_glyphs = drawn
                .iter()
                .filter(|glyph| (row_start..row_end).contains(&glyph.index))
                .peekable();
            if let Some(first) = row_glyphs.peek()
                && row_start > 0
            {
                let font_id = self.runs[first.run_ix].font_id;
                if wrapped.runs.last().is_none_or(|run| run.font_id != font_id) {
                    wrapped.runs.push(ShapedRun {
                        font_id,
                        glyphs: Vec::new(),
                    });
                }
                let run_ix = wrapped.runs.len() - 1;
                boundaries.push(WrapBoundary {
                    run_ix,
                    glyph_ix: wrapped.runs[run_ix].glyphs.len(),
                });
            }
            for drawn_glyph in row_glyphs {
                let run = &self.runs[drawn_glyph.run_ix];
                let glyph = &run.glyphs[drawn_glyph.glyph_ix];
                if wrapped
                    .runs
                    .last()
                    .is_none_or(|last| last.font_id != run.font_id)
                {
                    wrapped.runs.push(ShapedRun {
                        font_id: run.font_id,
                        glyphs: Vec::new(),
                    });
                }
                if let Some(last) = wrapped.runs.last_mut() {
                    last.glyphs.push(ShapedGlyph {
                        position: point(x, glyph.position.y),
                        ..glyph.clone()
                    });
                }
                x += drawn_glyph.advance;
            }
            row_start = row_end;
        }

        Some((wrapped, boundaries))
    }
}

/// A line of text that has been wrapped to fit a given width
#[derive(Default, Debug)]
pub struct WrappedLineLayout {
    /// The line laid out as one: its rows end to end when it was wrapped, each
    /// as it is drawn.
    pub unwrapped_layout: Arc<LineLayout>,

    /// The boundaries at which the line was wrapped
    pub wrap_boundaries: SmallVec<[WrapBoundary; 1]>,

    /// The width of the line, if it was wrapped
    pub wrap_width: Option<Pixels>,
}

/// A boundary at which a line was wrapped
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrapBoundary {
    /// The index in the run just before the line was wrapped
    pub run_ix: usize,
    /// The index of the glyph just before the line was wrapped
    pub glyph_ix: usize,
}

impl WrappedLineLayout {
    /// The length of the underlying text, in utf8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.unwrapped_layout.len
    }

    /// The width of this line, in pixels, whether or not it was wrapped.
    pub fn width(&self) -> Pixels {
        self.wrap_width
            .unwrap_or(Pixels::MAX)
            .min(self.unwrapped_layout.width)
    }

    /// The size of the whole wrapped text, for the given line_height.
    /// can span multiple lines if there are multiple wrap boundaries.
    pub fn size(&self, line_height: Pixels) -> Size<Pixels> {
        Size {
            width: self.width(),
            height: line_height * (self.wrap_boundaries.len() + 1),
        }
    }

    /// The ascent of a line in this layout
    pub fn ascent(&self) -> Pixels {
        self.unwrapped_layout.ascent
    }

    /// The descent of a line in this layout
    pub fn descent(&self) -> Pixels {
        self.unwrapped_layout.descent
    }

    /// The wrap boundaries in this layout
    pub fn wrap_boundaries(&self) -> &[WrapBoundary] {
        &self.wrap_boundaries
    }

    /// The byte offset at which each row starts, in order: a row holds the
    /// text up to where the next one starts.
    ///
    /// Not the index of a row's boundary glyph, which is the first drawn: in a
    /// right-to-left row that is its last character.
    pub fn row_starts(&self) -> SmallVec<[usize; 2]> {
        let mut starts = SmallVec::from_buf_and_len([0; 2], 1);
        let mut wraps = self.wrap_boundaries.iter().peekable();
        for (run_ix, run) in self.unwrapped_layout.runs.iter().enumerate() {
            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                if wraps
                    .next_if_eq(&&WrapBoundary { run_ix, glyph_ix })
                    .is_some()
                {
                    starts.push(glyph.index);
                } else if starts.len() > 1
                    && let Some(start) = starts.last_mut()
                {
                    *start = (*start).min(glyph.index);
                }
            }
        }
        starts
    }

    /// Where each row starts and ends on the unwrapped layout's x axis.
    fn row_extents(&self) -> impl Iterator<Item = (Pixels, Pixels)> + '_ {
        let edges = self
            .wrap_boundaries
            .iter()
            .map(|boundary| {
                self.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix]
                    .position
                    .x
            })
            .collect::<SmallVec<[Pixels; 2]>>();
        let lefts = [Pixels::ZERO].into_iter().chain(edges.clone());
        lefts.zip(edges.into_iter().chain([self.unwrapped_layout.width]))
    }

    /// The font size of this layout
    pub fn font_size(&self) -> Pixels {
        self.unwrapped_layout.font_size
    }

    /// The runs in this layout, sans wrapping
    pub fn runs(&self) -> &[ShapedRun] {
        &self.unwrapped_layout.runs
    }

    /// The index corresponding to a given position in this layout for the given line height.
    ///
    /// See also [`Self::closest_index_for_position`].
    pub fn index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, false)
    }

    /// The closest index to a given position in this layout for the given line height.
    ///
    /// Closest means the character boundary closest to the given position.
    ///
    /// See also [`LineLayout::closest_index_for_x`].
    pub fn closest_index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
    ) -> Result<usize, usize> {
        self._index_for_position(position, line_height, true)
    }

    fn _index_for_position(
        &self,
        position: Point<Pixels>,
        line_height: Pixels,
        closest: bool,
    ) -> Result<usize, usize> {
        let row = (position.y / line_height) as usize;
        let starts = self.row_starts();
        let (Some(&start), Some((left, right))) = (starts.get(row), self.row_extents().nth(row))
        else {
            return Err(0);
        };
        let end = starts.get(row + 1).copied().unwrap_or(self.len());

        let x = position.x + left;
        if x < left {
            Err(start)
        } else if x >= right {
            Err(end)
        } else if closest {
            Ok(self.unwrapped_layout.closest_index_for_x(x))
        } else {
            // The shaper can place a trailing zero-width wrap boundary glyph slightly past
            // the line's width, so the row can extend past where `index_for_x` has glyphs.
            self.unwrapped_layout.index_for_x(x).ok_or(end)
        }
    }

    /// Returns the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        let starts = self.row_starts();
        let ends = starts.iter().skip(1).copied().chain([self.len()]);
        for (row, ((&start, end), (left, _))) in
            starts.iter().zip(ends).zip(self.row_extents()).enumerate()
        {
            if index < start {
                break;
            } else if index <= end {
                let x = self.unwrapped_layout.x_for_index(index) - left;
                return Some(point(x, row as f32 * line_height));
            }
        }

        None
    }
}

pub(crate) struct LineLayoutCache {
    state: RefCell<LineLayoutCacheState>,
    wrap_scratch: RefCell<WrapScratch>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
    /// Advances when [`TextSystem::add_fonts`] successfully changes the font database.
    font_generation: Arc<AtomicUsize>,
    /// Records the generation represented by both frame caches.
    cached_font_generation: AtomicUsize,
}

#[derive(Default)]
struct WrapScratch {
    drawn: Vec<DrawnGlyph>,
    clusters: Vec<(usize, Pixels)>,
    break_indices: Vec<usize>,
}

/// A glyph of a line, found by where it is in the line's runs.
struct DrawnGlyph {
    run_ix: usize,
    glyph_ix: usize,
    /// Where its cluster starts in the text.
    index: usize,
    x: Pixels,
    /// How far it moves the pen: to the next glyph drawn, or the line's end.
    advance: Pixels,
}

#[derive(Default)]
struct LineLayoutCacheState {
    previous_frame: FrameCache,
    current_frame: FrameCache,
}

#[derive(Default)]
struct FrameCache {
    lines: FxHashMap<Arc<CacheKey>, Arc<LineLayout>>,
    wrapped_lines: FxHashMap<Arc<CacheKey>, Arc<WrappedLineLayout>>,
    used_lines: Vec<Arc<CacheKey>>,
    used_wrapped_lines: Vec<Arc<CacheKey>>,
}

#[derive(Clone, Default)]
pub(crate) struct LineLayoutIndex {
    font_generation: usize,
    lines_index: usize,
    wrapped_lines_index: usize,
}

impl LineLayoutCache {
    pub fn new(
        platform_text_system: Arc<dyn PlatformTextSystem>,
        font_generation: Arc<AtomicUsize>,
    ) -> Self {
        let cached_font_generation = font_generation.load(Ordering::Acquire);
        Self {
            state: RefCell::default(),
            wrap_scratch: RefCell::default(),
            platform_text_system,
            font_generation,
            cached_font_generation: AtomicUsize::new(cached_font_generation),
        }
    }

    pub fn layout_index(&self) -> LineLayoutIndex {
        let font_generation = self.clear_if_font_generation_changed();
        let state = self.state.borrow();
        let frame = &state.current_frame;
        LineLayoutIndex {
            font_generation,
            lines_index: frame.used_lines.len(),
            wrapped_lines_index: frame.used_wrapped_lines.len(),
        }
    }

    pub fn reuse_layouts(&self, range: Range<LineLayoutIndex>) {
        let font_generation = self.clear_if_font_generation_changed();
        if range.start.font_generation != font_generation
            || range.end.font_generation != font_generation
        {
            return;
        }
        let mut state = self.state.borrow_mut();
        let LineLayoutCacheState {
            previous_frame,
            current_frame,
        } = &mut *state;

        for key in &previous_frame.used_lines[range.start.lines_index..range.end.lines_index] {
            if let Some((key, line)) = previous_frame.lines.remove_entry(key) {
                current_frame.lines.insert(key, line);
            }
            current_frame.used_lines.push(key.clone());
        }

        for key in &previous_frame.used_wrapped_lines
            [range.start.wrapped_lines_index..range.end.wrapped_lines_index]
        {
            if let Some((key, line)) = previous_frame.wrapped_lines.remove_entry(key) {
                current_frame.wrapped_lines.insert(key, line);
            }
            current_frame.used_wrapped_lines.push(key.clone());
        }
    }

    pub fn truncate_layouts(&self, index: LineLayoutIndex) {
        let font_generation = self.clear_if_font_generation_changed();
        if index.font_generation != font_generation {
            return;
        }
        let mut state = self.state.borrow_mut();
        let current_frame = &mut state.current_frame;
        current_frame.used_lines.truncate(index.lines_index);
        current_frame
            .used_wrapped_lines
            .truncate(index.wrapped_lines_index);
    }

    pub fn finish_frame(&self) {
        let _font_generation = self.clear_if_font_generation_changed();
        let LineLayoutCacheState {
            previous_frame,
            current_frame,
        } = &mut *self.state.borrow_mut();
        std::mem::swap(previous_frame, current_frame);
        current_frame.lines.clear();
        current_frame.wrapped_lines.clear();
        current_frame.used_lines.clear();
        current_frame.used_wrapped_lines.clear();
    }

    pub fn layout_wrapped_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        max_lines: Option<usize>,
    ) -> Arc<WrappedLineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let _font_generation = self.clear_if_font_generation_changed();
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width,
            force_width: None,
        } as &dyn AsCacheKeyRef;

        let state = self.state.borrow();
        if let Some(layout) = state.current_frame.wrapped_lines.get(key) {
            return layout.clone();
        }
        drop(state);

        let mut state = self.state.borrow_mut();
        let previous_frame_entry = state.previous_frame.wrapped_lines.remove_entry(key);
        if let Some((key, layout)) = previous_frame_entry {
            state
                .current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            state.current_frame.used_wrapped_lines.push(key);
            layout
        } else {
            drop(state);
            let text = SharedString::from(text);
            let unwrapped_layout = self.layout_line::<&SharedString>(&text, font_size, runs, None);
            let wrapped = wrap_width.and_then(|wrap_width| {
                unwrapped_layout.wrap(
                    text.as_ref(),
                    wrap_width,
                    max_lines,
                    &mut self.wrap_scratch.borrow_mut(),
                )
            });
            let layout = Arc::new(match wrapped {
                Some((wrapped_layout, wrap_boundaries)) => WrappedLineLayout {
                    unwrapped_layout: Arc::new(wrapped_layout),
                    wrap_boundaries,
                    wrap_width,
                },
                None => WrappedLineLayout {
                    unwrapped_layout,
                    wrap_boundaries: SmallVec::new(),
                    wrap_width,
                },
            });
            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                force_width: None,
            });

            let mut state = self.state.borrow_mut();
            state
                .current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            state.current_frame.used_wrapped_lines.push(key);

            layout
        }
    }

    pub fn layout_line<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Arc<LineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        let _font_generation = self.clear_if_font_generation_changed();
        let key = &CacheKeyRef {
            text: text.as_ref(),
            font_size,
            runs,
            wrap_width: None,
            force_width,
        } as &dyn AsCacheKeyRef;

        let state = self.state.borrow();
        if let Some(layout) = state.current_frame.lines.get(key) {
            return layout.clone();
        }
        drop(state);

        let mut state = self.state.borrow_mut();
        let LineLayoutCacheState {
            previous_frame,
            current_frame,
        } = &mut *state;
        if let Some((key, layout)) = previous_frame.lines.remove_entry(key) {
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        } else {
            let text = SharedString::from(text);
            let mut layout = self
                .platform_text_system
                .layout_line(&text, font_size, runs);

            if let Some(force_width) = force_width {
                apply_force_width_to_layout(&mut layout, force_width);
            }

            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width: None,
                force_width,
            });
            let layout = Arc::new(layout);
            current_frame.lines.insert(key.clone(), layout.clone());
            current_frame.used_lines.push(key);
            layout
        }
    }

    fn clear_if_font_generation_changed(&self) -> usize {
        let font_generation = self.font_generation.load(Ordering::Acquire);
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        let mut state = self.state.borrow_mut();
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        *state = LineLayoutCacheState::default();
        self.cached_font_generation
            .store(font_generation, Ordering::Release);
        font_generation
    }
}

// Combining marks (e.g. Thai vowel signs, Arabic diacritics) are shaped by
// HarfBuzz at the same x position as their base character. The force-width
// loop must not advance the cell counter for these zero-advance glyphs,
// otherwise they get displaced into the next cell. We detect them by checking
// whether shaped x has advanced by at least half a cell beyond the last base.
fn apply_force_width_to_layout(layout: &mut LineLayout, force_width: Pixels) {
    let mut glyph_pos: usize = 0;
    // NEG_INFINITY ensures the first glyph is always classified as a base.
    let mut last_base_shaped_x = px(f32::NEG_INFINITY);
    let mut last_base_actual_x = px(0.);

    for run in layout.runs.iter_mut() {
        for glyph in run.glyphs.iter_mut() {
            let shaped_x = glyph.position.x;

            if shaped_x > last_base_shaped_x + force_width * 0.5 {
                let forced_x = glyph_pos * force_width;
                if (shaped_x - forced_x).abs() > px(1.) {
                    glyph.position.x = forced_x;
                }
                last_base_shaped_x = shaped_x;
                last_base_actual_x = glyph.position.x;
                glyph_pos += 1;
            } else {
                glyph.position.x = last_base_actual_x + (shaped_x - last_base_shaped_x);
            }
        }
    }
}

/// A run of text with a single font.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[expect(missing_docs)]
pub struct FontRun {
    pub len: usize,
    pub font_id: FontId,
}

trait AsCacheKeyRef {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_>;
}

#[derive(Clone, Debug, Eq)]
struct CacheKey {
    text: SharedString,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

impl PartialEq for dyn AsCacheKeyRef + '_ {
    fn eq(&self, other: &dyn AsCacheKeyRef) -> bool {
        self.as_cache_key_ref() == other.as_cache_key_ref()
    }
}

impl Eq for dyn AsCacheKeyRef + '_ {}

impl Hash for dyn AsCacheKeyRef + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state)
    }
}

impl AsCacheKeyRef for CacheKey {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        CacheKeyRef {
            text: &self.text,
            font_size: self.font_size,
            runs: self.runs.as_slice(),
            wrap_width: self.wrap_width,
            force_width: self.force_width,
        }
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_cache_key_ref().eq(&other.as_cache_key_ref())
    }
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_cache_key_ref().hash(state);
    }
}

impl<'a> Borrow<dyn AsCacheKeyRef + 'a> for Arc<CacheKey> {
    fn borrow(&self) -> &(dyn AsCacheKeyRef + 'a) {
        self.as_ref() as &dyn AsCacheKeyRef
    }
}

impl AsCacheKeyRef for CacheKeyRef<'_> {
    fn as_cache_key_ref(&self) -> CacheKeyRef<'_> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GlyphId, NoopTextSystem, TextRun, TextSystem, WindowTextSystem, font};

    #[test]
    fn shape_line_shares_its_text_with_the_layout_cache() {
        let text_system =
            WindowTextSystem::new(Arc::new(TextSystem::new(Arc::new(NoopTextSystem::new()))));
        // Long enough to be stored on the heap rather than inline.
        let text = SharedString::from("let value = compute_something(argument_one, argument_two);");
        let runs = [TextRun {
            len: text.len(),
            font: font(".ZedMono"),
            ..Default::default()
        }];

        text_system.shape_line(text.clone(), px(14.), &runs, None);
        let state = text_system.line_layout_cache.state.borrow();
        let (key, _) = state.current_frame.lines.iter().next().unwrap();
        assert_eq!(key.text.as_ptr(), text.as_ptr());
    }

    fn glyph_at(x: f32, index: usize) -> ShapedGlyph {
        ShapedGlyph {
            id: GlyphId(0),
            position: point(px(x), px(0.)),
            index,
            is_emoji: false,
        }
    }

    fn make_layout(glyphs: Vec<ShapedGlyph>) -> LineLayout {
        LineLayout {
            font_size: px(16.),
            width: px(100.),
            ascent: px(12.),
            descent: px(4.),
            runs: vec![ShapedRun {
                font_id: FontId(0),
                glyphs,
            }],
            len: 0,
        }
    }

    fn glyph_x_positions(layout: &LineLayout) -> Vec<f32> {
        layout.runs[0]
            .glyphs
            .iter()
            .map(|g| f32::from(g.position.x))
            .collect()
    }

    /// Wraps `layout` and returns, row by row, the byte index of each glyph
    /// left to right with its x within the row.
    fn wrapped_rows(layout: &LineLayout, text: &str, wrap_width: f32) -> Vec<Vec<(usize, f32)>> {
        let Some((wrapped, boundaries)) =
            layout.wrap(text, px(wrap_width), None, &mut WrapScratch::default())
        else {
            return vec![
                layout
                    .runs
                    .iter()
                    .flat_map(|run| &run.glyphs)
                    .map(|glyph| (glyph.index, f32::from(glyph.position.x)))
                    .collect(),
            ];
        };
        let mut rows = vec![Vec::new()];
        let mut row_x = 0.;
        let mut wraps = boundaries.iter().peekable();
        for (run_ix, run) in wrapped.runs.iter().enumerate() {
            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                let x = f32::from(glyph.position.x);
                if wraps
                    .next_if_eq(&&WrapBoundary { run_ix, glyph_ix })
                    .is_some()
                {
                    rows.push(Vec::new());
                    row_x = x;
                }
                if let Some(row) = rows.last_mut() {
                    row.push((glyph.index, x - row_x));
                }
            }
        }
        rows
    }

    /// A layout of `text` with one 10 px glyph per entry of `drawn`, the byte
    /// index each glyph is of, left to right.
    fn drawn_layout(text: &str, drawn: &[usize]) -> LineLayout {
        let mut layout = make_layout(
            drawn
                .iter()
                .enumerate()
                .map(|(position, index)| glyph_at(position as f32 * 10., *index))
                .collect(),
        );
        layout.width = px(drawn.len() as f32 * 10.);
        layout.len = text.len();
        layout
    }

    #[test]
    fn test_wrap_preserves_shaped_clusters() {
        let mut layout = make_layout(vec![
            glyph_at(0., 0),
            glyph_at(5., 0),
            glyph_at(10., 3),
            glyph_at(15., 3),
        ]);
        layout.width = px(20.);
        layout.len = "你好".len();
        assert_eq!(
            wrapped_rows(&layout, "你好", 15.),
            [vec![(0, 0.), (0, 5.)], vec![(3, 0.), (3, 5.)]],
        );

        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(10., 0)]);
        layout.width = px(20.);
        layout.len = 1;
        assert!(
            layout
                .wrap("a", px(10.), None, &mut WrapScratch::default())
                .is_none()
        );
    }

    #[test]
    fn a_right_to_left_paragraph_wraps_from_its_first_word() {
        // Drawn right to left: the first word, אבג, at the right.
        let text = "אבג דהו";
        let layout = drawn_layout(text, &[11, 9, 7, 6, 4, 2, 0]);
        assert_eq!(
            wrapped_rows(&layout, text, 40.),
            [
                vec![(6, 0.), (4, 10.), (2, 20.), (0, 30.)],
                vec![(11, 0.), (9, 10.), (7, 20.)],
            ],
        );
    }

    #[test]
    fn a_row_of_a_right_to_left_paragraph_keeps_its_latin_word_at_the_right() {
        // "אב ab גד": drawn as גד ab אב, the paragraph read from the right.
        let text = "אב ab גד";
        let layout = drawn_layout(text, &[10, 8, 7, 5, 6, 4, 2, 0]);
        assert_eq!(
            wrapped_rows(&layout, text, 50.),
            [
                vec![(4, 0.), (2, 10.), (0, 20.)],
                vec![(10, 0.), (8, 10.), (7, 20.), (5, 30.), (6, 40.)],
            ],
            "the second row starts with ab, which stays right of גד",
        );
    }

    #[test]
    fn a_left_to_right_paragraph_with_a_right_to_left_word_wraps_in_reading_order() {
        let text = "a אבג b";
        let layout = drawn_layout(text, &[0, 1, 6, 4, 2, 8, 9]);
        assert_eq!(
            wrapped_rows(&layout, text, 40.),
            [
                vec![(0, 0.), (1, 10.)],
                vec![(6, 0.), (4, 10.), (2, 20.), (8, 30.)],
                vec![(9, 0.)],
            ],
        );
    }

    #[test]
    fn test_wrap_ignores_breaks_inside_shaped_clusters() {
        let text = "x a-bcde";
        let mut layout = make_layout(
            [0, 1, 2, 5, 6, 7]
                .into_iter()
                .map(|index| glyph_at(index as f32 * 10., index))
                .collect(),
        );
        layout.width = px(80.);
        layout.len = text.len();
        assert_eq!(
            wrapped_rows(&layout, text, 60.),
            [
                vec![(0, 0.), (1, 10.)],
                vec![(2, 0.), (5, 30.), (6, 40.), (7, 50.)],
            ],
        );
    }

    #[test]
    fn test_force_width_latin_unchanged() {
        let cell_width = px(8.);
        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(8., 1), glyph_at(16., 2)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 8., 16.]);
    }

    #[test]
    fn test_force_width_combining_marks_not_advanced() {
        let cell_width = px(8.);
        // Simulates Thai "กี" — base consonant at x=0, combining vowel also at x=0
        let mut layout = make_layout(vec![
            glyph_at(0., 0), // ก (base)
            glyph_at(0., 3), // ี (combining mark, same x)
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0.]);
    }

    #[test]
    fn test_force_width_base_after_combining_mark() {
        let cell_width = px(8.);
        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(0., 3), glyph_at(8., 6)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0., 8.]);
    }

    #[test]
    fn test_force_width_multiple_combining_marks() {
        let cell_width = px(8.);
        // Simulates "ก้" — base + vowel + tone mark (two combining marks stacked)
        let mut layout = make_layout(vec![
            glyph_at(0., 0), // ก (base)
            glyph_at(0., 3), // vowel (combining)
            glyph_at(0., 6), // tone mark (combining)
            glyph_at(8., 9), // next base
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0., 0., 0., 8.]);
    }

    #[test]
    fn test_force_width_corrects_drifted_base_positions() {
        let cell_width = px(8.);
        // Font metrics don't perfectly match cell grid — glyphs drift >1px from cell boundary
        let mut layout = make_layout(vec![
            glyph_at(0.5, 0),  // within 1px tolerance, kept as-is
            glyph_at(10.2, 1), // >1px off from 8.0, corrected
            glyph_at(19.8, 2), // >1px off from 16.0, corrected
        ]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0.5, 8., 16.]);
    }

    #[test]
    fn test_force_width_combining_mark_after_within_tolerance_base() {
        let cell_width = px(8.);
        // Base glyph is within 1px of grid so it keeps its shaped position.
        // The combining mark must align to the base's actual position, not the grid slot.
        let mut layout = make_layout(vec![glyph_at(0.5, 0), glyph_at(0.5, 3)]);

        apply_force_width_to_layout(&mut layout, cell_width);

        let positions = glyph_x_positions(&layout);
        assert_eq!(positions, vec![0.5, 0.5]);
    }
}
