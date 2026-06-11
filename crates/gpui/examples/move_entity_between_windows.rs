//! An entity registers callbacks via the `_in` API family and then gets
//! re-hosted in a new window via a click. The point of the example is to
//! demonstrate that callbacks dispatched after the move correctly target the
//! entity's *current* window rather than the window it was in at
//! registration time.
//!
//! To run:  cargo run -p gpui --example move_entity_between_windows

#![cfg_attr(target_family = "wasm", no_main)]

use std::time::Duration;

use gpui::{
    App, AppContext as _, Bounds, Context, EventEmitter, MouseButton, Render, SharedString,
    Subscription, Task, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

struct MoveToNewWindow;

struct HelloWorld {
    text: SharedString,
    tick_count: u32,
    move_count: u32,
    _tasks: Vec<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<MoveToNewWindow> for HelloWorld {}

impl HelloWorld {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let self_entity = cx.entity();

        let task = cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let result = this.update_in(cx, |this, window, _cx| {
                    this.tick_count += 1;
                    println!(
                        "tick #{} fired in entity's current window {}",
                        this.tick_count,
                        window.window_handle().window_id().as_u64(),
                    );
                });
                if let Err(err) = result {
                    println!("tick task giving up: {err}");
                    return;
                }
            }
        });

        let subscription = cx.subscribe_in::<_, MoveToNewWindow>(
            &self_entity,
            window,
            move |this, _emitter, _event, window, cx| {
                let entered_window_id = window.window_handle().window_id().as_u64();
                println!(
                    "MoveToNewWindow handler fired in entity's current window {entered_window_id}",
                );

                this.move_count += 1;
                cx.notify();

                let entity = cx.entity();
                let old_window = window.window_handle();
                cx.defer(move |cx| {
                    let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
                    cx.open_window(
                        WindowOptions {
                            window_bounds: Some(WindowBounds::Windowed(bounds)),
                            ..Default::default()
                        },
                        move |_, _| entity,
                    )
                    .expect("failed to open new window");
                    old_window
                        .update(cx, |_, window, _| window.remove_window())
                        .ok();
                });
            },
        );

        Self {
            text: "World".into(),
            tick_count: 0,
            move_count: 0,
            _tasks: vec![task],
            _subscriptions: vec![subscription],
        }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_id = window.window_handle().window_id().as_u64();

        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(format!("Rendering in window: {window_id}"))
            .child(format!("Ticks observed by entity: {}", self.tick_count))
            .child(format!("Moves observed by entity: {}", self.move_count))
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x4040ff))
                    .rounded_md()
                    .child("Move me to a new window")
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|_this, _, _window, cx| {
                            cx.emit(MoveToNewWindow);
                        }),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| HelloWorld::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
