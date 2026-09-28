use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[cfg(feature = "x11")]
use anyhow::Context as _;
use calloop::{EventLoop, LoopHandle};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, ClipboardItem, CursorStyle, DisplayEnvironment, DisplayId, DisplayMode,
    PlatformDisplay, PlatformKeyboardLayout, PlatformWindow, Task, WindowParams,
};
use gpui_util::ResultExt as _;

#[cfg(feature = "x11")]
use super::X11Client;
use super::{LinuxClient, LinuxCommon, LinuxKeyboardLayout};
#[cfg(feature = "wayland")]
use super::{WaylandClient, take_startup_activation_token_from_environment};

/// A connection to a display server.
enum Attached {
    #[cfg(feature = "wayland")]
    Wayland(WaylandClient),
    #[cfg(feature = "x11")]
    X11(X11Client),
}

impl Drop for Attached {
    fn drop(&mut self) {
        // `WaylandClient` removes its event-loop sources when dropped; `X11Client` is a shared
        // handle, so its sources are removed explicitly.
        #[cfg(feature = "x11")]
        #[allow(irrefutable_let_patterns, reason = "irrefutable without the wayland feature")]
        if let Attached::X11(client) = self {
            client.detach();
        }
    }
}

/// Evaluates `$body` with `$client` bound to the attached client, or returns `None` if headless.
macro_rules! with_attached {
    ($self:expr, $client:ident => $body:expr) => {
        match $self.0.attached.borrow().as_ref() {
            None => None,
            #[cfg(feature = "wayland")]
            Some(Attached::Wayland($client)) => Some($body),
            #[cfg(feature = "x11")]
            Some(Attached::X11($client)) => Some($body),
        }
    };
}

struct SwitchableClientState {
    common: Rc<RefCell<LinuxCommon>>,
    /// The single event loop, until `run` takes it.
    event_loop: RefCell<Option<EventLoop<'static, ()>>>,
    loop_handle: LoopHandle<'static, ()>,
    /// The attached display server; `None` while headless.
    attached: RefCell<Option<Attached>>,
    /// A `set_display_mode` request waiting for the event loop to apply it.
    pending_mode: RefCell<Option<DisplayMode>>,
    transition_waiter: RefCell<Option<oneshot::Sender<anyhow::Result<()>>>>,
    #[cfg(feature = "wayland")]
    startup_activation_token: RefCell<Option<String>>,
}

/// A Linux client that starts headless and can attach to and detach from a display server at
/// runtime.
///
/// Each attach chooses Wayland or X11 from the [`DisplayEnvironment`] it is given. One event loop
/// serves every mode: the foreground executor and power listener are registered on it once, and
/// the attached client adds and removes its own sources. While headless, the client reports no
/// displays and cannot open windows.
#[derive(Clone)]
pub(crate) struct SwitchableClient(Rc<SwitchableClientState>);

impl SwitchableClient {
    pub(crate) fn new() -> Self {
        #[cfg(feature = "wayland")]
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
            attached: RefCell::new(None),
            pending_mode: RefCell::new(None),
            transition_waiter: RefCell::new(None),
            #[cfg(feature = "wayland")]
            startup_activation_token: RefCell::new(startup_activation_token),
        }))
    }

    fn is_headless(&self) -> bool {
        self.0.attached.borrow().is_none()
    }

    fn has_windows(&self) -> bool {
        with_attached!(self, client => client.has_windows()).unwrap_or(false)
    }

    /// Applies a pending `set_display_mode` request.
    ///
    /// Runs between event-loop iterations, so no event source is mid-dispatch when the attached
    /// client's sources are added or removed.
    fn apply_pending_mode(&self) {
        let Some(mode) = self.0.pending_mode.borrow().clone() else {
            return;
        };
        let result = match mode {
            DisplayMode::Headless => {
                // `set_display_mode` rejects open windows, but one can be opened before this runs.
                if self.has_windows() {
                    Err(anyhow::anyhow!(
                        "a native window was opened while entering headless mode"
                    ))
                } else if with_attached!(self, client => client.has_window_resources())
                    .unwrap_or(false)
                {
                    // A closed window is still releasing resources bound to the connection, from
                    // a task that runs on this loop. Try again on a later iteration.
                    return;
                } else {
                    let attached = self.0.attached.borrow_mut().take();
                    drop(attached);
                    Ok(())
                }
            }
            DisplayMode::Windowed(environment) => self.attach(&environment).map(|attached| {
                *self.0.attached.borrow_mut() = Some(attached);
            }),
        };
        self.0.pending_mode.borrow_mut().take();
        if let Some(waiter) = self.0.transition_waiter.borrow_mut().take() {
            waiter.send(result).ok();
        }
    }

    fn attach(&self, environment: &DisplayEnvironment) -> anyhow::Result<Attached> {
        let loop_handle = self.0.loop_handle.clone();
        let common = self.0.common.clone();
        match environment.guess_compositor() {
            #[cfg(feature = "wayland")]
            "Wayland" => {
                let startup_activation_token = self.0.startup_activation_token.borrow().clone();
                let client = WaylandClient::attach(
                    loop_handle,
                    common,
                    Some(environment),
                    startup_activation_token,
                )?;
                self.0.startup_activation_token.borrow_mut().take();
                Ok(Attached::Wayland(client))
            }
            #[cfg(feature = "x11")]
            "X11" => {
                let display = environment
                    .x11_display
                    .as_ref()
                    .context("DISPLAY is not set")?
                    .to_str()
                    .context("DISPLAY is not valid UTF-8")?;
                Ok(Attached::X11(X11Client::attach(
                    loop_handle,
                    common,
                    Some(display),
                )?))
            }
            _ => anyhow::bail!("the environment names no Wayland or X11 display server"),
        }
    }
}

impl LinuxClient for SwitchableClient {
    fn compositor_name(&self) -> &'static str {
        with_attached!(self, client => client.compositor_name()).unwrap_or("headless")
    }

    fn with_common<R>(&self, function: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        function(&mut self.0.common.borrow_mut())
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        with_attached!(self, client => client.keyboard_layout())
            .unwrap_or_else(|| Box::new(LinuxKeyboardLayout::new("unknown".into())))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        with_attached!(self, client => client.displays()).unwrap_or_default()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        with_attached!(self, client => client.display(id)).flatten()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        with_attached!(self, client => client.primary_display()).flatten()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        with_attached!(self, client => client.open_window(handle, options)).unwrap_or_else(|| {
            Err(anyhow::anyhow!(
                "cannot open a window while headless; switch to a windowed display mode first"
            ))
        })
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        with_attached!(self, client => client.set_cursor_style(style));
    }

    fn hide_cursor_until_mouse_moves(&self) {
        with_attached!(self, client => client.hide_cursor_until_mouse_moves());
    }

    fn is_cursor_visible(&self) -> bool {
        with_attached!(self, client => client.is_cursor_visible()).unwrap_or(true)
    }

    fn open_uri(&self, uri: &str) {
        with_attached!(self, client => client.open_uri(uri));
    }

    fn reveal_path(&self, path: PathBuf) {
        with_attached!(self, client => client.reveal_path(path));
    }

    fn write_to_primary(&self, item: ClipboardItem) {
        with_attached!(self, client => client.write_to_primary(item));
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        with_attached!(self, client => client.write_to_clipboard(item));
    }

    fn read_from_primary(&self) -> Option<ClipboardItem> {
        with_attached!(self, client => client.read_from_primary()).flatten()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        with_attached!(self, client => client.read_from_clipboard()).flatten()
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        with_attached!(self, client => client.active_window()).flatten()
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        with_attached!(self, client => client.window_stack()).flatten()
    }

    fn window_identifier(
        &self,
    ) -> impl Future<Output = Option<ashpd::WindowIdentifier>> + Send + 'static {
        let identifier: Option<futures::future::BoxFuture<'static, _>> =
            with_attached!(self, client => Box::pin(client.window_identifier()) as _);
        async move {
            match identifier {
                Some(identifier) => identifier.await,
                None => None,
            }
        }
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .event_loop
            .borrow_mut()
            .take()
            .expect("App is already running");
        event_loop
            .run(None, &mut (), |_| self.apply_pending_mode())
            .log_err();
    }

    fn set_display_mode(&self, mode: DisplayMode) -> Task<anyhow::Result<()>> {
        if self.0.pending_mode.borrow().is_some() {
            return Task::ready(Err(anyhow::anyhow!(
                "a display mode transition is already in progress"
            )));
        }
        match &mode {
            DisplayMode::Headless if self.is_headless() => return Task::ready(Ok(())),
            DisplayMode::Headless if self.has_windows() => {
                return Task::ready(Err(anyhow::anyhow!(
                    "cannot enter headless mode while native windows are open"
                )));
            }
            // Stays on the current display server, even if the environment names another one.
            DisplayMode::Windowed(_) if !self.is_headless() => return Task::ready(Ok(())),
            DisplayMode::Headless | DisplayMode::Windowed(_) => {}
        }

        let (sender, receiver) = oneshot::channel();
        self.0.transition_waiter.borrow_mut().replace(sender);
        *self.0.pending_mode.borrow_mut() = Some(mode);
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
                    .map_err(|_| anyhow::anyhow!("display mode transition was canceled"))
                    .and_then(|result| result)
            })
    }
}

#[cfg(all(test, feature = "wayland"))]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        process::Command,
    };

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
