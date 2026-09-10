use crate::{DevicePixels, Pixels, Result, SharedString, Size, size};
use smallvec::SmallVec;

use image::{Delay, Frame};
use std::{
    borrow::Cow,
    fmt,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

/// A source of assets for this app to use.
pub trait AssetSource: 'static + Send + Sync {
    /// Load the given asset from the source path.
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>>;

    /// List the assets at the given path.
    fn list(&self, path: &str) -> Result<Vec<SharedString>>;
}

impl AssetSource for () {
    fn load(&self, _path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(None)
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

/// A unique identifier for the image cache
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(pub usize);

/// A unique identifier for a mutable dynamic texture.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DynamicTextureId(pub usize);

#[derive(PartialEq, Eq, Hash, Clone)]
#[expect(missing_docs)]
pub struct RenderImageParams {
    pub image_id: ImageId,
    pub frame_index: usize,
}

/// The atlas lookup parameters for a dynamic texture.
#[derive(PartialEq, Eq, Hash, Clone)]
#[doc(hidden)]
pub struct DynamicTextureParams {
    /// The stable identity of the dynamic texture.
    pub texture_id: DynamicTextureId,
}

/// A mutable BGRA texture with a stable identity and fixed device-pixel size.
pub struct DynamicTexture {
    /// The stable ID associated with this texture.
    pub id: DynamicTextureId,
    size: Size<DevicePixels>,
}

impl PartialEq for DynamicTexture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DynamicTexture {}

impl DynamicTexture {
    /// Creates a dynamic texture with the given device-pixel size.
    pub fn new(size: Size<DevicePixels>) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let id = NEXT_ID
            .fetch_update(SeqCst, SeqCst, |id| id.checked_add(1))
            .expect("dynamic texture identifier space exhausted");

        Self {
            id: DynamicTextureId(id),
            size,
        }
    }

    /// Returns the fixed size of this texture in device pixels.
    pub fn size(&self) -> Size<DevicePixels> {
        self.size
    }
}

impl fmt::Debug for DynamicTexture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DynamicTexture")
            .field("id", &self.id)
            .field("size", &self.size)
            .finish()
    }
}

/// A cached and processed image, in BGRA format
pub struct RenderImage {
    /// The ID associated with this image
    pub id: ImageId,
    /// The scale factor of this image on render.
    pub(crate) scale_factor: f32,
    data: SmallVec<[Frame; 1]>,
}

impl PartialEq for RenderImage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RenderImage {}

impl RenderImage {
    /// Create a new image from the given data.
    pub fn new(data: impl Into<SmallVec<[Frame; 1]>>) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
            scale_factor: 1.0,
            data: data.into(),
        }
    }

    /// Convert this image into a byte slice.
    pub fn as_bytes(&self, frame_index: usize) -> Option<&[u8]> {
        self.data
            .get(frame_index)
            .map(|frame| frame.buffer().as_raw().as_slice())
    }

    /// Get the size of this image, in pixels.
    pub fn size(&self, frame_index: usize) -> Size<DevicePixels> {
        self.data
            .get(frame_index)
            .map(|frame| {
                let (width, height) = frame.buffer().dimensions();
                size(width.into(), height.into())
            })
            .unwrap_or_default()
    }

    /// Get the size of this image, in pixels for display, adjusted for the scale factor.
    pub(crate) fn render_size(&self, frame_index: usize) -> Size<Pixels> {
        self.size(frame_index)
            .map(|v| (v.0 as f32 / self.scale_factor).into())
    }

    /// Get the delay of this frame from the previous
    pub fn delay(&self, frame_index: usize) -> Delay {
        self.data
            .get(frame_index)
            .map(|frame| frame.delay())
            .unwrap_or(Delay::from_numer_denom_ms(100, 1))
    }

    /// Get the number of frames for this image.
    pub fn frame_count(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Debug for RenderImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageData")
            .field("id", &self.id)
            .field("size", &self.data.first().map(|f| f.buffer().dimensions()))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::SmallVec;

    #[test]
    fn empty_render_image_does_not_panic() {
        let image = RenderImage::new(SmallVec::new());
        assert_eq!(image.frame_count(), 0);
        assert_eq!(image.size(0), Size::default());
        assert_eq!(image.as_bytes(0), None);
        assert_eq!(image.render_size(0), Size::default());
        assert_eq!(image.delay(0), Delay::from_numer_denom_ms(100, 1));
        let _ = format!("{image:?}");
    }

    #[test]
    fn dynamic_textures_keep_distinct_stable_identities() {
        let size = size(DevicePixels(640), DevicePixels(480));
        let first = DynamicTexture::new(size);
        let second = DynamicTexture::new(size);

        assert_eq!(first.size(), size);
        assert_ne!(first.id, second.id);
        assert_ne!(first, second);
    }
}
