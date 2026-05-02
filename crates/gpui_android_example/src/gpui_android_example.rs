//! Minimal "Hello, GPUI on Android!" demo that proves the gpui_android
//! platform crate compiles and links into a real `cdylib`.
//!
//! Bootstrap:
//!
//! ```text
//! cargo build --release --target aarch64-linux-android -p gpui_android_example
//! ```
//!
//! …then drop the resulting `target/aarch64-linux-android/release/libgpui_android_example.so`
//! into a Gradle project's `app/src/main/jniLibs/arm64-v8a/` directory and
//! launch a `GameActivity` that calls `System.loadLibrary("gpui_android_example")`.
#![cfg(target_os = "android")]

use gpui::{
    App, Bounds, Context, Render, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct HelloAndroid {
    text: SharedString,
}

impl Render for HelloAndroid {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x202020))
            .text_color(rgb(0xffffff))
            .text_3xl()
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(rgb(0xff5252)).rounded_md())
                    .child(div().size_8().bg(rgb(0x52ff52)).rounded_md())
                    .child(div().size_8().bg(rgb(0x5252ff)).rounded_md()),
            )
    }
}

/// Entry point invoked by the `android-activity` glue once the JVM-side
/// `GameActivity` thread has been spawned. Marked `#[unsafe(no_mangle)]` so
/// the dynamic loader can find it; rust-edition 2024 makes this attribute
/// explicitly unsafe.
#[unsafe(no_mangle)]
fn android_main(app: gpui_android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    log::info!("gpui_android_example: android_main entered");

    gpui_android::set_android_app(app);

    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(360.), px(640.)), cx);
        if let Err(error) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloAndroid {
                    text: "Android".into(),
                })
            },
        ) {
            log::error!("failed to open Android window: {error:#}");
        }
        cx.activate(true);
    });
}
