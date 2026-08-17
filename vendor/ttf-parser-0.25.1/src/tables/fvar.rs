//! A [Font Variations Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/fvar) implementation.

use core::num::NonZeroU16;

use crate::parser::{f32_bound, Fixed, FromData, LazyArray16, Offset, Offset16, Stream};
use crate::{NormalizedCoordinate, Tag};

/// A [variation axis](https://docs.microsoft.com/en-us/typography/opentype/spec/fvar#variationaxisrecord).
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct VariationAxis {
    pub tag: Tag,
    pub min_value: f32,
    pub def_value: f32,
    pub max_value: f32,
    /// An axis name in the `name` table.
    pub name_id: u16,
    pub hidden: bool,
}

impl FromData for VariationAxis {
    const SIZE: usize = 20;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let tag = s.read::<Tag>()?;
        let min_value = s.read::<Fixed>()?;
        let def_value = s.read::<Fixed>()?;
        let max_value = s.read::<Fixed>()?;
        let flags = s.read::<u16>()?;
        let name_id = s.read::<u16>()?;

        Some(VariationAxis {
            tag,
            min_value: def_value.0.min(min_value.0),
            def_value: def_value.0,
            max_value: def_value.0.max(max_value.0),
            name_id,
            hidden: (flags >> 3) & 1 == 1,
        })
    }
}

impl VariationAxis {
    /// Returns a normalized variation coordinate for this axis.
    pub(crate) fn normalized_value(&self, mut v: f32) -> NormalizedCoordinate {
        // Based on
        // https://docs.microsoft.com/en-us/typography/opentype/spec/avar#overview

        v = f32_bound(self.min_value, v, self.max_value);
        if v == self.def_value {
            v = 0.0;
        } else if v < self.def_value {
            v = (v - self.def_value) / (self.def_value - self.min_value);
        } else {
            v = (v - self.def_value) / (self.max_value - self.def_value);
        }

        NormalizedCoordinate::from(v)
    }
}

/// A [Font Variations Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/fvar).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of variation axes.
    pub axes: LazyArray16<'a, VariationAxis>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let axes_array_offset = s.read::<Offset16>()?;
        s.skip::<u16>(); // reserved
        let axis_count = s.read::<u16>()?;

        // 'If axisCount is zero, then the font is not functional as a variable font,
        // and must be treated as a non-variable font;
        // any variation-specific tables or data is ignored.'
        let axis_count = NonZeroU16::new(axis_count)?;

        let mut s = Stream::new_at(data, axes_array_offset.to_usize())?;
        let axes = s.read_array16::<VariationAxis>(axis_count.get())?;

        Some(Table { axes })
    }
}
