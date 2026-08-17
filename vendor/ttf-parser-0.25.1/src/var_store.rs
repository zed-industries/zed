//! Implementation of Item Variation Store
//!
//! <https://docs.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats#item-variation-store>

use crate::parser::{FromData, LazyArray16, NumFrom, Stream};
use crate::NormalizedCoordinate;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ItemVariationStore<'a> {
    data: &'a [u8],
    data_offsets: LazyArray16<'a, u32>,
    pub regions: VariationRegionList<'a>,
}

impl<'a> Default for ItemVariationStore<'a> {
    #[inline]
    fn default() -> Self {
        ItemVariationStore {
            data: &[],
            data_offsets: LazyArray16::new(&[]),
            regions: VariationRegionList {
                axis_count: 0,
                regions: LazyArray16::new(&[]),
            },
        }
    }
}

impl<'a> ItemVariationStore<'a> {
    #[inline]
    pub fn parse(mut s: Stream) -> Option<ItemVariationStore> {
        let data = s.tail()?;

        let mut regions_s = s.clone();
        let format = s.read::<u16>()?;
        if format != 1 {
            return None;
        }

        let region_list_offset = s.read::<u32>()?;
        let count = s.read::<u16>()?;
        let offsets = s.read_array16::<u32>(count)?;

        let regions = {
            regions_s.advance(usize::num_from(region_list_offset));
            // TODO: should be the same as in `fvar`
            let axis_count = regions_s.read::<u16>()?;
            let count = regions_s.read::<u16>()?;
            let total = count.checked_mul(axis_count)?;
            VariationRegionList {
                axis_count,
                regions: regions_s.read_array16::<RegionAxisCoordinatesRecord>(total)?,
            }
        };

        Some(ItemVariationStore {
            data,
            data_offsets: offsets,
            regions,
        })
    }

    pub fn region_indices(&self, index: u16) -> Option<LazyArray16<u16>> {
        // Offsets in bytes from the start of the item variation store
        // to each item variation data subtable.
        let offset = self.data_offsets.get(index)?;
        let mut s = Stream::new_at(self.data, usize::num_from(offset))?;
        s.skip::<u16>(); // item_count
        s.skip::<u16>(); // short_delta_count
        let count = s.read::<u16>()?;
        s.read_array16::<u16>(count)
    }

    pub fn parse_delta(
        &self,
        outer_index: u16,
        inner_index: u16,
        coordinates: &[NormalizedCoordinate],
    ) -> Option<f32> {
        let offset = self.data_offsets.get(outer_index)?;
        let mut s = Stream::new_at(self.data, usize::num_from(offset))?;
        let item_count = s.read::<u16>()?;
        let word_delta_count = s.read::<u16>()?;
        let region_index_count = s.read::<u16>()?;
        let region_indices = s.read_array16::<u16>(region_index_count)?;

        if inner_index >= item_count {
            return None;
        }

        let has_long_words = (word_delta_count & 0x8000) != 0;
        let word_delta_count = word_delta_count & 0x7FFF;

        // From the spec: The length of the data for each row, in bytes, is
        // regionIndexCount + (wordDeltaCount & WORD_DELTA_COUNT_MASK)
        // if the LONG_WORDS flag is not set, or 2 x that amount if the flag is set.
        let mut delta_set_len = word_delta_count + region_index_count;
        if has_long_words {
            delta_set_len *= 2;
        }

        s.advance(usize::from(inner_index).checked_mul(usize::from(delta_set_len))?);

        let mut delta = 0.0;
        let mut i = 0;
        while i < word_delta_count {
            let idx = region_indices.get(i)?;
            let num = if has_long_words {
                // TODO: use f64?
                s.read::<i32>()? as f32
            } else {
                f32::from(s.read::<i16>()?)
            };
            delta += num * self.regions.evaluate_region(idx, coordinates);
            i += 1;
        }

        while i < region_index_count {
            let idx = region_indices.get(i)?;
            let num = if has_long_words {
                f32::from(s.read::<i16>()?)
            } else {
                f32::from(s.read::<i8>()?)
            };
            delta += num * self.regions.evaluate_region(idx, coordinates);
            i += 1;
        }

        Some(delta)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VariationRegionList<'a> {
    axis_count: u16,
    regions: LazyArray16<'a, RegionAxisCoordinatesRecord>,
}

impl<'a> VariationRegionList<'a> {
    #[inline]
    pub(crate) fn evaluate_region(&self, index: u16, coordinates: &[NormalizedCoordinate]) -> f32 {
        let mut v = 1.0;
        for (i, coord) in coordinates.iter().enumerate() {
            let region = match self.regions.get(index * self.axis_count + i as u16) {
                Some(r) => r,
                None => return 0.0,
            };

            let factor = region.evaluate_axis(coord.get());
            if factor == 0.0 {
                return 0.0;
            }

            v *= factor;
        }

        v
    }
}

#[derive(Clone, Copy, Debug)]
struct RegionAxisCoordinatesRecord {
    start_coord: i16,
    peak_coord: i16,
    end_coord: i16,
}

impl RegionAxisCoordinatesRecord {
    #[inline]
    pub fn evaluate_axis(&self, coord: i16) -> f32 {
        let start = self.start_coord;
        let peak = self.peak_coord;
        let end = self.end_coord;

        if start > peak || peak > end {
            return 1.0;
        }

        if start < 0 && end > 0 && peak != 0 {
            return 1.0;
        }

        if peak == 0 || coord == peak {
            return 1.0;
        }

        if coord <= start || end <= coord {
            return 0.0;
        }

        if coord < peak {
            f32::from(coord - start) / f32::from(peak - start)
        } else {
            f32::from(end - coord) / f32::from(end - peak)
        }
    }
}

impl FromData for RegionAxisCoordinatesRecord {
    const SIZE: usize = 6;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(RegionAxisCoordinatesRecord {
            start_coord: s.read::<i16>()?,
            peak_coord: s.read::<i16>()?,
            end_coord: s.read::<i16>()?,
        })
    }
}
