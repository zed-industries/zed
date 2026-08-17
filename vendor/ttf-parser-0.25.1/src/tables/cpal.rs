//! A [Color Palette Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/cpal) implementation.

use core::num::NonZeroU16;

use crate::parser::{FromData, LazyArray16, Offset, Offset32, Stream};
use crate::RgbaColor;

/// A [Color Palette Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/cpal).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    color_indices: LazyArray16<'a, u16>,
    colors: LazyArray16<'a, BgraColor>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u16>()?;
        if version > 1 {
            return None;
        }

        s.skip::<u16>(); // number of palette entries

        let num_palettes = s.read::<u16>()?;
        if num_palettes == 0 {
            return None; // zero palettes is an error
        }

        let num_colors = s.read::<u16>()?;
        let color_records_offset = s.read::<Offset32>()?;
        let color_indices = s.read_array16::<u16>(num_palettes)?;

        let colors = Stream::new_at(data, color_records_offset.to_usize())?
            .read_array16::<BgraColor>(num_colors)?;

        Some(Self {
            color_indices,
            colors,
        })
    }

    /// Returns the number of palettes.
    pub fn palettes(&self) -> NonZeroU16 {
        // Already checked during parsing.
        NonZeroU16::new(self.color_indices.len()).unwrap()
    }

    /// Returns the color at the given index into the given palette.
    pub fn get(&self, palette_index: u16, palette_entry: u16) -> Option<RgbaColor> {
        let index = self
            .color_indices
            .get(palette_index)?
            .checked_add(palette_entry)?;
        self.colors.get(index).map(|c| c.to_rgba())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BgraColor {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

impl BgraColor {
    #[inline]
    fn to_rgba(self) -> RgbaColor {
        RgbaColor::new(self.red, self.green, self.blue, self.alpha)
    }
}

impl FromData for BgraColor {
    const SIZE: usize = 4;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(Self {
            blue: s.read::<u8>()?,
            green: s.read::<u8>()?,
            red: s.read::<u8>()?,
            alpha: s.read::<u8>()?,
        })
    }
}
