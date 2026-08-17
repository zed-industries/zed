//! A [Horizontal/Vertical Metrics Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) implementation.

use core::num::NonZeroU16;

use crate::parser::{FromData, LazyArray16, Stream};
use crate::GlyphId;

/// Horizontal/Vertical Metrics.
#[derive(Clone, Copy, Debug)]
pub struct Metrics {
    /// Width/Height advance for `hmtx`/`vmtx`.
    pub advance: u16,
    /// Left/Top side bearing for `hmtx`/`vmtx`.
    pub side_bearing: i16,
}

impl FromData for Metrics {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Metrics {
            advance: s.read::<u16>()?,
            side_bearing: s.read::<i16>()?,
        })
    }
}

/// A [Horizontal/Vertical Metrics Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx).
///
/// `hmtx` and `vmtx` tables has the same structure, so we're reusing the same struct for both.
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of metrics indexed by glyph ID.
    pub metrics: LazyArray16<'a, Metrics>,
    /// Side bearings for glyph IDs greater than or equal to the number of `metrics` values.
    pub bearings: LazyArray16<'a, i16>,
    /// Sum of long metrics + bearings.
    pub number_of_metrics: u16,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// - `number_of_metrics` is from the `hhea`/`vhea` table.
    /// - `number_of_glyphs` is from the `maxp` table.
    pub fn parse(
        mut number_of_metrics: u16,
        number_of_glyphs: NonZeroU16,
        data: &'a [u8],
    ) -> Option<Self> {
        if number_of_metrics == 0 {
            return None;
        }

        let mut s = Stream::new(data);
        let metrics = s.read_array16::<Metrics>(number_of_metrics)?;

        // 'If the number_of_metrics is less than the total number of glyphs,
        // then that array is followed by an array for the left side bearing values
        // of the remaining glyphs.'
        let bearings_count = number_of_glyphs.get().checked_sub(number_of_metrics);
        let bearings = if let Some(count) = bearings_count {
            number_of_metrics += count;
            // Some malformed fonts can skip "left side bearing values"
            // even when they are expected.
            // Therefore if we weren't able to parser them, simply fallback to an empty array.
            // No need to mark the whole table as malformed.
            s.read_array16::<i16>(count).unwrap_or_default()
        } else {
            LazyArray16::default()
        };

        Some(Table {
            metrics,
            bearings,
            number_of_metrics,
        })
    }

    /// Returns advance for a glyph.
    #[inline]
    pub fn advance(&self, glyph_id: GlyphId) -> Option<u16> {
        if glyph_id.0 >= self.number_of_metrics {
            return None;
        }

        if let Some(metrics) = self.metrics.get(glyph_id.0) {
            Some(metrics.advance)
        } else {
            // 'As an optimization, the number of records can be less than the number of glyphs,
            // in which case the advance value of the last record applies
            // to all remaining glyph IDs.'
            self.metrics.last().map(|m| m.advance)
        }
    }

    /// Returns side bearing for a glyph.
    #[inline]
    pub fn side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        if let Some(metrics) = self.metrics.get(glyph_id.0) {
            Some(metrics.side_bearing)
        } else {
            // 'If the number_of_metrics is less than the total number of glyphs,
            // then that array is followed by an array for the side bearing values
            // of the remaining glyphs.'
            self.bearings
                .get(glyph_id.0.checked_sub(self.metrics.len())?)
        }
    }
}
