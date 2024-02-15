use itertools::Itertools;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;
use wayland_backend::protocol::WEnum;
use wayland_client::protocol::wl_callback::WlCallback;
use wayland_client::protocol::wl_pointer::AxisRelativeDirection;
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_callback, wl_compositor, wl_keyboard, wl_pointer, wl_registry, wl_seat,
        wl_shm, wl_shm_pool,
        wl_surface::{self, WlSurface},
    },
    Connection, Dispatch, EventQueue, Proxy, QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use xkbcommon::xkb;
use xkbcommon::xkb::ffi::XKB_KEYMAP_FORMAT_TEXT_V1;
use xkbcommon::xkb::{Keycode, KEYMAP_COMPILE_NO_FLAGS};

use crate::platform::linux::client::Client;
use crate::platform::linux::wayland::window::WaylandWindow;
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::PlatformInput::KeyDown;
use crate::ScrollDelta::Lines;
use crate::{
    platform::linux::wayland::window::WaylandWindowState, AnyWindowHandle, DisplayId, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, PlatformDisplay, PlatformInput, Point, ScrollWheelEvent, TouchPhase, WindowOptions,
};

pub(crate) struct WaylandClientState {
    compositor: Option<wl_compositor::WlCompositor>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    windows: Vec<(xdg_surface::XdgSurface, Rc<WaylandWindowState>)>,
    platform_inner: Rc<LinuxPlatformInner>,
    wl_seat: Option<wl_seat::WlSeat>,
    keymap: Option<xkb::Keymap>,
    keymap_state: Option<xkb::State>,
    modifiers: Modifiers,
    scroll_direction: f64,
    mouse_location: Option<Point<Pixels>>,
    button_pressed: Option<MouseButton>,
    focused_window: Option<Rc<WaylandWindowState>>,
}

pub(crate) struct WaylandClient {
    platform_inner: Rc<LinuxPlatformInner>,
    conn: Arc<Connection>,
    state: Mutex<WaylandClientState>,
    event_queue: Mutex<EventQueue<WaylandClientState>>,
    qh: Arc<QueueHandle<WaylandClientState>>,
}

impl WaylandClient {
    pub(crate) fn new(linux_platform_inner: Rc<LinuxPlatformInner>, conn: Arc<Connection>) -> Self {
        let state = WaylandClientState {
            compositor: None,
            buffer: None,
            wm_base: None,
            windows: Vec::new(),
            platform_inner: Rc::clone(&linux_platform_inner),
            wl_seat: None,
            keymap: None,
            keymap_state: None,
            modifiers: Modifiers {
                shift: false,
                control: false,
                alt: false,
                function: false,
                command: false,
            },
            scroll_direction: -1.0,
            mouse_location: None,
            button_pressed: None,
            focused_window: None,
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
        unimplemented!()
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

        let window_state = Rc::new(WaylandWindowState::new(
            &self.conn,
            wl_surface.clone(),
            Arc::new(toplevel),
            options,
        ));

        state.windows.push((xdg_surface, Rc::clone(&window_state)));
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
                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                    state.wl_seat = Some(seat);
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

impl Dispatch<wl_seat::WlSeat, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(&qh, ());
            }
            if capabilities.contains(wl_seat::Capability::Pointer) {
                seat.get_pointer(&qh, ());
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_keyboard::Event::Keymap {
            format: WEnum::Value(format),
            fd,
            size,
            ..
        } = event
        {
            assert_eq!(
                format,
                wl_keyboard::KeymapFormat::XkbV1,
                "Unsupported keymap format"
            );
            let keymap = unsafe {
                xkb::Keymap::new_from_fd(
                    &xkb::Context::new(xkb::CONTEXT_NO_FLAGS),
                    fd,
                    size as usize,
                    XKB_KEYMAP_FORMAT_TEXT_V1,
                    KEYMAP_COMPILE_NO_FLAGS,
                )
                .unwrap()
            }
            .unwrap();
            state.keymap_state = Some(xkb::State::new(&keymap));
            state.keymap = Some(keymap);
        } else if let wl_keyboard::Event::Enter { surface, .. } = event {
            for window in &state.windows {
                if window.1.surface.id() == surface.id() {
                    state.focused_window = Some(Rc::clone(&window.1));
                }
            }
        } else if let wl_keyboard::Event::Key {
            key,
            state: WEnum::Value(key_state),
            ..
        } = event
        {
            let keymap = state.keymap.as_ref().unwrap();
            let keymap_state = state.keymap_state.as_ref().unwrap();
            let key_string = keymap_state.key_get_utf8(Keycode::from(key + 8));
            let key_name =
                xkb::keysym_get_name(keymap.key_get_syms_by_level(Keycode::from(key + 8), 0, 0)[0])
                    .to_lowercase();

            let key = if matches!(
                key_name.as_str(),
                "backspace" | "left" | "right" | "down" | "up" | "super_l" | "super_r"
            ) {
                key_name.clone()
            } else {
                key_string.clone()
            };

            match key_state {
                wl_keyboard::KeyState::Pressed => {
                    if key_name.starts_with("shift") {
                        state.modifiers.shift = true;
                    } else if key_name.starts_with("control") {
                        state.modifiers.control = true;
                    } else if key_name.starts_with("alt") {
                        state.modifiers.alt = true;
                    } else if state.focused_window.is_some() {
                        state.focused_window.as_ref().unwrap().handle_input(KeyDown(
                            KeyDownEvent {
                                keystroke: Keystroke {
                                    modifiers: state.modifiers.clone(),
                                    key: key.clone(),
                                    ime_key: None,
                                },
                                is_held: true,
                            },
                        ));
                    }
                }
                wl_keyboard::KeyState::Released => {
                    if key_name.starts_with("shift") {
                        state.modifiers.shift = false;
                    } else if key_name.starts_with("control") {
                        state.modifiers.control = false;
                        state.modifiers.command = false;
                    } else if key_name.starts_with("alt") {
                        state.modifiers.alt = false;
                    } else if key_name.starts_with("super") {
                        state.modifiers.command = false;
                    } else if state.focused_window.is_some() {
                        state
                            .focused_window
                            .as_ref()
                            .unwrap()
                            .handle_input(PlatformInput::KeyUp(KeyUpEvent {
                                keystroke: Keystroke {
                                    modifiers: state.modifiers.clone(),
                                    key: key.clone(),
                                    ime_key: None,
                                },
                            }));
                    }
                }
                _ => {}
            }
        } else if let wl_keyboard::Event::Leave { .. } = event {
            state.modifiers = Modifiers {
                control: false,
                alt: false,
                shift: false,
                command: false,
                function: false,
            };
        }
    }
}

fn linux_button_to_gpui(button: u32) -> MouseButton {
    match button {
        0x110 => MouseButton::Left,
        0x111 => MouseButton::Right,
        0x112 => MouseButton::Middle,
        _ => unimplemented!(), // todo!(linux)
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for WaylandClientState {
    fn event(
        state: &mut Self,
        wl_pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        data: &(),
        conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_pointer::Event::Enter {
            surface,
            surface_x,
            surface_y,
            ..
        } = event
        {
            for window in &state.windows {
                if window.1.surface.id() == surface.id() {
                    state.focused_window = Some(Rc::clone(&window.1));
                }
            }

            state.mouse_location = Some(Point {
                x: Pixels::from(surface_x),
                y: Pixels::from(surface_y),
            });
        } else if state.focused_window.is_none() {
            return;
        } else if let wl_pointer::Event::Motion {
            time,
            surface_x,
            surface_y,
            ..
        } = event
        {
            state.mouse_location = Some(Point {
                x: Pixels::from(surface_x),
                y: Pixels::from(surface_y),
            });
            state
                .focused_window
                .as_ref()
                .unwrap()
                .handle_input(PlatformInput::MouseMove(MouseMoveEvent {
                    position: state.mouse_location.unwrap(),
                    pressed_button: state.button_pressed,
                    modifiers: state.modifiers.clone(),
                }))
        } else if let wl_pointer::Event::Button {
            button,
            state: WEnum::Value(button_state),
            ..
        } = event
        {
            match button_state {
                wl_pointer::ButtonState::Pressed => {
                    state.button_pressed = Some(linux_button_to_gpui(button));
                    state
                        .focused_window
                        .as_ref()
                        .unwrap()
                        .handle_input(PlatformInput::MouseDown(MouseDownEvent {
                            button: linux_button_to_gpui(button),
                            position: state.mouse_location.unwrap(),
                            modifiers: state.modifiers.clone(),
                            click_count: 1,
                        }));
                }
                wl_pointer::ButtonState::Released => {
                    state.button_pressed = None;
                    state
                        .focused_window
                        .as_ref()
                        .unwrap()
                        .handle_input(PlatformInput::MouseUp(MouseUpEvent {
                            button: linux_button_to_gpui(button),
                            position: state.mouse_location.unwrap(),
                            modifiers: Modifiers {
                                shift: false,
                                control: false,
                                alt: false,
                                function: false,
                                command: false,
                            },
                            click_count: 1,
                        }));
                }
                _ => {}
            }
        } else if let wl_pointer::Event::AxisRelativeDirection {
            direction: WEnum::Value(direction),
            ..
        } = event
        {
            state.scroll_direction = match direction {
                AxisRelativeDirection::Identical => -1.0,
                AxisRelativeDirection::Inverted => 1.0,
                _ => -1.0,
            }
        } else if let wl_pointer::Event::Axis {
            time,
            axis: WEnum::Value(axis),
            value,
            ..
        } = event
        {
            let value = value * state.scroll_direction;
            state
                .focused_window
                .as_ref()
                .unwrap()
                .handle_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position: state.mouse_location.unwrap(),
                    delta: match axis {
                        wl_pointer::Axis::VerticalScroll => Lines(Point::new(0.0, value as f32)),
                        wl_pointer::Axis::HorizontalScroll => Lines(Point::new(value as f32, 0.0)),
                        _ => unimplemented!(), // todo!(linux)
                    },
                    modifiers: state.modifiers.clone(),
                    touch_phase: TouchPhase::Started,
                }))
        } else if let wl_pointer::Event::Leave { surface, .. } = event {
            state.mouse_location = None;
            state.focused_window = None;
        }
    }
}
