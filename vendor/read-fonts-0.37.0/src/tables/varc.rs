//! the [VARC (Variable Composite/Component)](https://github.com/harfbuzz/boring-expansion-spec/blob/main/VARC.md) table

use super::variations::PackedDeltas;
pub use super::{
    layout::{Condition, CoverageTable},
    postscript::Index2,
};

#[cfg(feature = "libm")]
#[allow(unused_imports)]
use core_maths::*;

include!("../../generated/generated_varc.rs");

/// Let's us call self.something().get(i) instead of get(self.something(), i)
trait Get<'a> {
    fn get(self, nth: usize) -> Result<&'a [u8], ReadError>;
}

impl<'a> Get<'a> for Option<Result<Index2<'a>, ReadError>> {
    fn get(self, nth: usize) -> Result<&'a [u8], ReadError> {
        self.transpose()?
            .ok_or(ReadError::NullOffset)
            .and_then(|index| index.get(nth).map_err(|_| ReadError::OutOfBounds))
    }
}

impl Varc<'_> {
    /// Friendlier accessor than directly using raw data via [Index2]
    pub fn axis_indices(&self, nth: usize) -> Result<PackedDeltas<'_>, ReadError> {
        let raw = self.axis_indices_list().get(nth)?;
        Ok(PackedDeltas::consume_all(raw.into()))
    }

    /// Friendlier accessor than directly using raw data via [Index2]
    ///
    /// nth would typically be obtained by looking up a [GlyphId] in [Self::coverage].
    pub fn glyph(&self, nth: usize) -> Result<VarcGlyph<'_>, ReadError> {
        let raw = Some(self.var_composite_glyphs()).get(nth)?;
        Ok(VarcGlyph {
            table: self,
            data: raw.into(),
        })
    }
}

/// A VARC glyph doesn't have any root level attributes, it's just a list of components
///
/// <https://github.com/harfbuzz/boring-expansion-spec/blob/main/VARC.md#variable-composite-description>
pub struct VarcGlyph<'a> {
    table: &'a Varc<'a>,
    data: FontData<'a>,
}

impl<'a> VarcGlyph<'a> {
    /// <https://github.com/fonttools/fonttools/blob/5e6b12d12fa08abafbeb7570f47707fbedf69a45/Lib/fontTools/ttLib/tables/otTables.py#L404-L409>
    pub fn components(&self) -> impl Iterator<Item = Result<VarcComponent<'a>, ReadError>> {
        VarcComponentIter {
            table: self.table,
            cursor: self.data.cursor(),
        }
    }
}

struct VarcComponentIter<'a> {
    table: &'a Varc<'a>,
    cursor: Cursor<'a>,
}

impl<'a> Iterator for VarcComponentIter<'a> {
    type Item = Result<VarcComponent<'a>, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }
        Some(VarcComponent::parse(self.table, &mut self.cursor))
    }
}

#[allow(dead_code)] // TEMPORARY
pub struct VarcComponent<'a> {
    flags: VarcFlags,
    gid: GlyphId,
    condition_index: Option<u32>,
    axis_indices_index: Option<u32>,
    axis_values: Option<PackedDeltas<'a>>,
    axis_values_var_index: Option<u32>,
    transform_var_index: Option<u32>,
    transform: DecomposedTransform,
}

impl<'a> VarcComponent<'a> {
    /// Requires access to VARC fields to fully parse.
    ///
    ///  * HarfBuzz [VarComponent::get_path_at](https://github.com/harfbuzz/harfbuzz/blob/0c2f5ecd51d11e32836ee136a1bc765d650a4ec0/src/OT/Var/VARC/VARC.cc#L132)
    // TODO: do we want to be able to parse into an existing glyph to avoid allocation?
    fn parse(table: &Varc, cursor: &mut Cursor<'a>) -> Result<Self, ReadError> {
        let raw_flags = cursor.read_u32_var()?;
        let flags = VarcFlags::from_bits_truncate(raw_flags);
        // Ref https://github.com/harfbuzz/boring-expansion-spec/blob/main/VARC.md#variable-component-record

        // This is a GlyphID16 if GID_IS_24BIT bit of flags is clear, else GlyphID24.
        let gid = if flags.contains(VarcFlags::GID_IS_24BIT) {
            GlyphId::new(cursor.read::<Uint24>()?.to_u32())
        } else {
            GlyphId::from(cursor.read::<u16>()?)
        };

        let condition_index = if flags.contains(VarcFlags::HAVE_CONDITION) {
            Some(cursor.read_u32_var()?)
        } else {
            None
        };

        let (axis_indices_index, axis_values) = if flags.contains(VarcFlags::HAVE_AXES) {
            // <https://github.com/harfbuzz/harfbuzz/blob/0c2f5ecd51d11e32836ee136a1bc765d650a4ec0/src/OT/Var/VARC/VARC.cc#L195-L206>
            let axis_indices_index = cursor.read_u32_var()?;
            let num_axis_values = table.axis_indices(axis_indices_index as usize)?.count();
            // we need to consume num_axis_values entries in packed delta format
            let deltas = if num_axis_values > 0 {
                let Some(data) = cursor.remaining() else {
                    return Err(ReadError::OutOfBounds);
                };
                let deltas = PackedDeltas::new(data, num_axis_values);
                *cursor = deltas.iter().end(); // jump past the packed deltas
                Some(deltas)
            } else {
                None
            };
            (Some(axis_indices_index), deltas)
        } else {
            (None, None)
        };

        let axis_values_var_index = flags
            .contains(VarcFlags::AXIS_VALUES_HAVE_VARIATION)
            .then(|| cursor.read_u32_var())
            .transpose()?;

        let transform_var_index = if flags.contains(VarcFlags::TRANSFORM_HAS_VARIATION) {
            Some(cursor.read_u32_var()?)
        } else {
            None
        };

        let mut transform = DecomposedTransform::default();
        if flags.contains(VarcFlags::HAVE_TRANSLATE_X) {
            transform.translate_x = cursor.read::<FWord>()?.to_i16() as f64
        }
        if flags.contains(VarcFlags::HAVE_TRANSLATE_Y) {
            transform.translate_y = cursor.read::<FWord>()?.to_i16() as f64
        }
        if flags.contains(VarcFlags::HAVE_ROTATION) {
            transform.rotation = cursor.read::<F4Dot12>()?.to_f32() as f64
        }
        if flags.contains(VarcFlags::HAVE_SCALE_X) {
            transform.scale_x = cursor.read::<F6Dot10>()?.to_f32() as f64
        }
        transform.scale_y = if flags.contains(VarcFlags::HAVE_SCALE_Y) {
            cursor.read::<F6Dot10>()?.to_f32() as f64
        } else {
            transform.scale_x
        };
        if flags.contains(VarcFlags::HAVE_SKEW_X) {
            transform.skew_x = cursor.read::<F4Dot12>()?.to_f32() as f64
        }
        if flags.contains(VarcFlags::HAVE_SKEW_Y) {
            transform.skew_y = cursor.read::<F4Dot12>()?.to_f32() as f64
        }
        if flags.contains(VarcFlags::HAVE_TCENTER_X) {
            transform.center_x = cursor.read::<FWord>()?.to_i16() as f64
        }
        if flags.contains(VarcFlags::HAVE_TCENTER_Y) {
            transform.center_y = cursor.read::<FWord>()?.to_i16() as f64
        }

        // Optional, process and discard one uint32var per each set bit in RESERVED_MASK.
        let num_reserved = (raw_flags & VarcFlags::RESERVED_MASK.bits).count_ones();
        for _ in 0..num_reserved {
            cursor.read_u32_var()?;
        }
        Ok(VarcComponent {
            flags,
            gid,
            condition_index,
            axis_indices_index,
            axis_values,
            axis_values_var_index,
            transform_var_index,
            transform,
        })
    }
}

/// <https://github.com/fonttools/fonttools/blob/5e6b12d12fa08abafbeb7570f47707fbedf69a45/Lib/fontTools/misc/transform.py#L410>
pub struct DecomposedTransform {
    translate_x: f64,
    translate_y: f64,
    rotation: f64, // degrees, counter-clockwise
    scale_x: f64,
    scale_y: f64,
    skew_x: f64,
    skew_y: f64,
    center_x: f64,
    center_y: f64,
}

impl Default for DecomposedTransform {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            rotation: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
}

impl DecomposedTransform {
    /// Convert decomposed form to 2x3 matrix form.
    ///
    /// The first two values are x,y x-basis vector,
    /// the second 2 values are x,y y-basis vector, and the third 2 are translation.
    ///
    /// In augmented matrix
    /// form, if this method returns `[a, b, c, d, e, f]` that is taken as:
    ///
    /// ```text
    /// | a c e |
    /// | b d f |
    /// | 0 0 1 |
    /// ```
    ///
    /// References:
    /// * FontTools Python implementation <https://github.com/fonttools/fonttools/blob/5e6b12d12fa08abafbeb7570f47707fbedf69a45/Lib/fontTools/misc/transform.py#L484-L500>
    /// * Wikipedia [affine transformation](https://en.wikipedia.org/wiki/Affine_transformation)
    pub fn matrix(&self) -> [f64; 6] {
        // Python: t.translate(self.translateX + self.tCenterX, self.translateY + self.tCenterY)
        let mut transform = [
            1.0,
            0.0,
            0.0,
            1.0,
            self.translate_x + self.center_x,
            self.translate_y + self.center_y,
        ];

        // TODO: this produces very small floats for rotations, e.g. 90 degree rotation a basic scale
        // puts 1.2246467991473532e-16 into [0]. Should we special case? Round?

        // Python: t = t.rotate(math.radians(self.rotation))
        if self.rotation != 0.0 {
            let (s, c) = (self.rotation).to_radians().sin_cos();
            transform = transform.transform([c, s, -s, c, 0.0, 0.0]);
        }

        // Python: t = t.scale(self.scaleX, self.scaleY)
        if (self.scale_x, self.scale_y) != (1.0, 1.0) {
            transform = transform.transform([self.scale_x, 0.0, 0.0, self.scale_y, 0.0, 0.0]);
        }

        // Python: t = t.skew(math.radians(self.skewX), math.radians(self.skewY))
        if (self.skew_x, self.skew_y) != (0.0, 0.0) {
            transform = transform.transform([
                1.0,
                self.skew_y.to_radians().tan(),
                self.skew_x.to_radians().tan(),
                1.0,
                0.0,
                0.0,
            ])
        }

        // Python: t = t.translate(-self.tCenterX, -self.tCenterY)
        if (self.center_x, self.center_y) != (0.0, 0.0) {
            transform = transform.transform([1.0, 0.0, 0.0, 1.0, -self.center_x, -self.center_y]);
        }

        transform
    }
}

trait Transform {
    fn transform(self, other: Self) -> Self;
}

impl Transform for [f64; 6] {
    fn transform(self, other: Self) -> Self {
        // Shamelessly copied from kurbo Affine Mul
        [
            self[0] * other[0] + self[2] * other[1],
            self[1] * other[0] + self[3] * other[1],
            self[0] * other[2] + self[2] * other[3],
            self[1] * other[2] + self[3] * other[3],
            self[0] * other[4] + self[2] * other[5] + self[4],
            self[1] * other[4] + self[3] * other[5] + self[5],
        ]
    }
}

impl<'a> MultiItemVariationData<'a> {
    /// An [Index2] where each item is a [PackedDeltas]
    pub fn delta_sets(&self) -> Result<Index2<'a>, ReadError> {
        Index2::read(self.raw_delta_sets().into())
    }

    /// Read a specific delta set.
    ///
    /// Equivalent to calling [Self::delta_sets], fetching item i, and parsing as [PackedDeltas]
    pub fn delta_set(&self, i: usize) -> Result<PackedDeltas<'a>, ReadError> {
        let index = self.delta_sets()?;
        let raw_deltas = index.get(i).map_err(|_| ReadError::OutOfBounds)?;
        Ok(PackedDeltas::consume_all(raw_deltas.into()))
    }
}

#[cfg(test)]
mod tests {
    use types::GlyphId16;

    use crate::{FontRef, ReadError, TableProvider};

    use super::{Condition, DecomposedTransform, Varc};

    impl Varc<'_> {
        fn conditions(&self) -> impl Iterator<Item = Condition<'_>> {
            self.condition_list()
                .expect("A condition list is present")
                .expect("We could read the condition list")
                .conditions()
                .iter()
                .enumerate()
                .map(|(i, c)| c.unwrap_or_else(|e| panic!("condition {i} {e}")))
        }

        fn axis_indices_count(&self) -> Result<usize, ReadError> {
            let Some(axis_indices_list) = self.axis_indices_list() else {
                return Ok(0);
            };
            let axis_indices_list = axis_indices_list?;
            Ok(axis_indices_list.count() as usize)
        }
    }

    fn round6(v: f64) -> f64 {
        (v * 1_000_000.0).round() / 1_000_000.0
    }

    trait Round {
        fn round_for_test(self) -> Self;
    }

    impl Round for [f64; 6] {
        fn round_for_test(self) -> Self {
            [
                round6(self[0]),
                round6(self[1]),
                round6(self[2]),
                round6(self[3]),
                round6(self[4]),
                round6(self[5]),
            ]
        }
    }

    #[test]
    fn read_cjk_0x6868() {
        let font = FontRef::new(font_test_data::varc::CJK_6868).unwrap();
        let table = font.varc().unwrap();
        table.coverage().unwrap(); // should have coverage
    }

    #[test]
    fn identify_all_conditional_types() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();

        // We should have all 5 condition types in order
        assert_eq!(
            (1..=5).collect::<Vec<_>>(),
            table.conditions().map(|c| c.format()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_condition_format1_axis_range() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        let Some(Condition::Format1AxisRange(condition)) =
            table.conditions().find(|c| c.format() == 1)
        else {
            panic!("No such item");
        };

        assert_eq!(
            (0, 0.5, 1.0),
            (
                condition.axis_index(),
                condition.filter_range_min_value().to_f32(),
                condition.filter_range_max_value().to_f32(),
            )
        );
    }

    #[test]
    fn read_condition_format2_variable_value() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        let Some(Condition::Format2VariableValue(condition)) =
            table.conditions().find(|c| c.format() == 2)
        else {
            panic!("No such item");
        };

        assert_eq!((1, 2), (condition.default_value(), condition.var_index(),));
    }

    #[test]
    fn read_condition_format3_and() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        let Some(Condition::Format3And(condition)) = table.conditions().find(|c| c.format() == 3)
        else {
            panic!("No such item");
        };

        // Should reference a format 1 and a format 2
        assert_eq!(
            vec![1, 2],
            condition
                .conditions()
                .iter()
                .map(|c| c.unwrap().format())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_condition_format4_or() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        let Some(Condition::Format4Or(condition)) = table.conditions().find(|c| c.format() == 4)
        else {
            panic!("No such item");
        };

        // Should reference a format 1 and a format 2
        assert_eq!(
            vec![1, 2],
            condition
                .conditions()
                .iter()
                .map(|c| c.unwrap().format())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_condition_format5_negate() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        let Some(Condition::Format5Negate(condition)) =
            table.conditions().find(|c| c.format() == 5)
        else {
            panic!("No such item");
        };

        // Should reference a format 1
        assert_eq!(1, condition.condition().unwrap().format(),);
    }

    #[test]
    fn read_axis_indices_list() {
        let font = FontRef::new(font_test_data::varc::CONDITIONALS).unwrap();
        let table = font.varc().unwrap();
        assert_eq!(table.axis_indices_count().unwrap(), 2);
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            table.axis_indices(1).unwrap().iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_glyph_6868() {
        let font = FontRef::new(font_test_data::varc::CJK_6868).unwrap();
        let gid = font.cmap().unwrap().map_codepoint(0x6868_u32).unwrap();
        let table = font.varc().unwrap();
        let idx = table.coverage().unwrap().get(gid).unwrap();

        let glyph = table.glyph(idx as usize).unwrap();
        assert_eq!(
            vec![GlyphId16::new(2), GlyphId16::new(5), GlyphId16::new(7)],
            glyph
                .components()
                .map(|c| c.unwrap().gid)
                .collect::<Vec<_>>()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_scale_to_matrix() {
        let scale_x = 2.0;
        let scale_y = 3.0;
        assert_eq!(
            [scale_x, 0.0, 0.0, scale_y, 0.0, 0.0],
            DecomposedTransform {
                scale_x,
                scale_y,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_rotate_to_matrix() {
        assert_eq!(
            [0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
            DecomposedTransform {
                rotation: 90.0,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_skew_to_matrix() {
        let skew_x: f64 = 30.0;
        let skew_y: f64 = -60.0;
        assert_eq!(
            [
                1.0,
                round6(skew_y.to_radians().tan()),
                round6(skew_x.to_radians().tan()),
                1.0,
                0.0,
                0.0
            ],
            DecomposedTransform {
                skew_x,
                skew_y,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_scale_rotate_to_matrix() {
        let scale_x = 2.0;
        let scale_y = 3.0;
        assert_eq!(
            [0.0, scale_x, -scale_y, 0.0, 0.0, 0.0],
            DecomposedTransform {
                scale_x,
                scale_y,
                rotation: 90.0,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_scale_rotate_translate_to_matrix() {
        assert_eq!(
            [0.0, 2.0, -1.0, 0.0, 10.0, 20.0],
            DecomposedTransform {
                scale_x: 2.0,
                rotation: 90.0,
                translate_x: 10.0,
                translate_y: 20.0,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_scale_skew_translate_to_matrix() {
        assert_eq!(
            [-0.866025, 5.5, -0.5, 3.175426, 10.0, 20.0],
            DecomposedTransform {
                scale_x: 2.0,
                scale_y: 3.0,
                rotation: 30.0,
                skew_x: 30.0,
                skew_y: 60.0,
                translate_x: 10.0,
                translate_y: 20.0,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    // Expected created using the Python DecomposedTransform
    #[test]
    fn decomposed_rotate_around_to_matrix() {
        assert_eq!(
            [1.732051, 1.0, -0.5, 0.866025, 10.267949, 19.267949],
            DecomposedTransform {
                scale_x: 2.0,
                rotation: 30.0,
                translate_x: 10.0,
                translate_y: 20.0,
                center_x: 1.0,
                center_y: 2.0,
                ..Default::default()
            }
            .matrix()
            .round_for_test()
        );
    }

    #[test]
    fn read_multivar_store_region_list() {
        let font = FontRef::new(font_test_data::varc::CJK_6868).unwrap();
        let table = font.varc().unwrap();
        let varstore = table.multi_var_store().unwrap().unwrap();
        let regions = varstore.region_list().unwrap().regions();

        let sparse_regions = regions
            .iter()
            .map(|r| {
                r.unwrap()
                    .region_axis_offsets()
                    .iter()
                    .map(|a| {
                        (
                            a.axis_index(),
                            a.start().to_f32(),
                            a.peak().to_f32(),
                            a.end().to_f32(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Check a sampling of the regions
        assert_eq!(
            vec![
                vec![(0, 0.0, 1.0, 1.0),],
                vec![(0, 0.0, 1.0, 1.0), (1, 0.0, 1.0, 1.0),],
                vec![(6, -1.0, -1.0, 0.0),],
            ],
            [0, 2, 38]
                .into_iter()
                .map(|i| sparse_regions[i].clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn read_multivar_store_delta_sets() {
        let font = FontRef::new(font_test_data::varc::CJK_6868).unwrap();
        let table = font.varc().unwrap();
        let varstore = table.multi_var_store().unwrap().unwrap();
        assert_eq!(
            vec![(3, 6), (33, 6), (10, 5), (25, 8),],
            varstore
                .variation_data()
                .iter()
                .map(|d| d.unwrap())
                .map(|d| (d.region_index_count(), d.delta_sets().unwrap().count()))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            vec![-1, 33, 0, 0, 0, 0],
            varstore
                .variation_data()
                .get(0)
                .unwrap()
                .delta_set(5)
                .unwrap()
                .iter()
                .collect::<Vec<_>>()
        )
    }
}
