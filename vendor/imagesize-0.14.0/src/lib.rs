#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek};
use std::path::Path;

mod container;
mod formats;
mod util;

pub use container::heif::Compression;
use {
    container::heif::{self},
    formats::*,
};

/// An Error type used in failure cases.
#[derive(Debug)]
pub enum ImageError {
    /// Used when the given data is not a supported format.
    NotSupported,
    /// Used when the image has an invalid format.
    CorruptedImage,
    /// Used when an IoError occurs when trying to read the given data.
    IoError(std::io::Error),
}

impl Error for ImageError {}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ImageError::*;
        match self {
            NotSupported => f.write_str("Could not decode image"),
            CorruptedImage => f.write_str("Hit end of file before finding size"),
            IoError(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;

/// Types of image formats that this crate can identify.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageType {
    /// Animated sprite image format
    /// <https://github.com/aseprite/aseprite>
    #[cfg(feature = "aesprite")]
    Aseprite,
    /// Standard Bitmap
    #[cfg(feature = "bmp")]
    Bmp,
    /// DirectDraw Surface
    #[cfg(feature = "dds")]
    Dds,
    /// OpenEXR
    #[cfg(feature = "exr")]
    Exr,
    /// Farbfeld
    /// <https://tools.suckless.org/farbfeld/>
    #[cfg(feature = "farbfeld")]
    Farbfeld,
    /// Standard GIF
    #[cfg(feature = "gif")]
    Gif,
    /// Radiance HDR
    #[cfg(feature = "hdr")]
    Hdr,
    /// Image Container Format
    #[cfg(feature = "heif")]
    Heif(Compression),
    /// Icon file
    #[cfg(feature = "ico")]
    Ico,
    /// Interleaved Bitmap
    #[cfg(feature = "ilbm")]
    Ilbm,
    /// Standard JPEG
    #[cfg(feature = "jpeg")]
    Jpeg,
    /// JPEG XL
    #[cfg(feature = "jxl")]
    Jxl,
    /// Khronos Texture Container
    #[cfg(feature = "ktx2")]
    Ktx2,
    /// Standard PNG
    #[cfg(feature = "png")]
    Png,
    /// Portable Any Map
    #[cfg(feature = "pnm")]
    Pnm,
    /// Photoshop Document
    #[cfg(feature = "psd")]
    Psd,
    /// Quite OK Image Format
    /// <https://qoiformat.org/>
    #[cfg(feature = "qoi")]
    Qoi,
    /// Truevision Graphics Adapter
    #[cfg(feature = "tga")]
    Tga,
    /// Standard TIFF
    #[cfg(feature = "tiff")]
    Tiff,
    /// Valve Texture Format
    #[cfg(feature = "vtf")]
    Vtf,
    /// Standard Webp
    #[cfg(feature = "webp")]
    Webp,
}

impl ImageType {
    /// Calls the correct image size method based on the image type
    ///
    /// # Arguments
    /// * `reader` - A reader for the data
    pub fn reader_size<R: BufRead + Seek>(&self, reader: &mut R) -> ImageResult<ImageSize> {
        match self {
            #[cfg(feature = "aesprite")]
            ImageType::Aseprite => aesprite::size(reader),
            #[cfg(feature = "bmp")]
            ImageType::Bmp => bmp::size(reader),
            #[cfg(feature = "dds")]
            ImageType::Dds => dds::size(reader),
            #[cfg(feature = "exr")]
            ImageType::Exr => exr::size(reader),
            #[cfg(feature = "farbfeld")]
            ImageType::Farbfeld => farbfeld::size(reader),
            #[cfg(feature = "gif")]
            ImageType::Gif => gif::size(reader),
            #[cfg(feature = "hdr")]
            ImageType::Hdr => hdr::size(reader),
            #[cfg(feature = "ico")]
            ImageType::Ico => ico::size(reader),
            #[cfg(feature = "ilbm")]
            ImageType::Ilbm => ilbm::size(reader),
            #[cfg(feature = "jpeg")]
            ImageType::Jpeg => jpeg::size(reader),
            #[cfg(feature = "jxl")]
            ImageType::Jxl => jxl::size(reader),
            #[cfg(feature = "ktx2")]
            ImageType::Ktx2 => ktx2::size(reader),
            #[cfg(feature = "png")]
            ImageType::Png => png::size(reader),
            #[cfg(feature = "pnm")]
            ImageType::Pnm => pnm::size(reader),
            #[cfg(feature = "psd")]
            ImageType::Psd => psd::size(reader),
            #[cfg(feature = "qoi")]
            ImageType::Qoi => qoi::size(reader),
            #[cfg(feature = "tga")]
            ImageType::Tga => tga::size(reader),
            #[cfg(feature = "tiff")]
            ImageType::Tiff => tiff::size(reader),
            #[cfg(feature = "vtf")]
            ImageType::Vtf => vtf::size(reader),
            #[cfg(feature = "webp")]
            ImageType::Webp => webp::size(reader),

            #[cfg(feature = "heif")]
            ImageType::Heif(..) => heif::size(reader),
        }
    }
}

/// Holds the size information of an image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageSize {
    /// Width of an image in pixels.
    pub width: usize,
    /// Height of an image in pixels.
    pub height: usize,
}

impl Ord for ImageSize {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.width * self.height).cmp(&(other.width * other.height))
    }
}

impl PartialOrd for ImageSize {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Get the image type from a header
///
/// # Arguments
/// * `header` - The header of the file.
///
/// # Remarks
///
/// This will check the header to determine what image type the data is.
pub fn image_type(header: &[u8]) -> ImageResult<ImageType> {
    formats::image_type(&mut Cursor::new(header))
}

/// Get the image size from a local file
///
/// # Arguments
/// * `path` - A local path to the file to parse.
///
/// # Remarks
///
/// Will try to read as little of the file as possible in order to get the
/// proper size information.
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image format
/// * The data isn't long enough to find the size for the given format
///
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
///
/// # Examples
///
/// ```
/// use imagesize::size;
///
/// match size("test/test.webp") {
///     Ok(dim) => {
///         assert_eq!(dim.width, 716);
///         assert_eq!(dim.height, 716);
///     }
///     Err(why) => println!("Error getting size: {:?}", why)
/// }
/// ```
///
/// [`ImageError`]: enum.ImageError.html
pub fn size<P: AsRef<Path>>(path: P) -> ImageResult<ImageSize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader_size(reader)
}

/// Get the image size from a block of raw data.
///
/// # Arguments
/// * `data` - A Vec containing the data to parse for image size.
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image format
/// * The data isn't long enough to find the size for the given format
///
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
///
/// # Examples
///
/// ```
/// use imagesize::blob_size;
///
/// // First few bytes of arbitrary data.
/// let data = vec![0x89, 0x89, 0x89, 0x89, 0x0D, 0x0A, 0x1A, 0x0A,
///                 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
///                 0x00, 0x00, 0x00, 0x7B, 0x01, 0x00, 0x01, 0x41,
///                 0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4];
///
/// assert_eq!(blob_size(&data).is_err(), true);
/// ```
///
/// [`ImageError`]: enum.ImageError.html
pub fn blob_size(data: &[u8]) -> ImageResult<ImageSize> {
    let reader = Cursor::new(data);
    reader_size(reader)
}

/// Get the image size from a reader
///
/// # Arguments
/// * `reader` - A reader for the data
///
/// # Error
///
/// This method will return an [`ImageError`] under the following conditions:
///
/// * The header isn't recognized as a supported image format
/// * The data isn't long enough to find the size for the given format
///
/// The minimum data required is 12 bytes. Anything shorter will return [`ImageError::IoError`].
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use imagesize::reader_size;
///
/// // PNG Header with size 123x321
/// let reader = Cursor::new([
///     0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
///     0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
///     0x00, 0x00, 0x00, 0x7B, 0x00, 0x00, 0x01, 0x41,
///     0x08, 0x06, 0x00, 0x00, 0x00, 0x9A, 0x38, 0xC4
/// ]);
///
/// match reader_size(reader) {
///     Ok(dim) => {
///         assert_eq!(dim.width, 123);
///         assert_eq!(dim.height, 321);
///     }
///     Err(why) => println!("Error getting reader size: {:?}", why)
/// }
/// ```
///
/// [`ImageError`]: enum.ImageError.html
pub fn reader_size<R: BufRead + Seek>(mut reader: R) -> ImageResult<ImageSize> {
    reader_type(&mut reader)?.reader_size(&mut reader)
}

/// Get the image type from a reader
///
/// # Arguments
/// * `reader` - A reader for the data
///
/// # Remarks
///
/// This will check the header to determine what image type the data is.
pub fn reader_type<R: BufRead + Seek>(mut reader: R) -> ImageResult<ImageType> {
    formats::image_type(&mut reader)
}
