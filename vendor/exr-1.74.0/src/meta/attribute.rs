
//! Contains all meta data attributes.
//! Each layer can have any number of [`Attribute`]s, including custom attributes.

use smallvec::SmallVec;


/// Contains one of all possible attributes.
/// Includes a variant for custom attributes.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {

    /// Channel meta data.
    ChannelList(ChannelList),

    /// Color space definition.
    Chromaticities(Chromaticities),

    /// Compression method of this layer.
    Compression(Compression),

    /// This image is an environment map.
    EnvironmentMap(EnvironmentMap),

    /// Film roll information.
    KeyCode(KeyCode),

    /// Order of the bocks in the file.
    LineOrder(LineOrder),

    /// A 3x3 matrix of floats.
    Matrix3x3(Matrix3x3),

    /// A 4x4 matrix of floats.
    Matrix4x4(Matrix4x4),

    /// 8-bit rgba Preview of the image.
    Preview(Preview),

    /// An integer dividend and divisor.
    Rational(Rational),

    /// Deep or flat and tiled or scan line.
    BlockType(BlockType),

    /// List of texts.
    TextVector(Vec<Text>),

    /// How to tile up the image.
    TileDescription(TileDescription),

    /// Timepoint and more.
    TimeCode(TimeCode),

    /// A string of byte-chars.
    Text(Text),

    /// 64-bit float
    F64(f64),

    /// 32-bit float
    F32(f32),

    /// 32-bit signed integer
    I32(i32),

    /// 2D integer rectangle.
    IntegerBounds(IntegerBounds),

    /// 2D float rectangle.
    FloatRect(FloatRect),

    /// 2D integer vector.
    IntVec2(Vec2<i32>),

    /// 2D float vector.
    FloatVec2(Vec2<f32>),

    /// 3D integer vector.
    IntVec3((i32, i32, i32)),

    /// 3D float vector.
    FloatVec3((f32, f32, f32)),

    /// An explicitly untyped attribute for binary application data.
    /// Also contains the type name of this value.
    /// The format of the byte contents is explicitly unspecified.
    /// Used for custom application data.
    Bytes {

        /// An application-specific type hint of the byte contents.
        type_hint: Text,

        /// The contents of this byte array are completely unspecified
        /// and should be treated as untrusted data.
        bytes: SmallVec<[u8; 16]>
    },

    /// A custom attribute.
    /// Contains the type name of this value.
    Custom {

        /// The name of the type this attribute is an instance of.
        kind: Text,

        /// The value, stored in little-endian byte order, of the value.
        /// Use the `exr::io::Data` trait to extract binary values from this vector.
        bytes: SmallVec<[u8; 16]>
    },
}

/// A byte array with each byte being a char.
/// This is not UTF and it must be constructed from a standard string.
// TODO is this ascii? use a rust ascii crate?
#[derive(Clone, PartialEq, Ord, PartialOrd, Default)] // hash implemented manually
pub struct Text {
    bytes: TextBytes,
}

/// Contains time information for this frame within a sequence.
/// Also defined methods to compile this information into a
/// `TV60`, `TV50` or `Film24` bit sequence, packed into `u32`.
///
/// Satisfies the [SMPTE standard 12M-1999](https://en.wikipedia.org/wiki/SMPTE_timecode).
/// For more in-depth information, see [philrees.co.uk/timecode](http://www.philrees.co.uk/articles/timecode.htm).
#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct TimeCode {

    /// Hours 0 - 23 are valid.
    pub hours: u8,

    /// Minutes 0 - 59 are valid.
    pub minutes: u8,

    /// Seconds 0 - 59 are valid.
    pub seconds: u8,

    /// Frame Indices 0 - 29 are valid.
    pub frame: u8,

    /// Whether this is a drop frame.
    pub drop_frame: bool,

    /// Whether this is a color frame.
    pub color_frame: bool,

    /// Field Phase.
    pub field_phase: bool,

    /// Flags for `TimeCode.binary_groups`.
    pub binary_group_flags: [bool; 3],

    /// The user-defined control codes.
    /// Every entry in this array can use at most 3 bits.
    /// This results in a maximum value of 15, including 0, for each `u8`.
    pub binary_groups: [u8; 8]
}

/// layer type, specifies block type and deepness.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum BlockType {

    /// Corresponds to the string value `scanlineimage`.
    ScanLine,

    /// Corresponds to the string value `tiledimage`.
    Tile,

    /// Corresponds to the string value `deepscanline`.
    DeepScanLine,

    /// Corresponds to the string value `deeptile`.
    DeepTile,
}

/// The string literals used to represent a `BlockType` in a file.
pub mod block_type_strings {

    /// Type attribute text value of flat scan lines
    pub const SCAN_LINE: &'static [u8] = b"scanlineimage";

    /// Type attribute text value of flat tiles
    pub const TILE: &'static [u8] = b"tiledimage";

    /// Type attribute text value of deep scan lines
    pub const DEEP_SCAN_LINE: &'static [u8] = b"deepscanline";

    /// Type attribute text value of deep tiles
    pub const DEEP_TILE: &'static [u8] = b"deeptile";
}


pub use crate::compression::Compression;

/// The integer rectangle describing where an layer is placed on the infinite 2D global space.
pub type DataWindow = IntegerBounds;

/// The integer rectangle limiting which part of the infinite 2D global space should be displayed.
pub type DisplayWindow = IntegerBounds;

/// An integer dividend and divisor, together forming a ratio.
pub type Rational = (i32, u32);

/// A float matrix with four rows and four columns.
pub type Matrix4x4 = [f32; 4*4];

/// A float matrix with three rows and three columns.
pub type Matrix3x3 = [f32; 3*3];

/// A rectangular section anywhere in 2D integer space.
/// Valid from minimum coordinate (including) `-1,073,741,822`
/// to maximum coordinate (including) `1,073,741,822`, the value of (`i32::MAX/2 -1`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash)]
pub struct IntegerBounds {

    /// The top left corner of this rectangle.
    /// The `Box2I32` includes this pixel if the size is not zero.
    pub position: Vec2<i32>,

    /// How many pixels to include in this `Box2I32`.
    /// Extends to the right and downwards.
    /// Does not include the actual boundary, just like `Vec::len()`.
    pub size: Vec2<usize>,
}

/// A rectangular section anywhere in 2D float space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatRect {

    /// The top left corner location of the rectangle (inclusive)
    pub min: Vec2<f32>,

    /// The bottom right corner location of the rectangle (inclusive)
    pub max: Vec2<f32>
}

/// A List of channels. Channels must be sorted alphabetically.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChannelList {

    /// The channels in this list.
    pub list: SmallVec<[ChannelDescription; 5]>,

    /// The number of bytes that one pixel in this image needs.
    // FIXME this needs to account for subsampling anywhere?
    pub bytes_per_pixel: usize, // FIXME only makes sense for flat images!

    /// The sample type of all channels, if all channels have the same type.
    pub uniform_sample_type: Option<SampleType>,
}

/// A single channel in an layer.
/// Does not contain the actual pixel data,
/// but instead merely describes it.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChannelDescription {

    /// One of "R", "G", or "B" most of the time.
    pub name: Text,

    /// U32, F16 or F32.
    pub sample_type: SampleType,

    /// This attribute only tells lossy compression methods
    /// whether this value should be quantized exponentially or linearly.
    ///
    /// Should be `false` for red, green, or blue channels.
    /// Should be `true` for hue, chroma, saturation, or alpha channels.
    pub quantize_linearly: bool,

    /// How many of the samples are skipped compared to the other channels in this layer.
    ///
    /// Can be used for chroma subsampling for manual lossy data compression.
    /// Values other than 1 are allowed only in flat, scan-line based images.
    /// If an image is deep or tiled, x and y sampling rates for all of its channels must be 1.
    pub sampling: Vec2<usize>,
}

/// The type of samples in this channel.
#[derive(Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub enum SampleType {

    /// This channel contains 32-bit unsigned int values.
    U32,

    /// This channel contains 16-bit float values.
    F16,

    /// This channel contains 32-bit float values.
    F32,
}

/// The color space of the pixels.
///
/// If a file doesn't have a chromaticities attribute, display software
/// should assume that the file's primaries and the white point match `Rec. ITU-R BT.709-3`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chromaticities {

    /// "Red" location on the CIE XY chromaticity diagram.
    pub red: Vec2<f32>,

    /// "Green" location on the CIE XY chromaticity diagram.
    pub green: Vec2<f32>,

    /// "Blue" location on the CIE XY chromaticity diagram.
    pub blue: Vec2<f32>,

    /// "White" location on the CIE XY chromaticity diagram.
    pub white: Vec2<f32>
}

/// If this attribute is present, it describes
/// how this texture should be projected onto an environment.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EnvironmentMap {

    /// This image is an environment map projected like a world map.
    LatitudeLongitude,

    /// This image contains the six sides of a cube.
    Cube,
}

/// Uniquely identifies a motion picture film frame.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct KeyCode {

    /// Identifies a film manufacturer.
    pub film_manufacturer_code: i32,

    /// Identifies a film type.
    pub film_type: i32,

    /// Specifies the film roll prefix.
    pub film_roll_prefix: i32,

    /// Specifies the film count.
    pub count: i32,

    /// Specifies the perforation offset.
    pub perforation_offset: i32,

    /// Specifies the perforation count of each single frame.
    pub perforations_per_frame: i32,

    /// Specifies the perforation count of each single film.
    pub perforations_per_count: i32,
}

/// In what order the `Block`s of pixel data appear in a file.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LineOrder {

    /// The blocks in the file are ordered in descending rows from left to right.
    /// When compressing in parallel, this option requires potentially large amounts of memory.
    /// In that case, use `LineOrder::Unspecified` for best performance.
    Increasing,

    /// The blocks in the file are ordered in ascending rows from right to left.
    /// When compressing in parallel, this option requires potentially large amounts of memory.
    /// In that case, use `LineOrder::Unspecified` for best performance.
    Decreasing,

    /// The blocks are not ordered in a specific way inside the file.
    /// In multi-core file writing, this option offers the best performance.
    Unspecified,
}

/// A small `rgba` image of `i8` values that approximates the real exr image.
// TODO is this linear?
#[derive(Clone, Eq, PartialEq)]
pub struct Preview {

    /// The dimensions of the preview image.
    pub size: Vec2<usize>,

    /// An array with a length of 4 × width × height.
    /// The pixels are stored in `LineOrder::Increasing`.
    /// Each pixel consists of the four `u8` values red, green, blue, alpha.
    pub pixel_data: Vec<i8>,
}

/// Describes how the layer is divided into tiles.
/// Specifies the size of each tile in the image
/// and whether this image contains multiple resolution levels.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TileDescription {

    /// The size of each tile.
    /// Stays the same number of pixels across all levels.
    pub tile_size: Vec2<usize>,

    /// Whether to also store smaller versions of the image.
    pub level_mode: LevelMode,

    /// Whether to round up or down when calculating Mip/Rip levels.
    pub rounding_mode: RoundingMode,
}

/// Whether to also store increasingly smaller versions of the original image.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LevelMode {

    /// Only a single level.
    Singular,

    /// Levels with a similar aspect ratio.
    MipMap,

    /// Levels with all possible aspect ratios.
    RipMap,
}


/// The raw bytes that make up a string in an exr file.
/// Each `u8` is a single char.
// will mostly be "R", "G", "B" or "deepscanlineimage"
pub type TextBytes = SmallVec<[u8; 24]>;

/// A byte slice, interpreted as text
pub type TextSlice = [u8];


use crate::io::*;
use crate::meta::{sequence_end};
use crate::error::*;
use crate::math::{RoundingMode, Vec2};
use half::f16;
use std::convert::{TryFrom};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use bit_field::BitField;


fn invalid_type() -> Error {
    Error::invalid("attribute type mismatch")
}


impl Text {

    /// Create a `Text` from an `str` reference.
    /// Returns `None` if this string contains unsupported chars.
    pub fn new_or_none(string: impl AsRef<str>) -> Option<Self> {
        let vec : Option<TextBytes> = string.as_ref().chars()
            .map(|character| u8::try_from(character as u64).ok())
            .collect();

        vec.map(Self::from_bytes_unchecked)
    }

    /// Create a `Text` from an `str` reference.
    /// Panics if this string contains unsupported chars.
    pub fn new_or_panic(string: impl AsRef<str>) -> Self {
        Self::new_or_none(string).expect("exr::Text contains unsupported characters")
    }

    /// Create a `Text` from a slice of bytes,
    /// without checking any of the bytes.
    pub fn from_slice_unchecked(text: &TextSlice) -> Self {
        Self::from_bytes_unchecked(SmallVec::from_slice(text))
    }

    /// Create a `Text` from the specified bytes object,
    /// without checking any of the bytes.
    pub fn from_bytes_unchecked(bytes: TextBytes) -> Self {
        Text { bytes }
    }

    /// The internal ASCII bytes this text is made of.
    pub fn as_slice(&self) -> &TextSlice {
        self.bytes.as_slice()
    }

    /// Check whether this string is valid, adjusting `long_names` if required.
    /// If `long_names` is not provided, text length will be entirely unchecked.
    pub fn validate(&self, null_terminated: bool, long_names: Option<&mut bool>) -> UnitResult {
        Self::validate_bytes(self.as_slice(), null_terminated, long_names)
    }

    /// Check whether some bytes are valid, adjusting `long_names` if required.
    /// If `long_names` is not provided, text length will be entirely unchecked.
    pub fn validate_bytes(text: &TextSlice, null_terminated: bool, long_names: Option<&mut bool>) -> UnitResult {
        if null_terminated && text.is_empty() {
            return Err(Error::invalid("text must not be empty"));
        }

        if let Some(long) = long_names {
            if text.len() >= 256 { return Err(Error::invalid("text must not be longer than 255")); }
            if text.len() >= 32 { *long = true; }
        }

        Ok(())
    }

    /// The byte count this string would occupy if it were encoded as a null-terminated string.
    pub fn null_terminated_byte_size(&self) -> usize {
        self.bytes.len() + sequence_end::byte_size()
    }

    /// The byte count this string would occupy if it were encoded as a size-prefixed string.
    pub fn i32_sized_byte_size(&self) -> usize {
        self.bytes.len() + i32::BYTE_SIZE
    }

    /// The byte count this string would occupy if it were encoded as a size-prefixed string.
    pub fn u32_sized_byte_size(&self) -> usize {
        self.bytes.len() + u32::BYTE_SIZE
    }

    /// Write the length of a string and then the contents with that length.
    pub fn write_i32_sized_le<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert!(self.validate( false, None).is_ok(), "text size bug");
        i32::write_le(usize_to_i32(self.bytes.len(), "text length")?, write)?;
        Self::write_unsized_bytes(self.bytes.as_slice(), write)
    }

    /// Write the length of a string and then the contents with that length.
    pub fn write_u32_sized_le<W: Write>(&self, write: &mut W) -> UnitResult {
        debug_assert!(self.validate( false, None).is_ok(), "text size bug");
        u32::write_le(usize_to_u32(self.bytes.len(), "text length")?, write)?;
        Self::write_unsized_bytes(self.bytes.as_slice(), write)
    }

    /// Without validation, write this instance to the byte stream.
    fn write_unsized_bytes<W: Write>(bytes: &[u8], write: &mut W) -> UnitResult {
        u8::write_slice_le(write, bytes)?;
        Ok(())
    }

    /// Read the length of a string and then the contents with that length.
    pub fn read_i32_sized_le<R: Read>(read: &mut R, max_size: usize) -> Result<Self> {
        let size = i32_to_usize(i32::read_le(read)?, "vector size")?;
        Ok(Text::from_bytes_unchecked(SmallVec::from_vec(u8::read_vec_le(read, size, 1024, Some(max_size), "text attribute length")?)))
    }

    /// Read the length of a string and then the contents with that length.
    pub fn read_u32_sized_le<R: Read>(read: &mut R, max_size: usize) -> Result<Self> {
        let size = u32_to_usize(u32::read_le(read)?, "text length")?;
        Ok(Text::from_bytes_unchecked(SmallVec::from_vec(u8::read_vec_le(read, size, 1024, Some(max_size), "text attribute length")?)))
    }

    /// Read the contents with that length.
    pub fn read_sized<R: Read>(read: &mut R, size: usize) -> Result<Self> {
        const SMALL_SIZE: usize  = 24;

        // for small strings, read into small vec without heap allocation
        if size <= SMALL_SIZE {
            let mut buffer = [0_u8; SMALL_SIZE];
            let data = &mut buffer[..size];

            read.read_exact(data)?;
            Ok(Text::from_bytes_unchecked(SmallVec::from_slice(data)))
        }

        // for large strings, read a dynamic vec of arbitrary size
        else {
            Ok(Text::from_bytes_unchecked(SmallVec::from_vec(u8::read_vec_le(read, size, 1024, None, "text attribute length")?)))
        }
    }

    /// Write the string contents and a null-terminator.
    pub fn write_null_terminated<W: Write>(&self, write: &mut W) -> UnitResult {
        Self::write_null_terminated_bytes(self.as_slice(), write)
    }

    /// Write the string contents and a null-terminator.
    fn write_null_terminated_bytes<W: Write>(bytes: &[u8], write: &mut W) -> UnitResult {
        debug_assert!(!bytes.is_empty(), "text is empty bug"); // required to avoid mixup with "sequece_end"

        Text::write_unsized_bytes(bytes, write)?;
        sequence_end::write(write)?;
        Ok(())
    }

    /// Read a string until the null-terminator is found. Then skips the null-terminator.
    pub fn read_null_terminated<R: Read>(read: &mut R, max_len: usize) -> Result<Self> {
        let mut bytes = smallvec![ u8::read_le(read)? ]; // null-terminated strings are always at least 1 byte

        loop {
            match u8::read_le(read)? {
                0 => break,
                non_terminator => bytes.push(non_terminator),
            }

            if bytes.len() > max_len {
                return Err(Error::invalid("text too long"))
            }
        }

        Ok(Text { bytes })
    }

    /// Allows any text length since it is only used for attribute values,
    /// but not attribute names, attribute type names, or channel names.
    fn read_vec_of_i32_sized_texts_le(
        read: &mut PeekRead<impl Read>,
        total_byte_size: usize
    ) -> Result<Vec<Text>>
    {
        let mut result = Vec::with_capacity(2);

        // length of the text-vector can be inferred from attribute size
        let mut processed_bytes = 0;

        while processed_bytes < total_byte_size {
            let text = Text::read_i32_sized_le(read, total_byte_size)?;
            processed_bytes += ::std::mem::size_of::<i32>(); // size i32 of the text
            processed_bytes += text.bytes.len();
            result.push(text);
        }

        // the expected byte size did not match the actual text byte size
        if processed_bytes != total_byte_size {
            return Err(Error::invalid("text array byte size"))
        }

        Ok(result)
    }

    /// Allows any text length since it is only used for attribute values,
    /// but not attribute names, attribute type names, or channel names.
    fn write_vec_of_i32_sized_texts_le<W: Write>(write: &mut W, texts: &[Text]) -> UnitResult {
        // length of the text-vector can be inferred from attribute size
        for text in texts {
            text.write_i32_sized_le(write)?;
        }

        Ok(())
    }

    /// The underlying bytes that represent this text.
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Iterate over the individual chars in this text, similar to `String::chars()`.
    /// Does not do any heap-allocation but borrows from this instance instead.
    pub fn chars(&self) -> impl '_ + Iterator<Item = char> {
        self.bytes.iter().map(|&byte| byte as char)
    }

    /// Compare this `exr::Text` with a plain `&str`.
    pub fn eq(&self, string: &str) -> bool {
        string.chars().eq(self.chars())
    }

    /// Compare this `exr::Text` with a plain `&str` ignoring capitalization.
    pub fn eq_case_insensitive(&self, string: &str) -> bool {
        // this is technically not working for a "turkish i", but those cannot be encoded in exr files anyways
        let self_chars = self.chars().map(|char| char.to_ascii_lowercase());
        let string_chars = string.chars().flat_map(|ch| ch.to_lowercase());

        string_chars.eq(self_chars)
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.eq(other)
    }
}

impl PartialEq<Text> for str {
    fn eq(&self, other: &Text) -> bool {
        other.eq(self)
    }
}

impl Eq for Text {}

impl Borrow<TextSlice> for Text {
    fn borrow(&self) -> &TextSlice {
        self.as_slice()
    }
}

// forwarding implementation. guarantees `text.borrow().hash() == text.hash()` (required for Borrow)
impl Hash for Text {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state)
    }
}

impl Into<String> for Text {
    fn into(self) -> String {
        self.to_string()
    }
}

impl<'s> From<&'s str> for Text {

    /// Panics if the string contains an unsupported character
    fn from(str: &'s str) -> Self {
        Self::new_or_panic(str)
    }
}


/* TODO (currently conflicts with From<&str>)
impl<'s> TryFrom<&'s str> for Text {
    type Error = String;

    fn try_from(value: &'s str) -> std::result::Result<Self, Self::Error> {
        Text::new_or_none(value)
            .ok_or_else(|| format!(
                "exr::Text does not support all characters in the string `{}`",
                value
            ))
    }
}*/


impl ::std::fmt::Debug for Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "exr::Text(\"{}\")", self)
    }
}

// automatically implements to_string for us
impl ::std::fmt::Display for Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        use std::fmt::Write;

        for &byte in self.bytes.iter() {
            f.write_char(byte as char)?;
        }

        Ok(())
    }
}


impl ChannelList {

    /// Does not validate channel order.
    pub fn new(channels: SmallVec<[ChannelDescription; 5]>) -> Self {
        let uniform_sample_type = {
            if let Some(first) = channels.first() {
                let has_uniform_types = channels.iter().skip(1)
                    .all(|chan| chan.sample_type == first.sample_type);

                if has_uniform_types { Some(first.sample_type) } else { None }
            }
            else { None }
        };

        ChannelList {
            bytes_per_pixel: channels.iter().map(|channel| channel.sample_type.bytes_per_sample()).sum(),
            list: channels, uniform_sample_type,
        }
    }

    /// Iterate over the channels, and adds to each channel the byte offset of the channels sample type.
    /// Assumes the internal channel list is properly sorted.
    pub fn channels_with_byte_offset(&self) -> impl Iterator<Item=(usize, &ChannelDescription)> {
        self.list.iter().scan(0, |byte_position, channel|{
            let previous_position = *byte_position;
            *byte_position += channel.sample_type.bytes_per_sample();
            Some((previous_position, channel))
        })
    }

    /// Return the index of the channel with the exact name, case sensitive, or none.
    /// Potentially uses less than linear time.
    pub fn find_index_of_channel(&self, exact_name: &Text) -> Option<usize> {
        self.list.binary_search_by_key(&exact_name.bytes(), |chan| chan.name.bytes()).ok()
    }

    // TODO use this in compression methods
    /*pub fn pixel_section_indices(&self, bounds: IntegerBounds) -> impl '_ + Iterator<Item=(&Channel, usize, usize)> {
        (bounds.position.y() .. bounds.end().y()).flat_map(|y| {
            self.list
                .filter(|channel| mod_p(y, usize_to_i32(channel.sampling.1)) == 0)
                .flat_map(|channel|{
                    (bounds.position.x() .. bounds.end().x())
                        .filter(|x| mod_p(*x, usize_to_i32(channel.sampling.0)) == 0)
                        .map(|x| (channel, x, y))
                })
        })
    }*/
}

impl BlockType {

    /// The corresponding attribute type name literal
    const TYPE_NAME: &'static [u8] = type_names::TEXT;

    /// Return a `BlockType` object from the specified attribute text value.
    pub fn parse(text: Text) -> Result<Self> {
        match text.as_slice() {
            block_type_strings::SCAN_LINE => Ok(BlockType::ScanLine),
            block_type_strings::TILE => Ok(BlockType::Tile),

            block_type_strings::DEEP_SCAN_LINE => Ok(BlockType::DeepScanLine),
            block_type_strings::DEEP_TILE => Ok(BlockType::DeepTile),

            _ => Err(Error::invalid("block type attribute value")),
        }
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write(&self, write: &mut impl Write) -> UnitResult {
        u8::write_slice_le(write, self.to_text_bytes())?;
        Ok(())
    }

    /// Returns the raw attribute text value this type is represented by in a file.
    pub fn to_text_bytes(&self) -> &[u8] {
        match self {
            BlockType::ScanLine => block_type_strings::SCAN_LINE,
            BlockType::Tile => block_type_strings::TILE,
            BlockType::DeepScanLine => block_type_strings::DEEP_SCAN_LINE,
            BlockType::DeepTile => block_type_strings::DEEP_TILE,
        }
    }

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size(&self) -> usize {
        self.to_text_bytes().len()
    }
}


impl IntegerBounds {

    /// Create a box with no size located at (0,0).
    pub fn zero() -> Self {
        Self::from_dimensions(Vec2(0, 0))
    }

    /// Create a box with a size starting at zero.
    pub fn from_dimensions(size: impl Into<Vec2<usize>>) -> Self {
        Self::new(Vec2(0,0), size)
    }

    /// Create a box with a size and an origin point.
    pub fn new(start: impl Into<Vec2<i32>>, size: impl Into<Vec2<usize>>) -> Self {
        Self { position: start.into(), size: size.into() }
    }

    /// Returns the top-right coordinate of the rectangle.
    /// The row and column described by this vector are not included in the rectangle,
    /// just like `Vec::len()`.
    pub fn end(self) -> Vec2<i32> {
        self.position + self.size.to_i32() // larger than max int32 is panic
    }

    /// Returns the maximum coordinate that a value in this rectangle may have.
    pub fn max(self) -> Vec2<i32> {
        self.end() - Vec2(1,1)
    }

    /// Validate this instance.
    pub fn validate(&self, max_size: Option<Vec2<usize>>) -> UnitResult {
        if let Some(max_size) = max_size {
            if self.size.width() > max_size.width() || self.size.height() > max_size.height()  {
                return Err(Error::invalid("window attribute dimension value"));
            }
        }

        let min_i64 = Vec2(self.position.x() as i64, self.position.y() as i64);

        let max_i64 = Vec2(
            self.position.x() as i64 + self.size.width() as i64,
            self.position.y() as i64 + self.size.height() as i64,
        );

        Self::validate_min_max_u64(min_i64, max_i64)
    }

    fn validate_min_max_u64(min: Vec2<i64>, max: Vec2<i64>) -> UnitResult {
        let max_box_size_as_i64 = (i32::MAX / 2) as i64; // as defined in the original c++ library

        if     max.x() >=  max_box_size_as_i64
            || max.y() >=  max_box_size_as_i64
            || min.x() <= -max_box_size_as_i64
            || min.y() <= -max_box_size_as_i64
        {
            return Err(Error::invalid("window size exceeding integer maximum"));
        }

        Ok(())
    }

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        4 * i32::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        let Vec2(x_min, y_min) = self.position;
        let Vec2(x_max, y_max) = self.max();

        x_min.write_le(write)?;
        y_min.write_le(write)?;
        x_max.write_le(write)?;
        y_max.write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let x_min = i32::read_le(read)?;
        let y_min = i32::read_le(read)?;
        let x_max = i32::read_le(read)?;
        let y_max = i32::read_le(read)?;

        let min = Vec2(x_min.min(x_max), y_min.min(y_max));
        let max  = Vec2(x_min.max(x_max), y_min.max(y_max));

        // prevent addition overflow
        Self::validate_min_max_u64(
            Vec2(min.x() as i64, min.y() as i64),
            Vec2(max.x() as i64, max.y() as i64),
        )?;

        // add one to max because the max inclusive, but the size is not
        let size = Vec2(max.x() + 1 - min.x(), max.y() + 1 - min.y());
        let size = size.to_usize("box coordinates")?;

        Ok(IntegerBounds { position: min, size })
    }

    /// Create a new rectangle which is offset by the specified origin.
    pub fn with_origin(self, origin: Vec2<i32>) -> Self { // TODO rename to "move" or "translate"?
        IntegerBounds { position: self.position + origin, .. self }
    }

    /// Returns whether the specified rectangle is equal to or inside this rectangle.
    pub fn contains(self, subset: Self) -> bool {
           subset.position.x() >= self.position.x()
        && subset.position.y() >= self.position.y()
        && subset.end().x() <= self.end().x()
        && subset.end().y() <= self.end().y()
    }
}


impl FloatRect {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        4 * f32::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        self.min.x().write_le(write)?;
        self.min.y().write_le(write)?;
        self.max.x().write_le(write)?;
        self.max.y().write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let x_min = f32::read_le(read)?;
        let y_min = f32::read_le(read)?;
        let x_max = f32::read_le(read)?;
        let y_max = f32::read_le(read)?;

        Ok(FloatRect {
            min: Vec2(x_min, y_min),
            max: Vec2(x_max, y_max)
        })
    }
}

impl SampleType {

    /// How many bytes a single sample takes up.
    pub fn bytes_per_sample(&self) -> usize {
        match self {
            SampleType::F16 => f16::BYTE_SIZE,
            SampleType::F32 => f32::BYTE_SIZE,
            SampleType::U32 => u32::BYTE_SIZE,
        }
    }

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        i32::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        match *self {
            SampleType::U32 => 0_i32,
            SampleType::F16 => 1_i32,
            SampleType::F32 => 2_i32,
        }.write_le(write)?;

        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        // there's definitely going to be more than 255 different pixel types in the future
        Ok(match i32::read_le(read)? {
            0 => SampleType::U32,
            1 => SampleType::F16,
            2 => SampleType::F32,
            _ => return Err(Error::invalid("pixel type attribute value")),
        })
    }
}

impl ChannelDescription {
    /// Choose whether to compress samples linearly or not, based on the channel name.
    /// Luminance-based channels will be compressed differently than linear data such as alpha.
    pub fn guess_quantization_linearity(name: &Text) -> bool {
        !(
            name.eq_case_insensitive("R") || name.eq_case_insensitive("G") ||
                name.eq_case_insensitive("B") || name.eq_case_insensitive("L") ||
                name.eq_case_insensitive("Y") || name.eq_case_insensitive("X") ||
                name.eq_case_insensitive("Z")
        )
    }

    /// Create a new channel with the specified properties and a sampling rate of (1,1).
    /// Automatically chooses the linearity for compression based on the channel name.
    pub fn named(name: impl Into<Text>, sample_type: SampleType) -> Self {
        let name = name.into();
        let linearity = Self::guess_quantization_linearity(&name);
        Self::new(name, sample_type, linearity)
    }

    /*pub fn from_name<T: Into<Sample> + Default>(name: impl Into<Text>) -> Self {
        Self::named(name, T::default().into().sample_type())
    }*/

    /// Create a new channel with the specified properties and a sampling rate of (1,1).
    pub fn new(name: impl Into<Text>, sample_type: SampleType, quantize_linearly: bool) -> Self {
        Self { name: name.into(), sample_type, quantize_linearly, sampling: Vec2(1, 1) }
    }

    /// The count of pixels this channel contains, respecting subsampling.
    // FIXME this must be used everywhere
    pub fn subsampled_pixels(&self, dimensions: Vec2<usize>) -> usize {
        self.subsampled_resolution(dimensions).area()
    }

    /// The resolution pf this channel, respecting subsampling.
    pub fn subsampled_resolution(&self, dimensions: Vec2<usize>) -> Vec2<usize> {
        dimensions / self.sampling
    }

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size(&self) -> usize {
        self.name.null_terminated_byte_size()
            + SampleType::byte_size()
            + 1 // is_linear
            + 3 // reserved bytes
            + 2 * u32::BYTE_SIZE // sampling x, y
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        Text::write_null_terminated(&self.name, write)?;
        self.sample_type.write(write)?;

        match self.quantize_linearly {
            false => 0_u8,
            true  => 1_u8,
        }.write_le(write)?;

        i8::write_slice_le(write, &[0_i8, 0_i8, 0_i8])?;
        i32::write_le(usize_to_i32(self.sampling.x(), "text length")?, write)?;
        i32::write_le(usize_to_i32(self.sampling.y(), "text length")?, write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let name = Text::read_null_terminated(read, 256)?;
        let sample_type = SampleType::read(read)?;

        let is_linear = match u8::read_le(read)? {
            1 => true,
            0 => false,
            _ => return Err(Error::invalid("channel linearity attribute value")),
        };

        let mut reserved = [0_i8; 3];
        i8::read_slice_le(read, &mut reserved)?;

        let x_sampling = i32_to_usize(i32::read_le(read)?, "x channel sampling")?;
        let y_sampling = i32_to_usize(i32::read_le(read)?, "y channel sampling")?;

        Ok(ChannelDescription {
            name, sample_type,
            quantize_linearly: is_linear,
            sampling: Vec2(x_sampling, y_sampling),
        })
    }

    /// Validate this instance.
    pub fn validate(&self, allow_sampling: bool, data_window: IntegerBounds, strict: bool) -> UnitResult {
        self.name.validate(true, None)?; // TODO spec says this does not affect `requirements.long_names` but is that true?

        if self.sampling.x() == 0 || self.sampling.y() == 0 {
            return Err(Error::invalid("zero sampling factor"));
        }

        if strict && !allow_sampling && self.sampling != Vec2(1,1) {
            return Err(Error::invalid("subsampling is only allowed in flat scan line images"));
        }

        if data_window.position.x() % self.sampling.x() as i32 != 0 || data_window.position.y() % self.sampling.y() as i32 != 0 {
            return Err(Error::invalid("channel sampling factor not dividing data window position"));
        }

        if data_window.size.x() % self.sampling.x() != 0 || data_window.size.y() % self.sampling.y() != 0 {
            return Err(Error::invalid("channel sampling factor not dividing data window size"));
        }

        if self.sampling != Vec2(1,1) {
            // TODO this must only be implemented in the crate::image module and child modules,
            //      should not be too difficult

            return Err(Error::unsupported("channel subsampling not supported yet"));
        }

        Ok(())
    }
}

impl ChannelList {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size(&self) -> usize {
        self.list.iter().map(ChannelDescription::byte_size).sum::<usize>() + sequence_end::byte_size()
    }

    /// Without validation, write this instance to the byte stream.
    /// Assumes channels are sorted alphabetically and all values are validated.
    pub fn write(&self, write: &mut impl Write) -> UnitResult {
        for channel in &self.list {
            channel.write(write)?;
        }

        sequence_end::write(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read(read: &mut PeekRead<impl Read>) -> Result<Self> {
        let mut channels = SmallVec::new();
        while !sequence_end::has_come(read)? {
            channels.push(ChannelDescription::read(read)?);
        }

        Ok(ChannelList::new(channels))
    }

    /// Check if channels are valid and sorted.
    pub fn validate(&self, allow_sampling: bool, data_window: IntegerBounds, strict: bool) -> UnitResult {
        let mut iter = self.list.iter().map(|chan| chan.validate(allow_sampling, data_window, strict).map(|_| &chan.name));
        let mut previous = iter.next().ok_or(Error::invalid("at least one channel is required"))??;

        for result in iter {
            let value = result?;
            if strict && previous == value { return Err(Error::invalid("channel names are not unique")); }
            else if previous > value { return Err(Error::invalid("channel names are not sorted alphabetically")); }
            else { previous = value; }
        }

        Ok(())
    }
}

fn u8_to_decimal32(binary: u8) -> u32 {
    let units = binary as u32 % 10;
    let tens = (binary as u32 / 10) % 10;
    units | (tens << 4)
}

// assumes value fits into u8
fn u8_from_decimal32(coded: u32) -> u8 {
    ((coded & 0x0f) + 10 * ((coded >> 4) & 0x0f)) as u8
}

// https://github.com/AcademySoftwareFoundation/openexr/blob/master/src/lib/OpenEXR/ImfTimeCode.cpp
impl TimeCode {

    /// Number of bytes this would consume in an exr file.
    pub const BYTE_SIZE: usize = 2 * u32::BYTE_SIZE;

    /// Returns an error if this time code is considered invalid.
    pub fn validate(&self, strict: bool) -> UnitResult {
        if strict {
            if self.frame > 29 { Err(Error::invalid("time code frame larger than 29")) }
            else if self.seconds > 59 { Err(Error::invalid("time code seconds larger than 59")) }
            else if self.minutes > 59 { Err(Error::invalid("time code minutes larger than 59")) }
            else if self.hours > 23 { Err(Error::invalid("time code hours larger than 23")) }
            else if self.binary_groups.iter().any(|&group| group > 15) {
                Err(Error::invalid("time code binary group value too large for 3 bits"))
            }
            else { Ok(()) }
        }
        else { Ok(()) }
    }


    /// Pack the SMPTE time code into a u32 value, according to TV60 packing.
    /// This is the encoding which is used within a binary exr file.
    pub fn pack_time_as_tv60_u32(&self) -> Result<u32> {
        // validate strictly to prevent set_bit panic! below
        self.validate(true)?;

        Ok(*0_u32
            .set_bits(0..6, u8_to_decimal32(self.frame))
            .set_bit(6, self.drop_frame)
            .set_bit(7, self.color_frame)
            .set_bits(8..15, u8_to_decimal32(self.seconds))
            .set_bit(15, self.field_phase)
            .set_bits(16..23, u8_to_decimal32(self.minutes))
            .set_bit(23, self.binary_group_flags[0])
            .set_bits(24..30, u8_to_decimal32(self.hours))
            .set_bit(30, self.binary_group_flags[1])
            .set_bit(31, self.binary_group_flags[2])
        )
    }

    /// Unpack a time code from one TV60 encoded u32 value and the encoded user data.
    /// This is the encoding which is used within a binary exr file.
    pub fn from_tv60_time(tv60_time: u32, user_data: u32) -> Self {
        Self {
            frame: u8_from_decimal32(tv60_time.get_bits(0..6)), // cast cannot fail, as these are less than 8 bits
            drop_frame: tv60_time.get_bit(6),
            color_frame: tv60_time.get_bit(7),
            seconds: u8_from_decimal32(tv60_time.get_bits(8..15)), // cast cannot fail, as these are less than 8 bits
            field_phase: tv60_time.get_bit(15),
            minutes: u8_from_decimal32(tv60_time.get_bits(16..23)), // cast cannot fail, as these are less than 8 bits
            hours: u8_from_decimal32(tv60_time.get_bits(24..30)), // cast cannot fail, as these are less than 8 bits
            binary_group_flags: [
                tv60_time.get_bit(23),
                tv60_time.get_bit(30),
                tv60_time.get_bit(31),
            ],

            binary_groups: Self::unpack_user_data_from_u32(user_data)
        }
    }

    /// Pack the SMPTE time code into a u32 value, according to TV50 packing.
    /// This encoding does not support the `drop_frame` flag, it will be lost.
    pub fn pack_time_as_tv50_u32(&self) -> Result<u32> {
        Ok(*self.pack_time_as_tv60_u32()?

            // swap some fields by replacing some bits in the packed u32
            .set_bit(6, false)
            .set_bit(15, self.binary_group_flags[0])
            .set_bit(30, self.binary_group_flags[1])
            .set_bit(23, self.binary_group_flags[2])
            .set_bit(31, self.field_phase)
        )
    }

    /// Unpack a time code from one TV50 encoded u32 value and the encoded user data.
    /// This encoding does not support the `drop_frame` flag, it will always be false.
    pub fn from_tv50_time(tv50_time: u32, user_data: u32) -> Self {
        Self {
            drop_frame: false, // do not use bit [6]

            // swap some fields:
            field_phase: tv50_time.get_bit(31),
            binary_group_flags: [
                tv50_time.get_bit(15),
                tv50_time.get_bit(30),
                tv50_time.get_bit(23),
            ],

            .. Self::from_tv60_time(tv50_time, user_data)
        }
    }


    /// Pack the SMPTE time code into a u32 value, according to FILM24 packing.
    /// This encoding does not support the `drop_frame` and `color_frame` flags, they will be lost.
    pub fn pack_time_as_film24_u32(&self) -> Result<u32> {
        Ok(*self.pack_time_as_tv60_u32()?
            .set_bit(6, false)
            .set_bit(7, false)
        )
    }

    /// Unpack a time code from one TV60 encoded u32 value and the encoded user data.
    /// This encoding does not support the `drop_frame` and `color_frame` flags, they will always be `false`.
    pub fn from_film24_time(film24_time: u32, user_data: u32) -> Self {
        Self {
            drop_frame: false, // bit [6]
            color_frame: false, // bit [7]
            .. Self::from_tv60_time(film24_time, user_data)
        }
    }


    // in rust, group index starts at zero, not at one.
    fn user_data_bit_indices(group_index: usize) -> std::ops::Range<usize> {
        let min_bit = 4 * group_index;
        min_bit .. min_bit + 4 // +4, not +3, as `Range` is exclusive
    }

    /// Pack the user data `u8` array into one u32.
    /// User data values are clamped to the valid range (maximum value is 4).
    pub fn pack_user_data_as_u32(&self) -> u32 {
        let packed = self.binary_groups.iter().enumerate().fold(0_u32, |mut packed, (group_index, group_value)|
            *packed.set_bits(Self::user_data_bit_indices(group_index), *group_value.min(&15) as u32)
        );

        debug_assert_eq!(Self::unpack_user_data_from_u32(packed), self.binary_groups, "round trip user data encoding");
        packed
    }

    // Unpack the encoded u32 user data to an array of bytes, each byte having a value from 0 to 4.
    fn unpack_user_data_from_u32(user_data: u32) -> [u8; 8] {
        (0..8).map(|group_index| user_data.get_bits(Self::user_data_bit_indices(group_index)) as u8)
            .collect::<SmallVec<[u8;8]>>().into_inner().expect("array index bug")
    }


    /// Write this time code to the byte stream, encoded as TV60 integers.
    /// Returns an `Error::Invalid` if the fields are out of the allowed range.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        self.pack_time_as_tv60_u32()?.write_le(write)?; // will validate
        self.pack_user_data_as_u32().write_le(write)?;
        Ok(())
    }

    /// Read the time code, without validating, extracting from TV60 integers.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let time_and_flags = u32::read_le(read)?;
        let user_data = u32::read_le(read)?;
        Ok(Self::from_tv60_time(time_and_flags, user_data))
    }
}

impl Chromaticities {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        8 * f32::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        self.red.x().write_le(write)?;
        self.red.y().write_le(write)?;

        self.green.x().write_le(write)?;
        self.green.y().write_le(write)?;

        self.blue.x().write_le(write)?;
        self.blue.y().write_le(write)?;

        self.white.x().write_le(write)?;
        self.white.y().write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        Ok(Chromaticities {
            red: Vec2(f32::read_le(read)?, f32::read_le(read)?),
            green: Vec2(f32::read_le(read)?, f32::read_le(read)?),
            blue: Vec2(f32::read_le(read)?, f32::read_le(read)?),
            white: Vec2(f32::read_le(read)?, f32::read_le(read)?),
        })
    }
}

impl Compression {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize { u8::BYTE_SIZE }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(self, write: &mut W) -> UnitResult {
        use self::Compression::*;
        match self {
            Uncompressed => 0_u8,
            RLE => 1_u8,
            ZIP1 => 2_u8,
            ZIP16 => 3_u8,
            PIZ => 4_u8,
            PXR24 => 5_u8,
            B44 => 6_u8,
            B44A => 7_u8,
            DWAA(_) => 8_u8,
            DWAB(_) => 9_u8,
            HTJ2K256 => 10_u8,
            HTJ2K32 => 11_u8,
        }.write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        use self::Compression::*;
        Ok(match u8::read_le(read)? {
            0 => Uncompressed,
            1 => RLE,
            2 => ZIP1,
            3 => ZIP16,
            4 => PIZ,
            5 => PXR24,
            6 => B44,
            7 => B44A,
            8 => DWAA(None),
            9 => DWAB(None),
            10 => HTJ2K256,
            11 => HTJ2K32,
            _ => return Err(Error::unsupported("unknown compression method")),
        })
    }
}

impl EnvironmentMap {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        u8::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(self, write: &mut W) -> UnitResult {
        use self::EnvironmentMap::*;
        match self {
            LatitudeLongitude => 0_u8,
            Cube => 1_u8
        }.write_le(write)?;

        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        use self::EnvironmentMap::*;
        Ok(match u8::read_le(read)? {
            0 => LatitudeLongitude,
            1 => Cube,
            _ => return Err(Error::invalid("environment map attribute value")),
        })
    }
}

impl KeyCode {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        6 * i32::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        self.film_manufacturer_code.write_le(write)?;
        self.film_type.write_le(write)?;
        self.film_roll_prefix.write_le(write)?;
        self.count.write_le(write)?;
        self.perforation_offset.write_le(write)?;
        self.perforations_per_count.write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        Ok(KeyCode {
            film_manufacturer_code: i32::read_le(read)?,
            film_type: i32::read_le(read)?,
            film_roll_prefix: i32::read_le(read)?,
            count: i32::read_le(read)?,
            perforation_offset: i32::read_le(read)?,
            perforations_per_frame: i32::read_le(read)?,
            perforations_per_count: i32::read_le(read)?,
        })
    }
}

impl LineOrder {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        u8::BYTE_SIZE
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(self, write: &mut W) -> UnitResult {
        use self::LineOrder::*;
        match self {
            Increasing => 0_u8,
            Decreasing => 1_u8,
            Unspecified => 2_u8,
        }.write_le(write)?;

        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        use self::LineOrder::*;
        Ok(match u8::read_le(read)? {
            0 => Increasing,
            1 => Decreasing,
            2 => Unspecified,
            _ => return Err(Error::invalid("line order attribute value")),
        })
    }
}




impl Preview {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size(&self) -> usize {
        2 * u32::BYTE_SIZE + self.pixel_data.len()
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        u32::write_le(self.size.width() as u32, write)?;
        u32::write_le(self.size.height() as u32, write)?;

        i8::write_slice_le(write, &self.pixel_data)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let width = u32::read_le(read)? as usize;
        let height = u32::read_le(read)? as usize;

        if let Some(pixel_count) = width.checked_mul(height) {
            // Multiply by the number of bytes per pixel.
            if let Some(byte_count) = pixel_count.checked_mul(4) {
                let pixel_data = i8::read_vec_le(
                    read,
                    byte_count,
                    1024 * 1024 * 4,
                    None,
                    "preview attribute pixel count",
                )?;

                let preview = Preview {
                    size: Vec2(width, height),
                    pixel_data,
                };

                return Ok(preview);
            }
        }

        return Err(Error::invalid(
                format!("Overflow while calculating preview image Attribute size \
                (width: {}, height: {}).",
                width,
                height)));
    }

    /// Validate this instance.
    pub fn validate(&self, strict: bool) -> UnitResult {
        if strict && (self.size.area() * 4 != self.pixel_data.len()) {
            return Err(Error::invalid("preview dimensions do not match content length"))
        }

        Ok(())
    }
}

impl ::std::fmt::Debug for Preview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Preview ({}x{} px)", self.size.width(), self.size.height())
    }
}

impl TileDescription {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size() -> usize {
        2 * u32::BYTE_SIZE + 1 // size x,y + (level mode + rounding mode)
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        u32::write_le(self.tile_size.width() as u32, write)?;
        u32::write_le(self.tile_size.height() as u32, write)?;

        let level_mode = match self.level_mode {
            LevelMode::Singular => 0_u8,
            LevelMode::MipMap => 1_u8,
            LevelMode::RipMap => 2_u8,
        };

        let rounding_mode = match self.rounding_mode {
            RoundingMode::Down => 0_u8,
            RoundingMode::Up => 1_u8,
        };

        let mode: u8 = level_mode + (rounding_mode * 16);
        mode.write_le(write)?;
        Ok(())
    }

    /// Read the value without validating.
    pub fn read<R: Read>(read: &mut R) -> Result<Self> {
        let x_size = u32::read_le(read)? as usize;
        let y_size = u32::read_le(read)? as usize;

        let mode = u8::read_le(read)?;

        // wow you really saved that one byte here
        // mode = level_mode + (rounding_mode * 16)
        let level_mode = mode & 0b00001111; // wow that works
        let rounding_mode = mode >> 4; // wow that works

        let level_mode = match level_mode {
            0 => LevelMode::Singular,
            1 => LevelMode::MipMap,
            2 => LevelMode::RipMap,
            _ => return Err(Error::invalid("tile description level mode")),
        };

        let rounding_mode = match rounding_mode {
            0 => RoundingMode::Down,
            1 => RoundingMode::Up,
            _ => return Err(Error::invalid("tile description rounding mode")),
        };

        Ok(TileDescription { tile_size: Vec2(x_size, y_size), level_mode, rounding_mode, })
    }

    /// Validate this instance.
    pub fn validate(&self) -> UnitResult {
        let max = i32::MAX as i64 / 2;

        if self.tile_size.width() == 0 || self.tile_size.height() == 0
            || self.tile_size.width() as i64 >= max || self.tile_size.height() as i64 >= max
        {
            return Err(Error::invalid("tile size"))
        }

        Ok(())
    }
}


/// Number of bytes this attribute would consume in an exr file.
// TODO instead of pre calculating byte size, write to a tmp buffer whose length is inspected before actually writing?
pub fn byte_size(name: &Text, value: &AttributeValue) -> usize {
    name.null_terminated_byte_size()
        + value.kind_name().len() + sequence_end::byte_size()
        + i32::BYTE_SIZE // serialized byte size
        + value.byte_size()
}

/// Without validation, write this attribute to the byte stream.
pub fn write<W: Write>(name: &TextSlice, value: &AttributeValue, write: &mut W) -> UnitResult {
    Text::write_null_terminated_bytes(name, write)?;
    Text::write_null_terminated_bytes(value.kind_name(), write)?;
    i32::write_le(value.byte_size() as i32, write)?;
    value.write(write)
}

/// Read the attribute without validating. The result may be `Ok` even if this single attribute is invalid.
pub fn read(read: &mut PeekRead<impl Read>, max_size: usize) -> Result<(Text, Result<AttributeValue>)> {
    let name = Text::read_null_terminated(read, max_size)?;
    let kind = Text::read_null_terminated(read, max_size)?;
    let size = i32_to_usize(i32::read_le(read)?, "attribute size")?;
    let value = AttributeValue::read(read, kind, size)?;
    Ok((name, value))
}

/// Validate this attribute.
pub fn validate(name: &Text, value: &AttributeValue, long_names: &mut bool, allow_sampling: bool, data_window: IntegerBounds, strict: bool) -> UnitResult {
    name.validate(true, Some(long_names))?; // only name text has length restriction
    value.validate(allow_sampling, data_window, strict) // attribute value text length is never restricted
}


impl AttributeValue {

    /// Number of bytes this would consume in an exr file.
    pub fn byte_size(&self) -> usize {
        use self::AttributeValue::*;

        match *self {
            IntegerBounds(_) => self::IntegerBounds::byte_size(),
            FloatRect(_) => self::FloatRect::byte_size(),

            I32(_) => i32::BYTE_SIZE,
            F32(_) => f32::BYTE_SIZE,
            F64(_) => f64::BYTE_SIZE,

            Rational(_) => { i32::BYTE_SIZE + u32::BYTE_SIZE },
            TimeCode(_) => self::TimeCode::BYTE_SIZE,

            IntVec2(_) => { 2 * i32::BYTE_SIZE },
            FloatVec2(_) => { 2 * f32::BYTE_SIZE },
            IntVec3(_) => { 3 * i32::BYTE_SIZE },
            FloatVec3(_) => { 3 * f32::BYTE_SIZE },

            ChannelList(ref channels) => channels.byte_size(),
            Chromaticities(_) => self::Chromaticities::byte_size(),
            Compression(_) => self::Compression::byte_size(),
            EnvironmentMap(_) => self::EnvironmentMap::byte_size(),

            KeyCode(_) => self::KeyCode::byte_size(),
            LineOrder(_) => self::LineOrder::byte_size(),

            Matrix3x3(ref value) => value.len() * f32::BYTE_SIZE,
            Matrix4x4(ref value) => value.len() * f32::BYTE_SIZE,

            Preview(ref value) => value.byte_size(),

            // attribute value texts never have limited size.
            // also, don't serialize size, as it can be inferred from attribute size
            Text(ref value) => value.bytes.len(),

            TextVector(ref value) => value.iter().map(self::Text::i32_sized_byte_size).sum(),
            TileDescription(_) => self::TileDescription::byte_size(),
            Custom { ref bytes, .. } => bytes.len(),
            BlockType(ref kind) => kind.byte_size(),

            Bytes { ref bytes, ref type_hint } =>
                type_hint.u32_sized_byte_size() + bytes.len(),
        }
    }

    /// The exr name string of the type that an attribute can have.
    pub fn kind_name(&self) -> &TextSlice {
        use self::AttributeValue::*;
        use self::type_names as ty;

        match *self {
            IntegerBounds(_) =>  ty::I32BOX2,
            FloatRect(_) =>  ty::F32BOX2,
            I32(_) =>  ty::I32,
            F32(_) =>  ty::F32,
            F64(_) =>  ty::F64,
            Rational(_) => ty::RATIONAL,
            TimeCode(_) => ty::TIME_CODE,
            IntVec2(_) => ty::I32VEC2,
            FloatVec2(_) => ty::F32VEC2,
            IntVec3(_) => ty::I32VEC3,
            FloatVec3(_) => ty::F32VEC3,
            ChannelList(_) =>  ty::CHANNEL_LIST,
            Chromaticities(_) =>  ty::CHROMATICITIES,
            Compression(_) =>  ty::COMPRESSION,
            EnvironmentMap(_) =>  ty::ENVIRONMENT_MAP,
            KeyCode(_) =>  ty::KEY_CODE,
            LineOrder(_) =>  ty::LINE_ORDER,
            Matrix3x3(_) =>  ty::F32MATRIX3X3,
            Matrix4x4(_) =>  ty::F32MATRIX4X4,
            Preview(_) =>  ty::PREVIEW,
            Text(_) =>  ty::TEXT,
            TextVector(_) =>  ty::TEXT_VECTOR,
            TileDescription(_) =>  ty::TILES,
            BlockType(_) => super::BlockType::TYPE_NAME,
            Bytes{ .. } => ty::BYTES,
            Custom { ref kind, .. } => kind.as_slice(),
        }
    }

    /// Without validation, write this instance to the byte stream.
    pub fn write<W: Write>(&self, write: &mut W) -> UnitResult {
        use self::AttributeValue::*;
        match *self {
            IntegerBounds(value) => value.write(write)?,
            FloatRect(value) => value.write(write)?,

            I32(value) => value.write_le(write)?,
            F32(value) => value.write_le(write)?,
            F64(value) => value.write_le(write)?,

            Rational((a, b)) => { a.write_le(write)?; b.write_le(write)?; },
            TimeCode(codes) => codes.write(write)?,

            IntVec2(Vec2(x, y)) => { x.write_le(write)?; y.write_le(write)?; },
            FloatVec2(Vec2(x, y)) => { x.write_le(write)?; y.write_le(write)?; },
            IntVec3((x, y, z)) => { x.write_le(write)?; y.write_le(write)?; z.write_le(write)?; },
            FloatVec3((x, y, z)) => { x.write_le(write)?; y.write_le(write)?; z.write_le(write)?; },

            ChannelList(ref channels) => channels.write(write)?,
            Chromaticities(ref value) => value.write(write)?,
            Compression(value) => value.write(write)?,
            EnvironmentMap(value) => value.write(write)?,

            KeyCode(value) => value.write(write)?,
            LineOrder(value) => value.write(write)?,

            Matrix3x3(mut value) => f32::write_slice_le(write, &mut value)?,
            Matrix4x4(mut value) => f32::write_slice_le(write, &mut value)?,

            Preview(ref value) => value.write(write)?,

            // attribute value texts never have limited size.
            // also, don't serialize size, as it can be inferred from attribute size
            Text(ref value) => u8::write_slice_le(write, value.bytes.as_slice())?,

            TextVector(ref value) => self::Text::write_vec_of_i32_sized_texts_le(write, value)?,
            TileDescription(ref value) => value.write(write)?,
            BlockType(kind) => kind.write(write)?,

            Bytes { ref type_hint, ref bytes } => {
                type_hint.write_u32_sized_le(write)?; // no idea why this one is u32, everything else is usually i32...
                u8::write_slice_le(write, bytes.as_slice())?
            }

            Custom { ref bytes, .. } => u8::write_slice_le(write, &bytes)?, // write.write(&bytes).map(|_| ()),
        };

        Ok(())
    }

    /// Read the value without validating.
    /// Returns `Ok(Ok(attribute))` for valid attributes.
    /// Returns `Ok(Err(Error))` for malformed attributes from a valid byte source.
    /// Returns `Err(Error)` for invalid byte sources, for example for invalid files.
    pub fn read(read: &mut PeekRead<impl Read>, kind: Text, byte_size: usize) -> Result<Result<Self>> {
        use self::AttributeValue::*;
        use self::type_names as ty;

        // always read bytes as to leave the read position at the end of the attribute
        // even if the attribute contents fails to decode
        let mut attribute_bytes = SmallVec::<[u8; 64]>::new();
        u8::read_into_vec_le(read, &mut attribute_bytes, byte_size, 64, None, "attribute value size")?;
        // TODO: don't read into an array at all, just read directly from the reader and optionally seek afterwards?

        let parse_attribute = move || {
            let reader = &mut attribute_bytes.as_slice();

            Ok(match kind.bytes.as_slice() {
                ty::I32BOX2 => IntegerBounds(self::IntegerBounds::read(reader)?),
                ty::F32BOX2 => FloatRect(self::FloatRect::read(reader)?),

                ty::I32 => I32(i32::read_le(reader)?),
                ty::F32 => F32(f32::read_le(reader)?),
                ty::F64 => F64(f64::read_le(reader)?),

                ty::RATIONAL => Rational({
                    let a = i32::read_le(reader)?;
                    let b = u32::read_le(reader)?;
                    (a, b)
                }),

                ty::TIME_CODE => TimeCode(self::TimeCode::read(reader)?),

                ty::I32VEC2 => IntVec2({
                    let a = i32::read_le(reader)?;
                    let b = i32::read_le(reader)?;
                    Vec2(a, b)
                }),

                ty::F32VEC2 => FloatVec2({
                    let a = f32::read_le(reader)?;
                    let b = f32::read_le(reader)?;
                    Vec2(a, b)
                }),

                ty::I32VEC3 => IntVec3({
                    let a = i32::read_le(reader)?;
                    let b = i32::read_le(reader)?;
                    let c = i32::read_le(reader)?;
                    (a, b, c)
                }),

                ty::F32VEC3 => FloatVec3({
                    let a = f32::read_le(reader)?;
                    let b = f32::read_le(reader)?;
                    let c = f32::read_le(reader)?;
                    (a, b, c)
                }),

                ty::CHANNEL_LIST    => ChannelList(self::ChannelList::read(&mut PeekRead::new(attribute_bytes.as_slice()))?),
                ty::CHROMATICITIES  => Chromaticities(self::Chromaticities::read(reader)?),
                ty::COMPRESSION     => Compression(self::Compression::read(reader)?),
                ty::ENVIRONMENT_MAP => EnvironmentMap(self::EnvironmentMap::read(reader)?),

                ty::KEY_CODE   => KeyCode(self::KeyCode::read(reader)?),
                ty::LINE_ORDER => LineOrder(self::LineOrder::read(reader)?),

                ty::F32MATRIX3X3 => Matrix3x3({
                    let mut result = [0.0_f32; 9];
                    f32::read_slice_le(reader, &mut result)?;
                    result
                }),

                ty::F32MATRIX4X4 => Matrix4x4({
                    let mut result = [0.0_f32; 16];
                    f32::read_slice_le(reader, &mut result)?;
                    result
                }),

                ty::PREVIEW     => Preview(self::Preview::read(reader)?),
                ty::TEXT        => Text(self::Text::read_sized(reader, byte_size)?),

                // the number of strings can be inferred from the total attribute size
                ty::TEXT_VECTOR => TextVector(self::Text::read_vec_of_i32_sized_texts_le(
                    &mut PeekRead::new(attribute_bytes.as_slice()),
                    byte_size
                )?),

                ty::TILES => TileDescription(self::TileDescription::read(reader)?),

                ty::BYTES => {
                    // for some reason, they went for unsigned sizes, in this place only
                    let type_hint = self::Text::read_u32_sized_le(reader, reader.len())?;
                    let bytes = SmallVec::from(*reader);
                    Bytes { type_hint, bytes }
                }

                _ => Custom { kind: kind.clone(), bytes: SmallVec::from(*reader) }
            })
        };

        Ok(parse_attribute())
    }

    /// Validate this instance.
    pub fn validate(&self, allow_sampling: bool, data_window: IntegerBounds, strict: bool) -> UnitResult {
        use self::AttributeValue::*;

        match *self {
            ChannelList(ref channels) => channels.validate(allow_sampling, data_window, strict)?,
            TileDescription(ref value) => value.validate()?,
            Preview(ref value) => value.validate(strict)?,
            TimeCode(ref time_code) => time_code.validate(strict)?,

            TextVector(ref vec) => if strict && vec.is_empty() {
                return Err(Error::invalid("text vector may not be empty"))
            },

            _ => {}
        };

        Ok(())
    }


    /// Return `Ok(i32)` if this attribute is an i32.
    pub fn to_i32(&self) -> Result<i32> {
        match *self {
            AttributeValue::I32(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }

    /// Return `Ok(f32)` if this attribute is an f32.
    pub fn to_f32(&self) -> Result<f32> {
        match *self {
            AttributeValue::F32(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }

    /// Return `Ok(Text)` if this attribute is a text.
    pub fn into_text(self) -> Result<Text> {
        match self {
            AttributeValue::Text(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }

    /// Return `Ok(Text)` if this attribute is a text.
    pub fn to_text(&self) -> Result<&Text> {
        match self {
            AttributeValue::Text(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }

    /// Return `Ok(Chromaticities)` if this attribute is a chromaticities attribute.
    pub fn to_chromaticities(&self) -> Result<Chromaticities> {
        match *self {
            AttributeValue::Chromaticities(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }

    /// Return `Ok(TimeCode)` if this attribute is a time code.
    pub fn to_time_code(&self) -> Result<TimeCode> {
        match *self {
            AttributeValue::TimeCode(value) => Ok(value),
            _ => Err(invalid_type())
        }
    }
}



/// Contains string literals identifying the type of an attribute.
pub mod type_names {
    macro_rules! define_attribute_type_names {
        ( $($name: ident : $value: expr),* ) => {
            $(
                /// The byte-string name of this attribute type as it appears in an exr file.
                pub const $name: &'static [u8] = $value;
            )*
        };
    }

    define_attribute_type_names! {
        I32BOX2:        b"box2i",
        F32BOX2:        b"box2f",
        I32:            b"int",
        F32:            b"float",
        F64:            b"double",
        RATIONAL:       b"rational",
        TIME_CODE:      b"timecode",
        I32VEC2:        b"v2i",
        F32VEC2:        b"v2f",
        I32VEC3:        b"v3i",
        F32VEC3:        b"v3f",
        CHANNEL_LIST:   b"chlist",
        CHROMATICITIES: b"chromaticities",
        COMPRESSION:    b"compression",
        ENVIRONMENT_MAP:b"envmap",
        KEY_CODE:       b"keycode",
        LINE_ORDER:     b"lineOrder",
        F32MATRIX3X3:   b"m33f",
        F32MATRIX4X4:   b"m44f",
        PREVIEW:        b"preview",
        TEXT:           b"string",
        TEXT_VECTOR:    b"stringvector",
        TILES:          b"tiledesc",
        BYTES:          b"bytes"
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use ::std::io::Cursor;
    use rand::{random, thread_rng, Rng};

    #[test]
    fn text_ord() {
        for _ in 0..1024 {
            let text1 = Text::from_bytes_unchecked((0..4).map(|_| rand::random::<u8>()).collect());
            let text2 = Text::from_bytes_unchecked((0..4).map(|_| rand::random::<u8>()).collect());

            assert_eq!(text1.to_string().cmp(&text2.to_string()), text1.cmp(&text2), "in text {:?} vs {:?}", text1, text2);
        }
    }

    #[test]
    fn rounding_up(){
        let round_up = RoundingMode::Up;
        assert_eq!(round_up.divide(10, 10), 1, "divide equal");
        assert_eq!(round_up.divide(10, 2), 5, "divide even");
        assert_eq!(round_up.divide(10, 5), 2, "divide even");

        assert_eq!(round_up.divide(8, 5), 2, "round up");
        assert_eq!(round_up.divide(10, 3), 4, "round up");
        assert_eq!(round_up.divide(100, 50), 2, "divide even");
        assert_eq!(round_up.divide(100, 49), 3, "round up");
    }

    #[test]
    fn rounding_down(){
        let round_down = RoundingMode::Down;
        assert_eq!(round_down.divide(8, 5), 1, "round down");
        assert_eq!(round_down.divide(10, 3), 3, "round down");
        assert_eq!(round_down.divide(100, 50), 2, "divide even");
        assert_eq!(round_down.divide(100, 49), 2, "round down");
        assert_eq!(round_down.divide(100, 51), 1, "round down");
    }

    #[test]
    fn tile_description_write_read_roundtrip(){
        let tiles = [
            TileDescription {
                tile_size: Vec2(31, 7),
                level_mode: LevelMode::MipMap,
                rounding_mode: RoundingMode::Down,
            },

            TileDescription {
                tile_size: Vec2(0, 0),
                level_mode: LevelMode::Singular,
                rounding_mode: RoundingMode::Up,
            },

            TileDescription {
                tile_size: Vec2(4294967294, 4294967295),
                level_mode: LevelMode::RipMap,
                rounding_mode: RoundingMode::Down,
            },
        ];

        for tile in &tiles {
            let mut bytes = Vec::new();
            tile.write(&mut bytes).unwrap();

            let new_tile = TileDescription::read(&mut Cursor::new(bytes)).unwrap();
            assert_eq!(*tile, new_tile, "tile round trip");
        }
    }

    #[test]
    fn attribute_write_read_roundtrip_and_byte_size(){
        let attributes = [
            (
                Text::from("greeting"),
                AttributeValue::Text(Text::from("hello")),
            ),
            (
                Text::from("age"),
                AttributeValue::I32(923),
            ),
            (
                Text::from("leg count"),
                AttributeValue::F64(9.114939599234),
            ),
            (
                Text::from("rabbit area"),
                AttributeValue::FloatRect(FloatRect {
                    min: Vec2(23.4234, 345.23),
                    max: Vec2(68623.0, 3.12425926538),
                }),
            ),
            (
                Text::from("rabbit area int"),
                AttributeValue::IntegerBounds(IntegerBounds {
                    position: Vec2(23, 345),
                    size: Vec2(68623, 3),
                }),
            ),
            (
                Text::from("rabbit area int"),
                AttributeValue::IntegerBounds(IntegerBounds {
                    position: Vec2(-(i32::MAX / 2 - 1), -(i32::MAX / 2 - 1)),
                    size: Vec2(i32::MAX as usize - 2, i32::MAX as usize - 2),
                }),
            ),
            (
                Text::from("rabbit area int 2"),
                AttributeValue::IntegerBounds(IntegerBounds {
                    position: Vec2(0, 0),
                    size: Vec2(i32::MAX as usize / 2 - 1, i32::MAX as usize / 2 - 1),
                }),
            ),
            (
                Text::from("tests are difficult"),
                AttributeValue::TextVector(vec![
                    Text::from("sdoifjpsdv"),
                    Text::from("sdoifjpsdvxxxx"),
                    Text::from("sdoifjasd"),
                    Text::from("sdoifj"),
                    Text::from("sdoifjddddddddasdasd"),
                ]),
            ),
            (
                Text::from("what should we eat tonight"),
                AttributeValue::Preview(Preview {
                    size: Vec2(10, 30),
                    pixel_data: vec![31; 10 * 30 * 4],
                }),
            ),
            (
                Text::from("custom byte sequence: prime numbers single byte"),
                AttributeValue::Bytes{
                    type_hint: Text::from("byte-primes"),
                    bytes: smallvec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73],
                },
            ),
            (
                Text::from("leg count, again"),
                AttributeValue::ChannelList(ChannelList::new(smallvec![
                        ChannelDescription {
                            name: Text::from("Green"),
                            sample_type: SampleType::F16,
                            quantize_linearly: false,
                            sampling: Vec2(1,2)
                        },
                        ChannelDescription {
                            name: Text::from("Red"),
                            sample_type: SampleType::F32,
                            quantize_linearly: true,
                            sampling: Vec2(1,2)
                        },
                        ChannelDescription {
                            name: Text::from("Purple"),
                            sample_type: SampleType::U32,
                            quantize_linearly: false,
                            sampling: Vec2(0,0)
                        }
                    ],
                )),
            ),
        ];

        for (name, value) in &attributes {
            let mut bytes = Vec::new();
            super::write(name.as_slice(), value, &mut bytes).unwrap();
            assert_eq!(super::byte_size(name, value), bytes.len(), "attribute.byte_size() for {:?}", (name, value));

            let new_attribute = super::read(&mut PeekRead::new(Cursor::new(bytes)), 300).unwrap();
            assert_eq!((name.clone(), value.clone()), (new_attribute.0, new_attribute.1.unwrap()), "attribute round trip");
        }


        {
            let (name, value) = (
                Text::from("asdkaspfokpaosdkfpaokswdpoakpsfokaposdkf"),
                AttributeValue::I32(0),
            );

            let mut long_names = false;
            super::validate(&name, &value, &mut long_names, false, IntegerBounds::zero(), false).unwrap();
            assert!(long_names);
        }

        {
            let (name, value) = (
                Text::from("sdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfposdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfposdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfposdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfposdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfposdöksadöofkaspdolkpöasolfkcöalsod,kfcöaslodkcpöasolkfpo"),
                AttributeValue::I32(0),
            );

            super::validate(&name, &value, &mut false, false, IntegerBounds::zero(), false).expect_err("name length check failed");
        }
    }

    #[test]
    fn time_code_pack(){
        let mut rng = thread_rng();

        let codes = std::iter::repeat_with(|| TimeCode {
            hours: rng.gen_range(0 .. 24),
            minutes: rng.gen_range(0 .. 60),
            seconds: rng.gen_range(0 .. 60),
            frame: rng.gen_range(0 .. 29),
            drop_frame: random(),
            color_frame: random(),
            field_phase: random(),
            binary_group_flags: [random(),random(),random()],
            binary_groups: std::iter::repeat_with(|| rng.gen_range(0 .. 16)).take(8)
                .collect::<SmallVec<[u8;8]>>().into_inner().unwrap()
        });

        for code in codes.take(500) {
            code.validate(true).expect("invalid timecode test input");

            {   // through tv60 packing, roundtrip
                let packed_tv60 = code.pack_time_as_tv60_u32().expect("invalid timecode test input");
                let packed_user = code.pack_user_data_as_u32();
                assert_eq!(TimeCode::from_tv60_time(packed_tv60, packed_user), code);
            }

            {   // through bytes, roundtrip
                let mut bytes = Vec::<u8>::new();
                code.write(&mut bytes).unwrap();
                let decoded = TimeCode::read(&mut bytes.as_slice()).unwrap();
                assert_eq!(code, decoded);
            }

            {
                let tv50_code = TimeCode {
                    drop_frame: false, // apparently, tv50 does not support drop frame, so do not use this value
                   .. code
                };

                let packed_tv50 = code.pack_time_as_tv50_u32().expect("invalid timecode test input");
                let packed_user = code.pack_user_data_as_u32();
                assert_eq!(TimeCode::from_tv50_time(packed_tv50, packed_user), tv50_code);
            }

            {
                let film24_code = TimeCode {
                    // apparently, film24 does not support some flags, so do not use those values
                    color_frame: false,
                    drop_frame: false,
                   .. code
                };

                let packed_film24 = code.pack_time_as_film24_u32().expect("invalid timecode test input");
                let packed_user = code.pack_user_data_as_u32();
                assert_eq!(TimeCode::from_film24_time(packed_film24, packed_user), film24_code);
            }
        }
    }

}
