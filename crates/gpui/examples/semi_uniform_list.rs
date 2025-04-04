use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, semi_uniform_list,
};

struct SemiUniformListExample {}

impl Render for SemiUniformListExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0xffffff)).child(
            semi_uniform_list(
                cx.entity().clone(),
                "entries",
                50,
                |ix| {
                    // Alternate between tall and short items
                    if ix % 2 == 0 {
                        px(40.0) // Tall items
                    } else {
                        px(24.0) // Short items
                    }
                },
                |_this, range, _window, _cx| {
                    let mut items = Vec::new();
                    for ix in range {
                        let item = ix + 1;
                        let is_tall = ix % 2 == 0;
                        let height_text = if is_tall { "Tall" } else { "Short" };

                        items.push(
                            div()
                                .id(ix)
                                .px_2()
                                .bg(if is_tall { rgb(0xf5f5f5) } else { rgb(0xffffff) })
                                .cursor_pointer()
                                .on_click(move |_event, _window, _cx| {
                                    println!("clicked Item {item:?} ({height_text})");
                                })
                                .child(format!("Item {item} ({height_text})")),
                        );
                    }
                    items
                },
            )
            .h_full(),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| SemiUniformListExample {}),
        )
        .unwrap();
    });
}