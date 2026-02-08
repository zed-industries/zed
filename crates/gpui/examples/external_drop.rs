//! Minimal example for testing external drag-drop with files and URLs.
//!
//! Run with: cargo run -p gpui --example external_drop
//!
//! Test by:
//! 1. Dragging files from your file manager onto the window
//! 2. Dragging URLs from your browser onto the window
//! 3. Dragging mixed content (some browsers support this)

use gpui::{
    div, prelude::*, px, rgba, rgb, size, App, Application, Bounds, Context, DragMoveEvent,
    DragType, DropItem, ExternalDrop, SharedString, Window, WindowBounds, WindowOptions,
};
use smallvec::smallvec;

struct ExternalDropDemo {
    dropped_items: Vec<DroppedItem>,
    drag_hover: bool,
}

#[derive(Clone)]
enum DroppedItem {
    File(String),
    Url(String),
}

impl ExternalDropDemo {
    fn new() -> Self {
        Self {
            dropped_items: Vec::new(),
            drag_hover: false,
        }
    }
}

impl Render for ExternalDropDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items_display: Vec<SharedString> = self
            .dropped_items
            .iter()
            .map(|item| match item {
                DroppedItem::File(path) => format!("[FILE] {}", path).into(),
                DroppedItem::Url(url) => format!("[URL]  {}", url).into(),
            })
            .collect();

        div()
            .id("root")
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .p_4()
                    .child("External Drag-Drop Test")
                    .text_xl()
                    .border_b_1()
                    .border_color(rgb(0x444444)),
            )
            .child(
                div()
                    .p_4()
                    .text_sm()
                    .text_color(rgb(0x888888))
                    .child("Drag files or URLs from external apps onto the drop zone below"),
            )
            .child(
                div()
                    .id("drop-zone")
                    .m_4()
                    .p_4()
                    .min_h(px(200.0))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .border_2()
                    .border_dashed()
                    .rounded_lg()
                    .when(self.drag_hover, |el| {
                        el.border_color(rgb(0x4fc3f7)).bg(rgba(0x4fc3f71a))
                    })
                    .when(!self.drag_hover, |el| el.border_color(rgb(0x666666)))
                    .on_drop(cx.listener(|this, drop: &ExternalDrop, _window, _cx| {
                        for item in drop.items() {
                            match item {
                                DropItem::Path(path) => {
                                    this.dropped_items
                                        .push(DroppedItem::File(path.display().to_string()));
                                }
                                DropItem::Url(url) => {
                                    this.dropped_items
                                        .push(DroppedItem::Url(url.to_string()));
                                }
                            }
                        }
                        this.drag_hover = false;
                    }))
                    .on_drag_move(cx.listener(
                        |this, _: &DragMoveEvent<ExternalDrop>, _window, _cx| {
                            if !this.drag_hover {
                                this.drag_hover = true;
                            }
                        },
                    ))
                    .when(items_display.is_empty(), |el| {
                        el.justify_center().items_center().child(
                            div()
                                .text_color(rgb(0x666666))
                                .child("Drop files or URLs here"),
                        )
                    })
                    .when(!items_display.is_empty(), |el| {
                        el.children(items_display.into_iter().map(|text| {
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .bg(rgb(0x2d2d2d))
                                .text_sm()
                                .child(text)
                        }))
                    }),
            )
            .child(
                div()
                    .p_4()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .id("clear-button")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x444444))
                            .cursor_pointer()
                            .hover(|el| el.bg(rgb(0x555555)))
                            .on_click(cx.listener(|this, _, _window, _cx| {
                                this.dropped_items.clear();
                            }))
                            .child("Clear"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child(format!("{} items dropped", self.dropped_items.len())),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(400.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                drag_types: smallvec![DragType::Files, DragType::Urls],
                ..Default::default()
            },
            |_, cx| cx.new(|_| ExternalDropDemo::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
