use crate::{
    size, AppContext, DevicePixels, FetchImageTask, ImageCacheError, Result, SharedString, Size,
};
use anyhow::anyhow;
use collections::HashMap;
use futures::FutureExt;
use image::{Bgra, ImageBuffer};
use parking_lot::Mutex;
use std::{
    borrow::Cow,
    fmt,
    hash::Hash,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc,
    },
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
pub enum ImageData {
    /// A raster image.
    Raster {
        /// The unique identifier for this image.
        id: ImageId,
        /// The image buffer for this image.
        data: ImageBuffer<Bgra<u8>, Vec<u8>>,
    },
    /// A vector image.
    Vector {
        /// The unique identifier for this image.
        id: ImageId,
        /// The SVG tree for this image.
        data: usvg::Tree,
        /// Rendered image cache.
        rendered: Arc<Mutex<HashMap<Size<DevicePixels>, FetchImageTask>>>,
    },
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
impl ImageData {
    /// Create a new image from the given data.
    pub fn new(data: ImageBuffer<Bgra<u8>, Vec<u8>>) -> Self {
        Self::Raster {
            id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
            data,
        }
    }

    /// Try to create a new image from the given bytes.
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, ImageCacheError> {
        if let Ok(format) = image::guess_format(bytes) {
            let data = image::load_from_memory_with_format(bytes, format)?.into_bgra8();
            Ok(Self::Raster {
                id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
                data,
            })
        } else {
            let data = usvg::Tree::from_data(
                bytes,
                &usvg::Options::default(),
                &usvg::fontdb::Database::default(),
            )?;
            Ok(Self::Vector {
                id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
                data,
                rendered: Default::default(),
            })
        }
    }

    /// Get the rendered image for the given size.
    pub fn rendered(&self, size: Size<DevicePixels>, cx: &AppContext) -> Option<FetchImageTask> {
        match self {
            Self::Raster { .. } => None,
            Self::Vector { rendered, data, .. } => {
                let mut rendered = rendered.lock();
                if let Some(task) = rendered.get(&size) {
                    return Some(task.clone());
                }
                let task = cx
                    .background_executor()
                    .spawn({
                        let tree = data.clone();
                        async move {
                            let mut pixmap = resvg::tiny_skia::Pixmap::new(
                                size.width.0 as u32,
                                size.height.0 as u32,
                            )
                            .unwrap();
                            let ratio = size.width.0 as f32 / tree.size().width();
                            resvg::render(
                                &tree,
                                tiny_skia::Transform::from_scale(ratio, ratio),
                                &mut pixmap.as_mut(),
                            );
                            let png = pixmap.encode_png().unwrap();
                            let image =
                                image::load_from_memory_with_format(&png, image::ImageFormat::Png)?;
                            Ok(Arc::new(ImageData::new(image.into_bgra8())))
                        }
                    })
                    .shared();
                rendered.insert(size, task.clone());
                Some(task)
            }
        }
    }

    /// Convert this image into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Raster { data, .. } => data.as_ref(),
            // Should be unreachable as we always fall down to raster before rendering
            Self::Vector { .. } => &[],
        }
    }

    /// Get the size of this image, in pixels
    pub fn size(&self) -> Size<DevicePixels> {
        match self {
            Self::Raster { data, .. } => {
                let (width, height) = data.dimensions();
                size(width.into(), height.into())
            }
            Self::Vector { data, .. } => {
                let tree_size = data.size();
                size(
                    (tree_size.width() as u32).into(),
                    (tree_size.height() as u32).into(),
                )
            }
        }
    }

    /// Get the unique identifier for this image.
    pub fn id(&self) -> ImageId {
        match self {
            Self::Raster { id, .. } => *id,
            Self::Vector { id, .. } => *id,
        }
    }
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageData")
            .field("id", &self.id())
            .field("size", &self.size())
            .finish()
    }
}
