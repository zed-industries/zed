//! An [Axis Variations Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/avar) implementation.

use core::convert::TryFrom;

use crate::parser::{FromData, LazyArray16, Stream};
use crate::NormalizedCoordinate;

/// An axis value map.
#[derive(Clone, Copy, Debug)]
pub struct AxisValueMap {
    /// A normalized coordinate value obtained using default normalization.
    pub from_coordinate: i16,
    /// The modified, normalized coordinate value.
    pub to_coordinate: i16,
}

impl FromData for AxisValueMap {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(AxisValueMap {
            from_coordinate: s.read::<i16>()?,
            to_coordinate: s.read::<i16>()?,
        })
    }
}

/// A list of segment maps.
///
/// Can be empty.
///
/// The internal data layout is not designed for random access,
/// therefore we're not providing the `get()` method and only an iterator.
#[derive(Clone, Copy)]
pub struct SegmentMaps<'a> {
    count: u16,
    data: &'a [u8],
}

impl<'a> SegmentMaps<'a> {
    /// Returns the number of segments.
    pub fn len(&self) -> u16 {
        self.count
    }

    /// Checks if there are any segments.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl core::fmt::Debug for SegmentMaps<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "SegmentMaps {{ ... }}")
    }
}

impl<'a> IntoIterator for SegmentMaps<'a> {
    type Item = LazyArray16<'a, AxisValueMap>;
    type IntoIter = SegmentMapsIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SegmentMapsIter {
            stream: Stream::new(self.data),
        }
    }
}

/// An iterator over maps.
#[allow(missing_debug_implementations)]
pub struct SegmentMapsIter<'a> {
    stream: Stream<'a>,
}

impl<'a> Iterator for SegmentMapsIter<'a> {
    type Item = LazyArray16<'a, AxisValueMap>;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.stream.read::<u16>()?;
        self.stream.read_array16::<AxisValueMap>(count)
    }
}

/// An [Axis Variations Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/avar).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// The segment maps array — one segment map for each axis
    /// in the order of axes specified in the `fvar` table.
    pub segment_maps: SegmentMaps<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        s.skip::<u16>(); // reserved
        Some(Self {
            segment_maps: SegmentMaps {
                // TODO: check that `axisCount` is the same as in `fvar`?
                count: s.read::<u16>()?,
                data: s.tail()?,
            },
        })
    }

    /// Maps a single coordinate
    pub fn map_coordinate(
        &self,
        coordinates: &mut [NormalizedCoordinate],
        coordinate_index: usize,
    ) -> Option<()> {
        if usize::from(self.segment_maps.count) != coordinates.len() {
            return None;
        }

        if let Some((map, coord)) = self
            .segment_maps
            .into_iter()
            .zip(coordinates)
            .nth(coordinate_index)
        {
            *coord = NormalizedCoordinate::from(map_value(&map, coord.0)?);
        }

        Some(())
    }
}

fn map_value(map: &LazyArray16<AxisValueMap>, value: i16) -> Option<i16> {
    // This code is based on harfbuzz implementation.

    if map.is_empty() {
        return Some(value);
    } else if map.len() == 1 {
        let record = map.get(0)?;
        return Some(value - record.from_coordinate + record.to_coordinate);
    }

    let record_0 = map.get(0)?;
    if value <= record_0.from_coordinate {
        return Some(value - record_0.from_coordinate + record_0.to_coordinate);
    }

    let mut i = 1;
    while i < map.len() && value > map.get(i)?.from_coordinate {
        i += 1;
    }

    if i == map.len() {
        i -= 1;
    }

    let record_curr = map.get(i)?;
    let curr_from = record_curr.from_coordinate;
    let curr_to = record_curr.to_coordinate;
    if value >= curr_from {
        return Some(value - curr_from + curr_to);
    }

    let record_prev = map.get(i - 1)?;
    let prev_from = record_prev.from_coordinate;
    let prev_to = record_prev.to_coordinate;
    if prev_from == curr_from {
        return Some(prev_to);
    }

    let curr_from = i32::from(curr_from);
    let curr_to = i32::from(curr_to);
    let prev_from = i32::from(prev_from);
    let prev_to = i32::from(prev_to);

    let denom = curr_from - prev_from;
    let k = (curr_to - prev_to) * (i32::from(value) - prev_from) + denom / 2;
    let value = prev_to + k / denom;
    i16::try_from(value).ok()
}
