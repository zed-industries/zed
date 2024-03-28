use crate::{size, DevicePixels, Result, SharedString, Size};
use anyhow::anyhow;
use image::RgbaImage;
use std::{
    borrow::Cow,
    fmt,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

/// A source of assets for this app to use.
pub trait AssetSource: 'static + Send + Sync {
    /// Load the given asset from the source path.
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>>;

    /// List the assets at the given path.
    fn list(&self, path: &str) -> Result<Vec<SharedString>>;
}

impl AssetSource for () {
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>> {
        Err(anyhow!(
            "get called on empty asset provider with \"{}\"",
            path
        ))
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

/// A unique identifier for the image cache
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(usize);

/// A cached and processed image.
pub struct ImageData {
    /// The ID associated with this image
    pub id: ImageId,
    data: RgbaImage,
}

impl ImageData {
    /// Create a new image from the given data.
    pub fn new(data: RgbaImage) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
            data,
        }
    }

    /// Convert this image into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get the size of this image, in pixels
    pub fn size(&self) -> Size<DevicePixels> {
        let (width, height) = self.data.dimensions();
        size(width.into(), height.into())
    }
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageData")
            .field("id", &self.id)
            .field("size", &self.data.dimensions())
            .finish()
    }
}
