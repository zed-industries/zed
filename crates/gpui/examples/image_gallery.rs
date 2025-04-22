use futures::FutureExt;
use gpui::{
    App, AppContext, Application, Asset as _, AssetLogger, Bounds, ClickEvent, Context, ElementId,
    Entity, HashMapImageCache, ImageAssetLoader, ImageCache, ImageCacheProvider, KeyBinding, Menu,
    MenuItem, SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions, actions, div,
    hash, image_cache, img, prelude::*, px, rgb, size,
};
use reqwest_client::ReqwestClient;
use std::{collections::HashMap, sync::Arc};

const IMAGES_IN_GALLERY: usize = 30;

struct ImageGallery {
    image_key: String,
    items_count: usize,
    total_count: usize,
    image_cache: Entity<HashMapImageCache>,
}

impl ImageGallery {
    fn on_next_image(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.image_cache
            .update(cx, |image_cache, cx| image_cache.clear(window, cx));

        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        self.image_key = format!("{}", t);
        self.total_count += self.items_count;
        cx.notify();
    }
}

impl Render for ImageGallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let image_url: SharedString =
            format!("https://picsum.photos/400/200?t={}", self.image_key).into();

        div()
            .flex()
            .flex_col()
            .text_color(gpui::white())
            .child("Manually managed image cache:")
            .child(
                image_cache(self.image_cache.clone()).child(
                div()
                    .id("main")
                    .font_family(".SystemUIFont")
                    .text_color(gpui::black())
                    .bg(rgb(0xE9E9E9))
                    .overflow_y_scroll()
                    .p_4()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(format!(
                                "Example to show images and test memory usage (Rendered: {} images).",
                                self.total_count
                            ))
                            .child(
                                div()
                                    .id("btn")
                                    .py_1()
                                    .px_4()
                                    .bg(gpui::black())
                                    .hover(|this| this.opacity(0.8))
                                    .text_color(gpui::white())
                                    .text_center()
                                    .w_40()
                                    .child("Next Photos")
                                    .on_click(cx.listener(Self::on_next_image)),
                            ),
                    )
                    .child(
                        div()
                            .id("image-gallery")
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_x_4()
                            .gap_y_2()
                            .justify_around()
                            .children(
                                (0..self.items_count)
                                    .map(|ix| img(format!("{}-{}", image_url, ix)).size_20()),
                            ),
                    ),
            ))
            .child(
                "Automatically managed image cache:"
            )
            .child(image_cache(simple_lru_cache("lru-cache", IMAGES_IN_GALLERY)).child(
                div()
                    .id("main")
                    .font_family(".SystemUIFont")
                    .bg(rgb(0xE9E9E9))
                    .text_color(gpui::black())
                    .overflow_y_scroll()
                    .p_4()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .id("image-gallery")
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap_x_4()
                            .gap_y_2()
                            .justify_around()
                            .children(
                                (0..self.items_count)
                                    .map(|ix| img(format!("{}-{}", image_url, ix)).size_20()),
                            ),
                    )
            ))
    }
}

fn simple_lru_cache(id: impl Into<ElementId>, max_items: usize) -> SimpleLruCacheProvider {
    SimpleLruCacheProvider {
        id: id.into(),
        max_items,
    }
}

struct SimpleLruCacheProvider {
    id: ElementId,
    max_items: usize,
}

impl ImageCacheProvider for SimpleLruCacheProvider {
    fn provide(&mut self, window: &mut Window, cx: &mut App) -> gpui::AnyImageCache {
        window
            .with_global_id(self.id.clone(), |global_id, window| {
                window.with_element_state::<Entity<SimpleLruCache>, _>(
                    global_id,
                    |lru_cache, _window| {
                        let mut lru_cache = lru_cache.unwrap_or_else(|| {
                            cx.new(|cx| SimpleLruCache::new(self.max_items, cx))
                        });
                        if lru_cache.read(cx).max_items != self.max_items {
                            lru_cache = cx.new(|cx| SimpleLruCache::new(self.max_items, cx));
                        }
                        (lru_cache.clone(), lru_cache)
                    },
                )
            })
            .into()
    }
}

struct SimpleLruCache {
    max_items: usize,
    usages: Vec<u64>,
    cache: HashMap<u64, gpui::ImageCacheItem>,
}

impl SimpleLruCache {
    fn new(max_items: usize, cx: &mut Context<Self>) -> Self {
        cx.on_release(|simple_cache, cx| {
            for (_, mut item) in std::mem::take(&mut simple_cache.cache) {
                if let Some(Ok(image)) = item.get() {
                    cx.drop_image(image, None);
                }
            }
        })
        .detach();

        Self {
            max_items,
            usages: Vec::with_capacity(max_items),
            cache: HashMap::with_capacity(max_items),
        }
    }
}

impl ImageCache for SimpleLruCache {
    fn load(
        &mut self,
        resource: &gpui::Resource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Result<Arc<gpui::RenderImage>, gpui::ImageCacheError>> {
        assert_eq!(self.usages.len(), self.cache.len());
        assert!(self.cache.len() <= self.max_items);

        let hash = hash(resource);

        if let Some(item) = self.cache.get_mut(&hash) {
            let current_ix = self
                .usages
                .iter()
                .position(|item| *item == hash)
                .expect("cache and usages must stay in sync");
            self.usages.remove(current_ix);
            self.usages.insert(0, hash);

            return item.get();
        }

        let fut = AssetLogger::<ImageAssetLoader>::load(resource.clone(), cx);
        let task = cx.background_executor().spawn(fut).shared();
        if self.usages.len() == self.max_items {
            let oldest = self.usages.pop().unwrap();
            let mut image = self
                .cache
                .remove(&oldest)
                .expect("cache and usages must be in sync");
            if let Some(Ok(image)) = image.get() {
                cx.drop_image(image, Some(window));
            }
        }
        self.cache
            .insert(hash, gpui::ImageCacheItem::Loading(task.clone()));
        self.usages.insert(0, hash);

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
}

actions!(image, [Quit]);

fn main() {
    env_logger::init();

    Application::new().run(move |cx: &mut App| {
        let http_client = ReqwestClient::user_agent("gpui example").unwrap();
        cx.set_http_client(Arc::new(http_client));

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Image Gallery".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Image Gallery")),
                appears_transparent: false,
                ..Default::default()
            }),

            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1100.), px(860.)),
                cx,
            ))),

            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| {
            cx.new(|ctx| ImageGallery {
                image_key: "".into(),
                items_count: IMAGES_IN_GALLERY,
                total_count: 0,
                image_cache: HashMapImageCache::new(ctx),
            })
        })
        .unwrap();
    });
}
