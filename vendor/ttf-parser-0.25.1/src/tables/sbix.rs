//! A [Standard Bitmap Graphics Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/sbix) implementation.

use core::convert::TryFrom;
use core::num::NonZeroU16;

use crate::parser::{FromData, LazyArray16, LazyArray32, Offset, Offset32, Stream};
use crate::{GlyphId, RasterGlyphImage, RasterImageFormat, Tag};

/// A strike of glyphs.
#[derive(Clone, Copy)]
pub struct Strike<'a> {
    /// The pixels per EM size for which this strike was designed.
    pub pixels_per_em: u16,
    /// The device pixel density (in PPI) for which this strike was designed.
    pub ppi: u16,
    offsets: LazyArray16<'a, Offset32>,
    /// Data from the beginning of the `Strikes` table.
    data: &'a [u8],
}

impl<'a> Strike<'a> {
    fn parse(number_of_glyphs: u16, data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let pixels_per_em = s.read::<u16>()?;
        let ppi = s.read::<u16>()?;
        let offsets = s.read_array16(number_of_glyphs)?;
        Some(Strike {
            pixels_per_em,
            ppi,
            offsets,
            data,
        })
    }

    /// Returns a glyph data.
    pub fn get(&self, glyph_id: GlyphId) -> Option<RasterGlyphImage<'a>> {
        self.get_inner(glyph_id, 0)
    }

    fn get_inner(&self, glyph_id: GlyphId, depth: u8) -> Option<RasterGlyphImage<'a>> {
        // Recursive `dupe`. Bail.
        if depth == 10 {
            return None;
        }

        let start = self.offsets.get(glyph_id.0)?.to_usize();
        let end = self.offsets.get(glyph_id.0.checked_add(1)?)?.to_usize();

        if start == end {
            return None;
        }

        let data_len = end.checked_sub(start)?.checked_sub(8)?; // 8 is a Glyph data header size.

        let mut s = Stream::new_at(self.data, start)?;
        let x = s.read::<i16>()?;
        let y = s.read::<i16>()?;
        let image_type = s.read::<Tag>()?;
        let image_data = s.read_bytes(data_len)?;

        // We do ignore `pdf` and `mask` intentionally, because Apple docs state that:
        // 'Support for the 'pdf ' and 'mask' data types and sbixDrawOutlines flag
        // are planned for future releases of iOS and OS X.'
        let format = match &image_type.to_bytes() {
            b"png " => RasterImageFormat::PNG,
            b"dupe" => {
                // 'The special graphicType of 'dupe' indicates that
                // the data field contains a glyph ID. The bitmap data for
                // the indicated glyph should be used for the current glyph.'
                let glyph_id = GlyphId::parse(image_data)?;
                // TODO: The spec isn't clear about which x/y values should we use.
                //       The current glyph or the referenced one.
                return self.get_inner(glyph_id, depth + 1);
            }
            _ => {
                // TODO: support JPEG and TIFF
                return None;
            }
        };

        let (width, height) = png_size(image_data)?;

        Some(RasterGlyphImage {
            x,
            y,
            width,
            height,
            pixels_per_em: self.pixels_per_em,
            format,
            data: image_data,
        })
    }

    /// Returns the number of glyphs in this strike.
    #[inline]
    pub fn len(&self) -> u16 {
        // The last offset simply indicates the glyph data end. We don't need it.
        self.offsets.len() - 1
    }

    /// Checks if there are any glyphs.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl core::fmt::Debug for Strike<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Strike {{ ... }}")
    }
}

/// A list of [`Strike`]s.
#[derive(Clone, Copy)]
pub struct Strikes<'a> {
    /// `sbix` table data.
    data: &'a [u8],
    // Offsets from the beginning of the `sbix` table.
    offsets: LazyArray32<'a, Offset32>,
    // The total number of glyphs in the face + 1. From the `maxp` table.
    number_of_glyphs: u16,
}

impl<'a> Strikes<'a> {
    /// Returns a strike at the index.
    pub fn get(&self, index: u32) -> Option<Strike<'a>> {
        let offset = self.offsets.get(index)?.to_usize();
        let data = self.data.get(offset..)?;
        Strike::parse(self.number_of_glyphs, data)
    }

    /// Returns the number of strikes.
    #[inline]
    pub fn len(&self) -> u32 {
        self.offsets.len()
    }

    /// Checks if there are any strikes.
    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

impl core::fmt::Debug for Strikes<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Strikes {{ ... }}")
    }
}

impl<'a> IntoIterator for Strikes<'a> {
    type Item = Strike<'a>;
    type IntoIter = StrikesIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        StrikesIter {
            strikes: self,
            index: 0,
        }
    }
}

/// An iterator over [`Strikes`].
#[allow(missing_debug_implementations)]
pub struct StrikesIter<'a> {
    strikes: Strikes<'a>,
    index: u32,
}

impl<'a> Iterator for StrikesIter<'a> {
    type Item = Strike<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.strikes.len() {
            self.index += 1;
            self.strikes.get(self.index - 1)
        } else {
            None
        }
    }
}

/// A [Standard Bitmap Graphics Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/sbix).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// A list of [`Strike`]s.
    pub strikes: Strikes<'a>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    ///
    /// - `number_of_glyphs` is from the `maxp` table.
    pub fn parse(number_of_glyphs: NonZeroU16, data: &'a [u8]) -> Option<Self> {
        let number_of_glyphs = number_of_glyphs.get().checked_add(1)?;

        let mut s = Stream::new(data);

        let version = s.read::<u16>()?;
        if version != 1 {
            return None;
        }

        s.skip::<u16>(); // flags

        let strikes_count = s.read::<u32>()?;
        if strikes_count == 0 {
            return None;
        }

        let offsets = s.read_array32::<Offset32>(strikes_count)?;

        Some(Table {
            strikes: Strikes {
                data,
                offsets,
                number_of_glyphs,
            },
        })
    }

    /// Selects the best matching [`Strike`] based on `pixels_per_em`.
    pub fn best_strike(&self, pixels_per_em: u16) -> Option<Strike<'a>> {
        let mut idx = 0;
        let mut max_ppem = 0;
        for (i, strike) in self.strikes.into_iter().enumerate() {
            if (pixels_per_em <= strike.pixels_per_em && strike.pixels_per_em < max_ppem)
                || (pixels_per_em > max_ppem && strike.pixels_per_em > max_ppem)
            {
                idx = i as u32;
                max_ppem = strike.pixels_per_em;
            }
        }

        self.strikes.get(idx)
    }
}

// The `sbix` table doesn't store the image size, so we have to parse it manually.
// Which is quite simple in case of PNG, but way more complex for JPEG.
// Therefore we are omitting it for now.
fn png_size(data: &[u8]) -> Option<(u16, u16)> {
    // PNG stores its size as u32 BE at a fixed offset.
    let mut s = Stream::new_at(data, 16)?;
    let width = s.read::<u32>()?;
    let height = s.read::<u32>()?;

    // PNG size larger than u16::MAX is an error.
    Some((u16::try_from(width).ok()?, u16::try_from(height).ok()?))
}
