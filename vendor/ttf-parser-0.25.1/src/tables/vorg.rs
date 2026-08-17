//! A [Vertical Origin Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/vorg) implementation.

use crate::parser::{FromData, LazyArray16, Stream};
use crate::GlyphId;

/// Vertical origin metrics for the
/// [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).
#[derive(Clone, Copy, Debug)]
pub struct VerticalOriginMetrics {
    /// Glyph ID.
    pub glyph_id: GlyphId,
    /// Y coordinate, in the font's design coordinate system, of the vertical origin.
    pub y: i16,
}

impl FromData for VerticalOriginMetrics {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(VerticalOriginMetrics {
            glyph_id: s.read::<GlyphId>()?,
            y: s.read::<i16>()?,
        })
    }
}

/// A [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// Default origin.
    pub default_y: i16,
    /// A list of metrics for each glyph.
    ///
    /// Ordered by `glyph_id`.
    pub metrics: LazyArray16<'a, VerticalOriginMetrics>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let default_y = s.read::<i16>()?;
        let count = s.read::<u16>()?;
        let metrics = s.read_array16::<VerticalOriginMetrics>(count)?;

        Some(Table { default_y, metrics })
    }

    /// Returns glyph's Y origin.
    pub fn glyph_y_origin(&self, glyph_id: GlyphId) -> i16 {
        self.metrics
            .binary_search_by(|m| m.glyph_id.cmp(&glyph_id))
            .map(|(_, m)| m.y)
            .unwrap_or(self.default_y)
    }
}
