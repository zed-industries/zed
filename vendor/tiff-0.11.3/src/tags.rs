use crate::encoder::TiffValue;
use core::fmt;

macro_rules! tags {
    {
        // Permit arbitrary meta items, which include documentation.
        $( #[$enum_attr:meta] )*
        $vis:vis enum $name:ident($ty:tt) $(unknown(#[$unknown_meta:meta] $unknown_doc:ident))* {
            // Each of the `Name = Val,` permitting documentation.
            $($(#[$ident_attr:meta])* $tag:ident = $val:expr,)*
        }
    } => {
        $( #[$enum_attr] )*
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        #[non_exhaustive]
        #[repr($ty)]
        pub enum $name {
            $($(#[$ident_attr])* $tag = $val,)*
            $(
                #[$unknown_meta]
                Unknown($ty),
            )*
        }

        impl $name {
            #[inline(always)]
            const fn __from_inner_type(n: $ty) -> Result<Self, $ty> {
                match n {
                    $( $val => Ok($name::$tag), )*
                    n => Err(n),
                }
            }

            #[inline(always)]
            const fn __to_inner_type(&self) -> $ty {
                match *self {
                    $( $name::$tag => $val, )*
                    $( $name::Unknown($unknown_doc) => { $unknown_doc }, )*
                }
            }
        }

        tags!($name, $ty, $($unknown_doc)*);
    };
    // For u16 tags, provide direct inherent primitive conversion methods.
    ($name:tt, u16, $($unknown_doc:ident)*) => {
        impl $name {
            #[inline(always)]
            pub const fn from_u16(val: u16) -> Option<Self> {
                match Self::__from_inner_type(val) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }

            $(
            #[inline(always)]
            pub const fn from_u16_exhaustive($unknown_doc: u16) -> Self {
                match Self::__from_inner_type($unknown_doc) {
                    Ok(v) => v,
                    Err(_) => $name::Unknown($unknown_doc),
                }
            }
            )*

            #[inline(always)]
            pub const fn to_u16(&self) -> u16 {
                Self::__to_inner_type(self)
            }
        }
    };
    // For other tag types, do nothing for now. With concat_idents one could
    // provide inherent conversion methods for all types.
    ($name:tt, $ty:tt, $($unknown_doc:literal)*) => {};
}

// Note: These tags appear in the order they are mentioned in the TIFF reference
tags! {
/// TIFF tags
pub enum Tag(u16) unknown(
    /// A private or extension tag
    unknown
) {
    // Baseline tags:
    Artist = 315,
    // grayscale images PhotometricInterpretation 1 or 3
    BitsPerSample = 258,
    CellLength = 265, // TODO add support
    CellWidth = 264, // TODO add support
    // palette-color images (PhotometricInterpretation 3)
    ColorMap = 320, // TODO add support
    Compression = 259, // TODO add support for 2 and 32773
    DateTime = 306,
    ExtraSamples = 338, // TODO add support
    FillOrder = 266, // TODO add support
    FreeByteCounts = 289, // TODO add support
    FreeOffsets = 288, // TODO add support
    GrayResponseCurve = 291, // TODO add support
    GrayResponseUnit = 290, // TODO add support
    HostComputer = 316,
    ImageDescription = 270,
    ImageLength = 257,
    ImageWidth = 256,
    Make = 271,
    MaxSampleValue = 281, // TODO add support
    MinSampleValue = 280, // TODO add support
    Model = 272,
    NewSubfileType = 254, // TODO add support
    Orientation = 274, // TODO add support
    PhotometricInterpretation = 262,
    PlanarConfiguration = 284,
    ResolutionUnit = 296, // TODO add support
    RowsPerStrip = 278,
    SamplesPerPixel = 277,
    Software = 305,
    StripByteCounts = 279,
    StripOffsets = 273,
    SubfileType = 255, // TODO add support
    Threshholding = 263, // TODO add support
    XResolution = 282,
    YResolution = 283,
    // Advanced tags
    Predictor = 317,
    TileWidth = 322,
    TileLength = 323,
    TileOffsets = 324,
    TileByteCounts = 325,
    SubIfd = 330,
    // Data Sample Format
    SampleFormat = 339,
    SMinSampleValue = 340, // TODO add support
    SMaxSampleValue = 341, // TODO add support
    // JPEG
    JPEGTables = 347,
    // Subsampling
    #[doc(alias = "YCbCrSubsampling")]
    ChromaSubsampling = 530, // TODO add support
    #[doc(alias = "YCbCrPositioning")]
    ChromaPositioning = 531, // TODO add support
    // GeoTIFF
    ModelPixelScaleTag = 33550, // (SoftDesk)
    ModelTransformationTag = 34264, // (JPL Carto Group)
    ModelTiepointTag = 33922, // (Intergraph)
    // <https://web.archive.org/web/20131111073619/http://www.exif.org/Exif2-1.PDF>
    // *Do note its typo in the Decimal id*
    Copyright = 33_432,
    // <https://web.archive.org/web/20131111073619/http://www.exif.org/Exif2-1.PDF>
    ExifDirectory = 0x8769,
    // <https://web.archive.org/web/20131111073619/http://www.exif.org/Exif2-1.PDF>
    GpsDirectory = 0x8825,
    // <https://www.color.org/technotes/ICC-Technote-ProfileEmbedding.pdf>
    IccProfile = 34675,
    GeoKeyDirectoryTag = 34735, // (SPOT)
    GeoDoubleParamsTag = 34736, // (SPOT)
    GeoAsciiParamsTag = 34737, // (SPOT)
    ExifVersion = 0x9000,
    GdalNodata = 42113, // Contains areas with missing data
}
}

/// Identifies the offset of an IFD.
///
/// This is represented as a 64-bit integer but only BigTIFF can utilize the bits. It is encoded
/// as 32-bit unsigned value ([`Type::LONG`]) in regular TIFF files and as 64-bit unsigned value
/// ([`Type::IFD8`]) in BigTIFF files.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
// We could be using `NonZeroU64` here but I find that this complicates semantics. This type
// represents the integer value stored as a value in a tag. The semantics of treating `0` as an end
// marker are imposed by the IFD. (It's unclear if Pointer tags such as Exif would allow `0` but in
// practice it just returns garbage and the validity does not matter greatly to us).
pub struct IfdPointer(pub u64);

impl fmt::LowerHex for IfdPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl core::fmt::UpperHex for IfdPointer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

tags! {
/// The type of an IFD entry (a 2 byte field).
pub enum Type(u16) {
    /// 8-bit unsigned integer
    BYTE = 1,
    /// 8-bit byte that contains a 7-bit ASCII code; the last byte must be zero
    ASCII = 2,
    /// 16-bit unsigned integer
    SHORT = 3,
    /// 32-bit unsigned integer
    LONG = 4,
    /// Fraction stored as two 32-bit unsigned integers
    RATIONAL = 5,
    /// 8-bit signed integer
    SBYTE = 6,
    /// 8-bit byte that may contain anything, depending on the field
    UNDEFINED = 7,
    /// 16-bit signed integer
    SSHORT = 8,
    /// 32-bit signed integer
    SLONG = 9,
    /// Fraction stored as two 32-bit signed integers
    SRATIONAL = 10,
    /// 32-bit IEEE floating point
    FLOAT = 11,
    /// 64-bit IEEE floating point
    DOUBLE = 12,
    /// 32-bit unsigned integer (offset)
    IFD = 13,
    /// BigTIFF 64-bit unsigned integer
    LONG8 = 16,
    /// BigTIFF 64-bit signed integer
    SLONG8 = 17,
    /// BigTIFF 64-bit unsigned integer (offset)
    IFD8 = 18,
}
}

impl Type {
    pub(crate) fn byte_len(&self) -> u8 {
        match *self {
            Type::BYTE | Type::SBYTE | Type::ASCII | Type::UNDEFINED => 1,
            Type::SHORT | Type::SSHORT => 2,
            Type::LONG | Type::SLONG | Type::FLOAT | Type::IFD => 4,
            Type::LONG8
            | Type::SLONG8
            | Type::DOUBLE
            | Type::RATIONAL
            | Type::SRATIONAL
            | Type::IFD8 => 8,
        }
    }

    pub(crate) fn value_bytes(&self, count: u64) -> Result<u64, crate::error::TiffError> {
        let tag_size = u64::from(self.byte_len());

        match count.checked_mul(tag_size) {
            Some(n) => Ok(n),
            None => Err(crate::error::TiffError::LimitsExceeded),
        }
    }

    pub(crate) fn endian_bytes(self) -> EndianBytes {
        match self {
            Type::BYTE | Type::SBYTE | Type::ASCII | Type::UNDEFINED => EndianBytes::One,
            Type::SHORT | Type::SSHORT => EndianBytes::Two,
            Type::LONG
            | Type::SLONG
            | Type::FLOAT
            | Type::IFD
            | Type::RATIONAL
            | Type::SRATIONAL => EndianBytes::Four,
            Type::LONG8 | Type::SLONG8 | Type::DOUBLE | Type::IFD8 => EndianBytes::Eight,
        }
    }
}

tags! {
/// See [TIFF compression tags](https://www.awaresystems.be/imaging/tiff/tifftags/compression.html)
/// for reference.
pub enum CompressionMethod(u16) unknown(
    /// A custom compression method
    unknown
) {
    None = 1,
    Huffman = 2,
    Fax3 = 3,
    Fax4 = 4,
    LZW = 5,
    JPEG = 6,
    // "Extended JPEG" or "new JPEG" style
    ModernJPEG = 7,
    Deflate = 8,
    OldDeflate = 0x80B2,
    PackBits = 0x8005,

    // Self-assigned by libtiff
    ZSTD = 0xC350,

    // Self-assigned by libtiff
    WebP = 0xC351,
}
}

tags! {
pub enum PhotometricInterpretation(u16) {
    WhiteIsZero = 0,
    BlackIsZero = 1,
    RGB = 2,
    RGBPalette = 3,
    TransparencyMask = 4,
    CMYK = 5,
    YCbCr = 6,
    CIELab = 8,
    IccLab = 9,
    ItuLab = 10,
}
}

tags! {
pub enum PlanarConfiguration(u16) {
    Chunky = 1,
    Planar = 2,
}
}

tags! {
pub enum Predictor(u16) {
    /// No changes were made to the data
    None = 1,
    /// The images' rows were processed to contain the difference of each pixel from the previous one.
    ///
    /// This means that instead of having in order `[r1, g1. b1, r2, g2 ...]` you will find
    /// `[r1, g1, b1, r2-r1, g2-g1, b2-b1, r3-r2, g3-g2, ...]`
    Horizontal = 2,
    /// Not currently supported
    FloatingPoint = 3,
}
}

tags! {
/// Type to represent resolution units
pub enum ResolutionUnit(u16) {
    None = 1,
    Inch = 2,
    Centimeter = 3,
}
}

tags! {
pub enum SampleFormat(u16) unknown(
    /// An unknown extension sample format
    unknown
) {
    Uint = 1,
    Int = 2,
    IEEEFP = 3,
    Void = 4,
}
}

tags! {
pub enum ExtraSamples(u16) {
    /// There is no specified association between the sample and the image.
    Unspecified = 0,
    /// The sample is associated alpha, i.e. pre-multiplied color.
    AssociatedAlpha = 1,
    /// The sample is unassociated alpha such as a mask. There might be more than one such sample.
    UnassociatedAlpha = 2,
}
}

/// A value represented as in-memory bytes with flexible byteorder.
pub struct ValueBuffer {
    /// The raw bytes of the value.
    bytes: Vec<u8>,

    /// The type of the value.
    ty: Type,

    /// The number of items, as `bytes` may be oversized while holding bytes that are initialized
    /// but not used by any value.
    count: u64,

    /// The byte order of the value.
    byte_order: ByteOrder,
}

impl ValueBuffer {
    /// A value with a count of zero.
    ///
    /// The byte order is set to the native byte order of the platform.
    pub fn empty(ty: Type) -> Self {
        ValueBuffer {
            bytes: vec![],
            ty,
            count: 0,
            byte_order: ByteOrder::native(),
        }
    }

    /// Create a value with native byte order from in-memory data.
    pub fn from_value<T: TiffValue>(value: &T) -> Self {
        ValueBuffer {
            bytes: value.data().into_owned(),
            ty: <T as TiffValue>::FIELD_TYPE,
            count: value.count() as u64,
            byte_order: ByteOrder::native(),
        }
    }

    pub fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    pub fn data_type(&self) -> Type {
        self.ty
    }

    /// The count of items in the value.
    pub fn count(&self) -> u64 {
        debug_assert!({
            self.ty
                .value_bytes(self.count)
                .is_ok_and(|n| n <= self.bytes.len() as u64)
        });

        self.count
    }

    /// View the underlying raw bytes of this value.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.assumed_len_from_count()]
    }

    /// View the underlying mutable raw bytes of this value.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let len = self.assumed_len_from_count();
        &mut self.bytes[..len]
    }

    /// Change the byte order of the value representation.
    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        let len = self.assumed_len_from_count();

        self.byte_order
            .convert(self.ty, &mut self.bytes[..len], byte_order);

        self.byte_order = byte_order;
    }

    /// Prepare the internal for a value `to_len` bytes long.
    ///
    /// Shrinks the allocation if it is far too large or extends it if it is too small. In either
    /// case ensures that at least `to_len` bytes are initialized for [`Self::raw_bytes_mut`].
    pub(crate) fn prepare_length(&mut self, to_len: usize) {
        if to_len > self.bytes.len() {
            self.bytes.resize(to_len, 0);
        }

        if self.bytes.len() < to_len / 2 {
            self.bytes.truncate(to_len);
            self.bytes.shrink_to_fit();
        }
    }

    /// Internal method to change the type and count while re-interpreting the byte buffer.
    ///
    /// Should only be called after writing bytes to the internal buffer prepared with
    /// `Self::prepare_length`.
    pub(crate) fn assume_type(&mut self, ty: Type, count: u64, bo: ByteOrder) {
        debug_assert!({
            ty.value_bytes(count)
                .is_ok_and(|n| n <= self.bytes.len() as u64)
        });

        self.byte_order = bo;
        self.ty = ty;
        self.count = count;
    }

    pub(crate) fn raw_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    fn assumed_len_from_count(&self) -> usize {
        usize::from(self.ty.byte_len()) * self.count as usize
    }
}

/// Byte order of the TIFF file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    /// little endian byte order
    LittleEndian,
    /// big endian byte order
    BigEndian,
}

impl ByteOrder {
    /// Get the byte order representing the running target.
    ///
    /// The infallibility of this method represents the fact that only little and big endian
    /// systems are supported by the library. No mixed endian and no other weird stuff. (Note: as
    /// of Rust 1.90 this is a tautology as Rust itself only has those two kinds).
    pub const fn native() -> Self {
        match () {
            #[cfg(target_endian = "little")]
            () => ByteOrder::LittleEndian,
            #[cfg(target_endian = "big")]
            () => ByteOrder::BigEndian,
            #[cfg(not(any(target_endian = "big", target_endian = "little")))]
            () => compile_error!("Unsupported target"),
        }
    }

    /// Given a typed buffer, convert its contents to the specified byte order in-place.
    ///
    /// The buffer is assumed to represent an array of the given type. If the length of the buffer
    /// is not divisible into an integer number of values, the behavior for the remaining bytes it
    /// not specified.
    pub fn convert(self, ty: Type, buffer: &mut [u8], to: ByteOrder) {
        self.convert_endian_bytes(ty.endian_bytes(), buffer, to)
    }

    pub(crate) fn convert_endian_bytes(self, cls: EndianBytes, buffer: &mut [u8], to: ByteOrder) {
        if self == to {
            return;
        }

        // FIXME: at MSRV 1.89 or higher use `slice::as_chunks_mut`.
        match cls {
            EndianBytes::One => {
                // No change needed
            }
            EndianBytes::Two => {
                for chunk in buffer.chunks_exact_mut(2) {
                    let chunk: &mut [u8; 2] = chunk.try_into().unwrap();
                    *chunk = u16::from_be_bytes(*chunk).to_le_bytes();
                }
            }
            EndianBytes::Four => {
                for chunk in buffer.chunks_exact_mut(4) {
                    let chunk: &mut [u8; 4] = chunk.try_into().unwrap();
                    *chunk = u32::from_be_bytes(*chunk).to_le_bytes();
                }
            }
            EndianBytes::Eight => {
                for chunk in buffer.chunks_exact_mut(8) {
                    let chunk: &mut [u8; 8] = chunk.try_into().unwrap();
                    *chunk = u64::from_be_bytes(*chunk).to_le_bytes();
                }
            }
        }
    }
}

/// The size of individual byte-order corrected elements.
#[derive(Clone, Copy)]
pub(crate) enum EndianBytes {
    One,
    Two,
    Four,
    Eight,
}
