//! Checks `Window::set_borderless` and `Window::set_has_shadow` against real AppKit windows.
//!
//! Requires a macOS graphical session. AppKit must run on the process's main thread,
//! so this target uses its own harness rather than libtest's worker threads, and it is
//! excluded from default test runs. Run with
//! `cargo test -p gpui_macos --test window_decorations`.

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
    use objc2::rc::Retained;
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
            if let Err(error) = check_decorations(cx).and_then(|()| check_focus(cx)) {
                eprintln!("window decoration checks failed: {error:#}");
                std::process::exit(1);
            }
            eprintln!("window decoration checks passed");
            cx.quit();
        });
    }

    fn window_options(
        kind: WindowKind,
        titlebar: Option<bool>,
        background: WindowBackgroundAppearance,
    ) -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(100.), px(100.)),
                size(px(200.), px(160.)),
            ))),
            kind,
            titlebar: titlebar.map(|appears_transparent| TitlebarOptions {
                appears_transparent,
                ..Default::default()
            }),
            window_background: background,
            focus: false,
            show: false,
            ..Default::default()
        }
    }

    fn native_view(window: &Window) -> Result<&NSView> {
        let handle = HasWindowHandle::window_handle(window)
            .map_err(|error| anyhow::anyhow!("could not get the native window handle: {error}"))?;
        let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
            anyhow::bail!("expected an AppKit window handle");
        };
        // SAFETY: GPUI hands out the pointer to its live NSView, and the window that owns
        // the view outlives the returned reference.
        Ok(unsafe { &*handle.ns_view.as_ptr().cast::<NSView>() })
    }

    fn check_decorations(cx: &mut App) -> Result<()> {
        for (name, kind, titlebar) in [
            ("untitled popup", WindowKind::PopUp, None),
            ("titled popup", WindowKind::PopUp, Some(false)),
            (
                "popup with a transparent titlebar",
                WindowKind::PopUp,
                Some(true),
            ),
            ("untitled normal window", WindowKind::Normal, None),
            ("titled normal window", WindowKind::Normal, Some(false)),
        ] {
            for background in [
                WindowBackgroundAppearance::Transparent,
                WindowBackgroundAppearance::Opaque,
                WindowBackgroundAppearance::Blurred,
            ] {
                let handle = cx.open_window(
                    window_options(kind.clone(), titlebar, background),
                    |_, cx| cx.new(|_| Content),
                )?;
                handle.update(cx, |_, window, _| {
                    let case = format!("{name} ({background:?})");
                    let native = native_view(window)?
                        .window()
                        .context("the content view has no window")?;
                    let initial_style_mask = native.styleMask();
                    ensure!(
                        initial_style_mask.contains(NSWindowStyleMask::Titled),
                        "{case}: the window should open with a native frame"
                    );
                    ensure!(
                        native.hasShadow(),
                        "{case}: the window should open with a native shadow"
                    );

                    window.set_borderless(true);
                    window.set_has_shadow(false);
                    ensure!(
                        !native.styleMask().contains(NSWindowStyleMask::Titled),
                        "{case}: set_borderless(true) left the native frame"
                    );
                    ensure!(
                        !native.hasShadow(),
                        "{case}: set_has_shadow(false) left the native shadow"
                    );
                    ensure!(
                        native
                            .styleMask()
                            .contains(NSWindowStyleMask::NonactivatingPanel)
                            == (kind == WindowKind::PopUp),
                        "{case}: set_borderless changed the activation policy"
                    );
                    ensure!(
                        native.canBecomeKeyWindow(),
                        "{case}: the borderless window cannot accept keyboard focus"
                    );
                    ensure!(
                        !native.isVisible(),
                        "{case}: a show=false window became visible"
                    );

                    window.set_borderless(false);
                    window.set_has_shadow(true);
                    ensure!(
                        native.styleMask() == initial_style_mask,
                        "{case}: set_borderless(false) did not restore the style mask"
                    );
                    ensure!(
                        native.hasShadow(),
                        "{case}: set_has_shadow(true) did not restore the native shadow"
                    );

                    window.remove_window();
                    anyhow::Ok(())
                })??;
            }
        }
        Ok(())
    }

    fn check_focus(cx: &mut App) -> Result<()> {
        let handle = cx.open_window(
            WindowOptions {
                focus: true,
                show: true,
                ..window_options(
                    WindowKind::PopUp,
                    None,
                    WindowBackgroundAppearance::Transparent,
                )
            },
            |_, cx| cx.new(|_| Content),
        )?;
        handle.update(cx, |_, window, _| {
            let view = native_view(window)?;
            let native = view.window().context("the content view has no window")?;
            ensure!(
                native.isKeyWindow(),
                "the focused window did not become key"
            );

            window.set_borderless(true);
            ensure!(
                native.isKeyWindow(),
                "set_borderless dropped the window's key status"
            );
            let first_responder = native
                .firstResponder()
                .context("set_borderless left the window without a first responder")?;
            ensure!(
                std::ptr::eq(Retained::as_ptr(&first_responder).cast::<NSView>(), view),
                "set_borderless moved the first responder off GPUI's view"
            );

            window.remove_window();
            anyhow::Ok(())
        })?
    }
}
