//! A [Maximum Profile Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/maxp) implementation.

use core::num::NonZeroU16;

use crate::parser::Stream;

/// A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).
#[derive(Clone, Copy, Debug)]
pub struct Table {
    /// The total number of glyphs in the face.
    pub number_of_glyphs: NonZeroU16,
}

impl Table {
    /// Parses a table from raw data.
    pub fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let version = s.read::<u32>()?;
        if !(version == 0x00005000 || version == 0x00010000) {
            return None;
        }

        let n = s.read::<u16>()?;
        let number_of_glyphs = NonZeroU16::new(n)?;
        Some(Table { number_of_glyphs })
    }
}
