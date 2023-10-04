use crate::{size, DevicePixels, Result, SharedString, Size};
use anyhow::anyhow;
use image::{Bgra, ImageBuffer};
use std::{
    borrow::Cow,
    fmt,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

pub trait AssetSource: 'static + Send + Sync {
    fn load(&self, path: &SharedString) -> Result<Cow<[u8]>>;
    fn list(&self, path: &SharedString) -> Result<Vec<SharedString>>;
}

impl AssetSource for () {
    fn load(&self, path: &SharedString) -> Result<Cow<[u8]>> {
        Err(anyhow!(
            "get called on empty asset provider with \"{}\"",
            path
        ))
    }

    fn list(&self, _path: &SharedString) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImageId(usize);

pub struct ImageData {
    pub id: ImageId,
    data: ImageBuffer<Bgra<u8>, Vec<u8>>,
}

impl ImageData {
    pub fn new(data: ImageBuffer<Bgra<u8>, Vec<u8>>) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

        Self {
            id: ImageId(NEXT_ID.fetch_add(1, SeqCst)),
            data,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

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
