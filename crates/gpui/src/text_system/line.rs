use crate::{
    App, Bounds, DevicePixels, FontId, Half, Hsla, LineLayout, Pixels, Point, RenderGlyphParams,
    Result, ScaledPixels, SharedString, StrikethroughStyle, TextAlign, TextRun, UnderlineStyle,
    Window, WrapBoundary, WrappedLineLayout, black, fill, point, px, size, underline_y_offset,
};
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;
use std::{ops::Range, sync::Arc};

/// Pre-computed glyph data for efficient painting without per-glyph cache lookups.
///
/// This is produced by `ShapedLine::compute_glyph_raster_data` during prepaint
/// and consumed by `ShapedLine::paint_with_raster_data` during paint.
#[derive(Clone, Debug)]
pub struct GlyphRasterData {
    /// The raster bounds for each glyph, in paint order.
    pub bounds: Vec<Bounds<DevicePixels>>,
    /// The render params for each glyph (needed for sprite atlas lookup).
    pub params: Vec<RenderGlyphParams>,
}

/// Set the text decoration for a run of text.
#[derive(Debug, Clone)]
pub struct DecorationRun {
    /// The length of the run in utf-8 bytes.
    pub len: u32,

    /// The color for this run
    pub color: Hsla,

    /// The background color for this run
    pub background_color: Option<Hsla>,

    /// The underline style for this run
    pub underline: Option<UnderlineStyle>,

    /// The strikethrough style for this run
    pub strikethrough: Option<StrikethroughStyle>,
}

/// An underline's coverage and resolved wave phase.
#[derive(Clone, Copy, Debug)]
pub struct TextUnderline {
    /// The start of the covered span.
    pub origin: Point<Pixels>,
    /// The width of the covered span.
    pub width: Pixels,
    /// The stroke style.
    pub style: UnderlineStyle,
    phase: f64,
}

impl TextUnderline {
    /// Creates an underline whose wave starts at its origin.
    pub fn new(origin: Point<Pixels>, width: Pixels, style: UnderlineStyle) -> Self {
        Self {
            origin,
            width,
            style,
            phase: 0.,
        }
    }

    /// Returns the snapped coverage bounds.
    pub fn bounds(&self, window: &Window) -> Bounds<ScaledPixels> {
        window.underline_bounds(self.origin, self.width, &self.style)
    }

    /// Paints the covered span.
    pub fn paint(&self, window: &mut Window) {
        self.paint_with_exclusions(&[], window);
    }

    /// Paints the covered span excluding absolute device-space intervals.
    pub fn paint_with_exclusions(&self, exclusions: &[Range<ScaledPixels>], window: &mut Window) {
        if self.phase == 0. {
            window.paint_underline_with_exclusions(
                self.origin,
                self.width,
                &self.style,
                exclusions,
            );
            return;
        }
        let bounds = self.bounds(window);
        let scale = f64::from(window.scale_factor());
        let mut origin = self.origin;
        origin.x = px(((f64::from(bounds.left().0) - self.phase) / scale) as f32);
        let width = px(((f64::from(bounds.size.width.0) + self.phase) / scale) as f32);
        let extended = window.underline_bounds(origin, width, &self.style);
        let mut exclusions = exclusions.iter().cloned().collect::<SmallVec<[_; 4]>>();
        exclusions.extend([
            extended.left()..bounds.left(),
            bounds.right()..extended.right(),
        ]);
        window.paint_underline_with_exclusions(origin, width, &self.style, &exclusions);
    }

    fn with_anchor(mut self, anchor: f64, reference: f64, screen: f64, window: &Window) -> Self {
        let bounds = self.bounds(window);
        let period = f64::from(bounds.size.height.0) * 3.;
        if self.style.wavy && period > 0. && period.is_finite() {
            let scale = f64::from(window.scale_factor());
            let remainder = |value: f64| {
                let product = value * scale;
                (product % period + value.mul_add(scale, -product) % period) % period
            };
            let anchor_remainder =
                (remainder(anchor) - remainder(reference) + remainder(screen)).rem_euclid(period);
            let snapped_anchor = if anchor - reference + screen < 0. {
                (anchor_remainder + 0.5).floor()
            } else {
                (anchor_remainder - 0.5).ceil()
            };
            self.phase = (f64::from(bounds.left().0) - snapped_anchor).rem_euclid(period);
        }
        self
    }
}

/// A line of text that has been shaped and decorated.
#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct ShapedLine {
    #[deref]
    #[deref_mut]
    pub(crate) layout: Arc<LineLayout>,
    /// The text that was shaped for this line.
    pub text: SharedString,
    pub(crate) decoration_runs: SmallVec<[DecorationRun; 32]>,
    retained_decorations: Option<RetainedDecorations>,
}

#[derive(Clone, Debug)]
struct RetainedDecorations {
    offset: usize,
    origin_x: f64,
    offset_x: f64,
    underlines: Arc<[(Range<usize>, f64)]>,
}

impl RetainedDecorations {
    fn slice(&self, offset: usize, x: f64) -> Self {
        Self {
            offset: self.offset + offset,
            offset_x: self.offset_x + x,
            ..self.clone()
        }
    }
}

impl ShapedLine {
    /// Creates a line from an already shaped layout, decorated with the given runs.
    pub fn new(layout: Arc<LineLayout>, text: SharedString, runs: &[TextRun]) -> Self {
        let mut decoration_runs = SmallVec::<[DecorationRun; 32]>::new();
        for run in runs {
            if let Some(last_run) = decoration_runs.last_mut()
                && last_run.color == run.color
                && last_run.underline == run.underline
                && last_run.strikethrough == run.strikethrough
                && last_run.background_color == run.background_color
            {
                last_run.len += run.len as u32;
                continue;
            }
            decoration_runs.push(DecorationRun {
                len: run.len as u32,
                color: run.color,
                background_color: run.background_color,
                underline: run.underline,
                strikethrough: run.strikethrough,
            });
        }
        Self {
            layout,
            text,
            decoration_runs,
            retained_decorations: None,
        }
    }

    /// Retains row-space underline origins for byte ranges in this slice.
    pub fn with_underline_origins(
        mut self,
        origin_x: f64,
        underlines: Vec<(Range<usize>, f64)>,
    ) -> Self {
        self.retained_decorations = (!underlines.is_empty()).then(|| RetainedDecorations {
            offset: 0,
            origin_x,
            offset_x: 0.,
            underlines: Arc::from(underlines),
        });
        self
    }

    /// Returns a forward-only cursor for this shaped line.
    pub fn cursor(&self) -> ShapedLineCursor<'_> {
        assert_eq!(
            self.len(),
            self.text.len(),
            "cannot split a shaped line with an adjusted length"
        );
        let byte_ordered = self
            .layout
            .runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.index))
            .is_sorted();
        ShapedLineCursor {
            line: self,
            unordered_remainder: (!byte_ordered).then(|| self.clone()),
            byte_index: 0,
            run_index: 0,
            glyph_index: 0,
            caret_index: 0,
            decoration_index: 0,
            decoration_offset: 0,
            x_offset: 0.,
        }
    }

    /// The length of the line in utf-8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.layout.len
    }

    /// The width of the shaped line in pixels.
    ///
    /// This is the glyph advance width computed by the text shaping system and is useful for
    /// incrementally advancing a "pen" when painting multiple fragments on the same row.
    pub fn width(&self) -> Pixels {
        self.layout.width
    }

    /// Override the len, useful if you're rendering text a
    /// as text b (e.g. rendering invisibles).
    pub fn with_len(mut self, len: usize) -> Self {
        let layout = self.layout.as_ref();
        let mut caret_positions = layout.slice_caret_positions(0..len, 0.);
        if let Some((_, position)) = caret_positions.last_mut() {
            *position = layout.caret_x_for_index(layout.len);
        }
        self.layout = Arc::new(LineLayout {
            font_size: layout.font_size,
            width: layout.width,
            ascent: layout.ascent,
            descent: layout.descent,
            runs: layout.runs.clone(),
            len,
            caret_positions,
        });
        self
    }

    /// Paint the line of text to the window.
    pub fn paint(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        self.paint_with_underline_handler(
            origin,
            line_height,
            align,
            align_width,
            window,
            cx,
            |_, underline, window| underline.paint(window),
        )
    }

    /// Paint the line with a handler for each underline.
    pub fn paint_with_underline_handler(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
        mut paint_underline: impl FnMut(Range<usize>, TextUnderline, &mut Window),
    ) -> Result<()> {
        paint_line(
            origin,
            &self.layout,
            line_height,
            align,
            align_width,
            &self.decoration_runs,
            &[],
            self.retained_decorations.as_ref(),
            window,
            cx,
            &mut paint_underline,
        )
    }

    /// Paint the background of the line to the window.
    pub fn paint_background(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        paint_line_background(
            origin,
            &self.layout,
            line_height,
            align,
            align_width,
            &self.decoration_runs,
            &[],
            window,
            cx,
        )?;

        Ok(())
    }

    /// Split this shaped line at a byte index, returning `(prefix, suffix)`.
    ///
    /// - `prefix` contains glyphs for bytes `[0, byte_index)` with original positions.
    ///   Its width equals the x-advance up to the split point.
    /// - Decoration runs are partitioned at the boundary; a run that straddles it is
    ///   split into two with adjusted lengths.
    /// - `font_size`, `ascent`, and `descent` are copied to both halves.
    pub fn split_at(&self, byte_index: usize) -> (ShapedLine, ShapedLine) {
        let (left_layout, right_layout) = self.layout.split_at(byte_index);

        // Partition decoration runs. A run straddling the boundary is split into two.
        let mut left_decorations = SmallVec::new();
        let mut right_decorations = SmallVec::new();
        let mut decoration_offset = 0u32;
        let split_point = byte_index as u32;

        for decoration in &self.decoration_runs {
            let run_end = decoration_offset + decoration.len;

            if run_end <= split_point {
                left_decorations.push(decoration.clone());
            } else if decoration_offset >= split_point {
                right_decorations.push(decoration.clone());
            } else {
                let left_len = split_point - decoration_offset;
                let right_len = run_end - split_point;
                left_decorations.push(DecorationRun {
                    len: left_len,
                    color: decoration.color,
                    background_color: decoration.background_color,
                    underline: decoration.underline,
                    strikethrough: decoration.strikethrough,
                });
                right_decorations.push(DecorationRun {
                    len: right_len,
                    color: decoration.color,
                    background_color: decoration.background_color,
                    underline: decoration.underline,
                    strikethrough: decoration.strikethrough,
                });
            }

            decoration_offset = run_end;
        }

        // Split text
        let left_text = if byte_index == self.text.len() {
            self.text.clone()
        } else {
            SharedString::new(&self.text[..byte_index])
        };
        let right_text = if byte_index == 0 {
            self.text.clone()
        } else {
            SharedString::new(&self.text[byte_index..])
        };

        let left = ShapedLine {
            layout: Arc::new(left_layout),
            text: left_text,
            decoration_runs: left_decorations,
            retained_decorations: self.retained_decorations.clone(),
        };

        let right = ShapedLine {
            layout: Arc::new(right_layout),
            text: right_text,
            decoration_runs: right_decorations,
            retained_decorations: self.retained_decorations.as_ref().map(|decorations| {
                decorations.slice(byte_index, self.layout.caret_x_for_index(byte_index))
            }),
        };

        (left, right)
    }
}

/// Incrementally splits a [`ShapedLine`] at increasing UTF-8 byte boundaries.
///
/// Each piece preserves the original glyphs and decorations, rebased as in
/// [`ShapedLine::split_at`]. Byte-ordered glyphs are advanced in linear time,
/// copying each glyph and byte at most once; visually reordered glyphs fall
/// back to the existing split operation.
pub struct ShapedLineCursor<'a> {
    line: &'a ShapedLine,
    /// Bidirectional shaping can put glyphs out of byte order.
    unordered_remainder: Option<ShapedLine>,
    byte_index: usize,
    run_index: usize,
    glyph_index: usize,
    caret_index: usize,
    decoration_index: usize,
    decoration_offset: u32,
    x_offset: f64,
}

impl<'a> ShapedLineCursor<'a> {
    /// Takes the bytes since the previous boundary.
    ///
    /// Panics if the boundary precedes the previous one, exceeds the line's
    /// length, or falls inside a UTF-8 character.
    pub fn take_until(&mut self, byte_index: usize) -> ShapedLine {
        assert!(
            byte_index >= self.byte_index,
            "split boundary moved backwards"
        );
        assert!(
            byte_index <= self.line.len(),
            "split boundary exceeds line length"
        );
        assert!(
            self.line.text.is_char_boundary(byte_index),
            "split boundary is not a UTF-8 character boundary"
        );
        let previous_index = self.byte_index;
        let previous_x = self.x_offset;
        if let Some(remainder) = &mut self.unordered_remainder {
            let (piece, rest) = remainder.split_at(byte_index - previous_index);
            *remainder = rest;
            self.byte_index = byte_index;
            self.x_offset = self.line.layout.caret_x_for_index(byte_index);
            return piece;
        }
        let mut runs = Vec::new();
        let mut next_x = f64::from(self.line.layout.width);
        while let Some(run) = self.line.layout.runs.get(self.run_index) {
            let start = self.glyph_index;
            while let Some(glyph) = run.glyphs.get(self.glyph_index) {
                if glyph.index >= byte_index {
                    break;
                }
                self.glyph_index += 1;
            }
            let end = self.glyph_index;
            if start < end {
                runs.push(crate::ShapedRun {
                    font_id: run.font_id,
                    glyphs: run.glyphs[start..end]
                        .iter()
                        .map(|glyph| crate::ShapedGlyph {
                            id: glyph.id,
                            position: point(
                                Pixels::from(f64::from(glyph.position.x) - previous_x),
                                glyph.position.y,
                            ),
                            index: glyph.index - previous_index,
                            is_emoji: glyph.is_emoji,
                        })
                        .collect(),
                });
            }
            if let Some(glyph) = run.glyphs.get(self.glyph_index) {
                next_x = f64::from(glyph.position.x);
                break;
            }
            self.run_index += 1;
            self.glyph_index = 0;
        }
        let previous_caret_index = self.caret_index;
        if !self.line.layout.caret_positions.is_empty() {
            while self
                .line
                .layout
                .caret_positions
                .get(self.caret_index)
                .is_some_and(|(index, _)| *index < byte_index)
            {
                self.caret_index += 1;
            }
            next_x = self
                .line
                .layout
                .caret_positions
                .get(self.caret_index)
                .map_or(f64::from(self.line.layout.width), |(_, position)| *position);
        }
        let mut caret_positions = Vec::new();
        if previous_index < byte_index
            && !self.line.layout.caret_positions.is_empty()
            && self
                .line
                .layout
                .caret_positions
                .get(previous_caret_index)
                .is_none_or(|(index, _)| *index != previous_index)
        {
            caret_positions.push((0, 0.));
        }
        caret_positions.extend(
            self.line.layout.caret_positions[previous_caret_index..self.caret_index]
                .iter()
                .map(|(index, position)| (index - previous_index, position - previous_x)),
        );
        if previous_index < byte_index && !self.line.layout.caret_positions.is_empty() {
            caret_positions.push((byte_index - previous_index, next_x - previous_x));
        }
        let mut decorations = SmallVec::new();
        while let Some(decoration) = self.line.decoration_runs.get(self.decoration_index)
            && (self.decoration_offset < byte_index as u32
                || (decoration.len == 0 && self.decoration_offset == byte_index as u32))
        {
            let end = self.decoration_offset + decoration.len;
            let start = self.decoration_offset.max(previous_index as u32);
            let len = end.min(byte_index as u32) - start;
            if len > 0 || decoration.len == 0 {
                decorations.push(DecorationRun {
                    len,
                    color: decoration.color,
                    background_color: decoration.background_color,
                    underline: decoration.underline,
                    strikethrough: decoration.strikethrough,
                });
            }
            if end <= byte_index as u32 {
                self.decoration_index += 1;
                self.decoration_offset = end;
            } else {
                break;
            }
        }
        self.byte_index = byte_index;
        self.x_offset = next_x;
        ShapedLine {
            layout: Arc::new(LineLayout {
                font_size: self.line.layout.font_size,
                width: Pixels::from(next_x - previous_x),
                ascent: self.line.layout.ascent,
                descent: self.line.layout.descent,
                runs,
                len: byte_index - previous_index,
                caret_positions,
            }),
            text: SharedString::new(&self.line.text[previous_index..byte_index]),
            decoration_runs: decorations,
            retained_decorations: self
                .line
                .retained_decorations
                .as_ref()
                .map(|decorations| decorations.slice(previous_index, previous_x)),
        }
    }

    /// Returns the original line's x position at the current boundary.
    pub fn x_offset(&self) -> Pixels {
        Pixels::from(self.x_offset)
    }
}

impl LineLayout {
    /// Paint this layout to the window, using the given decoration runs to color
    /// glyphs and draw underlines and strikethroughs.
    ///
    /// This is a lower-level alternative to [`ShapedLine::paint`] for callers that
    /// hold a bare layout and track decorations themselves.
    pub fn paint(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        decoration_runs: &[DecorationRun],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        paint_line(
            origin,
            self,
            line_height,
            align,
            align_width,
            decoration_runs,
            &[],
            None,
            window,
            cx,
            &mut |_, underline, window| underline.paint(window),
        )
    }

    /// Paint the background of this layout to the window, using the given
    /// decoration runs to determine background colors.
    ///
    /// This is a lower-level alternative to [`ShapedLine::paint_background`] for
    /// callers that hold a bare layout and track decorations themselves.
    pub fn paint_background(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        align_width: Option<Pixels>,
        decoration_runs: &[DecorationRun],
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        paint_line_background(
            origin,
            self,
            line_height,
            align,
            align_width,
            decoration_runs,
            &[],
            window,
            cx,
        )
    }
}

/// A line of text that has been shaped, decorated, and wrapped by the text layout system.
#[derive(Default, Debug, Deref, DerefMut)]
pub struct WrappedLine {
    #[deref]
    #[deref_mut]
    pub(crate) layout: Arc<WrappedLineLayout>,
    /// The text that was shaped for this line.
    pub text: SharedString,
    pub(crate) decoration_runs: Vec<DecorationRun>,
}

impl WrappedLine {
    /// The length of the underlying, unwrapped layout, in utf-8 bytes.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.layout.len()
    }

    /// Paint this line of text to the window.
    pub fn paint(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        let align_width = match bounds {
            Some(bounds) => Some(bounds.size.width),
            None => self.layout.wrap_width,
        };

        paint_line(
            origin,
            &self.layout.unwrapped_layout,
            line_height,
            align,
            align_width,
            &self.decoration_runs,
            &self.wrap_boundaries,
            None,
            window,
            cx,
            &mut |_, underline, window| underline.paint(window),
        )?;

        Ok(())
    }

    /// Paint the background of line of text to the window.
    pub fn paint_background(
        &self,
        origin: Point<Pixels>,
        line_height: Pixels,
        align: TextAlign,
        bounds: Option<Bounds<Pixels>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<()> {
        let align_width = match bounds {
            Some(bounds) => Some(bounds.size.width),
            None => self.layout.wrap_width,
        };

        paint_line_background(
            origin,
            &self.layout.unwrapped_layout,
            line_height,
            align,
            align_width,
            &self.decoration_runs,
            &self.wrap_boundaries,
            window,
            cx,
        )?;

        Ok(())
    }
}

fn paint_line(
    origin: Point<Pixels>,
    layout: &LineLayout,
    line_height: Pixels,
    align: TextAlign,
    align_width: Option<Pixels>,
    decoration_runs: &[DecorationRun],
    wrap_boundaries: &[WrapBoundary],
    retained_decorations: Option<&RetainedDecorations>,
    window: &mut Window,
    cx: &mut App,
    paint_underline: &mut dyn FnMut(Range<usize>, TextUnderline, &mut Window),
) -> Result<()> {
    let aligned_origin = aligned_origin_x(
        origin,
        align_width.unwrap_or(layout.width),
        0.,
        &align,
        layout,
        wrap_boundaries.first().as_ref(),
    );
    let mut paint_underline =
        |range: Range<usize>, origin, width, style: &UnderlineStyle, window: &mut Window| {
            let mut underline = TextUnderline::new(origin, width, *style);
            if let Some(decorations) = retained_decorations {
                let column = decorations.offset + range.start;
                let index = decorations
                    .underlines
                    .partition_point(|(range, _)| range.end <= column);
                if let Some((range, anchor)) = decorations.underlines.get(index)
                    && range.contains(&column)
                {
                    underline = underline.with_anchor(
                        *anchor,
                        decorations.origin_x,
                        f64::from(aligned_origin) - decorations.offset_x,
                        window,
                    );
                }
            }
            window.paint_underline_layer(origin, width, style, |window| {
                paint_underline(range, underline, window);
            });
        };
    let paint_strikethrough = |origin, width, style: &StrikethroughStyle, window: &mut Window| {
        window.paint_underline_layer(
            origin,
            width,
            &UnderlineStyle {
                thickness: style.thickness,
                color: style.color,
                wavy: false,
            },
            |window| window.paint_strikethrough(origin, width, style),
        );
    };
    let mut glyphs = window.take_glyph_buffer();
    let mut glyph_bounds = None::<Bounds<ScaledPixels>>;
    let mut glyph_raster_memo = None;
    let padding_top = (line_height - layout.ascent - layout.descent) / 2.;
    let baseline_offset = point(px(0.), padding_top + layout.ascent);
    let underline_y_offset = underline_y_offset(line_height, layout.ascent, layout.descent);
    let mut decoration_end = 0;
    let decoration_colors = decoration_runs
        .iter()
        .map(|run| {
            decoration_end += run.len as usize;
            (decoration_end, run.color)
        })
        .collect::<SmallVec<[(usize, Hsla); 32]>>();
    let mut decoration_runs = decoration_runs.iter();
    let mut wraps = wrap_boundaries.iter().peekable();
    let mut run_end = 0;
    let mut color = black();
    let mut current_underline: Option<(Point<Pixels>, UnderlineStyle, Range<usize>)> = None;
    let mut current_strikethrough: Option<(Point<Pixels>, StrikethroughStyle)> = None;
    let text_system = cx.text_system().clone();
    let mut line_origin_x = aligned_origin;
    let mut line_start_position_x = 0.;
    let mut glyph_origin = point(line_origin_x, origin.y);
    let mut max_glyph_size = size(px(0.), px(0.));

    for (run_ix, run) in layout.runs.iter().enumerate() {
        max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

        for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
            let (cluster_index, cluster_x) = if layout.caret_positions.is_empty() {
                (glyph.index, f64::from(glyph.position.x))
            } else {
                layout.cluster_start_for_index(glyph.index)
            };
            let mut decoration_x = line_origin_x + Pixels::from(cluster_x - line_start_position_x);
            glyph_origin.x =
                line_origin_x + Pixels::from(f64::from(glyph.position.x) - line_start_position_x);

            if wraps.peek() == Some(&&WrapBoundary { run_ix, glyph_ix }) {
                wraps.next();
                if let Some((underline_origin, underline_style, underline_range)) =
                    current_underline.as_mut()
                {
                    if decoration_x == underline_origin.x {
                        underline_origin.x -= max_glyph_size.width.half();
                    };
                    glyph_raster_memo = None;
                    paint_underline(
                        underline_range.clone(),
                        *underline_origin,
                        decoration_x - underline_origin.x,
                        underline_style,
                        window,
                    );
                    if cluster_index < run_end {
                        underline_origin.x = origin.x;
                        underline_origin.y += line_height;
                    } else {
                        current_underline = None;
                    }
                }
                if let Some((strikethrough_origin, strikethrough_style)) =
                    current_strikethrough.as_mut()
                {
                    if decoration_x == strikethrough_origin.x {
                        strikethrough_origin.x -= max_glyph_size.width.half();
                    };
                    paint_strikethrough(
                        *strikethrough_origin,
                        decoration_x - strikethrough_origin.x,
                        strikethrough_style,
                        window,
                    );
                    if cluster_index < run_end {
                        strikethrough_origin.x = origin.x;
                        strikethrough_origin.y += line_height;
                    } else {
                        current_strikethrough = None;
                    }
                }

                line_origin_x = aligned_origin_x(
                    origin,
                    align_width.unwrap_or(layout.width),
                    cluster_x,
                    &align,
                    layout,
                    wraps.peek(),
                );
                line_start_position_x = cluster_x;
                decoration_x = line_origin_x;
                glyph_origin.x = line_origin_x
                    + Pixels::from(f64::from(glyph.position.x) - line_start_position_x);
                glyph_origin.y += line_height;
                if let Some((underline_origin, _, _)) = current_underline.as_mut() {
                    underline_origin.x = line_origin_x;
                }
                if let Some((strikethrough_origin, _)) = current_strikethrough.as_mut() {
                    strikethrough_origin.x = line_origin_x;
                }
            }

            let mut finished_underline: Option<(Point<Pixels>, UnderlineStyle, Range<usize>)> =
                None;
            let mut finished_strikethrough: Option<(Point<Pixels>, StrikethroughStyle)> = None;
            if cluster_index >= run_end {
                let mut style_run = decoration_runs.next();

                // ignore style runs that apply to a partial glyph
                while let Some(run) = style_run {
                    if cluster_index < run_end + (run.len as usize) {
                        break;
                    }
                    run_end += run.len as usize;
                    style_run = decoration_runs.next();
                }

                if let Some(style_run) = style_run {
                    let style_run_start = run_end;
                    if let Some((_, underline_style, underline_range)) = &mut current_underline {
                        if style_run.underline.as_ref() != Some(underline_style) {
                            finished_underline = current_underline.take();
                        } else {
                            underline_range.end = style_run_start + style_run.len as usize;
                        }
                    }
                    if let Some(run_underline) = style_run.underline.as_ref() {
                        current_underline.get_or_insert((
                            point(decoration_x, glyph_origin.y + underline_y_offset),
                            UnderlineStyle {
                                color: Some(run_underline.color.unwrap_or(style_run.color)),
                                thickness: run_underline.thickness,
                                wavy: run_underline.wavy,
                            },
                            style_run_start..style_run_start + style_run.len as usize,
                        ));
                    }
                    if let Some((_, strikethrough_style)) = &mut current_strikethrough
                        && style_run.strikethrough.as_ref() != Some(strikethrough_style)
                    {
                        finished_strikethrough = current_strikethrough.take();
                    }
                    if let Some(run_strikethrough) = style_run.strikethrough.as_ref() {
                        current_strikethrough.get_or_insert((
                            point(
                                decoration_x,
                                glyph_origin.y
                                    + (((layout.ascent * 0.5) + baseline_offset.y) * 0.5),
                            ),
                            StrikethroughStyle {
                                color: Some(run_strikethrough.color.unwrap_or(style_run.color)),
                                thickness: run_strikethrough.thickness,
                            },
                        ));
                    }

                    run_end += style_run.len as usize;
                    color = style_run.color;
                } else {
                    run_end = layout.len;
                    finished_underline = current_underline.take();
                    finished_strikethrough = current_strikethrough.take();
                }
            }

            if let Some((mut underline_origin, underline_style, underline_range)) =
                finished_underline
            {
                if underline_origin.x == decoration_x {
                    underline_origin.x -= max_glyph_size.width.half();
                };
                glyph_raster_memo = None;
                paint_underline(
                    underline_range,
                    underline_origin,
                    decoration_x - underline_origin.x,
                    &underline_style,
                    window,
                );
            }

            if let Some((mut strikethrough_origin, strikethrough_style)) = finished_strikethrough {
                if strikethrough_origin.x == decoration_x {
                    strikethrough_origin.x -= max_glyph_size.width.half();
                };
                paint_strikethrough(
                    strikethrough_origin,
                    decoration_x - strikethrough_origin.x,
                    &strikethrough_style,
                    window,
                );
            }

            let vertical_offset = point(px(0.0), glyph.position.y);
            let sprite = if glyph.is_emoji {
                window.prepare_emoji(
                    glyph_origin + baseline_offset + vertical_offset,
                    run.font_id,
                    glyph.id,
                    layout.font_size,
                    &mut glyph_raster_memo,
                )?
            } else {
                let color = decoration_colors
                    .get(decoration_colors.partition_point(|(end, _)| *end <= glyph.index))
                    .map_or(color, |(_, color)| *color);
                window.prepare_glyph(
                    glyph_origin + baseline_offset + vertical_offset,
                    run.font_id,
                    glyph.id,
                    layout.font_size,
                    color,
                    &mut glyph_raster_memo,
                )?
            };
            if let Some((sprite, bounds)) = sprite {
                glyph_bounds =
                    Some(glyph_bounds.map_or(bounds, |previous| previous.union(&bounds)));
                glyphs.push(sprite);
            }
        }
    }

    let last_line_end_x =
        line_origin_x + Pixels::from(layout.caret_x_for_index(layout.len) - line_start_position_x);

    if let Some((mut underline_start, underline_style, underline_range)) = current_underline.take()
    {
        if last_line_end_x == underline_start.x {
            underline_start.x -= max_glyph_size.width.half()
        };
        paint_underline(
            underline_range,
            underline_start,
            last_line_end_x - underline_start.x,
            &underline_style,
            window,
        );
    }

    if let Some((mut strikethrough_start, strikethrough_style)) = current_strikethrough.take() {
        if last_line_end_x == strikethrough_start.x {
            strikethrough_start.x -= max_glyph_size.width.half()
        };
        paint_strikethrough(
            strikethrough_start,
            last_line_end_x - strikethrough_start.x,
            &strikethrough_style,
            window,
        );
    }

    window.paint_glyphs(glyphs, glyph_bounds);
    Ok(())
}

fn paint_line_background(
    origin: Point<Pixels>,
    layout: &LineLayout,
    line_height: Pixels,
    align: TextAlign,
    align_width: Option<Pixels>,
    decoration_runs: &[DecorationRun],
    wrap_boundaries: &[WrapBoundary],
    window: &mut Window,
    cx: &mut App,
) -> Result<()> {
    if decoration_runs
        .iter()
        .all(|run| run.background_color.is_none())
    {
        return Ok(());
    }
    let line_bounds = line_paint_bounds(
        origin,
        layout,
        line_height,
        align,
        align_width,
        wrap_boundaries,
        decoration_runs,
        |font_id| cx.text_system().bounding_box(font_id, layout.font_size),
    );
    window.paint_layer(line_bounds, |window| {
        let mut decoration_runs = decoration_runs.iter();
        let mut wraps = wrap_boundaries.iter().peekable();
        let mut run_end = 0;
        let mut current_background: Option<(Point<Pixels>, Hsla)> = None;
        let text_system = cx.text_system().clone();
        let mut line_origin_x = aligned_origin_x(
            origin,
            align_width.unwrap_or(layout.width),
            0.,
            &align,
            layout,
            wraps.peek(),
        );
        let mut line_start_position_x = 0.;
        let mut glyph_origin = point(line_origin_x, origin.y);
        let mut max_glyph_size = size(px(0.), px(0.));
        for (run_ix, run) in layout.runs.iter().enumerate() {
            max_glyph_size = text_system.bounding_box(run.font_id, layout.font_size).size;

            for (glyph_ix, glyph) in run.glyphs.iter().enumerate() {
                let (cluster_index, cluster_x) = if layout.caret_positions.is_empty() {
                    (glyph.index, f64::from(glyph.position.x))
                } else {
                    layout.cluster_start_for_index(glyph.index)
                };
                glyph_origin.x = line_origin_x + Pixels::from(cluster_x - line_start_position_x);

                if wraps.peek() == Some(&&WrapBoundary { run_ix, glyph_ix }) {
                    wraps.next();
                    if let Some((background_origin, background_color)) = current_background.as_mut()
                    {
                        if glyph_origin.x == background_origin.x {
                            background_origin.x -= max_glyph_size.width.half()
                        }
                        window.paint_quad(fill(
                            Bounds {
                                origin: *background_origin,
                                size: size(glyph_origin.x - background_origin.x, line_height),
                            },
                            *background_color,
                        ));
                        if cluster_index < run_end {
                            background_origin.x = origin.x;
                            background_origin.y += line_height;
                        } else {
                            current_background = None;
                        }
                    }

                    line_origin_x = aligned_origin_x(
                        origin,
                        align_width.unwrap_or(layout.width),
                        cluster_x,
                        &align,
                        layout,
                        wraps.peek(),
                    );
                    line_start_position_x = cluster_x;
                    glyph_origin.x = line_origin_x;
                    glyph_origin.y += line_height;
                    if let Some((background_origin, _)) = current_background.as_mut() {
                        background_origin.x = line_origin_x;
                    }
                }

                let mut finished_background: Option<(Point<Pixels>, Hsla)> = None;
                if cluster_index >= run_end {
                    let mut style_run = decoration_runs.next();

                    // ignore style runs that apply to a partial glyph
                    while let Some(run) = style_run {
                        if cluster_index < run_end + (run.len as usize) {
                            break;
                        }
                        run_end += run.len as usize;
                        style_run = decoration_runs.next();
                    }

                    if let Some(style_run) = style_run {
                        if let Some((_, background_color)) = &mut current_background
                            && style_run.background_color.as_ref() != Some(background_color)
                        {
                            finished_background = current_background.take();
                        }
                        if let Some(run_background) = style_run.background_color {
                            current_background.get_or_insert((
                                point(glyph_origin.x, glyph_origin.y),
                                run_background,
                            ));
                        }
                        run_end += style_run.len as usize;
                    } else {
                        run_end = layout.len;
                        finished_background = current_background.take();
                    }
                }

                if let Some((mut background_origin, background_color)) = finished_background {
                    let mut width = glyph_origin.x - background_origin.x;
                    if background_origin.x == glyph_origin.x {
                        background_origin.x -= max_glyph_size.width.half();
                    };
                    window.paint_quad(fill(
                        Bounds {
                            origin: background_origin,
                            size: size(width, line_height),
                        },
                        background_color,
                    ));
                }
            }
        }

        let last_line_end_x = line_origin_x
            + Pixels::from(layout.caret_x_for_index(layout.len) - line_start_position_x);

        if let Some((mut background_origin, background_color)) = current_background.take() {
            if last_line_end_x == background_origin.x {
                background_origin.x -= max_glyph_size.width.half()
            };
            window.paint_quad(fill(
                Bounds {
                    origin: background_origin,
                    size: size(last_line_end_x - background_origin.x, line_height),
                },
                background_color,
            ));
        }

        Ok(())
    })
}

fn line_paint_bounds(
    origin: Point<Pixels>,
    layout: &LineLayout,
    line_height: Pixels,
    align: TextAlign,
    align_width: Option<Pixels>,
    wrap_boundaries: &[WrapBoundary],
    decoration_runs: &[DecorationRun],
    mut font_bounds: impl FnMut(FontId) -> Bounds<Pixels>,
) -> Bounds<Pixels> {
    let mut bounds = Bounds::new(
        origin,
        size(
            layout.width,
            line_height * (wrap_boundaries.len() as f32 + 1.),
        ),
    );
    if layout.len == 0 {
        return bounds;
    }

    let has_decorations = decoration_runs.iter().any(|run| {
        run.background_color.is_some() || run.underline.is_some() || run.strikethrough.is_some()
    });
    let mut decoration_left = origin.x;
    let mut decoration_padding = Pixels::ZERO;
    let mut glyphs = layout
        .runs
        .iter()
        .enumerate()
        .flat_map(|(run_ix, run)| {
            let font_bounds = font_bounds(run.font_id);
            if has_decorations {
                decoration_padding = decoration_padding.max(font_bounds.size.width.half());
            }
            let left = font_bounds.left().min(Pixels::ZERO);
            let right = font_bounds.right().max(Pixels::ZERO);
            run.glyphs
                .iter()
                .enumerate()
                .map(move |(glyph_ix, glyph)| ((run_ix, glyph_ix), glyph.position.x, left..right))
        })
        .peekable();
    let align_width = align_width.unwrap_or(layout.width);
    let mut row_start = 0.;
    for (row, boundary) in wrap_boundaries.iter().map(Some).chain([None]).enumerate() {
        let row_end = boundary.map_or_else(
            || layout.caret_x_for_index(layout.len),
            |boundary| {
                layout
                    .cluster_start_for_index(
                        layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index,
                    )
                    .1
            },
        );
        let row_origin_x = aligned_origin_x(
            origin,
            align_width,
            row_start,
            &align,
            layout,
            boundary.as_ref(),
        );
        bounds = bounds.union(&Bounds::new(
            point(row_origin_x, origin.y + line_height * row as f32),
            size(Pixels::from(row_end - row_start), line_height),
        ));
        decoration_left = decoration_left
            .min(row_origin_x)
            .min(row_origin_x + Pixels::from(row_end - row_start));
        while let Some((_, position_x, extent)) = glyphs.next_if(|(index, _, _)| {
            boundary.is_none_or(|boundary| *index < (boundary.run_ix, boundary.glyph_ix))
        }) {
            let glyph_x = row_origin_x + Pixels::from(f64::from(position_x) - row_start);
            decoration_left = decoration_left.min(glyph_x);
            let left = bounds.left().min(glyph_x + extent.start);
            let right = bounds.right().max(glyph_x + extent.end);
            bounds.origin.x = left;
            bounds.size.width = right - left;
        }
        row_start = row_end;
    }
    let right = bounds.right();
    bounds.origin.x = bounds.left().min(decoration_left - decoration_padding);
    bounds.size.width = right - bounds.origin.x;
    bounds
}

fn aligned_origin_x(
    origin: Point<Pixels>,
    align_width: Pixels,
    last_glyph_x: f64,
    align: &TextAlign,
    layout: &LineLayout,
    wrap_boundary: Option<&&WrapBoundary>,
) -> Pixels {
    let end_of_line = if let Some(WrapBoundary { run_ix, glyph_ix }) = wrap_boundary {
        layout
            .cluster_start_for_index(layout.runs[*run_ix].glyphs[*glyph_ix].index)
            .1
    } else {
        layout.caret_x_for_index(layout.len)
    };

    let line_width = Pixels::from(end_of_line - last_glyph_x);

    match align {
        TextAlign::Left => origin.x,
        TextAlign::Center => (origin.x * 2.0 + align_width - line_width) / 2.0,
        TextAlign::Right => origin.x + align_width - line_width,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AppContext as _, AtlasKey, ContentMask, Context, GlyphId, IntoElement, PrimitiveBatch,
        Render, SUBPIXEL_VARIANTS_X, ScaledPixels, Scene, ShapedGlyph, ShapedRun, Styled,
        TestAppContext, TextRun, Underline, canvas, font, hsla,
    };
    use std::rc::Rc;

    /// Helper: build a ShapedLine from glyph descriptors without the platform text system.
    /// Each glyph is described as (byte_index, x_position).
    fn make_shaped_line(
        text: &str,
        glyphs: &[(usize, f32)],
        width: f32,
        decorations: &[DecorationRun],
    ) -> ShapedLine {
        let shaped_glyphs: Vec<ShapedGlyph> = glyphs
            .iter()
            .map(|&(index, x)| ShapedGlyph {
                id: GlyphId(0),
                position: point(px(x), px(0.0)),
                index,
                is_emoji: false,
            })
            .collect();

        ShapedLine {
            layout: Arc::new(LineLayout {
                font_size: px(16.0),
                width: px(width),
                ascent: px(12.0),
                descent: px(4.0),
                runs: vec![ShapedRun {
                    font_id: FontId(0),
                    glyphs: shaped_glyphs,
                }],
                len: text.len(),
                caret_positions: Vec::new(),
            }),
            text: SharedString::new(text),
            decoration_runs: SmallVec::from(decorations.to_vec()),
            retained_decorations: None,
        }
    }

    #[test]
    fn test_aligned_line_paint_bounds() {
        let line = make_shaped_line("abcd", &[(0, 0.), (1, 10.), (2, 20.), (3, 30.)], 40., &[]);
        let origin = point(px(10.), px(20.));
        let line_height = px(16.);
        for (wrapped, align_width, expected_edges) in [
            (false, None, [(10., 50.), (10., 50.), (10., 50.)]),
            (true, None, [(10., 50.), (10., 50.), (10., 50.)]),
            (false, Some(40.), [(10., 50.), (10., 50.), (10., 50.)]),
            (true, Some(40.), [(10., 50.), (10., 50.), (10., 50.)]),
            (false, Some(100.), [(10., 50.), (10., 80.), (10., 110.)]),
            (true, Some(100.), [(10., 50.), (10., 70.), (10., 110.)]),
            (false, Some(10.), [(10., 50.), (-5., 50.), (-20., 50.)]),
            (true, Some(10.), [(10., 50.), (5., 50.), (0., 50.)]),
            (false, Some(0.), [(10., 50.), (-10., 50.), (-30., 50.)]),
            (true, Some(0.), [(10., 50.), (0., 50.), (-10., 50.)]),
        ] {
            let boundaries = if wrapped {
                vec![WrapBoundary {
                    run_ix: 0,
                    glyph_ix: 2,
                }]
            } else {
                Vec::new()
            };
            for (align, (left, right)) in [TextAlign::Left, TextAlign::Center, TextAlign::Right]
                .into_iter()
                .zip(expected_edges)
            {
                assert_eq!(
                    line_paint_bounds(
                        origin,
                        &line.layout,
                        line_height,
                        align,
                        align_width.map(px),
                        &boundaries,
                        &[],
                        |_| Bounds::default(),
                    ),
                    Bounds::new(
                        point(px(left), origin.y),
                        size(px(right - left), px(if wrapped { 32. } else { 16. })),
                    ),
                    "wrapped={wrapped}, align={align:?}, align_width={align_width:?}",
                );
            }
        }
        let empty = make_shaped_line("", &[], 0., &[]);
        for align in [TextAlign::Left, TextAlign::Center, TextAlign::Right] {
            assert_eq!(
                line_paint_bounds(
                    origin,
                    &empty.layout,
                    line_height,
                    align,
                    Some(px(100.)),
                    &[],
                    &[],
                    |_| Bounds::default(),
                ),
                Bounds::new(origin, size(Pixels::ZERO, line_height)),
            );
        }
    }

    #[test]
    fn test_line_paint_bounds_include_font_bearings() {
        let mut line =
            make_shaped_line("abcd", &[(0, -8.), (1, 12.), (2, 20.), (3, 10.)], 44., &[]);
        let layout = Arc::get_mut(&mut line.layout).expect("unshared test layout");
        let glyphs = layout.runs[0].glyphs.split_off(2);
        layout.runs.push(ShapedRun {
            font_id: FontId(1),
            glyphs,
        });
        for (wrapped, expected_edges) in [
            (false, [(16., 100.), (34., 118.), (40., 136.)]),
            (true, [(16., 84.), (40., 108.), (40., 136.)]),
        ] {
            let boundaries = if wrapped {
                vec![WrapBoundary {
                    run_ix: 1,
                    glyph_ix: 0,
                }]
            } else {
                Vec::new()
            };
            for (align, (left, right)) in [TextAlign::Left, TextAlign::Center, TextAlign::Right]
                .into_iter()
                .zip(expected_edges)
            {
                assert_eq!(
                    line_paint_bounds(
                        point(px(40.), px(10.)),
                        layout,
                        px(20.),
                        align,
                        Some(px(80.)),
                        &boundaries,
                        &[],
                        |font_id| Bounds::new(
                            point(px(if font_id == FontId(0) { -16. } else { 16. }), px(0.)),
                            size(px(24.), px(16.)),
                        ),
                    ),
                    Bounds::new(
                        point(px(left), px(10.)),
                        size(px(right - left), px(if wrapped { 40. } else { 20. })),
                    ),
                    "wrapped={wrapped}, align={align:?}",
                );
            }
        }
    }

    #[gpui::test]
    fn test_font_bearings_preserve_clipped_sprites_and_scene_order(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let text_system = cx.text_system().clone();
            let original_metrics = text_system.read_metrics(FontId(0), |metrics| *metrics);
            let origin = point(px(40.), px(42.));
            let scale = window.scale_factor();
            let atlas = window.platform_window.sprite_atlas();
            for (bearing, vertical_offset) in [(-12., 0.), (12., 0.), (-12., -32.), (12., 32.)] {
                for (is_emoji, quarter_pixel_offset) in [
                    (false, 0),
                    (false, 1),
                    (false, 3),
                    (true, 0),
                    (true, 1),
                    (true, 3),
                ] {
                    let subpixel_variant = if is_emoji { 0 } else { quarter_pixel_offset };
                    let params = RenderGlyphParams {
                        font_id: FontId(0),
                        glyph_id: GlyphId(0),
                        font_size: px(16.),
                        subpixel_variant: point(subpixel_variant, 0),
                        scale_factor: scale,
                        is_emoji,
                        subpixel_rendering: false,
                        dilation: if is_emoji {
                            0
                        } else {
                            text_system.glyph_dilation_for_color(black())
                        },
                    };
                    let mut line = make_shaped_line(
                        "ffi",
                        &[(0, f32::from(quarter_pixel_offset) / (4. * scale))],
                        8.,
                        &[],
                    );
                    let glyph = &mut Arc::get_mut(&mut line.layout)
                        .expect("unshared layout")
                        .runs[0]
                        .glyphs[0];
                    glyph.position.y = px(vertical_offset);
                    glyph.is_emoji = is_emoji;
                    let mut metrics = original_metrics;
                    metrics.units_per_em = 16;
                    metrics.bounding_box = Bounds::new(point(bearing, -8.), size(16., 8.));
                    text_system.font_metrics.write().insert(FontId(0), metrics);
                    let raster_bounds = Bounds::new(
                        point(
                            DevicePixels((bearing * scale) as i32),
                            DevicePixels((-8. * scale) as i32),
                        ),
                        size(
                            DevicePixels((16. * scale) as i32 + i32::from(subpixel_variant > 0)),
                            DevicePixels((8. * scale) as i32),
                        ),
                    );

                    text_system
                        .raster_bounds
                        .write()
                        .insert(params.clone(), raster_bounds);
                    let key = AtlasKey::from(params.clone());
                    atlas.remove(&key);

                    let expected_bounds = Bounds::new(
                        point(
                            ScaledPixels(
                                (40. + bearing) * scale
                                    + if is_emoji && quarter_pixel_offset == 3 {
                                        1.
                                    } else {
                                        0.
                                    },
                            ),
                            ScaledPixels((48. + vertical_offset) * scale),
                        ),
                        raster_bounds.size.map(ScaledPixels::from),
                    );
                    let left = expected_bounds.left().0;
                    let top = expected_bounds.top().0;
                    let right = expected_bounds.right().0;
                    let bottom = expected_bounds.bottom().0;
                    for (left, top, right, bottom, visible) in [
                        (left, top, right, bottom, true),
                        (left + 0.5, top, left + 1.5, bottom, true),
                        (right - 0.5, top, right + 0.5, bottom, true),
                        (left, top + 0.5, right, top + 1.5, true),
                        (left, bottom - 0.5, right, bottom + 0.5, true),
                        (left - 1.5, top, left - 0.5, bottom, false),
                        (left, top - 1.5, right, top - 0.5, false),
                        (right + 0.5, top, right + 1.5, bottom, false),
                        (left, bottom + 0.5, right, bottom + 1.5, false),
                    ] {
                        let mask = Bounds::from_corners(
                            point(px(left / scale), px(top / scale)),
                            point(px(right / scale), px(bottom / scale)),
                        );
                        let snapped_mask = Bounds::from_corners(
                            mask.origin
                                .scale(scale)
                                .map(|value| ScaledPixels(value.0.floor())),
                            mask.bottom_right()
                                .scale(scale)
                                .map(|value| ScaledPixels(value.0.ceil())),
                        );
                        let overlap = expected_bounds
                            .intersect(&snapped_mask)
                            .map(|value| px(value.0 / scale));
                        window.next_frame.scene.clear();
                        window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                            if visible {
                                window.paint_quad(fill(overlap, black()));
                            }
                            line.paint(origin, px(20.), TextAlign::Left, None, window, cx)
                                .expect("paint clipped glyph");
                            if visible {
                                window.paint_quad(fill(overlap, black()));
                            }
                        });
                        let scene = &window.next_frame.scene;
                        let mut replay = Scene::default();
                        replay.replay(0..scene.len(), scene);
                        for scene in [scene, &replay] {
                            let sprites = scene
                                .monochrome_sprites
                                .iter()
                                .map(|sprite| {
                                    (sprite.bounds, sprite.content_mask.bounds, sprite.order)
                                })
                                .chain(scene.polychrome_sprites.iter().map(|sprite| {
                                    (sprite.bounds, sprite.content_mask.bounds, sprite.order)
                                }))
                                .collect::<Vec<_>>();
                            if visible {
                                assert_eq!(
                                    sprites,
                                    [(expected_bounds, snapped_mask, 2)],
                                    "scale={scale}, bearing={bearing}, vertical_offset={vertical_offset}, is_emoji={is_emoji}, quarter_pixel_offset={quarter_pixel_offset}, mask={mask:?}"
                                );
                                assert_eq!(
                                    scene
                                        .quads
                                        .iter()
                                        .map(|quad| quad.order)
                                        .collect::<Vec<_>>(),
                                    [1, 3]
                                );
                            } else {
                                assert_eq!(sprites, Vec::new());
                                assert_eq!(scene.len(), 0);
                            }
                        }
                    }
                    atlas.remove(&key);
                    text_system.raster_bounds.write().remove(&params);
                }
            }
            text_system
                .font_metrics
                .write()
                .insert(FontId(0), original_metrics);
            window.next_frame.scene.clear();
        });
    }

    #[gpui::test]
    fn test_raster_envelope_contains_prepared_sprites(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let text_system = cx.text_system().clone();
            let scale = window.scale_factor();
            let config = window.glyph_raster_config();
            let atlas = window.platform_window.sprite_atlas();
            let mut glyph_raster_memo = None;
            for (font_id, is_emoji) in [(FontId(0), false), (FontId(1), false), (FontId(1), true)] {
                let params = (0..if is_emoji { 1 } else { SUBPIXEL_VARIANTS_X })
                    .map(|phase| RenderGlyphParams {
                        font_id,
                        glyph_id: GlyphId(0),
                        font_size: px(16.),
                        subpixel_variant: point(phase, 0),
                        scale_factor: scale,
                        is_emoji,
                        subpixel_rendering: false,
                        dilation: 0,
                    })
                    .collect::<Vec<_>>();
                for params in &params {
                    text_system.raster_bounds.write().insert(
                        params.clone(),
                        Bounds::new(
                            point(DevicePixels(-3), DevicePixels(-16)),
                            size(
                                DevicePixels(
                                    16 + font_id.0 as i32
                                        + i32::from(params.subpixel_variant.x > 0),
                                ),
                                DevicePixels(20),
                            ),
                        ),
                    );
                    atlas.remove(&AtlasKey::from(params.clone()));
                }
                let envelope = text_system
                    .glyph_raster_envelope(font_id, GlyphId(0), px(16.), config, is_emoji)
                    .expect("raster envelope");
                for step in -64..=64 {
                    let origin = point(
                        px(step as f32 / (32. * scale)),
                        px(20. + step as f32 / (32. * scale)),
                    );
                    let mask = ContentMask {
                        bounds: Bounds::new(
                            point(
                                px(if step == -64 { 100. } else { (step & 1) as f32 }),
                                px(0.),
                            ),
                            size(px(if step == -32 { 0. } else { 6. }), px(100.)),
                        ),
                    };
                    let color = hsla((step & 1) as f32 * 0.5, 1., 0.5, 1.);
                    let opacity = if step & 1 == 0 { 0.5 } else { 1. };
                    window.with_content_mask(Some(mask), |window| {
                        window.with_element_opacity(Some(opacity), |window| {
                            let prepared = if is_emoji {
                                window.prepare_emoji(origin, font_id, GlyphId(0), px(16.), &mut glyph_raster_memo)
                            } else {
                                window.prepare_glyph(origin, font_id, GlyphId(0), px(16.), color, &mut glyph_raster_memo)
                            }
                            .expect("prepared sprite");
                            if step == -64 || step == -32 {
                                assert!(prepared.is_none());
                                return;
                            }
                            let (sprite, clipped_bounds) = prepared.expect("intersecting sprite");
                            if scale == 1. && font_id == FontId(0) && (-63..=-62).contains(&step) {
                                assert_eq!(
                                    clipped_bounds,
                                    Bounds::new(
                                        point(ScaledPixels((step & 1) as f32), ScaledPixels(2.)),
                                        size(ScaledPixels(6.), ScaledPixels(20.)),
                                    ),
                                );
                            }
                            let actual = sprite.bounds.map(|value| f64::from(value.0));
                            let bounds = envelope + origin.scale(scale).map(|value| f64::from(value.0));
                            assert!(
                                bounds.left() <= actual.left() && bounds.right() >= actual.right(),
                                "scale={scale}, emoji={is_emoji}, origin={origin:?}, bounds={bounds:?}, actual={actual:?}"
                            );
                            assert!(
                                bounds.top() <= actual.top() && bounds.bottom() >= actual.bottom(),
                                "scale={scale}, emoji={is_emoji}, origin={origin:?}, bounds={bounds:?}, actual={actual:?}"
                            );
                            window.next_frame.scene.clear();
                            window.paint_glyphs(vec![sprite], Some(clipped_bounds));
                            if is_emoji {
                                window.paint_emoji(origin, font_id, GlyphId(0), px(16.)).unwrap();
                                let sprites = &window.next_frame.scene.polychrome_sprites;
                                assert_eq!(sprites.len(), 2);
                                assert_eq!(sprites[0].opacity, 0.5 * opacity);
                                assert_eq!(
                                    (sprites[0].bounds, sprites[0].content_mask, sprites[0].tile, sprites[0].opacity),
                                    (sprites[1].bounds, sprites[1].content_mask, sprites[1].tile, sprites[1].opacity),
                                );
                            } else {
                                window.paint_glyph(origin, font_id, GlyphId(0), px(16.), color).unwrap();
                                let sprites = &window.next_frame.scene.monochrome_sprites;
                                assert_eq!(sprites.len(), 2);
                                assert_eq!(sprites[0].color, color.opacity(0.5 * opacity));
                                assert_eq!(
                                    (sprites[0].bounds, sprites[0].content_mask, sprites[0].tile, sprites[0].color),
                                    (sprites[1].bounds, sprites[1].content_mask, sprites[1].tile, sprites[1].color),
                                );
                            }
                            window.next_frame.scene.clear();
                        });
                    });
                }
                for params in &params {
                    atlas.remove(&AtlasKey::from(params.clone()));
                    text_system.raster_bounds.write().remove(params);
                }
            }
        });
    }

    #[gpui::test]
    fn test_fractional_clip_preserves_raster_coverage(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            if window.scale_factor() != 1. {
                return;
            }
            let text_system = cx.text_system().clone();
            let original_metrics = text_system.read_metrics(FontId(0), |metrics| *metrics);
            let mut metrics = original_metrics;
            metrics.units_per_em = 16;
            metrics.bounding_box =
                Bounds::new(point(-8.9296875, -5.9921875), size(20.4140625, 22.65625));
            text_system.font_metrics.write().insert(FontId(0), metrics);
            let params = RenderGlyphParams {
                font_id: FontId(0),
                glyph_id: GlyphId(0),
                font_size: px(16.),
                subpixel_variant: point(0, 0),
                scale_factor: 1.,
                is_emoji: false,
                subpixel_rendering: false,
                dilation: text_system.glyph_dilation_for_color(black()),
            };
            text_system.raster_bounds.write().insert(
                params.clone(),
                Bounds::new(
                    point(DevicePixels(-3), DevicePixels(-16)),
                    size(DevicePixels(16), DevicePixels(6)),
                ),
            );
            let line = make_shaped_line("\u{0361}", &[(0, 0.)], 0., &[]);
            let mask = Bounds::new(point(px(31.5), px(8.)), size(px(4.), px(6.)));
            let sprite_count =
                window.with_content_mask(Some(ContentMask { bounds: mask }), |window| {
                    window.next_frame.scene.clear();
                    window
                        .paint_glyph(
                            point(px(20.), px(24.)),
                            FontId(0),
                            GlyphId(0),
                            px(16.),
                            black(),
                        )
                        .expect("paint reference glyph");
                    assert_eq!(window.next_frame.scene.monochrome_sprites.len(), 1);
                    let sprite = &window.next_frame.scene.monochrome_sprites[0];
                    assert_eq!(
                        sprite.content_mask.bounds,
                        Bounds::new(point(px(31.), px(8.)), size(px(5.), px(6.))).scale(1.),
                    );
                    assert_eq!(
                        sprite.bounds.intersect(&sprite.content_mask.bounds),
                        Bounds::new(point(px(31.), px(8.)), size(px(2.), px(6.))).scale(1.),
                    );
                    window.next_frame.scene.clear();
                    line.paint(
                        point(px(20.), px(10.)),
                        px(20.),
                        TextAlign::Left,
                        None,
                        window,
                        cx,
                    )
                    .expect("paint clipped shaped line");
                    let count = window.next_frame.scene.monochrome_sprites.len();
                    window.next_frame.scene.clear();
                    count
                });
            text_system.raster_bounds.write().remove(&params);
            text_system
                .font_metrics
                .write()
                .insert(FontId(0), original_metrics);
            assert_eq!(
                sprite_count, 1,
                "fractional clip must retain native raster column 31",
            );
        });
    }

    #[gpui::test]
    fn test_retained_underline_phase_survives_slicing_and_exclusions(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let scale_quarters = (window.scale_factor() * 4.) as i128;
            let thickness = ((scale_quarters + 1) / 4).max(1);
            let period = 9 * thickness;
            for reference_quarters in [0_i128, 1 << 42, (1 << 53) + 2, 1 << 62] {
                let line = underline_test_line(
                    "abcdef",
                    &[DecorationRun {
                        len: 6,
                        color: black(),
                        background_color: None,
                        underline: Some(UnderlineStyle {
                            thickness: px(1.),
                            color: Some(black()),
                            wavy: true,
                        }),
                        strikethrough: None,
                    }],
                    false,
                    window,
                )
                .with_underline_origins(reference_quarters as f64 / 4., vec![(0..6, 0.)]);
                for start in [0, 1, 2] {
                    for incremental in [false, true] {
                        let slice = if incremental {
                            let mut cursor = line.cursor();
                            cursor.take_until(start);
                            cursor.take_until(6)
                        } else {
                            line.split_at(start).1
                        };
                        window.next_frame.scene.clear();
                        slice
                            .paint_with_underline_handler(
                                point(px(5.25), px(12.)),
                                px(20.),
                                TextAlign::Left,
                                None,
                                window,
                                cx,
                                |_, underline, window| {
                                    underline.paint_with_exclusions(
                                        &[ScaledPixels(20.)..ScaledPixels(22.)],
                                        window,
                                    )
                                },
                            )
                            .unwrap();
                        let numerator =
                            (21 - start as i128 * 32 - reference_quarters) * scale_quarters;
                        let anchor = ((numerator.abs() + 7) / 16) * numerator.signum();
                        let left = (21 * scale_quarters + 7) / 16;
                        let right = left + (6 - start as i128) * 2 * scale_quarters;
                        let underlines = &window.next_frame.scene.underlines;
                        assert_eq!(
                            underlines
                                .iter()
                                .map(|underline| {
                                    let bounds =
                                        underline.bounds.intersect(&underline.content_mask.bounds);
                                    (bounds.left().0 as i128)..(bounds.right().0 as i128)
                                })
                                .collect::<Vec<_>>(),
                            [left..20, 22..right]
                        );
                        for underline in underlines {
                            let origin = underline.bounds.left().0 as i128;
                            assert_eq!((origin - anchor).rem_euclid(period), 0);
                            assert!(origin <= left && left - origin < period);
                        }
                    }
                }
            }
        });
    }

    #[gpui::test]
    fn test_underline_handler_matches_default_paint(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let first_style = UnderlineStyle {
                thickness: px(1.),
                color: Some(hsla(0., 1., 0.5, 1.)),
                wavy: true,
            };
            let last_style = UnderlineStyle {
                color: Some(hsla(0.5, 1., 0.5, 1.)),
                wavy: false,
                ..first_style
            };
            let fallback_style = UnderlineStyle {
                color: Some(black()),
                ..first_style
            };
            let decoration = DecorationRun {
                len: 1,
                color: black(),
                background_color: None,
                underline: Some(first_style),
                strikethrough: None,
            };
            let line = underline_test_line(
                "aébcde",
                &[
                    decoration.clone(),
                    DecorationRun {
                        len: 2,
                        ..decoration.clone()
                    },
                    DecorationRun {
                        underline: None,
                        ..decoration.clone()
                    },
                    DecorationRun {
                        underline: Some(UnderlineStyle {
                            color: None,
                            ..first_style
                        }),
                        ..decoration.clone()
                    },
                    DecorationRun {
                        underline: Some(last_style),
                        ..decoration.clone()
                    },
                    DecorationRun {
                        underline: Some(last_style),
                        ..decoration
                    },
                ],
                false,
                window,
            );
            assert_eq!(line.width(), px(48.));
            for origin_x in [-3.25, 0., 4.25] {
                for (align, align_width, offset) in [
                    (TextAlign::Left, None, 0.),
                    (TextAlign::Center, None, 0.),
                    (TextAlign::Right, None, 0.),
                    (TextAlign::Left, Some(px(96.)), 0.),
                    (TextAlign::Center, Some(px(96.)), 24.),
                    (TextAlign::Right, Some(px(96.)), 48.),
                ] {
                    let origin = point(px(origin_x), px(12.25));
                    let line_height = px(20.);
                    window.next_frame.scene.clear();
                    line.paint(origin, line_height, align, align_width, window, cx)
                        .unwrap();
                    let original = window.next_frame.scene.underlines.clone();
                    window.next_frame.scene.clear();
                    line.layout
                        .paint(
                            origin,
                            line_height,
                            align,
                            align_width,
                            &line.decoration_runs,
                            window,
                            cx,
                        )
                        .unwrap();
                    assert_underline_primitives_eq(&window.next_frame.scene.underlines, &original);

                    window.next_frame.scene.clear();
                    let mut strokes = Vec::new();
                    line.paint_with_underline_handler(
                        origin,
                        line_height,
                        align,
                        align_width,
                        window,
                        cx,
                        |range, underline, window| {
                            strokes.push((
                                range,
                                underline.origin,
                                underline.width,
                                underline.style,
                            ));
                            underline.paint(window);
                        },
                    )
                    .unwrap();
                    let start = px(origin_x + offset);
                    let y = origin.y + underline_y_offset(line_height, line.ascent, line.descent);
                    assert_eq!(
                        strokes,
                        [
                            (0..3, point(start, y), px(16.), first_style),
                            (4..5, point(start + px(24.), y), px(8.), fallback_style),
                            (5..7, point(start + px(32.), y), px(16.), last_style),
                        ]
                    );
                    assert_underline_primitives_eq(&window.next_frame.scene.underlines, &original);

                    window.next_frame.scene.clear();
                    let mut captured = Vec::new();
                    line.paint_with_underline_handler(
                        origin,
                        line_height,
                        align,
                        align_width,
                        window,
                        cx,
                        |range, underline, _| {
                            captured.push((
                                range,
                                underline.origin,
                                underline.width,
                                underline.style,
                            ))
                        },
                    )
                    .unwrap();
                    assert_eq!(captured, strokes);
                    assert_eq!(window.next_frame.scene.underlines.len(), 0);
                }
            }
            for text in ["", "abc"] {
                let line = underline_test_line(text, &[], false, window);
                let mut calls = 0;
                line.paint_with_underline_handler(
                    point(px(4.), px(10.)),
                    px(20.),
                    TextAlign::Left,
                    None,
                    window,
                    cx,
                    |_, _, _| calls += 1,
                )
                .unwrap();
                assert_eq!(calls, 0);
            }
        });
    }

    #[gpui::test]
    fn test_underline_handler_reports_zero_advance_geometry(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let first_style = UnderlineStyle {
                thickness: px(1.),
                color: Some(black()),
                wavy: true,
            };
            let last_style = UnderlineStyle {
                wavy: false,
                ..first_style
            };
            let decoration = DecorationRun {
                len: 1,
                color: black(),
                background_color: None,
                underline: Some(first_style),
                strikethrough: None,
            };
            let line = underline_test_line(
                "ab",
                &[
                    decoration.clone(),
                    DecorationRun {
                        underline: Some(last_style),
                        ..decoration
                    },
                ],
                true,
                window,
            );
            let half_width = cx
                .text_system()
                .bounding_box(line.runs[0].font_id, line.font_size)
                .size
                .width
                / 2.;
            let origin = point(px(40.25), px(10.25));
            let line_height = px(20.);
            let y = origin.y + underline_y_offset(line_height, line.ascent, line.descent);
            for (align, offset) in [
                (TextAlign::Left, 0.),
                (TextAlign::Center, 16.),
                (TextAlign::Right, 32.),
            ] {
                window.next_frame.scene.clear();
                line.paint(origin, line_height, align, Some(px(32.)), window, cx)
                    .unwrap();
                let original = window.next_frame.scene.underlines.clone();
                window.next_frame.scene.clear();
                let mut strokes = Vec::new();
                line.paint_with_underline_handler(
                    origin,
                    line_height,
                    align,
                    Some(px(32.)),
                    window,
                    cx,
                    |range, underline, window| {
                        strokes.push((range, underline.origin, underline.width, underline.style));
                        underline.paint(window);
                    },
                )
                .unwrap();
                let end = origin.x + px(offset);
                let start = point(end - half_width, y);
                let width = end - start.x;
                assert_eq!(
                    strokes,
                    [
                        (0..1, start, width, first_style),
                        (1..2, start, width, last_style),
                    ]
                );
                assert_underline_primitives_eq(&window.next_frame.scene.underlines, &original);
            }
        });
    }

    #[gpui::test]
    fn test_underline_handler_matches_wrapped_paint(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let style = UnderlineStyle {
                thickness: px(1.),
                color: Some(black()),
                wavy: true,
            };
            for zero_advance in [false, true] {
                let line = underline_test_line(
                    "abcd",
                    &[DecorationRun {
                        len: 4,
                        color: black(),
                        background_color: None,
                        underline: Some(style),
                        strikethrough: None,
                    }],
                    zero_advance,
                    window,
                );
                let half_width = cx
                    .text_system()
                    .bounding_box(line.runs[0].font_id, line.font_size)
                    .size
                    .width
                    / 2.;
                let origin = point(px(40.25), px(10.25));
                let line_height = px(20.);
                let y = origin.y + underline_y_offset(line_height, line.ascent, line.descent);
                let wrapped = WrappedLine {
                    layout: Arc::new(WrappedLineLayout {
                        unwrapped_layout: line.layout,
                        wrap_boundaries: SmallVec::from_buf([WrapBoundary {
                            run_ix: 0,
                            glyph_ix: 2,
                        }]),
                        wrap_width: Some(px(16.)),
                    }),
                    text: line.text,
                    decoration_runs: line.decoration_runs.into_vec(),
                };
                window.next_frame.scene.clear();
                wrapped
                    .paint(origin, line_height, TextAlign::Left, None, window, cx)
                    .unwrap();
                let original = window.next_frame.scene.underlines.clone();
                window.next_frame.scene.clear();
                let mut strokes = Vec::new();
                paint_line(
                    origin,
                    &wrapped.unwrapped_layout,
                    line_height,
                    TextAlign::Left,
                    Some(px(16.)),
                    &wrapped.decoration_runs,
                    &wrapped.wrap_boundaries,
                    None,
                    window,
                    cx,
                    &mut |range, underline, window| {
                        strokes.push((range, underline.origin, underline.width, underline.style));
                        underline.paint(window);
                    },
                )
                .unwrap();
                let (start, width) = if zero_advance {
                    let start = origin.x - half_width;
                    (start, origin.x - start)
                } else {
                    (origin.x, px(16.))
                };
                assert_eq!(
                    strokes,
                    [
                        (0..4, point(start, y), width, style),
                        (0..4, point(start, y + line_height), width, style),
                    ]
                );
                assert_underline_primitives_eq(&window.next_frame.scene.underlines, &original);
            }
        });
    }

    #[gpui::test]
    fn test_far_glyphs_paint_at_their_shaped_positions(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let glyph_count = 8_192;
            let target = 8_133;
            let character = "é";
            let target_byte = target * character.len();
            let style = UnderlineStyle {
                thickness: px(1.),
                color: Some(black()),
                wavy: false,
            };
            let plain = DecorationRun {
                len: target_byte as u32,
                color: black(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let mut line = underline_test_line(
                &character.repeat(glyph_count),
                &[
                    plain.clone(),
                    DecorationRun {
                        len: character.len() as u32,
                        background_color: Some(black()),
                        underline: Some(style),
                        ..plain.clone()
                    },
                    DecorationRun {
                        len: ((glyph_count - target - 1) * character.len()) as u32,
                        ..plain
                    },
                ],
                false,
                window,
            );
            let layout = Arc::get_mut(&mut line.layout).expect("unshared test layout");
            layout.width = px(glyph_count as f32 * 8.4);
            for (index, glyph) in layout
                .runs
                .iter_mut()
                .flat_map(|run| &mut run.glyphs)
                .enumerate()
            {
                glyph.position.x = px(index as f32 * 8.4);
            }
            let origin = point(px(-68_309.), px(10.));
            let mut strokes = Vec::new();
            line.paint_with_underline_handler(
                origin,
                px(20.),
                TextAlign::Left,
                None,
                window,
                cx,
                |range, underline, _| strokes.push((range, underline.origin.x)),
            )
            .unwrap();
            assert_eq!(
                strokes,
                [(target_byte..target_byte + character.len(), px(8.1953125))]
            );
            window.next_frame.scene.clear();
            line.paint_background(origin, px(20.), TextAlign::Left, None, window, cx)
                .unwrap();
            let scale = window.scale_factor();
            assert_eq!(
                window
                    .next_frame
                    .scene
                    .quads
                    .iter()
                    .map(|quad| (quad.bounds.left().0, quad.bounds.right().0))
                    .collect::<Vec<_>>(),
                [((8.1953125 * scale).round(), (16.59375 * scale).round())]
            );
        });
    }

    #[gpui::test]
    fn test_decorations_follow_carets_not_glyph_origins(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let decorations = [black(), hsla(0., 1., 0.5, 1.)].map(|color| DecorationRun {
                len: 1,
                color,
                background_color: Some(color),
                underline: Some(UnderlineStyle {
                    thickness: px(1.),
                    color: Some(color),
                    wavy: false,
                }),
                strikethrough: Some(StrikethroughStyle {
                    thickness: px(1.),
                    color: Some(color),
                }),
            });
            let mut line = underline_test_line("ab", &decorations, false, window);
            let layout = Arc::get_mut(&mut line.layout).expect("unshared test layout");
            layout.caret_positions = vec![(0, 0.), (1, 8.), (2, 16.)];
            for (glyph, x) in layout
                .runs
                .iter_mut()
                .flat_map(|run| &mut run.glyphs)
                .zip([6., 12.])
            {
                glyph.position.x = px(x);
            }
            for wrapped in [false, true] {
                let boundaries = if wrapped {
                    vec![WrapBoundary {
                        run_ix: 0,
                        glyph_ix: 1,
                    }]
                } else {
                    Vec::new()
                };
                for (align, offset) in [
                    (TextAlign::Left, 0.),
                    (TextAlign::Center, if wrapped { 16. } else { 12. }),
                    (TextAlign::Right, if wrapped { 32. } else { 24. }),
                ] {
                    let origin = point(px(20.), px(10.));
                    let start = origin.x + px(offset);
                    let second_start = start + if wrapped { px(0.) } else { px(8.) };
                    let expected = [
                        (start, start + px(8.)),
                        (second_start, second_start + px(8.)),
                    ];
                    window.next_frame.scene.clear();
                    let mut strokes = Vec::new();
                    paint_line(
                        origin,
                        &line.layout,
                        px(20.),
                        align,
                        Some(px(40.)),
                        &decorations,
                        &boundaries,
                        None,
                        window,
                        cx,
                        &mut |_, underline, _| {
                            strokes
                                .push((underline.origin.x, underline.origin.x + underline.width));
                        },
                    )
                    .unwrap();
                    assert_eq!(strokes, expected);
                    let scale = window.scale_factor();
                    let expected_scaled = expected
                        .map(|(start, end)| (f32::from(start) * scale, f32::from(end) * scale));
                    assert_eq!(
                        window
                            .next_frame
                            .scene
                            .underlines
                            .iter()
                            .map(|stroke| { (stroke.bounds.left().0, stroke.bounds.right().0) })
                            .collect::<Vec<_>>(),
                        expected_scaled,
                    );
                    window.next_frame.scene.clear();
                    paint_line_background(
                        origin,
                        &line.layout,
                        px(20.),
                        align,
                        Some(px(40.)),
                        &decorations,
                        &boundaries,
                        window,
                        cx,
                    )
                    .unwrap();
                    assert_eq!(
                        window
                            .next_frame
                            .scene
                            .quads
                            .iter()
                            .map(|quad| { (quad.bounds.left().0, quad.bounds.right().0) })
                            .collect::<Vec<_>>(),
                        expected_scaled,
                    );
                }
            }
            Arc::get_mut(&mut line.layout)
                .expect("unshared test layout")
                .caret_positions = vec![(0, 0.), (2, 16.)];
            let text_system = cx.text_system().clone();
            let scale = window.scale_factor();
            let atlas = window.platform_window.sprite_atlas();
            let params = (0..SUBPIXEL_VARIANTS_X)
                .map(|phase| RenderGlyphParams {
                    font_id: line.runs[0].font_id,
                    glyph_id: line.runs[0].glyphs[0].id,
                    font_size: line.font_size,
                    subpixel_variant: point(phase, 0),
                    scale_factor: scale,
                    is_emoji: false,
                    subpixel_rendering: false,
                    dilation: text_system.glyph_dilation_for_color(black()),
                })
                .collect::<Vec<_>>();
            for params in &params {
                text_system.raster_bounds.write().insert(
                    params.clone(),
                    Bounds::new(
                        point(DevicePixels(0), DevicePixels((-12. * scale) as i32)),
                        size(
                            DevicePixels((8. * scale) as i32),
                            DevicePixels((16. * scale) as i32),
                        ),
                    ),
                );
                atlas.remove(&AtlasKey::from(params.clone()));
            }
            window.next_frame.scene.clear();
            line.paint(
                point(px(20.), px(10.)),
                px(20.),
                TextAlign::Left,
                None,
                window,
                cx,
            )
            .unwrap();
            assert_eq!(
                window
                    .next_frame
                    .scene
                    .monochrome_sprites
                    .iter()
                    .map(|sprite| sprite.color)
                    .chain(
                        window
                            .next_frame
                            .scene
                            .subpixel_sprites
                            .iter()
                            .map(|sprite| sprite.color)
                    )
                    .collect::<Vec<_>>(),
                decorations.map(|run| run.color.opacity(0.5)),
            );
            for params in &params {
                atlas.remove(&AtlasKey::from(params.clone()));
                text_system.raster_bounds.write().remove(params);
            }
        });
    }

    #[gpui::test]
    fn test_rebased_terminal_decorations(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            let mut line = underline_test_line(
                "abc",
                &[DecorationRun {
                    len: 3,
                    color: black(),
                    background_color: Some(black()),
                    underline: Some(UnderlineStyle::default()),
                    strikethrough: Some(StrikethroughStyle {
                        thickness: px(1.),
                        color: Some(black()),
                    }),
                }],
                false,
                window,
            );
            let layout = Arc::get_mut(&mut line.layout).expect("unshared test layout");
            layout.width -= px(12.);
            for glyph in layout.runs.iter_mut().flat_map(|run| &mut run.glyphs) {
                glyph.position.x -= px(12.);
            }
            for (align, offset) in [
                (TextAlign::Left, 0.),
                (TextAlign::Center, 16.),
                (TextAlign::Right, 32.),
            ] {
                window.next_frame.scene.clear();
                let origin = point(px(20.), px(10.));
                let mut strokes = Vec::new();
                line.paint_with_underline_handler(
                    origin,
                    px(20.),
                    align,
                    Some(px(44.)),
                    window,
                    cx,
                    |_, underline, _| {
                        strokes.push((underline.origin.x, underline.origin.x + underline.width))
                    },
                )
                .unwrap();
                assert_eq!(strokes, [(px(8. + offset), px(32. + offset))]);
                let scale = window.scale_factor();
                assert_eq!(
                    window
                        .next_frame
                        .scene
                        .underlines
                        .iter()
                        .map(|stroke| (stroke.bounds.left().0, stroke.bounds.right().0))
                        .collect::<Vec<_>>(),
                    [((8. + offset) * scale, (32. + offset) * scale)]
                );
                line.paint_background(origin, px(20.), align, Some(px(44.)), window, cx)
                    .unwrap();
                assert_eq!(
                    window
                        .next_frame
                        .scene
                        .quads
                        .iter()
                        .map(|quad| (quad.bounds.left().0, quad.bounds.right().0))
                        .collect::<Vec<_>>(),
                    [(
                        ((8. + offset) * scale).round(),
                        ((32. + offset) * scale).round()
                    )]
                );
            }
        });
    }

    #[gpui::test]
    fn test_rebased_line_scene_order(cx: &mut TestAppContext) {
        test_underline_handler_at_scales(cx, |window, cx| {
            for zero_advance in [false, true] {
                let plain = DecorationRun {
                    len: 1,
                    color: black(),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                let mut line = underline_test_line(
                    "abc",
                    &[
                        plain.clone(),
                        DecorationRun {
                            background_color: Some(black()),
                            underline: Some(UnderlineStyle {
                                thickness: px(1.),
                                color: Some(black()),
                                wavy: false,
                            }),
                            strikethrough: Some(StrikethroughStyle {
                                thickness: px(1.),
                                color: Some(black()),
                            }),
                            ..plain.clone()
                        },
                        plain,
                    ],
                    false,
                    window,
                );
                let layout = Arc::get_mut(&mut line.layout).expect("unshared test layout");
                layout.width = px(12.);
                for (glyph, x) in layout.runs.iter_mut().flat_map(|run| &mut run.glyphs).zip([
                    0.,
                    -12.,
                    if zero_advance { -12. } else { -4. },
                ]) {
                    glyph.position.x = px(x);
                }
                let left = if zero_advance {
                    px(28.)
                        - cx.text_system()
                            .bounding_box(line.runs[0].font_id, line.font_size)
                            .size
                            .width
                            .half()
                } else {
                    px(28.)
                };
                for (align, offset) in [
                    (TextAlign::Left, 0.),
                    (TextAlign::Center, 16.),
                    (TextAlign::Right, 32.),
                ] {
                    window.next_frame.scene.clear();
                    let origin = point(px(40.), px(10.));
                    let line_height = px(20.);
                    line.paint_background(origin, line_height, align, Some(px(44.)), window, cx)
                        .unwrap();
                    line.paint(origin, line_height, align, Some(px(44.)), window, cx)
                        .unwrap();
                    let strokes = window.next_frame.scene.underlines.clone();
                    assert_eq!(strokes.len(), 2);
                    window.paint_quad(fill(
                        Bounds::new(
                            point(left + px(offset + 2.), origin.y),
                            size(px(2.), line_height),
                        ),
                        black(),
                    ));
                    let caret = window.next_frame.scene.quads.last().unwrap();
                    for stroke in strokes {
                        assert!(stroke.bounds.intersects(&caret.bounds));
                        assert!(stroke.order < caret.order);
                    }
                }
            }

            let scale = window.scale_factor();
            let text_system = cx.text_system().clone();
            let params = RenderGlyphParams {
                font_id: FontId(0),
                glyph_id: GlyphId(0),
                font_size: px(16.),
                subpixel_variant: point(0, 0),
                scale_factor: scale,
                is_emoji: false,
                subpixel_rendering: false,
                dilation: text_system.glyph_dilation_for_color(black()),
            };
            let raster_bounds = Bounds::new(
                point(
                    DevicePixels(scale.floor() as i32),
                    DevicePixels((-9. * scale).floor() as i32),
                ),
                size(
                    DevicePixels((6. * scale).ceil() as i32),
                    DevicePixels((16. * scale).ceil() as i32),
                ),
            );
            text_system
                .raster_bounds
                .write()
                .insert(params.clone(), raster_bounds);
            let atlas = window.platform_window.sprite_atlas();
            let key = AtlasKey::from(params.clone());
            atlas.remove(&key);
            for (origin_x, glyph_x, width) in [(20., 0., 8.), (24., -4., 4.)] {
                for glyph_count in [1, 64] {
                    for strikethrough in [false, true] {
                        window.next_frame.scene.clear();
                        let line = make_shaped_line(
                            &"a".repeat(glyph_count),
                            &(0..glyph_count)
                                .map(|index| (index, glyph_x))
                                .collect::<Vec<_>>(),
                            width,
                            &[DecorationRun {
                                len: glyph_count as u32,
                                color: black(),
                                background_color: None,
                                underline: (!strikethrough).then_some(UnderlineStyle {
                                    thickness: px(1.),
                                    color: Some(hsla(0., 1., 0.5, 1.)),
                                    wavy: false,
                                }),
                                strikethrough: strikethrough.then_some(StrikethroughStyle {
                                    thickness: px(1.),
                                    color: Some(hsla(0., 1., 0.5, 1.)),
                                }),
                            }],
                        );
                        let paint = |window: &mut Window, cx: &mut App| {
                            line.paint(
                                point(px(origin_x), px(10.)),
                                px(20.),
                                TextAlign::Left,
                                None,
                                window,
                                cx,
                            )
                            .expect("paint decorated glyphs");
                        };
                        paint(window, cx);
                        let scene = &mut window.next_frame.scene;
                        assert_eq!(scene.monochrome_sprites.len(), glyph_count);
                        assert_eq!(scene.underlines.len(), 1);
                        for sprite in &scene.monochrome_sprites {
                            assert_eq!(
                                sprite.bounds,
                                Bounds::new(
                                    point(
                                        ScaledPixels((21. * scale).floor()),
                                        ScaledPixels((15. * scale).floor()),
                                    ),
                                    size(
                                        ScaledPixels((6. * scale).ceil()),
                                        ScaledPixels((16. * scale).ceil()),
                                    ),
                                ),
                            );
                            assert!(sprite.bounds.intersects(&scene.underlines[0].bounds));
                            assert!(scene.underlines[0].order < sprite.order);
                        }
                        scene.finish();
                        assert_eq!(
                            scene
                                .batches()
                                .map(|batch| match batch {
                                    PrimitiveBatch::Underlines(range) => ("underlines", range),
                                    PrimitiveBatch::MonochromeSprites { range, .. } =>
                                        ("monochrome", range),
                                    _ => panic!("unexpected batch {}", batch.label()),
                                })
                                .collect::<Vec<_>>(),
                            [("underlines", 0..1), ("monochrome", 0..glyph_count)],
                        );
                        scene.clear();
                        window.paint_layer(
                            Bounds::new(point(px(20.), px(10.)), size(px(8.), px(20.))),
                            |window| {
                                for _ in 0..2 {
                                    paint(window, cx);
                                }
                            },
                        );
                        let scene = &mut window.next_frame.scene;
                        assert_eq!(scene.underlines.len(), 2);
                        assert_eq!(scene.monochrome_sprites.len(), glyph_count * 2);
                        assert!(
                            scene.monochrome_sprites[glyph_count - 1].order
                                < scene.underlines[1].order
                        );
                        assert!(
                            scene.underlines[1].order < scene.monochrome_sprites[glyph_count].order
                        );
                        scene.clear();
                    }
                }
            }
            atlas.remove(&key);
            text_system.raster_bounds.write().remove(&params);
        });
    }

    #[test]
    fn test_split_at_invariants() {
        // Split "abcdef" at every possible byte index and verify structural invariants.
        let line = make_shaped_line(
            "abcdef",
            &[
                (0, 0.0),
                (1, 10.0),
                (2, 20.0),
                (3, 30.0),
                (4, 40.0),
                (5, 50.0),
            ],
            60.0,
            &[],
        );

        for i in 0..=6 {
            let (left, right) = line.split_at(i);

            assert_eq!(
                left.width() + right.width(),
                line.width(),
                "widths must sum at split={i}"
            );
            assert_eq!(
                left.len() + right.len(),
                line.len(),
                "lengths must sum at split={i}"
            );
            assert_eq!(
                format!("{}{}", left.text.as_ref(), right.text.as_ref()),
                "abcdef",
                "text must concatenate at split={i}"
            );
            assert_eq!(left.font_size, line.font_size, "font_size at split={i}");
            assert_eq!(right.ascent, line.ascent, "ascent at split={i}");
            assert_eq!(right.descent, line.descent, "descent at split={i}");
        }

        // Edge: split at 0 produces no left runs, full content on right
        let (left, right) = line.split_at(0);
        assert_eq!(left.runs.len(), 0);
        assert_eq!(right.runs[0].glyphs.len(), 6);

        // Edge: split at end produces full content on left, no right runs
        let (left, right) = line.split_at(6);
        assert_eq!(left.runs[0].glyphs.len(), 6);
        assert_eq!(right.runs.len(), 0);
    }

    #[test]
    fn test_split_at_glyph_rebasing() {
        // Two font runs (simulating a font fallback boundary at byte 3):
        //   run A (FontId 0): glyphs at bytes 0,1,2  positions 0,10,20
        //   run B (FontId 1): glyphs at bytes 3,4,5  positions 30,40,50
        // Successive splits simulate the incremental splitting done during wrap.
        let line = ShapedLine {
            layout: Arc::new(LineLayout {
                font_size: px(16.0),
                width: px(60.0),
                ascent: px(12.0),
                descent: px(4.0),
                runs: vec![
                    ShapedRun {
                        font_id: FontId(0),
                        glyphs: vec![
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(0.0), px(0.0)),
                                index: 0,
                                is_emoji: false,
                            },
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(10.0), px(0.0)),
                                index: 1,
                                is_emoji: false,
                            },
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(20.0), px(0.0)),
                                index: 2,
                                is_emoji: false,
                            },
                        ],
                    },
                    ShapedRun {
                        font_id: FontId(1),
                        glyphs: vec![
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(30.0), px(0.0)),
                                index: 3,
                                is_emoji: false,
                            },
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(40.0), px(0.0)),
                                index: 4,
                                is_emoji: false,
                            },
                            ShapedGlyph {
                                id: GlyphId(0),
                                position: point(px(50.0), px(0.0)),
                                index: 5,
                                is_emoji: false,
                            },
                        ],
                    },
                ],
                len: 6,
                caret_positions: Vec::new(),
            }),
            text: "abcdef".into(),
            decoration_runs: SmallVec::new(),
            retained_decorations: None,
        };

        // First split at byte 2 — mid-run in run A
        let (first, remainder) = line.split_at(2);
        assert_eq!(first.text.as_ref(), "ab");
        assert_eq!(first.runs.len(), 1);
        assert_eq!(first.runs[0].font_id, FontId(0));

        // Remainder "cdef" should have two runs: tail of A (1 glyph) + all of B (3 glyphs)
        assert_eq!(remainder.text.as_ref(), "cdef");
        assert_eq!(remainder.runs.len(), 2);
        assert_eq!(remainder.runs[0].font_id, FontId(0));
        assert_eq!(remainder.runs[0].glyphs.len(), 1);
        assert_eq!(remainder.runs[0].glyphs[0].index, 0);
        assert_eq!(remainder.runs[0].glyphs[0].position.x, px(0.0));
        assert_eq!(remainder.runs[1].font_id, FontId(1));
        assert_eq!(remainder.runs[1].glyphs[0].index, 1);
        assert_eq!(remainder.runs[1].glyphs[0].position.x, px(10.0));

        // Second split at byte 2 within remainder — crosses the run boundary
        let (second, final_part) = remainder.split_at(2);
        assert_eq!(second.text.as_ref(), "cd");
        assert_eq!(final_part.text.as_ref(), "ef");
        assert_eq!(final_part.runs[0].glyphs[0].index, 0);
        assert_eq!(final_part.runs[0].glyphs[0].position.x, px(0.0));

        // Widths must sum across all three pieces
        assert_eq!(
            first.width() + second.width() + final_part.width(),
            line.width()
        );
    }

    #[test]
    fn test_split_at_decorations() {
        // Three decoration runs: red [0..2), green [2..5), blue [5..6).
        // Split at byte 3 — red goes entirely left, green straddles, blue goes entirely right.
        let red = Hsla {
            h: 0.0,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let green = Hsla {
            h: 0.3,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };
        let blue = Hsla {
            h: 0.6,
            s: 1.0,
            l: 0.5,
            a: 1.0,
        };

        let line = make_shaped_line(
            "abcdef",
            &[
                (0, 0.0),
                (1, 10.0),
                (2, 20.0),
                (3, 30.0),
                (4, 40.0),
                (5, 50.0),
            ],
            60.0,
            &[
                DecorationRun {
                    len: 2,
                    color: red,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                },
                DecorationRun {
                    len: 3,
                    color: green,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                },
                DecorationRun {
                    len: 1,
                    color: blue,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                },
            ],
        );

        let (left, right) = line.split_at(3);

        // Left: red(2) + green(1) — green straddled, left portion has len 1
        assert_eq!(left.decoration_runs.len(), 2);
        assert_eq!(left.decoration_runs[0].len, 2);
        assert_eq!(left.decoration_runs[0].color, red);
        assert_eq!(left.decoration_runs[1].len, 1);
        assert_eq!(left.decoration_runs[1].color, green);

        // Right: green(2) + blue(1) — green straddled, right portion has len 2
        assert_eq!(right.decoration_runs.len(), 2);
        assert_eq!(right.decoration_runs[0].len, 2);
        assert_eq!(right.decoration_runs[0].color, green);
        assert_eq!(right.decoration_runs[1].len, 1);
        assert_eq!(right.decoration_runs[1].color, blue);
    }

    struct UnderlineHandlerTestView(Rc<dyn Fn(&mut Window, &mut App)>);

    impl Render for UnderlineHandlerTestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let paint = self.0.clone();
            canvas(
                |_, _, _| {},
                move |_, _, window, cx| {
                    window.with_element_opacity(Some(0.5), |window| paint(window, cx));
                },
            )
            .size_full()
        }
    }

    fn test_underline_handler_at_scales(
        cx: &mut TestAppContext,
        paint: impl Fn(&mut Window, &mut App) + 'static,
    ) {
        let window = cx.add_window(move |_, _| UnderlineHandlerTestView(Rc::new(paint)));
        for scale in [1., 1.25, 1.5, 2., 3.] {
            cx.simulate_window_scale_factor_change(window.into(), scale);
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .unwrap();
        }
    }

    fn underline_test_line(
        text: &str,
        decorations: &[DecorationRun],
        zero_advance: bool,
        window: &Window,
    ) -> ShapedLine {
        let mut line = window.text_system().shape_line(
            SharedString::new(text),
            px(16.),
            &[TextRun {
                len: text.len(),
                font: font(".ZedMono"),
                color: black(),
                ..TextRun::default()
            }],
            None,
        );
        line.decoration_runs = SmallVec::from(decorations.to_vec());
        let layout = &line.layout;
        let mut runs = layout.runs.clone();
        let advance = if zero_advance { px(0.) } else { px(8.) };
        let mut width = px(0.);
        for glyph in runs.iter_mut().flat_map(|run| &mut run.glyphs) {
            glyph.position.x = width;
            width += advance;
        }
        line.layout = Arc::new(LineLayout {
            font_size: layout.font_size,
            width,
            ascent: layout.ascent,
            descent: layout.descent,
            runs,
            len: layout.len,
            caret_positions: Vec::new(),
        });
        line
    }

    fn assert_underline_primitives_eq(actual: &[Underline], expected: &[Underline]) {
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected) {
            assert_eq!(actual.bounds, expected.bounds);
            assert_eq!(actual.content_mask, expected.content_mask);
            assert_eq!(actual.color, expected.color);
            assert_eq!(actual.thickness, expected.thickness);
            assert_eq!(actual.wavy, expected.wavy);
            assert_eq!(actual.order, expected.order);
            assert_eq!(actual.pad, expected.pad);
        }
    }

    #[test]
    fn test_cursor_preserves_shaping_metadata_across_runs() {
        let line = ShapedLine {
            layout: Arc::new(LineLayout {
                font_size: px(16.0),
                width: px(50.0),
                ascent: px(12.0),
                descent: px(4.0),
                runs: vec![
                    ShapedRun {
                        font_id: FontId(3),
                        glyphs: vec![
                            ShapedGlyph {
                                id: GlyphId(11),
                                position: point(px(0.0), px(1.0)),
                                index: 0,
                                is_emoji: true,
                            },
                            ShapedGlyph {
                                id: GlyphId(12),
                                position: point(px(17.0), px(1.0)),
                                index: 1,
                                is_emoji: false,
                            },
                            ShapedGlyph {
                                id: GlyphId(13),
                                position: point(px(19.0), px(-1.0)),
                                index: 1,
                                is_emoji: false,
                            },
                        ],
                    },
                    ShapedRun {
                        font_id: FontId(8),
                        glyphs: vec![
                            ShapedGlyph {
                                id: GlyphId(21),
                                position: point(px(25.0), px(1.0)),
                                index: 5,
                                is_emoji: true,
                            },
                            ShapedGlyph {
                                id: GlyphId(22),
                                position: point(px(41.0), px(1.0)),
                                index: 7,
                                is_emoji: false,
                            },
                        ],
                    },
                ],
                len: 10,
                caret_positions: vec![(0, 0.), (1, 10.), (5, 20.), (7, 40.)],
            }),
            text: "a😀bcdef".into(),
            decoration_runs: SmallVec::new(),
            retained_decorations: None,
        };
        let mut cursor = line.cursor();
        let first = cursor.take_until(5);
        assert_eq!(first.text.as_ref(), "a😀");
        assert_eq!(first.runs[0].font_id, FontId(3));
        assert_eq!(first.runs[0].glyphs[0].id, GlyphId(11));
        assert!(first.runs[0].glyphs[0].is_emoji);
        assert_eq!(first.runs[0].glyphs[1].index, 1);
        assert_eq!(first.runs[0].glyphs[1].position, point(px(17.0), px(1.0)));
        assert_eq!(first.runs[0].glyphs.len(), 3);
        assert_eq!(first.runs[0].glyphs[2].index, 1);
        assert_eq!(first.runs[0].glyphs[2].position, point(px(19.0), px(-1.0)));
        assert_eq!(cursor.x_offset(), px(20.0));
        assert_eq!(first.caret_positions, [(0, 0.), (1, 10.), (5, 20.)]);
        let adjusted = line.clone().with_len(5);
        assert_eq!(adjusted.caret_positions, [(0, 0.), (1, 10.), (5, 50.)]);
        assert_eq!(adjusted.width, px(50.));

        let second = cursor.take_until(7);
        assert_eq!(second.text.as_ref(), "bc");
        assert_eq!(second.runs[0].font_id, FontId(8));
        assert_eq!(second.runs[0].glyphs[0].id, GlyphId(21));
        assert_eq!(second.runs[0].glyphs[0].index, 0);
        assert_eq!(second.runs[0].glyphs[0].position, point(px(5.0), px(1.0)));
        assert_eq!(second.caret_positions, [(0, 0.), (2, 20.)]);
        assert_eq!(cursor.x_offset(), px(40.0));

        let final_part = cursor.take_until(10);
        assert_eq!(final_part.text.as_ref(), "def");
        assert_eq!(final_part.runs[0].font_id, FontId(8));
        assert_eq!(final_part.runs[0].glyphs[0].id, GlyphId(22));
        assert_eq!(final_part.runs[0].glyphs[0].index, 0);
        assert_eq!(
            final_part.runs[0].glyphs[0].position,
            point(px(1.0), px(1.0))
        );
        assert_eq!(final_part.caret_positions, [(0, 0.), (3, 10.)]);

        let mut cursor = line.cursor();
        cursor.take_until(8);
        let ligature_tail = cursor.take_until(10);
        assert_eq!(ligature_tail.text.as_ref(), "ef");
        assert_eq!(ligature_tail.caret_positions, [(0, 0.), (2, 0.)]);
        assert_eq!(ligature_tail.width(), px(0.));
        assert_eq!(ligature_tail.x_for_index(0), px(0.));
    }

    #[test]
    fn test_cursor_preserves_precise_terminal_caret() {
        let mut line = make_shaped_line("ab", &[(0, 0.), (1, 16_777_216.)], 16_777_216., &[])
            .with_underline_origins(0., vec![(0..2, 0.)]);
        Arc::get_mut(&mut line.layout)
            .expect("unique layout")
            .caret_positions = vec![(0, 0.), (1, 16_777_216.5), (2, 16_777_217.)];
        let mut cursor = line.cursor();
        cursor.take_until(1);
        let suffix = cursor.take_until(2);
        assert_eq!(suffix.width(), px(0.5));
        assert_eq!(suffix.caret_positions, [(0, 0.), (1, 0.5)]);
        assert_eq!(suffix.closest_index_for_x(px(0.3)), 1);
        assert_eq!(
            suffix
                .retained_decorations
                .as_ref()
                .map(|decorations| decorations.offset_x),
            Some(16_777_216.5)
        );
        let (_, suffix) = line.split_at(1);
        assert_eq!(suffix.width(), px(0.5));
        assert_eq!(suffix.caret_positions, [(0, 0.), (1, 0.5)]);
        assert_eq!(
            suffix
                .retained_decorations
                .as_ref()
                .map(|decorations| decorations.offset_x),
            Some(16_777_216.5)
        );
    }

    #[test]
    fn test_cursor_preserves_existing_visual_order_splitting() {
        let line = make_shaped_line("abc", &[(0, 0.0), (2, 10.0), (1, 20.0)], 30.0, &[]);
        let mut cursor = line.cursor();
        let mut remainder = line.clone();
        let mut previous_boundary = 0;
        for boundary in [0, 1, 2, 3] {
            let (expected, rest) = remainder.split_at(boundary - previous_boundary);
            let actual = cursor.take_until(boundary);
            assert_eq!(actual.text, expected.text);
            assert_eq!(actual.width(), expected.width());
            assert_eq!(actual.runs.len(), expected.runs.len());
            for (actual, expected) in actual.runs.iter().zip(&expected.runs) {
                assert_eq!(actual.font_id, expected.font_id);
                assert_eq!(actual.glyphs.len(), expected.glyphs.len());
                for (actual, expected) in actual.glyphs.iter().zip(&expected.glyphs) {
                    assert_eq!(actual.id, expected.id);
                    assert_eq!(actual.index, expected.index);
                    assert_eq!(actual.position, expected.position);
                }
            }
            assert_eq!(cursor.x_offset(), line.x_for_index(boundary));
            remainder = rest;
            previous_boundary = boundary;
        }
    }

    #[test]
    fn test_cursor_partitions_one_decoration_across_three_chunks() {
        let line = make_shaped_line(
            "abcdef",
            &[
                (0, 0.0),
                (1, 10.0),
                (2, 20.0),
                (3, 30.0),
                (4, 40.0),
                (5, 50.0),
            ],
            60.0,
            &[DecorationRun {
                len: 6,
                color: Hsla {
                    h: 0.2,
                    s: 0.4,
                    l: 0.6,
                    a: 1.0,
                },
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
        );
        let mut cursor = line.cursor();
        assert_eq!(cursor.take_until(2).decoration_runs[0].len, 2);
        assert_eq!(cursor.take_until(4).decoration_runs[0].len, 2);
        assert_eq!(cursor.take_until(6).decoration_runs[0].len, 2);
    }

    #[test]
    fn test_cursor_matches_successive_splits_at_ordered_boundaries() {
        let decorations: Vec<_> = [2, 0, 3, 1]
            .into_iter()
            .map(|len| DecorationRun {
                len,
                color: Hsla {
                    h: len as f32 / 10.0,
                    s: 0.5,
                    l: 0.5,
                    a: 1.0,
                },
                background_color: Some(black()),
                underline: None,
                strikethrough: None,
            })
            .collect();
        let line = make_shaped_line(
            "abcdef",
            &[(0, 5.0), (0, 5.0), (2, 15.0), (4, 25.0), (5, 35.0)],
            45.0,
            &decorations,
        );
        for first in 0..=line.len() {
            for second in first..=line.len() {
                let mut cursor = line.cursor();
                let mut remainder = line.clone();
                let mut previous_boundary = 0;
                let mut total_width = px(0.0);
                let mut text = String::new();
                for boundary in [first, second, line.len(), line.len()] {
                    let (expected, rest) = remainder.split_at(boundary - previous_boundary);
                    let actual = cursor.take_until(boundary);
                    assert_eq!(actual.text, expected.text);
                    assert_eq!(actual.len(), expected.len());
                    assert_eq!(actual.width(), expected.width());
                    assert_eq!(actual.runs.len(), expected.runs.len());
                    for (actual, expected) in actual.runs.iter().zip(&expected.runs) {
                        assert_eq!(actual.font_id, expected.font_id);
                        assert_eq!(actual.glyphs.len(), expected.glyphs.len());
                        for (actual, expected) in actual.glyphs.iter().zip(&expected.glyphs) {
                            assert_eq!(actual.id, expected.id);
                            assert_eq!(actual.index, expected.index);
                            assert_eq!(actual.position, expected.position);
                        }
                    }
                    assert_eq!(actual.decoration_runs.len(), expected.decoration_runs.len());
                    for (actual, expected) in
                        actual.decoration_runs.iter().zip(&expected.decoration_runs)
                    {
                        assert_eq!(actual.len, expected.len);
                        assert_eq!(actual.color, expected.color);
                        assert_eq!(actual.background_color, expected.background_color);
                    }
                    total_width += actual.width();
                    text.push_str(&actual.text);
                    remainder = rest;
                    previous_boundary = boundary;
                }
                assert_eq!(total_width, line.width());
                assert_eq!(text, line.text.as_ref());
            }
        }
    }

    #[test]
    fn test_cursor_empty_chunks_and_repeated_boundaries() {
        let line = make_shaped_line("ab", &[(0, 5.0), (1, 15.0)], 20.0, &[]);
        let mut cursor = line.cursor();
        assert_eq!(cursor.take_until(0).text.as_ref(), "");
        assert_eq!(cursor.take_until(0).text.as_ref(), "");
        assert_eq!(cursor.take_until(1).text.as_ref(), "a");
        assert_eq!(cursor.take_until(2).text.as_ref(), "b");
        assert_eq!(cursor.take_until(2).text.as_ref(), "");
        let empty = make_shaped_line("", &[], 0.0, &[]);
        let piece = empty.cursor().take_until(0);
        assert!(piece.text.is_empty());
        assert!(piece.runs.is_empty());
        assert_eq!(piece.width(), px(0.0));
    }

    #[test]
    fn test_cursor_rejects_invalid_boundaries() {
        let line = make_shaped_line("é", &[(0, 0.0)], 10.0, &[]);
        let mut cursor = line.cursor();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                cursor.take_until(1);
            }))
            .is_err()
        );
        let mut cursor = line.cursor();
        cursor.take_until(2);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                cursor.take_until(0);
            }))
            .is_err()
        );
        let mut cursor = line.cursor();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                cursor.take_until(3);
            }))
            .is_err()
        );
    }
}
