//! Requires a macOS graphical session. AppKit must run on the process's main thread,
//! so this target uses its own harness rather than libtest's worker threads.
//! Run with `cargo test -p gpui_macos --test popup_decorations`.

fn main() {
    #[cfg(target_os = "macos")]
    native::run();
}

#[cfg(target_os = "macos")]
mod native {
    use anyhow::{Context as _, Result, ensure};
    use gpui::{
        App, Application, Bounds, Context, TitlebarOptions, Window, WindowBackgroundAppearance,
        WindowBounds, WindowKind, WindowOptions, div, point, prelude::*, px, rgb, size,
    };
    use gpui_macos::MacPlatform;
    use objc2_app_kit::{NSView, NSWindowStyleMask};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::rc::Rc;

    struct Content;

    impl Render for Content {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().size_full().rounded_full().bg(rgb(0x3366ff))
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(MacPlatform::new(false))).run(|cx| {
            if let Err(error) = check_decorations(cx) {
                eprintln!("popup decoration checks failed: {error:#}");
                std::process::exit(1);
            }
            eprintln!("popup decoration checks passed");
            cx.quit();
        });
    }

    fn check_decorations(cx: &mut App) -> Result<()> {
        for (name, kind, titlebar, decorated) in [
            ("untitled popup", WindowKind::PopUp, None, false),
            ("titled popup", WindowKind::PopUp, Some(false), true),
            (
                "popup with custom titlebar",
                WindowKind::PopUp,
                Some(true),
                true,
            ),
            ("untitled normal window", WindowKind::Normal, None, true),
        ] {
            for background in [
                WindowBackgroundAppearance::Transparent,
                WindowBackgroundAppearance::Opaque,
                WindowBackgroundAppearance::Blurred,
            ] {
                let handle = cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                            point(px(100.), px(100.)),
                            size(px(200.), px(160.)),
                        ))),
                        kind: kind.clone(),
                        titlebar: titlebar.map(|appears_transparent| TitlebarOptions {
                            appears_transparent,
                            ..Default::default()
                        }),
                        window_background: background,
                        focus: false,
                        show: false,
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| Content),
                )?;

                handle.update(cx, |_, window, _| {
                    let handle = HasWindowHandle::window_handle(window)
                        .map_err(|error| anyhow::anyhow!("could not get native handle: {error}"))?;
                    let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
                        anyhow::bail!("expected an AppKit window");
                    };
                    // SAFETY: GPUI's raw handle refers to its live NSView on the main thread.
                    let view = unsafe { &*handle.ns_view.as_ptr().cast::<NSView>() };
                    let native = view.window().context("content view has no window")?;
                    ensure!(
                        native.styleMask().contains(NSWindowStyleMask::Titled) == decorated,
                        "{name} ({background:?}): unexpected native frame"
                    );
                    ensure!(
                        native.hasShadow() == decorated,
                        "{name} ({background:?}): unexpected native shadow"
                    );
                    ensure!(
                        native
                            .styleMask()
                            .contains(NSWindowStyleMask::NonactivatingPanel)
                            == (kind == WindowKind::PopUp),
                        "{name} ({background:?}): unexpected activation policy"
                    );
                    ensure!(!native.isVisible(), "show=false window became visible");
                    ensure!(!native.isKeyWindow(), "focus=false window stole focus");
                    ensure!(
                        native.canBecomeKeyWindow(),
                        "window cannot accept keyboard focus"
                    );
                    window.remove_window();
                    anyhow::Ok(())
                })??;
            }
        }
        Ok(())
    }
}
