use super::super::{
    palette::{ColorPalette, ColorPalettes},
    FontRef, GlyphId,
};
use super::internal::{raw_tag, Bytes, RawFont, RawTag};

const COLR: RawTag = raw_tag(b"COLR");
const CPAL: RawTag = raw_tag(b"CPAL");

#[derive(Copy, Clone, Default)]
pub struct ColorProxy {
    pub colr: u32,
    pub cpal: u32,
}

impl ColorProxy {
    pub fn from_font(font: &FontRef) -> Self {
        Self {
            colr: font.table_offset(COLR),
            cpal: font.table_offset(CPAL),
        }
    }

    pub fn layers<'a>(&self, data: &'a [u8], glyph_id: GlyphId) -> Option<Layers<'a>> {
        let b = Bytes::with_offset(data, self.colr as usize)?;
        let count = b.read::<u16>(2)? as usize;
        let base_offset = b.read::<u32>(4)? as usize;
        let mut l = 0;
        let mut h = count;
        while l < h {
            use core::cmp::Ordering::*;
            let i = l + (h - l) / 2;
            let rec = base_offset + i * 6;
            let id = b.read::<u16>(rec)?;
            match glyph_id.cmp(&id) {
                Less => h = i,
                Greater => l = i + 1,
                Equal => {
                    let first = b.read::<u16>(rec + 2)? as usize;
                    let offset = b.read::<u32>(8)? as usize + first * 4;
                    let len = b.read::<u16>(rec + 4)?;
                    return Some(Layers {
                        data: b,
                        offset,
                        len,
                    });
                }
            }
        }
        None
    }

    // Unused when render feature is disabled.
    #[allow(dead_code)]
    pub fn palette<'a>(&self, font: &FontRef<'a>, index: u16) -> Option<ColorPalette<'a>> {
        if self.cpal != 0 {
            ColorPalettes::from_font_and_offset(font, self.cpal).nth(index as usize)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Layers<'a> {
    data: Bytes<'a>,
    offset: usize,
    len: u16,
}

impl<'a> Layers<'a> {
    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn get(&self, index: u16) -> Option<Layer> {
        let b = &self.data;
        let base = self.offset + index as usize * 4;
        let glyph_id = b.read::<u16>(base)?;
        let color_index = b.read::<u16>(base + 2)?;
        Some(Layer {
            glyph_id,
            color_index: if color_index != 0xFFFF {
                Some(color_index)
            } else {
                None
            },
        })
    }
}

#[derive(Copy, Clone)]
pub struct Layer {
    pub glyph_id: GlyphId,
    pub color_index: Option<u16>,
}
