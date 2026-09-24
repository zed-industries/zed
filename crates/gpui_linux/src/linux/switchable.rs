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
    PriorityQueueCalloopReceiver, SystemPowerEvent, WaylandClient,
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
    transition_waiter: RefCell<Option<oneshot::Sender<()>>>,
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

    fn finish_transition(&self) {
        if let Some(waiter) = self.0.transition_waiter.borrow_mut().take() {
            waiter.send(()).unwrap_or(());
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

            let wayland = WaylandClient::new_with_services(
                Some((self.0.common.clone(), main_receiver, power_receiver)),
                self.0.startup_activation_token.borrow_mut().take(),
            );
            self.0.headless.set(false);
            self.0.wayland.borrow_mut().replace(wayland);
            self.finish_transition();
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
            self.finish_transition();
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
            })
    }

    fn quit(&self) {
        self.0.quitting.set(true);
        self.0.common.borrow().signal.stop();
    }
}
