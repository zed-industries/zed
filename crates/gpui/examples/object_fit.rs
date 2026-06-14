#![cfg_attr(target_family = "wasm", no_main)]

use std::sync::Arc;

use gpui::{
    App, AppContext, Bounds, Context, KeyBinding, Menu, MenuItem, ObjectFit, Point, SharedString,
    TitlebarOptions, Window, WindowBounds, WindowOptions, actions, div, img, prelude::*, px, rgb,
    size,
};
#[cfg(not(target_family = "wasm"))]
use reqwest_client::ReqwestClient;

struct ObjectFitShowcase {
    image: SharedString,
}

impl ObjectFitShowcase {
    fn fit_demo(&self, label: &str, object_fit: ObjectFit) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(label.to_string())
            .child(
                img(self.image.clone())
                    .overflow_hidden()
                    .w(px(220.))
                    .h(px(140.))
                    .border_2()
                    .border_color(rgb(0xC0392B))
                    .rounded(px(24.))
                    .object_fit(object_fit),
            )
    }
}

impl Render for ObjectFitShowcase {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("main")
            .bg(gpui::white())
            .overflow_y_scroll()
            .p_8()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_6()
                    .child("ObjectFit with rounded corners")
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .justify_center()
                            .gap_8()
                            .child(self.fit_demo("Fill", ObjectFit::Fill))
                            .child(self.fit_demo("Contain", ObjectFit::Contain))
                            .child(self.fit_demo("Cover", ObjectFit::Cover))
                            .child(self.fit_demo("ScaleDown", ObjectFit::ScaleDown))
                            .child(self.fit_demo("None", ObjectFit::None)),
                    ),
            )
    }
}

actions!(object_fit, [Quit]);

fn run_example() {
    #[cfg(not(target_family = "wasm"))]
    let app = gpui_platform::application();
    #[cfg(target_family = "wasm")]
    let app = gpui_platform::single_threaded_web();

    app.run(move |cx: &mut App| {
        #[cfg(not(target_family = "wasm"))]
        {
            let http_client = ReqwestClient::user_agent("gpui example").unwrap();
            cx.set_http_client(Arc::new(http_client));
        }
        #[cfg(target_family = "wasm")]
        {
            // Safety: the web examples run single-threaded; the client is
            // created and used exclusively on the main thread.
            let http_client = unsafe {
                gpui_web::FetchHttpClient::with_user_agent("gpui example")
                    .expect("failed to create FetchHttpClient")
            };
            cx.set_http_client(Arc::new(http_client));
        }

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Object Fit".into(),
            items: vec![MenuItem::action("Quit", Quit)],
            disabled: false,
        }]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Object Fit Example")),
                appears_transparent: false,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                size: size(px(1200.), px(700.)),
                origin: Point::new(px(200.), px(200.)),
            })),
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| {
            cx.new(|_| ObjectFitShowcase {
                image: "https://picsum.photos/id/237/400/200".into(),
            })
        })
        .unwrap();
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    env_logger::init();
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
