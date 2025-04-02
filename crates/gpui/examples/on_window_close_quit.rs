use gpui::{
    App, Application, Bounds, Context, FocusHandle, KeyBinding, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, rgb, size,
};

actions!(example, [CloseWindow]);

struct ExampleWindow {
    focus_handle: FocusHandle,
}

impl Render for ExampleWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(|_: &CloseWindow, window, _| {
                window.remove_window();
            })
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                "Closing this window with cmd-w or the traffic lights should quit the application!",
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let mut bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.bind_keys([KeyBinding::new("cmd-w", CloseWindow, None)]);
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.activate(false);
                cx.new(|cx| {
                    let focus_handle = cx.focus_handle();
                    focus_handle.focus(window);
                    ExampleWindow { focus_handle }
                })
            },
        )
        .unwrap();

        bounds.origin.x += bounds.size.width;

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let focus_handle = cx.focus_handle();
                    focus_handle.focus(window);
                    ExampleWindow { focus_handle }
                })
            },
        )
        .unwrap();
    });
}
