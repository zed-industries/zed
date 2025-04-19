use gpui::{
    App, AppContext, Application, Bounds, ClickEvent, Context, Entity, HashMapImageCache,
    KeyBinding, Menu, MenuItem, SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions,
    actions, div, image_cache, img, prelude::*, px, rgb, size,
};
use reqwest_client::ReqwestClient;
use std::sync::Arc;

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

        image_cache(&self.image_cache).child(
            div()
                .id("main")
                .font_family(".SystemUIFont")
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
        )
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
                items_count: 99,
                total_count: 0,
                image_cache: HashMapImageCache::new(ctx),
            })
        })
        .unwrap();
    });
}
