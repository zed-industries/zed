//! A [Vertical Header Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) implementation.

use crate::parser::Stream;

/// A [Vertical Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea).
#[derive(Clone, Copy, Default, Debug)]
pub struct Table {
    /// Face ascender.
    pub ascender: i16,
    /// Face descender.
    pub descender: i16,
    /// Face line gap.
    pub line_gap: i16,
    /// Number of metrics in the `vmtx` table.
    pub number_of_metrics: u16,
}

impl Table {
    /// Parses a table from raw data.
    pub fn parse(data: &[u8]) -> Option<Self> {
        // Do not check the exact length, because some fonts include
        // padding in table's length in table records, which is incorrect.
        if data.len() < 36 {
            return None;
        }

        let mut s = Stream::new(data);
        s.skip::<u32>(); // version
        let ascender = s.read::<i16>()?;
        let descender = s.read::<i16>()?;
        let line_gap = s.read::<i16>()?;
        s.advance(24);
        let number_of_metrics = s.read::<u16>()?;

        Some(Table {
            ascender,
            descender,
            line_gap,
            number_of_metrics,
        })
    }
}
