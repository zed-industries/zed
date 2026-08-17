//! Collections of colors for layered outlines.

use super::internal::*;
use super::{
    string::{LocalizedString, StringId},
    FontRef,
};

const CPAL: RawTag = raw_tag(b"CPAL");

/// Iterator over a collection of color palettes.
#[derive(Copy, Clone)]
pub struct ColorPalettes<'a> {
    font: FontRef<'a>,
    data: Bytes<'a>,
    len: usize,
    pos: usize,
}

impl<'a> ColorPalettes<'a> {
    pub(crate) fn new(font: FontRef<'a>, data: &'a [u8]) -> Self {
        let data = Bytes::new(data);
        let len = data.read_or_default::<u16>(4) as usize;
        Self {
            font,
            data,
            len,
            pos: 0,
        }
    }

    pub(crate) fn from_font(font: &FontRef<'a>) -> Self {
        let data = font.table_data(CPAL).unwrap_or(&[]);
        Self::new(*font, data)
    }

    // Unused when render feature is disabled.
    #[allow(dead_code)]
    pub(crate) fn from_font_and_offset(font: &FontRef<'a>, offset: u32) -> Self {
        let data = font.data.get(offset as usize..).unwrap_or(&[]);
        Self::new(*font, data)
    }

    fn get(&self, index: usize) -> Option<ColorPalette<'a>> {
        if index >= self.len {
            return None;
        }
        let b = &self.data;
        let version = b.read::<u16>(0)?;
        let num_entries = b.read::<u16>(2)?;
        let offset = b.read::<u32>(8)? as usize;
        let first = b.read::<u16>(12 + index * 2)? as usize;
        let offset = offset + first * 4;
        Some(ColorPalette {
            font: self.font,
            data: *b,
            version,
            index,
            num_entries,
            offset,
        })
    }
}

impl_iter!(ColorPalettes, ColorPalette);

/// Collection of colors.
#[derive(Copy, Clone)]
pub struct ColorPalette<'a> {
    font: FontRef<'a>,
    data: Bytes<'a>,
    version: u16,
    index: usize,
    num_entries: u16,
    offset: usize,
}

impl<'a> ColorPalette<'a> {
    /// Returns the index of the palette.
    pub fn index(&self) -> u16 {
        self.index as u16
    }

    /// Returns the name identifier for the palette, if available.
    pub fn name_id(&self) -> Option<StringId> {
        if self.version == 0 {
            return None;
        }
        let d = &self.data;
        let num_palettes = d.read::<u16>(4)? as usize;
        let base = 16 + num_palettes * 2;
        let labels_offset = d.read::<u32>(base)? as usize;
        if labels_offset == 0 {
            return None;
        }
        Some(StringId::Other(
            d.read::<u16>(labels_offset + self.index * 2)?,
        ))
    }

    /// Returns the name for the palette, optionally for a particular
    /// language.
    pub fn name(&self, language: Option<&str>) -> Option<LocalizedString<'a>> {
        self.name_id()
            .and_then(|id| self.font.localized_strings().find_by_id(id, language))
    }

    /// Returns the theme usability of the palette, if available.
    pub fn usability(&self) -> Option<Usability> {
        let flags = self.flags()?;
        Some(match flags & 0b11 {
            0b01 => Usability::Light,
            0b10 => Usability::Dark,
            0b11 => Usability::Both,
            _ => return None,
        })
    }

    /// Returns the number of color entries in the palette.
    pub fn len(&self) -> u16 {
        self.num_entries
    }

    /// Returns whether this palette is empty.
    pub fn is_empty(&self) -> bool {
        self.num_entries == 0
    }

    /// Returns the color for the specified entry in RGBA order.
    pub fn get(&self, index: u16) -> [u8; 4] {
        if index >= self.num_entries {
            return [0; 4];
        }
        let offset = self.offset + index as usize * 4;
        let d = &self.data;
        let b = d.read_or_default::<u8>(offset);
        let g = d.read_or_default::<u8>(offset + 1);
        let r = d.read_or_default::<u8>(offset + 2);
        let a = d.read_or_default::<u8>(offset + 3);
        [r, g, b, a]
    }

    fn flags(&self) -> Option<u32> {
        if self.version == 0 {
            return None;
        }
        let d = &self.data;
        let num_palettes = d.read::<u16>(4)? as usize;
        let base = 12 + num_palettes * 2;
        let types_offset = d.read::<u32>(base)? as usize;
        if types_offset == 0 {
            return None;
        }
        d.read::<u32>(types_offset + self.index * 4)
    }
}

/// Theme of a palette with respect to background color.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Usability {
    /// Usable with light backgrounds.
    Light,
    /// Usable with dark backgrounds.
    Dark,
    /// Usable with both light and dark backgrounds.
    Both,
}
