use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;

use crate::{
    hash, App, Asset, AssetLogger, AsyncApp, ImageAssetLoader, ImageCacheError, RenderImage,
    Resource, Task, Window,
};
use futures::{future::Shared, FutureExt};

type ImageLoadingTask = Shared<Task<Result<Arc<RenderImage>, ImageCacheError>>>;

struct ImageCacheInner {
    app: AsyncApp,
    images: Mutex<HashMap<u64, ImageLoadingTask>>,
}

impl Drop for ImageCacheInner {
    fn drop(&mut self) {
        let app = self.app.clone();
        let mut images = self.images.lock();
        let images = images
            .drain()
            .filter_map(|(_, task)| task.now_or_never().transpose().ok().flatten())
            .collect::<Vec<_>>();

        // Spawn a task to drop the images in the background
        self.app
            .foreground_executor()
            .spawn(async move {
                _ = app.update(move |cx| {
                    for image in images {
                        for window in cx.windows.values_mut() {
                            if let Some(window) = window {
                                _ = window.drop_image(image.clone());
                            }
                        }
                    }
                });
            })
            .detach();
    }
}

/// An cache for loading images from external sources.
#[derive(Clone)]
pub struct ImageCache(Arc<ImageCacheInner>);

impl ImageCache {
    /// Create a new image cache.
    #[inline]
    pub fn new(cx: &mut App) -> Self {
        ImageCache(Arc::new(ImageCacheInner {
            app: cx.to_async(),
            images: Default::default(),
        }))
    }

    pub(crate) fn load(
        &self,
        source: &Resource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        let mut images = self.0.images.lock();
        let hash = hash(source);
        let mut is_first = false;
        let task = images
            .entry(hash)
            .or_insert_with(|| {
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
        let mut images = self.0.images.lock();
        for (_, task) in images.drain() {
            if let Some(Ok(image)) = task.now_or_never() {
                // remove the texture from the current window
                _ = window.drop_image(image.clone());

                // remove the texture from all other windows
                for window in cx.windows.values_mut() {
                    if let Some(window) = window {
                        _ = window.drop_image(image.clone());
                    }
                }
            }
        }
    }

    /// Remove the image from the cache by the given source.
    pub fn remove(&self, source: &Resource, window: &mut Window, cx: &mut App) {
        let mut images = self.0.images.lock();
        let hash = hash(source);
        if let Some(task) = images.remove(&hash) {
            if let Some(Ok(image)) = task.now_or_never() {
                // remove the texture from the current window
                _ = window.drop_image(image.clone());

                // remove the texture from all other windows
                for window in cx.windows.values_mut() {
                    if let Some(window) = window {
                        _ = window.drop_image(image.clone());
                    }
                }
            }
        }
    }

    /// Returns the number of images in the cache.
    pub fn len(&self) -> usize {
        self.0.images.lock().len()
    }
}
