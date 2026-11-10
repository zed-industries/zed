//! A small GUI for trying out `sandbox_linux` policies by hand.
//!
//! ```sh
//! cargo run -p sandbox_linux_gui
//! ```

#[cfg(target_os = "linux")]
mod tester;
#[cfg(target_os = "linux")]
mod text_input;

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("sandbox_linux_gui only runs on Linux");
}

#[cfg(target_os = "linux")]
fn main() {
    use gpui::{App, AppContext as _, Bounds, WindowBounds, WindowOptions, px, size};

    // The sandbox re-executes this binary as its supervisor and helper.
    sandbox_linux::run_hidden_entrypoint_if_invoked();

    gpui_platform::application().run(|cx: &mut App| {
        text_input::bind_keys(cx);
        let bounds = Bounds::centered(None, size(px(960.), px(720.)), cx);
        let window = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(tester::Tester::new),
        );
        if let Err(error) = window {
            eprintln!("failed to open window: {error:#}");
            cx.quit();
            return;
        }
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.activate(true);
    });
}
