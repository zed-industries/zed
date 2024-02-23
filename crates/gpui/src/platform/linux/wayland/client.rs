use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;
use wayland_backend::client::ObjectId;
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
use wayland_protocols::wp::fractional_scale::v1::client::{
    wp_fractional_scale_manager_v1, wp_fractional_scale_v1,
};
use wayland_protocols::wp::viewporter::client::{wp_viewport, wp_viewporter};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use xkbcommon::xkb;
use xkbcommon::xkb::ffi::XKB_KEYMAP_FORMAT_TEXT_V1;
use xkbcommon::xkb::{Keycode, KEYMAP_COMPILE_NO_FLAGS};

use crate::platform::linux::client::Client;
use crate::platform::linux::wayland::window::WaylandWindow;
use crate::platform::{LinuxPlatformInner, PlatformWindow};
use crate::ScrollDelta;
use crate::{
    platform::linux::wayland::window::WaylandWindowState, AnyWindowHandle, DisplayId, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    Pixels, PlatformDisplay, PlatformInput, Point, ScrollWheelEvent, TouchPhase, WindowOptions,
};

const MIN_KEYCODE: u32 = 8; // used to convert evdev scancode to xkb scancode

pub(crate) struct WaylandClientState {
    compositor: Option<wl_compositor::WlCompositor>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    viewporter: Option<wp_viewporter::WpViewporter>,
    fractional_scale_manager: Option<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1>,
    windows: Vec<(xdg_surface::XdgSurface, Rc<WaylandWindowState>)>,
    platform_inner: Rc<LinuxPlatformInner>,
    wl_seat: Option<wl_seat::WlSeat>,
    keymap_state: Option<xkb::State>,
    modifiers: Modifiers,
    scroll_direction: f64,
    mouse_location: Option<Point<Pixels>>,
    button_pressed: Option<MouseButton>,
    mouse_focused_window: Option<Rc<WaylandWindowState>>,
    keyboard_focused_window: Option<Rc<WaylandWindowState>>,
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
            viewporter: None,
            fractional_scale_manager: None,
            windows: Vec::new(),
            platform_inner: Rc::clone(&linux_platform_inner),
            wl_seat: None,
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
            mouse_focused_window: None,
            keyboard_focused_window: None,
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

        let viewport = state
            .viewporter
            .as_ref()
            .map(|viewporter| viewporter.get_viewport(&wl_surface, &self.qh, ()));

        wl_surface.frame(&self.qh, wl_surface.clone());
        wl_surface.commit();

        let window_state = Rc::new(WaylandWindowState::new(
            &self.conn,
            wl_surface.clone(),
            viewport,
            Arc::new(toplevel),
            options,
        ));

        if let Some(fractional_scale_manager) = state.fractional_scale_manager.as_ref() {
            fractional_scale_manager.get_fractional_scale(&wl_surface, &self.qh, xdg_surface.id());
        }

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
                "wp_fractional_scale_manager_v1" => {
                    let manager = registry
                        .bind::<wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );
                    state.fractional_scale_manager = Some(manager);
                }
                "wp_viewporter" => {
                    let view_porter =
                        registry.bind::<wp_viewporter::WpViewporter, _, _>(name, 1, qh, ());
                    state.viewporter = Some(view_porter);
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
delegate_noop!(WaylandClientState: ignore wp_fractional_scale_manager_v1::WpFractionalScaleManagerV1);
delegate_noop!(WaylandClientState: ignore wp_viewporter::WpViewporter);
delegate_noop!(WaylandClientState: ignore wp_viewport::WpViewport);

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
                seat.get_keyboard(qh, ());
            }
            if capabilities.contains(wl_seat::Capability::Pointer) {
                seat.get_pointer(qh, ());
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
        match event {
            wl_keyboard::Event::Keymap {
                format: WEnum::Value(format),
                fd,
                size,
                ..
            } => {
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
            }
            wl_keyboard::Event::Enter { surface, .. } => {
                for window in &state.windows {
                    if window.1.surface.id() == surface.id() {
                        state.keyboard_focused_window = Some(Rc::clone(&window.1));
                    }
                }
            }
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => {
                let keymap_state = state.keymap_state.as_mut().unwrap();
                keymap_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
                state.modifiers.shift =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE);
                state.modifiers.alt =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE);
                state.modifiers.control =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE);
                state.modifiers.command =
                    keymap_state.mod_name_is_active(xkb::MOD_NAME_LOGO, xkb::STATE_MODS_EFFECTIVE);
            }
            wl_keyboard::Event::Key {
                key,
                state: WEnum::Value(key_state),
                ..
            } => {
                let keymap_state = state.keymap_state.as_ref().unwrap();
                let keycode = Keycode::from(key + MIN_KEYCODE);
                let key_utf32 = keymap_state.key_get_utf32(keycode);
                let key_utf8 = keymap_state.key_get_utf8(keycode);
                let key_sym = keymap_state.key_get_one_sym(keycode);
                let key = xkb::keysym_get_name(key_sym).to_lowercase();

                // Ignore control characters (and DEL) for the purposes of ime_key,
                // but if key_utf32 is 0 then assume it isn't one
                let ime_key =
                    (key_utf32 == 0 || (key_utf32 >= 32 && key_utf32 != 127)).then_some(key_utf8);

                let focused_window = &state.keyboard_focused_window;
                if let Some(focused_window) = focused_window {
                    match key_state {
                        wl_keyboard::KeyState::Pressed => {
                            focused_window.handle_input(PlatformInput::KeyDown(KeyDownEvent {
                                keystroke: Keystroke {
                                    modifiers: state.modifiers,
                                    key,
                                    ime_key,
                                },
                                is_held: false, // todo!(linux)
                            }));
                        }
                        wl_keyboard::KeyState::Released => {
                            focused_window.handle_input(PlatformInput::KeyUp(KeyUpEvent {
                                keystroke: Keystroke {
                                    modifiers: state.modifiers,
                                    key,
                                    ime_key,
                                },
                            }));
                        }
                        _ => {}
                    }
                }
            }
            wl_keyboard::Event::Leave { .. } => {}
            _ => {}
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
        match event {
            wl_pointer::Event::Enter {
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                for window in &state.windows {
                    if window.1.surface.id() == surface.id() {
                        state.mouse_focused_window = Some(Rc::clone(&window.1));
                    }
                }
                state.mouse_location = Some(Point {
                    x: Pixels::from(surface_x),
                    y: Pixels::from(surface_y),
                });
            }
            wl_pointer::Event::Motion {
                time,
                surface_x,
                surface_y,
                ..
            } => {
                let focused_window = &state.mouse_focused_window;
                if let Some(focused_window) = focused_window {
                    state.mouse_location = Some(Point {
                        x: Pixels::from(surface_x),
                        y: Pixels::from(surface_y),
                    });
                    focused_window.handle_input(PlatformInput::MouseMove(MouseMoveEvent {
                        position: state.mouse_location.unwrap(),
                        pressed_button: state.button_pressed,
                        modifiers: state.modifiers,
                    }))
                }
            }
            wl_pointer::Event::Button {
                button,
                state: WEnum::Value(button_state),
                ..
            } => {
                let focused_window = &state.mouse_focused_window;
                let mouse_location = &state.mouse_location;
                if let (Some(focused_window), Some(mouse_location)) =
                    (focused_window, mouse_location)
                {
                    match button_state {
                        wl_pointer::ButtonState::Pressed => {
                            state.button_pressed = Some(linux_button_to_gpui(button));
                            focused_window.handle_input(PlatformInput::MouseDown(MouseDownEvent {
                                button: linux_button_to_gpui(button),
                                position: *mouse_location,
                                modifiers: state.modifiers,
                                click_count: 1,
                            }));
                        }
                        wl_pointer::ButtonState::Released => {
                            state.button_pressed = None;
                            focused_window.handle_input(PlatformInput::MouseUp(MouseUpEvent {
                                button: linux_button_to_gpui(button),
                                position: *mouse_location,
                                modifiers: Modifiers::default(),
                                click_count: 1,
                            }));
                        }
                        _ => {}
                    }
                }
            }
            wl_pointer::Event::AxisRelativeDirection {
                direction: WEnum::Value(direction),
                ..
            } => {
                state.scroll_direction = match direction {
                    AxisRelativeDirection::Identical => -1.0,
                    AxisRelativeDirection::Inverted => 1.0,
                    _ => -1.0,
                }
            }
            wl_pointer::Event::Axis {
                time,
                axis: WEnum::Value(axis),
                value,
                ..
            } => {
                let focused_window = &state.mouse_focused_window;
                let mouse_location = &state.mouse_location;
                if let (Some(focused_window), Some(mouse_location)) =
                    (focused_window, mouse_location)
                {
                    let value = value * state.scroll_direction;
                    focused_window.handle_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: *mouse_location,
                        delta: match axis {
                            wl_pointer::Axis::VerticalScroll => {
                                ScrollDelta::Pixels(Point::new(Pixels(0.0), Pixels(value as f32)))
                            }
                            wl_pointer::Axis::HorizontalScroll => {
                                ScrollDelta::Pixels(Point::new(Pixels(value as f32), Pixels(0.0)))
                            }
                            _ => unimplemented!(),
                        },
                        modifiers: state.modifiers,
                        touch_phase: TouchPhase::Started,
                    }))
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                let focused_window = &state.mouse_focused_window;
                if let Some(focused_window) = focused_window {
                    focused_window.handle_input(PlatformInput::MouseMove(MouseMoveEvent {
                        position: Point::<Pixels>::default(),
                        pressed_button: None,
                        modifiers: Modifiers::default(),
                    }));
                }
                state.mouse_focused_window = None;
                state.mouse_location = None;
            }
            _ => {}
        }
    }
}

impl Dispatch<wp_fractional_scale_v1::WpFractionalScaleV1, ObjectId> for WaylandClientState {
    fn event(
        state: &mut Self,
        _: &wp_fractional_scale_v1::WpFractionalScaleV1,
        event: <wp_fractional_scale_v1::WpFractionalScaleV1 as Proxy>::Event,
        id: &ObjectId,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wp_fractional_scale_v1::Event::PreferredScale { scale, .. } = event {
            for window in &state.windows {
                if window.0.id() == *id {
                    window.1.rescale(scale as f32 / 120.0);
                    return;
                }
            }
        }
    }
}
