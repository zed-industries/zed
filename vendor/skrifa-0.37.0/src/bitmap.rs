//! Bitmap strikes and glyphs.

use super::{instance::Size, metrics::GlyphMetrics, MetadataProvider};
use crate::prelude::LocationRef;
use raw::{
    tables::{bitmap, cbdt, cblc, ebdt, eblc, sbix},
    types::{GlyphId, Tag},
    FontData, FontRef, TableProvider,
};

/// Set of strikes, each containing embedded bitmaps of a single size.
#[derive(Clone)]
pub struct BitmapStrikes<'a>(StrikesKind<'a>);

impl<'a> BitmapStrikes<'a> {
    /// Creates a new `BitmapStrikes` for the given font.
    ///
    /// This will prefer `sbix`, `CBDT`, and `CBLC` formats in that order.
    ///
    /// To select a specific format, use [`with_format`](Self::with_format).
    pub fn new(font: &FontRef<'a>) -> Self {
        for format in [BitmapFormat::Sbix, BitmapFormat::Cbdt, BitmapFormat::Ebdt] {
            if let Some(strikes) = Self::with_format(font, format) {
                return strikes;
            }
        }
        Self(StrikesKind::None)
    }

    /// Creates a new `BitmapStrikes` for the given font and format.
    ///
    /// Returns `None` if the requested format is not available.
    pub fn with_format(font: &FontRef<'a>, format: BitmapFormat) -> Option<Self> {
        let kind = match format {
            BitmapFormat::Sbix => StrikesKind::Sbix(
                font.sbix().ok()?,
                font.glyph_metrics(Size::unscaled(), LocationRef::default()),
            ),
            BitmapFormat::Cbdt => {
                StrikesKind::Cbdt(CbdtTables::new(font.cblc().ok()?, font.cbdt().ok()?))
            }
            BitmapFormat::Ebdt => {
                StrikesKind::Ebdt(EbdtTables::new(font.eblc().ok()?, font.ebdt().ok()?))
            }
        };
        Some(Self(kind))
    }

    /// Returns the format representing the underlying table for this set of
    /// strikes.
    pub fn format(&self) -> Option<BitmapFormat> {
        match &self.0 {
            StrikesKind::None => None,
            StrikesKind::Sbix(..) => Some(BitmapFormat::Sbix),
            StrikesKind::Cbdt(..) => Some(BitmapFormat::Cbdt),
            StrikesKind::Ebdt(..) => Some(BitmapFormat::Ebdt),
        }
    }

    /// Returns the number of available strikes.
    pub fn len(&self) -> usize {
        match &self.0 {
            StrikesKind::None => 0,
            StrikesKind::Sbix(sbix, _) => sbix.strikes().len(),
            StrikesKind::Cbdt(cbdt) => cbdt.location.bitmap_sizes().len(),
            StrikesKind::Ebdt(ebdt) => ebdt.location.bitmap_sizes().len(),
        }
    }

    /// Returns true if there are no available strikes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the strike at the given index.
    pub fn get(&self, index: usize) -> Option<BitmapStrike<'a>> {
        let kind = match &self.0 {
            StrikesKind::None => return None,
            StrikesKind::Sbix(sbix, metrics) => {
                StrikeKind::Sbix(sbix.strikes().get(index).ok()?, metrics.clone())
            }
            StrikesKind::Cbdt(tables) => StrikeKind::Cbdt(
                tables.location.bitmap_sizes().get(index).copied()?,
                tables.clone(),
            ),
            StrikesKind::Ebdt(tables) => StrikeKind::Ebdt(
                tables.location.bitmap_sizes().get(index).copied()?,
                tables.clone(),
            ),
        };
        Some(BitmapStrike(kind))
    }

    /// Returns the best matching glyph for the given size and glyph
    /// identifier.
    ///
    /// In this case, "best" means a glyph of the exact size, nearest larger
    /// size, or nearest smaller size, in that order.
    pub fn glyph_for_size(&self, size: Size, glyph_id: GlyphId) -> Option<BitmapGlyph<'a>> {
        // Return the largest size for an unscaled request
        let size = size.ppem().unwrap_or(f32::MAX);
        self.iter()
            .fold(None, |best: Option<BitmapGlyph<'a>>, entry| {
                let entry_size = entry.ppem();
                if let Some(best) = best {
                    let best_size = best.ppem_y;
                    if (entry_size >= size && entry_size < best_size)
                        || (best_size < size && entry_size > best_size)
                    {
                        entry.get(glyph_id).or(Some(best))
                    } else {
                        Some(best)
                    }
                } else {
                    entry.get(glyph_id)
                }
            })
    }

    /// Returns an iterator over all available strikes.
    pub fn iter(&self) -> impl Iterator<Item = BitmapStrike<'a>> + 'a + Clone {
        let this = self.clone();
        (0..this.len()).filter_map(move |ix| this.get(ix))
    }
}

#[derive(Clone)]
enum StrikesKind<'a> {
    None,
    Sbix(sbix::Sbix<'a>, GlyphMetrics<'a>),
    Cbdt(CbdtTables<'a>),
    Ebdt(EbdtTables<'a>),
}

/// Set of embedded bitmap glyphs of a specific size.
#[derive(Clone)]
pub struct BitmapStrike<'a>(StrikeKind<'a>);

impl<'a> BitmapStrike<'a> {
    /// Returns the pixels-per-em (size) of this strike.
    pub fn ppem(&self) -> f32 {
        match &self.0 {
            StrikeKind::Sbix(sbix, _) => sbix.ppem() as f32,
            // Original implementation also considers `ppem_y` here:
            // https://github.com/google/skia/blob/02cd0561f4f756bf4f7b16641d8fc4c61577c765/src/ports/fontations/src/bitmap.rs#L48
            StrikeKind::Cbdt(size, _) => size.ppem_y() as f32,
            StrikeKind::Ebdt(size, _) => size.ppem_y() as f32,
        }
    }

    /// Returns a bitmap glyph for the given identifier, if available.
    pub fn get(&self, glyph_id: GlyphId) -> Option<BitmapGlyph<'a>> {
        match &self.0 {
            StrikeKind::Sbix(sbix, metrics) => {
                let glyph = sbix.glyph_data(glyph_id).ok()??;
                if glyph.graphic_type() != Tag::new(b"png ") {
                    return None;
                }

                // Note that this calculation does not entirely correspond to the description in
                // the specification, but it's implemented this way in Skia (https://github.com/google/skia/blob/02cd0561f4f756bf4f7b16641d8fc4c61577c765/src/ports/fontations/src/bitmap.rs#L161-L178),
                // the implementation of which has been tested against behavior in CoreText.
                let glyf_bb = metrics.bounds(glyph_id).unwrap_or_default();
                let lsb = metrics.left_side_bearing(glyph_id).unwrap_or_default();
                let ppem = sbix.ppem() as f32;
                let png_data = glyph.data();
                // PNG format:
                // 8 byte header, IHDR chunk (4 byte length, 4 byte chunk type), width, height
                let reader = FontData::new(png_data);
                let width = reader.read_at::<u32>(16).ok()?;
                let height = reader.read_at::<u32>(20).ok()?;
                Some(BitmapGlyph {
                    data: BitmapData::Png(glyph.data()),
                    bearing_x: lsb,
                    bearing_y: glyf_bb.y_min,
                    inner_bearing_x: glyph.origin_offset_x() as f32,
                    inner_bearing_y: glyph.origin_offset_y() as f32,
                    ppem_x: ppem,
                    ppem_y: ppem,
                    width,
                    height,
                    advance: None,
                    placement_origin: Origin::BottomLeft,
                })
            }
            StrikeKind::Cbdt(size, tables) => {
                let location = size
                    .location(tables.location.offset_data(), glyph_id)
                    .ok()?;
                let data = tables.data.data(&location).ok()?;
                BitmapGlyph::from_bdt(size, &data)
            }
            StrikeKind::Ebdt(size, tables) => {
                let location = size
                    .location(tables.location.offset_data(), glyph_id)
                    .ok()?;
                let data = tables.data.data(&location).ok()?;
                BitmapGlyph::from_bdt(size, &data)
            }
        }
    }
}

#[derive(Clone)]
enum StrikeKind<'a> {
    Sbix(sbix::Strike<'a>, GlyphMetrics<'a>),
    Cbdt(bitmap::BitmapSize, CbdtTables<'a>),
    Ebdt(bitmap::BitmapSize, EbdtTables<'a>),
}

#[derive(Clone)]
struct BdtTables<L, D> {
    location: L,
    data: D,
}

impl<L, D> BdtTables<L, D> {
    fn new(location: L, data: D) -> Self {
        Self { location, data }
    }
}

type CbdtTables<'a> = BdtTables<cblc::Cblc<'a>, cbdt::Cbdt<'a>>;
type EbdtTables<'a> = BdtTables<eblc::Eblc<'a>, ebdt::Ebdt<'a>>;

/// An embedded bitmap glyph.
#[derive(Clone)]
pub struct BitmapGlyph<'a> {
    /// The underlying data of the bitmap glyph.
    pub data: BitmapData<'a>,
    /// Outer glyph bearings in the x direction, given in font units.
    pub bearing_x: f32,
    /// Outer glyph bearings in the y direction, given in font units.
    pub bearing_y: f32,
    /// Inner glyph bearings in the x direction, given in pixels. This value should be scaled
    /// by `ppem_*` and be applied as an offset when placing the image within the bounds rectangle.
    pub inner_bearing_x: f32,
    /// Inner glyph bearings in the y direction, given in pixels. This value should be scaled
    /// by `ppem_*` and be applied as an offset when placing the image within the bounds rectangle.
    pub inner_bearing_y: f32,
    /// The assumed pixels-per-em in the x direction.
    pub ppem_x: f32,
    /// The assumed pixels-per-em in the y direction.
    pub ppem_y: f32,
    /// The horizontal advance width of the bitmap glyph in pixels, if given.
    pub advance: Option<f32>,
    /// The number of columns in the bitmap.
    pub width: u32,
    /// The number of rows in the bitmap.
    pub height: u32,
    /// The placement origin of the bitmap.
    pub placement_origin: Origin,
}

impl<'a> BitmapGlyph<'a> {
    fn from_bdt(
        bitmap_size: &bitmap::BitmapSize,
        bitmap_data: &bitmap::BitmapData<'a>,
    ) -> Option<Self> {
        let metrics = BdtMetrics::new(bitmap_data);
        let (ppem_x, ppem_y) = (bitmap_size.ppem_x() as f32, bitmap_size.ppem_y() as f32);
        let bpp = bitmap_size.bit_depth();
        let data = match bpp {
            32 => {
                match &bitmap_data.content {
                    bitmap::BitmapContent::Data(bitmap::BitmapDataFormat::Png, bytes) => {
                        BitmapData::Png(bytes)
                    }
                    // 32-bit formats are always byte aligned
                    bitmap::BitmapContent::Data(bitmap::BitmapDataFormat::ByteAligned, bytes) => {
                        BitmapData::Bgra(bytes)
                    }
                    _ => return None,
                }
            }
            1 | 2 | 4 | 8 => {
                let (data, is_packed) = match &bitmap_data.content {
                    bitmap::BitmapContent::Data(bitmap::BitmapDataFormat::ByteAligned, bytes) => {
                        (bytes, false)
                    }
                    bitmap::BitmapContent::Data(bitmap::BitmapDataFormat::BitAligned, bytes) => {
                        (bytes, true)
                    }
                    _ => return None,
                };
                BitmapData::Mask(MaskData {
                    bpp,
                    is_packed,
                    data,
                })
            }
            // All other bit depth values are invalid
            _ => return None,
        };
        Some(Self {
            data,
            bearing_x: 0.0,
            bearing_y: 0.0,
            inner_bearing_x: metrics.inner_bearing_x,
            inner_bearing_y: metrics.inner_bearing_y,
            ppem_x,
            ppem_y,
            width: metrics.width,
            height: metrics.height,
            advance: Some(metrics.advance),
            placement_origin: Origin::TopLeft,
        })
    }
}

struct BdtMetrics {
    inner_bearing_x: f32,
    inner_bearing_y: f32,
    advance: f32,
    width: u32,
    height: u32,
}

impl BdtMetrics {
    fn new(data: &bitmap::BitmapData) -> Self {
        match data.metrics {
            bitmap::BitmapMetrics::Small(metrics) => Self {
                inner_bearing_x: metrics.bearing_x() as f32,
                inner_bearing_y: metrics.bearing_y() as f32,
                advance: metrics.advance() as f32,
                width: metrics.width() as u32,
                height: metrics.height() as u32,
            },
            bitmap::BitmapMetrics::Big(metrics) => Self {
                inner_bearing_x: metrics.hori_bearing_x() as f32,
                inner_bearing_y: metrics.hori_bearing_y() as f32,
                advance: metrics.hori_advance() as f32,
                width: metrics.width() as u32,
                height: metrics.height() as u32,
            },
        }
    }
}

///The origin point for drawing a bitmap glyph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Origin {
    /// The origin is in the top-left.
    TopLeft,
    /// The origin is in the bottom-left.
    BottomLeft,
}

/// Data content of a bitmap.
#[derive(Clone)]
pub enum BitmapData<'a> {
    /// Uncompressed 32-bit color bitmap data, pre-multiplied in BGRA order
    /// and encoded in the sRGB color space.
    Bgra(&'a [u8]),
    /// Compressed PNG bitmap data.
    Png(&'a [u8]),
    /// Data representing a single channel alpha mask.
    Mask(MaskData<'a>),
}

/// A single channel alpha mask.
#[derive(Clone)]
pub struct MaskData<'a> {
    /// Number of bits-per-pixel. Always 1, 2, 4 or 8.
    pub bpp: u8,
    /// True if each row of the data is bit-aligned. Otherwise, each row
    /// is padded to the next byte.
    pub is_packed: bool,
    /// Raw bitmap data.
    pub data: &'a [u8],
}

/// The format (or table) containing the data backing a set of bitmap strikes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BitmapFormat {
    Sbix,
    Cbdt,
    Ebdt,
}

#[cfg(test)]
mod tests {
    use crate::bitmap::{BitmapData, StrikesKind};
    use crate::prelude::Size;
    use crate::{GlyphId, MetadataProvider};
    use raw::FontRef;

    #[test]
    fn cbdt_metadata() {
        let font = FontRef::new(font_test_data::CBDT).unwrap();
        let strikes = font.bitmap_strikes();

        assert!(matches!(strikes.0, StrikesKind::Cbdt(_)));
        assert!(matches!(strikes.len(), 3));

        // Note that this is only `ppem_y`.
        assert!(matches!(strikes.get(0).unwrap().ppem(), 16.0));
        assert!(matches!(strikes.get(1).unwrap().ppem(), 64.0));
        assert!(matches!(strikes.get(2).unwrap().ppem(), 128.0));
    }

    #[test]
    fn cbdt_glyph_metrics() {
        let font = FontRef::new(font_test_data::CBDT).unwrap();
        let strike_0 = font.bitmap_strikes().get(0).unwrap();

        let zero = strike_0.get(GlyphId::new(0)).unwrap();
        assert_eq!(zero.width, 11);
        assert_eq!(zero.height, 13);
        assert_eq!(zero.bearing_x, 0.0);
        assert_eq!(zero.bearing_y, 0.0);
        assert_eq!(zero.inner_bearing_x, 1.0);
        assert_eq!(zero.inner_bearing_y, 13.0);
        assert_eq!(zero.advance, Some(12.0));

        let strike_1 = font.bitmap_strikes().get(1).unwrap();

        let zero = strike_1.get(GlyphId::new(2)).unwrap();
        assert_eq!(zero.width, 39);
        assert_eq!(zero.height, 52);
        assert_eq!(zero.bearing_x, 0.0);
        assert_eq!(zero.bearing_y, 0.0);
        assert_eq!(zero.inner_bearing_x, 6.0);
        assert_eq!(zero.inner_bearing_y, 52.0);
        assert_eq!(zero.advance, Some(51.0));
    }

    #[test]
    fn cbdt_glyph_selection() {
        let font = FontRef::new(font_test_data::CBDT).unwrap();
        let strikes = font.bitmap_strikes();

        let g1 = strikes
            .glyph_for_size(Size::new(12.0), GlyphId::new(2))
            .unwrap();
        assert_eq!(g1.ppem_x, 16.0);

        let g2 = strikes
            .glyph_for_size(Size::new(17.0), GlyphId::new(2))
            .unwrap();
        assert_eq!(g2.ppem_x, 64.0);

        let g3 = strikes
            .glyph_for_size(Size::new(60.0), GlyphId::new(2))
            .unwrap();
        assert_eq!(g3.ppem_x, 64.0);

        let g4 = strikes
            .glyph_for_size(Size::unscaled(), GlyphId::new(2))
            .unwrap();
        assert_eq!(g4.ppem_x, 128.0);
    }

    #[test]
    fn sbix_metadata() {
        let font = FontRef::new(font_test_data::NOTO_HANDWRITING_SBIX).unwrap();
        let strikes = font.bitmap_strikes();

        assert!(matches!(strikes.0, StrikesKind::Sbix(_, _)));
        assert!(matches!(strikes.len(), 1));

        assert!(matches!(strikes.get(0).unwrap().ppem(), 109.0));
    }

    #[test]
    fn sbix_glyph_metrics() {
        let font = FontRef::new(font_test_data::NOTO_HANDWRITING_SBIX).unwrap();
        let strike_0 = font.bitmap_strikes().get(0).unwrap();

        let g0 = strike_0.get(GlyphId::new(7)).unwrap();
        // `bearing_x` is always the lsb, which is 0 for this glyph.
        assert_eq!(g0.bearing_x, 0.0);
        // The glyph doesn't have an associated outline, so `bbox.min_y` is 0, and thus bearing_y
        // should also be 0.

        assert_eq!(g0.bearing_y, 0.0);
        // Origin offsets are 4.0 and -27.0 respectively.
        assert_eq!(g0.inner_bearing_x, 4.0);
        assert_eq!(g0.inner_bearing_y, -27.0);
        assert!(matches!(g0.data, BitmapData::Png(_)))
    }
}
