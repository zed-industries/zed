//! An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca)
//! implementation.

use core::convert::TryFrom;
use core::num::NonZeroU16;
use core::ops::Range;

use crate::parser::{LazyArray16, NumFrom, Stream};
use crate::{GlyphId, IndexToLocationFormat};

/// An [Index to Location Table](https://docs.microsoft.com/en-us/typography/opentype/spec/loca).
#[derive(Clone, Copy, Debug)]
pub enum Table<'a> {
    /// Short offsets.
    Short(LazyArray16<'a, u16>),
    /// Long offsets.
    Long(LazyArray16<'a, u32>),
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// - `number_of_glyphs` is from the `maxp` table.
    /// - `format` is from the `head` table.
    pub fn parse(
        number_of_glyphs: NonZeroU16,
        format: IndexToLocationFormat,
        data: &'a [u8],
    ) -> Option<Self> {
        // The number of ranges is `maxp.numGlyphs + 1`.
        //
        // Check for overflow first.
        let mut total = if number_of_glyphs.get() == u16::MAX {
            number_of_glyphs.get()
        } else {
            number_of_glyphs.get() + 1
        };

        // By the spec, the number of `loca` offsets is `maxp.numGlyphs + 1`.
        // But some malformed fonts can have less glyphs than that.
        // In which case we try to parse only the available offsets
        // and do not return an error, since the expected data length
        // would go beyond table's length.
        //
        // In case when `loca` has more data than needed we simply ignore the rest.
        let actual_total = match format {
            IndexToLocationFormat::Short => data.len() / 2,
            IndexToLocationFormat::Long => data.len() / 4,
        };
        let actual_total = u16::try_from(actual_total).ok()?;
        total = total.min(actual_total);

        let mut s = Stream::new(data);
        match format {
            IndexToLocationFormat::Short => Some(Table::Short(s.read_array16::<u16>(total)?)),
            IndexToLocationFormat::Long => Some(Table::Long(s.read_array16::<u32>(total)?)),
        }
    }

    /// Returns the number of offsets.
    #[inline]
    pub fn len(&self) -> u16 {
        match self {
            Table::Short(ref array) => array.len(),
            Table::Long(ref array) => array.len(),
        }
    }

    /// Checks if there are any offsets.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns glyph's range in the `glyf` table.
    #[inline]
    pub fn glyph_range(&self, glyph_id: GlyphId) -> Option<Range<usize>> {
        let glyph_id = glyph_id.0;
        if glyph_id == u16::MAX {
            return None;
        }

        // Glyph ID must be smaller than total number of values in a `loca` array.
        if glyph_id + 1 >= self.len() {
            return None;
        }

        let range = match self {
            Table::Short(ref array) => {
                // 'The actual local offset divided by 2 is stored.'
                usize::from(array.get(glyph_id)?) * 2..usize::from(array.get(glyph_id + 1)?) * 2
            }
            Table::Long(ref array) => {
                usize::num_from(array.get(glyph_id)?)..usize::num_from(array.get(glyph_id + 1)?)
            }
        };

        if range.start >= range.end {
            // 'The offsets must be in ascending order.'
            // And range cannot be empty.
            None
        } else {
            Some(range)
        }
    }
}
