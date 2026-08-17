//! A [Color Bitmap Data Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt) implementation.

use crate::cblc::{self, BitmapDataFormat, Metrics, MetricsFormat};
use crate::parser::{NumFrom, Stream};
use crate::{GlyphId, RasterGlyphImage, RasterImageFormat};

/// A [Color Bitmap Data Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt).
///
/// EBDT and bdat also share the same structure, so this is re-used for them.
#[derive(Clone, Copy)]
pub struct Table<'a> {
    locations: cblc::Table<'a>,
    data: &'a [u8],
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(locations: cblc::Table<'a>, data: &'a [u8]) -> Option<Self> {
        Some(Self { locations, data })
    }

    /// Returns a raster image for the glyph.
    pub fn get(&self, glyph_id: GlyphId, pixels_per_em: u16) -> Option<RasterGlyphImage<'a>> {
        let location = self.locations.get(glyph_id, pixels_per_em)?;
        let mut s = Stream::new_at(self.data, location.offset)?;
        let metrics = match location.format.metrics {
            MetricsFormat::Small => {
                let height = s.read::<u8>()?;
                let width = s.read::<u8>()?;
                let bearing_x = s.read::<i8>()?;
                let bearing_y = s.read::<i8>()?;
                s.skip::<u8>(); // advance
                Metrics {
                    x: bearing_x,
                    y: bearing_y,
                    width,
                    height,
                }
            }
            MetricsFormat::Big => {
                let height = s.read::<u8>()?;
                let width = s.read::<u8>()?;
                let hor_bearing_x = s.read::<i8>()?;
                let hor_bearing_y = s.read::<i8>()?;
                s.skip::<u8>(); // hor_advance
                s.skip::<i8>(); // ver_bearing_x
                s.skip::<i8>(); // ver_bearing_y
                s.skip::<u8>(); // ver_advance
                Metrics {
                    x: hor_bearing_x,
                    y: hor_bearing_y,
                    width,
                    height,
                }
            }
            MetricsFormat::Shared => location.metrics,
        };
        match location.format.data {
            BitmapDataFormat::ByteAligned { bit_depth } => {
                let row_len = (u32::from(metrics.width) * u32::from(bit_depth) + 7) / 8;
                let data_len = row_len * u32::from(metrics.height);
                let data = s.read_bytes(usize::num_from(data_len))?;
                Some(RasterGlyphImage {
                    x: i16::from(metrics.x),
                    // `y` in CBDT is a bottom bound, not top one.
                    y: i16::from(metrics.y) - i16::from(metrics.height),
                    width: u16::from(metrics.width),
                    height: u16::from(metrics.height),
                    pixels_per_em: location.ppem,
                    format: match bit_depth {
                        1 => RasterImageFormat::BitmapMono,
                        2 => RasterImageFormat::BitmapGray2,
                        4 => RasterImageFormat::BitmapGray4,
                        8 => RasterImageFormat::BitmapGray8,
                        32 => RasterImageFormat::BitmapPremulBgra32,
                        _ => return None,
                    },
                    data,
                })
            }
            BitmapDataFormat::BitAligned { bit_depth } => {
                let data_len = {
                    let w = u32::from(metrics.width);
                    let h = u32::from(metrics.height);
                    let d = u32::from(bit_depth);
                    (w * h * d + 7) / 8
                };

                let data = s.read_bytes(usize::num_from(data_len))?;
                Some(RasterGlyphImage {
                    x: i16::from(metrics.x),
                    // `y` in CBDT is a bottom bound, not top one.
                    y: i16::from(metrics.y) - i16::from(metrics.height),
                    width: u16::from(metrics.width),
                    height: u16::from(metrics.height),
                    pixels_per_em: location.ppem,
                    format: match bit_depth {
                        1 => RasterImageFormat::BitmapMonoPacked,
                        2 => RasterImageFormat::BitmapGray2Packed,
                        4 => RasterImageFormat::BitmapGray4Packed,
                        8 => RasterImageFormat::BitmapGray8,
                        32 => RasterImageFormat::BitmapPremulBgra32,
                        _ => return None,
                    },
                    data,
                })
            }
            BitmapDataFormat::PNG => {
                let data_len = s.read::<u32>()?;
                let data = s.read_bytes(usize::num_from(data_len))?;
                Some(RasterGlyphImage {
                    x: i16::from(metrics.x),
                    // `y` in CBDT is a bottom bound, not top one.
                    y: i16::from(metrics.y) - i16::from(metrics.height),
                    width: u16::from(metrics.width),
                    height: u16::from(metrics.height),
                    pixels_per_em: location.ppem,
                    format: RasterImageFormat::PNG,
                    data,
                })
            }
        }
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
