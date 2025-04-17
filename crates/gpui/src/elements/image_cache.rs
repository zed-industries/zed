use crate::{
    AnyElement, App, AppContext, Asset, AssetLogger, AsyncApp, Bounds, Element, ElementId, Entity,
    GlobalElementId, ImageAssetLoader, ImageCacheError, IntoElement, LayoutId, ParentElement,
    Pixels, RenderImage, Resource, Style, StyleRefinement, Styled, Task, Window, hash,
};

use futures::{FutureExt, future::Shared};
use lru::LruCache;
use refineable::Refineable;
use smallvec::SmallVec;
use std::{fmt, sync::Arc};

/// An image cache element, all its child img elements will use the cache specified by this element.
pub fn image_cache(image_cache: &Entity<ImageCache>) -> ImageCacheElement {
    ImageCacheElement {
        image_cache: image_cache.clone(),
        style: StyleRefinement::default(),
        children: SmallVec::default(),
    }
}

/// An image cache element.
pub struct ImageCacheElement {
    image_cache: Entity<ImageCache>,
    style: StyleRefinement,
    children: SmallVec<[AnyElement; 2]>,
}

impl ParentElement for ImageCacheElement {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl Styled for ImageCacheElement {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for ImageCacheElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ImageCacheElement {
    type RequestLayoutState = SmallVec<[LayoutId; 4]>;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_image_cache(self.image_cache.clone(), |window| {
            let child_layout_ids = self
                .children
                .iter_mut()
                .map(|child| child.request_layout(window, cx))
                .collect::<SmallVec<_>>();
            let mut style = Style::default();
            style.refine(&self.style);
            let layout_id = window.request_layout(style, child_layout_ids.iter().copied(), cx);
            (layout_id, child_layout_ids)
        })
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        for child in &mut self.children {
            child.prepaint(window, cx);
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_image_cache(self.image_cache.clone(), |window| {
            for child in &mut self.children {
                child.paint(window, cx);
            }
        })
    }
}

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

impl Drop for ImageCache {
    fn drop(&mut self) {
        let app = self.app.clone();
        let images = std::mem::replace(&mut self.images, LruCache::unbounded())
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
pub struct ImageCache {
    app: AsyncApp,
    images: LruCache<u64, CacheItem>,
    max_items: Option<usize>,
}

impl fmt::Debug for ImageCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCache")
            .field("num_images", &self.images.len())
            .finish()
    }
}

impl ImageCache {
    /// Create a new image cache.
    #[inline]
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| ImageCache {
            app: cx.to_async(),
            images: LruCache::unbounded(),
            max_items: None,
        })
    }

    /// Create a new image cache with a maximum number of items.
    pub fn lru(max_items: usize, cx: &mut App) -> Self {
        ImageCache {
            app: cx.to_async(),
            images: LruCache::unbounded(),
            max_items: Some(max_items),
        }
    }

    /// Load an image from the given source.
    ///
    /// Returns `None` if the image is loading.
    pub fn load(
        &mut self,
        source: &Resource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Result<Arc<RenderImage>, ImageCacheError>> {
        if let Some(max_items) = self.max_items {
            // remove least recently used images
            while self.images.len() >= max_items {
                if let Some((_, mut item)) = self.images.pop_lru() {
                    if let Some(Ok(image)) = item.get() {
                        remove_image_from_windows(image, window, cx);
                    }
                }
            }
        }

        let hash = hash(source);

        if let Some(item) = self.images.get_mut(&hash) {
            return item.get();
        }

        self.images.len();

        let fut = AssetLogger::<ImageAssetLoader>::load(source.clone(), cx);
        let task = cx.background_executor().spawn(fut).shared();
        self.images.push(hash, CacheItem::Loading(task.clone()));

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
    pub fn clear(&mut self, window: &mut Window, cx: &mut App) {
        for (_, mut item) in std::mem::replace(&mut self.images, LruCache::unbounded()) {
            if let Some(Ok(image)) = item.get() {
                remove_image_from_windows(image, window, cx);
            }
        }
    }

    /// Remove the image from the cache by the given source.
    pub fn remove(&mut self, source: &Resource, window: &mut Window, cx: &mut App) {
        let hash = hash(source);
        if let Some(mut item) = self.images.pop(&hash) {
            if let Some(Ok(image)) = item.get() {
                remove_image_from_windows(image, window, cx);
            }
        }
    }

    /// Returns the number of images in the cache.
    pub fn len(&self) -> usize {
        self.images.len()
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
