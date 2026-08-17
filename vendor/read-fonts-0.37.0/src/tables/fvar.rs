//! The [Font Variations](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar) table

include!("../../generated/generated_fvar.rs");

#[path = "./instance_record.rs"]
mod instance_record;

use super::{
    avar::Avar,
    variations::{DeltaSetIndex, FloatItemDeltaTarget},
};
pub use instance_record::InstanceRecord;

impl<'a> Fvar<'a> {
    /// Returns the array of variation axis records.
    pub fn axes(&self) -> Result<&'a [VariationAxisRecord], ReadError> {
        Ok(self.axis_instance_arrays()?.axes())
    }

    /// Returns the array of instance records.
    pub fn instances(&self) -> Result<ComputedArray<'a, InstanceRecord<'a>>, ReadError> {
        Ok(self.axis_instance_arrays()?.instances())
    }

    /// Converts user space coordinates provided by an unordered iterator
    /// of `(tag, value)` pairs to normalized coordinates in axis list order.
    ///
    /// Stores the resulting normalized coordinates in the given slice.
    ///
    /// * User coordinate tags that don't match an axis are ignored.
    /// * User coordinate values are clamped to the range of their associated
    ///   axis before normalization.
    /// * If more than one user coordinate is provided for the same tag, the
    ///   last one is used.
    /// * If no user coordinate for an axis is provided, the associated
    ///   coordinate is set to the normalized value 0.0, representing the
    ///   default location in variation space.
    /// * The length of `normalized_coords` should equal the number of axes
    ///   present in the this table. If the length is smaller, axes at
    ///   out of bounds indices are ignored. If the length is larger, the
    ///   excess entries will be filled with zeros.
    ///
    /// If the [`Avar`] table is provided, applies remapping of coordinates
    /// according to the specification.
    pub fn user_to_normalized(
        &self,
        avar: Option<&Avar>,
        user_coords: impl IntoIterator<Item = (Tag, Fixed)>,
        normalized_coords: &mut [F2Dot14],
    ) {
        normalized_coords.fill(F2Dot14::default());
        let axes = self.axes().unwrap_or_default();
        let avar_mappings = avar.map(|avar| avar.axis_segment_maps());
        for user_coord in user_coords {
            // To permit non-linear interpolation, iterate over all axes to ensure we match
            // multiple axes with the same tag:
            // https://github.com/PeterConstable/OT_Drafts/blob/master/NLI/UnderstandingNLI.md
            // We accept quadratic behavior here to avoid dynamic allocation and with the assumption
            // that fonts contain a relatively small number of axes.
            for (i, axis) in axes
                .iter()
                .enumerate()
                .filter(|(_, axis)| axis.axis_tag() == user_coord.0)
            {
                if let Some(target_coord) = normalized_coords.get_mut(i) {
                    let coord = axis.normalize(user_coord.1);
                    *target_coord = avar_mappings
                        .as_ref()
                        .and_then(|mappings| mappings.get(i).transpose().ok())
                        .flatten()
                        .map(|mapping| mapping.apply(coord))
                        .unwrap_or(coord)
                        .to_f2dot14();
                }
            }
        }
        let Some(avar) = avar else { return };
        if avar.version() == MajorMinor::VERSION_1_0 {
            return;
        }
        let var_store = avar.var_store();
        let var_index_map = avar.axis_index_map();

        let actual_len = axes.len().min(normalized_coords.len());
        let mut new_coords = [F2Dot14::ZERO; 64];
        if actual_len > 64 {
            // No avar2 for monster fonts.
            // <https://github.com/googlefonts/fontations/issues/1148>
            return;
        }

        let new_coords = &mut new_coords[..actual_len];
        let normalized_coords = &mut normalized_coords[..actual_len];
        new_coords.copy_from_slice(normalized_coords);

        for (i, v) in normalized_coords.iter().enumerate() {
            let var_index = if let Some(Ok(ref map)) = var_index_map {
                map.get(i as u32).ok()
            } else {
                Some(DeltaSetIndex {
                    outer: 0,
                    inner: i as u16,
                })
            };
            if var_index.is_none() {
                continue;
            }
            if let Some(Ok(varstore)) = var_store.as_ref() {
                if let Ok(delta) =
                    varstore.compute_float_delta(var_index.unwrap(), normalized_coords)
                {
                    new_coords[i] = F2Dot14::from_f32((*v).apply_float_delta(delta))
                        .clamp(F2Dot14::MIN, F2Dot14::MAX);
                }
            }
        }
        normalized_coords.copy_from_slice(new_coords);
    }
}

impl VariationAxisRecord {
    /// Returns a normalized coordinate for the given value.
    pub fn normalize(&self, mut value: Fixed) -> Fixed {
        use core::cmp::Ordering::*;
        let min_value = self.min_value();
        let default_value = self.default_value();
        // Make sure max is >= min to avoid potential panic in clamp.
        let max_value = self.max_value().max(min_value);
        value = value.clamp(min_value, max_value);
        value = match value.cmp(&default_value) {
            Less => {
                -((default_value.saturating_sub(value)) / (default_value.saturating_sub(min_value)))
            }
            Greater => {
                (value.saturating_sub(default_value)) / (max_value.saturating_sub(default_value))
            }
            Equal => Fixed::ZERO,
        };
        value.clamp(-Fixed::ONE, Fixed::ONE)
    }
}

#[cfg(test)]
mod tests {
    use crate::{FontRef, TableProvider};
    use types::{F2Dot14, Fixed, NameId, Tag};

    #[test]
    fn axes() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let fvar = font.fvar().unwrap();
        assert_eq!(fvar.axis_count(), 1);
        let wght = &fvar.axes().unwrap().first().unwrap();
        assert_eq!(wght.axis_tag(), Tag::new(b"wght"));
        assert_eq!(wght.min_value(), Fixed::from_f64(100.0));
        assert_eq!(wght.default_value(), Fixed::from_f64(400.0));
        assert_eq!(wght.max_value(), Fixed::from_f64(900.0));
        assert_eq!(wght.flags(), 0);
        assert_eq!(wght.axis_name_id(), NameId::new(257));
    }

    #[test]
    fn instances() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let fvar = font.fvar().unwrap();
        assert_eq!(fvar.instance_count(), 9);
        // There are 9 instances equally spaced from 100.0 to 900.0
        // with name id monotonically increasing starting at 258.
        let instances = fvar.instances().unwrap();
        for i in 0..9 {
            let value = 100.0 * (i + 1) as f64;
            let name_id = NameId::new(258 + i as u16);
            let instance = instances.get(i).unwrap();
            assert_eq!(instance.coordinates.len(), 1);
            assert_eq!(
                instance.coordinates.first().unwrap().get(),
                Fixed::from_f64(value)
            );
            assert_eq!(instance.subfamily_name_id, name_id);
            assert_eq!(instance.post_script_name_id, None);
        }
    }

    #[test]
    fn normalize() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let fvar = font.fvar().unwrap();
        let axis = fvar.axes().unwrap().first().unwrap();
        let values = [100.0, 220.0, 250.0, 400.0, 650.0, 900.0];
        let expected = [-1.0, -0.60001, -0.5, 0.0, 0.5, 1.0];
        for (value, expected) in values.into_iter().zip(expected) {
            assert_eq!(
                axis.normalize(Fixed::from_f64(value)),
                Fixed::from_f64(expected)
            );
        }
    }

    #[test]
    fn normalize_overflow() {
        // From: https://bugs.chromium.org/p/oss-fuzz/issues/detail?id=69787
        // & https://oss-fuzz.com/testcase?key=6159008335986688
        // fvar entry triggering overflow:
        // min: -26335.87451171875 def 8224.12548828125 max 8224.12548828125
        let test_case = &[
            79, 84, 84, 79, 0, 1, 32, 32, 255, 32, 32, 32, 102, 118, 97, 114, 32, 32, 32, 32, 0, 0,
            0, 28, 0, 0, 0, 41, 32, 0, 0, 0, 0, 1, 32, 32, 0, 2, 32, 32, 32, 32, 0, 0, 32, 32, 32,
            32, 32, 0, 0, 0, 0, 153, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        ];
        let font = FontRef::new(test_case).unwrap();
        let fvar = font.fvar().unwrap();
        let axis = fvar.axes().unwrap()[1];
        // Should not panic with "attempt to subtract with overflow".
        assert_eq!(
            axis.normalize(Fixed::from_f64(0.0)),
            Fixed::from_f64(-0.2509765625)
        );
    }

    #[test]
    fn user_to_normalized() {
        let font = FontRef::from_index(font_test_data::VAZIRMATN_VAR, 0).unwrap();
        let fvar = font.fvar().unwrap();
        let avar = font.avar().ok();
        let wght = Tag::new(b"wght");
        let axis = fvar.axes().unwrap()[0];
        let mut normalized_coords = [F2Dot14::default(); 1];
        // avar table maps 0.8 to 0.83875
        let avar_user = axis.default_value().to_f32()
            + (axis.max_value().to_f32() - axis.default_value().to_f32()) * 0.8;
        let avar_normalized = 0.83875;
        #[rustfmt::skip]
        let cases = [
            // (user, normalized)
            (-1000.0, -1.0f32),
            (100.0, -1.0),
            (200.0, -0.5),
            (400.0, 0.0),
            (900.0, 1.0),
            (avar_user, avar_normalized),
            (1251.5, 1.0),
        ];
        for (user, normalized) in cases {
            fvar.user_to_normalized(
                avar.as_ref(),
                [(wght, Fixed::from_f64(user as f64))],
                &mut normalized_coords,
            );
            assert_eq!(normalized_coords[0], F2Dot14::from_f32(normalized));
        }
    }

    #[test]
    fn avar2() {
        let font = FontRef::new(font_test_data::AVAR2_CHECKER).unwrap();
        let avar = font.avar().ok();
        let fvar = font.fvar().unwrap();
        let avar_axis = Tag::new(b"AVAR");
        let avwk_axis = Tag::new(b"AVWK");
        let mut normalized_coords = [F2Dot14::default(); 2];
        let cases = [
            ((100.0, 0.0), (1.0, 1.0)),
            ((50.0, 0.0), (0.5, 0.5)),
            ((0.0, 50.0), (0.0, 0.5)),
        ];
        for (user, expected) in cases {
            fvar.user_to_normalized(
                avar.as_ref(),
                [
                    (avar_axis, Fixed::from_f64(user.0)),
                    (avwk_axis, Fixed::from_f64(user.1)),
                ],
                &mut normalized_coords,
            );
            assert_eq!(normalized_coords[0], F2Dot14::from_f32(expected.0));
            assert_eq!(normalized_coords[1], F2Dot14::from_f32(expected.1));
        }
    }

    #[test]
    fn avar2_no_panic_with_wrong_size_coords_array() {
        // this font has 2 axes
        let font = FontRef::new(font_test_data::AVAR2_CHECKER).unwrap();
        let avar = font.avar().ok();
        let fvar = font.fvar().unwrap();
        // output array too small
        let mut normalized_coords = [F2Dot14::default(); 1];
        fvar.user_to_normalized(avar.as_ref(), [], &mut normalized_coords);
        // output array too large
        let mut normalized_coords = [F2Dot14::default(); 4];
        fvar.user_to_normalized(avar.as_ref(), [], &mut normalized_coords);
    }
}
