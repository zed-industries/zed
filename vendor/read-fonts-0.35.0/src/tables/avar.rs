//! The [Axis Variations](https://docs.microsoft.com/en-us/typography/opentype/spec/avar) table

use super::variations::{DeltaSetIndexMap, ItemVariationStore};

include!("../../generated/generated_avar.rs");

impl SegmentMaps<'_> {
    /// Applies the piecewise linear mapping to the specified coordinate,
    /// matching HarfBuzz's extended avar behavior.
    pub fn apply(&self, coord: Fixed) -> Fixed {
        let maps = self.axis_value_maps();
        let len = maps.len();

        // Helpers
        #[inline]
        fn from(m: &AxisValueMap) -> Fixed {
            m.from_coordinate().to_fixed()
        }
        #[inline]
        fn to_(m: &AxisValueMap) -> Fixed {
            m.to_coordinate().to_fixed()
        }

        // Special-cases (error-recovery / robustness), as in HB:
        if len < 2 {
            return if len == 0 {
                coord
            } else {
                // len == 1: shift by the single mapping delta
                coord - from(&maps[0]) + to_(&maps[0])
            };
        }

        // Now we have at least two mappings.
        // Trim "duplicate" -1/+1 caps in the wild (CoreText quirks), like HB:
        let neg1 = Fixed::from_i32(-1);
        let pos1 = Fixed::from_i32(1);

        let mut start = 0usize;
        let mut end = len;

        if from(&maps[start]) == neg1 && to_(&maps[start]) == neg1 && from(&maps[start + 1]) == neg1
        {
            start += 1;
        }

        if from(&maps[end - 1]) == pos1
            && to_(&maps[end - 1]) == pos1
            && from(&maps[end - 2]) == pos1
        {
            end -= 1;
        }

        // Look for exact match first; handle multiple identical "from" entries.
        let mut i = start;
        while i < end {
            if coord == from(&maps[i]) {
                break;
            }
            i += 1;
        }

        if i < end {
            // Found at least one exact match; check if there are consecutive equals.
            let mut j = i;
            while j + 1 < end && coord == from(&maps[j + 1]) {
                j += 1;
            }

            // [i, j] inclusive are exact matches.

            // Spec-compliant case: exactly one -> return its 'to'.
            if i == j {
                return to_(&maps[i]);
            }

            // Exactly three -> return the middle one.
            if i + 2 == j {
                return to_(&maps[i + 1]);
            }

            // Otherwise, ignore the middle ones.
            // Return the mapping closer to 0 on the *from* side, following HB:
            if coord < Fixed::ZERO {
                return to_(&maps[j]);
            }
            if coord > Fixed::ZERO {
                return to_(&maps[i]);
            }

            // coord == 0: choose the one with smaller |to|.
            let ti = to_(&maps[i]);
            let tj = to_(&maps[j]);
            return if ti.abs() <= tj.abs() { ti } else { tj };
        }

        // Not an exact match: find the segment for interpolation.
        let mut k = start;
        while k < end {
            if coord < from(&maps[k]) {
                break;
            }
            k += 1;
        }

        if k == 0 {
            // Before all segments: shift by first mapping delta
            return coord - from(&maps[0]) + to_(&maps[0]);
        }
        if k == end {
            // After all segments: shift by last mapping delta
            return coord - from(&maps[end - 1]) + to_(&maps[end - 1]);
        }

        // Interpolate between maps[k-1] and maps[k].
        let before = &maps[k - 1];
        let after = &maps[k];

        let bf = from(before);
        let bt = to_(before);
        let af = from(after);
        let at = to_(after);

        let denom = af - bf; // guaranteed non-zero by construction
        bt + (at - bt).mul_div(coord - bf, denom)
    }
}

impl VarSize for SegmentMaps<'_> {
    type Size = u16;

    fn read_len_at(data: FontData, pos: usize) -> Option<usize> {
        Some(
            data.read_at::<u16>(pos).ok()? as usize * AxisValueMap::RAW_BYTE_LEN
                + u16::RAW_BYTE_LEN,
        )
    }
}

impl<'a> FontRead<'a> for SegmentMaps<'a> {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        let mut cursor = data.cursor();
        let position_map_count: BigEndian<u16> = cursor.read_be()?;
        let axis_value_maps = cursor.read_array(position_map_count.get() as _)?;
        Ok(SegmentMaps {
            position_map_count,
            axis_value_maps,
        })
    }
}

#[cfg(test)]
mod tests {

    use font_test_data::bebuffer::BeBuffer;

    use super::*;
    use crate::{FontRef, TableProvider};

    fn value_map(from: f32, to: f32) -> [F2Dot14; 2] {
        [F2Dot14::from_f32(from), F2Dot14::from_f32(to)]
    }

    // for the purpose of testing it is easier for us to use an array
    // instead of a concrete type, since we can write that into BeBuffer
    impl PartialEq<[F2Dot14; 2]> for AxisValueMap {
        fn eq(&self, other: &[F2Dot14; 2]) -> bool {
            self.from_coordinate == other[0] && self.to_coordinate == other[1]
        }
    }

    #[test]
    fn segment_maps() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let avar = font.avar().unwrap();
        assert_eq!(avar.axis_count(), 1);
        let expected_segment_maps = &[vec![
            value_map(-1.0, -1.0),
            value_map(-0.6667, -0.5),
            value_map(-0.3333, -0.25),
            value_map(0.0, 0.0),
            value_map(0.2, 0.3674),
            value_map(0.4, 0.52246),
            value_map(0.6, 0.67755),
            value_map(0.8, 0.83875),
            value_map(1.0, 1.0),
        ]];
        let segment_maps = avar
            .axis_segment_maps()
            .iter()
            .map(|segment_map| segment_map.unwrap().axis_value_maps().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(segment_maps, expected_segment_maps);
    }

    #[test]
    fn segment_maps_multi_axis() {
        let segment_one_maps = [
            value_map(-1.0, -1.0),
            value_map(-0.6667, -0.5),
            value_map(-0.3333, -0.25),
        ];
        let segment_two_maps = [value_map(0.8, 0.83875), value_map(1.0, 1.0)];

        let data = BeBuffer::new()
            .push(MajorMinor::VERSION_1_0)
            .push(0u16) // reserved
            .push(2u16) // axis count
            // segment map one
            .push(3u16) // position count
            .extend(segment_one_maps[0])
            .extend(segment_one_maps[1])
            .extend(segment_one_maps[2])
            // segment map two
            .push(2u16) // position count
            .extend(segment_two_maps[0])
            .extend(segment_two_maps[1]);

        let avar = super::Avar::read(data.data().into()).unwrap();
        assert_eq!(avar.axis_segment_maps().iter().count(), 2);
        assert_eq!(
            avar.axis_segment_maps()
                .get(0)
                .unwrap()
                .unwrap()
                .axis_value_maps,
            segment_one_maps,
        );
        assert_eq!(
            avar.axis_segment_maps()
                .get(1)
                .unwrap()
                .unwrap()
                .axis_value_maps,
            segment_two_maps,
        );
    }

    #[test]
    fn piecewise_linear() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let avar = font.avar().unwrap();
        let segment_map = avar.axis_segment_maps().get(0).unwrap().unwrap();
        let coords = [-1.0, -0.5, 0.0, 0.5, 1.0];
        let expected_result = [-1.0, -0.375, 0.0, 0.600006103515625, 1.0];
        assert_eq!(
            &expected_result[..],
            &coords
                .iter()
                .map(|coord| segment_map.apply(Fixed::from_f64(*coord)).to_f64())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn avar2() {
        let font = FontRef::new(font_test_data::AVAR2_CHECKER).unwrap();
        let avar = font.avar().unwrap();
        assert_eq!(avar.version(), MajorMinor::VERSION_2_0);
        assert!(avar.axis_index_map_offset().is_some());
        assert!(avar.var_store_offset().is_some());
        assert!(avar.var_store().is_some());
    }
}
