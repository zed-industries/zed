#[cfg(feature = "dbus")]
use dbus::arg::messageitem::{MessageItem, MessageItemArray};
pub use image::DynamicImage;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::miniver::Version;

mod constants {
    pub const IMAGE_DATA: &str = "image-data";
    pub const IMAGE_DATA_1_1: &str = "image_data";
    pub const IMAGE_DATA_1_0: &str = "icon_data";
}

/// Image data for inline notifications. Send via [`Notification::image_data()`](crate::Notification::image_data).
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Image {
    width: i32,
    height: i32,
    rowstride: i32,
    alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>,
}

impl Image {
    fn from_raw_data(
        width: i32,
        height: i32,
        data: Vec<u8>,
        channels: i32,
        bits_per_sample: i32,
        alpha: bool,
    ) -> Result<Self, ImageError> {
        const MAX_SIZE: i32 = 0x0fff_ffff;
        if width > MAX_SIZE || height > MAX_SIZE {
            return Err(ImageError::TooBig);
        }

        if data.len() != (width * height * channels) as usize {
            Err(ImageError::WrongDataSize)
        } else {
            Ok(Self {
                width,
                height,
                bits_per_sample,
                channels,
                data,
                rowstride: width * channels,
                alpha,
            })
        }
    }

    /// Creates an image from a raw vector of bytes
    pub fn from_rgb(width: i32, height: i32, data: Vec<u8>) -> Result<Self, ImageError> {
        let channels = 3i32;
        let bits_per_sample = 8;
        Self::from_raw_data(width, height, data, channels, bits_per_sample, false)
    }

    /// Creates an image from a raw vector of bytes with alpha
    pub fn from_rgba(width: i32, height: i32, data: Vec<u8>) -> Result<Self, ImageError> {
        let channels = 4i32;
        let bits_per_sample = 8;
        Self::from_raw_data(width, height, data, channels, bits_per_sample, true)
    }

    /// Attempts to open the given path as an image.
    pub fn open<T: AsRef<Path> + Sized>(path: T) -> Result<Self, ImageError> {
        let dyn_img = image::open(&path).map_err(ImageError::CantOpen)?;
        Image::try_from(dyn_img)
    }

    #[cfg(all(feature = "images_no_default_features", feature = "zbus"))]
    pub(crate) fn to_tuple(&self) -> (i32, i32, i32, bool, i32, i32, Vec<u8>) {
        (
            self.width,
            self.height,
            self.rowstride,
            self.alpha,
            self.bits_per_sample,
            self.channels,
            self.data.clone(),
        )
    }
}

impl TryFrom<DynamicImage> for Image {
    type Error = ImageError;

    fn try_from(dyn_img: DynamicImage) -> Result<Self, Self::Error> {
        match dyn_img {
            DynamicImage::ImageRgb8(img) => Self::try_from(img),
            DynamicImage::ImageRgba8(img) => Self::try_from(img),
            _ => Err(ImageError::CantConvert),
        }
    }
}

impl TryFrom<image::RgbImage> for Image {
    type Error = ImageError;

    fn try_from(img: image::RgbImage) -> Result<Self, Self::Error> {
        let (width, height) = img.dimensions();
        let image_data = img.into_raw();
        Image::from_rgb(width as i32, height as i32, image_data)
    }
}

impl TryFrom<image::RgbaImage> for Image {
    type Error = ImageError;

    fn try_from(img: image::RgbaImage) -> Result<Self, Self::Error> {
        let (width, height) = img.dimensions();
        let image_data = img.into_raw();
        Image::from_rgba(width as i32, height as i32, image_data)
    }
}

/// Errors that can occur when creating an Image
#[derive(Debug)]
pub enum ImageError {
    /// The given image is too big. DBus only has 32 bits for width / height
    TooBig,
    /// The given bytes don't match the width, height and channel count
    WrongDataSize,
    /// Can't open given path
    CantOpen(image::ImageError),
    /// Can't convert from given input
    CantConvert,
}

impl Error for ImageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ImageError::*;
        match self {
            TooBig | WrongDataSize | CantConvert => None,
            CantOpen(e) => Some(e),
        }
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImageError::*;
        match self {
            TooBig => writeln!(
                f,
                "The given image is too big. DBus only has 32 bits for width / height"
            ),
            WrongDataSize => writeln!(
                f,
                "The given bytes don't match the width, height and channel count"
            ),
            CantOpen(e) => writeln!(f, "Can't open given path {}", e),
            CantConvert => writeln!(f, "Can't convert from given input"),
        }
    }
}

/// matching image data key for each spec version
#[cfg(feature = "dbus")]
pub(crate) fn image_spec(version: Version) -> String {
    match version.cmp(&Version::new(1, 1)) {
        Ordering::Less => constants::IMAGE_DATA_1_0.to_owned(),
        Ordering::Equal => constants::IMAGE_DATA_1_1.to_owned(),
        Ordering::Greater => constants::IMAGE_DATA.to_owned(),
    }
}

/// matching image data key for each spec version
#[cfg(feature = "zbus")]
pub(crate) fn image_spec_str(version: Version) -> &'static str {
    match version.cmp(&Version::new(1, 1)) {
        Ordering::Less => constants::IMAGE_DATA_1_0,
        Ordering::Equal => constants::IMAGE_DATA_1_1,
        Ordering::Greater => constants::IMAGE_DATA,
    }
}

#[cfg(feature = "dbus")]
pub struct ImageMessage(Image);

#[cfg(feature = "dbus")]
impl From<Image> for ImageMessage {
    fn from(hint: Image) -> Self {
        ImageMessage(hint)
    }
}

impl From<image::ImageError> for ImageError {
    fn from(image_error: image::ImageError) -> Self {
        ImageError::CantOpen(image_error)
    }
}

#[cfg(feature = "dbus")]
impl std::ops::Deref for ImageMessage {
    type Target = Image;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "dbus")]
impl From<ImageMessage> for MessageItem {
    fn from(img_msg: ImageMessage) -> Self {
        let img = img_msg.0;

        let bytes = img.data.into_iter().map(MessageItem::Byte).collect();

        MessageItem::Struct(vec![
            MessageItem::Int32(img.width),
            MessageItem::Int32(img.height),
            MessageItem::Int32(img.rowstride),
            MessageItem::Bool(img.alpha),
            MessageItem::Int32(img.bits_per_sample),
            MessageItem::Int32(img.channels),
            MessageItem::Array(MessageItemArray::new(bytes, "ay".into()).unwrap()),
        ])
    }
}
