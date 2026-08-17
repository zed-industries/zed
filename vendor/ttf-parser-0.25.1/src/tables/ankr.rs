//! An [Anchor Point Table](
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ankr.html) implementation.

use core::num::NonZeroU16;

use crate::aat;
use crate::parser::{FromData, LazyArray32, Offset, Offset32, Stream};
use crate::GlyphId;

/// An anchor point.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl FromData for Point {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Point {
            x: s.read::<i16>()?,
            y: s.read::<i16>()?,
        })
    }
}

/// An [Anchor Point Table](
/// https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ankr.html).
#[derive(Clone)]
pub struct Table<'a> {
    lookup: aat::Lookup<'a>,
    // Ideally, Glyphs Data can be represented as an array,
    // but Apple's spec doesn't specify that Glyphs Data members have padding or not.
    // Meaning we cannot simply iterate over them.
    glyphs_data: &'a [u8],
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// `number_of_glyphs` is from the `maxp` table.
    pub fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u16>()?;
        if version != 0 {
            return None;
        }

        s.skip::<u16>(); // reserved

        // TODO: we should probably check that offset is larger than the header size (8)
        let lookup_table = s.read_at_offset32(data)?;
        let glyphs_data = s.read_at_offset32(data)?;

        Some(Table {
            lookup: aat::Lookup::parse(number_of_glyphs, lookup_table)?,
            glyphs_data,
        })
    }

    /// Returns a list of anchor points for the specified glyph.
    pub fn points(&self, glyph_id: GlyphId) -> Option<LazyArray32<'a, Point>> {
        let offset = self.lookup.value(glyph_id)?;

        let mut s = Stream::new_at(self.glyphs_data, usize::from(offset))?;
        let number_of_points = s.read::<u32>()?;
        s.read_array32::<Point>(number_of_points)
    }
}

trait StreamExt<'a> {
    fn read_at_offset32(&mut self, data: &'a [u8]) -> Option<&'a [u8]>;
}

impl<'a> StreamExt<'a> for Stream<'a> {
    fn read_at_offset32(&mut self, data: &'a [u8]) -> Option<&'a [u8]> {
        let offset = self.read::<Offset32>()?.to_usize();
        data.get(offset..)
    }
}
