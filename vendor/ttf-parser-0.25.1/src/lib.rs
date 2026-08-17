/*!
A high-level, safe, zero-allocation font parser for:
* [TrueType](https://docs.microsoft.com/en-us/typography/truetype/),
* [OpenType](https://docs.microsoft.com/en-us/typography/opentype/spec/), and
* [AAT](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html)
  fonts.

Font parsing starts with a [`Face`].

## Features

- A high-level API for most common properties, hiding all parsing and data resolving logic.
- A low-level, but safe API to access TrueType tables data.
- Highly configurable. You can disable most of the features, reducing binary size.
  You can also parse TrueType tables separately, without loading the whole font/face.
- Zero heap allocations.
- Zero unsafe.
- Zero dependencies.
- `no_std`/WASM compatible.
- Fast.
- Stateless. All parsing methods are immutable.
- Simple and maintainable code (no magic numbers).

## Safety

- The library must not panic. Any panic considered as a critical bug and should be reported.
- The library forbids unsafe code.
- No heap allocations, so crash due to OOM is not possible.
- All recursive methods have a depth limit.
- Technically, should use less than 64KiB of stack in worst case scenario.
- Most of arithmetic operations are checked.
- Most of numeric casts are checked.
*/

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![allow(clippy::get_first)] // we use it for readability
#![allow(clippy::identity_op)] // we use it for readability
#![allow(clippy::too_many_arguments)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::bool_assert_comparison)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(any(feature = "std", feature = "no-std-float")))]
compile_error!("You have to activate either the `std` or the `no-std-float` feature.");

#[cfg(not(feature = "std"))]
use core_maths::CoreFloat;

#[cfg(feature = "apple-layout")]
mod aat;
#[cfg(feature = "variable-fonts")]
mod delta_set;
#[cfg(feature = "opentype-layout")]
mod ggg;
mod language;
mod parser;
mod tables;
#[cfg(feature = "variable-fonts")]
mod var_store;

use head::IndexToLocationFormat;
pub use parser::{Fixed, FromData, LazyArray16, LazyArray32, LazyArrayIter16, LazyArrayIter32};
use parser::{NumFrom, Offset, Offset32, Stream, TryNumFrom};

#[cfg(feature = "variable-fonts")]
pub use fvar::VariationAxis;

pub use language::Language;
pub use name::{name_id, PlatformId};
pub use os2::{Permissions, ScriptMetrics, Style, UnicodeRanges, Weight, Width};
pub use tables::CFFError;
#[cfg(feature = "apple-layout")]
pub use tables::{ankr, feat, kerx, morx, trak};
#[cfg(feature = "variable-fonts")]
pub use tables::{avar, cff2, fvar, gvar, hvar, mvar, vvar};
pub use tables::{cbdt, cblc, cff1 as cff, vhea};
pub use tables::{
    cmap, colr, cpal, glyf, head, hhea, hmtx, kern, loca, maxp, name, os2, post, sbix, stat, svg,
    vorg,
};
#[cfg(feature = "opentype-layout")]
pub use tables::{gdef, gpos, gsub, math};

#[cfg(feature = "opentype-layout")]
pub mod opentype_layout {
    //! This module contains
    //! [OpenType Layout](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#overview)
    //! supplementary tables implementation.
    pub use crate::ggg::*;
}

#[cfg(feature = "apple-layout")]
pub mod apple_layout {
    //! This module contains
    //! [Apple Advanced Typography Layout](
    //! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6AATIntro.html)
    //! supplementary tables implementation.
    pub use crate::aat::*;
}

/// A type-safe wrapper for glyph ID.
#[repr(transparent)]
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default, Debug, Hash)]
pub struct GlyphId(pub u16);

impl FromData for GlyphId {
    const SIZE: usize = 2;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u16::parse(data).map(GlyphId)
    }
}

/// A TrueType font magic.
///
/// https://docs.microsoft.com/en-us/typography/opentype/spec/otff#organization-of-an-opentype-font
#[derive(Clone, Copy, PartialEq, Debug)]
enum Magic {
    TrueType,
    OpenType,
    FontCollection,
}

impl FromData for Magic {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        match u32::parse(data)? {
            0x00010000 | 0x74727565 => Some(Magic::TrueType),
            0x4F54544F => Some(Magic::OpenType),
            0x74746366 => Some(Magic::FontCollection),
            _ => None,
        }
    }
}

/// A variation coordinate in a normalized coordinate system.
///
/// Basically any number in a -1.0..1.0 range.
/// Where 0 is a default value.
///
/// The number is stored as f2.16
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct NormalizedCoordinate(i16);

impl From<i16> for NormalizedCoordinate {
    /// Creates a new coordinate.
    ///
    /// The provided number will be clamped to the -16384..16384 range.
    #[inline]
    fn from(n: i16) -> Self {
        NormalizedCoordinate(parser::i16_bound(-16384, n, 16384))
    }
}

impl From<f32> for NormalizedCoordinate {
    /// Creates a new coordinate.
    ///
    /// The provided number will be clamped to the -1.0..1.0 range.
    #[inline]
    fn from(n: f32) -> Self {
        NormalizedCoordinate((parser::f32_bound(-1.0, n, 1.0) * 16384.0) as i16)
    }
}

impl NormalizedCoordinate {
    /// Returns the coordinate value as f2.14.
    #[inline]
    pub fn get(self) -> i16 {
        self.0
    }
}

/// A font variation value.
///
/// # Example
///
/// ```
/// use ttf_parser::{Variation, Tag};
///
/// Variation { axis: Tag::from_bytes(b"wght"), value: 500.0 };
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Variation {
    /// An axis tag name.
    pub axis: Tag,
    /// An axis value.
    pub value: f32,
}

/// A 4-byte tag.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(pub u32);

impl Tag {
    /// Creates a `Tag` from bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{}", ttf_parser::Tag::from_bytes(b"name"));
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: &[u8; 4]) -> Self {
        Tag(((bytes[0] as u32) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | (bytes[3] as u32))
    }

    /// Creates a `Tag` from bytes.
    ///
    /// In case of empty data will return `Tag` set to 0.
    ///
    /// When `bytes` are shorter than 4, will set missing bytes to ` `.
    ///
    /// Data after first 4 bytes is ignored.
    #[inline]
    pub fn from_bytes_lossy(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Tag::from_bytes(&[0, 0, 0, 0]);
        }

        let mut iter = bytes.iter().cloned().chain(core::iter::repeat(b' '));
        Tag::from_bytes(&[
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ])
    }

    /// Returns tag as 4-element byte array.
    #[inline]
    pub const fn to_bytes(self) -> [u8; 4] {
        [
            (self.0 >> 24 & 0xff) as u8,
            (self.0 >> 16 & 0xff) as u8,
            (self.0 >> 8 & 0xff) as u8,
            (self.0 >> 0 & 0xff) as u8,
        ]
    }

    /// Returns tag as 4-element byte array.
    #[inline]
    pub const fn to_chars(self) -> [char; 4] {
        [
            (self.0 >> 24 & 0xff) as u8 as char,
            (self.0 >> 16 & 0xff) as u8 as char,
            (self.0 >> 8 & 0xff) as u8 as char,
            (self.0 >> 0 & 0xff) as u8 as char,
        ]
    }

    /// Checks if tag is null / `[0, 0, 0, 0]`.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    /// Returns tag value as `u32` number.
    #[inline]
    pub const fn as_u32(&self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for Tag {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Tag({})", self)
    }
}

impl core::fmt::Display for Tag {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let b = self.to_chars();
        write!(
            f,
            "{}{}{}{}",
            b.get(0).unwrap_or(&' '),
            b.get(1).unwrap_or(&' '),
            b.get(2).unwrap_or(&' '),
            b.get(3).unwrap_or(&' ')
        )
    }
}

impl FromData for Tag {
    const SIZE: usize = 4;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        u32::parse(data).map(Tag)
    }
}

/// A line metrics.
///
/// Used for underline and strikeout.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LineMetrics {
    /// Line position.
    pub position: i16,

    /// Line thickness.
    pub thickness: i16,
}

/// A rectangle.
///
/// Doesn't guarantee that `x_min` <= `x_max` and/or `y_min` <= `y_max`.
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
}

impl Rect {
    #[inline]
    fn zero() -> Self {
        Self {
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
        }
    }

    /// Returns rect's width.
    #[inline]
    pub fn width(&self) -> i16 {
        self.x_max - self.x_min
    }

    /// Returns rect's height.
    #[inline]
    pub fn height(&self) -> i16 {
        self.y_max - self.y_min
    }
}

/// A rectangle described by the left-lower and upper-right points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    /// The horizontal minimum of the rect.
    pub x_min: f32,
    /// The vertical minimum of the rect.
    pub y_min: f32,
    /// The horizontal maximum of the rect.
    pub x_max: f32,
    /// The vertical maximum of the rect.
    pub y_max: f32,
}

impl RectF {
    #[inline]
    fn new() -> Self {
        RectF {
            x_min: f32::MAX,
            y_min: f32::MAX,
            x_max: f32::MIN,
            y_max: f32::MIN,
        }
    }

    #[inline]
    fn is_default(&self) -> bool {
        self.x_min == f32::MAX
            && self.y_min == f32::MAX
            && self.x_max == f32::MIN
            && self.y_max == f32::MIN
    }

    #[inline]
    fn extend_by(&mut self, x: f32, y: f32) {
        self.x_min = self.x_min.min(x);
        self.y_min = self.y_min.min(y);
        self.x_max = self.x_max.max(x);
        self.y_max = self.y_max.max(y);
    }

    #[inline]
    fn to_rect(self) -> Option<Rect> {
        Some(Rect {
            x_min: i16::try_num_from(self.x_min)?,
            y_min: i16::try_num_from(self.y_min)?,
            x_max: i16::try_num_from(self.x_max)?,
            y_max: i16::try_num_from(self.y_max)?,
        })
    }
}

/// An affine transform.
#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    /// The 'a' component of the transform.
    pub a: f32,
    /// The 'b' component of the transform.
    pub b: f32,
    /// The 'c' component of the transform.
    pub c: f32,
    /// The 'd' component of the transform.
    pub d: f32,
    /// The 'e' component of the transform.
    pub e: f32,
    /// The 'f' component of the transform.
    pub f: f32,
}

impl Transform {
    /// Creates a new transform with the specified components.
    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Transform { a, b, c, d, e, f }
    }

    /// Creates a new translation transform.
    #[inline]
    pub fn new_translate(tx: f32, ty: f32) -> Self {
        Transform::new(1.0, 0.0, 0.0, 1.0, tx, ty)
    }

    /// Creates a new rotation transform.
    #[inline]
    pub fn new_rotate(angle: f32) -> Self {
        let cc = (angle * core::f32::consts::PI).cos();
        let ss = (angle * core::f32::consts::PI).sin();

        Transform::new(cc, ss, -ss, cc, 0.0, 0.0)
    }

    /// Creates a new skew transform.
    #[inline]
    pub fn new_skew(skew_x: f32, skew_y: f32) -> Self {
        let x = (skew_x * core::f32::consts::PI).tan();
        let y = (skew_y * core::f32::consts::PI).tan();

        Transform::new(1.0, y, -x, 1.0, 0.0, 0.0)
    }

    /// Creates a new scale transform.
    #[inline]
    pub fn new_scale(sx: f32, sy: f32) -> Self {
        Transform::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Combines two transforms with each other.
    #[inline]
    pub fn combine(ts1: Self, ts2: Self) -> Self {
        Transform {
            a: ts1.a * ts2.a + ts1.c * ts2.b,
            b: ts1.b * ts2.a + ts1.d * ts2.b,
            c: ts1.a * ts2.c + ts1.c * ts2.d,
            d: ts1.b * ts2.c + ts1.d * ts2.d,
            e: ts1.a * ts2.e + ts1.c * ts2.f + ts1.e,
            f: ts1.b * ts2.e + ts1.d * ts2.f + ts1.f,
        }
    }

    #[inline]
    fn apply_to(&self, x: &mut f32, y: &mut f32) {
        let tx = *x;
        let ty = *y;
        *x = self.a * tx + self.c * ty + self.e;
        *y = self.b * tx + self.d * ty + self.f;
    }

    /// Checks whether a transform is the identity transform.
    #[inline]
    pub fn is_default(&self) -> bool {
        // A direct float comparison is fine in our case.
        self.a == 1.0
            && self.b == 0.0
            && self.c == 0.0
            && self.d == 1.0
            && self.e == 0.0
            && self.f == 0.0
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Transform {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }
}

impl core::fmt::Debug for Transform {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Transform({} {} {} {} {} {})",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }
}

/// A float point.
#[derive(Clone, Copy, Debug)]
pub struct PointF {
    /// The X-axis coordinate.
    pub x: f32,
    /// The Y-axis coordinate.
    pub y: f32,
}

/// Phantom points.
///
/// Available only for variable fonts with the `gvar` table.
#[derive(Clone, Copy, Debug)]
pub struct PhantomPoints {
    /// Left side bearing point.
    pub left: PointF,
    /// Right side bearing point.
    pub right: PointF,
    /// Top side bearing point.
    pub top: PointF,
    /// Bottom side bearing point.
    pub bottom: PointF,
}

/// A RGBA color in the sRGB color space.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RgbaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RgbaColor {
    /// Creates a new `RgbaColor`.
    #[inline]
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            blue,
            green,
            red,
            alpha,
        }
    }

    pub(crate) fn apply_alpha(&mut self, alpha: f32) {
        self.alpha = (((f32::from(self.alpha) / 255.0) * alpha) * 255.0) as u8;
    }
}

/// A trait for glyph outline construction.
pub trait OutlineBuilder {
    /// Appends a MoveTo segment.
    ///
    /// Start of a contour.
    fn move_to(&mut self, x: f32, y: f32);

    /// Appends a LineTo segment.
    fn line_to(&mut self, x: f32, y: f32);

    /// Appends a QuadTo segment.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32);

    /// Appends a CurveTo segment.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32);

    /// Appends a ClosePath segment.
    ///
    /// End of a contour.
    fn close(&mut self);
}

struct DummyOutline;
impl OutlineBuilder for DummyOutline {
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
}

/// A glyph raster image format.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RasterImageFormat {
    PNG,

    /// A monochrome bitmap.
    ///
    /// The most significant bit of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte. 1 corresponds
    /// to black, and 0 to white.
    BitmapMono,

    /// A packed monochrome bitmap.
    ///
    /// The most significant bit of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding. 1
    /// corresponds to black, and 0 to white.
    BitmapMonoPacked,

    /// A grayscale bitmap with 2 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte.
    BitmapGray2,

    /// A packed grayscale bitmap with 2 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding.
    BitmapGray2Packed,

    /// A grayscale bitmap with 4 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. The data for each row is padded to a byte
    /// boundary, so the next row begins with the most significant bit of a new byte.
    BitmapGray4,

    /// A packed grayscale bitmap with 4 bits per pixel.
    ///
    /// The most significant bits of the first byte corresponds to the top-left pixel, proceeding
    /// through succeeding bits moving left to right. Data is tightly packed with no padding.
    BitmapGray4Packed,

    /// A grayscale bitmap with 8 bits per pixel.
    ///
    /// The first byte corresponds to the top-left pixel, proceeding through succeeding bytes
    /// moving left to right.
    BitmapGray8,

    /// A color bitmap with 32 bits per pixel.
    ///
    /// The first group of four bytes corresponds to the top-left pixel, proceeding through
    /// succeeding pixels moving left to right. Each byte corresponds to a color channel and the
    /// channels within a pixel are in blue, green, red, alpha order. Color values are
    /// pre-multiplied by the alpha. For example, the color "full-green with half translucency"
    /// is encoded as `\x00\x80\x00\x80`, and not `\x00\xFF\x00\x80`.
    BitmapPremulBgra32,
}

/// A glyph's raster image.
///
/// Note, that glyph metrics are in pixels and not in font units.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RasterGlyphImage<'a> {
    /// Horizontal offset.
    pub x: i16,

    /// Vertical offset.
    pub y: i16,

    /// Image width.
    ///
    /// It doesn't guarantee that this value is the same as set in the `data`.
    pub width: u16,

    /// Image height.
    ///
    /// It doesn't guarantee that this value is the same as set in the `data`.
    pub height: u16,

    /// A pixels per em of the selected strike.
    pub pixels_per_em: u16,

    /// An image format.
    pub format: RasterImageFormat,

    /// A raw image data. It's up to the caller to decode it.
    pub data: &'a [u8],
}

/// A raw table record.
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub struct TableRecord {
    pub tag: Tag,
    #[allow(dead_code)]
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

impl FromData for TableRecord {
    const SIZE: usize = 16;

    #[inline]
    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(TableRecord {
            tag: s.read::<Tag>()?,
            check_sum: s.read::<u32>()?,
            offset: s.read::<u32>()?,
            length: s.read::<u32>()?,
        })
    }
}

#[cfg(feature = "variable-fonts")]
const MAX_VAR_COORDS: usize = 64;

#[cfg(feature = "variable-fonts")]
#[derive(Clone)]
struct VarCoords {
    data: [NormalizedCoordinate; MAX_VAR_COORDS],
    len: u8,
}

#[cfg(feature = "variable-fonts")]
impl Default for VarCoords {
    fn default() -> Self {
        Self {
            data: [NormalizedCoordinate::default(); MAX_VAR_COORDS],
            len: u8::default(),
        }
    }
}

#[cfg(feature = "variable-fonts")]
impl VarCoords {
    #[inline]
    fn as_slice(&self) -> &[NormalizedCoordinate] {
        &self.data[0..usize::from(self.len)]
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [NormalizedCoordinate] {
        let end = usize::from(self.len);
        &mut self.data[0..end]
    }
}

/// A list of font face parsing errors.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FaceParsingError {
    /// An attempt to read out of bounds detected.
    ///
    /// Should occur only on malformed fonts.
    MalformedFont,

    /// Face data must start with `0x00010000`, `0x74727565`, `0x4F54544F` or `0x74746366`.
    UnknownMagic,

    /// The face index is larger than the number of faces in the font.
    FaceIndexOutOfBounds,

    /// The `head` table is missing or malformed.
    NoHeadTable,

    /// The `hhea` table is missing or malformed.
    NoHheaTable,

    /// The `maxp` table is missing or malformed.
    NoMaxpTable,
}

impl core::fmt::Display for FaceParsingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FaceParsingError::MalformedFont => write!(f, "malformed font"),
            FaceParsingError::UnknownMagic => write!(f, "unknown magic"),
            FaceParsingError::FaceIndexOutOfBounds => write!(f, "face index is out of bounds"),
            FaceParsingError::NoHeadTable => write!(f, "the head table is missing or malformed"),
            FaceParsingError::NoHheaTable => write!(f, "the hhea table is missing or malformed"),
            FaceParsingError::NoMaxpTable => write!(f, "the maxp table is missing or malformed"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FaceParsingError {}

/// A raw font face.
///
/// You are probably looking for [`Face`]. This is a low-level type.
///
/// Unlike [`Face`], [`RawFace`] parses only face table records.
/// Meaning all you can get from this type is a raw (`&[u8]`) data of a requested table.
/// Then you can either parse just a singe table from a font/face or populate [`RawFaceTables`]
/// manually before passing it to [`Face::from_raw_tables`].
#[derive(Clone, Copy)]
pub struct RawFace<'a> {
    /// The input font file data.
    pub data: &'a [u8],
    /// An array of table records.
    pub table_records: LazyArray16<'a, TableRecord>,
}

impl<'a> RawFace<'a> {
    /// Creates a new [`RawFace`] from a raw data.
    ///
    /// `index` indicates the specific font face in a font collection.
    /// Use [`fonts_in_collection`] to get the total number of font faces.
    /// Set to 0 if unsure.
    ///
    /// While we do reuse [`FaceParsingError`], `No*Table` errors will not be throws.
    #[deprecated(since = "0.16.0", note = "use `parse` instead")]
    pub fn from_slice(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError> {
        Self::parse(data, index)
    }

    /// Creates a new [`RawFace`] from a raw data.
    ///
    /// `index` indicates the specific font face in a font collection.
    /// Use [`fonts_in_collection`] to get the total number of font faces.
    /// Set to 0 if unsure.
    ///
    /// While we do reuse [`FaceParsingError`], `No*Table` errors will not be throws.
    pub fn parse(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError> {
        // https://docs.microsoft.com/en-us/typography/opentype/spec/otff#organization-of-an-opentype-font

        let mut s = Stream::new(data);

        // Read **font** magic.
        let magic = s.read::<Magic>().ok_or(FaceParsingError::UnknownMagic)?;
        if magic == Magic::FontCollection {
            s.skip::<u32>(); // version
            let number_of_faces = s.read::<u32>().ok_or(FaceParsingError::MalformedFont)?;
            let offsets = s
                .read_array32::<Offset32>(number_of_faces)
                .ok_or(FaceParsingError::MalformedFont)?;

            let face_offset = offsets
                .get(index)
                .ok_or(FaceParsingError::FaceIndexOutOfBounds)?;
            // Face offset is from the start of the font data,
            // so we have to adjust it to the current parser offset.
            let face_offset = face_offset
                .to_usize()
                .checked_sub(s.offset())
                .ok_or(FaceParsingError::MalformedFont)?;
            s.advance_checked(face_offset)
                .ok_or(FaceParsingError::MalformedFont)?;

            // Read **face** magic.
            // Each face in a font collection also starts with a magic.
            let magic = s.read::<Magic>().ok_or(FaceParsingError::UnknownMagic)?;
            // And face in a font collection can't be another collection.
            if magic == Magic::FontCollection {
                return Err(FaceParsingError::UnknownMagic);
            }
        } else {
            // When reading from a regular font (not a collection) disallow index to be non-zero
            // Basically treat the font as a one-element collection
            if index != 0 {
                return Err(FaceParsingError::FaceIndexOutOfBounds);
            }
        }

        let num_tables = s.read::<u16>().ok_or(FaceParsingError::MalformedFont)?;
        s.advance(6); // searchRange (u16) + entrySelector (u16) + rangeShift (u16)
        let table_records = s
            .read_array16::<TableRecord>(num_tables)
            .ok_or(FaceParsingError::MalformedFont)?;

        Ok(RawFace {
            data,
            table_records,
        })
    }

    /// Returns the raw data of a selected table.
    pub fn table(&self, tag: Tag) -> Option<&'a [u8]> {
        let (_, table) = self
            .table_records
            .binary_search_by(|record| record.tag.cmp(&tag))?;
        let offset = usize::num_from(table.offset);
        let length = usize::num_from(table.length);
        let end = offset.checked_add(length)?;
        self.data.get(offset..end)
    }
}

impl core::fmt::Debug for RawFace<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RawFace {{ ... }}")
    }
}

/// A list of all supported tables as raw data.
///
/// This type should be used in tandem with
/// [`Face::from_raw_tables()`](struct.Face.html#method.from_raw_tables).
///
/// This allows loading font faces not only from TrueType font files,
/// but from any source. Mainly used for parsing WOFF.
#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
#[derive(Clone, Default)]
pub struct RawFaceTables<'a> {
    // Mandatory tables.
    pub head: &'a [u8],
    pub hhea: &'a [u8],
    pub maxp: &'a [u8],

    pub bdat: Option<&'a [u8]>,
    pub bloc: Option<&'a [u8]>,
    pub cbdt: Option<&'a [u8]>,
    pub cblc: Option<&'a [u8]>,
    pub cff: Option<&'a [u8]>,
    pub cmap: Option<&'a [u8]>,
    pub colr: Option<&'a [u8]>,
    pub cpal: Option<&'a [u8]>,
    pub ebdt: Option<&'a [u8]>,
    pub eblc: Option<&'a [u8]>,
    pub glyf: Option<&'a [u8]>,
    pub hmtx: Option<&'a [u8]>,
    pub kern: Option<&'a [u8]>,
    pub loca: Option<&'a [u8]>,
    pub name: Option<&'a [u8]>,
    pub os2: Option<&'a [u8]>,
    pub post: Option<&'a [u8]>,
    pub sbix: Option<&'a [u8]>,
    pub stat: Option<&'a [u8]>,
    pub svg: Option<&'a [u8]>,
    pub vhea: Option<&'a [u8]>,
    pub vmtx: Option<&'a [u8]>,
    pub vorg: Option<&'a [u8]>,

    #[cfg(feature = "opentype-layout")]
    pub gdef: Option<&'a [u8]>,
    #[cfg(feature = "opentype-layout")]
    pub gpos: Option<&'a [u8]>,
    #[cfg(feature = "opentype-layout")]
    pub gsub: Option<&'a [u8]>,
    #[cfg(feature = "opentype-layout")]
    pub math: Option<&'a [u8]>,

    #[cfg(feature = "apple-layout")]
    pub ankr: Option<&'a [u8]>,
    #[cfg(feature = "apple-layout")]
    pub feat: Option<&'a [u8]>,
    #[cfg(feature = "apple-layout")]
    pub kerx: Option<&'a [u8]>,
    #[cfg(feature = "apple-layout")]
    pub morx: Option<&'a [u8]>,
    #[cfg(feature = "apple-layout")]
    pub trak: Option<&'a [u8]>,

    #[cfg(feature = "variable-fonts")]
    pub avar: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub cff2: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub fvar: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub gvar: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub hvar: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub mvar: Option<&'a [u8]>,
    #[cfg(feature = "variable-fonts")]
    pub vvar: Option<&'a [u8]>,
}

/// Parsed face tables.
///
/// Unlike [`Face`], provides a low-level parsing abstraction over TrueType tables.
/// Useful when you need a direct access to tables data.
///
/// Also, used when high-level API is problematic to implement.
/// A good example would be OpenType layout tables (GPOS/GSUB).
#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct FaceTables<'a> {
    // Mandatory tables.
    pub head: head::Table,
    pub hhea: hhea::Table,
    pub maxp: maxp::Table,

    pub bdat: Option<cbdt::Table<'a>>,
    pub cbdt: Option<cbdt::Table<'a>>,
    pub cff: Option<cff::Table<'a>>,
    pub cmap: Option<cmap::Table<'a>>,
    pub colr: Option<colr::Table<'a>>,
    pub ebdt: Option<cbdt::Table<'a>>,
    pub glyf: Option<glyf::Table<'a>>,
    pub hmtx: Option<hmtx::Table<'a>>,
    pub kern: Option<kern::Table<'a>>,
    pub name: Option<name::Table<'a>>,
    pub os2: Option<os2::Table<'a>>,
    pub post: Option<post::Table<'a>>,
    pub sbix: Option<sbix::Table<'a>>,
    pub stat: Option<stat::Table<'a>>,
    pub svg: Option<svg::Table<'a>>,
    pub vhea: Option<vhea::Table>,
    pub vmtx: Option<hmtx::Table<'a>>,
    pub vorg: Option<vorg::Table<'a>>,

    #[cfg(feature = "opentype-layout")]
    pub gdef: Option<gdef::Table<'a>>,
    #[cfg(feature = "opentype-layout")]
    pub gpos: Option<opentype_layout::LayoutTable<'a>>,
    #[cfg(feature = "opentype-layout")]
    pub gsub: Option<opentype_layout::LayoutTable<'a>>,
    #[cfg(feature = "opentype-layout")]
    pub math: Option<math::Table<'a>>,

    #[cfg(feature = "apple-layout")]
    pub ankr: Option<ankr::Table<'a>>,
    #[cfg(feature = "apple-layout")]
    pub feat: Option<feat::Table<'a>>,
    #[cfg(feature = "apple-layout")]
    pub kerx: Option<kerx::Table<'a>>,
    #[cfg(feature = "apple-layout")]
    pub morx: Option<morx::Table<'a>>,
    #[cfg(feature = "apple-layout")]
    pub trak: Option<trak::Table<'a>>,

    #[cfg(feature = "variable-fonts")]
    pub avar: Option<avar::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub cff2: Option<cff2::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub fvar: Option<fvar::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub gvar: Option<gvar::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub hvar: Option<hvar::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub mvar: Option<mvar::Table<'a>>,
    #[cfg(feature = "variable-fonts")]
    pub vvar: Option<vvar::Table<'a>>,
}

/// A font face.
///
/// Provides a high-level API for working with TrueType fonts.
/// If you're not familiar with how TrueType works internally, you should use this type.
/// If you do know and want a bit more low-level access - checkout [`FaceTables`].
///
/// Note that `Face` doesn't own the font data and doesn't allocate anything in heap.
/// Therefore you cannot "store" it. The idea is that you should parse the `Face`
/// when needed, get required data and forget about it.
/// That's why the initial parsing is highly optimized and should not become a bottleneck.
///
/// If you still want to store `Face` - checkout
/// [owned_ttf_parser](https://crates.io/crates/owned_ttf_parser). Requires `unsafe`.
///
/// While `Face` is technically copyable, we disallow it because it's almost 2KB big.
#[derive(Clone)]
pub struct Face<'a> {
    raw_face: RawFace<'a>,
    tables: FaceTables<'a>, // Parsed tables.
    #[cfg(feature = "variable-fonts")]
    coordinates: VarCoords,
}

impl<'a> Face<'a> {
    /// Creates a new [`Face`] from a raw data.
    ///
    /// `index` indicates the specific font face in a font collection.
    /// Use [`fonts_in_collection`] to get the total number of font faces.
    /// Set to 0 if unsure.
    ///
    /// This method will do some parsing and sanitization,
    /// but in general can be considered free. No significant performance overhead.
    ///
    /// Required tables: `head`, `hhea` and `maxp`.
    ///
    /// If an optional table has invalid data it will be skipped.
    #[deprecated(since = "0.16.0", note = "use `parse` instead")]
    pub fn from_slice(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError> {
        Self::parse(data, index)
    }

    /// Creates a new [`Face`] from a raw data.
    ///
    /// `index` indicates the specific font face in a font collection.
    /// Use [`fonts_in_collection`] to get the total number of font faces.
    /// Set to 0 if unsure.
    ///
    /// This method will do some parsing and sanitization,
    /// but in general can be considered free. No significant performance overhead.
    ///
    /// Required tables: `head`, `hhea` and `maxp`.
    ///
    /// If an optional table has invalid data it will be skipped.
    pub fn parse(data: &'a [u8], index: u32) -> Result<Self, FaceParsingError> {
        let raw_face = RawFace::parse(data, index)?;
        let raw_tables = Self::collect_tables(raw_face);

        #[allow(unused_mut)]
        let mut face = Face {
            raw_face,
            #[cfg(feature = "variable-fonts")]
            coordinates: VarCoords::default(),
            tables: Self::parse_tables(raw_tables)?,
        };

        #[cfg(feature = "variable-fonts")]
        {
            if let Some(ref fvar) = face.tables.fvar {
                face.coordinates.len = fvar.axes.len().min(MAX_VAR_COORDS as u16) as u8;
            }
        }

        Ok(face)
    }

    fn collect_tables(raw_face: RawFace<'a>) -> RawFaceTables<'a> {
        let mut tables = RawFaceTables::default();

        for record in raw_face.table_records {
            let start = usize::num_from(record.offset);
            let end = match start.checked_add(usize::num_from(record.length)) {
                Some(v) => v,
                None => continue,
            };

            let table_data = raw_face.data.get(start..end);
            match &record.tag.to_bytes() {
                b"bdat" => tables.bdat = table_data,
                b"bloc" => tables.bloc = table_data,
                b"CBDT" => tables.cbdt = table_data,
                b"CBLC" => tables.cblc = table_data,
                b"CFF " => tables.cff = table_data,
                #[cfg(feature = "variable-fonts")]
                b"CFF2" => tables.cff2 = table_data,
                b"COLR" => tables.colr = table_data,
                b"CPAL" => tables.cpal = table_data,
                b"EBDT" => tables.ebdt = table_data,
                b"EBLC" => tables.eblc = table_data,
                #[cfg(feature = "opentype-layout")]
                b"GDEF" => tables.gdef = table_data,
                #[cfg(feature = "opentype-layout")]
                b"GPOS" => tables.gpos = table_data,
                #[cfg(feature = "opentype-layout")]
                b"GSUB" => tables.gsub = table_data,
                #[cfg(feature = "opentype-layout")]
                b"MATH" => tables.math = table_data,
                #[cfg(feature = "variable-fonts")]
                b"HVAR" => tables.hvar = table_data,
                #[cfg(feature = "variable-fonts")]
                b"MVAR" => tables.mvar = table_data,
                b"OS/2" => tables.os2 = table_data,
                b"SVG " => tables.svg = table_data,
                b"VORG" => tables.vorg = table_data,
                #[cfg(feature = "variable-fonts")]
                b"VVAR" => tables.vvar = table_data,
                #[cfg(feature = "apple-layout")]
                b"ankr" => tables.ankr = table_data,
                #[cfg(feature = "variable-fonts")]
                b"avar" => tables.avar = table_data,
                b"cmap" => tables.cmap = table_data,
                #[cfg(feature = "apple-layout")]
                b"feat" => tables.feat = table_data,
                #[cfg(feature = "variable-fonts")]
                b"fvar" => tables.fvar = table_data,
                b"glyf" => tables.glyf = table_data,
                #[cfg(feature = "variable-fonts")]
                b"gvar" => tables.gvar = table_data,
                b"head" => tables.head = table_data.unwrap_or_default(),
                b"hhea" => tables.hhea = table_data.unwrap_or_default(),
                b"hmtx" => tables.hmtx = table_data,
                b"kern" => tables.kern = table_data,
                #[cfg(feature = "apple-layout")]
                b"kerx" => tables.kerx = table_data,
                b"loca" => tables.loca = table_data,
                b"maxp" => tables.maxp = table_data.unwrap_or_default(),
                #[cfg(feature = "apple-layout")]
                b"morx" => tables.morx = table_data,
                b"name" => tables.name = table_data,
                b"post" => tables.post = table_data,
                b"sbix" => tables.sbix = table_data,
                b"STAT" => tables.stat = table_data,
                #[cfg(feature = "apple-layout")]
                b"trak" => tables.trak = table_data,
                b"vhea" => tables.vhea = table_data,
                b"vmtx" => tables.vmtx = table_data,
                _ => {}
            }
        }

        tables
    }

    /// Creates a new [`Face`] from provided [`RawFaceTables`].
    pub fn from_raw_tables(raw_tables: RawFaceTables<'a>) -> Result<Self, FaceParsingError> {
        #[allow(unused_mut)]
        let mut face = Face {
            raw_face: RawFace {
                data: &[],
                table_records: LazyArray16::default(),
            },
            #[cfg(feature = "variable-fonts")]
            coordinates: VarCoords::default(),
            tables: Self::parse_tables(raw_tables)?,
        };

        #[cfg(feature = "variable-fonts")]
        {
            if let Some(ref fvar) = face.tables.fvar {
                face.coordinates.len = fvar.axes.len().min(MAX_VAR_COORDS as u16) as u8;
            }
        }

        Ok(face)
    }

    fn parse_tables(raw_tables: RawFaceTables<'a>) -> Result<FaceTables<'a>, FaceParsingError> {
        let head = head::Table::parse(raw_tables.head).ok_or(FaceParsingError::NoHeadTable)?;
        let hhea = hhea::Table::parse(raw_tables.hhea).ok_or(FaceParsingError::NoHheaTable)?;
        let maxp = maxp::Table::parse(raw_tables.maxp).ok_or(FaceParsingError::NoMaxpTable)?;

        let hmtx = raw_tables.hmtx.and_then(|data| {
            hmtx::Table::parse(hhea.number_of_metrics, maxp.number_of_glyphs, data)
        });

        let vhea = raw_tables.vhea.and_then(vhea::Table::parse);
        let vmtx = if let Some(vhea) = vhea {
            raw_tables.vmtx.and_then(|data| {
                hmtx::Table::parse(vhea.number_of_metrics, maxp.number_of_glyphs, data)
            })
        } else {
            None
        };

        let loca = raw_tables.loca.and_then(|data| {
            loca::Table::parse(maxp.number_of_glyphs, head.index_to_location_format, data)
        });
        let glyf = if let Some(loca) = loca {
            raw_tables
                .glyf
                .and_then(|data| glyf::Table::parse(loca, data))
        } else {
            None
        };

        let bdat = if let Some(bloc) = raw_tables.bloc.and_then(cblc::Table::parse) {
            raw_tables
                .bdat
                .and_then(|data| cbdt::Table::parse(bloc, data))
        } else {
            None
        };

        let cbdt = if let Some(cblc) = raw_tables.cblc.and_then(cblc::Table::parse) {
            raw_tables
                .cbdt
                .and_then(|data| cbdt::Table::parse(cblc, data))
        } else {
            None
        };

        let ebdt = if let Some(eblc) = raw_tables.eblc.and_then(cblc::Table::parse) {
            raw_tables
                .ebdt
                .and_then(|data| cbdt::Table::parse(eblc, data))
        } else {
            None
        };

        let cpal = raw_tables.cpal.and_then(cpal::Table::parse);
        let colr = if let Some(cpal) = cpal {
            raw_tables
                .colr
                .and_then(|data| colr::Table::parse(cpal, data))
        } else {
            None
        };

        Ok(FaceTables {
            head,
            hhea,
            maxp,

            bdat,
            cbdt,
            cff: raw_tables.cff.and_then(cff::Table::parse),
            cmap: raw_tables.cmap.and_then(cmap::Table::parse),
            colr,
            ebdt,
            glyf,
            hmtx,
            kern: raw_tables.kern.and_then(kern::Table::parse),
            name: raw_tables.name.and_then(name::Table::parse),
            os2: raw_tables.os2.and_then(os2::Table::parse),
            post: raw_tables.post.and_then(post::Table::parse),
            sbix: raw_tables
                .sbix
                .and_then(|data| sbix::Table::parse(maxp.number_of_glyphs, data)),
            stat: raw_tables.stat.and_then(stat::Table::parse),
            svg: raw_tables.svg.and_then(svg::Table::parse),
            vhea: raw_tables.vhea.and_then(vhea::Table::parse),
            vmtx,
            vorg: raw_tables.vorg.and_then(vorg::Table::parse),

            #[cfg(feature = "opentype-layout")]
            gdef: raw_tables.gdef.and_then(gdef::Table::parse),
            #[cfg(feature = "opentype-layout")]
            gpos: raw_tables
                .gpos
                .and_then(opentype_layout::LayoutTable::parse),
            #[cfg(feature = "opentype-layout")]
            gsub: raw_tables
                .gsub
                .and_then(opentype_layout::LayoutTable::parse),
            #[cfg(feature = "opentype-layout")]
            math: raw_tables.math.and_then(math::Table::parse),

            #[cfg(feature = "apple-layout")]
            ankr: raw_tables
                .ankr
                .and_then(|data| ankr::Table::parse(maxp.number_of_glyphs, data)),
            #[cfg(feature = "apple-layout")]
            feat: raw_tables.feat.and_then(feat::Table::parse),
            #[cfg(feature = "apple-layout")]
            kerx: raw_tables
                .kerx
                .and_then(|data| kerx::Table::parse(maxp.number_of_glyphs, data)),
            #[cfg(feature = "apple-layout")]
            morx: raw_tables
                .morx
                .and_then(|data| morx::Table::parse(maxp.number_of_glyphs, data)),
            #[cfg(feature = "apple-layout")]
            trak: raw_tables.trak.and_then(trak::Table::parse),

            #[cfg(feature = "variable-fonts")]
            avar: raw_tables.avar.and_then(avar::Table::parse),
            #[cfg(feature = "variable-fonts")]
            cff2: raw_tables.cff2.and_then(cff2::Table::parse),
            #[cfg(feature = "variable-fonts")]
            fvar: raw_tables.fvar.and_then(fvar::Table::parse),
            #[cfg(feature = "variable-fonts")]
            gvar: raw_tables.gvar.and_then(gvar::Table::parse),
            #[cfg(feature = "variable-fonts")]
            hvar: raw_tables.hvar.and_then(hvar::Table::parse),
            #[cfg(feature = "variable-fonts")]
            mvar: raw_tables.mvar.and_then(mvar::Table::parse),
            #[cfg(feature = "variable-fonts")]
            vvar: raw_tables.vvar.and_then(vvar::Table::parse),
        })
    }

    /// Returns low-level face tables.
    #[inline]
    pub fn tables(&self) -> &FaceTables<'a> {
        &self.tables
    }

    /// Returns the `RawFace` used to create this `Face`.
    ///
    /// Useful if you want to parse the data manually.
    ///
    /// Available only for faces created using [`Face::parse()`](struct.Face.html#method.parse).
    #[inline]
    pub fn raw_face(&self) -> &RawFace<'a> {
        &self.raw_face
    }

    /// Returns the raw data of a selected table.
    ///
    /// Useful if you want to parse the data manually.
    ///
    /// Available only for faces created using [`Face::parse()`](struct.Face.html#method.parse).
    #[deprecated(since = "0.16.0", note = "use `self.raw_face().table()` instead")]
    #[inline]
    pub fn table_data(&self, tag: Tag) -> Option<&'a [u8]> {
        self.raw_face.table(tag)
    }

    /// Returns a list of names.
    ///
    /// Contains face name and other strings.
    #[inline]
    pub fn names(&self) -> name::Names<'a> {
        self.tables.name.unwrap_or_default().names
    }

    /// Checks that face is marked as *Regular*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[inline]
    pub fn is_regular(&self) -> bool {
        self.style() == Style::Normal
    }

    /// Checks that face is marked as *Italic*.
    #[inline]
    pub fn is_italic(&self) -> bool {
        // A face can have a Normal style and a non-zero italic angle, which also makes it italic.
        self.style() == Style::Italic || self.italic_angle() != 0.0
    }

    /// Checks that face is marked as *Bold*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[inline]
    pub fn is_bold(&self) -> bool {
        self.tables.os2.map(|os2| os2.is_bold()).unwrap_or(false)
    }

    /// Checks that face is marked as *Oblique*.
    ///
    /// Returns `false` when OS/2 table is not present or when its version is < 4.
    #[inline]
    pub fn is_oblique(&self) -> bool {
        self.style() == Style::Oblique
    }

    /// Returns face style.
    #[inline]
    pub fn style(&self) -> Style {
        self.tables.os2.map(|os2| os2.style()).unwrap_or_default()
    }

    /// Checks that face is marked as *Monospaced*.
    ///
    /// Returns `false` when `post` table is not present.
    #[inline]
    pub fn is_monospaced(&self) -> bool {
        self.tables
            .post
            .map(|post| post.is_monospaced)
            .unwrap_or(false)
    }

    /// Checks that face is variable.
    ///
    /// Simply checks the presence of a `fvar` table.
    #[inline]
    pub fn is_variable(&self) -> bool {
        #[cfg(feature = "variable-fonts")]
        {
            // `fvar::Table::parse` already checked that `axisCount` is non-zero.
            self.tables.fvar.is_some()
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            false
        }
    }

    /// Returns face's weight.
    ///
    /// Returns `Weight::Normal` when OS/2 table is not present.
    #[inline]
    pub fn weight(&self) -> Weight {
        self.tables.os2.map(|os2| os2.weight()).unwrap_or_default()
    }

    /// Returns face's width.
    ///
    /// Returns `Width::Normal` when OS/2 table is not present or when value is invalid.
    #[inline]
    pub fn width(&self) -> Width {
        self.tables.os2.map(|os2| os2.width()).unwrap_or_default()
    }

    /// Returns face's italic angle.
    ///
    /// Returns `0.0` when `post` table is not present.
    #[inline]
    pub fn italic_angle(&self) -> f32 {
        self.tables
            .post
            .map(|table| table.italic_angle)
            .unwrap_or(0.0)
    }

    // Read https://github.com/freetype/freetype/blob/49270c17011491227ec7bd3fb73ede4f674aa065/src/sfnt/sfobjs.c#L1279
    // to learn more about the logic behind the following functions.

    /// Returns a horizontal face ascender.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn ascender(&self) -> i16 {
        if let Some(os_2) = self.tables.os2 {
            if os_2.use_typographic_metrics() {
                let value = os_2.typographic_ascender();
                return self.apply_metrics_variation(Tag::from_bytes(b"hasc"), value);
            }
        }

        let mut value = self.tables.hhea.ascender;
        if value == 0 {
            if let Some(os_2) = self.tables.os2 {
                value = os_2.typographic_ascender();
                if value == 0 {
                    value = os_2.windows_ascender();
                    value = self.apply_metrics_variation(Tag::from_bytes(b"hcla"), value);
                } else {
                    value = self.apply_metrics_variation(Tag::from_bytes(b"hasc"), value);
                }
            }
        }

        value
    }

    /// Returns a horizontal face descender.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn descender(&self) -> i16 {
        if let Some(os_2) = self.tables.os2 {
            if os_2.use_typographic_metrics() {
                let value = os_2.typographic_descender();
                return self.apply_metrics_variation(Tag::from_bytes(b"hdsc"), value);
            }
        }

        let mut value = self.tables.hhea.descender;
        if value == 0 {
            if let Some(os_2) = self.tables.os2 {
                value = os_2.typographic_descender();
                if value == 0 {
                    value = os_2.windows_descender();
                    value = self.apply_metrics_variation(Tag::from_bytes(b"hcld"), value);
                } else {
                    value = self.apply_metrics_variation(Tag::from_bytes(b"hdsc"), value);
                }
            }
        }

        value
    }

    /// Returns face's height.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn height(&self) -> i16 {
        self.ascender() - self.descender()
    }

    /// Returns a horizontal face line gap.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn line_gap(&self) -> i16 {
        if let Some(os_2) = self.tables.os2 {
            if os_2.use_typographic_metrics() {
                let value = os_2.typographic_line_gap();
                return self.apply_metrics_variation(Tag::from_bytes(b"hlgp"), value);
            }
        }

        let mut value = self.tables.hhea.line_gap;
        // For line gap, we have to check that ascender or descender are 0, not line gap itself.
        if self.tables.hhea.ascender == 0 || self.tables.hhea.descender == 0 {
            if let Some(os_2) = self.tables.os2 {
                if os_2.typographic_ascender() != 0 || os_2.typographic_descender() != 0 {
                    value = os_2.typographic_line_gap();
                    value = self.apply_metrics_variation(Tag::from_bytes(b"hlgp"), value);
                } else {
                    value = 0;
                }
            }
        }

        value
    }

    /// Returns a horizontal typographic face ascender.
    ///
    /// Prefer `Face::ascender` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn typographic_ascender(&self) -> Option<i16> {
        self.tables.os2.map(|table| {
            let v = table.typographic_ascender();
            self.apply_metrics_variation(Tag::from_bytes(b"hasc"), v)
        })
    }

    /// Returns a horizontal typographic face descender.
    ///
    /// Prefer `Face::descender` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn typographic_descender(&self) -> Option<i16> {
        self.tables.os2.map(|table| {
            let v = table.typographic_descender();
            self.apply_metrics_variation(Tag::from_bytes(b"hdsc"), v)
        })
    }

    /// Returns a horizontal typographic face line gap.
    ///
    /// Prefer `Face::line_gap` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn typographic_line_gap(&self) -> Option<i16> {
        self.tables.os2.map(|table| {
            let v = table.typographic_line_gap();
            self.apply_metrics_variation(Tag::from_bytes(b"hlgp"), v)
        })
    }

    /// Returns a vertical face ascender.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn vertical_ascender(&self) -> Option<i16> {
        self.tables
            .vhea
            .map(|vhea| vhea.ascender)
            .map(|v| self.apply_metrics_variation(Tag::from_bytes(b"vasc"), v))
    }

    /// Returns a vertical face descender.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn vertical_descender(&self) -> Option<i16> {
        self.tables
            .vhea
            .map(|vhea| vhea.descender)
            .map(|v| self.apply_metrics_variation(Tag::from_bytes(b"vdsc"), v))
    }

    /// Returns a vertical face height.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn vertical_height(&self) -> Option<i16> {
        Some(self.vertical_ascender()? - self.vertical_descender()?)
    }

    /// Returns a vertical face line gap.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn vertical_line_gap(&self) -> Option<i16> {
        self.tables
            .vhea
            .map(|vhea| vhea.line_gap)
            .map(|v| self.apply_metrics_variation(Tag::from_bytes(b"vlgp"), v))
    }

    /// Returns face's units per EM.
    ///
    /// Guarantee to be in a 16..=16384 range.
    #[inline]
    pub fn units_per_em(&self) -> u16 {
        self.tables.head.units_per_em
    }

    /// Returns face's x height.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present or when its version is < 2.
    #[inline]
    pub fn x_height(&self) -> Option<i16> {
        self.tables
            .os2
            .and_then(|os_2| os_2.x_height())
            .map(|v| self.apply_metrics_variation(Tag::from_bytes(b"xhgt"), v))
    }

    /// Returns face's capital height.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present or when its version is < 2.
    #[inline]
    pub fn capital_height(&self) -> Option<i16> {
        self.tables
            .os2
            .and_then(|os_2| os_2.capital_height())
            .map(|v| self.apply_metrics_variation(Tag::from_bytes(b"cpht"), v))
    }

    /// Returns face's underline metrics.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when `post` table is not present.
    #[inline]
    pub fn underline_metrics(&self) -> Option<LineMetrics> {
        let mut metrics = self.tables.post?.underline_metrics;

        if self.is_variable() {
            self.apply_metrics_variation_to(Tag::from_bytes(b"undo"), &mut metrics.position);
            self.apply_metrics_variation_to(Tag::from_bytes(b"unds"), &mut metrics.thickness);
        }

        Some(metrics)
    }

    /// Returns face's strikeout metrics.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn strikeout_metrics(&self) -> Option<LineMetrics> {
        let mut metrics = self.tables.os2?.strikeout_metrics();

        if self.is_variable() {
            self.apply_metrics_variation_to(Tag::from_bytes(b"stro"), &mut metrics.position);
            self.apply_metrics_variation_to(Tag::from_bytes(b"strs"), &mut metrics.thickness);
        }

        Some(metrics)
    }

    /// Returns face's subscript metrics.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn subscript_metrics(&self) -> Option<ScriptMetrics> {
        let mut metrics = self.tables.os2?.subscript_metrics();

        if self.is_variable() {
            self.apply_metrics_variation_to(Tag::from_bytes(b"sbxs"), &mut metrics.x_size);
            self.apply_metrics_variation_to(Tag::from_bytes(b"sbys"), &mut metrics.y_size);
            self.apply_metrics_variation_to(Tag::from_bytes(b"sbxo"), &mut metrics.x_offset);
            self.apply_metrics_variation_to(Tag::from_bytes(b"sbyo"), &mut metrics.y_offset);
        }

        Some(metrics)
    }

    /// Returns face's superscript metrics.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[inline]
    pub fn superscript_metrics(&self) -> Option<ScriptMetrics> {
        let mut metrics = self.tables.os2?.superscript_metrics();

        if self.is_variable() {
            self.apply_metrics_variation_to(Tag::from_bytes(b"spxs"), &mut metrics.x_size);
            self.apply_metrics_variation_to(Tag::from_bytes(b"spys"), &mut metrics.y_size);
            self.apply_metrics_variation_to(Tag::from_bytes(b"spxo"), &mut metrics.x_offset);
            self.apply_metrics_variation_to(Tag::from_bytes(b"spyo"), &mut metrics.y_offset);
        }

        Some(metrics)
    }

    /// Returns face permissions.
    ///
    /// Returns `None` in case of a malformed value.
    #[inline]
    pub fn permissions(&self) -> Option<Permissions> {
        self.tables.os2?.permissions()
    }

    /// Checks if the face allows embedding a subset, further restricted by [`Self::permissions`].
    #[inline]
    pub fn is_subsetting_allowed(&self) -> bool {
        self.tables
            .os2
            .map(|t| t.is_subsetting_allowed())
            .unwrap_or(false)
    }

    /// Checks if the face allows outline data to be embedded.
    ///
    /// If false, only bitmaps may be embedded in accordance with [`Self::permissions`].
    ///
    /// If the font contains no bitmaps and this flag is not set, it implies no embedding is allowed.
    #[inline]
    pub fn is_outline_embedding_allowed(&self) -> bool {
        self.tables
            .os2
            .map(|t| t.is_outline_embedding_allowed())
            .unwrap_or(false)
    }

    /// Returns [Unicode Ranges](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ur).
    #[inline]
    pub fn unicode_ranges(&self) -> UnicodeRanges {
        self.tables
            .os2
            .map(|t| t.unicode_ranges())
            .unwrap_or_default()
    }

    /// Returns a total number of glyphs in the face.
    ///
    /// Never zero.
    ///
    /// The value was already parsed, so this function doesn't involve any parsing.
    #[inline]
    pub fn number_of_glyphs(&self) -> u16 {
        self.tables.maxp.number_of_glyphs.get()
    }

    /// Resolves a Glyph ID for a code point.
    ///
    /// Returns `None` instead of `0` when glyph is not found.
    ///
    /// All subtable formats except Mixed Coverage (8) are supported.
    ///
    /// If you need a more low-level control, prefer `Face::tables().cmap`.
    #[inline]
    pub fn glyph_index(&self, code_point: char) -> Option<GlyphId> {
        for subtable in self.tables.cmap?.subtables {
            if !subtable.is_unicode() {
                continue;
            }

            if let Some(id) = subtable.glyph_index(u32::from(code_point)) {
                return Some(id);
            }
        }

        None
    }

    /// Resolves a Glyph ID for a glyph name.
    ///
    /// Uses the `post` and `CFF` tables as sources.
    ///
    /// Returns `None` when no name is associated with a `glyph`.
    #[cfg(feature = "glyph-names")]
    #[inline]
    pub fn glyph_index_by_name(&self, name: &str) -> Option<GlyphId> {
        if let Some(name) = self
            .tables
            .post
            .and_then(|post| post.glyph_index_by_name(name))
        {
            return Some(name);
        }

        if let Some(name) = self
            .tables
            .cff
            .as_ref()
            .and_then(|cff| cff.glyph_index_by_name(name))
        {
            return Some(name);
        }

        None
    }

    /// Resolves a variation of a Glyph ID from two code points.
    ///
    /// Implemented according to
    /// [Unicode Variation Sequences](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/cmap#format-14-unicode-variation-sequences).
    ///
    /// Returns `None` instead of `0` when glyph is not found.
    #[inline]
    pub fn glyph_variation_index(&self, code_point: char, variation: char) -> Option<GlyphId> {
        for subtable in self.tables.cmap?.subtables {
            if let cmap::Format::UnicodeVariationSequences(ref table) = subtable.format {
                return match table.glyph_index(u32::from(code_point), u32::from(variation))? {
                    cmap::GlyphVariationResult::Found(v) => Some(v),
                    cmap::GlyphVariationResult::UseDefault => self.glyph_index(code_point),
                };
            }
        }

        None
    }

    /// Returns glyph's horizontal advance.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn glyph_hor_advance(&self, glyph_id: GlyphId) -> Option<u16> {
        #[cfg(feature = "variable-fonts")]
        {
            let mut advance = self.tables.hmtx?.advance(glyph_id)? as f32;

            if self.is_variable() {
                // Ignore variation offset when `hvar` is not set.
                if let Some(hvar) = self.tables.hvar {
                    if let Some(offset) = hvar.advance_offset(glyph_id, self.coords()) {
                        // We can't use `round()` in `no_std`, so this is the next best thing.
                        advance += offset + 0.5;
                    }
                } else if let Some(points) = self.glyph_phantom_points(glyph_id) {
                    // We can't use `round()` in `no_std`, so this is the next best thing.
                    advance += points.right.x + 0.5
                }
            }

            u16::try_num_from(advance)
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            self.tables.hmtx?.advance(glyph_id)
        }
    }

    /// Returns glyph's vertical advance.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn glyph_ver_advance(&self, glyph_id: GlyphId) -> Option<u16> {
        #[cfg(feature = "variable-fonts")]
        {
            let mut advance = self.tables.vmtx?.advance(glyph_id)? as f32;

            if self.is_variable() {
                // Ignore variation offset when `vvar` is not set.
                if let Some(vvar) = self.tables.vvar {
                    if let Some(offset) = vvar.advance_offset(glyph_id, self.coords()) {
                        // We can't use `round()` in `no_std`, so this is the next best thing.
                        advance += offset + 0.5;
                    }
                } else if let Some(points) = self.glyph_phantom_points(glyph_id) {
                    // We can't use `round()` in `no_std`, so this is the next best thing.
                    advance += points.bottom.y + 0.5
                }
            }

            u16::try_num_from(advance)
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            self.tables.vmtx?.advance(glyph_id)
        }
    }

    /// Returns glyph's horizontal side bearing.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn glyph_hor_side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        #[cfg(feature = "variable-fonts")]
        {
            let mut bearing = self.tables.hmtx?.side_bearing(glyph_id)? as f32;

            if self.is_variable() {
                // Ignore variation offset when `hvar` is not set.
                if let Some(hvar) = self.tables.hvar {
                    if let Some(offset) = hvar.left_side_bearing_offset(glyph_id, self.coords()) {
                        // We can't use `round()` in `no_std`, so this is the next best thing.
                        bearing += offset + 0.5;
                    }
                }
            }

            i16::try_num_from(bearing)
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            self.tables.hmtx?.side_bearing(glyph_id)
        }
    }

    /// Returns glyph's vertical side bearing.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn glyph_ver_side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        #[cfg(feature = "variable-fonts")]
        {
            let mut bearing = self.tables.vmtx?.side_bearing(glyph_id)? as f32;

            if self.is_variable() {
                // Ignore variation offset when `vvar` is not set.
                if let Some(vvar) = self.tables.vvar {
                    if let Some(offset) = vvar.top_side_bearing_offset(glyph_id, self.coords()) {
                        // We can't use `round()` in `no_std`, so this is the next best thing.
                        bearing += offset + 0.5;
                    }
                }
            }

            i16::try_num_from(bearing)
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            self.tables.vmtx?.side_bearing(glyph_id)
        }
    }

    /// Returns glyph's vertical origin according to
    /// [Vertical Origin Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg).
    ///
    /// This method is affected by variation axes.
    pub fn glyph_y_origin(&self, glyph_id: GlyphId) -> Option<i16> {
        #[cfg(feature = "variable-fonts")]
        {
            let mut origin = self.tables.vorg.map(|vorg| vorg.glyph_y_origin(glyph_id))? as f32;

            if self.is_variable() {
                // Ignore variation offset when `vvar` is not set.
                if let Some(vvar) = self.tables.vvar {
                    if let Some(offset) = vvar.vertical_origin_offset(glyph_id, self.coords()) {
                        // We can't use `round()` in `no_std`, so this is the next best thing.
                        origin += offset + 0.5;
                    }
                }
            }

            i16::try_num_from(origin)
        }

        #[cfg(not(feature = "variable-fonts"))]
        {
            self.tables.vorg.map(|vorg| vorg.glyph_y_origin(glyph_id))
        }
    }

    /// Returns glyph's name.
    ///
    /// Uses the `post` and `CFF` tables as sources.
    ///
    /// Returns `None` when no name is associated with a `glyph`.
    #[cfg(feature = "glyph-names")]
    #[inline]
    pub fn glyph_name(&self, glyph_id: GlyphId) -> Option<&str> {
        if let Some(name) = self.tables.post.and_then(|post| post.glyph_name(glyph_id)) {
            return Some(name);
        }

        if let Some(name) = self
            .tables
            .cff
            .as_ref()
            .and_then(|cff1| cff1.glyph_name(glyph_id))
        {
            return Some(name);
        }

        None
    }

    /// Outlines a glyph and returns its tight bounding box.
    ///
    /// **Warning**: since `ttf-parser` is a pull parser,
    /// `OutlineBuilder` will emit segments even when outline is partially malformed.
    /// You must check `outline_glyph()` result before using
    /// `OutlineBuilder`'s output.
    ///
    /// `gvar`, `glyf`, `CFF` and `CFF2` tables are supported.
    /// And they will be accesses in this specific order.
    ///
    /// This method is affected by variation axes.
    ///
    /// Returns `None` when glyph has no outline or on error.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fmt::Write;
    /// use ttf_parser;
    ///
    /// struct Builder(String);
    ///
    /// impl ttf_parser::OutlineBuilder for Builder {
    ///     fn move_to(&mut self, x: f32, y: f32) {
    ///         write!(&mut self.0, "M {} {} ", x, y).unwrap();
    ///     }
    ///
    ///     fn line_to(&mut self, x: f32, y: f32) {
    ///         write!(&mut self.0, "L {} {} ", x, y).unwrap();
    ///     }
    ///
    ///     fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    ///         write!(&mut self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    ///     }
    ///
    ///     fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    ///         write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    ///     }
    ///
    ///     fn close(&mut self) {
    ///         write!(&mut self.0, "Z ").unwrap();
    ///     }
    /// }
    ///
    /// let data = std::fs::read("tests/fonts/demo.ttf").unwrap();
    /// let face = ttf_parser::Face::parse(&data, 0).unwrap();
    /// let mut builder = Builder(String::new());
    /// let bbox = face.outline_glyph(ttf_parser::GlyphId(1), &mut builder).unwrap();
    /// assert_eq!(builder.0, "M 173 267 L 369 267 L 270 587 L 173 267 Z M 6 0 L 224 656 \
    ///                        L 320 656 L 541 0 L 452 0 L 390 200 L 151 200 L 85 0 L 6 0 Z ");
    /// assert_eq!(bbox, ttf_parser::Rect { x_min: 6, y_min: 0, x_max: 541, y_max: 656 });
    /// ```
    #[inline]
    pub fn outline_glyph(
        &self,
        glyph_id: GlyphId,
        builder: &mut dyn OutlineBuilder,
    ) -> Option<Rect> {
        #[cfg(feature = "variable-fonts")]
        {
            if let Some(ref gvar) = self.tables.gvar {
                return gvar.outline(self.tables.glyf?, self.coords(), glyph_id, builder);
            }
        }

        if let Some(table) = self.tables.glyf {
            return table.outline(glyph_id, builder);
        }

        if let Some(ref cff) = self.tables.cff {
            return cff.outline(glyph_id, builder).ok();
        }

        #[cfg(feature = "variable-fonts")]
        {
            if let Some(ref cff2) = self.tables.cff2 {
                return cff2.outline(self.coords(), glyph_id, builder).ok();
            }
        }

        None
    }

    /// Returns a tight glyph bounding box.
    ///
    /// This is just a shorthand for `outline_glyph()` since only the `glyf` table stores
    /// a bounding box. We ignore `glyf` table bboxes because they can be malformed.
    /// In case of CFF and variable fonts we have to actually outline
    /// a glyph to find it's bounding box.
    ///
    /// When a glyph is defined by a raster or a vector image,
    /// that can be obtained via `glyph_image()`,
    /// the bounding box must be calculated manually and this method will return `None`.
    ///
    /// Note: the returned bbox is not validated in any way. A font file can have a glyph bbox
    /// set to zero/negative width and/or height and this is perfectly ok.
    /// For calculated bboxes, zero width and/or height is also perfectly fine.
    ///
    /// This method is affected by variation axes.
    #[inline]
    pub fn glyph_bounding_box(&self, glyph_id: GlyphId) -> Option<Rect> {
        self.outline_glyph(glyph_id, &mut DummyOutline)
    }

    /// Returns a bounding box that large enough to enclose any glyph from the face.
    #[inline]
    pub fn global_bounding_box(&self) -> Rect {
        self.tables.head.global_bbox
    }

    /// Returns a reference to a glyph's raster image.
    ///
    /// A font can define a glyph using a raster or a vector image instead of a simple outline.
    /// Which is primarily used for emojis. This method should be used to access raster images.
    ///
    /// `pixels_per_em` allows selecting a preferred image size. The chosen size will
    /// be closer to an upper one. So when font has 64px and 96px images and `pixels_per_em`
    /// is set to 72, 96px image will be returned.
    /// To get the largest image simply use `std::u16::MAX`.
    ///
    /// Note that this method will return an encoded image. It should be decoded
    /// by the caller. We don't validate or preprocess it in any way.
    ///
    /// Also, a font can contain both: images and outlines. So when this method returns `None`
    /// you should also try `outline_glyph()` afterwards.
    ///
    /// There are multiple ways an image can be stored in a TrueType font
    /// and this method supports most of them.
    /// This includes `sbix`, `bloc` + `bdat`, `EBLC` + `EBDT`, `CBLC` + `CBDT`.
    /// And font's tables will be accesses in this specific order.
    #[inline]
    pub fn glyph_raster_image(
        &self,
        glyph_id: GlyphId,
        pixels_per_em: u16,
    ) -> Option<RasterGlyphImage> {
        if let Some(table) = self.tables.sbix {
            if let Some(strike) = table.best_strike(pixels_per_em) {
                return strike.get(glyph_id);
            }
        }
        if let Some(bdat) = self.tables.bdat {
            return bdat.get(glyph_id, pixels_per_em);
        }

        if let Some(ebdt) = self.tables.ebdt {
            return ebdt.get(glyph_id, pixels_per_em);
        }

        if let Some(cbdt) = self.tables.cbdt {
            return cbdt.get(glyph_id, pixels_per_em);
        }

        None
    }

    /// Returns a reference to a glyph's SVG image.
    ///
    /// A font can define a glyph using a raster or a vector image instead of a simple outline.
    /// Which is primarily used for emojis. This method should be used to access SVG images.
    ///
    /// Note that this method will return just an SVG data. It should be rendered
    /// or even decompressed (in case of SVGZ) by the caller.
    /// We don't validate or preprocess it in any way.
    ///
    /// Also, a font can contain both: images and outlines. So when this method returns `None`
    /// you should also try `outline_glyph()` afterwards.
    #[inline]
    pub fn glyph_svg_image(&self, glyph_id: GlyphId) -> Option<svg::SvgDocument<'a>> {
        self.tables.svg.and_then(|svg| svg.documents.find(glyph_id))
    }

    /// Returns `true` if the glyph can be colored/painted using the `COLR`+`CPAL` tables.
    ///
    /// See [`paint_color_glyph`](Face::paint_color_glyph) for details.
    pub fn is_color_glyph(&self, glyph_id: GlyphId) -> bool {
        self.tables()
            .colr
            .map(|colr| colr.contains(glyph_id))
            .unwrap_or(false)
    }

    /// Returns the number of palettes stored in the `COLR`+`CPAL` tables.
    ///
    /// See [`paint_color_glyph`](Face::paint_color_glyph) for details.
    pub fn color_palettes(&self) -> Option<core::num::NonZeroU16> {
        Some(self.tables().colr?.palettes.palettes())
    }

    /// Paints a color glyph from the `COLR` table.
    ///
    /// A font can have multiple palettes, which you can check via
    /// [`color_palettes`](Face::color_palettes).
    /// If unsure, just pass 0 to the `palette` argument, which is the default.
    ///
    /// A font can define a glyph using layers of colored shapes instead of a
    /// simple outline. Which is primarily used for emojis. This method should
    /// be used to access glyphs defined in the `COLR` table.
    ///
    /// Also, a font can contain both: a layered definition and outlines. So
    /// when this method returns `None` you should also try
    /// [`outline_glyph`](Face::outline_glyph) afterwards.
    ///
    /// Returns `None` if the glyph has no `COLR` definition or if the glyph
    /// definition is malformed.
    ///
    /// See `examples/font2svg.rs` for usage examples.
    #[inline]
    pub fn paint_color_glyph(
        &self,
        glyph_id: GlyphId,
        palette: u16,
        foreground_color: RgbaColor,
        painter: &mut dyn colr::Painter<'a>,
    ) -> Option<()> {
        self.tables.colr?.paint(
            glyph_id,
            palette,
            painter,
            #[cfg(feature = "variable-fonts")]
            self.coords(),
            foreground_color,
        )
    }

    /// Returns an iterator over variation axes.
    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub fn variation_axes(&self) -> LazyArray16<'a, VariationAxis> {
        self.tables.fvar.map(|fvar| fvar.axes).unwrap_or_default()
    }

    /// Sets a variation axis coordinate.
    ///
    /// This is one of the two only mutable methods in the library.
    /// We can simplify the API a lot by storing the variable coordinates
    /// in the face object itself.
    ///
    /// Since coordinates are stored on the stack, we allow only 64 of them.
    ///
    /// Returns `None` when face is not variable or doesn't have such axis.
    #[cfg(feature = "variable-fonts")]
    pub fn set_variation(&mut self, axis: Tag, value: f32) -> Option<()> {
        if !self.is_variable() {
            return None;
        }

        if usize::from(self.variation_axes().len()) >= MAX_VAR_COORDS {
            return None;
        }

        for (i, var_axis) in self.variation_axes().into_iter().enumerate() {
            if var_axis.tag == axis {
                self.coordinates.data[i] = var_axis.normalized_value(value);

                if let Some(avar) = self.tables.avar {
                    let _ = avar.map_coordinate(self.coordinates.as_mut_slice(), i);
                }
            }
        }

        Some(())
    }

    /// Returns the current normalized variation coordinates.
    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub fn variation_coordinates(&self) -> &[NormalizedCoordinate] {
        self.coordinates.as_slice()
    }

    /// Checks that face has non-default variation coordinates.
    #[cfg(feature = "variable-fonts")]
    #[inline]
    pub fn has_non_default_variation_coordinates(&self) -> bool {
        self.coordinates.as_slice().iter().any(|c| c.0 != 0)
    }

    /// Parses glyph's phantom points.
    ///
    /// Available only for variable fonts with the `gvar` table.
    #[cfg(feature = "variable-fonts")]
    pub fn glyph_phantom_points(&self, glyph_id: GlyphId) -> Option<PhantomPoints> {
        let glyf = self.tables.glyf?;
        let gvar = self.tables.gvar?;
        gvar.phantom_points(glyf, self.coords(), glyph_id)
    }

    #[cfg(feature = "variable-fonts")]
    #[inline]
    fn metrics_var_offset(&self, tag: Tag) -> f32 {
        self.tables
            .mvar
            .and_then(|table| table.metric_offset(tag, self.coords()))
            .unwrap_or(0.0)
    }

    #[inline]
    fn apply_metrics_variation(&self, tag: Tag, mut value: i16) -> i16 {
        self.apply_metrics_variation_to(tag, &mut value);
        value
    }

    #[cfg(feature = "variable-fonts")]
    #[inline]
    fn apply_metrics_variation_to(&self, tag: Tag, value: &mut i16) {
        if self.is_variable() {
            let v = f32::from(*value) + self.metrics_var_offset(tag);
            // TODO: Should probably round it, but f32::round is not available in core.
            if let Some(v) = i16::try_num_from(v) {
                *value = v;
            }
        }
    }

    #[cfg(not(feature = "variable-fonts"))]
    #[inline]
    fn apply_metrics_variation_to(&self, _: Tag, _: &mut i16) {}

    #[cfg(feature = "variable-fonts")]
    #[inline]
    fn coords(&self) -> &[NormalizedCoordinate] {
        self.coordinates.as_slice()
    }
}

impl core::fmt::Debug for Face<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Face()")
    }
}

/// Returns the number of fonts stored in a TrueType font collection.
///
/// Returns `None` if a provided data is not a TrueType font collection.
#[inline]
pub fn fonts_in_collection(data: &[u8]) -> Option<u32> {
    let mut s = Stream::new(data);
    if s.read::<Magic>()? != Magic::FontCollection {
        return None;
    }

    s.skip::<u32>(); // version
    s.read::<u32>()
}
