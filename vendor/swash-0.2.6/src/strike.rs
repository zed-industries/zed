//! Alpha and color bitmaps.

use super::internal::*;
use super::{FontRef, GlyphId};

#[cfg(feature = "scale")]
use alloc::vec::Vec;
#[cfg(all(feature = "libm", feature = "scale"))]
use core_maths::CoreFloat;

/// Proxy for rematerializing strike collections.
#[derive(Copy, Clone)]
pub struct BitmapStrikesProxy {
    bitmaps: (u32, u32),
    color_bitmaps: (u32, u32),
    upem: u16,
    is_apple: bool,
}

impl BitmapStrikesProxy {
    /// Creates strike collection proxy from the specified font.
    pub fn from_font<'a>(font: &FontRef<'a>) -> Self {
        let upem = font.head().map(|h| h.units_per_em()).unwrap_or(1);
        let mut bitmaps = (0, 0);
        let eblc = font.table_offset(raw_tag(b"EBLC"));
        if eblc != 0 {
            let ebdt = font.table_offset(raw_tag(b"EBDT"));
            if ebdt != 0 {
                bitmaps = (eblc, ebdt);
            }
        }
        let mut color_bitmaps = (0, 0);
        let sbix = font.table_offset(raw_tag(b"sbix"));
        let mut is_apple = false;
        if sbix != 0 {
            color_bitmaps = (sbix, sbix);
            use super::string::StringId::Family;
            if let Some(name) = font.localized_strings().find_by_id(Family, None) {
                is_apple = name.chars().eq("Apple Color Emoji".chars());
            }
        } else {
            let cblc = font.table_offset(raw_tag(b"CBLC"));
            if cblc != 0 {
                let cbdt = font.table_offset(raw_tag(b"CBDT"));
                if cbdt != 0 {
                    color_bitmaps = (cblc, cbdt);
                }
            }
        }
        Self {
            bitmaps,
            color_bitmaps,
            upem,
            is_apple,
        }
    }

    /// Returns true if the font has alpha bitmap strikes.
    pub fn has_alpha(&self) -> bool {
        self.bitmaps.0 != 0
    }

    /// Returns true if the font has color bitmap strikes.
    pub fn has_color(&self) -> bool {
        self.color_bitmaps.0 != 0
    }

    /// Materializes an alpha strike iterator for the specified font. This
    /// proxy must have been created from the same font.
    pub fn materialize_alpha<'a>(&self, font: &FontRef<'a>) -> BitmapStrikes<'a> {
        self.materialize_impl(font.data, self.bitmaps.0, self.bitmaps.1, self.upem, false)
    }

    /// Materializes a color strike iterator for the specified font. This
    /// proxy must have been created from the same font.
    pub fn materialize_color<'a>(&self, font: &FontRef<'a>) -> BitmapStrikes<'a> {
        self.materialize_impl(
            font.data,
            self.color_bitmaps.0,
            self.color_bitmaps.1,
            self.upem,
            self.is_apple,
        )
    }

    fn materialize_impl<'a>(
        &self,
        data: &'a [u8],
        loc: u32,
        dat: u32,
        upem: u16,
        is_apple: bool,
    ) -> BitmapStrikes<'a> {
        if loc == 0 {
            BitmapStrikes::new(&[], &[], upem, false, false)
        } else if loc == dat {
            let loc = data.get(loc as usize..).unwrap_or(&[]);
            BitmapStrikes::new(loc, loc, upem, true, is_apple)
        } else {
            let loc = data.get(loc as usize..).unwrap_or(&[]);
            let dat = data.get(dat as usize..).unwrap_or(&[]);
            BitmapStrikes::new(loc, dat, upem, false, false)
        }
    }
}

/// Iterator over a collection of bitmap strikes.
#[derive(Copy, Clone)]
pub struct BitmapStrikes<'a> {
    data: Bytes<'a>,
    bitmap_data: &'a [u8],
    is_sbix: bool,
    is_apple: bool,
    upem: u16,
    len: usize,
    pos: usize,
}

impl<'a> BitmapStrikes<'a> {
    fn new(
        data: &'a [u8],
        bitmap_data: &'a [u8],
        upem: u16,
        is_sbix: bool,
        is_apple: bool,
    ) -> Self {
        let data = Bytes::new(data);
        Self {
            data,
            bitmap_data,
            upem,
            is_sbix,
            is_apple,
            len: data.read_or_default::<u32>(4) as usize,
            pos: 0,
        }
    }

    /// Returns the bitmap strike at the specified index.
    fn get(&self, index: usize) -> Option<BitmapStrike<'a>> {
        if index >= self.len {
            return None;
        }
        let offset = if self.is_sbix {
            self.data.read::<u32>(8 + index * 4)? as usize
        } else {
            8 + index * 48
        };
        Some(BitmapStrike {
            data: self.data,
            bitmap_data: self.bitmap_data,
            upem: self.upem,
            is_sbix: self.is_sbix,
            is_apple: self.is_apple,
            offset,
        })
    }

    /// Searches for a strike that matches the specified size and glyph
    /// identifier. Returns the strike of the nearest suitable size, preferring
    /// larger strikes if no exact match is available.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_nearest_ppem(&self, ppem: u16, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        let mut best = None;
        let mut best_size = 0;
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            best = Some(strike);
            let strike_ppem = strike.ppem();
            if strike_ppem > best_size {
                best = Some(strike);
                best_size = strike_ppem;
            }
            if strike_ppem >= ppem {
                return Some(strike);
            }
        }
        best
    }

    /// Searches for a strike that exactly matches the specified size and glyph
    /// identifier.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_exact_ppem(&self, ppem: u16, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            if strike.ppem() == ppem {
                return Some(strike);
            }
        }
        None
    }

    /// Searches for a strike with the largest size that contains the specified
    /// glyph.
    ///
    /// ## Iteration behavior
    /// This function searches the entire strike collection without regard
    /// for the current state of the iterator.
    pub fn find_by_largest_ppem(&self, glyph_id: GlyphId) -> Option<BitmapStrike<'a>> {
        let mut largest = None;
        let mut largest_ppem = 0;
        for i in 0..self.len {
            let strike = match self.get(i) {
                Some(strike) => strike,
                _ => continue,
            };
            if !strike.contains(glyph_id) {
                continue;
            }
            let strike_ppem = strike.ppem();
            if largest.is_none() || strike_ppem > largest_ppem {
                largest = Some(strike);
                largest_ppem = strike_ppem;
            }
        }
        largest
    }
}

impl_iter!(BitmapStrikes, BitmapStrike);

/// Collection of bitmaps of a specific size and format.
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BitmapStrike<'a> {
    data: Bytes<'a>,
    bitmap_data: &'a [u8],
    offset: usize,
    upem: u16,
    is_sbix: bool,
    is_apple: bool,
}

impl<'a> BitmapStrike<'a> {
    /// Returns the device pixel density for which the strike was designed.
    pub fn ppi(&self) -> u16 {
        if self.is_sbix {
            self.data.read_or_default::<u16>(self.offset + 2)
        } else {
            72
        }
    }

    /// Returns the bit depth of the strike.
    pub fn bit_depth(&self) -> u8 {
        if self.is_sbix {
            return 32;
        }
        self.data.read_or_default::<u8>(self.offset + 46)
    }

    /// Returns the size of the strike in pixels per em.
    pub fn ppem(&self) -> u16 {
        if self.is_sbix {
            self.data.read_or_default::<u16>(self.offset)
        } else {
            self.data.read_or_default::<u8>(self.offset + 45) as u16
        }
    }

    /// Returns true if the specified glyph is covered by the strike.
    pub fn contains(&self, glyph_id: GlyphId) -> bool {
        get_coverage(&self.data, self.offset, self.is_sbix, glyph_id).unwrap_or(false)
    }

    /// Returns the bitmap for the specified glyph, if available.
    #[cfg(feature = "scale")]
    pub(crate) fn get(&self, glyph_id: GlyphId) -> Option<Bitmap<'a>> {
        let loc = get_location(&self.data, self.offset, self.is_sbix, glyph_id)?;
        loc.get(self.bitmap_data, self.upem, self.is_apple)
    }
}

/// Format of a bitmap.
#[cfg(feature = "scale")]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BitmapFormat {
    /// Alpha mask with the specified number of bits.
    Alpha(u8),
    /// Packed alpha mask with the specified number of bits.
    Packed(u8),
    /// 32-bit RGBA color.
    Color,
    /// PNG encoded.
    Png,
}

#[cfg(feature = "scale")]
impl BitmapFormat {
    /// Returns the number of channels.
    pub fn channels(&self) -> u32 {
        match self {
            Self::Alpha(..) | Self::Packed(..) => 1,
            _ => 4,
        }
    }

    /// Returns the buffer size for a bitmap of the specified dimensions in this
    /// format.
    pub fn buffer_size(&self, width: u32, height: u32) -> usize {
        (width * height * self.channels()) as usize
    }
}

/// Bitmap glyph.
#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
pub struct Bitmap<'a> {
    pub format: BitmapFormat,
    pub ppem: u16,
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
    pub data: &'a [u8],
}

#[cfg(feature = "scale")]
impl<'a> Bitmap<'a> {
    fn new(data: &BitmapData<'a>, upem: u16, is_apple: bool) -> Option<Self> {
        let format = if data.is_packed {
            BitmapFormat::Packed(data.bit_depth)
        } else if data.is_png {
            BitmapFormat::Png
        } else if data.bit_depth == 32 {
            BitmapFormat::Color
        } else {
            BitmapFormat::Alpha(data.bit_depth)
        };
        let (width, height, left, top) = if data.is_png {
            let png = Bytes::new(data.data);
            let width = png.read::<u32>(16)?;
            let height = png.read::<u32>(20)?;
            let (left, mut top) = (data.metrics.x as i32, data.metrics.y as i32);
            if data.is_sbix {
                if top == 0 && is_apple {
                    let s = data.ppem as f32 / upem as f32;
                    top = (-100. * s).round() as i32;
                }
                top += height as i32;
            }
            (width, height, left, top)
        } else {
            (
                data.width as u32,
                data.height as u32,
                data.metrics.x as i32,
                data.metrics.y as i32,
            )
        };
        Some(Bitmap {
            format,
            ppem: data.ppem,
            width,
            height,
            left,
            top,
            data: data.data,
        })
    }

    /// Returns the size of an image buffer necessary for storing the decoded bitmap.
    pub fn decoded_size(&self) -> usize {
        self.format.buffer_size(self.width, self.height)
    }

    /// Returns the width, height and buffer size for a scaled version of the bitmap.
    pub fn scaled_size(&self, size: f32) -> (u32, u32, usize) {
        let mut w = self.width;
        let mut h = self.height;
        if size != 0. {
            let scale = size / self.ppem as f32;
            w = (w as f32 * scale) as u32;
            h = (h as f32 * scale) as u32;
        }
        (w, h, self.format.buffer_size(w, h))
    }

    /// Decodes the bitmap into the specified target buffer.
    pub fn decode(&self, scratch: Option<&mut Vec<u8>>, target: &mut [u8]) -> bool {
        let mut tmp = Vec::new();
        let scratch = if let Some(scratch) = scratch {
            scratch
        } else {
            &mut tmp
        };
        let size = self.decoded_size();
        if target.len() < size {
            return false;
        }
        let w = self.width as usize;
        let h = self.height as usize;
        let src = self.data;
        let dst = &mut *target;
        match self.format {
            BitmapFormat::Packed(bits) => match bits {
                1 => {
                    for x in 0..(w * h) {
                        dst[x] = (src[x >> 3] >> (!x & 7) & 1) * 255;
                    }
                }
                2 => {
                    for x in 0..(w * h) {
                        dst[x] = (src[x >> 2] >> (!(x * 2) & 2) & 3) * 85;
                    }
                }
                4 => {
                    for x in 0..(w * h) {
                        dst[x] = (src[x >> 1] >> (!(x * 4) & 4) & 15) * 17;
                    }
                }
                8 | 32 => {
                    dst.copy_from_slice(src);
                }
                _ => return false,
            },
            BitmapFormat::Alpha(bits) => match bits {
                1 => {
                    let mut dst_idx = 0;
                    for row in src.chunks(((w * bits as usize) + 7) / 8) {
                        for x in 0..w {
                            dst[dst_idx] = (row[x >> 3] >> (!x & 7) & 1) * 255;
                            dst_idx += 1;
                        }
                    }
                }
                2 => {
                    let mut dst_idx = 0;
                    for row in src.chunks(((w * bits as usize) + 7) / 8) {
                        for x in 0..w {
                            dst[dst_idx] = (row[x >> 2] >> (!(x * 2) & 2) & 3) * 85;
                            dst_idx += 1;
                        }
                    }
                }
                4 => {
                    let mut dst_idx = 0;
                    for row in src.chunks(((w * bits as usize) + 7) / 8) {
                        for x in 0..w {
                            dst[dst_idx] = (row[x >> 1] >> (!(x * 4) & 4) & 15) * 17;
                            dst_idx += 1;
                        }
                    }
                }
                8 | 32 => {
                    dst.copy_from_slice(src);
                }
                _ => return false,
            },

            BitmapFormat::Color => {
                dst.copy_from_slice(src);
            }
            BitmapFormat::Png => {
                use super::scale::decode_png;
                scratch.clear();
                if decode_png(src, scratch, target).is_none() {
                    return false;
                }
            }
        }
        true
    }
}

/// The location of a bitmap in the bitmap data table.
#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
struct Location {
    format: u8,
    flags: u8,
    offset: u32,
    size: u32,
    ppem: u16,
    bit_depth: u8,
    width: u8,
    height: u8,
    metrics: Metrics,
    vertical_metrics: Metrics,
}

#[cfg(feature = "scale")]
impl Location {
    /// Gets a bitmap from this location in the specified data source.
    pub fn get<'a>(&self, data: &'a [u8], upem: u16, is_apple: bool) -> Option<Bitmap<'a>> {
        Bitmap::new(&get_data(data, self)?, upem, is_apple)
    }
}

fn get_coverage(table: &[u8], strike_base: usize, is_sbix: bool, glyph_id: u16) -> Option<bool> {
    if is_sbix {
        return Some(sbix_range(table, strike_base, glyph_id, 0).is_some());
    }
    let b = Bytes::with_offset(table, strike_base)?;
    if glyph_id < b.read(40)? || glyph_id > b.read(42)? {
        return None;
    }
    let count = b.read::<u32>(8)? as usize;
    let array_offset = b.read::<u32>(0)? as usize;
    let b = Bytes::with_offset(table, array_offset)?;
    for i in 0..count {
        let offset = i * 8;
        let first = b.read::<u16>(offset)?;
        if glyph_id < first {
            return None;
        }
        if glyph_id > b.read::<u16>(offset + 2)? {
            continue;
        }
        return Some(true);
    }
    None
}

#[cfg(feature = "scale")]
fn get_location(
    table: &[u8],
    strike_base: usize,
    is_sbix: bool,
    glyph_id: u16,
) -> Option<Location> {
    let d = Bytes::new(table);
    if is_sbix {
        let (start, end) = sbix_range(table, strike_base, glyph_id, 0)?;
        let len = (end - start) as usize;
        let start = start as usize;
        let x = d.read::<i16>(start)?;
        let y = d.read::<i16>(start + 2)?;
        let ppem = d.read::<u16>(strike_base)?;
        return Some(Location {
            ppem,
            bit_depth: 32,
            width: 0,
            height: 0,
            metrics: Metrics {
                x: x as i8,
                y: y as i8,
                advance: 0,
            },
            vertical_metrics: Metrics::default(),
            format: 0xFF,
            flags: 1,
            offset: start as u32 + 8,
            size: len as u32 - 8,
        });
    }
    if glyph_id < d.read(strike_base + 40)? || glyph_id > d.read(strike_base + 42)? {
        return None;
    }
    let count = d.read::<u32>(strike_base + 8)? as usize;
    let ppem = d.read::<u8>(strike_base + 45)? as u16;
    let bit_depth = d.read::<u8>(strike_base + 46)?;
    let flags = d.read::<u8>(strike_base + 47)?;
    let array_offset = d.read::<u32>(strike_base)? as usize;
    for i in 0..count {
        let offset = array_offset + i * 8;
        let first = d.read::<u16>(offset)?;
        if glyph_id < first {
            return None;
        }
        if glyph_id > d.read::<u16>(offset + 2)? {
            continue;
        }
        let offset = array_offset + d.read::<u32>(offset + 4)? as usize;
        let index_format = d.read::<u16>(offset)?;
        let image_format = d.read::<u16>(offset + 2)? as u8;
        let image_offset = d.read::<u32>(offset + 4)?;
        let base = offset + 8;
        let mut loc = Location {
            ppem,
            bit_depth,
            width: 0,
            height: 0,
            metrics: Metrics::default(),
            vertical_metrics: Metrics::default(),
            format: image_format,
            flags,
            offset: 0,
            size: 0,
        };
        match index_format {
            1 => {
                loc.offset =
                    image_offset + d.read::<u32>(base + (glyph_id - first) as usize * 4)?;
                return Some(loc);
            }
            2 => {
                loc.size = d.read::<u32>(base)?;
                loc.offset = image_offset + loc.size * (glyph_id - first) as u32;
                let (w, h) = get_metrics(
                    &d,
                    base + 4,
                    flags,
                    true,
                    &mut loc.metrics,
                    &mut loc.vertical_metrics,
                )?;
                loc.width = w;
                loc.height = h;
                return Some(loc);
            }
            3 => {
                loc.offset =
                    image_offset + d.read::<u16>(base + (glyph_id - first) as usize * 2)? as u32;
                return Some(loc);
            }
            4 => {
                let mut l = 0;
                let mut h = d.read::<u32>(base)? as usize;
                while l < h {
                    use core::cmp::Ordering::*;
                    let i = (l + h) / 2;
                    let rec = base + i * 4;
                    let id = d.read::<u16>(rec)?;
                    match glyph_id.cmp(&id) {
                        Less => h = i,
                        Greater => l = i + i,
                        Equal => {
                            let offset1 = d.read::<u16>(rec + 2)? as u32;
                            let offset2 = d.read::<u16>(rec + 6)? as u32;
                            if offset2 <= offset1 {
                                return None;
                            }
                            loc.offset = image_offset + offset1;
                            loc.size = offset2 - offset1;
                            return Some(loc);
                        }
                    }
                }
            }
            _ => {
                return None;
            }
        }
    }
    None
}

#[cfg(feature = "scale")]
fn get_data<'a>(table: &'a [u8], loc: &Location) -> Option<BitmapData<'a>> {
    let depth = loc.bit_depth as usize;
    let mut bitmap = BitmapData {
        data: &[],
        ppem: loc.ppem,
        bit_depth: loc.bit_depth,
        width: loc.width,
        height: loc.height,
        metrics: loc.metrics,
        vertical_metrics: loc.vertical_metrics,
        is_packed: false,
        is_png: false,
        is_sbix: false,
    };
    let d = &Bytes::new(table);
    let offset = loc.offset as usize;
    let flags = loc.flags;
    let size = loc.size as usize;
    match loc.format {
        0xFF => {
            bitmap.data = d.read_bytes(offset, size)?;
            bitmap.is_png = true;
            bitmap.is_sbix = true;
            Some(bitmap)
        }
        1 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let w = (bitmap.width as usize * depth + 7) / 8;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 5, w * h)?;
            Some(bitmap)
        }
        2 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let w = bitmap.width as usize * depth;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 5, (w * h + 7) / 8)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        5 => {
            bitmap.data = d.read_bytes(offset, size)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        6 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let w = (bitmap.width as usize * depth + 7) / 8;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 8, w * h)?;
            Some(bitmap)
        }
        7 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let w = bitmap.width as usize * depth;
            let h = bitmap.height as usize;
            bitmap.data = d.read_bytes(offset + 8, (w * h + 7) / 8)?;
            bitmap.is_packed = true;
            Some(bitmap)
        }
        17 => {
            bitmap.read_metrics(d, offset, flags, false)?;
            let size = d.read::<u32>(offset + 5)? as usize;
            bitmap.data = d.read_bytes(offset + 9, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        18 => {
            bitmap.read_metrics(d, offset, flags, true)?;
            let size = d.read::<u32>(offset + 8)? as usize;
            bitmap.data = d.read_bytes(offset + 12, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        19 => {
            let size = d.read::<u32>(offset)? as usize;
            bitmap.data = d.read_bytes(offset + 4, size)?;
            bitmap.is_png = true;
            Some(bitmap)
        }
        _ => None,
    }
}

fn sbix_range(table: &[u8], strike_base: usize, glyph_id: u16, recurse: i32) -> Option<(u32, u32)> {
    const DUPE: RawTag = raw_tag(b"dupe");
    const PNG: RawTag = raw_tag(b"png ");
    if recurse > 1 {
        return None;
    }
    let b = Bytes::new(table);
    let id = glyph_id as usize;
    let base = strike_base + 4;
    let mut start = b.read::<u32>(base + id * 4)?;
    let mut end = b.read::<u32>(base + (id + 1) * 4)?;
    if end <= start {
        return None;
    }
    start += strike_base as u32;
    end += strike_base as u32;
    let tag = b.read::<u32>(start as usize + 4)?;
    if tag == DUPE {
        let dupe = b.read::<u16>(start as usize + 8)?;
        sbix_range(table, strike_base, dupe, recurse + 1)
    } else if tag == PNG {
        Some((start, end))
    } else {
        None
    }
}

#[cfg(feature = "scale")]
#[derive(Copy, Clone)]
struct BitmapData<'a> {
    pub data: &'a [u8],
    pub ppem: u16,
    pub bit_depth: u8,
    pub width: u8,
    pub height: u8,
    pub metrics: Metrics,
    pub vertical_metrics: Metrics,
    pub is_packed: bool,
    pub is_png: bool,
    pub is_sbix: bool,
}

#[cfg(feature = "scale")]
impl<'a> BitmapData<'a> {
    fn read_metrics(&mut self, d: &Bytes, offset: usize, flags: u8, big: bool) -> Option<()> {
        let (w, h) = get_metrics(
            d,
            offset,
            flags,
            big,
            &mut self.metrics,
            &mut self.vertical_metrics,
        )?;
        self.width = w;
        self.height = h;
        Some(())
    }
}

#[cfg(feature = "scale")]
#[derive(Copy, Clone, Default)]
struct Metrics {
    pub x: i8,
    pub y: i8,
    pub advance: u8,
}

#[cfg(feature = "scale")]
fn get_metrics(
    d: &Bytes,
    offset: usize,
    flags: u8,
    big: bool,
    h: &mut Metrics,
    v: &mut Metrics,
) -> Option<(u8, u8)> {
    let width = d.read::<u8>(offset + 1)?;
    let height = d.read::<u8>(offset)?;
    if big {
        h.x = d.read::<i8>(offset + 2)?;
        h.y = d.read::<i8>(offset + 3)?;
        h.advance = d.read::<u8>(offset + 4)?;
        v.x = d.read::<i8>(offset + 5)?;
        v.y = d.read::<i8>(offset + 6)?;
        v.advance = d.read::<u8>(offset + 7)?;
    } else {
        let m = if flags & 2 != 0 { v } else { h };
        m.x = d.read::<i8>(offset + 2)?;
        m.y = d.read::<i8>(offset + 3)?;
        m.advance = d.read::<u8>(offset + 4)?;
    }
    Some((width, height))
}
