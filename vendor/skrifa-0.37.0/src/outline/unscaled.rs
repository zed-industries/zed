//! Compact representation of an unscaled, unhinted outline.

#![allow(dead_code)]

use super::DrawError;
use crate::collections::SmallVec;
use core::ops::Range;
use raw::{
    tables::glyf::PointFlags,
    types::{F26Dot6, Point},
};

#[derive(Copy, Clone, Default, Debug)]
pub(super) struct UnscaledPoint {
    pub x: i16,
    pub y: i16,
    pub flags: PointFlags,
    pub is_contour_start: bool,
}

impl UnscaledPoint {
    pub fn from_glyf_point(
        point: Point<F26Dot6>,
        flags: PointFlags,
        is_contour_start: bool,
    ) -> Self {
        let point = point.map(|x| (x.to_bits() >> 6) as i16);
        Self {
            x: point.x,
            y: point.y,
            flags: flags.without_markers(),
            is_contour_start,
        }
    }

    pub fn is_on_curve(self) -> bool {
        self.flags.is_on_curve()
    }
}

pub(super) trait UnscaledOutlineSink {
    fn try_reserve(&mut self, additional: usize) -> Result<(), DrawError>;
    fn push(&mut self, point: UnscaledPoint) -> Result<(), DrawError>;
    fn extend(&mut self, points: impl IntoIterator<Item = UnscaledPoint>) -> Result<(), DrawError> {
        for point in points.into_iter() {
            self.push(point)?;
        }
        Ok(())
    }
}

// please can I have smallvec?
pub(super) struct UnscaledOutlineBuf<const INLINE_CAP: usize>(SmallVec<UnscaledPoint, INLINE_CAP>);

impl<const INLINE_CAP: usize> UnscaledOutlineBuf<INLINE_CAP> {
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn as_ref(&self) -> UnscaledOutlineRef<'_> {
        UnscaledOutlineRef {
            points: self.0.as_slice(),
        }
    }
}

impl<const INLINE_CAP: usize> UnscaledOutlineSink for UnscaledOutlineBuf<INLINE_CAP> {
    fn try_reserve(&mut self, additional: usize) -> Result<(), DrawError> {
        if !self.0.try_reserve(additional) {
            Err(DrawError::InsufficientMemory)
        } else {
            Ok(())
        }
    }

    fn push(&mut self, point: UnscaledPoint) -> Result<(), DrawError> {
        self.0.push(point);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub(super) struct UnscaledOutlineRef<'a> {
    pub points: &'a [UnscaledPoint],
}

impl UnscaledOutlineRef<'_> {
    /// Returns the range of contour points and the index of the point within
    /// that contour for the last point where `f` returns true.
    ///
    /// This is common code used for finding extrema when materializing blue
    /// zones.
    ///
    /// For example: <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L509>
    pub fn find_last_contour(
        &self,
        mut f: impl FnMut(&UnscaledPoint) -> bool,
    ) -> Option<(Range<usize>, usize)> {
        if self.points.is_empty() {
            return None;
        }
        let mut best_contour = 0..0;
        // Index of the best point relative to the start of the best contour
        let mut best_point = 0;
        let mut cur_contour = 0..0;
        let mut found_best_in_cur_contour = false;
        for (point_ix, point) in self.points.iter().enumerate() {
            if point.is_contour_start {
                if found_best_in_cur_contour {
                    best_contour = cur_contour;
                }
                cur_contour = point_ix..point_ix;
                found_best_in_cur_contour = false;
                // Ignore single point contours
                match self.points.get(point_ix + 1) {
                    Some(next_point) if next_point.is_contour_start => continue,
                    None => continue,
                    _ => {}
                }
            }
            cur_contour.end += 1;
            if f(point) {
                best_point = point_ix - cur_contour.start;
                found_best_in_cur_contour = true;
            }
        }
        if found_best_in_cur_contour {
            best_contour = cur_contour;
        }
        if !best_contour.is_empty() {
            Some((best_contour, best_point))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
enum PendingElement {
    Line([f32; 2]),
    Cubic([f32; 6]),
}

/// Adapts an UnscaledOutlineSink to be fed from a pen while tracking
/// memory allocation errors.
pub(super) struct UnscaledPenAdapter<'a, T> {
    sink: &'a mut T,
    failed: bool,
    last_start: Option<(f32, f32)>,
    pending: Option<PendingElement>,
}

impl<'a, T> UnscaledPenAdapter<'a, T> {
    pub fn new(sink: &'a mut T) -> Self {
        Self {
            sink,
            failed: false,
            last_start: None,
            pending: None,
        }
    }
}

impl<T> UnscaledPenAdapter<'_, T>
where
    T: UnscaledOutlineSink,
{
    fn push(&mut self, x: f32, y: f32, flags: PointFlags, is_contour_start: bool) {
        if self
            .sink
            .push(UnscaledPoint {
                x: x as i16,
                y: y as i16,
                flags,
                is_contour_start,
            })
            .is_err()
        {
            self.failed = true;
        }
    }

    fn flush_pending(&mut self, for_close: bool) {
        if let Some(element) = self.pending.take() {
            let [x, y] = match element {
                PendingElement::Line([x, y]) => [x, y],
                PendingElement::Cubic([x0, y0, x1, y1, x, y]) => {
                    self.push(x0, y0, PointFlags::off_curve_cubic(), false);
                    self.push(x1, y1, PointFlags::off_curve_cubic(), false);
                    [x, y]
                }
            };
            if !for_close || self.last_start != Some((x, y)) {
                self.push(x, y, PointFlags::on_curve(), false);
            }
        }
    }

    pub fn finish(mut self) -> Result<(), DrawError> {
        self.flush_pending(true);
        if self.failed {
            Err(DrawError::InsufficientMemory)
        } else {
            Ok(())
        }
    }
}

impl<T: UnscaledOutlineSink> super::OutlinePen for UnscaledPenAdapter<'_, T> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.push(x, y, PointFlags::on_curve(), true);
        self.last_start = Some((x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.flush_pending(false);
        self.pending = Some(PendingElement::Line([x, y]));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.flush_pending(false);
        self.push(cx0, cy0, PointFlags::off_curve_quad(), false);
        self.push(x, y, PointFlags::on_curve(), false);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.flush_pending(false);
        self.pending = Some(PendingElement::Cubic([cx0, cy0, cx1, cy1, x, y]));
    }

    fn close(&mut self) {
        self.flush_pending(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{outline::OutlinePen, prelude::LocationRef, MetadataProvider};
    use raw::{types::GlyphId, FontRef};

    #[test]
    fn read_glyf_outline() {
        let font = FontRef::new(font_test_data::MATERIAL_SYMBOLS_SUBSET).unwrap();
        let glyph = font.outline_glyphs().get(GlyphId::new(5)).unwrap();
        let mut outline = UnscaledOutlineBuf::<32>::new();
        glyph
            .draw_unscaled(LocationRef::default(), None, &mut outline)
            .unwrap();
        let outline = outline.as_ref();
        let expected = [
            // contour 0
            (400, 80, 1),
            (400, 360, 1),
            (320, 360, 1),
            (320, 600, 1),
            (320, 633, 0),
            (367, 680, 0),
            (400, 680, 1),
            (560, 680, 1),
            (593, 680, 0),
            (640, 633, 0),
            (640, 600, 1),
            (640, 360, 1),
            (560, 360, 1),
            (560, 80, 1),
            // contour 1
            (480, 720, 1),
            (447, 720, 0),
            (400, 767, 0),
            (400, 800, 1),
            (400, 833, 0),
            (447, 880, 0),
            (480, 880, 1),
            (513, 880, 0),
            (560, 833, 0),
            (560, 800, 1),
            (560, 767, 0),
            (513, 720, 0),
        ];
        let points = outline
            .points
            .iter()
            .map(|point| (point.x, point.y, point.flags.to_bits()))
            .collect::<Vec<_>>();
        assert_eq!(points, expected);
    }

    #[test]
    #[cfg(feature = "spec_next")]
    fn read_cubic_glyf_outline() {
        let font = FontRef::new(font_test_data::CUBIC_GLYF).unwrap();
        let glyph = font.outline_glyphs().get(GlyphId::new(2)).unwrap();
        let mut outline = UnscaledOutlineBuf::<32>::new();
        glyph
            .draw_unscaled(LocationRef::default(), None, &mut outline)
            .unwrap();
        let outline = outline.as_ref();
        let expected = [
            // contour 0
            (278, 710, 1),
            (278, 470, 1),
            (300, 500, 128),
            (800, 500, 128),
            (998, 470, 1),
            (998, 710, 1),
        ];
        let points = outline
            .points
            .iter()
            .map(|point| (point.x, point.y, point.flags.to_bits()))
            .collect::<Vec<_>>();
        assert_eq!(points, expected);
    }

    #[test]
    fn read_cff_outline() {
        let font = FontRef::new(font_test_data::CANTARELL_VF_TRIMMED).unwrap();
        let glyph = font.outline_glyphs().get(GlyphId::new(2)).unwrap();
        let mut outline = UnscaledOutlineBuf::<32>::new();
        glyph
            .draw_unscaled(LocationRef::default(), None, &mut outline)
            .unwrap();
        let outline = outline.as_ref();
        let expected = [
            // contour 0
            (83, 0, 1),
            (163, 0, 1),
            (163, 482, 1),
            (83, 482, 1),
            // contour 1
            (124, 595, 1),
            (160, 595, 128),
            (181, 616, 128),
            (181, 652, 1),
            (181, 688, 128),
            (160, 709, 128),
            (124, 709, 1),
            (88, 709, 128),
            (67, 688, 128),
            (67, 652, 1),
            (67, 616, 128),
            (88, 595, 128),
        ];
        let points = outline
            .points
            .iter()
            .map(|point| (point.x, point.y, point.flags.to_bits()))
            .collect::<Vec<_>>();
        assert_eq!(points, expected);
    }

    #[test]
    fn find_vertical_extrema() {
        let font = FontRef::new(font_test_data::MATERIAL_SYMBOLS_SUBSET).unwrap();
        let glyph = font.outline_glyphs().get(GlyphId::new(5)).unwrap();
        let mut outline = UnscaledOutlineBuf::<32>::new();
        glyph
            .draw_unscaled(LocationRef::default(), None, &mut outline)
            .unwrap();
        let outline = outline.as_ref();
        // Find the maximum Y value and its containing contour
        let mut top_y = None;
        let (top_contour, top_point_ix) = outline
            .find_last_contour(|point| {
                if top_y.is_none() || Some(point.y) > top_y {
                    top_y = Some(point.y);
                    true
                } else {
                    false
                }
            })
            .unwrap();
        assert_eq!(top_contour, 14..26);
        assert_eq!(top_point_ix, 5);
        assert_eq!(top_y, Some(880));
        // Find the minimum Y value and its containing contour
        let mut bottom_y = None;
        let (bottom_contour, bottom_point_ix) = outline
            .find_last_contour(|point| {
                if bottom_y.is_none() || Some(point.y) < bottom_y {
                    bottom_y = Some(point.y);
                    true
                } else {
                    false
                }
            })
            .unwrap();
        assert_eq!(bottom_contour, 0..14);
        assert_eq!(bottom_point_ix, 0);
        assert_eq!(bottom_y, Some(80));
    }

    /// When a contour ends with a line or cubic whose end matches the start
    /// point, omit the last on curve. This matches FreeType behavior when
    /// constructing a TrueType style outline from a CFF font.
    #[test]
    fn omit_unnecessary_trailing_oncurves() {
        let mut outline = UnscaledOutlineBuf::<64>::new();
        let mut pen = UnscaledPenAdapter::new(&mut outline);
        pen.move_to(0.5, 1.5);
        pen.line_to(1.0, 2.0);
        // matches start, omit last on curve
        pen.line_to(0.5, 1.5);
        pen.close();
        pen.move_to(5.0, 6.0);
        pen.curve_to(1.0, 1.0, 2.0, 2.0, 2.0, 3.0);
        // matches start, omit last on curve
        pen.curve_to(1.0, 1.0, 2.0, 2.0, 5.0, 6.0);
        pen.close();
        pen.move_to(5.0, 6.0);
        // doesn't match start, keep on curve
        pen.curve_to(1.0, 1.0, 2.0, 2.0, 2.0, 3.0);
        pen.close();
        pen.finish().unwrap();
        // Collect a vec of bools where true means on curve
        let on_curves = outline
            .0
            .iter()
            .map(|point| point.flags.is_on_curve())
            .collect::<Vec<_>>();
        #[rustfmt::skip]
        let expected_on_curves = [
            true,  // move
            true,  // line
                   // trailing line omitted
            true,  // move
            false, // cubic 1 off curve 1
            false, // cubic 1 off curve 2
            true,  // cubic 1 on curve
            false, // cubic 2 off curve 1
            false, // cubic 2 off curve 2
                   // trailing on curve omitted
            true,  // move
            false, // cubic 1 off curve 1
            false, // cubic 1 off curve 2
            true,  // trailing on curve retained
        ];
        assert_eq!(on_curves, expected_on_curves);
    }
}
