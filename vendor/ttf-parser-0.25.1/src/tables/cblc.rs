//! A [Color Bitmap Location Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/cblc) implementation.

use crate::parser::{FromData, NumFrom, Offset, Offset16, Offset32, Stream};
use crate::GlyphId;

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct BitmapFormat {
    pub metrics: MetricsFormat,
    pub data: BitmapDataFormat,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum MetricsFormat {
    Small,
    Big,
    Shared,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BitmapDataFormat {
    ByteAligned { bit_depth: u8 },
    BitAligned { bit_depth: u8 },
    PNG,
}

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Metrics {
    pub x: i8,
    pub y: i8,
    pub width: u8,
    pub height: u8,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Location {
    pub format: BitmapFormat,
    pub offset: usize,
    pub metrics: Metrics,
    pub ppem: u16,
}

#[derive(Clone, Copy)]
struct BitmapSizeTable {
    subtable_array_offset: Offset32,
    number_of_subtables: u32,
    ppem: u16,
    bit_depth: u8,
    // Many fields are omitted.
}

fn select_bitmap_size_table(
    glyph_id: GlyphId,
    pixels_per_em: u16,
    mut s: Stream,
) -> Option<BitmapSizeTable> {
    let subtable_count = s.read::<u32>()?;
    let orig_s = s.clone();

    let mut idx = None;
    let mut max_ppem = 0;
    let mut bit_depth_for_max_ppem = 0;
    for i in 0..subtable_count {
        // Check that the current subtable contains a provided glyph id.
        s.advance(40); // Jump to `start_glyph_index`.
        let start_glyph_id = s.read::<GlyphId>()?;
        let end_glyph_id = s.read::<GlyphId>()?;
        let ppem_x = u16::from(s.read::<u8>()?);
        s.advance(1); // ppem_y
        let bit_depth = s.read::<u8>()?;
        s.advance(1); // flags

        if !(start_glyph_id..=end_glyph_id).contains(&glyph_id) {
            continue;
        }

        // Select a best matching subtable based on `pixels_per_em`.
        if (pixels_per_em <= ppem_x && ppem_x < max_ppem)
            || (pixels_per_em > max_ppem && ppem_x > max_ppem)
        {
            idx = Some(usize::num_from(i));
            max_ppem = ppem_x;
            bit_depth_for_max_ppem = bit_depth;
        }
    }

    let mut s = orig_s;
    s.advance(idx? * 48); // 48 is BitmapSize Table size

    let subtable_array_offset = s.read::<Offset32>()?;
    s.skip::<u32>(); // index_tables_size
    let number_of_subtables = s.read::<u32>()?;

    Some(BitmapSizeTable {
        subtable_array_offset,
        number_of_subtables,
        ppem: max_ppem,
        bit_depth: bit_depth_for_max_ppem,
    })
}

#[derive(Clone, Copy)]
struct IndexSubtableInfo {
    start_glyph_id: GlyphId,
    offset: usize, // absolute offset
}

fn select_index_subtable(
    data: &[u8],
    size_table: BitmapSizeTable,
    glyph_id: GlyphId,
) -> Option<IndexSubtableInfo> {
    let mut s = Stream::new_at(data, size_table.subtable_array_offset.to_usize())?;
    for _ in 0..size_table.number_of_subtables {
        let start_glyph_id = s.read::<GlyphId>()?;
        let end_glyph_id = s.read::<GlyphId>()?;
        let offset = s.read::<Offset32>()?;

        if (start_glyph_id..=end_glyph_id).contains(&glyph_id) {
            let offset = size_table.subtable_array_offset.to_usize() + offset.to_usize();
            return Some(IndexSubtableInfo {
                start_glyph_id,
                offset,
            });
        }
    }

    None
}

#[derive(Clone, Copy)]
struct GlyphIdOffsetPair {
    glyph_id: GlyphId,
    offset: Offset16,
}

impl FromData for GlyphIdOffsetPair {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(GlyphIdOffsetPair {
            glyph_id: s.read::<GlyphId>()?,
            offset: s.read::<Offset16>()?,
        })
    }
}

// TODO: rewrite

/// A [Color Bitmap Location Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/cblc).
///
/// EBLC and bloc also share the same structure, so this is re-used for them.
#[derive(Clone, Copy)]
pub struct Table<'a> {
    data: &'a [u8],
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        Some(Self { data })
    }

    pub(crate) fn get(&self, glyph_id: GlyphId, pixels_per_em: u16) -> Option<Location> {
        let mut s = Stream::new(self.data);

        // The CBLC table version is a bit tricky, so we are ignoring it for now.
        // The CBLC table is based on EBLC table, which was based on the `bloc` table.
        // And before the CBLC table specification was finished, some fonts,
        // notably Noto Emoji, have used version 2.0, but the final spec allows only 3.0.
        // So there are perfectly valid fonts in the wild, which have an invalid version.
        s.skip::<u32>(); // version

        let size_table = select_bitmap_size_table(glyph_id, pixels_per_em, s)?;
        let info = select_index_subtable(self.data, size_table, glyph_id)?;

        let mut s = Stream::new_at(self.data, info.offset)?;
        let index_format = s.read::<u16>()?;
        let image_format = s.read::<u16>()?;
        let mut image_offset = s.read::<Offset32>()?.to_usize();

        let bit_depth = size_table.bit_depth;
        let image_format = match image_format {
            1 => BitmapFormat {
                metrics: MetricsFormat::Small,
                data: BitmapDataFormat::ByteAligned { bit_depth },
            },
            2 => BitmapFormat {
                metrics: MetricsFormat::Small,
                data: BitmapDataFormat::BitAligned { bit_depth },
            },
            5 => BitmapFormat {
                metrics: MetricsFormat::Shared,
                data: BitmapDataFormat::BitAligned { bit_depth },
            },
            6 => BitmapFormat {
                metrics: MetricsFormat::Big,
                data: BitmapDataFormat::ByteAligned { bit_depth },
            },
            7 => BitmapFormat {
                metrics: MetricsFormat::Big,
                data: BitmapDataFormat::BitAligned { bit_depth },
            },
            17 => BitmapFormat {
                metrics: MetricsFormat::Small,
                data: BitmapDataFormat::PNG,
            },
            18 => BitmapFormat {
                metrics: MetricsFormat::Big,
                data: BitmapDataFormat::PNG,
            },
            19 => BitmapFormat {
                metrics: MetricsFormat::Shared,
                data: BitmapDataFormat::PNG,
            },
            _ => return None, // Invalid format.
        };

        // TODO: I wasn't able to find fonts with index 4 and 5, so they are untested.

        let glyph_diff = glyph_id.0.checked_sub(info.start_glyph_id.0)?;
        let mut metrics = Metrics::default();
        match index_format {
            1 => {
                s.advance(usize::from(glyph_diff) * Offset32::SIZE);
                let offset = s.read::<Offset32>()?;
                image_offset += offset.to_usize();
            }
            2 => {
                let image_size = s.read::<u32>()?;
                image_offset += usize::from(glyph_diff).checked_mul(usize::num_from(image_size))?;
                metrics.height = s.read::<u8>()?;
                metrics.width = s.read::<u8>()?;
                metrics.x = s.read::<i8>()?;
                metrics.y = s.read::<i8>()?;
            }
            3 => {
                s.advance(usize::from(glyph_diff) * Offset16::SIZE);
                let offset = s.read::<Offset16>()?;
                image_offset += offset.to_usize();
            }
            4 => {
                let num_glyphs = s.read::<u32>()?;
                let num_glyphs = num_glyphs.checked_add(1)?;
                let pairs = s.read_array32::<GlyphIdOffsetPair>(num_glyphs)?;
                let pair = pairs.into_iter().find(|pair| pair.glyph_id == glyph_id)?;
                image_offset += pair.offset.to_usize();
            }
            5 => {
                let image_size = s.read::<u32>()?;
                metrics.height = s.read::<u8>()?;
                metrics.width = s.read::<u8>()?;
                metrics.x = s.read::<i8>()?;
                metrics.y = s.read::<i8>()?;
                s.skip::<u8>(); // hor_advance
                s.skip::<i8>(); // ver_bearing_x
                s.skip::<i8>(); // ver_bearing_y
                s.skip::<u8>(); // ver_advance
                let num_glyphs = s.read::<u32>()?;
                let glyphs = s.read_array32::<GlyphId>(num_glyphs)?;
                let (index, _) = glyphs.binary_search(&glyph_id)?;
                image_offset = image_offset.checked_add(
                    usize::num_from(index).checked_mul(usize::num_from(image_size))?,
                )?;
            }
            _ => return None, // Invalid format.
        }

        Some(Location {
            format: image_format,
            offset: image_offset,
            metrics,
            ppem: size_table.ppem,
        })
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
