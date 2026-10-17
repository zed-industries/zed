use crate::{FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString, Size, point, px};
use collections::FxHashMap;
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
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
    #[expect(missing_docs)]
    pub caret_positions: Vec<(usize, f64)>,
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
            self.index_for_x_with_origin(f64::from(x), 0.)
        }
    }

    /// closest_index_for_x returns the character boundary closest to the given x coordinate
    /// (e.g. to handle aligning up/down arrow keys)
    pub fn closest_index_for_x(&self, x: Pixels) -> usize {
        self.closest_index_for_x_with_origin(f64::from(x), 0.)
    }

    /// The x position of the character at the given index
    pub fn x_for_index(&self, index: usize) -> Pixels {
        Pixels::from(self.caret_x_for_index(index))
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
    /// - `font_size`, `ascent`, and `descent` are copied to both halves.
    pub fn split_at(&self, byte_index: usize) -> (LineLayout, LineLayout) {
        let caret_offset = self.caret_x_for_index(byte_index);
        let x_offset = Pixels::from(caret_offset);

        // Partition glyph runs. A single run may contribute glyphs to both halves.
        let mut left_runs = Vec::new();
        let mut right_runs = Vec::new();

        for run in &self.runs {
            let mut left_glyphs = Vec::new();
            let mut right_glyphs = Vec::new();
            for glyph in &run.glyphs {
                if glyph.index < byte_index {
                    left_glyphs.push(glyph.clone());
                } else {
                    right_glyphs.push(ShapedGlyph {
                        id: glyph.id,
                        position: point(
                            Pixels::from(f64::from(glyph.position.x) - caret_offset),
                            glyph.position.y,
                        ),
                        index: glyph.index - byte_index,
                        is_emoji: glyph.is_emoji,
                    });
                }
            }
            if !left_glyphs.is_empty() {
                left_runs.push(ShapedRun {
                    font_id: run.font_id,
                    glyphs: left_glyphs,
                });
            }
            if !right_glyphs.is_empty() {
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
            caret_positions: self.slice_caret_positions(0..byte_index, 0.),
        };

        let right = LineLayout {
            font_size: self.font_size,
            width: Pixels::from(self.caret_x_for_index(self.len) - caret_offset),
            ascent: self.ascent,
            descent: self.descent,
            runs: right_runs,
            len: self.len - byte_index,
            caret_positions: self.slice_caret_positions(byte_index..self.len, caret_offset),
        };

        (left, right)
    }

    #[expect(missing_docs)]
    pub fn caret_x_for_index(&self, index: usize) -> f64 {
        if !self.caret_positions.is_empty() {
            let position = self
                .caret_positions
                .partition_point(|(boundary, _)| *boundary < index);
            return self
                .caret_positions
                .get(position)
                .filter(|(boundary, _)| *boundary <= self.len)
                .map_or_else(|| self.caret_width(), |(_, position)| *position);
        }
        self.caret_boundaries()
            .find(|(boundary, _)| *boundary >= index)
            .map_or(f64::from(self.width), |(_, position)| position)
    }

    #[expect(missing_docs)]
    pub fn slice_caret_positions(&self, range: Range<usize>, origin: f64) -> Vec<(usize, f64)> {
        if self.caret_positions.is_empty() || range.is_empty() {
            return Vec::new();
        }
        let start = self
            .caret_positions
            .partition_point(|(index, _)| *index < range.start);
        let end = self
            .caret_positions
            .partition_point(|(index, _)| *index < range.end);
        let mut positions = Vec::new();
        if self
            .caret_positions
            .get(start)
            .is_none_or(|(index, _)| *index != range.start)
        {
            positions.push((0, 0.));
        }
        positions.extend(
            self.caret_positions[start..end]
                .iter()
                .map(|(index, position)| (index - range.start, position - origin)),
        );
        positions.push((range.len(), self.caret_x_for_index(range.end) - origin));
        positions
    }

    #[expect(missing_docs)]
    pub fn cluster_start_for_index(&self, index: usize) -> (usize, f64) {
        if self.caret_positions.is_empty() {
            return (index, self.caret_x_for_index(index));
        }
        self.caret_positions
            .partition_point(|(boundary, _)| *boundary <= index)
            .checked_sub(1)
            .and_then(|position| self.caret_positions.get(position))
            .copied()
            .unwrap_or((0, 0.))
    }

    fn index_for_x_with_origin(&self, x: f64, origin: f64) -> Option<usize> {
        if x >= self.caret_width() - origin {
            None
        } else {
            Some(
                self.caret_boundaries()
                    .filter(|(_, position)| *position - origin <= x)
                    .last()
                    .map_or(0, |(index, _)| index),
            )
        }
    }

    fn closest_index_for_x_with_origin(&self, x: f64, origin: f64) -> usize {
        let mut previous_index = 0;
        let mut previous_x = -origin;
        for (index, position) in self.caret_boundaries() {
            let position = position - origin;
            if position >= x {
                return if position - x < x - previous_x {
                    index
                } else {
                    previous_index
                };
            }
            previous_index = index;
            previous_x = position;
        }
        self.len
    }

    fn caret_width(&self) -> f64 {
        self.caret_positions
            .last()
            .filter(|(index, _)| *index == self.len)
            .map_or(f64::from(self.width), |(_, position)| *position)
    }

    fn caret_boundaries(&self) -> impl Iterator<Item = (usize, f64)> {
        let fallback_runs = if self.caret_positions.is_empty() {
            self.runs.len()
        } else {
            0
        };
        let mut previous_index = None;
        self.caret_positions
            .iter()
            .copied()
            .chain(
                self.runs
                    .iter()
                    .take(fallback_runs)
                    .flat_map(|run| &run.glyphs)
                    .map(|glyph| (glyph.index, f64::from(glyph.position.x))),
            )
            .filter(move |(index, _)| {
                if *index >= self.len || previous_index == Some(*index) {
                    return false;
                }
                previous_index = Some(*index);
                true
            })
            .chain([(self.len, self.caret_width())])
    }

    fn compute_wrap_boundaries(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
    ) -> SmallVec<[WrapBoundary; 1]> {
        let mut boundaries = SmallVec::new();
        let mut first_non_whitespace_ix = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = 0.;
        let mut last_boundary = WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_boundary_x = 0.;
        let mut prev_ch = '\0';
        let mut previous_cluster = None;
        let mut glyphs = self
            .runs
            .iter()
            .enumerate()
            .flat_map(|(run_ix, run)| {
                run.glyphs
                    .iter()
                    .enumerate()
                    .map(move |(glyph_ix, glyph)| (WrapBoundary { run_ix, glyph_ix }, glyph))
            })
            .filter_map(|(boundary, glyph)| {
                let (index, position) = if self.caret_positions.is_empty() {
                    (glyph.index, f64::from(glyph.position.x))
                } else {
                    self.cluster_start_for_index(glyph.index)
                };
                if previous_cluster == Some(index) || index >= text.len() {
                    return None;
                }
                previous_cluster = Some(index);
                Some((boundary, text.get(index..)?.chars().next()?, position))
            })
            .peekable();

        while let Some((boundary, ch, x)) = glyphs.next() {
            if ch == '\n' {
                continue;
            }

            // Here is very similar to `LineWrapper::wrap_line` to determine text wrapping,
            // but there are some differences, so we have to duplicate the code here.
            if LineWrapper::is_word_char(ch) {
                if prev_ch == ' ' && ch != ' ' && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = Some(boundary);
                    last_candidate_x = x;
                }
            } else {
                if ch != ' ' && first_non_whitespace_ix.is_some() {
                    last_candidate_ix = Some(boundary);
                    last_candidate_x = x;
                }
            }

            if ch != ' ' && first_non_whitespace_ix.is_none() {
                first_non_whitespace_ix = Some(boundary);
            }

            let next_x = glyphs.peek().map_or(self.caret_width(), |(_, _, x)| *x);
            let width = next_x - last_boundary_x;

            if width > f64::from(wrap_width) && boundary > last_boundary {
                // When used line_clamp, we should limit the number of lines.
                if let Some(max_lines) = max_lines
                    && boundaries.len() >= max_lines.saturating_sub(1)
                {
                    break;
                }

                if let Some(last_candidate_ix) = last_candidate_ix.take() {
                    last_boundary = last_candidate_ix;
                    last_boundary_x = last_candidate_x;
                } else {
                    last_boundary = boundary;
                    last_boundary_x = x;
                }
                boundaries.push(last_boundary);
            }
            prev_ch = ch;
        }

        boundaries
    }
}

/// A line of text that has been wrapped to fit a given width
#[derive(Default, Debug)]
pub struct WrappedLineLayout {
    /// The line layout, pre-wrapping.
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
        let wrapped_line_ix = (position.y / line_height) as usize;

        let wrapped_line_start_index;
        let wrapped_line_start_x;
        if wrapped_line_ix > 0 {
            let Some(line_start_boundary) = self.wrap_boundaries.get(wrapped_line_ix - 1) else {
                return Err(0);
            };
            let run = &self.unwrapped_layout.runs[line_start_boundary.run_ix];
            let glyph = &run.glyphs[line_start_boundary.glyph_ix];
            (wrapped_line_start_index, wrapped_line_start_x) =
                self.unwrapped_layout.cluster_start_for_index(glyph.index);
        } else {
            wrapped_line_start_index = 0;
            wrapped_line_start_x = 0.;
        };

        let wrapped_line_end_index;
        let wrapped_line_end_x;
        if wrapped_line_ix < self.wrap_boundaries.len() {
            let next_wrap_boundary_ix = wrapped_line_ix;
            let next_wrap_boundary = self.wrap_boundaries[next_wrap_boundary_ix];
            let run = &self.unwrapped_layout.runs[next_wrap_boundary.run_ix];
            let glyph = &run.glyphs[next_wrap_boundary.glyph_ix];
            (wrapped_line_end_index, wrapped_line_end_x) =
                self.unwrapped_layout.cluster_start_for_index(glyph.index);
        } else {
            wrapped_line_end_index = self.unwrapped_layout.len;
            wrapped_line_end_x = self.unwrapped_layout.caret_width();
        };

        let x = f64::from(position.x);
        if x < 0. {
            Err(wrapped_line_start_index)
        } else if x >= wrapped_line_end_x - wrapped_line_start_x {
            Err(wrapped_line_end_index)
        } else {
            if closest {
                Ok(self
                    .unwrapped_layout
                    .closest_index_for_x_with_origin(x, wrapped_line_start_x))
            } else {
                // The shaper can place a trailing zero-width wrap boundary glyph slightly past
                // the line's width, so the row can extend past where `index_for_x` has glyphs.
                self.unwrapped_layout
                    .index_for_x_with_origin(x, wrapped_line_start_x)
                    .ok_or(wrapped_line_end_index)
            }
        }
    }

    /// Returns the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        let mut line_start_ix = 0;
        let mut line_end_indices = self
            .wrap_boundaries
            .iter()
            .map(|wrap_boundary| {
                let run = &self.unwrapped_layout.runs[wrap_boundary.run_ix];
                let glyph = &run.glyphs[wrap_boundary.glyph_ix];
                self.unwrapped_layout.cluster_start_for_index(glyph.index).0
            })
            .chain([self.len()])
            .enumerate();
        for (ix, line_end_ix) in line_end_indices {
            let line_y = ix as f32 * line_height;
            if index < line_start_ix {
                break;
            } else if index > line_end_ix {
                line_start_ix = line_end_ix;
                continue;
            } else {
                let line_start_x = self.unwrapped_layout.caret_x_for_index(line_start_ix);
                let x = self.unwrapped_layout.caret_x_for_index(index) - line_start_x;
                return Some(point(Pixels::from(x), line_y));
            }
        }

        None
    }
}

pub(crate) struct LineLayoutCache {
    previous_frame: Mutex<FrameCache>,
    current_frame: RwLock<FrameCache>,
    platform_text_system: Arc<dyn PlatformTextSystem>,
    /// Advances when [`TextSystem::add_fonts`] successfully changes the font database.
    font_generation: Arc<AtomicUsize>,
    /// Records the generation represented by both frame caches.
    cached_font_generation: AtomicUsize,
}

#[derive(Default)]
struct FrameCache {
    lines: FxHashMap<Arc<CacheKey>, Arc<LineLayout>>,
    wrapped_lines: FxHashMap<Arc<CacheKey>, Arc<WrappedLineLayout>>,
    used_lines: Vec<Arc<CacheKey>>,
    used_wrapped_lines: Vec<Arc<CacheKey>>,

    // Content-addressable caches keyed by caller-provided text hash + layout params.
    // These allow cache hits without materializing a contiguous `SharedString`.
    //
    // IMPORTANT: To support allocation-free lookups, we store these maps using a key type
    // (`HashedCacheKeyRef`) that can be computed without building a contiguous `&str`/`SharedString`.
    // On miss, we allocate once and store under an owned `HashedCacheKey`.
    lines_by_hash: FxHashMap<Arc<HashedCacheKey>, Arc<LineLayout>>,
    wrapped_lines_by_hash: FxHashMap<Arc<HashedCacheKey>, Arc<WrappedLineLayout>>,
    used_lines_by_hash: Vec<Arc<HashedCacheKey>>,
    used_wrapped_lines_by_hash: Vec<Arc<HashedCacheKey>>,
}

#[derive(Clone, Default)]
pub(crate) struct LineLayoutIndex {
    font_generation: usize,
    lines_index: usize,
    wrapped_lines_index: usize,
    lines_by_hash_index: usize,
    wrapped_lines_by_hash_index: usize,
}

impl LineLayoutCache {
    pub fn new(
        platform_text_system: Arc<dyn PlatformTextSystem>,
        font_generation: Arc<AtomicUsize>,
    ) -> Self {
        let cached_font_generation = font_generation.load(Ordering::Acquire);
        Self {
            previous_frame: Mutex::default(),
            current_frame: RwLock::default(),
            platform_text_system,
            font_generation,
            cached_font_generation: AtomicUsize::new(cached_font_generation),
        }
    }

    pub fn layout_index(&self) -> LineLayoutIndex {
        let font_generation = self.clear_if_font_generation_changed();
        let frame = self.current_frame.read();
        LineLayoutIndex {
            font_generation,
            lines_index: frame.used_lines.len(),
            wrapped_lines_index: frame.used_wrapped_lines.len(),
            lines_by_hash_index: frame.used_lines_by_hash.len(),
            wrapped_lines_by_hash_index: frame.used_wrapped_lines_by_hash.len(),
        }
    }

    pub fn reuse_layouts(&self, range: Range<LineLayoutIndex>) {
        let font_generation = self.clear_if_font_generation_changed();
        if range.start.font_generation != font_generation
            || range.end.font_generation != font_generation
        {
            return;
        }
        let mut current_frame = &mut *self.current_frame.write();
        let mut previous_frame = &mut *self.previous_frame.lock();

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

        for key in &previous_frame.used_lines_by_hash
            [range.start.lines_by_hash_index..range.end.lines_by_hash_index]
        {
            if let Some((key, line)) = previous_frame.lines_by_hash.remove_entry(key) {
                current_frame.lines_by_hash.insert(key, line);
            }
            current_frame.used_lines_by_hash.push(key.clone());
        }

        for key in &previous_frame.used_wrapped_lines_by_hash
            [range.start.wrapped_lines_by_hash_index..range.end.wrapped_lines_by_hash_index]
        {
            if let Some((key, line)) = previous_frame.wrapped_lines_by_hash.remove_entry(key) {
                current_frame.wrapped_lines_by_hash.insert(key, line);
            }
            current_frame.used_wrapped_lines_by_hash.push(key.clone());
        }
    }

    pub fn truncate_layouts(&self, index: LineLayoutIndex) {
        let font_generation = self.clear_if_font_generation_changed();
        if index.font_generation != font_generation {
            return;
        }
        let mut current_frame = &mut *self.current_frame.write();
        current_frame.used_lines.truncate(index.lines_index);
        current_frame
            .used_wrapped_lines
            .truncate(index.wrapped_lines_index);
        current_frame
            .used_lines_by_hash
            .truncate(index.lines_by_hash_index);
        current_frame
            .used_wrapped_lines_by_hash
            .truncate(index.wrapped_lines_by_hash_index);
    }

    pub fn finish_frame(&self) {
        let _font_generation = self.clear_if_font_generation_changed();
        let mut curr_frame = self.current_frame.write();
        let mut prev_frame = self.previous_frame.lock();
        std::mem::swap(&mut *prev_frame, &mut *curr_frame);
        curr_frame.lines.clear();
        curr_frame.wrapped_lines.clear();
        curr_frame.used_lines.clear();
        curr_frame.used_wrapped_lines.clear();

        curr_frame.lines_by_hash.clear();
        curr_frame.wrapped_lines_by_hash.clear();
        curr_frame.used_lines_by_hash.clear();
        curr_frame.used_wrapped_lines_by_hash.clear();
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

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.wrapped_lines.get(key) {
            return layout.clone();
        }

        let previous_frame_entry = self.previous_frame.lock().wrapped_lines.remove_entry(key);
        if let Some((key, layout)) = previous_frame_entry {
            let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);
            layout
        } else {
            drop(current_frame);
            let text = SharedString::from(text);
            let unwrapped_layout = self.layout_line::<&SharedString>(&text, font_size, runs, None);
            let wrap_boundaries = if let Some(wrap_width) = wrap_width {
                unwrapped_layout.compute_wrap_boundaries(text.as_ref(), wrap_width, max_lines)
            } else {
                SmallVec::new()
            };
            let layout = Arc::new(WrappedLineLayout {
                unwrapped_layout,
                wrap_boundaries,
                wrap_width,
            });
            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                force_width: None,
            });

            let mut current_frame = self.current_frame.write();
            current_frame
                .wrapped_lines
                .insert(key.clone(), layout.clone());
            current_frame.used_wrapped_lines.push(key);

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

        let current_frame = self.current_frame.upgradable_read();
        if let Some(layout) = current_frame.lines.get(key) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);
        if let Some((key, layout)) = self.previous_frame.lock().lines.remove_entry(key) {
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

    /// Try to retrieve a previously-shaped line layout using a caller-provided content hash.
    ///
    /// This is a *non-allocating* cache probe: it does not materialize any text. If the layout
    /// is not already cached in either the current frame or previous frame, returns `None`.
    ///
    /// Contract (caller enforced):
    /// - Same `text_hash` implies identical text content (collision risk accepted by caller).
    /// - `text_len` should be the UTF-8 byte length of the text (helps reduce accidental collisions).
    pub fn try_layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
    ) -> Option<Arc<LineLayout>> {
        let _font_generation = self.clear_if_font_generation_changed();
        let key_ref = HashedCacheKeyRef {
            text_hash,
            text_len,
            font_size,
            runs,
            wrap_width: None,
            force_width,
        };

        let current_frame = self.current_frame.read();
        if let Some((_, layout)) = current_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return Some(layout.clone());
        }

        let previous_frame = self.previous_frame.lock();
        if let Some((_, layout)) = previous_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return Some(layout.clone());
        }

        None
    }

    /// Layout a line of text using a caller-provided content hash as the cache key.
    ///
    /// This enables cache hits without materializing a contiguous `SharedString` for `text`.
    /// If the cache misses, `materialize_text` is invoked to produce the `SharedString` for shaping.
    ///
    /// Contract (caller enforced):
    /// - Same `text_hash` implies identical text content (collision risk accepted by caller).
    /// - `text_len` should be the UTF-8 byte length of the text (helps reduce accidental collisions).
    pub fn layout_line_by_hash(
        &self,
        text_hash: u64,
        text_len: usize,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
        materialize_text: impl FnOnce() -> SharedString,
    ) -> Arc<LineLayout> {
        let _font_generation = self.clear_if_font_generation_changed();
        let key_ref = HashedCacheKeyRef {
            text_hash,
            text_len,
            font_size,
            runs,
            wrap_width: None,
            force_width,
        };

        // Fast path: already cached (no allocation).
        let current_frame = self.current_frame.upgradable_read();
        if let Some((_, layout)) = current_frame.lines_by_hash.iter().find(|(key, _)| {
            HashedCacheKeyRef {
                text_hash: key.text_hash,
                text_len: key.text_len,
                font_size: key.font_size,
                runs: key.runs.as_slice(),
                wrap_width: key.wrap_width,
                force_width: key.force_width,
            } == key_ref
        }) {
            return layout.clone();
        }

        let mut current_frame = RwLockUpgradableReadGuard::upgrade(current_frame);

        // Try to reuse from previous frame without allocating; do a linear scan to find a matching key.
        // (We avoid `drain()` here because it would eagerly move all entries.)
        let mut previous_frame = self.previous_frame.lock();
        if let Some(existing_key) = previous_frame
            .used_lines_by_hash
            .iter()
            .find(|key| {
                HashedCacheKeyRef {
                    text_hash: key.text_hash,
                    text_len: key.text_len,
                    font_size: key.font_size,
                    runs: key.runs.as_slice(),
                    wrap_width: key.wrap_width,
                    force_width: key.force_width,
                } == key_ref
            })
            .cloned()
        {
            if let Some((key, layout)) = previous_frame.lines_by_hash.remove_entry(&existing_key) {
                current_frame
                    .lines_by_hash
                    .insert(key.clone(), layout.clone());
                current_frame.used_lines_by_hash.push(key);
                return layout;
            }
        }

        let text = materialize_text();
        let mut layout = self
            .platform_text_system
            .layout_line(&text, font_size, runs);

        if let Some(force_width) = force_width {
            apply_force_width_to_layout(&mut layout, force_width);
        }

        let key = Arc::new(HashedCacheKey {
            text_hash,
            text_len,
            font_size,
            runs: SmallVec::from(runs),
            wrap_width: None,
            force_width,
        });
        let layout = Arc::new(layout);
        current_frame
            .lines_by_hash
            .insert(key.clone(), layout.clone());
        current_frame.used_lines_by_hash.push(key);
        layout
    }

    fn clear_if_font_generation_changed(&self) -> usize {
        let font_generation = self.font_generation.load(Ordering::Acquire);
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        let mut current_frame = self.current_frame.write();
        if self.cached_font_generation.load(Ordering::Acquire) == font_generation {
            return font_generation;
        }

        *current_frame = FrameCache::default();
        *self.previous_frame.lock() = FrameCache::default();
        self.cached_font_generation
            .store(font_generation, Ordering::Release);
        font_generation
    }
}

fn apply_force_width_to_layout(layout: &mut LineLayout, force_width: Pixels) {
    layout.caret_positions.clear();
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

#[derive(Clone, Debug)]
struct HashedCacheKey {
    text_hash: u64,
    text_len: usize,
    font_size: Pixels,
    runs: SmallVec<[FontRun; 1]>,
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
}

#[derive(Copy, Clone)]
struct HashedCacheKeyRef<'a> {
    text_hash: u64,
    text_len: usize,
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

impl PartialEq for HashedCacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.text_hash == other.text_hash
            && self.text_len == other.text_len
            && self.font_size == other.font_size
            && self.runs.as_slice() == other.runs.as_slice()
            && self.wrap_width == other.wrap_width
            && self.force_width == other.force_width
    }
}

impl Eq for HashedCacheKey {}

impl Hash for HashedCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_hash.hash(state);
        self.text_len.hash(state);
        self.font_size.hash(state);
        self.runs.as_slice().hash(state);
        self.wrap_width.hash(state);
        self.force_width.hash(state);
    }
}

impl PartialEq for HashedCacheKeyRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.text_hash == other.text_hash
            && self.text_len == other.text_len
            && self.font_size == other.font_size
            && self.runs == other.runs
            && self.wrap_width == other.wrap_width
            && self.force_width == other.force_width
    }
}

impl Eq for HashedCacheKeyRef<'_> {}

impl Hash for HashedCacheKeyRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text_hash.hash(state);
        self.text_len.hash(state);
        self.font_size.hash(state);
        self.runs.hash(state);
        self.wrap_width.hash(state);
        self.force_width.hash(state);
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
    use crate::GlyphId;

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
            caret_positions: Vec::new(),
        }
    }

    fn glyph_x_positions(layout: &LineLayout) -> Vec<f32> {
        layout.runs[0]
            .glyphs
            .iter()
            .map(|g| f32::from(g.position.x))
            .collect()
    }

    #[test]
    fn test_caret_boundaries_ignore_glyph_offsets() {
        for mark_index in [1, 2] {
            for offset in [-32., 0., 32.] {
                let layout = LineLayout {
                    font_size: px(16.),
                    width: px(30.),
                    ascent: px(12.),
                    descent: px(4.),
                    runs: vec![
                        ShapedRun {
                            font_id: FontId(3),
                            glyphs: vec![glyph_at(offset, 0), glyph_at(5. + offset, 1)],
                        },
                        ShapedRun {
                            font_id: FontId(8),
                            glyphs: vec![
                                glyph_at(9. + offset, mark_index),
                                glyph_at(14. + offset, 4),
                                glyph_at(17. + offset, 5),
                            ],
                        },
                    ],
                    len: " X\u{0301} fi".len(),
                    caret_positions: vec![(0, 0.), (1, 4.), (4, 14.), (5, 18.)],
                };
                for (x, hit, closest) in [
                    (-1., Some(0), 0),
                    (0., Some(0), 0),
                    (3., Some(0), 1),
                    (4., Some(1), 1),
                    (9., Some(1), 1),
                    (10., Some(1), 4),
                    (14., Some(4), 4),
                    (16., Some(4), 4),
                    (17., Some(4), 5),
                    (18., Some(5), 5),
                    (24., Some(5), 5),
                    (25., Some(5), 7),
                    (30., None, 7),
                ] {
                    assert_eq!(layout.index_for_x(px(x)), hit);
                    assert_eq!(layout.closest_index_for_x(px(x)), closest);
                }
                for (index, x) in [
                    (0, 0.),
                    (1, 4.),
                    (2, 14.),
                    (4, 14.),
                    (5, 18.),
                    (6, 30.),
                    (7, 30.),
                ] {
                    assert_eq!(layout.x_for_index(index), px(x));
                }
                let (left, right) = layout.split_at(4);
                assert_eq!((left.len, right.len), (4, 3));
                assert_eq!((left.width, right.width), (px(14.), px(16.)));
                assert_eq!(left.caret_positions, [(0, 0.), (1, 4.), (4, 14.)]);
                assert_eq!(right.caret_positions, [(0, 0.), (1, 4.), (3, 16.)]);
                for part in [&left, &right] {
                    assert_eq!(
                        (part.font_size, part.ascent, part.descent),
                        (px(16.), px(12.), px(4.))
                    );
                }
                assert_eq!(right.runs[0].font_id, FontId(8));
                assert_eq!(right.runs[0].glyphs[0].position.x, px(offset));
                assert_eq!(right.runs[0].glyphs[1].position.x, px(3. + offset));
                assert_eq!(right.closest_index_for_x(px(11.)), 3);
                let (_, mark_and_suffix) = layout.split_at(2);
                let (mark, _) = mark_and_suffix.split_at(2);
                assert_eq!(mark.caret_positions, [(0, 0.), (2, 0.)]);
                assert_eq!(mark.width, px(0.));
                assert_eq!(mark.x_for_index(0), px(0.));
                let boundaries = layout.compute_wrap_boundaries(" X\u{0301} fi", px(18.), None);
                assert_eq!(
                    boundaries.as_slice(),
                    &[WrapBoundary {
                        run_ix: 1,
                        glyph_ix: 2
                    }]
                );
            }
        }
    }

    #[test]
    fn test_reordered_ltr_clusters_wrap_and_split_by_source_index() {
        let mut layout = make_layout(vec![
            glyph_at(0., 3),
            glyph_at(4., 0),
            glyph_at(16., 9),
            glyph_at(20., 6),
            glyph_at(32., 12),
        ]);
        layout.len = "किकिa".len();
        layout.width = px(40.);
        layout.caret_positions = vec![(0, 0.), (6, 16.), (12, 32.), (13, 40.)];
        let boundaries = layout.compute_wrap_boundaries("किकिa", px(18.), None);
        assert_eq!(
            boundaries.as_slice(),
            [
                WrapBoundary {
                    run_ix: 0,
                    glyph_ix: 2
                },
                WrapBoundary {
                    run_ix: 0,
                    glyph_ix: 4
                },
            ],
        );
        assert_eq!(
            layout
                .compute_wrap_boundaries("किकिa", px(18.), Some(2))
                .as_slice(),
            &boundaries[..1],
        );
        let (left, right) = layout.split_at(3);
        assert_eq!(
            left.runs[0]
                .glyphs
                .iter()
                .map(|glyph| (glyph.index, glyph.position.x))
                .collect::<Vec<_>>(),
            [(0, px(4.))],
        );
        assert_eq!(
            right.runs[0]
                .glyphs
                .iter()
                .map(|glyph| (glyph.index, glyph.position.x))
                .collect::<Vec<_>>(),
            [(0, px(-16.)), (6, px(0.)), (3, px(4.)), (9, px(16.))],
        );
        assert_eq!((left.len, right.len), (3, 10));
        let wrapped = WrappedLineLayout {
            unwrapped_layout: Arc::new(layout),
            wrap_boundaries: boundaries,
            wrap_width: Some(px(18.)),
        };
        assert_eq!(
            wrapped.index_for_position(point(px(0.), px(20.)), px(20.)),
            Ok(6)
        );
        assert_eq!(
            wrapped.closest_index_for_position(point(px(9.), px(20.)), px(20.)),
            Ok(12)
        );
        assert_eq!(
            wrapped.position_for_index(13, px(20.)),
            Some(point(px(8.), px(40.)))
        );
    }

    #[test]
    fn test_wrapped_carets_preserve_relative_precision() {
        let mut layout = make_layout(vec![
            glyph_at(0., 0),
            glyph_at(16_777_216., 1),
            glyph_at(16_777_216., 2),
        ]);
        layout.len = 3;
        layout.width = px(16_777_216.);
        layout.caret_positions = vec![
            (0, 0.),
            (1, 16_777_216.5),
            (2, 16_777_216.75),
            (3, 16_777_217.),
        ];
        let boundaries = layout.compute_wrap_boundaries("abc", px(1.), None);
        assert_eq!(
            boundaries.as_slice(),
            [WrapBoundary {
                run_ix: 0,
                glyph_ix: 1
            }]
        );
        let wrapped = WrappedLineLayout {
            unwrapped_layout: Arc::new(layout),
            wrap_boundaries: boundaries,
            wrap_width: Some(px(1.)),
        };
        assert_eq!(
            wrapped.position_for_index(2, px(20.)),
            Some(point(px(0.25), px(20.)))
        );
        assert_eq!(
            wrapped.position_for_index(3, px(20.)),
            Some(point(px(0.5), px(20.)))
        );
        for (x, floor, closest) in [
            (0., Ok(1), Ok(1)),
            (0.125, Ok(1), Ok(1)),
            (0.2, Ok(1), Ok(2)),
            (0.375, Ok(2), Ok(2)),
            (0.4, Ok(2), Ok(3)),
            (0.5, Err(3), Err(3)),
        ] {
            let position = point(px(x), px(20.));
            assert_eq!(wrapped.index_for_position(position, px(20.)), floor);
            assert_eq!(
                wrapped.closest_index_for_position(position, px(20.)),
                closest
            );
        }
    }

    #[test]
    fn test_force_width_ignores_native_carets() {
        for caret_positions in [Vec::new(), vec![(0, 0.), (6, 20.), (7, 30.), (8, 40.)]] {
            let mut layout = make_layout(vec![
                glyph_at(0., 0),
                glyph_at(10., 3),
                glyph_at(10., 3),
                glyph_at(20., 6),
                glyph_at(30., 7),
            ]);
            layout.len = "กำab".len();
            layout.width = px(40.);
            layout.caret_positions = caret_positions;

            apply_force_width_to_layout(&mut layout, px(8.));

            assert_eq!(glyph_x_positions(&layout), [0., 8., 8., 16., 24.]);
            assert!(layout.caret_positions.is_empty());
            assert_eq!(layout.x_for_index(3), px(8.));
            assert_eq!(layout.x_for_index(6), px(16.));
            assert_eq!(layout.width, px(40.));
        }
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
