//! Requires a macOS graphical session. AppKit must run on the process's main thread,
//! so this target uses its own harness rather than libtest's worker threads.
//! Run with `cargo test -p gpui_macos --features test-support --test native_popup`.

fn main() {
    #[cfg(target_os = "macos")]
    native::run();
}

#[cfg(target_os = "macos")]
mod native {
    use anyhow::{Context as _, Result, ensure};
    use futures::{FutureExt as _, pin_mut, select_biased};
    use gpui::{
        App, Application, AsyncApp, Bounds, Context, FocusHandle, Window, WindowBounds,
        WindowHandle, WindowKind, WindowOptions, div, point, popup::*, prelude::*, px, size,
    };
    use gpui_macos::MacPlatform;
    use objc2::rc::Retained;
    use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventType, NSView, NSWindow};
    use objc2_foundation::{NSPoint, NSString};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use std::{rc::Rc, time::Duration};

    struct Content {
        focus: FocusHandle,
        consume_escape: bool,
    }

    impl Render for Content {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div().track_focus(&self.focus).on_key_down(cx.listener(
                |this, event: &gpui::KeyDownEvent, _, cx| {
                    if this.consume_escape && event.keystroke.key == "escape" {
                        cx.stop_propagation();
                    }
                },
            ))
        }
    }

    pub fn run() {
        Application::with_platform(Rc::new(MacPlatform::new(false))).run(|cx| {
            cx.spawn(async |cx| {
                let result = {
                    let timeout = cx
                        .background_executor()
                        .timer(Duration::from_secs(30))
                        .fuse();
                    let tests = async {
                        hidden_popup_lifecycle(false, cx).await?;
                        hidden_popup_lifecycle(true, cx).await?;
                        escape_dismissal(cx).await?;
                        anyhow::Ok(())
                    }
                    .fuse();
                    pin_mut!(timeout, tests);
                    select_biased! {
                        result = tests => result,
                        _ = timeout => Err(anyhow::anyhow!("native popup tests timed out")),
                    }
                };
                if let Err(error) = result {
                    eprintln!("native popup regression checks failed: {error:#}");
                    std::process::exit(1);
                }
                eprintln!("native popup regression checks passed");
                cx.update(|cx| cx.quit());
            })
            .detach();
        });
    }

    fn open(options: WindowOptions, cx: &mut App) -> Result<WindowHandle<Content>> {
        cx.open_window(options, |window, cx| {
            cx.new(|cx| {
                let focus = cx.focus_handle();
                focus.focus(window, cx);
                Content {
                    focus,
                    consume_escape: false,
                }
            })
        })
    }

    fn popup_options(parent: WindowHandle<Content>, grab: bool) -> WindowOptions {
        WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(0.), px(0.)),
                size(px(180.), px(120.)),
            ))),
            kind: WindowKind::AnchoredPopup(PopupOptions {
                parent: parent.into(),
                anchor_rect: Bounds::new(point(px(20.), px(40.)), size(px(70.), px(25.))),
                anchor: PopupAnchor::BottomLeft,
                gravity: PopupGravity::BottomRight,
                constraint_adjustment: PopupConstraintAdjustment::all(),
                offset: point(px(0.), px(4.)),
                grab,
            }),
            ..Default::default()
        }
    }

    fn native_window(
        handle: WindowHandle<Content>,
        cx: &mut AsyncApp,
    ) -> Result<Retained<NSWindow>> {
        handle.update(cx, |_, window, _| {
            let handle = HasWindowHandle::window_handle(window)
                .map_err(|error| anyhow::anyhow!("could not get native window handle: {error}"))?;
            let RawWindowHandle::AppKit(handle) = handle.as_raw() else {
                anyhow::bail!("expected an AppKit window");
            };
            // SAFETY: GPUI's raw handle refers to its live NSView; we are on the main thread.
            let view = unsafe { &*handle.ns_view.as_ptr().cast::<NSView>() };
            view.window().context("content view has no native window")
        })?
    }

    async fn hidden_popup_lifecycle(grab: bool, cx: &mut AsyncApp) -> Result<()> {
        let parent = cx.update(|cx| open(WindowOptions::default(), cx))?;
        let parent_native = native_window(parent, cx)?;
        ensure!(parent_native.isVisible(), "test parent must be visible");
        let hidden = cx.update(|cx| {
            open(
                WindowOptions {
                    show: false,
                    ..popup_options(parent, grab)
                },
                cx,
            )
        })?;
        let hidden_native = native_window(hidden, cx)?;
        ensure!(
            !hidden_native.isVisible(),
            "show=false popup became visible"
        );
        ensure!(!hidden_native.isKeyWindow(), "hidden popup stole focus");

        hidden.update(cx, |_, window, _| window.activate_window())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            hidden_native.isVisible(),
            "explicit activation did not show popup"
        );
        ensure!(
            hidden_native.parentWindow().as_ref() == Some(&parent_native),
            "shown popup lost its parent"
        );
        hidden.update(cx, |_, window, _| window.remove_window())?;
        cx.spawn(async |_| {}).await;

        let hidden = cx.update(|cx| {
            open(
                WindowOptions {
                    show: false,
                    ..popup_options(parent, grab)
                },
                cx,
            )
        })?;
        parent.update(cx, |_, window, _| window.remove_window())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            !cx.update(|cx| cx.windows().contains(&hidden.into())),
            "parent close leaked hidden popup"
        );
        Ok(())
    }

    fn escape(window: &NSWindow, modifiers: NSEventModifierFlags) -> Result<()> {
        let characters = NSString::from_str("\u{1b}");
        let event = NSEvent::keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode(
            NSEventType::KeyDown, NSPoint::ZERO, modifiers, 0., window.windowNumber(), None,
            &characters, &characters, false, 53,
        ).context("could not construct Escape event")?;
        window.sendEvent(&event);
        Ok(())
    }

    async fn escape_dismissal(cx: &mut AsyncApp) -> Result<()> {
        let parent = cx.update(|cx| open(WindowOptions::default(), cx))?;
        // There is no triggering NSEvent: accessibility and programmatic opens must work too.
        let popup = cx.update(|cx| open(popup_options(parent, true), cx))?;
        let native = native_window(popup, cx)?;
        popup.update(cx, |content, _, _| content.consume_escape = true)?;
        cx.update_window(popup.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
        })?;

        escape(&native, NSEventModifierFlags::empty())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            cx.update(|cx| cx.windows().contains(&popup.into())),
            "consumed Escape closed popup"
        );

        popup.update(cx, |content, _, _| content.consume_escape = false)?;
        escape(&native, NSEventModifierFlags::Shift)?;
        cx.spawn(async |_| {}).await;
        ensure!(
            cx.update(|cx| cx.windows().contains(&popup.into())),
            "modified Escape closed popup"
        );

        escape(&native, NSEventModifierFlags::empty())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            !cx.update(|cx| cx.windows().contains(&popup.into())),
            "unhandled Escape did not close popup"
        );

        let passive = cx.update(|cx| open(popup_options(parent, false), cx))?;
        let native = native_window(passive, cx)?;
        escape(&native, NSEventModifierFlags::empty())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            cx.update(|cx| cx.windows().contains(&passive.into())),
            "Escape closed passive popup"
        );
        parent.update(cx, |_, window, _| window.remove_window())?;
        cx.spawn(async |_| {}).await;
        ensure!(
            cx.update(|cx| cx.windows().is_empty()),
            "native test leaked windows"
        );
        Ok(())
    }
}
