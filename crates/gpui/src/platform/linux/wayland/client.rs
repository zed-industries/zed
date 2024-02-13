use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;
use wayland_client::{
    Connection,
    delegate_noop,
    Dispatch, EventQueue, protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_keyboard, wl_registry, wl_seat, wl_shm,
        wl_shm_pool,
        wl_surface::{self, WlSurface},
    }, Proxy, QueueHandle,
};
use wayland_client::protocol::wl_callback::WlCallback;
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

use crate::{
    AnyWindowHandle, DisplayId, platform::linux::wayland::window::WaylandWindowState,
    PlatformDisplay, WindowOptions,
};
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::platform::linux::client::Client;
use crate::platform::linux::wayland::window::WaylandWindow;

pub(crate) struct WaylandClientState {
    compositor: Option<wl_compositor::WlCompositor>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    windows: Vec<(xdg_surface::XdgSurface, Arc<WaylandWindowState>)>,
    platform_inner: Arc<LinuxPlatformInner>,
}

pub(crate) struct WaylandClient {
    platform_inner: Arc<LinuxPlatformInner>,
    conn: Arc<Connection>,
    state: Mutex<WaylandClientState>,
    event_queue: Mutex<EventQueue<WaylandClientState>>,
    qh: Arc<QueueHandle<WaylandClientState>>,
}

impl WaylandClient {
    pub(crate) fn new(
        linux_platform_inner: Arc<LinuxPlatformInner>,
        conn: Arc<Connection>,
    ) -> Self {
        let state = WaylandClientState {
            compositor: None,
            buffer: None,
            wm_base: None,
            windows: Vec::new(),
            platform_inner: Arc::clone(&linux_platform_inner),
        };
        let event_queue: EventQueue<WaylandClientState> = conn.new_event_queue();
        let qh = event_queue.handle();
        Self {
            platform_inner: linux_platform_inner,
            conn,
            state: Mutex::new(state),
            event_queue: Mutex::new(event_queue),
            qh: Arc::new(qh),
        }
    }
}

impl Client for WaylandClient {
    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        let display = self.conn.display();
        let mut eq = self.event_queue.lock();
        let _registry = display.get_registry(&self.qh, ());

        eq.roundtrip(&mut self.state.lock()).unwrap();

        on_finish_launching();
        while !self.platform_inner.state.lock().quit_requested {
            eq.flush().unwrap();
            eq.dispatch_pending(&mut self.state.lock()).unwrap();
            if let Some(guard) = self.conn.prepare_read() {
                guard.read().unwrap();
                eq.dispatch_pending(&mut self.state.lock()).unwrap();
            }
            if let Ok(runnable) = self.platform_inner.main_receiver.try_recv() {
                runnable.run();
            }
        }
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        Vec::new()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        todo!()
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.state.lock();

        let wm_base = state.wm_base.as_ref().unwrap();
        let compositor = state.compositor.as_ref().unwrap();
        let wl_surface = compositor.create_surface(&self.qh, ());
        let xdg_surface = wm_base.get_xdg_surface(&wl_surface, &self.qh, ());
        let toplevel = xdg_surface.get_toplevel(&self.qh, ());
        let wl_surface = Arc::new(wl_surface);

        wl_surface.frame(&self.qh, wl_surface.clone());
        wl_surface.commit();

        let window_state: Arc<WaylandWindowState> = Arc::new(WaylandWindowState::new(
            &self.conn,
            wl_surface.clone(),
            Arc::new(toplevel),
            options,
        ));

        state.windows.push((xdg_surface, Arc::clone(&window_state)));
        Box::new(WaylandWindow(window_state))
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    state.compositor = Some(compositor);
                }
                "xdg_wm_base" => {
                    let wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, 1, qh, ());
                    state.wm_base = Some(wm_base);
                }
                _ => {}
            };
        }
    }
}

delegate_noop!(WaylandClientState: ignore wl_compositor::WlCompositor);
delegate_noop!(WaylandClientState: ignore wl_surface::WlSurface);
delegate_noop!(WaylandClientState: ignore wl_shm::WlShm);
delegate_noop!(WaylandClientState: ignore wl_shm_pool::WlShmPool);
delegate_noop!(WaylandClientState: ignore wl_buffer::WlBuffer);
delegate_noop!(WaylandClientState: ignore wl_seat::WlSeat);
delegate_noop!(WaylandClientState: ignore wl_keyboard::WlKeyboard);

impl Dispatch<WlCallback, Arc<WlSurface>> for WaylandClientState {
    fn event(
        state: &mut Self,
        _: &WlCallback,
        event: wl_callback::Event,
        surf: &Arc<WlSurface>,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_callback::Event::Done { .. } = event {
            for window in &state.windows {
                if window.1.surface.id() == surf.id() {
                    window.1.surface.frame(qh, surf.clone());
                    window.1.update();
                    window.1.surface.commit();
                }
            }
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial, .. } = event {
            xdg_surface.ack_configure(serial);
            for window in &state.windows {
                if &window.0 == xdg_surface {
                    window.1.update();
                    window.1.surface.commit();
                    return;
                }
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        xdg_toplevel: &xdg_toplevel::XdgToplevel,
        event: <xdg_toplevel::XdgToplevel as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_toplevel::Event::Configure {
            width,
            height,
            states: _states,
        } = event
        {
            if width == 0 || height == 0 {
                return;
            }
            for window in &state.windows {
                if window.1.toplevel.id() == xdg_toplevel.id() {
                    window.1.resize(width, height);
                    window.1.surface.commit();
                    return;
                }
            }
        } else if let xdg_toplevel::Event::Close = event {
            state.windows.retain(|(_, window)| {
                if window.toplevel.id() == xdg_toplevel.id() {
                    window.toplevel.destroy();
                    false
                } else {
                    true
                }
            });
            state.platform_inner.state.lock().quit_requested |= state.windows.is_empty();
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for WaylandClientState {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: <xdg_wm_base::XdgWmBase as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}
