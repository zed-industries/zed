// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use core::{cmp, fmt};

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    render_decoration, Affinity, Align, Attrs, AttrsList, BidiParagraphs, BorrowedWithFontSystem,
    BufferLine, Color, Cursor, DecorationSpan, Ellipsize, FontSystem, Hinting, LayoutCursor,
    LayoutGlyph, LayoutLine, LineEnding, LineIter, Motion, Renderer, Scroll, ShapeLine, Shaping,
    Wrap,
};

bitflags::bitflags! {
    /// Tracks which buffer-wide properties have changed since the last layout.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    struct DirtyFlags: u8 {
        /// Layout caches are stale (wrap, size, metrics, hinting, ellipsize, monospace_width changed)
        const RELAYOUT  = 0b0001;
        /// tab_width changed — lines containing tabs need reshape
        const TAB_SHAPE = 0b0010;
        /// Text was replaced via set_text/set_rich_text — lines are fresh, just need shape_until_scroll
        const TEXT_SET  = 0b0100;
        /// Scroll position changed — visible region may have shifted to unshaped lines
        const SCROLL    = 0b1000;
    }
}

/// A line of visible text for rendering
#[derive(Debug)]
pub struct LayoutRun<'a> {
    /// The index of the original text line
    pub line_i: usize,
    /// The original text line
    pub text: &'a str,
    /// True if the original paragraph direction is RTL
    pub rtl: bool,
    /// The array of layout glyphs to draw
    pub glyphs: &'a [LayoutGlyph],
    /// Text decoration spans covering ranges of glyphs
    pub decorations: &'a [DecorationSpan],
    /// Y offset to baseline of line
    pub line_y: f32,
    /// Y offset to top of line
    pub line_top: f32,
    /// Y offset to next line
    pub line_height: f32,
    /// Width of line
    pub line_w: f32,
}

impl LayoutRun<'_> {
    /// Return an iterator of `(x_left, x_width)` pixel spans for the highlighted areas
    /// between `cursor_start` and `cursor_end` within this run.
    ///
    /// For pure LTR or pure RTL runs this yields at most one span. For mixed BiDi runs
    /// (where selected and unselected glyphs interleave visually) it yields multiple
    /// disjoint spans.
    ///
    /// Returns an empty iterator if the cursor range does not intersect this run.
    pub fn highlight(
        &self,
        cursor_start: Cursor,
        cursor_end: Cursor,
    ) -> impl Iterator<Item = (f32, f32)> {
        let line_i = self.line_i;
        let mut results = Vec::new();
        let mut range_opt: Option<(f32, f32)> = None;

        for glyph in self.glyphs {
            let cluster = &self.text[glyph.start..glyph.end];
            let total = cluster.grapheme_indices(true).count().max(1);
            let c_w = glyph.w / total as f32;
            let mut c_x = glyph.x;

            for (i, c) in cluster.grapheme_indices(true) {
                let c_start = glyph.start + i;
                let c_end = glyph.start + i + c.len();

                let is_selected = (cursor_start.line != line_i || c_end > cursor_start.index)
                    && (cursor_end.line != line_i || c_start < cursor_end.index);

                if is_selected {
                    range_opt = Some(match range_opt {
                        Some((min, max)) => (min.min(c_x), max.max(c_x + c_w)),
                        None => (c_x, c_x + c_w),
                    });
                } else if let Some((min_x, max_x)) = range_opt.take() {
                    let width = max_x - min_x;
                    if width > 0.0 {
                        results.push((min_x, width));
                    }
                }

                c_x += c_w;
            }
        }

        // Flush remaining highlighted region
        if let Some((min_x, max_x)) = range_opt {
            let width = max_x - min_x;
            if width > 0.0 {
                results.push((min_x, width));
            }
        }

        results.into_iter()
    }

    /// Returns the visual x position (in pixels) of `cursor` within this run,
    /// or `None` if the cursor does not belong to this run.
    ///
    /// For RTL glyphs the cursor is placed at the right edge minus the offset;
    /// for LTR glyphs it is placed at the left edge plus the offset.
    pub fn cursor_position(&self, cursor: &Cursor) -> Option<f32> {
        let (glyph_idx, glyph_offset) = self.cursor_glyph(cursor)?;
        let x = self.glyphs.get(glyph_idx).map_or_else(
            || {
                // Past-the-end: position after the last glyph
                self.glyphs.last().map_or(0.0, |glyph| {
                    if glyph.level.is_rtl() {
                        glyph.x
                    } else {
                        glyph.x + glyph.w
                    }
                })
            },
            |glyph| {
                if glyph.level.is_rtl() {
                    glyph.x + glyph.w - glyph_offset
                } else {
                    glyph.x + glyph_offset
                }
            },
        );
        Some(x)
    }

    /// Find which glyph in this run contains `cursor`, returning
    /// `(glyph_index, pixel_offset_within_glyph)`, or `None` if the cursor
    /// is not on this run.
    pub fn cursor_glyph(&self, cursor: &Cursor) -> Option<(usize, f32)> {
        if cursor.line != self.line_i {
            return None;
        }
        for (glyph_i, glyph) in self.glyphs.iter().enumerate() {
            if cursor.index == glyph.start {
                return Some((glyph_i, 0.0));
            } else if cursor.index > glyph.start && cursor.index < glyph.end {
                // Guess x offset based on graphemes within the cluster
                let cluster = &self.text[glyph.start..glyph.end];
                let mut before = 0;
                let mut total = 0;
                for (i, _) in cluster.grapheme_indices(true) {
                    if glyph.start + i < cursor.index {
                        before += 1;
                    }
                    total += 1;
                }
                let offset = glyph.w * (before as f32) / (total as f32);
                return Some((glyph_i, offset));
            }
        }
        // in mixed BiDi the last logical glyph may not be the last visual glyph.
        for (glyph_i, glyph) in self.glyphs.iter().enumerate() {
            if cursor.index == glyph.end {
                return Some((glyph_i, glyph.w));
            }
        }
        if self.glyphs.is_empty() {
            return Some((0, 0.0));
        }
        None
    }

    /// Get the left-edge cursor position of a glyph, accounting for paragraph direction.
    pub const fn cursor_from_glyph_left(&self, glyph: &LayoutGlyph) -> Cursor {
        if self.rtl {
            Cursor::new_with_affinity(self.line_i, glyph.end, Affinity::Before)
        } else {
            Cursor::new_with_affinity(self.line_i, glyph.start, Affinity::After)
        }
    }

    /// Get the right-edge cursor position of a glyph, accounting for paragraph direction.
    pub const fn cursor_from_glyph_right(&self, glyph: &LayoutGlyph) -> Cursor {
        if self.rtl {
            Cursor::new_with_affinity(self.line_i, glyph.start, Affinity::After)
        } else {
            Cursor::new_with_affinity(self.line_i, glyph.end, Affinity::Before)
        }
    }
}

/// An iterator of visible text lines, see [`LayoutRun`]
#[derive(Debug)]
pub struct LayoutRunIter<'b> {
    lines: &'b [BufferLine],
    height_opt: Option<f32>,
    line_height: f32,
    scroll: f32,
    line_i: usize,
    layout_i: usize,
    total_height: f32,
    line_top: f32,
}

impl<'b> LayoutRunIter<'b> {
    pub const fn new(buffer: &'b Buffer) -> Self {
        Self::from_lines(
            buffer.lines.as_slice(),
            buffer.height_opt,
            buffer.metrics.line_height,
            buffer.scroll.vertical,
            buffer.scroll.line,
        )
    }

    pub const fn from_lines(
        lines: &'b [BufferLine],
        height_opt: Option<f32>,
        line_height: f32,
        scroll: f32,
        start: usize,
    ) -> Self {
        Self {
            lines,
            height_opt,
            line_height,
            scroll,
            line_i: start,
            layout_i: 0,
            total_height: 0.0,
            line_top: 0.0,
        }
    }
}

impl<'b> Iterator for LayoutRunIter<'b> {
    type Item = LayoutRun<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.lines.get(self.line_i) {
            let shape = line.shape_opt()?;
            let layout = line.layout_opt()?;
            while let Some(layout_line) = layout.get(self.layout_i) {
                self.layout_i += 1;

                let line_height = layout_line.line_height_opt.unwrap_or(self.line_height);
                self.total_height += line_height;

                let line_top = self.line_top - self.scroll;
                let glyph_height = layout_line.max_ascent + layout_line.max_descent;
                let centering_offset = (line_height - glyph_height) / 2.0;
                let line_y = line_top + centering_offset + layout_line.max_ascent;
                if let Some(height) = self.height_opt {
                    if line_y - layout_line.max_ascent > height {
                        return None;
                    }
                }
                self.line_top += line_height;
                if line_y + layout_line.max_descent < 0.0 {
                    continue;
                }

                return Some(LayoutRun {
                    line_i: self.line_i,
                    text: line.text(),
                    rtl: shape.rtl,
                    glyphs: &layout_line.glyphs,
                    decorations: &layout_line.decorations,
                    line_y,
                    line_top,
                    line_height,
                    line_w: layout_line.w,
                });
            }
            self.line_i += 1;
            self.layout_i = 0;
        }

        None
    }
}

/// Metrics of text
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metrics {
    /// Font size in pixels
    pub font_size: f32,
    /// Line height in pixels
    pub line_height: f32,
}

impl Metrics {
    /// Create metrics with given font size and line height
    pub const fn new(font_size: f32, line_height: f32) -> Self {
        Self {
            font_size,
            line_height,
        }
    }

    /// Create metrics with given font size and calculate line height using relative scale
    pub fn relative(font_size: f32, line_height_scale: f32) -> Self {
        Self {
            font_size,
            line_height: font_size * line_height_scale,
        }
    }

    /// Scale font size and line height
    pub fn scale(self, scale: f32) -> Self {
        Self {
            font_size: self.font_size * scale,
            line_height: self.line_height * scale,
        }
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}px / {}px", self.font_size, self.line_height)
    }
}

/// A buffer of text that is shaped and laid out
#[derive(Debug)]
pub struct Buffer {
    /// [`BufferLine`]s (or paragraphs) of text in the buffer
    pub lines: Vec<BufferLine>,
    metrics: Metrics,
    width_opt: Option<f32>,
    height_opt: Option<f32>,
    scroll: Scroll,
    /// True if a redraw is requires. Set to false after processing
    redraw: bool,
    wrap: Wrap,
    ellipsize: Ellipsize,
    monospace_width: Option<f32>,
    tab_width: u16,
    hinting: Hinting,
    /// Dirty flags tracking which properties changed since last layout
    dirty: DirtyFlags,
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self {
            lines: self.lines.clone(),
            metrics: self.metrics,
            width_opt: self.width_opt,
            height_opt: self.height_opt,
            scroll: self.scroll,
            redraw: self.redraw,
            wrap: self.wrap,
            ellipsize: self.ellipsize,
            monospace_width: self.monospace_width,
            tab_width: self.tab_width,
            hinting: self.hinting,
            dirty: self.dirty,
        }
    }
}

impl Buffer {
    /// Create an empty [`Buffer`] with the provided [`Metrics`].
    /// This is useful for initializing a [`Buffer`] without a [`FontSystem`].
    ///
    /// You must populate the [`Buffer`] with at least one [`BufferLine`] before shaping and layout,
    /// for example by calling [`Buffer::set_text`].
    ///
    /// If you have a [`FontSystem`] in scope, you should use [`Buffer::new`] instead.
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.line_height` is zero.
    pub fn new_empty(metrics: Metrics) -> Self {
        assert_ne!(metrics.line_height, 0.0, "line height cannot be 0");
        Self {
            lines: Vec::new(),
            metrics,
            width_opt: None,
            height_opt: None,
            scroll: Scroll::default(),
            redraw: false,
            wrap: Wrap::WordOrGlyph,
            ellipsize: Ellipsize::None,
            monospace_width: None,
            tab_width: 8,
            hinting: Hinting::default(),
            dirty: DirtyFlags::empty(),
        }
    }

    /// Create a new [`Buffer`] with the provided [`FontSystem`] and [`Metrics`]
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.line_height` is zero.
    pub fn new(font_system: &mut FontSystem, metrics: Metrics) -> Self {
        let mut buffer = Self::new_empty(metrics);
        buffer.set_text("", &Attrs::new(), Shaping::Advanced, None);
        buffer.shape_until_scroll(font_system, false);
        buffer
    }

    /// Mutably borrows the buffer together with an [`FontSystem`] for more convenient methods
    pub fn borrow_with<'a>(
        &'a mut self,
        font_system: &'a mut FontSystem,
    ) -> BorrowedWithFontSystem<'a, Self> {
        BorrowedWithFontSystem {
            inner: self,
            font_system,
        }
    }

    /// Process dirty flags: invalidate shape/layout caches as needed, then clear flags.
    /// Returns `true` if any flags were set (i.e., work may be needed).
    fn resolve_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        if dirty.is_empty() {
            // individual lines may have been externally invalidated
            if self.lines.iter().any(|line| line.needs_reshaping()) {
                self.redraw = true;
                return true;
            }
            return false;
        }

        if dirty.contains(DirtyFlags::TEXT_SET) {
            // Lines were replaced — already fresh, no cache to invalidate.
        } else {
            if dirty.contains(DirtyFlags::TAB_SHAPE) {
                for line in &mut self.lines {
                    if line.shape_opt().is_some() && line.text().contains('\t') {
                        line.reset_shaping();
                    }
                }
            }
            if dirty.contains(DirtyFlags::RELAYOUT) {
                for line in &mut self.lines {
                    if line.shape_opt().is_some() {
                        line.reset_layout();
                    }
                }
            }
        }

        self.redraw = true;
        self.dirty = DirtyFlags::empty();
        true
    }

    /// Shape lines until cursor, also scrolling to include cursor in view
    #[allow(clippy::missing_panics_doc)]
    pub fn shape_until_cursor(
        &mut self,
        font_system: &mut FontSystem,
        cursor: Cursor,
        prune: bool,
    ) {
        self.shape_until_scroll(font_system, prune);
        let metrics = self.metrics;
        let old_scroll = self.scroll;

        let layout_cursor = self
            .layout_cursor(font_system, cursor)
            .expect("shape_until_cursor invalid cursor");

        let mut layout_y = 0.0;
        let mut total_height = {
            let layout = self
                .line_layout(font_system, layout_cursor.line)
                .expect("shape_until_cursor failed to scroll forwards");
            (0..layout_cursor.layout).for_each(|layout_i| {
                layout_y += layout[layout_i]
                    .line_height_opt
                    .unwrap_or(metrics.line_height);
            });
            layout_y
                + layout[layout_cursor.layout]
                    .line_height_opt
                    .unwrap_or(metrics.line_height)
        };

        if self.scroll.line > layout_cursor.line
            || (self.scroll.line == layout_cursor.line && self.scroll.vertical > layout_y)
        {
            // Adjust scroll backwards if cursor is before it
            self.scroll.line = layout_cursor.line;
            self.scroll.vertical = layout_y;
        } else if let Some(height) = self.height_opt {
            // Adjust scroll forwards if cursor is after it
            let mut line_i = layout_cursor.line;
            if line_i <= self.scroll.line {
                // This is a single line that may wrap
                if total_height > height + self.scroll.vertical {
                    self.scroll.vertical = total_height - height;
                }
            } else {
                while line_i > self.scroll.line {
                    line_i -= 1;
                    let layout = self
                        .line_layout(font_system, line_i)
                        .expect("shape_until_cursor failed to scroll forwards");
                    for layout_line in layout {
                        total_height += layout_line.line_height_opt.unwrap_or(metrics.line_height);
                    }
                    if total_height > height + self.scroll.vertical {
                        self.scroll.line = line_i;
                        self.scroll.vertical = total_height - height;
                    }
                }
            }
        }

        if old_scroll != self.scroll {
            self.dirty |= DirtyFlags::SCROLL;
        }

        self.shape_until_scroll(font_system, prune);

        // Adjust horizontal scroll to include cursor
        if let Some(layout_cursor) = self.layout_cursor(font_system, cursor) {
            if let Some(layout_lines) = self.line_layout(font_system, layout_cursor.line) {
                if let Some(layout_line) = layout_lines.get(layout_cursor.layout) {
                    let (x_min, x_max) = layout_line
                        .glyphs
                        .get(layout_cursor.glyph)
                        .or_else(|| layout_line.glyphs.last())
                        .map_or((0.0, 0.0), |glyph| {
                            //TODO: use code from cursor_glyph_opt?
                            let x_a = glyph.x;
                            let x_b = glyph.x + glyph.w;
                            (x_a.min(x_b), x_a.max(x_b))
                        });
                    if x_min < self.scroll.horizontal {
                        self.scroll.horizontal = x_min;
                        self.redraw = true;
                    }
                    if let Some(width) = self.width_opt {
                        if x_max > self.scroll.horizontal + width {
                            self.scroll.horizontal = x_max - width;
                            self.redraw = true;
                        }
                    }
                }
            }
        }
    }

    /// Shape lines until scroll, resolving any pending dirty state first.
    ///
    /// This processes dirty flags (invalidating caches for lines that need
    /// reshaping or relayout) and then shapes/layouts visible lines.
    ///
    /// Call this before reading layout results via [`layout_runs`] or [`hit`]
    /// when working with the `Buffer` directly. The [`BorrowedWithFontSystem`]
    /// wrapper calls this automatically.
    ///
    /// [`layout_runs`]: Self::layout_runs
    /// [`hit`]: Self::hit
    #[allow(clippy::missing_panics_doc)]
    pub fn shape_until_scroll(&mut self, font_system: &mut FontSystem, prune: bool) {
        if !self.resolve_dirty() {
            return;
        }
        let metrics = self.metrics;

        // Clamp scroll.line to valid range (lines may have been removed by editing)
        if self.scroll.line >= self.lines.len() {
            self.scroll.line = self.lines.len().saturating_sub(1);
            self.scroll.vertical = 0.0;
        }

        let old_scroll = self.scroll;

        loop {
            // Adjust scroll.layout to be positive by moving scroll.line backwards
            while self.scroll.vertical < 0.0 {
                if self.scroll.line > 0 {
                    let line_i = self.scroll.line - 1;
                    if let Some(layout) = self.line_layout(font_system, line_i) {
                        let mut layout_height = 0.0;
                        for layout_line in layout {
                            layout_height +=
                                layout_line.line_height_opt.unwrap_or(metrics.line_height);
                        }
                        self.scroll.line = line_i;
                        self.scroll.vertical += layout_height;
                    } else {
                        // If layout is missing, just assume line height
                        self.scroll.line = line_i;
                        self.scroll.vertical += metrics.line_height;
                    }
                } else {
                    self.scroll.vertical = 0.0;
                    break;
                }
            }

            let scroll_start = self.scroll.vertical;
            let scroll_end = scroll_start + self.height_opt.unwrap_or(f32::INFINITY);

            if prune {
                for line_i in 0..self.scroll.line {
                    self.lines[line_i].reset_shaping();
                }
            }
            let mut total_height = 0.0;
            for line_i in self.scroll.line..self.lines.len() {
                if total_height > scroll_end {
                    if prune {
                        self.lines[line_i].reset_shaping();
                        continue;
                    }
                    break;
                }

                let mut layout_height = 0.0;
                let layout = self
                    .line_layout(font_system, line_i)
                    .expect("shape_until_scroll invalid line");
                for layout_line in layout {
                    let line_height = layout_line.line_height_opt.unwrap_or(metrics.line_height);
                    layout_height += line_height;
                    total_height += line_height;
                }

                // Adjust scroll.vertical to be smaller by moving scroll.line forwards
                if line_i == self.scroll.line && layout_height <= self.scroll.vertical {
                    self.scroll.line += 1;
                    self.scroll.vertical -= layout_height;
                }
            }

            if total_height < scroll_end && self.scroll.line > 0 {
                // Need to scroll up to stay inside of buffer
                self.scroll.vertical -= scroll_end - total_height;
            } else {
                // Done adjusting scroll
                break;
            }
        }

        if old_scroll != self.scroll {
            self.redraw = true;
        }
    }

    /// Convert a [`Cursor`] to a [`LayoutCursor`]
    pub fn layout_cursor(
        &mut self,
        font_system: &mut FontSystem,
        cursor: Cursor,
    ) -> Option<LayoutCursor> {
        let layout = self.line_layout(font_system, cursor.line)?;
        for (layout_i, layout_line) in layout.iter().enumerate() {
            for (glyph_i, glyph) in layout_line.glyphs.iter().enumerate() {
                let cursor_end =
                    Cursor::new_with_affinity(cursor.line, glyph.end, Affinity::Before);
                let cursor_start =
                    Cursor::new_with_affinity(cursor.line, glyph.start, Affinity::After);
                let (cursor_left, cursor_right) = if glyph.level.is_ltr() {
                    (cursor_start, cursor_end)
                } else {
                    (cursor_end, cursor_start)
                };
                if cursor == cursor_left {
                    return Some(LayoutCursor::new(cursor.line, layout_i, glyph_i));
                }
                if cursor == cursor_right {
                    return Some(LayoutCursor::new(cursor.line, layout_i, glyph_i + 1));
                }
            }
        }

        // Fall back to start of line
        //TODO: should this be the end of the line?
        Some(LayoutCursor::new(cursor.line, 0, 0))
    }

    /// Shape the provided line index and return the result
    pub fn line_shape(
        &mut self,
        font_system: &mut FontSystem,
        line_i: usize,
    ) -> Option<&ShapeLine> {
        let line = self.lines.get_mut(line_i)?;
        Some(line.shape(font_system, self.tab_width))
    }

    /// Lay out the provided line index and return the result
    pub fn line_layout(
        &mut self,
        font_system: &mut FontSystem,
        line_i: usize,
    ) -> Option<&[LayoutLine]> {
        let line = self.lines.get_mut(line_i)?;
        Some(line.layout(
            font_system,
            self.metrics.font_size,
            self.width_opt,
            self.wrap,
            self.ellipsize,
            self.monospace_width,
            self.tab_width,
            self.hinting,
        ))
    }

    /// Get the current [`Metrics`]
    pub const fn metrics(&self) -> Metrics {
        self.metrics
    }

    /// Set the current [`Metrics`].
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.font_size` is zero.
    pub fn set_metrics(&mut self, metrics: Metrics) {
        if metrics != self.metrics {
            assert_ne!(metrics.font_size, 0.0, "font size cannot be 0");
            assert_ne!(metrics.line_height, 0.0, "line height cannot be 0");
            self.metrics = metrics;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current [`Hinting`] strategy.
    pub const fn hinting(&self) -> Hinting {
        self.hinting
    }

    /// Set the current [`Hinting`] strategy.
    pub fn set_hinting(&mut self, hinting: Hinting) {
        if hinting != self.hinting {
            self.hinting = hinting;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current [`Wrap`]
    pub const fn wrap(&self) -> Wrap {
        self.wrap
    }

    /// Set the current [`Wrap`].
    pub fn set_wrap(&mut self, wrap: Wrap) {
        if wrap != self.wrap {
            self.wrap = wrap;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current [`Ellipsize`]
    pub const fn ellipsize(&self) -> Ellipsize {
        self.ellipsize
    }

    /// Set the current [`Ellipsize`].
    pub fn set_ellipsize(&mut self, ellipsize: Ellipsize) {
        if ellipsize != self.ellipsize {
            self.ellipsize = ellipsize;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current `monospace_width`
    pub const fn monospace_width(&self) -> Option<f32> {
        self.monospace_width
    }

    /// Set monospace width monospace glyphs should be resized to match. `None` means don't resize.
    pub fn set_monospace_width(&mut self, monospace_width: Option<f32>) {
        if monospace_width != self.monospace_width {
            self.monospace_width = monospace_width;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current `tab_width`
    pub const fn tab_width(&self) -> u16 {
        self.tab_width
    }

    /// Set tab width (number of spaces between tab stops).
    pub fn set_tab_width(&mut self, tab_width: u16) {
        if tab_width == 0 {
            return;
        }
        if tab_width != self.tab_width {
            self.tab_width = tab_width;
            self.dirty |= DirtyFlags::TAB_SHAPE | DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Get the current buffer dimensions (width, height)
    pub const fn size(&self) -> (Option<f32>, Option<f32>) {
        (self.width_opt, self.height_opt)
    }

    /// Set the current buffer dimensions.
    pub fn set_size(&mut self, width_opt: Option<f32>, height_opt: Option<f32>) {
        let width_clamped = width_opt.map(|v| v.max(0.0));
        let height_clamped = height_opt.map(|v| v.max(0.0));
        if width_clamped != self.width_opt {
            self.width_opt = width_clamped;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
        if height_clamped != self.height_opt {
            self.height_opt = height_clamped;
            self.dirty |= DirtyFlags::RELAYOUT;
            self.redraw = true;
        }
    }

    /// Set the current [`Metrics`] and buffer dimensions at the same time.
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.font_size` is zero.
    pub fn set_metrics_and_size(
        &mut self,
        metrics: Metrics,
        width_opt: Option<f32>,
        height_opt: Option<f32>,
    ) {
        self.set_metrics(metrics);
        self.set_size(width_opt, height_opt);
    }

    /// Get the current scroll location
    pub const fn scroll(&self) -> Scroll {
        self.scroll
    }

    /// Set the current scroll location
    pub fn set_scroll(&mut self, scroll: Scroll) {
        if scroll != self.scroll {
            self.scroll = scroll;
            self.dirty |= DirtyFlags::SCROLL;
            self.redraw = true;
        }
    }

    /// Internal: set text of buffer, reusing existing line allocations.
    ///
    /// Does NOT call `shape_until_scroll` — the caller is responsible for that.
    fn set_text_impl(
        &mut self,
        text: &str,
        attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) {
        let mut line_count = 0;
        for (range, ending) in LineIter::new(text) {
            let line_text = &text[range];
            if line_count < self.lines.len() {
                // Reuse existing line: reclaim String/AttrsList allocations
                let mut reused_text = self.lines[line_count].reclaim_text();
                reused_text.push_str(line_text);
                let reused_attrs = self.lines[line_count].reclaim_attrs().reset(attrs);
                self.lines[line_count].reset_new(reused_text, ending, reused_attrs, shaping);
            } else {
                self.lines.push(BufferLine::new(
                    line_text,
                    ending,
                    AttrsList::new(attrs),
                    shaping,
                ));
            }
            line_count += 1;
        }

        // Ensure there is an ending line with no line ending.
        // When no lines were produced (empty text), unwrap_or_default() returns
        // LineEnding::Lf (the Default), which is != None, so we add an empty line.
        let last_ending = if line_count > 0 {
            self.lines[line_count - 1].ending()
        } else {
            LineEnding::default()
        };
        if last_ending != LineEnding::None {
            if line_count < self.lines.len() {
                let reused_text = self.lines[line_count].reclaim_text();
                let reused_attrs = self.lines[line_count].reclaim_attrs().reset(attrs);
                self.lines[line_count].reset_new(
                    reused_text,
                    LineEnding::None,
                    reused_attrs,
                    shaping,
                );
            } else {
                self.lines.push(BufferLine::new(
                    "",
                    LineEnding::None,
                    AttrsList::new(attrs),
                    shaping,
                ));
            }
            line_count += 1;
        }

        // Discard excess lines now that we have reused as much of the existing allocations as possible.
        self.lines.truncate(line_count);

        if alignment.is_some() {
            self.lines.iter_mut().for_each(|line| {
                line.set_align(alignment);
            });
        }

        self.scroll = Scroll::default();
    }

    /// Set text of buffer, using provided attributes for each line by default.
    pub fn set_text(
        &mut self,
        text: &str,
        attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) {
        self.set_text_impl(text, attrs, shaping, alignment);
        self.dirty |= DirtyFlags::TEXT_SET;
        self.redraw = true;
    }

    /// Internal: set rich text of buffer, reusing existing line allocations.
    ///
    /// Does NOT call `shape_until_scroll` — the caller is responsible for that.
    fn set_rich_text_impl<'r, 's, I>(
        &mut self,
        spans: I,
        default_attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        let mut end = 0;
        // TODO: find a way to cache this string and vec for reuse
        let (string, spans_data): (String, Vec<_>) = spans
            .into_iter()
            .map(|(s, attrs)| {
                let start = end;
                end += s.len();
                (s, (attrs, start..end))
            })
            .unzip();

        let mut spans_iter = spans_data.into_iter();
        let mut maybe_span = spans_iter.next();

        // split the string into lines, as ranges
        let string_start = string.as_ptr() as usize;
        let mut lines_iter = BidiParagraphs::new(&string).map(|line: &str| {
            let start = line.as_ptr() as usize - string_start;
            let end = start + line.len();
            start..end
        });
        let mut maybe_line = lines_iter.next();
        //TODO: set this based on information from spans
        let line_ending = LineEnding::default();

        let mut line_count = 0;
        let mut attrs_list = self
            .lines
            .get_mut(line_count)
            .map_or_else(|| AttrsList::new(&Attrs::new()), BufferLine::reclaim_attrs)
            .reset(default_attrs);
        let mut line_string = self
            .lines
            .get_mut(line_count)
            .map(BufferLine::reclaim_text)
            .unwrap_or_default();

        loop {
            let (Some(line_range), Some((attrs, span_range))) = (&maybe_line, &maybe_span) else {
                // this is reached only if this text is empty
                if self.lines.len() == line_count {
                    self.lines.push(BufferLine::empty());
                }
                self.lines[line_count].reset_new(
                    String::new(),
                    line_ending,
                    AttrsList::new(default_attrs),
                    shaping,
                );
                line_count += 1;
                break;
            };

            // start..end is the intersection of this line and this span
            let start = line_range.start.max(span_range.start);
            let end = line_range.end.min(span_range.end);
            if start < end {
                let text = &string[start..end];
                let text_start = line_string.len();
                line_string.push_str(text);
                let text_end = line_string.len();
                // Only add attrs if they don't match the defaults
                if *attrs != attrs_list.defaults() {
                    attrs_list.add_span(text_start..text_end, attrs);
                }
            } else if line_string.is_empty() && attrs.metrics_opt.is_some() {
                // reset the attrs list with the span's attrs so the line height
                // matches the span's font size rather than falling back to
                // the buffer default
                attrs_list = attrs_list.reset(attrs);
            }

            // we know that at the end of a line,
            // span text's end index is always >= line text's end index
            // so if this span ends before this line ends,
            // there is another span in this line.
            // otherwise, we move on to the next line.
            if span_range.end < line_range.end {
                maybe_span = spans_iter.next();
            } else {
                maybe_line = lines_iter.next();
                if maybe_line.is_some() {
                    // finalize this line and start a new line
                    let next_attrs_list = self
                        .lines
                        .get_mut(line_count + 1)
                        .map_or_else(|| AttrsList::new(&Attrs::new()), BufferLine::reclaim_attrs)
                        .reset(default_attrs);
                    let next_line_string = self
                        .lines
                        .get_mut(line_count + 1)
                        .map(BufferLine::reclaim_text)
                        .unwrap_or_default();
                    let prev_attrs_list = core::mem::replace(&mut attrs_list, next_attrs_list);
                    let prev_line_string = core::mem::replace(&mut line_string, next_line_string);
                    if self.lines.len() == line_count {
                        self.lines.push(BufferLine::empty());
                    }
                    self.lines[line_count].reset_new(
                        prev_line_string,
                        line_ending,
                        prev_attrs_list,
                        shaping,
                    );
                    line_count += 1;
                } else {
                    // finalize the final line
                    if self.lines.len() == line_count {
                        self.lines.push(BufferLine::empty());
                    }
                    self.lines[line_count].reset_new(line_string, line_ending, attrs_list, shaping);
                    line_count += 1;
                    break;
                }
            }
        }

        // Discard excess lines now that we have reused as much of the existing allocations as possible.
        self.lines.truncate(line_count);

        self.lines.iter_mut().for_each(|line| {
            line.set_align(alignment);
        });

        self.scroll = Scroll::default();
    }

    /// Set text of buffer, using an iterator of styled spans (pairs of text and attributes).
    ///
    /// ```
    /// # use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
    /// # let mut font_system = FontSystem::new();
    /// let mut buffer = Buffer::new_empty(Metrics::new(32.0, 44.0));
    /// let attrs = Attrs::new().family(Family::Serif);
    /// buffer.set_rich_text(
    ///     [
    ///         ("hello, ", attrs.clone()),
    ///         ("cosmic\ntext", attrs.clone().family(Family::Monospace)),
    ///     ],
    ///     &attrs,
    ///     Shaping::Advanced,
    ///     None,
    /// );
    /// ```
    pub fn set_rich_text<'r, 's, I>(
        &mut self,
        spans: I,
        default_attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        self.set_rich_text_impl(spans, default_attrs, shaping, alignment);
        self.dirty |= DirtyFlags::TEXT_SET;
        self.redraw = true;
    }

    /// True if a redraw is needed
    pub const fn redraw(&self) -> bool {
        self.redraw
    }

    /// Set redraw needed flag
    pub fn set_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }

    /// Get the visible layout runs for rendering and other tasks.
    ///
    /// This returns an iterator over the laid-out runs that are visible in the
    /// current scroll region. Call [`shape_until_scroll`] first to ensure the buffer
    /// is up to date, or use [`BorrowedWithFontSystem`] which calls it
    /// automatically.
    ///
    /// [`shape_until_scroll`]: Self::shape_until_scroll
    pub fn layout_runs(&self) -> LayoutRunIter<'_> {
        LayoutRunIter::new(self)
    }

    /// Convert x, y position to Cursor (hit detection).
    ///
    /// Call [`shape_until_scroll`] first to ensure the buffer is up to date,
    /// or use [`BorrowedWithFontSystem`] which calls it automatically.
    ///
    /// [`shape_until_scroll`]: Self::shape_until_scroll
    pub fn hit(&self, x: f32, y: f32) -> Option<Cursor> {
        #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
        let instant = std::time::Instant::now();

        let mut new_cursor_opt = None;

        let mut runs = self.layout_runs().peekable();
        let mut first_run = true;
        while let Some(run) = runs.next() {
            let line_top = run.line_top;
            let line_height = run.line_height;

            if first_run && y < line_top {
                first_run = false;
                let new_cursor = Cursor::new(run.line_i, 0);
                new_cursor_opt = Some(new_cursor);
            } else if y >= line_top && y < line_top + line_height {
                let mut new_cursor_glyph = run.glyphs.len();
                let mut new_cursor_char = 0;
                let mut new_cursor_affinity = Affinity::After;

                let mut first_glyph = true;

                'hit: for (glyph_i, glyph) in run.glyphs.iter().enumerate() {
                    if first_glyph {
                        first_glyph = false;
                        if (run.rtl && x > glyph.x) || (!run.rtl && x < 0.0) {
                            new_cursor_glyph = 0;
                            new_cursor_char = 0;
                        }
                    }
                    if x >= glyph.x && x <= glyph.x + glyph.w {
                        new_cursor_glyph = glyph_i;

                        let cluster = &run.text[glyph.start..glyph.end];
                        let total = cluster.grapheme_indices(true).count();
                        let mut egc_x = glyph.x;
                        let egc_w = glyph.w / (total as f32);
                        for (egc_i, egc) in cluster.grapheme_indices(true) {
                            if x >= egc_x && x <= egc_x + egc_w {
                                new_cursor_char = egc_i;

                                let right_half = x >= egc_x + egc_w / 2.0;
                                if right_half != glyph.level.is_rtl() {
                                    // If clicking on last half of glyph, move cursor past glyph
                                    new_cursor_char += egc.len();
                                    new_cursor_affinity = Affinity::Before;
                                }
                                break 'hit;
                            }
                            egc_x += egc_w;
                        }

                        let right_half = x >= glyph.x + glyph.w / 2.0;
                        if right_half != glyph.level.is_rtl() {
                            // If clicking on last half of glyph, move cursor past glyph
                            new_cursor_char = cluster.len();
                            new_cursor_affinity = Affinity::Before;
                        }
                        break 'hit;
                    }
                }

                let mut new_cursor = Cursor::new(run.line_i, 0);

                match run.glyphs.get(new_cursor_glyph) {
                    Some(glyph) => {
                        // Position at glyph
                        new_cursor.index = glyph.start + new_cursor_char;
                        new_cursor.affinity = new_cursor_affinity;
                    }
                    None => {
                        // Click was past all glyphs in this visual run.
                        // Use the maximum glyph.end across all glyphs.
                        // this is the logical end of this visual line's byte coverage,
                        // correct for LTR, RTL, mixed-BiDi, and wrapped paragraphs.
                        let run_end = run.glyphs.iter().map(|g| g.end).max().unwrap_or(0);
                        new_cursor.index = run_end;
                        new_cursor.affinity = Affinity::Before;
                    }
                }

                new_cursor_opt = Some(new_cursor);

                break;
            } else if runs.peek().is_none() && y > run.line_y {
                // Click below the last run: place cursor at the logical end of the
                // line, regardless of paragraph direction or BiDi mixing.
                let new_cursor =
                    Cursor::new_with_affinity(run.line_i, run.text.len(), Affinity::Before);
                new_cursor_opt = Some(new_cursor);
            }
        }

        #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
        log::trace!("click({}, {}): {:?}", x, y, instant.elapsed());

        new_cursor_opt
    }

    /// Returns the visual (x, y) position of a cursor within the buffer.
    /// y is the top of the line containing the cursor.
    /// This is a convenience wrapper around [`LayoutRun::cursor_position`].
    pub fn cursor_position(&self, cursor: &Cursor) -> Option<(f32, f32)> {
        self.layout_runs()
            .filter(|run| run.line_i == cursor.line)
            .find_map(|run| run.cursor_position(cursor).map(|x| (x, run.line_top)))
    }

    /// Returns if the text direction for a given line is RTL
    /// Returns `None` if the line doesn't exist or hasn't been shaped yet.
    pub fn is_rtl(&self, line: usize) -> Option<bool> {
        self.lines.get(line)?.shape_opt().map(|shape| shape.rtl)
    }

    /// Apply a [`Motion`] to a [`Cursor`]
    pub fn cursor_motion(
        &mut self,
        font_system: &mut FontSystem,
        mut cursor: Cursor,
        mut cursor_x_opt: Option<i32>,
        motion: Motion,
    ) -> Option<(Cursor, Option<i32>)> {
        match motion {
            Motion::LayoutCursor(layout_cursor) => {
                let layout = self.line_layout(font_system, layout_cursor.line)?;

                let layout_line = match layout.get(layout_cursor.layout) {
                    Some(some) => some,
                    None => match layout.last() {
                        Some(some) => some,
                        None => {
                            return None;
                        }
                    },
                };

                let (new_index, new_affinity) =
                    layout_line.glyphs.get(layout_cursor.glyph).map_or_else(
                        || {
                            layout_line
                                .glyphs
                                .last()
                                .map_or((0, Affinity::After), |glyph| (glyph.end, Affinity::Before))
                        },
                        |glyph| (glyph.start, Affinity::After),
                    );

                if cursor.line != layout_cursor.line
                    || cursor.index != new_index
                    || cursor.affinity != new_affinity
                {
                    cursor.line = layout_cursor.line;
                    cursor.index = new_index;
                    cursor.affinity = new_affinity;
                }
            }
            Motion::Previous => {
                let line = self.lines.get(cursor.line)?;
                if cursor.index > 0 {
                    // Find previous character index
                    let mut prev_index = 0;
                    for (i, _) in line.text().grapheme_indices(true) {
                        if i < cursor.index {
                            prev_index = i;
                        } else {
                            break;
                        }
                    }

                    cursor.index = prev_index;
                    cursor.affinity = Affinity::After;
                } else if cursor.line > 0 {
                    cursor.line -= 1;
                    cursor.index = self.lines.get(cursor.line)?.text().len();
                    cursor.affinity = Affinity::After;
                }
                cursor_x_opt = None;
            }
            Motion::Next => {
                let line = self.lines.get(cursor.line)?;
                if cursor.index < line.text().len() {
                    for (i, c) in line.text().grapheme_indices(true) {
                        if i == cursor.index {
                            cursor.index += c.len();
                            cursor.affinity = Affinity::Before;
                            break;
                        }
                    }
                } else if cursor.line + 1 < self.lines.len() {
                    cursor.line += 1;
                    cursor.index = 0;
                    cursor.affinity = Affinity::Before;
                }
                cursor_x_opt = None;
            }
            Motion::Left => {
                let rtl_opt = self
                    .line_shape(font_system, cursor.line)
                    .map(|shape| shape.rtl);
                if let Some(rtl) = rtl_opt {
                    if rtl {
                        (cursor, cursor_x_opt) =
                            self.cursor_motion(font_system, cursor, cursor_x_opt, Motion::Next)?;
                    } else {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::Previous,
                        )?;
                    }
                }
            }
            Motion::Right => {
                let rtl_opt = self
                    .line_shape(font_system, cursor.line)
                    .map(|shape| shape.rtl);
                if let Some(rtl) = rtl_opt {
                    if rtl {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::Previous,
                        )?;
                    } else {
                        (cursor, cursor_x_opt) =
                            self.cursor_motion(font_system, cursor, cursor_x_opt, Motion::Next)?;
                    }
                }
            }
            Motion::Up => {
                let mut layout_cursor = self.layout_cursor(font_system, cursor)?;

                if cursor_x_opt.is_none() {
                    cursor_x_opt = Some(
                        layout_cursor.glyph as i32, //TODO: glyph x position
                    );
                }

                if layout_cursor.layout > 0 {
                    layout_cursor.layout -= 1;
                } else if layout_cursor.line > 0 {
                    layout_cursor.line -= 1;
                    layout_cursor.layout = usize::MAX;
                }

                if let Some(cursor_x) = cursor_x_opt {
                    layout_cursor.glyph = cursor_x as usize; //TODO: glyph x position
                }

                (cursor, cursor_x_opt) = self.cursor_motion(
                    font_system,
                    cursor,
                    cursor_x_opt,
                    Motion::LayoutCursor(layout_cursor),
                )?;
            }
            Motion::Down => {
                let mut layout_cursor = self.layout_cursor(font_system, cursor)?;

                let layout_len = self.line_layout(font_system, layout_cursor.line)?.len();

                if cursor_x_opt.is_none() {
                    cursor_x_opt = Some(
                        layout_cursor.glyph as i32, //TODO: glyph x position
                    );
                }

                if layout_cursor.layout + 1 < layout_len {
                    layout_cursor.layout += 1;
                } else if layout_cursor.line + 1 < self.lines.len() {
                    layout_cursor.line += 1;
                    layout_cursor.layout = 0;
                }

                if let Some(cursor_x) = cursor_x_opt {
                    layout_cursor.glyph = cursor_x as usize; //TODO: glyph x position
                }

                (cursor, cursor_x_opt) = self.cursor_motion(
                    font_system,
                    cursor,
                    cursor_x_opt,
                    Motion::LayoutCursor(layout_cursor),
                )?;
            }
            Motion::Home => {
                cursor.index = 0;
                cursor_x_opt = None;
            }
            Motion::SoftHome => {
                let line = self.lines.get(cursor.line)?;
                cursor.index = line
                    .text()
                    .char_indices()
                    .find_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
                    .unwrap_or(0);
                cursor_x_opt = None;
            }
            Motion::End => {
                let line = self.lines.get(cursor.line)?;
                cursor.index = line.text().len();
                cursor_x_opt = None;
            }
            Motion::ParagraphStart => {
                cursor.index = 0;
                cursor_x_opt = None;
            }
            Motion::ParagraphEnd => {
                cursor.index = self.lines.get(cursor.line)?.text().len();
                cursor_x_opt = None;
            }
            Motion::PageUp => {
                if let Some(height) = self.height_opt {
                    (cursor, cursor_x_opt) = self.cursor_motion(
                        font_system,
                        cursor,
                        cursor_x_opt,
                        Motion::Vertical(-height as i32),
                    )?;
                }
            }
            Motion::PageDown => {
                if let Some(height) = self.height_opt {
                    (cursor, cursor_x_opt) = self.cursor_motion(
                        font_system,
                        cursor,
                        cursor_x_opt,
                        Motion::Vertical(height as i32),
                    )?;
                }
            }
            Motion::Vertical(px) => {
                // TODO more efficient, use layout run line height
                let lines = px / self.metrics().line_height as i32;
                match lines.cmp(&0) {
                    cmp::Ordering::Less => {
                        for _ in 0..-lines {
                            (cursor, cursor_x_opt) =
                                self.cursor_motion(font_system, cursor, cursor_x_opt, Motion::Up)?;
                        }
                    }
                    cmp::Ordering::Greater => {
                        for _ in 0..lines {
                            (cursor, cursor_x_opt) = self.cursor_motion(
                                font_system,
                                cursor,
                                cursor_x_opt,
                                Motion::Down,
                            )?;
                        }
                    }
                    cmp::Ordering::Equal => {}
                }
            }
            Motion::PreviousWord => {
                let line = self.lines.get(cursor.line)?;
                if cursor.index > 0 {
                    cursor.index = line
                        .text()
                        .unicode_word_indices()
                        .rev()
                        .map(|(i, _)| i)
                        .find(|&i| i < cursor.index)
                        .unwrap_or(0);
                } else if cursor.line > 0 {
                    cursor.line -= 1;
                    cursor.index = self.lines.get(cursor.line)?.text().len();
                }
                cursor_x_opt = None;
            }
            Motion::NextWord => {
                let line = self.lines.get(cursor.line)?;
                if cursor.index < line.text().len() {
                    cursor.index = line
                        .text()
                        .unicode_word_indices()
                        .map(|(i, word)| i + word.len())
                        .find(|&i| i > cursor.index)
                        .unwrap_or_else(|| line.text().len());
                } else if cursor.line + 1 < self.lines.len() {
                    cursor.line += 1;
                    cursor.index = 0;
                }
                cursor_x_opt = None;
            }
            Motion::LeftWord => {
                let rtl_opt = self
                    .line_shape(font_system, cursor.line)
                    .map(|shape| shape.rtl);
                if let Some(rtl) = rtl_opt {
                    if rtl {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::NextWord,
                        )?;
                    } else {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::PreviousWord,
                        )?;
                    }
                }
            }
            Motion::RightWord => {
                let rtl_opt = self
                    .line_shape(font_system, cursor.line)
                    .map(|shape| shape.rtl);
                if let Some(rtl) = rtl_opt {
                    if rtl {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::PreviousWord,
                        )?;
                    } else {
                        (cursor, cursor_x_opt) = self.cursor_motion(
                            font_system,
                            cursor,
                            cursor_x_opt,
                            Motion::NextWord,
                        )?;
                    }
                }
            }
            Motion::BufferStart => {
                cursor.line = 0;
                cursor.index = 0;
                cursor_x_opt = None;
            }
            Motion::BufferEnd => {
                cursor.line = self.lines.len().saturating_sub(1);
                cursor.index = self.lines.get(cursor.line)?.text().len();
                cursor_x_opt = None;
            }
            Motion::GotoLine(line) => {
                let mut layout_cursor = self.layout_cursor(font_system, cursor)?;
                layout_cursor.line = line;
                (cursor, cursor_x_opt) = self.cursor_motion(
                    font_system,
                    cursor,
                    cursor_x_opt,
                    Motion::LayoutCursor(layout_cursor),
                )?;
            }
        }
        Some((cursor, cursor_x_opt))
    }

    /// Draw the buffer.
    ///
    /// Automatically resolves any pending dirty state before drawing.
    #[cfg(feature = "swash")]
    pub fn draw<F>(
        &mut self,
        font_system: &mut FontSystem,
        cache: &mut crate::SwashCache,
        color: Color,
        callback: F,
    ) where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.shape_until_scroll(font_system, false);
        let mut renderer = crate::LegacyRenderer {
            font_system,
            cache,
            callback,
        };
        for run in self.layout_runs() {
            for glyph in run.glyphs {
                let physical_glyph = glyph.physical((0., run.line_y), 1.0);
                let glyph_color = glyph.color_opt.map_or(color, |some| some);
                renderer.glyph(physical_glyph, glyph_color);
            }
            render_decoration(&mut renderer, &run, color);
        }
    }

    /// Render the buffer using the provided renderer.
    ///
    /// Automatically resolves any pending dirty state before rendering.
    pub fn render<R: Renderer>(
        &mut self,
        font_system: &mut FontSystem,
        renderer: &mut R,
        color: Color,
    ) {
        self.shape_until_scroll(font_system, false);
        for run in self.layout_runs() {
            for glyph in run.glyphs {
                let physical_glyph = glyph.physical((0., run.line_y), 1.0);
                let glyph_color = glyph.color_opt.map_or(color, |some| some);
                renderer.glyph(physical_glyph, glyph_color);
            }
            // draw decorations after glyphs so strikethrough is over the glyphs
            render_decoration(renderer, &run, color);
        }
    }
}

impl BorrowedWithFontSystem<'_, Buffer> {
    /// Shape lines until cursor, also scrolling to include cursor in view
    pub fn shape_until_cursor(&mut self, cursor: Cursor, prune: bool) {
        self.inner
            .shape_until_cursor(self.font_system, cursor, prune);
    }

    /// Shape the provided line index and return the result
    pub fn line_shape(&mut self, line_i: usize) -> Option<&ShapeLine> {
        self.inner.line_shape(self.font_system, line_i)
    }

    /// Lay out the provided line index and return the result
    pub fn line_layout(&mut self, line_i: usize) -> Option<&[LayoutLine]> {
        self.inner.line_layout(self.font_system, line_i)
    }

    /// Set the current [`Metrics`].
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.font_size` is zero.
    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.inner.set_metrics(metrics);
    }

    /// Set the current [`Hinting`] strategy.
    pub fn set_hinting(&mut self, hinting: Hinting) {
        self.inner.set_hinting(hinting);
    }

    /// Set the current [`Wrap`].
    pub fn set_wrap(&mut self, wrap: Wrap) {
        self.inner.set_wrap(wrap);
    }

    /// Set the current [`Ellipsize`].
    pub fn set_ellipsize(&mut self, ellipsize: Ellipsize) {
        self.inner.set_ellipsize(ellipsize);
    }

    /// Set the current buffer dimensions.
    pub fn set_size(&mut self, width_opt: Option<f32>, height_opt: Option<f32>) {
        self.inner.set_size(width_opt, height_opt);
    }

    /// Set the current [`Metrics`] and buffer dimensions at the same time.
    ///
    /// # Panics
    ///
    /// Will panic if `metrics.font_size` is zero.
    pub fn set_metrics_and_size(
        &mut self,
        metrics: Metrics,
        width_opt: Option<f32>,
        height_opt: Option<f32>,
    ) {
        self.inner
            .set_metrics_and_size(metrics, width_opt, height_opt);
    }

    /// Set tab width (number of spaces between tab stops).
    ///
    /// A `tab_width` of 0 is ignored.
    pub fn set_tab_width(&mut self, tab_width: u16) {
        self.inner.set_tab_width(tab_width);
    }

    /// Set monospace width monospace glyphs should be resized to match. `None` means don't resize.
    pub fn set_monospace_width(&mut self, monospace_width: Option<f32>) {
        self.inner.set_monospace_width(monospace_width);
    }

    /// Set text of buffer, using provided attributes for each line by default.
    pub fn set_text(
        &mut self,
        text: &str,
        attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) {
        self.inner.set_text(text, attrs, shaping, alignment);
    }

    /// Set text of buffer, using an iterator of styled spans (pairs of text and attributes).
    pub fn set_rich_text<'r, 's, I>(
        &mut self,
        spans: I,
        default_attrs: &Attrs,
        shaping: Shaping,
        alignment: Option<Align>,
    ) where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        self.inner
            .set_rich_text(spans, default_attrs, shaping, alignment);
    }

    /// Shape lines until scroll, resolving any pending dirty state first.
    ///
    /// See [`Buffer::shape_until_scroll`].
    pub fn shape_until_scroll(&mut self, prune: bool) {
        self.inner.shape_until_scroll(self.font_system, prune);
    }

    /// Get the visible layout runs for rendering and other tasks.
    ///
    /// Automatically resolves any pending dirty state.
    pub fn layout_runs(&mut self) -> LayoutRunIter<'_> {
        self.inner.shape_until_scroll(self.font_system, false);
        self.inner.layout_runs()
    }

    /// Convert x, y position to Cursor (hit detection).
    ///
    /// Automatically resolves any pending dirty state.
    pub fn hit(&mut self, x: f32, y: f32) -> Option<Cursor> {
        self.inner.shape_until_scroll(self.font_system, false);
        self.inner.hit(x, y)
    }

    /// Apply a [`Motion`] to a [`Cursor`]
    pub fn cursor_motion(
        &mut self,
        cursor: Cursor,
        cursor_x_opt: Option<i32>,
        motion: Motion,
    ) -> Option<(Cursor, Option<i32>)> {
        self.inner
            .cursor_motion(self.font_system, cursor, cursor_x_opt, motion)
    }

    /// Draw the buffer.
    ///
    /// Automatically resolves any pending dirty state.
    #[cfg(feature = "swash")]
    pub fn draw<F>(&mut self, cache: &mut crate::SwashCache, color: Color, f: F)
    where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.inner.draw(self.font_system, cache, color, f);
    }
}
