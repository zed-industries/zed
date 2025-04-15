use futures::{FutureExt, future::Shared};
use lru::LruCache;
use parking_lot::RwLock;
use std::{fmt, rc::Rc, sync::Arc};

use crate::{
    App, Asset, AssetLogger, AsyncApp, ImageAssetLoader, ImageCacheError, RenderImage, Resource,
    Task, Window, hash,
};

type ImageLoadingTask = Shared<Task<Result<Arc<RenderImage>, ImageCacheError>>>;

enum CacheItem {
    Loading(ImageLoadingTask),
    Loaded(Result<Arc<RenderImage>, ImageCacheError>),
}

impl CacheItem {
    fn get(&mut self) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        match self {
            CacheItem::Loading(task) => {
                let res = task.now_or_never()?;
                *self = CacheItem::Loaded(res.clone());
                Some(res)
            }
            CacheItem::Loaded(res) => Some(res.clone()),
        }
    }
}

struct ImageCacheInner {
    app: AsyncApp,
    images: RwLock<LruCache<u64, CacheItem>>,
    max_items: Option<usize>,
}

impl Drop for ImageCacheInner {
    fn drop(&mut self) {
        let app = self.app.clone();
        let mut images = self.images.write();
        let images = std::mem::replace(&mut *images, LruCache::unbounded())
            .into_iter()
            .filter_map(|(_, mut item)| item.get().transpose().ok().flatten())
            .collect::<Vec<_>>();

        // Spawn a task to drop the images in the background
        self.app
            .foreground_executor()
            .spawn(async move {
                _ = app.update(move |cx| {
                    for image in images {
                        for window in cx.windows.values_mut().flatten() {
                            _ = window.drop_image(image.clone());
                        }
                    }
                });
            })
            .detach();
    }
}

/// An cache for loading images from external sources.
#[derive(Clone)]
pub struct ImageCache(Rc<ImageCacheInner>);

impl fmt::Debug for ImageCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCache")
            .field("num_images", &self.0.images.read().len())
            .finish()
    }
}

impl ImageCache {
    /// Create a new image cache.
    #[inline]
    pub fn new(cx: &mut App) -> Self {
        ImageCache(Rc::new(ImageCacheInner {
            app: cx.to_async(),
            images: RwLock::new(LruCache::unbounded()),
            max_items: None,
        }))
    }

    /// Create a new image cache with a maximum number of items.
    pub fn max_items(max_items: usize, cx: &mut App) -> Self {
        ImageCache(Rc::new(ImageCacheInner {
            app: cx.to_async(),
            images: RwLock::new(LruCache::unbounded()),
            max_items: Some(max_items),
        }))
    }

    /// Load an image from the given source.
    ///
    /// Returns `None` if the image is loading.
    pub fn load(
        &self,
        source: &Resource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        let mut images = self.0.images.write();

        if let Some(max_items) = self.0.max_items {
            // remove least recently used images
            while images.len() >= max_items {
                if let Some((_, mut item)) = images.pop_lru() {
                    if let Some(Ok(image)) = item.get() {
                        remove_image_from_windows(image, window, cx);
                    }
                }
            }
        }

        let hash = hash(source);

        if let Some(item) = images.get_mut(&hash) {
            return item.get();
        }

        let fut = AssetLogger::<ImageAssetLoader>::load(source.clone(), cx);
        let task = cx.background_executor().spawn(fut).shared();
        images.push(hash, CacheItem::Loading(task.clone()));

        let entity = window.current_view();
        window
            .spawn(cx, {
                async move |cx| {
                    _ = task.await;
                    cx.on_next_frame(move |_, cx| {
                        cx.notify(entity);
                    });
                }
            })
            .detach();

        None
    }

    /// Clear the image cache.
    pub fn clear(&self, window: &mut Window, cx: &mut App) {
        let mut images = self.0.images.write();
        for (_, mut item) in std::mem::replace(&mut *images, LruCache::unbounded()) {
            if let Some(Ok(image)) = item.get() {
                remove_image_from_windows(image, window, cx);
            }
        }
    }

    /// Remove the image from the cache by the given source.
    pub fn remove(&self, source: &Resource, window: &mut Window, cx: &mut App) {
        let mut images = self.0.images.write();
        let hash = hash(source);
        if let Some(mut item) = images.pop(&hash) {
            if let Some(Ok(image)) = item.get() {
                remove_image_from_windows(image, window, cx);
            }
        }
    }

    /// Returns the number of images in the cache.
    pub fn len(&self) -> usize {
        self.0.images.read().len()
    }
}

fn remove_image_from_windows(image: Arc<RenderImage>, window: &mut Window, cx: &mut App) {
    // remove the texture from all other windows
    for window in cx.windows.values_mut().flatten() {
        _ = window.drop_image(image.clone());
    }

    // remove the texture from the current window
    _ = window.drop_image(image);
}
