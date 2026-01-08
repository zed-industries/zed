use gpui::{
    App, Application, Bounds, Context, DefiniteLength, Entity, Render, ResizableSplitState,
    ResizableSplits, SplitResizeBehavior, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct ResizableSplitsExample {
    split_state: Entity<ResizableSplitState>,
}

impl ResizableSplitsExample {
    fn new(cx: &mut Context<Self>) -> Self {
        let split_state = cx.new(|cx| ResizableSplitState::new(3, cx));
        Self { split_state }
    }
}

impl Render for ResizableSplitsExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let red_pane = div()
            .size_full()
            .bg(rgb(0xE53935))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .text_xl()
            .child("Red Pane");

        let green_pane = div()
            .size_full()
            .bg(rgb(0x43A047))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .text_xl()
            .child("Green Pane");

        let blue_pane = div()
            .size_full()
            .bg(rgb(0x1E88E5))
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .text_xl()
            .child("Blue Pane");

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x1E1E1E))
            .child(
                div()
                    .p_4()
                    .text_color(rgb(0xFFFFFF))
                    .child("Resizable Splits Example - Drag the handles between panes to resize. Double-click to reset."),
            )
            .child(
                div()
                    .flex_grow()
                    .p_2()
                    .child(
                        ResizableSplits::new(
                            "example-splits",
                            vec![
                                DefiniteLength::Fraction(0.3),
                                DefiniteLength::Fraction(0.4),
                                DefiniteLength::Fraction(0.3),
                            ],
                        )
                        .with_resize_behavior(
                            vec![
                                SplitResizeBehavior::Resizable,
                                SplitResizeBehavior::Resizable,
                                SplitResizeBehavior::Resizable,
                            ],
                            &self.split_state,
                            cx,
                        )
                        .handle_color(rgb(0x424242).into())
                        .handle_hover_color(rgb(0x757575).into())
                        .child(red_pane)
                        .child(green_pane)
                        .child(blue_pane),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|cx| ResizableSplitsExample::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
