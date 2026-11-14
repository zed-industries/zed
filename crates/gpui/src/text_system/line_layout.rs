use crate::{
    Bounds, FontId, GlyphId, Pixels, PlatformTextSystem, Point, SharedString, Size, TextAlign,
    point, px, size,
};
use collections::{FxHashMap, FxHashSet};
use itertools::Itertools;
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

    /// Whether this glyph is a placeholder for an [`InlineObject`] and should not be painted.
    pub is_replaced: bool,
}

/// Dimensions and byte span of an inline object embedded within shaped text.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InlineObject {
    /// The UTF-8 byte range in the text replaced by this inline object.
    pub range: Range<usize>,
    /// The horizontal advance width reserved for this inline object.
    pub width: Pixels,
    /// The height of this inline object above the text baseline.
    pub ascent: Pixels,
    /// The depth of this inline object below the text baseline.
    pub descent: Pixels,
}

/// Layout metrics for a single wrapped row within a [`WrappedLineLayout`].
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedRowLayout {
    /// The UTF-8 byte range covered by this row within the line.
    pub byte_range: Range<usize>,
    /// The horizontal x-range of this row in the unwrapped [`LineLayout`].
    pub unwrapped_x_range: Range<Pixels>,
    /// The vertical offset of the top of this row from the top of the [`WrappedLineLayout`].
    pub y: Pixels,
    /// The total height of this row.
    pub height: Pixels,
    /// The distance from `y` to the text baseline of this row.
    pub baseline: Pixels,
}

/// Resolved bounds and baseline position of a laid-out [`InlineObject`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InlineObjectPlacement {
    /// The bounding box of the inline object (`width` by `ascent + descent`).
    pub bounds: Bounds<Pixels>,
    /// The y-coordinate of the text baseline for the row containing this inline object.
    pub baseline_y: Pixels,
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
                        is_replaced: g.is_replaced,
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

    fn compute_wrap_boundaries(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
        scratch: &mut WrapScratch,
    ) -> SmallVec<[WrapBoundary; 1]> {
        self.compute_wrap_boundaries_with_inline_objects(text, wrap_width, max_lines, &[], scratch)
    }

    fn compute_wrap_boundaries_with_inline_objects(
        &self,
        text: &str,
        wrap_width: Pixels,
        max_lines: Option<usize>,
        inline_objects: &[InlineObject],
        scratch: &mut WrapScratch,
    ) -> SmallVec<[WrapBoundary; 1]> {
        let mut boundaries = SmallVec::new();
        let mut current_segment: Option<Range<usize>> = None;
        let mut last_candidate_ix = None;
        let mut last_candidate_x = px(0.);
        let mut last_boundary = WrapBoundary {
            run_ix: 0,
            glyph_ix: 0,
        };
        let mut last_boundary_x = px(0.);
        // Shaping can combine characters across a line break opportunity, so we ignore those
        // break opportunities that would separate them.
        let WrapScratch {
            glyph_indices,
            break_indices,
            break_text,
        } = scratch;
        let break_source = if inline_objects.is_empty() {
            text
        } else {
            // U+FFFC has UAX #14 class CB (Contingent Break), which allows line breaks immediately
            // before and after the placeholder even when adjacent to word characters or punctuation
            // (such as `($x$,` or `$n$th`). Substituting ASCII letters across inline-object spans
            // matches `LineWrapper::wrap_line` so inline objects attach to surrounding words.
            break_text.clear();
            break_text.reserve(text.len());
            let mut cursor = 0;
            for object in inline_objects {
                break_text.push_str(&text[cursor..object.range.start]);
                break_text.extend(std::iter::repeat_n('a', object.range.len()));
                cursor = object.range.end;
            }
            break_text.push_str(&text[cursor..]);
            break_text.as_str()
        };
        glyph_indices.clear();
        glyph_indices.extend(
            self.runs
                .iter()
                .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.index)),
        );
        break_indices.clear();
        break_indices.extend(
            LineWrapper::break_indices(break_source).filter(|index| glyph_indices.contains(index)),
        );
        let mut glyphs = self
            .runs
            .iter()
            .enumerate()
            .flat_map(move |(run_ix, run)| {
                run.glyphs.iter().enumerate().map(move |(glyph_ix, glyph)| {
                    (
                        WrapBoundary { run_ix, glyph_ix },
                        glyph.index,
                        glyph.position.x,
                    )
                })
            })
            .dedup_by(|(_, previous_index, ..), (_, index, ..)| previous_index == index)
            .peekable();

        while let Some((boundary, index, x)) = glyphs.next() {
            let byte = break_source.as_bytes()[index];
            if byte == b'\n' {
                continue;
            }

            if byte != b' '
                && current_segment
                    .as_ref()
                    .is_none_or(|segment| !segment.contains(&index))
            {
                // Detect transitions between line-break segments rather than matching glyph
                // indices directly, to accommodate RTL text. Workaround for the lack of complete
                // UAX #9 support.
                let segment = break_indices.partition_point(|break_index| *break_index <= index);
                if current_segment.is_some() {
                    last_candidate_ix = Some(boundary);
                    last_candidate_x = x;
                }
                let start = segment
                    .checked_sub(1)
                    .and_then(|previous| break_indices.get(previous))
                    .copied()
                    .unwrap_or(0);
                let end = break_indices
                    .get(segment)
                    .copied()
                    .unwrap_or(break_source.len());
                current_segment = Some(start..end);
            }

            let next_x = glyphs.peek().map_or(self.width, |(_, _, x)| *x);
            let width = next_x - last_boundary_x;

            if width > wrap_width && boundary > last_boundary {
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

    /// Inline objects embedded in this line, if any.
    pub(crate) inline_objects: Option<Arc<[InlineObject]>>,
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
        let height = if self.inline_objects.is_some() {
            self.wrapped_rows(line_height)
                .last()
                .map_or(line_height, |row| row.y + row.height)
        } else {
            line_height * (self.wrap_boundaries.len() + 1)
        };
        Size {
            width: self.width(),
            height,
        }
    }

    /// The inline objects embedded in this layout, in source byte order.
    pub fn inline_objects(&self) -> &[InlineObject] {
        self.inline_objects.as_deref().unwrap_or(&[])
    }

    /// Computes the horizontal and vertical metrics of each wrapped row in this layout.
    pub fn wrapped_rows(&self, line_height: Pixels) -> SmallVec<[WrappedRowLayout; 2]> {
        let default_padding_top =
            (line_height - self.unwrapped_layout.ascent - self.unwrapped_layout.descent) / 2.;
        let default_baseline = default_padding_top + self.unwrapped_layout.ascent;
        let default_below = line_height - default_baseline;
        let inline_objects = self.inline_objects();

        let mut rows = SmallVec::with_capacity(self.wrap_boundaries.len() + 1);
        let mut row_start_index = 0;
        let mut row_start_x = Pixels::ZERO;
        let mut current_y = Pixels::ZERO;
        let mut object_cursor = 0;

        let row_ends = self
            .wrap_boundaries
            .iter()
            .map(|wrap_boundary| {
                let run = &self.unwrapped_layout.runs[wrap_boundary.run_ix];
                let glyph = &run.glyphs[wrap_boundary.glyph_ix];
                (glyph.index, glyph.position.x)
            })
            .chain([(self.len(), self.unwrapped_layout.width)]);

        for (row_end_index, row_end_x) in row_ends {
            let mut above = default_baseline;
            let mut below = default_below;

            if !inline_objects.is_empty() {
                while let Some(object) = inline_objects.get(object_cursor)
                    && object.range.start < row_start_index
                {
                    object_cursor += 1;
                }
                let mut scan = object_cursor;
                while let Some(object) = inline_objects.get(scan)
                    && object.range.start < row_end_index
                {
                    above = above.max(object.ascent);
                    below = below.max(object.descent);
                    scan += 1;
                }
                object_cursor = scan;
            }

            let height = above + below;
            rows.push(WrappedRowLayout {
                byte_range: row_start_index..row_end_index,
                unwrapped_x_range: row_start_x..row_end_x,
                y: current_y,
                height,
                baseline: above,
            });

            row_start_index = row_end_index;
            row_start_x = row_end_x;
            current_y += height;
        }

        rows
    }

    /// Resolves the painted bounds and baseline for the inline object at `object_index`.
    pub fn inline_object_placement(
        &self,
        object_index: usize,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
    ) -> Option<InlineObjectPlacement> {
        let object = self.inline_objects().get(object_index)?;
        let rows = self.wrapped_rows(line_height);
        let row = rows.iter().find(|row| {
            object.range.start >= row.byte_range.start && object.range.start < row.byte_range.end
        })?;

        let row_width = row.unwrapped_x_range.end - row.unwrapped_x_range.start;
        let effective_align_width = align_width
            .or(self.wrap_width)
            .unwrap_or(self.unwrapped_layout.width);
        let row_origin_x = match align {
            TextAlign::Left => origin.x,
            TextAlign::Center => (origin.x * 2.0 + effective_align_width - row_width) / 2.0,
            TextAlign::Right => origin.x + effective_align_width - row_width,
        };
        let x_in_row =
            self.unwrapped_layout.x_for_index(object.range.start) - row.unwrapped_x_range.start;
        let baseline_y = origin.y + row.y + row.baseline;
        let bounds = Bounds {
            origin: point(row_origin_x + x_in_row, baseline_y - object.ascent),
            size: size(object.width, object.ascent + object.descent),
        };
        Some(InlineObjectPlacement { bounds, baseline_y })
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
        let (
            wrapped_line_start_index,
            wrapped_line_start_x,
            wrapped_line_end_index,
            wrapped_line_end_x,
        ) = if self.inline_objects.is_some() {
            if position.y < Pixels::ZERO {
                return Err(0);
            }
            let rows = self.wrapped_rows(line_height);
            let Some(row) = rows.iter().find(|row| position.y < row.y + row.height) else {
                return Err(0);
            };
            (
                row.byte_range.start,
                row.unwrapped_x_range.start,
                row.byte_range.end,
                row.unwrapped_x_range.end,
            )
        } else {
            let wrapped_line_ix = (position.y / line_height) as usize;

            let wrapped_line_start_index;
            let wrapped_line_start_x;
            if wrapped_line_ix > 0 {
                let Some(line_start_boundary) = self.wrap_boundaries.get(wrapped_line_ix - 1)
                else {
                    return Err(0);
                };
                let run = &self.unwrapped_layout.runs[line_start_boundary.run_ix];
                let glyph = &run.glyphs[line_start_boundary.glyph_ix];
                wrapped_line_start_index = glyph.index;
                wrapped_line_start_x = glyph.position.x;
            } else {
                wrapped_line_start_index = 0;
                wrapped_line_start_x = Pixels::ZERO;
            };

            let wrapped_line_end_index;
            let wrapped_line_end_x;
            if wrapped_line_ix < self.wrap_boundaries.len() {
                let next_wrap_boundary_ix = wrapped_line_ix;
                let next_wrap_boundary = self.wrap_boundaries[next_wrap_boundary_ix];
                let run = &self.unwrapped_layout.runs[next_wrap_boundary.run_ix];
                let glyph = &run.glyphs[next_wrap_boundary.glyph_ix];
                wrapped_line_end_index = glyph.index;
                wrapped_line_end_x = glyph.position.x;
            } else {
                wrapped_line_end_index = self.unwrapped_layout.len;
                wrapped_line_end_x = self.unwrapped_layout.width;
            };
            (
                wrapped_line_start_index,
                wrapped_line_start_x,
                wrapped_line_end_index,
                wrapped_line_end_x,
            )
        };

        let mut position_in_unwrapped_line = position;
        position_in_unwrapped_line.x += wrapped_line_start_x;
        if position_in_unwrapped_line.x < wrapped_line_start_x {
            Err(wrapped_line_start_index)
        } else if position_in_unwrapped_line.x >= wrapped_line_end_x {
            Err(wrapped_line_end_index)
        } else {
            if closest {
                Ok(self
                    .unwrapped_layout
                    .closest_index_for_x(position_in_unwrapped_line.x))
            } else {
                // The shaper can place a trailing zero-width wrap boundary glyph slightly past
                // the line's width, so the row can extend past where `index_for_x` has glyphs.
                self.unwrapped_layout
                    .index_for_x(position_in_unwrapped_line.x)
                    .ok_or(wrapped_line_end_index)
            }
        }
    }

    /// Returns the pixel position for the given byte index.
    pub fn position_for_index(&self, index: usize, line_height: Pixels) -> Option<Point<Pixels>> {
        if self.inline_objects.is_some() {
            for row in self.wrapped_rows(line_height) {
                if index < row.byte_range.start {
                    break;
                } else if index > row.byte_range.end {
                    continue;
                } else {
                    let x = self.unwrapped_layout.x_for_index(index) - row.unwrapped_x_range.start;
                    return Some(point(x, row.y));
                }
            }
            return None;
        }

        let mut line_start_ix = 0;
        let line_end_indices = self
            .wrap_boundaries
            .iter()
            .map(|wrap_boundary| {
                let run = &self.unwrapped_layout.runs[wrap_boundary.run_ix];
                let glyph = &run.glyphs[wrap_boundary.glyph_ix];
                glyph.index
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
                let line_start_x = self.unwrapped_layout.x_for_index(line_start_ix);
                let x = self.unwrapped_layout.x_for_index(index) - line_start_x;
                return Some(point(x, line_y));
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
    glyph_indices: FxHashSet<usize>,
    break_indices: Vec<usize>,
    break_text: String,
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
        self.layout_wrapped_line_with_inline_objects(
            text,
            font_size,
            runs,
            wrap_width,
            max_lines,
            &[],
        )
    }

    pub fn layout_wrapped_line_with_inline_objects<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        wrap_width: Option<Pixels>,
        max_lines: Option<usize>,
        inline_objects: &[InlineObject],
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
            inline_objects,
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
            let unwrapped_layout = if inline_objects.is_empty() {
                self.layout_line::<&SharedString>(&text, font_size, runs, None)
            } else {
                self.layout_line_with_inline_objects::<&SharedString>(
                    &text,
                    font_size,
                    runs,
                    inline_objects,
                )
            };
            let wrap_boundaries = match wrap_width {
                Some(wrap_width) if inline_objects.is_empty() => unwrapped_layout
                    .compute_wrap_boundaries(
                        text.as_ref(),
                        wrap_width,
                        max_lines,
                        &mut self.wrap_scratch.borrow_mut(),
                    ),
                Some(wrap_width) => unwrapped_layout.compute_wrap_boundaries_with_inline_objects(
                    text.as_ref(),
                    wrap_width,
                    max_lines,
                    inline_objects,
                    &mut self.wrap_scratch.borrow_mut(),
                ),
                None => SmallVec::new(),
            };
            let inline_objects_arc =
                (!inline_objects.is_empty()).then(|| Arc::<[InlineObject]>::from(inline_objects));
            let layout = Arc::new(WrappedLineLayout {
                unwrapped_layout,
                wrap_boundaries,
                wrap_width,
                inline_objects: inline_objects_arc.clone(),
            });
            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width,
                force_width: None,
                inline_objects: inline_objects_arc,
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

    pub fn layout_line_with_inline_objects<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        inline_objects: &[InlineObject],
    ) -> Arc<LineLayout>
    where
        Text: AsRef<str>,
        SharedString: From<Text>,
    {
        self.layout_line_internal(text, font_size, runs, None, inline_objects)
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
        self.layout_line_internal(text, font_size, runs, force_width, &[])
    }

    fn layout_line_internal<Text>(
        &self,
        text: Text,
        font_size: Pixels,
        runs: &[FontRun],
        force_width: Option<Pixels>,
        inline_objects: &[InlineObject],
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
            inline_objects,
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
            let inline_objects_arc =
                (!inline_objects.is_empty()).then(|| Arc::<[InlineObject]>::from(inline_objects));
            let mut layout = if inline_objects.is_empty() {
                self.platform_text_system
                    .layout_line(&text, font_size, runs)
            } else {
                shape_line_replacing_inline_objects(
                    &*self.platform_text_system,
                    &text,
                    font_size,
                    runs,
                    inline_objects,
                )
            };

            if let Some(force_width) = force_width {
                apply_force_width_to_layout(&mut layout, force_width);
            }

            let key = Arc::new(CacheKey {
                text,
                font_size,
                runs: SmallVec::from(runs),
                wrap_width: None,
                force_width,
                inline_objects: inline_objects_arc,
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
    inline_objects: Option<Arc<[InlineObject]>>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct CacheKeyRef<'a> {
    text: &'a str,
    font_size: Pixels,
    runs: &'a [FontRun],
    wrap_width: Option<Pixels>,
    force_width: Option<Pixels>,
    inline_objects: &'a [InlineObject],
}

impl Hash for CacheKeyRef<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.font_size.hash(state);
        self.runs.hash(state);
        self.wrap_width.hash(state);
        self.force_width.hash(state);
        if !self.inline_objects.is_empty() {
            self.inline_objects.hash(state);
        }
    }
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
            inline_objects: self.inline_objects.as_deref().unwrap_or(&[]),
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

fn orig_to_shaping_offset(orig_index: usize, inline_objects: &[InlineObject]) -> usize {
    let mut removed_bytes = 0;
    for object in inline_objects {
        if orig_index <= object.range.start {
            break;
        }
        if orig_index < object.range.end {
            return (object.range.start - removed_bytes) + 1;
        }
        removed_bytes += object.range.len() - 1;
    }
    orig_index - removed_bytes
}

fn shaping_to_orig_offset(shaping_index: usize, inline_objects: &[InlineObject]) -> usize {
    let mut added_bytes = 0;
    for object in inline_objects {
        let object_shaping_index = object.range.start - added_bytes;
        if shaping_index <= object_shaping_index {
            return shaping_index + added_bytes;
        }
        added_bytes += object.range.len() - 1;
    }
    shaping_index + added_bytes
}

fn shape_line_replacing_inline_objects(
    platform_text_system: &dyn PlatformTextSystem,
    text: &str,
    font_size: Pixels,
    runs: &[FontRun],
    inline_objects: &[InlineObject],
) -> LineLayout {
    let mut shaping_text = String::with_capacity(text.len());
    let mut replacement_slots: SmallVec<[(usize, usize, Pixels); 4]> =
        SmallVec::with_capacity(inline_objects.len());
    let mut cursor = 0;
    for object in inline_objects {
        debug_assert!(object.range.start >= cursor);
        debug_assert!(object.range.start < object.range.end);
        debug_assert!(object.range.end <= text.len());
        debug_assert!(text.is_char_boundary(object.range.start));
        debug_assert!(text.is_char_boundary(object.range.end));

        shaping_text.push_str(&text[cursor..object.range.start]);
        let shaping_index = shaping_text.len();
        // Substitute an ASCII 'H' rather than U+FFFC during platform shaping so shapers never
        // trigger missing-glyph reporting, system fallback font scans, ligatures, or diagonal
        // kerning pairs with neighboring punctuation.
        shaping_text.push('H');
        replacement_slots.push((shaping_index, object.range.start, object.width));
        cursor = object.range.end;
    }
    shaping_text.push_str(&text[cursor..]);

    let mut shaping_runs: SmallVec<[FontRun; 4]> = SmallVec::with_capacity(runs.len());
    let mut orig_offset = 0;
    for run in runs {
        let orig_end = orig_offset + run.len;
        let shaping_start = orig_to_shaping_offset(orig_offset, inline_objects);
        let shaping_end = orig_to_shaping_offset(orig_end, inline_objects);
        if shaping_end > shaping_start {
            shaping_runs.push(FontRun {
                len: shaping_end - shaping_start,
                font_id: run.font_id,
            });
        }
        orig_offset = orig_end;
    }

    let mut layout = platform_text_system.layout_line(&shaping_text, font_size, &shaping_runs);
    let old_layout_width = layout.width;
    let mut x_shift = Pixels::ZERO;
    let mut applied_slots = SmallVec::<[bool; 4]>::from_elem(false, replacement_slots.len());

    for run_ix in 0..layout.runs.len() {
        for glyph_ix in 0..layout.runs[run_ix].glyphs.len() {
            let shaped_x = layout.runs[run_ix].glyphs[glyph_ix].position.x;
            let next_shaped_x = layout.runs[run_ix]
                .glyphs
                .get(glyph_ix + 1)
                .map(|glyph| glyph.position.x)
                .or_else(|| {
                    layout.runs[run_ix + 1..]
                        .iter()
                        .find_map(|run| run.glyphs.first().map(|glyph| glyph.position.x))
                })
                .unwrap_or(old_layout_width);

            let glyph = &mut layout.runs[run_ix].glyphs[glyph_ix];
            glyph.position.x = shaped_x + x_shift;

            if let Ok(slot_ix) = replacement_slots
                .binary_search_by_key(&glyph.index, |&(shaping_ix, _, _)| shaping_ix)
            {
                let (_, orig_start, target_width) = replacement_slots[slot_ix];
                let shaped_advance = (next_shaped_x - shaped_x).max(Pixels::ZERO);
                glyph.position.y = Pixels::ZERO;
                glyph.index = orig_start;
                glyph.is_replaced = true;
                if !applied_slots[slot_ix] {
                    applied_slots[slot_ix] = true;
                    x_shift += target_width - shaped_advance;
                } else {
                    x_shift -= shaped_advance;
                }
            } else {
                glyph.index = shaping_to_orig_offset(glyph.index, inline_objects);
            }
        }
    }

    layout.width = (old_layout_width + x_shift).max(Pixels::ZERO);
    layout.len = text.len();
    layout
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
            is_replaced: false,
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
            layout
                .compute_wrap_boundaries("你好", px(15.), None, &mut WrapScratch::default())
                .as_slice(),
            &[WrapBoundary {
                run_ix: 0,
                glyph_ix: 2
            }],
        );

        let mut layout = make_layout(vec![glyph_at(0., 0), glyph_at(10., 0)]);
        layout.width = px(20.);
        layout.len = 1;
        assert!(
            layout
                .compute_wrap_boundaries("a", px(10.), None, &mut WrapScratch::default())
                .is_empty()
        );
    }

    #[test]
    fn test_wrap_right_to_left_text() {
        for (text, indices, expected_boundaries) in [
            ("אבג דהו", vec![11, 9, 7, 6, 4, 2, 0], vec![4]),
            ("a אבג b", vec![0, 1, 6, 4, 2, 8, 9], vec![2, 6]),
        ] {
            let mut layout = make_layout(
                indices
                    .iter()
                    .enumerate()
                    .map(|(position, index)| glyph_at(position as f32 * 10., *index))
                    .collect(),
            );
            layout.width = px(indices.len() as f32 * 10.);
            layout.len = text.len();
            assert_eq!(
                layout.compute_wrap_boundaries(text, px(40.), None, &mut WrapScratch::default()),
                expected_boundaries
                    .into_iter()
                    .map(|glyph_ix| WrapBoundary {
                        run_ix: 0,
                        glyph_ix
                    })
                    .collect::<SmallVec<[WrapBoundary; 1]>>(),
                "{text:?}",
            );
        }
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
            layout
                .compute_wrap_boundaries(text, px(60.), None, &mut WrapScratch::default())
                .as_slice(),
            &[WrapBoundary {
                run_ix: 0,
                glyph_ix: 2
            }],
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

    #[test]
    fn test_inline_object_shaping_and_x_mapping() {
        let text_system =
            WindowTextSystem::new(Arc::new(TextSystem::new(Arc::new(NoopTextSystem::new()))));
        // "a\u{fffc}bc\u{fffc}d":
        // 'a' at 0..1 (advance 6px at font_size 10px in NoopTextSystem)
        // '\u{fffc}' at 1..4 (replaced with width 40px)
        // 'b' at 4..5 (advance 6px)
        // 'c' at 5..6 (advance 6px)
        // '\u{fffc}' at 6..9 (replaced with width 25px)
        // 'd' at 9..10 (advance 6px)
        let text = SharedString::from("a\u{fffc}bc\u{fffc}d");
        let runs = [TextRun {
            len: text.len(),
            font: font(".ZedMono"),
            ..Default::default()
        }];
        let inline_objects = [
            InlineObject {
                range: 1..4,
                width: px(40.),
                ascent: px(18.),
                descent: px(6.),
            },
            InlineObject {
                range: 6..9,
                width: px(25.),
                ascent: px(10.),
                descent: px(2.),
            },
        ];

        let shaped = text_system.shape_line_with_inline_objects(
            text.clone(),
            px(10.),
            &runs,
            &inline_objects,
        );

        {
            let state = text_system.line_layout_cache.state.borrow();
            assert_eq!(state.current_frame.lines.len(), 1);
        }
        let plain_shaped = text_system.shape_line(text.clone(), px(10.), &runs, None);
        assert_ne!(shaped.width, plain_shaped.width);
        {
            let state = text_system.line_layout_cache.state.borrow();
            assert_eq!(state.current_frame.lines.len(), 2);
        }

        assert_eq!(shaped.len, text.len());
        assert_eq!(shaped.width, px(89.));
        let glyphs = &shaped.runs[0].glyphs;
        assert_eq!(glyphs.len(), 6);
        assert_eq!(
            glyphs
                .iter()
                .map(|glyph| (glyph.index, f32::from(glyph.position.x), glyph.is_replaced))
                .collect::<Vec<_>>(),
            vec![
                (0, 0., false),
                (1, 6., true),
                (4, 46., false),
                (5, 52., false),
                (6, 58., true),
                (9, 83., false),
            ],
        );

        assert_eq!(shaped.x_for_index(0), px(0.));
        assert_eq!(shaped.x_for_index(1), px(6.));
        assert_eq!(shaped.x_for_index(4), px(46.));
        assert_eq!(shaped.x_for_index(6), px(58.));
        assert_eq!(shaped.x_for_index(9), px(83.));
        assert_eq!(shaped.x_for_index(10), px(89.));

        assert_eq!(shaped.index_for_x(px(20.)), Some(1));
        assert_eq!(shaped.closest_index_for_x(px(25.)), 1);
        assert_eq!(shaped.closest_index_for_x(px(27.)), 4);
    }

    #[test]
    fn test_wrap_keeps_inline_objects_attached_to_adjacent_punctuation_and_words() {
        // Each ASCII char is 6px wide at 10px in NoopTextSystem.
        // "See " = 24px, "\u{fffc}" = 30px, "," = 6px -> "See \u{fffc}," = 60px.
        // With wrap_width = 56px, "," would overflow if broken after '\u{fffc}',
        // so UAX #14 must wrap BEFORE "\u{fffc}," at byte 4.
        let text_system =
            WindowTextSystem::new(Arc::new(TextSystem::new(Arc::new(NoopTextSystem::new()))));
        let text = SharedString::from("See \u{fffc}, and \u{fffc}th");
        let runs = [TextRun {
            len: text.len(),
            font: font(".ZedMono"),
            ..Default::default()
        }];
        let first_start = "See ".len();
        let first_end = first_start + '\u{fffc}'.len_utf8();
        let second_start = "See \u{fffc}, and ".len();
        let second_end = second_start + '\u{fffc}'.len_utf8();
        let inline_objects = [
            InlineObject {
                range: first_start..first_end,
                width: px(30.),
                ascent: px(12.),
                descent: px(4.),
            },
            InlineObject {
                range: second_start..second_end,
                width: px(30.),
                ascent: px(12.),
                descent: px(4.),
            },
        ];

        let wrapped = text_system
            .shape_text_with_inline_objects(
                text,
                px(10.),
                &runs,
                Some(px(56.)),
                None,
                &inline_objects,
            )
            .unwrap();
        assert_eq!(wrapped.len(), 1);
        let boundary_indices: Vec<usize> = wrapped[0]
            .wrap_boundaries
            .iter()
            .map(|boundary| {
                wrapped[0].unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
            })
            .collect();
        assert_eq!(
            boundary_indices,
            vec![first_start, "See \u{fffc}, ".len(), second_start]
        );
    }

    #[test]
    fn test_wrapped_rows_and_inline_object_placement() {
        // Row 0: "ab " (bytes 0..3, x 0..30), plain text -> height 20px, baseline 14px
        // Row 1: "c\u{fffc}d" (bytes 3..8, x 30..90), object at 4..7 (width 40, ascent 24, descent 11)
        //        -> above = max(14, 24) = 24, below = max(6, 11) = 11, height = 35px
        let unwrapped_layout = Arc::new(LineLayout {
            font_size: px(16.),
            width: px(90.),
            ascent: px(12.),
            descent: px(4.),
            runs: vec![ShapedRun {
                font_id: FontId(0),
                glyphs: vec![
                    glyph_at(0., 0),
                    glyph_at(10., 1),
                    glyph_at(20., 2),
                    glyph_at(30., 3),
                    ShapedGlyph {
                        id: GlyphId(0),
                        position: point(px(40.), px(0.)),
                        index: 4,
                        is_emoji: false,
                        is_replaced: true,
                    },
                    glyph_at(80., 7),
                ],
            }],
            len: 8,
        });
        let inline_objects: Arc<[InlineObject]> = Arc::from([InlineObject {
            range: 4..7,
            width: px(40.),
            ascent: px(24.),
            descent: px(11.),
        }]);
        let wrapped = WrappedLineLayout {
            unwrapped_layout,
            wrap_boundaries: SmallVec::from_slice(&[WrapBoundary {
                run_ix: 0,
                glyph_ix: 3,
            }]),
            wrap_width: Some(px(100.)),
            inline_objects: Some(inline_objects),
        };

        let line_height = px(20.);
        let rows = wrapped.wrapped_rows(line_height);
        assert_eq!(
            rows.as_slice(),
            &[
                WrappedRowLayout {
                    byte_range: 0..3,
                    unwrapped_x_range: px(0.)..px(30.),
                    y: px(0.),
                    height: px(20.),
                    baseline: px(14.),
                },
                WrappedRowLayout {
                    byte_range: 3..8,
                    unwrapped_x_range: px(30.)..px(90.),
                    y: px(20.),
                    height: px(35.),
                    baseline: px(24.),
                },
            ],
        );
        assert_eq!(wrapped.size(line_height), size(px(90.), px(55.)));
        assert_eq!(
            wrapped.position_for_index(1, line_height),
            Some(point(px(10.), px(0.)))
        );
        assert_eq!(
            wrapped.position_for_index(4, line_height),
            Some(point(px(10.), px(20.)))
        );
        assert_eq!(
            wrapped.position_for_index(7, line_height),
            Some(point(px(50.), px(20.)))
        );
        assert_eq!(
            wrapped.index_for_position(point(px(12.), px(10.)), line_height),
            Ok(1)
        );
        assert_eq!(
            wrapped.index_for_position(point(px(25.), px(45.)), line_height),
            Ok(4)
        );
        assert_eq!(
            wrapped.index_for_position(point(px(52.), px(45.)), line_height),
            Ok(7)
        );

        let origin = point(px(10.), px(5.));
        assert_eq!(
            wrapped.inline_object_placement(
                0,
                origin,
                line_height,
                crate::TextAlign::Left,
                Some(px(100.))
            ),
            Some(InlineObjectPlacement {
                bounds: Bounds {
                    origin: point(px(20.), px(25.)),
                    size: size(px(40.), px(35.)),
                },
                baseline_y: px(49.),
            }),
        );
        assert_eq!(
            wrapped.inline_object_placement(
                0,
                origin,
                line_height,
                crate::TextAlign::Center,
                Some(px(100.))
            ),
            Some(InlineObjectPlacement {
                bounds: Bounds {
                    origin: point(px(40.), px(25.)),
                    size: size(px(40.), px(35.)),
                },
                baseline_y: px(49.),
            }),
        );
        assert_eq!(
            wrapped.inline_object_placement(
                0,
                origin,
                line_height,
                crate::TextAlign::Right,
                Some(px(100.))
            ),
            Some(InlineObjectPlacement {
                bounds: Bounds {
                    origin: point(px(60.), px(25.)),
                    size: size(px(40.), px(35.)),
                },
                baseline_y: px(49.),
            }),
        );
    }
}
