use crate::{DisplayPoint, Point};
use core::ops::Range;
use git::diff::{DiffHunk, DiffHunkStatus};
use gpui::{fill, point, Bounds, Hsla, Pixels};
use language::DiagnosticEntry;
use lsp::DiagnosticSeverity;
use std::sync::Arc;
use std::{iter::IntoIterator, vec::IntoIter};
use theme::Theme;
use ui::ElementContext;

/// A row range that must be marked on the scrollbar using the given color.
pub struct MarkedRowRange {
    pub start: u32,
    pub end: u32,
    pub color: Hsla,
}

/// Information about a marker quad that must be painted on the scrollbar.
pub struct Marker {
    pub start: Pixels,
    pub end: Pixels,
    pub color: Hsla,
}

impl Marker {
    pub fn new(start: Pixels, end: Pixels, color: Hsla) -> Self {
        Self { start, end, color }
    }

    /// Tries to merge two marker quads into a single one.
    pub fn try_merge(&mut self, other: &Marker) -> bool {
        if other.start <= self.end && other.end >= self.start && other.color == self.color {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
            return true;
        }
        false
    }

    pub fn paint(&self, x_range: &Range<Pixels>, cx: &mut ElementContext) {
        let bounds = Bounds::from_corners(
            point(x_range.start, self.start),
            point(x_range.end, self.end),
        );
        cx.paint_quad(fill(bounds, self.color));
    }
}

/// Iterator over row ranges colored the same way.
pub struct SameColorRangesIter {
    inner_iter: IntoIter<Range<u32>>,
    color: Hsla,
}

impl SameColorRangesIter {
    pub fn new(row_ranges: Vec<Range<u32>>, color: Hsla) -> Self {
        Self {
            inner_iter: row_ranges.into_iter(),
            color,
        }
    }
}

impl Iterator for SameColorRangesIter {
    type Item = MarkedRowRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|range| MarkedRowRange {
            start: range.start,
            end: range.end,
            color: self.color,
        })
    }
}

/// Wrapper over iterator over diff hunk ranges.
/// Adds colors to diff ranges and adjusts row ranges.
pub struct DiffHunkRangesIter<'a> {
    inner_iter: Box<dyn Iterator<Item = DiffHunk<u32>> + 'a>,
    transform: Box<dyn Fn(Point) -> DisplayPoint + 'a>,
    theme: Arc<Theme>,
}

impl<'a> DiffHunkRangesIter<'a> {
    pub fn new(
        inner_iter: Box<dyn Iterator<Item = DiffHunk<u32>> + 'a>,
        transform: Box<dyn Fn(Point) -> DisplayPoint + 'a>,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            inner_iter,
            transform,
            theme,
        }
    }
}

impl Iterator for DiffHunkRangesIter<'_> {
    type Item = MarkedRowRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|range| {
            let start = (self.transform)(Point::new(range.associated_range.start, 0)).row();
            let mut end = (self.transform)(Point::new(range.associated_range.end, 0)).row();
            end = if end == start { end } else { end - 1 };
            let color = match range.status() {
                DiffHunkStatus::Added => self.theme.status().created,
                DiffHunkStatus::Modified => self.theme.status().modified,
                DiffHunkStatus::Removed => self.theme.status().deleted,
            };
            MarkedRowRange { start, end, color }
        })
    }
}

/// Wrapper over iterator over diagnostic ranges.
/// Adds colors to the ranges.
pub struct DiagnosticRangesIter<'a> {
    inner_iter: Box<dyn Iterator<Item = DiagnosticEntry<Point>> + 'a>,
    transform: Box<dyn Fn(Point) -> DisplayPoint + 'a>,
    theme: Arc<Theme>,
}

impl<'a> DiagnosticRangesIter<'a> {
    pub fn new(
        inner_iter: Box<dyn Iterator<Item = DiagnosticEntry<Point>> + 'a>,
        transform: Box<dyn Fn(Point) -> DisplayPoint + 'a>,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            inner_iter,
            transform,
            theme,
        }
    }
}

impl Iterator for DiagnosticRangesIter<'_> {
    type Item = MarkedRowRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|diagnostic| {
            let color = match diagnostic.diagnostic.severity {
                DiagnosticSeverity::ERROR => self.theme.status().error,
                DiagnosticSeverity::WARNING => self.theme.status().warning,
                DiagnosticSeverity::INFORMATION => self.theme.status().info,
                _ => self.theme.status().hint,
            };
            MarkedRowRange {
                start: (self.transform)(diagnostic.range.start).row(),
                end: (self.transform)(diagnostic.range.end).row(),
                color,
            }
        })
    }
}
