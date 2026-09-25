use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    rc::Rc,
};

use calloop::{Dispatcher, EventLoop, channel::Channel};
use futures::channel::oneshot;
use gpui::{
    AnyWindowHandle, ClipboardItem, CursorStyle, DisplayId, PlatformDisplay,
    PlatformKeyboardLayout, PlatformWindow, RunnableVariant, Task, WindowParams,
};
use gpui_util::ResultExt as _;

use super::{
    HeadlessDisplay, HeadlessWindow, LinuxClient, LinuxCommon, LinuxKeyboardLayout,
    PriorityQueueCalloopReceiver, SystemPowerEvent, WaylandClient, WaylandServices,
    take_startup_activation_token_from_environment,
};

struct HeadlessRuntime {
    event_loop: EventLoop<'static, SwitchableClient>,
    main_receiver: PriorityQueueCalloopReceiver<RunnableVariant>,
    power_receiver: Channel<SystemPowerEvent>,
}

struct SwitchableClientState {
    common: Rc<RefCell<LinuxCommon>>,
    headless_runtime: RefCell<Option<HeadlessRuntime>>,
    wayland: RefCell<Option<WaylandClient>>,
    headless: Cell<bool>,
    requested_headless: Cell<bool>,
    transition_waiter: RefCell<Option<oneshot::Sender<anyhow::Result<()>>>>,
    quitting: Cell<bool>,
    display: Rc<dyn PlatformDisplay>,
    startup_activation_token: RefCell<Option<String>>,
}

#[derive(Clone)]
pub(crate) struct SwitchableClient(Rc<SwitchableClientState>);

impl SwitchableClient {
    pub(crate) fn new() -> Self {
        let startup_activation_token = take_startup_activation_token_from_environment();
        let event_loop = EventLoop::try_new().expect("failed to create Linux event loop");
        let (common, main_receiver, power_receiver) = LinuxCommon::new(event_loop.get_signal());
        Self(Rc::new(SwitchableClientState {
            common: Rc::new(RefCell::new(common)),
            headless_runtime: RefCell::new(Some(HeadlessRuntime {
                event_loop,
                main_receiver,
                power_receiver,
            })),
            wayland: RefCell::new(None),
            headless: Cell::new(true),
            requested_headless: Cell::new(true),
            transition_waiter: RefCell::new(None),
            quitting: Cell::new(false),
            display: Rc::new(HeadlessDisplay::new()),
            startup_activation_token: RefCell::new(startup_activation_token),
        }))
    }

    fn run_headless(
        &self,
        mut runtime: HeadlessRuntime,
    ) -> (
        PriorityQueueCalloopReceiver<RunnableVariant>,
        Channel<SystemPowerEvent>,
    ) {
        self.0.common.borrow_mut().signal = runtime.event_loop.get_signal();
        let handle = runtime.event_loop.handle();
        let main_dispatcher = Dispatcher::new(
            runtime.main_receiver,
            |event, _, _: &mut SwitchableClient| {
                if let calloop::channel::Event::Msg(runnable) = event {
                    runnable.run();
                }
            },
        );
        let main_registration = handle
            .register_dispatcher(main_dispatcher.clone())
            .expect("failed to register foreground executor");
        let power_dispatcher = Dispatcher::new(
            runtime.power_receiver,
            |event, _, client: &mut SwitchableClient| {
                if let calloop::channel::Event::Msg(event) = event {
                    client
                        .0
                        .common
                        .borrow_mut()
                        .handle_system_power_event(event);
                }
            },
        );
        let power_registration = handle
            .register_dispatcher(power_dispatcher.clone())
            .expect("failed to register power listener");

        runtime
            .event_loop
            .run(None, &mut self.clone(), |_| {})
            .log_err();

        handle.remove(main_registration);
        handle.remove(power_registration);
        drop(runtime.event_loop);
        (
            main_dispatcher.into_source_inner(),
            power_dispatcher.into_source_inner(),
        )
    }

    fn with_wayland<R>(&self, function: impl FnOnce(&WaylandClient) -> R) -> Option<R> {
        let wayland = self.0.wayland.borrow();
        wayland.as_ref().map(function)
    }

    fn finish_transition(&self, result: anyhow::Result<()>) {
        if let Some(waiter) = self.0.transition_waiter.borrow_mut().take() {
            waiter.send(result).unwrap_or(());
        }
    }
}

impl LinuxClient for SwitchableClient {
    fn compositor_name(&self) -> &'static str {
        if self.0.headless.get() {
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
        if self.0.wayland.borrow().is_some() {
            self.with_wayland(|wayland| wayland.open_window(handle, options))
                .expect("Wayland client disappeared while opening a window")
        } else {
            Ok(Box::new(HeadlessWindow::new(
                options,
                self.0.display.clone(),
            )))
        }
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
        let mut runtime = self
            .0
            .headless_runtime
            .borrow_mut()
            .take()
            .expect("App is already running");

        loop {
            let (main_receiver, power_receiver) = self.run_headless(runtime);
            if self.0.quitting.get() {
                break;
            }
            if self.0.requested_headless.get() {
                break;
            }

            let wayland = match WaylandClient::new_with_services(
                Some(WaylandServices {
                    common: self.0.common.clone(),
                    main_receiver,
                    power_receiver,
                }),
                self.0.startup_activation_token.borrow().clone(),
            ) {
                Ok(wayland) => wayland,
                Err(error) => {
                    let (error, services) = error.into_parts();
                    let Some(services) = services else {
                        self.finish_transition(Err(error.context(
                            "Wayland initialization did not return the headless services",
                        )));
                        break;
                    };
                    self.0.requested_headless.set(true);
                    let event_loop = match EventLoop::try_new() {
                        Ok(event_loop) => event_loop,
                        Err(event_loop_error) => {
                            self.finish_transition(Err(error.context(format!(
                                "also failed to recreate headless event loop: {event_loop_error}"
                            ))));
                            break;
                        }
                    };
                    runtime = HeadlessRuntime {
                        event_loop,
                        main_receiver: services.main_receiver,
                        power_receiver: services.power_receiver,
                    };
                    self.finish_transition(Err(error));
                    continue;
                }
            };
            self.0.startup_activation_token.borrow_mut().take();
            self.0.headless.set(false);
            self.0.wayland.borrow_mut().replace(wayland);
            self.finish_transition(Ok(()));
            let (main_receiver, power_receiver) = self
                .0
                .wayland
                .borrow()
                .as_ref()
                .expect("Wayland client was just installed")
                .run_and_recover_services();

            if self.0.quitting.get() {
                break;
            }
            if !self.0.requested_headless.get() {
                break;
            }

            self.0.wayland.borrow_mut().take();
            self.0.headless.set(true);
            let event_loop = EventLoop::try_new().expect("failed to recreate headless event loop");
            runtime = HeadlessRuntime {
                event_loop,
                main_receiver,
                power_receiver,
            };
            self.finish_transition(Ok(()));
        }
    }

    fn set_headless(&self, headless: bool) -> Task<anyhow::Result<()>> {
        if self.0.requested_headless.get() != self.0.headless.get() {
            return Task::ready(Err(anyhow::anyhow!(
                "a display backend transition is already in progress"
            )));
        }
        if headless == self.0.headless.get() {
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
        self.0.common.borrow().signal.stop();
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

    fn quit(&self) {
        self.0.quitting.set(true);
        self.0.common.borrow().signal.stop();
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

                            let first_attach = cx.update(|cx| cx.set_headless(false));
                            assert!(first_attach.await.is_err());
                            assert_eq!(entity.read_with(cx, |value, _| *value), 2);
                            release_task
                                .send(())
                                .expect("queued task remains attached to the executor");
                            queued_task.await;

                            let second_attach = cx.update(|cx| cx.set_headless(false));
                            assert!(second_attach.await.is_err());
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
}
