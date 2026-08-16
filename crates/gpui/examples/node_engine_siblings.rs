#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, Render, StyleRefinement, Task, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_platform::application;
use std::time::Duration;

struct Leaf {
    label: &'static str,
    color: u32,
    ticks: usize,
    render_count: usize,
    _tick_task: Option<Task<()>>,
}

impl Leaf {
    fn new(label: &'static str, color: u32) -> Self {
        Self {
            label,
            color,
            ticks: 0,
            render_count: 0,
            _tick_task: None,
        }
    }

    fn new_ticking(cx: &mut Context<Self>) -> Self {
        let tick_task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                if this
                    .update(cx, |leaf, cx| {
                        leaf.ticks += 1;
                        cx.notify();
                    })
                    .is_err()
                {
                    break;
                }
            }
        });
        Self {
            label: "notified leaf",
            color: 0x3a7d44,
            ticks: 0,
            render_count: 0,
            _tick_task: Some(tick_task),
        }
    }
}

impl Render for Leaf {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.render_count += 1;
        println!("{} render #{}", self.label, self.render_count);
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(self.color))
            .text_color(gpui::white())
            .child(self.label)
            .child(format!("ticks: {}", self.ticks))
    }
}

struct Siblings {
    left: gpui::Entity<Leaf>,
    middle: gpui::Entity<Leaf>,
    right: gpui::Entity<Leaf>,
}

impl Render for Siblings {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let leaf_style = || StyleRefinement::default().w(px(180.)).h(px(180.));
        div()
            .flex()
            .flex_row()
            .gap_4()
            .size_full()
            .items_center()
            .justify_center()
            .child(self.left.clone().cached(leaf_style()))
            .child(self.middle.clone().cached(leaf_style()))
            .child(self.right.clone().cached(leaf_style()))
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(620.), px(240.)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| Siblings {
                    left: cx.new(|_| Leaf::new("clean left", 0x6b4f9f)),
                    middle: cx.new(Leaf::new_ticking),
                    right: cx.new(|_| Leaf::new("clean right", 0x9f5f4f)),
                })
            },
        );
        if let Err(error) = result {
            log::error!("failed to open node engine example window: {error}");
            return;
        }
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
