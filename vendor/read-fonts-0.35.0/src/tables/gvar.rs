//! The [gvar (Glyph Variations)](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar)
//! table

include!("../../generated/generated_gvar.rs");

use super::{
    glyf::{CompositeGlyphFlags, Glyf, Glyph, PointCoord},
    loca::Loca,
    variations::{
        PackedPointNumbers, Tuple, TupleDelta, TupleVariationCount, TupleVariationData,
        TupleVariationHeader,
    },
};

/// Variation data specialized for the glyph variations table.
pub type GlyphVariationData<'a> = TupleVariationData<'a, GlyphDelta>;

#[derive(Clone, Copy, Debug)]
pub struct U16Or32(u32);

impl ReadArgs for U16Or32 {
    type Args = GvarFlags;
}

impl ComputeSize for U16Or32 {
    fn compute_size(args: &GvarFlags) -> Result<usize, ReadError> {
        Ok(if args.contains(GvarFlags::LONG_OFFSETS) {
            4
        } else {
            2
        })
    }
}

impl FontReadWithArgs<'_> for U16Or32 {
    fn read_with_args(data: FontData<'_>, args: &Self::Args) -> Result<Self, ReadError> {
        if args.contains(GvarFlags::LONG_OFFSETS) {
            data.read_at::<u32>(0).map(Self)
        } else {
            data.read_at::<u16>(0).map(|v| Self(v as u32 * 2))
        }
    }
}

impl U16Or32 {
    #[inline]
    pub fn get(self) -> u32 {
        self.0
    }
}

impl<'a> GlyphVariationDataHeader<'a> {
    fn raw_tuple_header_data(&self) -> FontData<'a> {
        let range = self.shape.tuple_variation_headers_byte_range();
        self.data.split_off(range.start).unwrap()
    }
}

impl<'a> Gvar<'a> {
    /// Return the raw data for this gid.
    ///
    /// If there is no variation data for the glyph, returns `Ok(None)`.
    pub fn data_for_gid(&self, gid: GlyphId) -> Result<Option<FontData<'a>>, ReadError> {
        let range = self.data_range_for_gid(gid)?;
        if range.is_empty() {
            return Ok(None);
        }
        match self.data.slice(range) {
            Some(data) => Ok(Some(data)),
            None => Err(ReadError::OutOfBounds),
        }
    }

    pub fn glyph_variation_data_for_range(
        &self,
        offset_range: Range<usize>,
    ) -> Result<FontData<'a>, ReadError> {
        let base = self.glyph_variation_data_array_offset() as usize;
        let start = base
            .checked_add(offset_range.start)
            .ok_or(ReadError::OutOfBounds)?;
        let end = base
            .checked_add(offset_range.end)
            .ok_or(ReadError::OutOfBounds)?;
        self.data.slice(start..end).ok_or(ReadError::OutOfBounds)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }

    fn data_range_for_gid(&self, gid: GlyphId) -> Result<Range<usize>, ReadError> {
        let start_idx = gid.to_u32() as usize;
        let end_idx = start_idx + 1;
        let data_start = self.glyph_variation_data_array_offset();
        let start =
            data_start.checked_add(self.glyph_variation_data_offsets().get(start_idx)?.get());
        let end = data_start.checked_add(self.glyph_variation_data_offsets().get(end_idx)?.get());
        let (Some(start), Some(end)) = (start, end) else {
            return Err(ReadError::OutOfBounds);
        };
        Ok(start as usize..end as usize)
    }

    /// Get the variation data for a specific glyph.
    ///
    /// Returns `Ok(None)` if there is no variation data for this glyph, and
    /// returns an error if there is data but it is malformed.
    pub fn glyph_variation_data(
        &self,
        gid: GlyphId,
    ) -> Result<Option<GlyphVariationData<'a>>, ReadError> {
        let shared_tuples = self.shared_tuples()?;
        let axis_count = self.axis_count();
        let data = self.data_for_gid(gid)?;
        data.map(|data| GlyphVariationData::new(data, axis_count, shared_tuples))
            .transpose()
    }

    /// Returns the phantom point deltas for the given variation coordinates
    /// and glyph identifier, if variation data exists for the glyph.
    ///
    /// The resulting array will contain four deltas:
    /// `[left, right, top, bottom]`.
    pub fn phantom_point_deltas(
        &self,
        glyf: &Glyf,
        loca: &Loca,
        coords: &[F2Dot14],
        glyph_id: GlyphId,
    ) -> Result<Option<[Point<Fixed>; 4]>, ReadError> {
        // For any given glyph, there's only one outline that contributes to
        // metrics deltas (via "phantom points"). For simple glyphs, that is
        // the glyph itself. For composite glyphs, it is the last component
        // in the tree that has the USE_MY_METRICS flag set or, if there are
        // none, the composite glyph itself.
        //
        // This searches for the glyph that meets that criteria and also
        // returns the point count (for composites, this is the component
        // count), so that we know where the deltas for phantom points start
        // in the variation data.
        let (glyph_id, point_count) = find_glyph_and_point_count(glyf, loca, glyph_id, 0)?;
        let mut phantom_deltas = [Point::default(); 4];
        let phantom_range = point_count..point_count + 4;
        let Some(var_data) = self.glyph_variation_data(glyph_id)? else {
            return Ok(None);
        };
        // Note that phantom points can never belong to a contour so we don't have
        // to handle the IUP case here.
        for (tuple, scalar) in var_data.active_tuples_at(coords) {
            for tuple_delta in tuple.deltas() {
                let ix = tuple_delta.position as usize;
                if phantom_range.contains(&ix) {
                    phantom_deltas[ix - phantom_range.start] += tuple_delta.apply_scalar(scalar);
                }
            }
        }
        Ok(Some(phantom_deltas))
    }
}

impl<'a> GlyphVariationData<'a> {
    pub(crate) fn new(
        data: FontData<'a>,
        axis_count: u16,
        shared_tuples: SharedTuples<'a>,
    ) -> Result<Self, ReadError> {
        let header = GlyphVariationDataHeader::read(data)?;

        let header_data = header.raw_tuple_header_data();
        let count = header.tuple_variation_count();
        let data = header.serialized_data()?;

        // if there are shared point numbers, get them now
        let (shared_point_numbers, serialized_data) =
            if header.tuple_variation_count().shared_point_numbers() {
                let (packed, data) = PackedPointNumbers::split_off_front(data);
                (Some(packed), data)
            } else {
                (None, data)
            };

        Ok(GlyphVariationData {
            tuple_count: count,
            axis_count,
            shared_tuples: Some(shared_tuples.tuples()),
            shared_point_numbers,
            header_data,
            serialized_data,
            _marker: std::marker::PhantomData,
        })
    }
}

/// Delta information for a single point or component in a glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlyphDelta {
    /// The point or component index.
    pub position: u16,
    /// The x delta.
    pub x_delta: i32,
    /// The y delta.
    pub y_delta: i32,
}

impl GlyphDelta {
    /// Applies a tuple scalar to this delta.
    pub fn apply_scalar<D: PointCoord>(self, scalar: Fixed) -> Point<D> {
        let scalar = D::from_fixed(scalar);
        Point::new(self.x_delta, self.y_delta).map(D::from_i32) * scalar
    }
}

impl TupleDelta for GlyphDelta {
    fn is_point() -> bool {
        true
    }

    fn new(position: u16, x: i32, y: i32) -> Self {
        Self {
            position,
            x_delta: x,
            y_delta: y,
        }
    }
}

/// Given a glyph identifier, searches for the glyph that contains the actual
/// metrics for rendering.
///
/// For simple glyphs, that is simply the requested glyph. For composites, it
/// depends on the USE_MY_METRICS flag.
///
/// Returns the resulting glyph identifier and the number of points (or
/// components) in that glyph. This count represents the start of the phantom
/// points.
fn find_glyph_and_point_count(
    glyf: &Glyf,
    loca: &Loca,
    glyph_id: GlyphId,
    recurse_depth: usize,
) -> Result<(GlyphId, usize), ReadError> {
    // Matches HB's nesting limit
    const RECURSION_LIMIT: usize = 64;
    if recurse_depth > RECURSION_LIMIT {
        return Err(ReadError::MalformedData(
            "nesting too deep in composite glyph",
        ));
    }
    let glyph = loca.get_glyf(glyph_id, glyf)?;
    let Some(glyph) = glyph else {
        // Empty glyphs might still contain gvar data that
        // only affects phantom points
        return Ok((glyph_id, 0));
    };
    match glyph {
        Glyph::Simple(simple) => {
            // Simple glyphs always use their own metrics
            Ok((glyph_id, simple.num_points()))
        }
        Glyph::Composite(composite) => {
            // For composite glyphs, recurse into the glyph referenced by the
            // *last* component that has the USE_MY_METRICS flag set.
            // Otherwise, return the composite glyph itself and the number of
            // components as the point count.
            // https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs
            let (count, inherit_metrics) = composite.component_glyphs_and_flags().fold(
                (0, None),
                |(count, inherit_metrics), (component_id, flags)| {
                    let has_flag = flags.contains(CompositeGlyphFlags::USE_MY_METRICS);
                    let preferred = has_flag.then_some(component_id).or(inherit_metrics);

                    (count + 1, preferred)
                },
            );

            if let Some(component) = inherit_metrics {
                find_glyph_and_point_count(glyf, loca, component.into(), recurse_depth + 1)
            } else {
                Ok((glyph_id, count))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use font_test_data::bebuffer::BeBuffer;

    use super::*;
    use crate::{FontRef, TableProvider};

    // Shared tuples in the 'gvar' table of the Skia font, as printed
    // in Apple's TrueType specification.
    // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6gvar.html
    static SKIA_GVAR_SHARED_TUPLES_DATA: FontData = FontData::new(&[
        0x40, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0xC0,
        0x00, 0xC0, 0x00, 0xC0, 0x00, 0x40, 0x00, 0xC0, 0x00, 0x40, 0x00, 0x40, 0x00, 0xC0, 0x00,
        0x40, 0x00,
    ]);

    static SKIA_GVAR_I_DATA: FontData = FontData::new(&[
        0x00, 0x08, 0x00, 0x24, 0x00, 0x33, 0x20, 0x00, 0x00, 0x15, 0x20, 0x01, 0x00, 0x1B, 0x20,
        0x02, 0x00, 0x24, 0x20, 0x03, 0x00, 0x15, 0x20, 0x04, 0x00, 0x26, 0x20, 0x07, 0x00, 0x0D,
        0x20, 0x06, 0x00, 0x1A, 0x20, 0x05, 0x00, 0x40, 0x01, 0x01, 0x01, 0x81, 0x80, 0x43, 0xFF,
        0x7E, 0xFF, 0x7E, 0xFF, 0x7E, 0xFF, 0x7E, 0x00, 0x81, 0x45, 0x01, 0x01, 0x01, 0x03, 0x01,
        0x04, 0x01, 0x04, 0x01, 0x04, 0x01, 0x02, 0x80, 0x40, 0x00, 0x82, 0x81, 0x81, 0x04, 0x3A,
        0x5A, 0x3E, 0x43, 0x20, 0x81, 0x04, 0x0E, 0x40, 0x15, 0x45, 0x7C, 0x83, 0x00, 0x0D, 0x9E,
        0xF3, 0xF2, 0xF0, 0xF0, 0xF0, 0xF0, 0xF3, 0x9E, 0xA0, 0xA1, 0xA1, 0xA1, 0x9F, 0x80, 0x00,
        0x91, 0x81, 0x91, 0x00, 0x0D, 0x0A, 0x0A, 0x09, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A, 0x0A,
        0x0A, 0x0A, 0x0A, 0x0B, 0x80, 0x00, 0x15, 0x81, 0x81, 0x00, 0xC4, 0x89, 0x00, 0xC4, 0x83,
        0x00, 0x0D, 0x80, 0x99, 0x98, 0x96, 0x96, 0x96, 0x96, 0x99, 0x80, 0x82, 0x83, 0x83, 0x83,
        0x81, 0x80, 0x40, 0xFF, 0x18, 0x81, 0x81, 0x04, 0xE6, 0xF9, 0x10, 0x21, 0x02, 0x81, 0x04,
        0xE8, 0xE5, 0xEB, 0x4D, 0xDA, 0x83, 0x00, 0x0D, 0xCE, 0xD3, 0xD4, 0xD3, 0xD3, 0xD3, 0xD5,
        0xD2, 0xCE, 0xCC, 0xCD, 0xCD, 0xCD, 0xCD, 0x80, 0x00, 0xA1, 0x81, 0x91, 0x00, 0x0D, 0x07,
        0x03, 0x04, 0x02, 0x02, 0x02, 0x03, 0x03, 0x07, 0x07, 0x08, 0x08, 0x08, 0x07, 0x80, 0x00,
        0x09, 0x81, 0x81, 0x00, 0x28, 0x40, 0x00, 0xA4, 0x02, 0x24, 0x24, 0x66, 0x81, 0x04, 0x08,
        0xFA, 0xFA, 0xFA, 0x28, 0x83, 0x00, 0x82, 0x02, 0xFF, 0xFF, 0xFF, 0x83, 0x02, 0x01, 0x01,
        0x01, 0x84, 0x91, 0x00, 0x80, 0x06, 0x07, 0x08, 0x08, 0x08, 0x08, 0x0A, 0x07, 0x80, 0x03,
        0xFE, 0xFF, 0xFF, 0xFF, 0x81, 0x00, 0x08, 0x81, 0x82, 0x02, 0xEE, 0xEE, 0xEE, 0x8B, 0x6D,
        0x00,
    ]);

    #[test]
    fn test_shared_tuples() {
        #[allow(overflowing_literals)]
        const MINUS_ONE: F2Dot14 = F2Dot14::from_bits(0xC000);
        assert_eq!(MINUS_ONE, F2Dot14::from_f32(-1.0));

        static EXPECTED: &[(F2Dot14, F2Dot14)] = &[
            (F2Dot14::ONE, F2Dot14::ZERO),
            (MINUS_ONE, F2Dot14::ZERO),
            (F2Dot14::ZERO, F2Dot14::ONE),
            (F2Dot14::ZERO, MINUS_ONE),
            (MINUS_ONE, MINUS_ONE),
            (F2Dot14::ONE, MINUS_ONE),
            (F2Dot14::ONE, F2Dot14::ONE),
            (MINUS_ONE, F2Dot14::ONE),
        ];

        const N_AXES: u16 = 2;

        let tuples =
            SharedTuples::read(SKIA_GVAR_SHARED_TUPLES_DATA, EXPECTED.len() as u16, N_AXES)
                .unwrap();
        let tuple_vec: Vec<_> = tuples
            .tuples()
            .iter()
            .map(|tup| {
                let values = tup.unwrap().values();
                assert_eq!(values.len(), N_AXES as usize);
                (values[0].get(), values[1].get())
            })
            .collect();

        assert_eq!(tuple_vec, EXPECTED);
    }

    // https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6gvar.html
    #[test]
    fn smoke_test() {
        let header = GlyphVariationDataHeader::read(SKIA_GVAR_I_DATA).unwrap();
        assert_eq!(header.serialized_data_offset(), 36);
        assert_eq!(header.tuple_variation_count().count(), 8);
        let shared_tuples = SharedTuples::read(SKIA_GVAR_SHARED_TUPLES_DATA, 8, 2).unwrap();

        let vardata = GlyphVariationData::new(SKIA_GVAR_I_DATA, 2, shared_tuples).unwrap();
        assert_eq!(vardata.tuple_count(), 8);
        let deltas = vardata
            .tuples()
            .next()
            .unwrap()
            .deltas()
            .collect::<Vec<_>>();
        assert_eq!(deltas.len(), 18);
        static EXPECTED: &[(i32, i32)] = &[
            (257, 0),
            (-127, 0),
            (-128, 58),
            (-130, 90),
            (-130, 62),
            (-130, 67),
            (-130, 32),
            (-127, 0),
            (257, 0),
            (259, 14),
            (260, 64),
            (260, 21),
            (260, 69),
            (258, 124),
            (0, 0),
            (130, 0),
            (0, 0),
            (0, 0),
        ];
        let expected = EXPECTED
            .iter()
            .copied()
            .enumerate()
            .map(|(pos, (x_delta, y_delta))| GlyphDelta {
                position: pos as _,
                x_delta,
                y_delta,
            })
            .collect::<Vec<_>>();

        for (a, b) in deltas.iter().zip(expected.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn vazirmatn_var_a() {
        let gvar = FontRef::new(font_test_data::VAZIRMATN_VAR)
            .unwrap()
            .gvar()
            .unwrap();
        let a_glyph_var = gvar.glyph_variation_data(GlyphId::new(1)).unwrap().unwrap();
        assert_eq!(a_glyph_var.axis_count, 1);
        let mut tuples = a_glyph_var.tuples();
        let tup1 = tuples.next().unwrap();
        assert_eq!(tup1.peak().values(), &[F2Dot14::from_f32(-1.0)]);
        assert_eq!(tup1.deltas().count(), 18);
        let x_vals = &[
            -90, -134, 4, -6, -81, 18, -25, -33, -109, -121, -111, -111, -22, -22, 0, -113, 0, 0,
        ];
        let y_vals = &[
            83, 0, 0, 0, 0, 0, 83, 0, 0, 0, -50, 54, 54, -50, 0, 0, -21, 0,
        ];
        assert_eq!(tup1.deltas().map(|d| d.x_delta).collect::<Vec<_>>(), x_vals);
        assert_eq!(tup1.deltas().map(|d| d.y_delta).collect::<Vec<_>>(), y_vals);
        let tup2 = tuples.next().unwrap();
        assert_eq!(tup2.peak().values(), &[F2Dot14::from_f32(1.0)]);
        let x_vals = &[
            20, 147, -33, -53, 59, -90, 37, -6, 109, 90, -79, -79, -8, -8, 0, 59, 0, 0,
        ];
        let y_vals = &[
            -177, 0, 0, 0, 0, 0, -177, 0, 0, 0, 4, -109, -109, 4, 0, 0, 9, 0,
        ];

        assert_eq!(tup2.deltas().map(|d| d.x_delta).collect::<Vec<_>>(), x_vals);
        assert_eq!(tup2.deltas().map(|d| d.y_delta).collect::<Vec<_>>(), y_vals);
        assert!(tuples.next().is_none());
    }

    #[test]
    fn vazirmatn_var_agrave() {
        let gvar = FontRef::new(font_test_data::VAZIRMATN_VAR)
            .unwrap()
            .gvar()
            .unwrap();
        let agrave_glyph_var = gvar.glyph_variation_data(GlyphId::new(2)).unwrap().unwrap();
        let mut tuples = agrave_glyph_var.tuples();
        let tup1 = tuples.next().unwrap();
        assert_eq!(
            tup1.deltas()
                .map(|d| (d.position, d.x_delta, d.y_delta))
                .collect::<Vec<_>>(),
            &[(1, -51, 8), (3, -113, 0)]
        );
        let tup2 = tuples.next().unwrap();
        assert_eq!(
            tup2.deltas()
                .map(|d| (d.position, d.x_delta, d.y_delta))
                .collect::<Vec<_>>(),
            &[(1, -54, -1), (3, 59, 0)]
        );
    }

    #[test]
    fn vazirmatn_var_grave() {
        let gvar = FontRef::new(font_test_data::VAZIRMATN_VAR)
            .unwrap()
            .gvar()
            .unwrap();
        let grave_glyph_var = gvar.glyph_variation_data(GlyphId::new(3)).unwrap().unwrap();
        let mut tuples = grave_glyph_var.tuples();
        let tup1 = tuples.next().unwrap();
        let tup2 = tuples.next().unwrap();
        assert!(tuples.next().is_none());
        assert_eq!(tup1.deltas().count(), 8);
        assert_eq!(
            tup2.deltas().map(|d| d.y_delta).collect::<Vec<_>>(),
            &[0, -20, -20, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn phantom_point_deltas() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        #[rustfmt::skip]
        let a_cases = [
            // (coords, deltas)
            (&[0.0], [(0.0, 0.0); 4]),
            (&[1.0], [(0.0, 0.0), (59.0, 0.0), (0.0, 9.0), (0.0, 0.0)]),
            (&[-1.0], [(0.0, 0.0), (-113.0, 0.0), (0.0, -21.0), (0.0, 0.0)]),
            (&[0.5], [(0.0, 0.0), (29.5, 0.0), (0.0, 4.5), (0.0, 0.0)]),
            (&[-0.5], [(0.0, 0.0), (-56.5, 0.0), (0.0, -10.5), (0.0, 0.0)]),
        ];
        for (coords, deltas) in a_cases {
            // This is simple glyph "A"
            assert_eq!(
                compute_phantom_deltas(&font, coords, GlyphId::new(1)),
                deltas
            );
            // This is composite glyph "Agrave" with USE_MY_METRICS set on "A" so
            // the deltas are the same
            assert_eq!(
                compute_phantom_deltas(&font, coords, GlyphId::new(2)),
                deltas
            );
        }
        #[rustfmt::skip]
        let grave_cases = [
            // (coords, deltas)
            (&[0.0], [(0.0, 0.0); 4]),
            (&[1.0], [(0.0, 0.0), (63.0, 0.0), (0.0, 0.0), (0.0, 0.0)]),
            (&[-1.0], [(0.0, 0.0), (-96.0, 0.0), (0.0, 0.0), (0.0, 0.0)]),
            (&[0.5], [(0.0, 0.0), (31.5, 0.0), (0.0, 0.0), (0.0, 0.0)]),
            (&[-0.5], [(0.0, 0.0), (-48.0, 0.0), (0.0, 0.0), (0.0, 0.0)]),
        ];
        // This is simple glyph "grave"
        for (coords, deltas) in grave_cases {
            assert_eq!(
                compute_phantom_deltas(&font, coords, GlyphId::new(3)),
                deltas
            );
        }
    }

    fn compute_phantom_deltas(
        font: &FontRef,
        coords: &[f32],
        glyph_id: GlyphId,
    ) -> [(f32, f32); 4] {
        let loca = font.loca(None).unwrap();
        let glyf = font.glyf().unwrap();
        let gvar = font.gvar().unwrap();
        let coords = coords
            .iter()
            .map(|coord| F2Dot14::from_f32(*coord))
            .collect::<Vec<_>>();
        gvar.phantom_point_deltas(&glyf, &loca, &coords, glyph_id)
            .unwrap()
            .unwrap()
            .map(|delta| delta.map(Fixed::to_f32))
            .map(|p| (p.x, p.y))
    }

    // fuzzer: add with overflow when computing glyph data range
    // ref: <https://g-issues.oss-fuzz.com/issues/385918147>
    #[test]
    fn avoid_data_range_overflow() {
        // Construct a gvar table with data offsets that overflow
        // a u32
        let mut buf = BeBuffer::new();
        // major/minor version
        buf = buf.push(1u16).push(0u16);
        // axis count
        buf = buf.push(0u16);
        // shared tuple count and offset
        buf = buf.push(0u16).push(0u32);
        // glyph count = 1
        buf = buf.push(1u16);
        // flags, bit 1 = 32 bit offsets
        buf = buf.push(1u16);
        // variation data offset
        buf = buf.push(u32::MAX - 10);
        // two 32-bit entries that overflow when added to the above offset
        buf = buf.push(0u32).push(11u32);
        let gvar = Gvar::read(buf.data().into()).unwrap();
        // don't panic with overflow!
        let _ = gvar.data_range_for_gid(GlyphId::new(0));
    }

    // Test that we select the correct component to derive metrics from,
    // considering ambiguity. Only covers the shallow case.
    #[test]
    fn follow_use_my_metrics() {
        // Load the font and required tables
        let font = FontRef::new(font_test_data::gvar::USE_MY_METRICS).unwrap();
        let glyf = font.glyf().unwrap();
        let loca = font.loca(None).unwrap();
        let post = font.post().unwrap();

        // Grab names for test quality-of-life and legibility
        let gids = (0..post.num_glyphs().unwrap())
            .map(GlyphId16::new)
            .map(|gid| (post.glyph_name(gid).unwrap(), gid))
            .collect::<HashMap<_, _>>();

        // Test various cases
        let (source, _) =
            find_glyph_and_point_count(&glyf, &loca, gids["neither"].into(), 5).unwrap();
        assert_eq!(
            source, gids["neither"],
            "a composite without any USE_MY_METRICS components should use its own metrics"
        );

        let (source, _) =
            find_glyph_and_point_count(&glyf, &loca, gids["firstonly"].into(), 5).unwrap();
        assert_eq!(
            source, gids["first"],
            "a composite with a single USE_MY_METRICS component should use that component's metrics"
        );

        let (source, _) = find_glyph_and_point_count(&glyf, &loca, gids["both"].into(), 5).unwrap();
        assert_eq!(
            source, gids["second"],
            "a composite with multiple USE_MY_METRICS components should use the last flagged component's metrics"
         );
    }
}
