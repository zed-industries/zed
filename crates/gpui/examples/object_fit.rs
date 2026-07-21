#![cfg_attr(target_family = "wasm", no_main)]

use std::sync::Arc;

use gpui::{
    App, AppContext, Bounds, Context, KeyBinding, Menu, MenuItem, ObjectFit, Point, SharedString,
    TitlebarOptions, Window, WindowBounds, WindowOptions, actions, div, img, prelude::*, px, rgb,
    size,
};
#[cfg(not(target_family = "wasm"))]
use reqwest_client::ReqwestClient;

struct Source {
    label: &'static str,
    image: SharedString,
}

struct ObjectFitShowcase {
    sources: Vec<Source>,
}

impl ObjectFitShowcase {
    fn fit_demo(image: &SharedString, label: &str, object_fit: ObjectFit) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(label.to_string())
            .child(
                img(image.clone())
                    .overflow_hidden()
                    .w(px(220.))
                    .h(px(140.))
                    .border_2()
                    .border_color(rgb(0xC0392B))
                    .rounded(px(24.))
                    .object_fit(object_fit),
            )
    }

    fn source_row(source: &Source) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(source.label.to_string())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_center()
                    .gap_8()
                    .child(Self::fit_demo(&source.image, "Fill", ObjectFit::Fill))
                    .child(Self::fit_demo(&source.image, "Contain", ObjectFit::Contain))
                    .child(Self::fit_demo(&source.image, "Cover", ObjectFit::Cover))
                    .child(Self::fit_demo(
                        &source.image,
                        "ScaleDown",
                        ObjectFit::ScaleDown,
                    ))
                    .child(Self::fit_demo(&source.image, "None", ObjectFit::None)),
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
                    .gap_8()
                    .child("ObjectFit with rounded corners")
                    .children(self.sources.iter().map(Self::source_row)),
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
                sources: vec![
                    Source {
                        label: "Landscape source (400x200)",
                        image: "https://picsum.photos/id/237/400/200".into(),
                    },
                    Source {
                        label: "Portrait source (200x400)",
                        image: "https://picsum.photos/id/1003/200/400".into(),
                    },
                    Source {
                        label: "Square source (400x400)",
                        image: "https://picsum.photos/id/1025/400/400".into(),
                    },
                    Source {
                        label: "Small source (120x80)",
                        image: "https://picsum.photos/id/1062/120/80".into(),
                    },
                ],
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
