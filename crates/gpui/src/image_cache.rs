use std::{cell::RefCell, rc::Rc, sync::Arc};

use futures::{FutureExt, future::Shared};
use lru::LruCache;

use crate::{
    App, Asset, AssetLogger, AsyncApp, ImageAssetLoader, ImageCacheError, RenderImage, Resource,
    Task, Window, hash,
};

type ImageLoadingTask = Shared<Task<Result<Arc<RenderImage>, ImageCacheError>>>;

struct ImageCacheInner {
    app: AsyncApp,
    images: RefCell<LruCache<u64, ImageLoadingTask>>,
    max_items: Option<usize>,
}

impl Drop for ImageCacheInner {
    fn drop(&mut self) {
        let app = self.app.clone();
        let mut images = self.images.borrow_mut();
        let images = std::mem::replace(&mut *images, LruCache::unbounded())
            .into_iter()
            .filter_map(|(_, task)| task.now_or_never().transpose().ok().flatten())
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

impl ImageCache {
    /// Create a new image cache.
    #[inline]
    pub fn new(cx: &mut App) -> Self {
        ImageCache(Rc::new(ImageCacheInner {
            app: cx.to_async(),
            images: RefCell::new(LruCache::unbounded()),
            max_items: None,
        }))
    }

    /// Create a new image cache with a maximum number of items.
    pub fn max_items(max_items: usize, cx: &mut App) -> Self {
        ImageCache(Rc::new(ImageCacheInner {
            app: cx.to_async(),
            images: RefCell::new(LruCache::unbounded()),
            max_items: Some(max_items),
        }))
    }

    pub(crate) fn load(
        &self,
        source: &Resource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        let mut images = self.0.images.borrow_mut();

        if let Some(max_items) = self.0.max_items {
            // remove least recently used images
            while images.len() >= max_items {
                if let Some((_, task)) = images.pop_lru() {
                    if let Some(Ok(image)) = task.now_or_never() {
                        remove_image_from_windows(image.clone(), window, cx);
                    }
                }
            }
        }

        let hash = hash(source);
        let mut is_first = false;
        let task = images
            .get_or_insert(hash, || {
                is_first = true;
                let fut = AssetLogger::<ImageAssetLoader>::load(source.clone(), cx);
                cx.background_executor().spawn(fut).shared()
            })
            .clone();

        task.clone().now_or_never().or_else(|| {
            if is_first {
                let entity = window.current_view();
                window
                    .spawn(cx, {
                        let task = task.clone();
                        async move |cx| {
                            _ = task.await;
                            cx.on_next_frame(move |_, cx| {
                                cx.notify(entity);
                            });
                        }
                    })
                    .detach();
            }

            None
        })
    }

    /// Clear the image cache.
    pub fn clear(&self, window: &mut Window, cx: &mut App) {
        let mut images = self.0.images.borrow_mut();
        for (_, task) in std::mem::replace(&mut *images, LruCache::unbounded()) {
            if let Some(Ok(image)) = task.now_or_never() {
                remove_image_from_windows(image.clone(), window, cx);
            }
        }
    }

    /// Remove the image from the cache by the given source.
    pub fn remove(&self, source: &Resource, window: &mut Window, cx: &mut App) {
        let mut images = self.0.images.borrow_mut();
        let hash = hash(source);
        if let Some(task) = images.pop(&hash) {
            if let Some(Ok(image)) = task.now_or_never() {
                remove_image_from_windows(image.clone(), window, cx);
            }
        }
    }

    /// Returns the number of images in the cache.
    pub fn len(&self) -> usize {
        self.0.images.borrow_mut().len()
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
