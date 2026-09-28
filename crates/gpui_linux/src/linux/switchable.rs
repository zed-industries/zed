use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    rc::Rc,
};

use calloop::{EventLoop, LoopHandle};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, ClipboardItem, CursorStyle, DisplayId, PlatformDisplay,
    PlatformKeyboardLayout, PlatformWindow, Task, WindowParams,
};
use gpui_util::ResultExt as _;

use super::{
    HeadlessDisplay, HeadlessWindow, LinuxClient, LinuxCommon, LinuxKeyboardLayout, WaylandClient,
    take_startup_activation_token_from_environment,
};

struct SwitchableClientState {
    common: Rc<RefCell<LinuxCommon>>,
    /// The single event loop, until `run` takes it.
    event_loop: RefCell<Option<EventLoop<'static, ()>>>,
    loop_handle: LoopHandle<'static, ()>,
    /// The attached compositor connection; `None` while headless.
    wayland: RefCell<Option<WaylandClient>>,
    /// The mode `set_headless` last asked for. It differs from the current mode while a
    /// transition is waiting for the event loop to apply it.
    requested_headless: Cell<bool>,
    transition_waiter: RefCell<Option<oneshot::Sender<anyhow::Result<()>>>>,
    display: Rc<dyn PlatformDisplay>,
    startup_activation_token: RefCell<Option<String>>,
}

/// A Linux client that starts headless and can attach to and detach from Wayland at runtime.
///
/// One event loop serves both modes. The foreground executor and power listener are registered on
/// it once; attaching Wayland adds the compositor's sources, and detaching removes them.
#[derive(Clone)]
pub(crate) struct SwitchableClient(Rc<SwitchableClientState>);

impl SwitchableClient {
    pub(crate) fn new() -> Self {
        let startup_activation_token = take_startup_activation_token_from_environment();
        let event_loop = EventLoop::try_new().expect("failed to create Linux event loop");
        let (common, main_receiver, power_receiver) = LinuxCommon::new(event_loop.get_signal());
        let common = Rc::new(RefCell::new(common));
        let loop_handle = event_loop.handle();
        LinuxCommon::register_sources(&common, &loop_handle, main_receiver, power_receiver)
            .expect("failed to register Linux event sources");
        Self(Rc::new(SwitchableClientState {
            common,
            event_loop: RefCell::new(Some(event_loop)),
            loop_handle,
            wayland: RefCell::new(None),
            requested_headless: Cell::new(true),
            transition_waiter: RefCell::new(None),
            display: Rc::new(HeadlessDisplay::new()),
            startup_activation_token: RefCell::new(startup_activation_token),
        }))
    }

    fn is_headless(&self) -> bool {
        self.0.wayland.borrow().is_none()
    }

    fn with_wayland<R>(&self, function: impl FnOnce(&WaylandClient) -> R) -> Option<R> {
        let wayland = self.0.wayland.borrow();
        wayland.as_ref().map(function)
    }

    /// Applies a pending `set_headless` request.
    ///
    /// Runs between event-loop iterations, so no event source is mid-dispatch when Wayland's
    /// sources are added or removed.
    fn apply_requested_mode(&self) {
        let headless = self.0.requested_headless.get();
        if headless == self.is_headless() {
            return;
        }
        let result = if headless {
            // `set_headless` rejects open windows, but one can be opened before this runs.
            if self.with_wayland(WaylandClient::has_windows) == Some(true) {
                self.0.requested_headless.set(false);
                Err(anyhow::anyhow!(
                    "a native window was opened while entering headless mode"
                ))
            } else if self.with_wayland(WaylandClient::has_window_resources) == Some(true) {
                // A closed window is still releasing GPU resources bound to the connection, from
                // a task that runs on this loop. Try again on a later iteration.
                return;
            } else {
                let wayland = self.0.wayland.borrow_mut().take();
                drop(wayland);
                Ok(())
            }
        } else {
            let startup_activation_token = self.0.startup_activation_token.borrow().clone();
            match WaylandClient::attach(
                self.0.loop_handle.clone(),
                self.0.common.clone(),
                startup_activation_token,
            ) {
                Ok(wayland) => {
                    self.0.startup_activation_token.borrow_mut().take();
                    *self.0.wayland.borrow_mut() = Some(wayland);
                    Ok(())
                }
                Err(error) => {
                    self.0.requested_headless.set(true);
                    Err(error)
                }
            }
        };
        if let Some(waiter) = self.0.transition_waiter.borrow_mut().take() {
            waiter.send(result).ok();
        }
    }
}

impl LinuxClient for SwitchableClient {
    fn compositor_name(&self) -> &'static str {
        if self.is_headless() {
            "headless"
        } else {
            "Wayland"
        }
    }

    fn with_common<R>(&self, function: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        function(&mut self.0.common.borrow_mut())
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        self.with_wayland(LinuxClient::keyboard_layout)
            .unwrap_or_else(|| Box::new(LinuxKeyboardLayout::new("unknown".into())))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        self.with_wayland(LinuxClient::displays)
            .unwrap_or_else(|| vec![self.0.display.clone()])
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        self.with_wayland(|wayland| wayland.display(id))
            .flatten()
            .or_else(|| {
                let display = self.0.display.clone();
                (display.id() == id).then_some(display)
            })
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        self.with_wayland(LinuxClient::primary_display)
            .flatten()
            .or_else(|| Some(self.0.display.clone()))
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        if let Some(wayland) = self.0.wayland.borrow().as_ref() {
            return wayland.open_window(handle, options);
        }
        Ok(Box::new(HeadlessWindow::new(
            options,
            self.0.display.clone(),
        )))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        self.with_wayland(|wayland| wayland.set_cursor_style(style));
    }

    fn hide_cursor_until_mouse_moves(&self) {
        self.with_wayland(LinuxClient::hide_cursor_until_mouse_moves);
    }

    fn is_cursor_visible(&self) -> bool {
        self.with_wayland(LinuxClient::is_cursor_visible)
            .unwrap_or(true)
    }

    fn open_uri(&self, uri: &str) {
        self.with_wayland(|wayland| wayland.open_uri(uri));
    }

    fn reveal_path(&self, path: PathBuf) {
        self.with_wayland(|wayland| wayland.reveal_path(path));
    }

    fn write_to_primary(&self, item: ClipboardItem) {
        self.with_wayland(|wayland| wayland.write_to_primary(item));
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        self.with_wayland(|wayland| wayland.write_to_clipboard(item));
    }

    fn read_from_primary(&self) -> Option<ClipboardItem> {
        self.with_wayland(LinuxClient::read_from_primary).flatten()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.with_wayland(LinuxClient::read_from_clipboard)
            .flatten()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        self.with_wayland(LinuxClient::active_window).flatten()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        self.with_wayland(LinuxClient::window_stack).flatten()
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .event_loop
            .borrow_mut()
            .take()
            .expect("App is already running");
        event_loop
            .run(None, &mut (), |_| self.apply_requested_mode())
            .log_err();
    }

    fn set_headless(&self, headless: bool) -> Task<anyhow::Result<()>> {
        if self.0.requested_headless.get() != self.is_headless() {
            return Task::ready(Err(anyhow::anyhow!(
                "a display backend transition is already in progress"
            )));
        }
        if headless == self.is_headless() {
            return Task::ready(Ok(()));
        }
        if headless
            && self
                .with_wayland(WaylandClient::has_windows)
                .unwrap_or(false)
        {
            return Task::ready(Err(anyhow::anyhow!(
                "cannot enter headless mode while native windows are open"
            )));
        }

        let (sender, receiver) = oneshot::channel();
        self.0.transition_waiter.borrow_mut().replace(sender);
        self.0.requested_headless.set(headless);
        // The wakeup persists until the loop next polls, so a request made before `run` starts is
        // applied after its first iteration.
        self.0.common.borrow().signal.wakeup();
        self.0
            .common
            .borrow()
            .foreground_executor
            .spawn(async move {
                receiver
                    .await
                    .map_err(|_| anyhow::anyhow!("display backend transition was canceled"))
                    .and_then(|result| result)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, process::Command};

    use gpui::{AppContext as _, Application, QuitMode};

    use super::*;
    use crate::linux::LinuxPlatform;

    #[test]
    fn failed_wayland_attach_preserves_headless_app_and_allows_retry() {
        const CHILD_ENV: &str = "GPUI_FAILED_WAYLAND_ATTACH_TEST_CHILD";

        if std::env::var_os(CHILD_ENV).is_some() {
            run_failed_wayland_attach_scenario();
            return;
        }

        let test_name = format!(
            "{}::failed_wayland_attach_preserves_headless_app_and_allows_retry",
            module_path!()
        );
        #[allow(
            clippy::disallowed_methods,
            reason = "the test thread has nothing else to do while the child runs"
        )]
        let output = Command::new(std::env::current_exe().expect("current test executable"))
            .args(["--exact", &test_name, "--nocapture"])
            .env(CHILD_ENV, "1")
            .env(
                "WAYLAND_DISPLAY",
                "/gpui-test/wayland-display-does-not-exist",
            )
            .output()
            .expect("run isolated invalid-display scenario");

        assert!(
            output.status.success(),
            "invalid-display scenario failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    fn run_failed_wayland_attach_scenario() {
        let platform = Rc::new(LinuxPlatform {
            inner: SwitchableClient::new(),
        });
        let outcome = Rc::new(RefCell::new(None));
        let queued_task_ran = Rc::new(Cell::new(false));

        Application::with_platform(platform)
            .with_quit_mode(QuitMode::Explicit)
            .run({
                let outcome = outcome.clone();
                let queued_task_ran = queued_task_ran.clone();
                move |cx| {
                    let entity = cx.new(|_| 1usize);
                    // Requested before the event loop starts running.
                    let first_attach = cx.set_headless(false);
                    cx.spawn(async move |cx| {
                        let result = async {
                            entity.update(cx, |value, _| *value += 1);
                            let (release_task, wait_for_release) = oneshot::channel();
                            let queued_task = cx.spawn(async move |_| {
                                wait_for_release
                                    .await
                                    .expect("queued task release sender remains alive");
                                queued_task_ran.set(true);
                            });

                            assert_connection_error(first_attach.await);
                            assert_eq!(entity.read_with(cx, |value, _| *value), 2);
                            release_task
                                .send(())
                                .expect("queued task remains attached to the executor");
                            queued_task.await;

                            let second_attach = cx.update(|cx| cx.set_headless(false));
                            assert_connection_error(second_attach.await);
                            entity.update(cx, |value, _| *value += 1);
                            assert_eq!(entity.read_with(cx, |value, _| *value), 3);
                            anyhow::Ok(())
                        }
                        .await;
                        *outcome.borrow_mut() = Some(result);
                        cx.update(|cx| cx.quit());
                        anyhow::Ok(())
                    })
                    .detach();
                }
            });

        assert!(queued_task_ran.get());
        outcome
            .borrow_mut()
            .take()
            .expect("scenario completed")
            .expect("headless app survived failed Wayland attachments");
    }

    fn assert_connection_error(result: anyhow::Result<()>) {
        let error = result.expect_err("attaching to a missing Wayland display fails");
        assert!(
            format!("{error:#}").contains("failed to connect to Wayland compositor"),
            "unexpected error: {error:#}"
        );
    }
}
