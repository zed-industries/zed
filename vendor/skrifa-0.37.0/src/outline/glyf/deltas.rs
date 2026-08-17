use core::ops::RangeInclusive;

use raw::tables::glyf::PointCoord;
use read_fonts::{
    tables::glyf::{PointFlags, PointMarker},
    tables::gvar::{GlyphDelta, Gvar},
    tables::variations::TupleVariation,
    types::{F2Dot14, Fixed, GlyphId, Point},
    ReadError,
};

use super::PHANTOM_POINT_COUNT;

/// Compute a set of deltas for the component offsets of a composite glyph.
///
/// Interpolation is meaningless for component offsets so this is a
/// specialized function that skips the expensive bits.
pub(super) fn composite_glyph<D: PointCoord>(
    gvar: &Gvar,
    glyph_id: GlyphId,
    coords: &[F2Dot14],
    deltas: &mut [Point<D>],
) -> Result<(), ReadError> {
    compute_deltas_for_glyph(gvar, glyph_id, coords, deltas, |scalar, tuple, deltas| {
        for tuple_delta in tuple.deltas() {
            let ix = tuple_delta.position as usize;
            if let Some(delta) = deltas.get_mut(ix) {
                *delta += tuple_delta.apply_scalar(scalar);
            }
        }
        Ok(())
    })?;
    Ok(())
}

pub(super) struct SimpleGlyph<'a, C: PointCoord> {
    pub points: &'a [Point<C>],
    pub flags: &'a mut [PointFlags],
    pub contours: &'a [u16],
}

/// Compute a set of deltas for the points in a simple glyph.
///
/// This function will use interpolation to infer missing deltas for tuples
/// that contain sparse sets. The `iup_buffer` buffer is temporary storage
/// used for this and the length must be >= glyph.points.len().
pub(super) fn simple_glyph<C, D>(
    gvar: &Gvar,
    glyph_id: GlyphId,
    coords: &[F2Dot14],
    glyph: SimpleGlyph<C>,
    iup_buffer: &mut [Point<D>],
    deltas: &mut [Point<D>],
) -> Result<(), ReadError>
where
    C: PointCoord,
    D: PointCoord,
    D: From<C>,
{
    if iup_buffer.len() < glyph.points.len() || glyph.points.len() < PHANTOM_POINT_COUNT {
        return Err(ReadError::InvalidArrayLen);
    }
    for delta in deltas.iter_mut() {
        *delta = Default::default();
    }
    if gvar.glyph_variation_data(glyph_id).is_err() {
        // Empty variation data for a glyph is not an error.
        return Ok(());
    };
    let SimpleGlyph {
        points,
        flags,
        contours,
    } = glyph;
    compute_deltas_for_glyph(gvar, glyph_id, coords, deltas, |scalar, tuple, deltas| {
        // Infer missing deltas by interpolation.
        // Prepare our working buffer by converting the points to 16.16
        // and clearing the HAS_DELTA flags.
        for ((flag, point), iup_point) in flags.iter_mut().zip(points).zip(&mut iup_buffer[..]) {
            *iup_point = point.map(D::from);
            flag.clear_marker(PointMarker::HAS_DELTA);
        }
        tuple.accumulate_sparse_deltas(iup_buffer, flags, scalar)?;
        interpolate_deltas(points, flags, contours, &mut iup_buffer[..])
            .ok_or(ReadError::OutOfBounds)?;
        for ((delta, point), iup_point) in deltas.iter_mut().zip(points).zip(iup_buffer.iter()) {
            *delta += *iup_point - point.map(D::from);
        }
        Ok(())
    })?;
    Ok(())
}

/// The common parts of simple and complex glyph processing
fn compute_deltas_for_glyph<C, D>(
    gvar: &Gvar,
    glyph_id: GlyphId,
    coords: &[F2Dot14],
    deltas: &mut [Point<D>],
    mut apply_tuple_missing_deltas_fn: impl FnMut(
        Fixed,
        TupleVariation<GlyphDelta>,
        &mut [Point<D>],
    ) -> Result<(), ReadError>,
) -> Result<(), ReadError>
where
    C: PointCoord,
    D: PointCoord,
    D: From<C>,
{
    for delta in deltas.iter_mut() {
        *delta = Default::default();
    }
    let Ok(Some(var_data)) = gvar.glyph_variation_data(glyph_id) else {
        // Empty variation data for a glyph is not an error.
        return Ok(());
    };
    for (tuple, scalar) in var_data.active_tuples_at(coords) {
        // Fast path: tuple contains all points, we can simply accumulate
        // the deltas directly.
        if tuple.has_deltas_for_all_points() {
            tuple.accumulate_dense_deltas(deltas, scalar)?;
        } else {
            // Slow path is, annoyingly, different for simple vs composite
            // so let the caller handle it
            apply_tuple_missing_deltas_fn(scalar, tuple, deltas)?;
        }
    }
    Ok(())
}

/// Interpolate points without delta values, similar to the IUP hinting
/// instruction.
///
/// Modeled after the FreeType implementation:
/// <https://github.com/freetype/freetype/blob/bbfcd79eacb4985d4b68783565f4b494aa64516b/src/truetype/ttgxvar.c#L3881>
fn interpolate_deltas<C, D>(
    points: &[Point<C>],
    flags: &[PointFlags],
    contours: &[u16],
    out_points: &mut [Point<D>],
) -> Option<()>
where
    C: PointCoord,
    D: PointCoord,
    D: From<C>,
{
    let mut jiggler = Jiggler { points, out_points };
    let mut point_ix = 0usize;
    for &end_point_ix in contours {
        let end_point_ix = end_point_ix as usize;
        let first_point_ix = point_ix;
        // Search for first point that has a delta.
        while point_ix <= end_point_ix && !flags.get(point_ix)?.has_marker(PointMarker::HAS_DELTA) {
            point_ix += 1;
        }
        // If we didn't find any deltas, no variations in the current tuple
        // apply, so skip it.
        if point_ix > end_point_ix {
            continue;
        }
        let first_delta_ix = point_ix;
        let mut cur_delta_ix = point_ix;
        point_ix += 1;
        // Search for next point that has a delta...
        while point_ix <= end_point_ix {
            if flags.get(point_ix)?.has_marker(PointMarker::HAS_DELTA) {
                // ... and interpolate intermediate points.
                jiggler.interpolate(
                    cur_delta_ix + 1..=point_ix - 1,
                    RefPoints(cur_delta_ix, point_ix),
                )?;
                cur_delta_ix = point_ix;
            }
            point_ix += 1;
        }
        // If we only have a single delta, shift the contour.
        if cur_delta_ix == first_delta_ix {
            jiggler.shift(first_point_ix..=end_point_ix, cur_delta_ix)?;
        } else {
            // Otherwise, handle remaining points at beginning and end of
            // contour.
            jiggler.interpolate(
                cur_delta_ix + 1..=end_point_ix,
                RefPoints(cur_delta_ix, first_delta_ix),
            )?;
            if first_delta_ix > 0 {
                jiggler.interpolate(
                    first_point_ix..=first_delta_ix - 1,
                    RefPoints(cur_delta_ix, first_delta_ix),
                )?;
            }
        }
    }
    Some(())
}

struct RefPoints(usize, usize);

struct Jiggler<'a, C, D>
where
    C: PointCoord,
    D: PointCoord,
    D: From<C>,
{
    points: &'a [Point<C>],
    out_points: &'a mut [Point<D>],
}

impl<C, D> Jiggler<'_, C, D>
where
    C: PointCoord,
    D: PointCoord,
    D: From<C>,
{
    /// Shift the coordinates of all points in the specified range using the
    /// difference given by the point at `ref_ix`.
    ///
    /// Modeled after the FreeType implementation: <https://github.com/freetype/freetype/blob/bbfcd79eacb4985d4b68783565f4b494aa64516b/src/truetype/ttgxvar.c#L3776>
    fn shift(&mut self, range: RangeInclusive<usize>, ref_ix: usize) -> Option<()> {
        let ref_in = self.points.get(ref_ix)?.map(D::from);
        let ref_out = self.out_points.get(ref_ix)?;
        let delta = *ref_out - ref_in;
        if delta.x == D::zeroed() && delta.y == D::zeroed() {
            return Some(());
        }
        // Apply the reference point delta to the entire range excluding the
        // reference point itself which would apply the delta twice.
        for out_point in self.out_points.get_mut(*range.start()..ref_ix)? {
            *out_point += delta;
        }
        for out_point in self.out_points.get_mut(ref_ix + 1..=*range.end())? {
            *out_point += delta;
        }
        Some(())
    }

    /// Interpolate the coordinates of all points in the specified range using
    /// `ref1_ix` and `ref2_ix` as the reference point indices.
    ///
    /// Modeled after the FreeType implementation: <https://github.com/freetype/freetype/blob/bbfcd79eacb4985d4b68783565f4b494aa64516b/src/truetype/ttgxvar.c#L3813>
    ///
    /// For details on the algorithm, see: <https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#inferred-deltas-for-un-referenced-point-numbers>
    fn interpolate(&mut self, range: RangeInclusive<usize>, ref_points: RefPoints) -> Option<()> {
        if range.is_empty() {
            return Some(());
        }
        // FreeType uses pointer tricks to handle x and y coords with a single piece of code.
        // Try a macro instead.
        macro_rules! interp_coord {
            ($coord:ident) => {
                let RefPoints(mut ref1_ix, mut ref2_ix) = ref_points;
                if self.points.get(ref1_ix)?.$coord > self.points.get(ref2_ix)?.$coord {
                    core::mem::swap(&mut ref1_ix, &mut ref2_ix);
                }
                let in1 = D::from(self.points.get(ref1_ix)?.$coord);
                let in2 = D::from(self.points.get(ref2_ix)?.$coord);
                let out1 = self.out_points.get(ref1_ix)?.$coord;
                let out2 = self.out_points.get(ref2_ix)?.$coord;
                // If the reference points have the same coordinate but different delta,
                // inferred delta is zero. Otherwise interpolate.
                if in1 != in2 || out1 == out2 {
                    let scale = if in1 != in2 {
                        (out2 - out1) / (in2 - in1)
                    } else {
                        D::zeroed()
                    };
                    let d1 = out1 - in1;
                    let d2 = out2 - in2;
                    for (point, out_point) in self
                        .points
                        .get(range.clone())?
                        .iter()
                        .zip(self.out_points.get_mut(range.clone())?)
                    {
                        let mut out = D::from(point.$coord);
                        if out <= in1 {
                            out += d1;
                        } else if out >= in2 {
                            out += d2;
                        } else {
                            out = out1 + (out - in1) * scale;
                        }
                        out_point.$coord = out;
                    }
                }
            };
        }
        interp_coord!(x);
        interp_coord!(y);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_points(tuples: &[(i32, i32)]) -> Vec<Point<i32>> {
        tuples.iter().map(|&(x, y)| Point::new(x, y)).collect()
    }

    fn make_working_points_and_flags(
        points: &[Point<i32>],
        deltas: &[Point<i32>],
    ) -> (Vec<Point<Fixed>>, Vec<PointFlags>) {
        let working_points = points
            .iter()
            .zip(deltas)
            .map(|(point, delta)| point.map(Fixed::from_i32) + delta.map(Fixed::from_i32))
            .collect();
        let flags = deltas
            .iter()
            .map(|delta| {
                let mut flags = PointFlags::default();
                if delta.x != 0 || delta.y != 0 {
                    flags.set_marker(PointMarker::HAS_DELTA);
                }
                flags
            })
            .collect();
        (working_points, flags)
    }

    #[test]
    fn shift() {
        let points = make_points(&[(245, 630), (260, 700), (305, 680)]);
        // Single delta triggers a full contour shift.
        let deltas = make_points(&[(20, -10), (0, 0), (0, 0)]);
        let (mut working_points, flags) = make_working_points_and_flags(&points, &deltas);
        interpolate_deltas(&points, &flags, &[2], &mut working_points).unwrap();
        let expected = &[
            Point::new(265, 620).map(Fixed::from_i32),
            Point::new(280, 690).map(Fixed::from_i32),
            Point::new(325, 670).map(Fixed::from_i32),
        ];
        assert_eq!(&working_points, expected);
    }

    #[test]
    fn interpolate() {
        // Test taken from the spec:
        // https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#inferred-deltas-for-un-referenced-point-numbers
        // with a minor adjustment to account for the precision of our fixed point math.
        let points = make_points(&[(245, 630), (260, 700), (305, 680)]);
        let deltas = make_points(&[(28, -62), (0, 0), (-42, -57)]);
        let (mut working_points, flags) = make_working_points_and_flags(&points, &deltas);
        interpolate_deltas(&points, &flags, &[2], &mut working_points).unwrap();
        assert_eq!(
            working_points[1],
            Point::new(
                Fixed::from_f64(260.0 + 10.4999237060547),
                Fixed::from_f64(700.0 - 57.0)
            )
        );
    }
}
